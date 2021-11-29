#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ICS_TARGETTYPE(pub i32);
pub const ICSTT_NAME: ICS_TARGETTYPE = ICS_TARGETTYPE(0i32);
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = ICS_TARGETTYPE(1i32);
impl ::core::convert::From<i32> for ICS_TARGETTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ICS_TARGETTYPE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDynamicPortMapping(pub ::windows::core::IUnknown);
impl IDynamicPortMapping {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExternalIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InternalClient(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn LeaseDuration(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lleasedurationdesired), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditInternalClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), bstrinternalclient.into_param().abi()).ok()
    }
    pub unsafe fn Enable(&self, vb: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(vb)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(linternalport)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDynamicPortMapping {
    type Vtable = IDynamicPortMapping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fc80282_23b6_4378_9a27_cd8f17c9400c);
}
impl ::core::convert::From<IDynamicPortMapping> for ::windows::core::IUnknown {
    fn from(value: IDynamicPortMapping) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDynamicPortMapping> for ::windows::core::IUnknown {
    fn from(value: &IDynamicPortMapping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDynamicPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDynamicPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDynamicPortMapping> for super::super::System::Com::IDispatch {
    fn from(value: IDynamicPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDynamicPortMapping> for super::super::System::Com::IDispatch {
    fn from(value: &IDynamicPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IDynamicPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IDynamicPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vb: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, linternalport: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDynamicPortMappingCollection(pub ::windows::core::IUnknown);
impl IDynamicPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows::core::Result<IDynamicPortMapping> {
        let mut result__: <IDynamicPortMapping as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), &mut result__).from_abi::<IDynamicPortMapping>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2, linternalport: i32, bstrinternalclient: Param4, benabled: i16, bstrdescription: Param6, lleaseduration: i32) -> ::windows::core::Result<IDynamicPortMapping> {
        let mut result__: <IDynamicPortMapping as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::core::mem::transmute(benabled), bstrdescription.into_param().abi(), ::core::mem::transmute(lleaseduration), &mut result__).from_abi::<IDynamicPortMapping>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDynamicPortMappingCollection {
    type Vtable = IDynamicPortMappingCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb60de00f_156e_4e8d_9ec1_3a2342c10899);
}
impl ::core::convert::From<IDynamicPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: IDynamicPortMappingCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDynamicPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: &IDynamicPortMappingCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDynamicPortMappingCollection> for super::super::System::Com::IDispatch {
    fn from(value: IDynamicPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDynamicPortMappingCollection> for super::super::System::Com::IDispatch {
    fn from(value: &IDynamicPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumNetConnection(pub ::windows::core::IUnknown);
impl IEnumNetConnection {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetConnection>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetConnection> {
        let mut result__: <IEnumNetConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetConnection>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumNetConnection {
    type Vtable = IEnumNetConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a0_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<IEnumNetConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumNetConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumNetSharingEveryConnection(pub ::windows::core::IUnknown);
impl IEnumNetSharingEveryConnection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingEveryConnection> {
        let mut result__: <IEnumNetSharingEveryConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingEveryConnection>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingEveryConnection {
    type Vtable = IEnumNetSharingEveryConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b8_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<IEnumNetSharingEveryConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingEveryConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumNetSharingEveryConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingEveryConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingEveryConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetSharingEveryConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingEveryConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumNetSharingPortMapping(pub ::windows::core::IUnknown);
impl IEnumNetSharingPortMapping {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPortMapping> {
        let mut result__: <IEnumNetSharingPortMapping as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingPortMapping>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingPortMapping {
    type Vtable = IEnumNetSharingPortMapping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b0_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<IEnumNetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingPortMapping) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumNetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingPortMapping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumNetSharingPrivateConnection(pub ::windows::core::IUnknown);
impl IEnumNetSharingPrivateConnection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPrivateConnection> {
        let mut result__: <IEnumNetSharingPrivateConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingPrivateConnection>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingPrivateConnection {
    type Vtable = IEnumNetSharingPrivateConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b5_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<IEnumNetSharingPrivateConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingPrivateConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumNetSharingPrivateConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingPrivateConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingPrivateConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetSharingPrivateConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPrivateConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumNetSharingPublicConnection(pub ::windows::core::IUnknown);
impl IEnumNetSharingPublicConnection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPublicConnection> {
        let mut result__: <IEnumNetSharingPublicConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingPublicConnection>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingPublicConnection {
    type Vtable = IEnumNetSharingPublicConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b4_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<IEnumNetSharingPublicConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingPublicConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumNetSharingPublicConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingPublicConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingPublicConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetSharingPublicConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPublicConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INATEventManager(pub ::windows::core::IUnknown);
impl INATEventManager {
    pub unsafe fn SetExternalIPAddressCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn SetNumberOfEntriesCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for INATEventManager {
    type Vtable = INATEventManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624bd588_9060_4109_b0b0_1adbbcac32df);
}
impl ::core::convert::From<INATEventManager> for ::windows::core::IUnknown {
    fn from(value: INATEventManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INATEventManager> for ::windows::core::IUnknown {
    fn from(value: &INATEventManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INATEventManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INATEventManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INATEventManager> for super::super::System::Com::IDispatch {
    fn from(value: INATEventManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INATEventManager> for super::super::System::Com::IDispatch {
    fn from(value: &INATEventManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INATEventManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INATEventManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INATEventManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INATExternalIPAddressCallback(pub ::windows::core::IUnknown);
impl INATExternalIPAddressCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NewExternalIPAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnewexternalipaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrnewexternalipaddress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for INATExternalIPAddressCallback {
    type Vtable = INATExternalIPAddressCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c416740_a34e_446f_ba06_abd04c3149ae);
}
impl ::core::convert::From<INATExternalIPAddressCallback> for ::windows::core::IUnknown {
    fn from(value: INATExternalIPAddressCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INATExternalIPAddressCallback> for ::windows::core::IUnknown {
    fn from(value: &INATExternalIPAddressCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INATExternalIPAddressCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INATExternalIPAddressCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INATExternalIPAddressCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INATNumberOfEntriesCallback(pub ::windows::core::IUnknown);
impl INATNumberOfEntriesCallback {
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewnumberofentries)).ok()
    }
}
unsafe impl ::windows::core::Interface for INATNumberOfEntriesCallback {
    type Vtable = INATNumberOfEntriesCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc83a0a74_91ee_41b6_b67a_67e0f00bbd78);
}
impl ::core::convert::From<INATNumberOfEntriesCallback> for ::windows::core::IUnknown {
    fn from(value: INATNumberOfEntriesCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INATNumberOfEntriesCallback> for ::windows::core::IUnknown {
    fn from(value: &INATNumberOfEntriesCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INATNumberOfEntriesCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INATNumberOfEntriesCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INATNumberOfEntriesCallback_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lnewnumberofentries: i32) -> ::windows::core::HRESULT);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INET_FIREWALL_AC_BINARIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INET_FIREWALL_AC_BINARIES").field("count", &self.count).field("binaries", &self.binaries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_BINARIES {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.binaries == other.binaries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_BINARIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for INET_FIREWALL_AC_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INET_FIREWALL_AC_CAPABILITIES").field("count", &self.count).field("capabilities", &self.capabilities).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.capabilities == other.capabilities
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub displayName: super::super::Foundation::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CHANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CHANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CHANGE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CHANGE_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(pub i32);
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(0i32);
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(1i32);
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(3i32);
impl ::core::convert::From<i32> for INET_FIREWALL_AC_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CHANGE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(pub i32);
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(0i32);
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(1i32);
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(2i32);
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(4i32);
impl ::core::convert::From<i32> for INET_FIREWALL_AC_CREATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CREATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub appContainerName: super::super::Foundation::PWSTR,
    pub displayName: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: super::super::Foundation::PWSTR,
    pub packageFullName: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for INET_FIREWALL_APP_CONTAINER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INET_FIREWALL_APP_CONTAINER").field("appContainerSid", &self.appContainerSid).field("userSid", &self.userSid).field("appContainerName", &self.appContainerName).field("displayName", &self.displayName).field("description", &self.description).field("capabilities", &self.capabilities).field("binaries", &self.binaries).field("workingDirectory", &self.workingDirectory).field("packageFullName", &self.packageFullName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_APP_CONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.appContainerSid == other.appContainerSid && self.userSid == other.userSid && self.appContainerName == other.appContainerName && self.displayName == other.displayName && self.description == other.description && self.capabilities == other.capabilities && self.binaries == other.binaries && self.workingDirectory == other.workingDirectory && self.packageFullName == other.packageFullName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_APP_CONTAINER {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetConnection(pub ::windows::core::IUnknown);
impl INetConnection {
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Duplicate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwduplicatename: Param0) -> ::windows::core::Result<INetConnection> {
        let mut result__: <INetConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszwduplicatename.into_param().abi(), &mut result__).from_abi::<INetConnection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<*mut NETCON_PROPERTIES> {
        let mut result__: <*mut NETCON_PROPERTIES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut NETCON_PROPERTIES>(result__)
    }
    pub unsafe fn GetUiObjectClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Rename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwnewname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszwnewname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for INetConnection {
    type Vtable = INetConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a1_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<INetConnection> for ::windows::core::IUnknown {
    fn from(value: INetConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetConnection> for ::windows::core::IUnknown {
    fn from(value: &INetConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszwduplicatename: super::super::Foundation::PWSTR, ppcon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetConnectionConnectUi(pub ::windows::core::IUnknown);
impl INetConnectionConnectUi {
    pub unsafe fn SetConnection<'a, Param0: ::windows::core::IntoParam<'a, INetConnection>>(&self, pcon: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Disconnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetConnectionConnectUi {
    type Vtable = INetConnectionConnectUi_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a3_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<INetConnectionConnectUi> for ::windows::core::IUnknown {
    fn from(value: INetConnectionConnectUi) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetConnectionConnectUi> for ::windows::core::IUnknown {
    fn from(value: &INetConnectionConnectUi) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnectionConnectUi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetConnectionConnectUi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionConnectUi_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcon: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetConnectionManager(pub ::windows::core::IUnknown);
impl INetConnectionManager {
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows::core::Result<IEnumNetConnection> {
        let mut result__: <IEnumNetConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<IEnumNetConnection>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetConnectionManager {
    type Vtable = INetConnectionManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a2_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<INetConnectionManager> for ::windows::core::IUnknown {
    fn from(value: INetConnectionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetConnectionManager> for ::windows::core::IUnknown {
    fn from(value: &INetConnectionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionManager_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetConnectionProps(pub ::windows::core::IUnknown);
impl INetConnectionProps {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Guid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<NETCON_STATUS> {
        let mut result__: <NETCON_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NETCON_STATUS>(result__)
    }
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<NETCON_MEDIATYPE> {
        let mut result__: <NETCON_MEDIATYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NETCON_MEDIATYPE>(result__)
    }
    pub unsafe fn Characteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetConnectionProps {
    type Vtable = INetConnectionProps_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4277c95_ce5b_463d_8167_5662d9bcaa72);
}
impl ::core::convert::From<INetConnectionProps> for ::windows::core::IUnknown {
    fn from(value: INetConnectionProps) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetConnectionProps> for ::windows::core::IUnknown {
    fn from(value: &INetConnectionProps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnectionProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetConnectionProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetConnectionProps> for super::super::System::Com::IDispatch {
    fn from(value: INetConnectionProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetConnectionProps> for super::super::System::Com::IDispatch {
    fn from(value: &INetConnectionProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetConnectionProps {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetConnectionProps {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionProps_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdevicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: *mut NETCON_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwAuthorizedApplication(pub ::windows::core::IUnknown);
impl INetFwAuthorizedApplication {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessImageFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProcessImageFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwAuthorizedApplication {
    type Vtable = INetFwAuthorizedApplication_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e64ffa_c2c5_444e_a301_fb5e00018050);
}
impl ::core::convert::From<INetFwAuthorizedApplication> for ::windows::core::IUnknown {
    fn from(value: INetFwAuthorizedApplication) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwAuthorizedApplication> for ::windows::core::IUnknown {
    fn from(value: &INetFwAuthorizedApplication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwAuthorizedApplication> for super::super::System::Com::IDispatch {
    fn from(value: INetFwAuthorizedApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwAuthorizedApplication> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwAuthorizedApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwAuthorizedApplications(pub ::windows::core::IUnknown);
impl INetFwAuthorizedApplications {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, INetFwAuthorizedApplication>>(&self, app: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), app.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<INetFwAuthorizedApplication> {
        let mut result__: <INetFwAuthorizedApplication as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imagefilename.into_param().abi(), &mut result__).from_abi::<INetFwAuthorizedApplication>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwAuthorizedApplications {
    type Vtable = INetFwAuthorizedApplications_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x644efd52_ccf9_486c_97a2_39f352570b30);
}
impl ::core::convert::From<INetFwAuthorizedApplications> for ::windows::core::IUnknown {
    fn from(value: INetFwAuthorizedApplications) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwAuthorizedApplications> for ::windows::core::IUnknown {
    fn from(value: &INetFwAuthorizedApplications) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwAuthorizedApplications> for super::super::System::Com::IDispatch {
    fn from(value: INetFwAuthorizedApplications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwAuthorizedApplications> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwAuthorizedApplications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplications_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, app: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwIcmpSettings(pub ::windows::core::IUnknown);
impl INetFwIcmpSettings {
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundDestinationUnreachable(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowRedirect(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowRedirect(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundEchoRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundEchoRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundTimeExceeded(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundParameterProblem(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundParameterProblem(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundSourceQuench(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundSourceQuench(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundRouterRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundRouterRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundTimestampRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundTimestampRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowInboundMaskRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowInboundMaskRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowOutboundPacketTooBig(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwIcmpSettings {
    type Vtable = INetFwIcmpSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6207b2e_7cdd_426a_951e_5e1cbc5afead);
}
impl ::core::convert::From<INetFwIcmpSettings> for ::windows::core::IUnknown {
    fn from(value: INetFwIcmpSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwIcmpSettings> for ::windows::core::IUnknown {
    fn from(value: &INetFwIcmpSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwIcmpSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwIcmpSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwIcmpSettings> for super::super::System::Com::IDispatch {
    fn from(value: INetFwIcmpSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwIcmpSettings> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwIcmpSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwIcmpSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwIcmpSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allow: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwMgr(pub ::windows::core::IUnknown);
impl INetFwMgr {
    pub unsafe fn LocalPolicy(&self) -> ::windows::core::Result<INetFwPolicy> {
        let mut result__: <INetFwPolicy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwPolicy>(result__)
    }
    pub unsafe fn CurrentProfileType(&self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__: <NET_FW_PROFILE_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    pub unsafe fn RestoreDefaults(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsPortAllowed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: Param3, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imagefilename.into_param().abi(), ::core::mem::transmute(ipversion), ::core::mem::transmute(portnumber), localaddress.into_param().abi(), ::core::mem::transmute(ipprotocol), ::core::mem::transmute(allowed), ::core::mem::transmute(restricted)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsIcmpTypeAllowed<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, ipversion: NET_FW_IP_VERSION, localaddress: Param1, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion), localaddress.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(allowed), ::core::mem::transmute(restricted)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwMgr {
    type Vtable = INetFwMgr_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7898af5_cac4_4632_a2ec_da06e5111af2);
}
impl ::core::convert::From<INetFwMgr> for ::windows::core::IUnknown {
    fn from(value: INetFwMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwMgr> for ::windows::core::IUnknown {
    fn from(value: &INetFwMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwMgr> for super::super::System::Com::IDispatch {
    fn from(value: INetFwMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwMgr> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwMgr {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwMgr {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, restricted: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, restricted: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwOpenPort(pub ::windows::core::IUnknown);
impl INetFwOpenPort {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<NET_FW_IP_PROTOCOL> {
        let mut result__: <NET_FW_IP_PROTOCOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_PROTOCOL>(result__)
    }
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipprotocol)).ok()
    }
    pub unsafe fn Port(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPort(&self, portnumber: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(portnumber)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn BuiltIn(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwOpenPort {
    type Vtable = INetFwOpenPort_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0483ba0_47ff_4d9c_a6d6_7741d0b195f7);
}
impl ::core::convert::From<INetFwOpenPort> for ::windows::core::IUnknown {
    fn from(value: INetFwOpenPort) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwOpenPort> for ::windows::core::IUnknown {
    fn from(value: &INetFwOpenPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwOpenPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwOpenPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwOpenPort> for super::super::System::Com::IDispatch {
    fn from(value: INetFwOpenPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwOpenPort> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwOpenPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwOpenPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwOpenPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumber: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumber: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, builtin: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwOpenPorts(pub ::windows::core::IUnknown);
impl INetFwOpenPorts {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, INetFwOpenPort>>(&self, port: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), port.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(portnumber), ::core::mem::transmute(ipprotocol)).ok()
    }
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<INetFwOpenPort> {
        let mut result__: <INetFwOpenPort as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(portnumber), ::core::mem::transmute(ipprotocol), &mut result__).from_abi::<INetFwOpenPort>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwOpenPorts {
    type Vtable = INetFwOpenPorts_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e9d7fa_e07e_430a_b19a_090ce82d92e2);
}
impl ::core::convert::From<INetFwOpenPorts> for ::windows::core::IUnknown {
    fn from(value: INetFwOpenPorts) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwOpenPorts> for ::windows::core::IUnknown {
    fn from(value: &INetFwOpenPorts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwOpenPorts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwOpenPorts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwOpenPorts> for super::super::System::Com::IDispatch {
    fn from(value: INetFwOpenPorts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwOpenPorts> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwOpenPorts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwOpenPorts {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwOpenPorts {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPorts_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, port: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwPolicy(pub ::windows::core::IUnknown);
impl INetFwPolicy {
    pub unsafe fn CurrentProfile(&self) -> ::windows::core::Result<INetFwProfile> {
        let mut result__: <INetFwProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwProfile>(result__)
    }
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows::core::Result<INetFwProfile> {
        let mut result__: <INetFwProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<INetFwProfile>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwPolicy {
    type Vtable = INetFwPolicy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd46d2478_9ac9_4008_9dc7_5563ce5536cc);
}
impl ::core::convert::From<INetFwPolicy> for ::windows::core::IUnknown {
    fn from(value: INetFwPolicy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwPolicy> for ::windows::core::IUnknown {
    fn from(value: &INetFwPolicy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwPolicy> for super::super::System::Com::IDispatch {
    fn from(value: INetFwPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwPolicy> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwPolicy2(pub ::windows::core::IUnknown);
impl INetFwPolicy2 {
    pub unsafe fn CurrentProfileTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExcludedInterfaces<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), interfaces.into_param().abi()).ok()
    }
    pub unsafe fn BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetBlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(block)).ok()
    }
    pub unsafe fn NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(disabled)).ok()
    }
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(disabled)).ok()
    }
    pub unsafe fn Rules(&self) -> ::windows::core::Result<INetFwRules> {
        let mut result__: <INetFwRules as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwRules>(result__)
    }
    pub unsafe fn ServiceRestriction(&self) -> ::windows::core::Result<INetFwServiceRestriction> {
        let mut result__: <INetFwServiceRestriction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwServiceRestriction>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableRuleGroup<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1, enable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask), group.into_param().abi(), ::core::mem::transmute(enable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRuleGroupEnabled<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask), group.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetDefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetDefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(action)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRuleGroupCurrentlyEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, group: Param0) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), group.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn LocalPolicyModifyState(&self) -> ::windows::core::Result<NET_FW_MODIFY_STATE> {
        let mut result__: <NET_FW_MODIFY_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_MODIFY_STATE>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwPolicy2 {
    type Vtable = INetFwPolicy2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98325047_c671_4174_8d81_defcd3f03186);
}
impl ::core::convert::From<INetFwPolicy2> for ::windows::core::IUnknown {
    fn from(value: INetFwPolicy2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwPolicy2> for ::windows::core::IUnknown {
    fn from(value: &INetFwPolicy2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwPolicy2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwPolicy2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwPolicy2> for super::super::System::Com::IDispatch {
    fn from(value: INetFwPolicy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwPolicy2> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwPolicy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwPolicy2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwPolicy2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicerestriction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwProduct(pub ::windows::core::IUnknown);
impl INetFwProduct {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RuleCategories(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetRuleCategories<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, rulecategories: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), rulecategories.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, displayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), displayname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathToSignedProductExe(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwProduct {
    type Vtable = INetFwProduct_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71881699_18f4_458b_b892_3ffce5e07f75);
}
impl ::core::convert::From<INetFwProduct> for ::windows::core::IUnknown {
    fn from(value: INetFwProduct) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwProduct> for ::windows::core::IUnknown {
    fn from(value: &INetFwProduct) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwProduct> for super::super::System::Com::IDispatch {
    fn from(value: INetFwProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwProduct> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwProduct {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwProduct {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rulecategories: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displayname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwProducts(pub ::windows::core::IUnknown);
impl INetFwProducts {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Register<'a, Param0: ::windows::core::IntoParam<'a, INetFwProduct>>(&self, product: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), product.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<INetFwProduct> {
        let mut result__: <INetFwProduct as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<INetFwProduct>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwProducts {
    type Vtable = INetFwProducts_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39eb36e0_2097_40bd_8af2_63a13b525362);
}
impl ::core::convert::From<INetFwProducts> for ::windows::core::IUnknown {
    fn from(value: INetFwProducts) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwProducts> for ::windows::core::IUnknown {
    fn from(value: &INetFwProducts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwProducts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwProducts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwProducts> for super::super::System::Com::IDispatch {
    fn from(value: INetFwProducts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwProducts> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwProducts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwProducts {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwProducts {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProducts_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, product: ::windows::core::RawPtr, registration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, product: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwProfile(pub ::windows::core::IUnknown);
impl INetFwProfile {
    pub unsafe fn Type(&self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__: <NET_FW_PROFILE_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    pub unsafe fn FirewallEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFirewallEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn ExceptionsNotAllowed(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetExceptionsNotAllowed(&self, notallowed: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(notallowed)).ok()
    }
    pub unsafe fn NotificationsDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNotificationsDisabled(&self, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(disabled)).ok()
    }
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(disabled)).ok()
    }
    pub unsafe fn RemoteAdminSettings(&self) -> ::windows::core::Result<INetFwRemoteAdminSettings> {
        let mut result__: <INetFwRemoteAdminSettings as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwRemoteAdminSettings>(result__)
    }
    pub unsafe fn IcmpSettings(&self) -> ::windows::core::Result<INetFwIcmpSettings> {
        let mut result__: <INetFwIcmpSettings as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwIcmpSettings>(result__)
    }
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows::core::Result<INetFwOpenPorts> {
        let mut result__: <INetFwOpenPorts as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwOpenPorts>(result__)
    }
    pub unsafe fn Services(&self) -> ::windows::core::Result<INetFwServices> {
        let mut result__: <INetFwServices as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwServices>(result__)
    }
    pub unsafe fn AuthorizedApplications(&self) -> ::windows::core::Result<INetFwAuthorizedApplications> {
        let mut result__: <INetFwAuthorizedApplications as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwAuthorizedApplications>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwProfile {
    type Vtable = INetFwProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x174a0dda_e9f9_449d_993b_21ab667ca456);
}
impl ::core::convert::From<INetFwProfile> for ::windows::core::IUnknown {
    fn from(value: INetFwProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwProfile> for ::windows::core::IUnknown {
    fn from(value: &INetFwProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwProfile> for super::super::System::Com::IDispatch {
    fn from(value: INetFwProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwProfile> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwProfile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwProfile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notallowed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, notallowed: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteadminsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmpsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, services: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, apps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwRemoteAdminSettings(pub ::windows::core::IUnknown);
impl INetFwRemoteAdminSettings {
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwRemoteAdminSettings {
    type Vtable = INetFwRemoteAdminSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4becddf_6f73_4a83_b832_9c66874cd20e);
}
impl ::core::convert::From<INetFwRemoteAdminSettings> for ::windows::core::IUnknown {
    fn from(value: INetFwRemoteAdminSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwRemoteAdminSettings> for ::windows::core::IUnknown {
    fn from(value: &INetFwRemoteAdminSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwRemoteAdminSettings> for super::super::System::Com::IDispatch {
    fn from(value: INetFwRemoteAdminSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwRemoteAdminSettings> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwRemoteAdminSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwRule(pub ::windows::core::IUnknown);
impl INetFwRule {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: <NET_FW_RULE_DIRECTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(action)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwRule {
    type Vtable = INetFwRule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf230d27_baba_4e42_aced_f524f22cfce2);
}
impl ::core::convert::From<INetFwRule> for ::windows::core::IUnknown {
    fn from(value: INetFwRule) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwRule> for ::windows::core::IUnknown {
    fn from(value: &INetFwRule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwRule> for super::super::System::Com::IDispatch {
    fn from(value: INetFwRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwRule> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmptypesandcodes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaces: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacetypes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwRule2(pub ::windows::core::IUnknown);
impl INetFwRule2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: <NET_FW_RULE_DIRECTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(loptions)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwRule2 {
    type Vtable = INetFwRule2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c27c8da_189b_4dde_89f7_8b39a316782c);
}
impl ::core::convert::From<INetFwRule2> for ::windows::core::IUnknown {
    fn from(value: INetFwRule2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwRule2> for ::windows::core::IUnknown {
    fn from(value: &INetFwRule2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<INetFwRule2> for INetFwRule {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRule2> for INetFwRule {
    fn from(value: &INetFwRule2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetFwRule> for INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, INetFwRule> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetFwRule> for &INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, INetFwRule> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwRule2> for super::super::System::Com::IDispatch {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwRule2> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwRule2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmptypesandcodes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaces: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacetypes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loptions: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwRule3(pub ::windows::core::IUnknown);
impl INetFwRule3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: <NET_FW_RULE_DIRECTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(action)).ok()
    }
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAppPackageId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAppPackageId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszpackageid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), wszpackageid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserOwner(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserOwner<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserowner: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), wszuserowner.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserAuthorizedList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteUserAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteUserAuthorizedList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteMachineAuthorizedList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn SecureFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(loptions)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetFwRule3 {
    type Vtable = INetFwRule3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb21563ff_d696_4222_ab46_4e89b73ab34a);
}
impl ::core::convert::From<INetFwRule3> for ::windows::core::IUnknown {
    fn from(value: INetFwRule3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwRule3> for ::windows::core::IUnknown {
    fn from(value: &INetFwRule3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<INetFwRule3> for INetFwRule2 {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRule3> for INetFwRule2 {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetFwRule2> for INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, INetFwRule2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetFwRule2> for &INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, INetFwRule2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INetFwRule3> for INetFwRule {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRule3> for INetFwRule {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetFwRule> for INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, INetFwRule> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetFwRule> for &INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, INetFwRule> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwRule3> for super::super::System::Com::IDispatch {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwRule3> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmptypesandcodes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaces: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacetypes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiletypesbitmask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loptions: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpackageid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserowner: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserauthlist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserauthlist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserauthlist: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loptions: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwRules(pub ::windows::core::IUnknown);
impl INetFwRules {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, INetFwRule>>(&self, rule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), rule.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<INetFwRule> {
        let mut result__: <INetFwRule as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<INetFwRule>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwRules {
    type Vtable = INetFwRules_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c4c6277_5027_441e_afae_ca1f542da009);
}
impl ::core::convert::From<INetFwRules> for ::windows::core::IUnknown {
    fn from(value: INetFwRules) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwRules> for ::windows::core::IUnknown {
    fn from(value: &INetFwRules) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRules {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwRules {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwRules> for super::super::System::Com::IDispatch {
    fn from(value: INetFwRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwRules> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwRules {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwRules {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRules_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwService(pub ::windows::core::IUnknown);
impl INetFwService {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<NET_FW_SERVICE_TYPE> {
        let mut result__: <NET_FW_SERVICE_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SERVICE_TYPE>(result__)
    }
    pub unsafe fn Customized(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows::core::Result<INetFwOpenPorts> {
        let mut result__: <INetFwOpenPorts as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwOpenPorts>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwService {
    type Vtable = INetFwService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79fd57c8_908e_4a36_9888_d5b3f0a444cf);
}
impl ::core::convert::From<INetFwService> for ::windows::core::IUnknown {
    fn from(value: INetFwService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwService> for ::windows::core::IUnknown {
    fn from(value: &INetFwService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwService> for super::super::System::Com::IDispatch {
    fn from(value: INetFwService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwService> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwService {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwService {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customized: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwServiceRestriction(pub ::windows::core::IUnknown);
impl INetFwServiceRestriction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestrictService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, appname: Param1, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), servicename.into_param().abi(), appname.into_param().abi(), ::core::mem::transmute(restrictservice), ::core::mem::transmute(servicesidrestricted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceRestricted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, appname: Param1) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), servicename.into_param().abi(), appname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Rules(&self) -> ::windows::core::Result<INetFwRules> {
        let mut result__: <INetFwRules as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetFwRules>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwServiceRestriction {
    type Vtable = INetFwServiceRestriction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8267bbe3_f890_491c_b7b6_2db1ef0e5d2b);
}
impl ::core::convert::From<INetFwServiceRestriction> for ::windows::core::IUnknown {
    fn from(value: INetFwServiceRestriction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwServiceRestriction> for ::windows::core::IUnknown {
    fn from(value: &INetFwServiceRestriction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwServiceRestriction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwServiceRestriction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwServiceRestriction> for super::super::System::Com::IDispatch {
    fn from(value: INetFwServiceRestriction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwServiceRestriction> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwServiceRestriction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwServiceRestriction {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwServiceRestriction {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestriction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetFwServices(pub ::windows::core::IUnknown);
impl INetFwServices {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> ::windows::core::Result<INetFwService> {
        let mut result__: <INetFwService as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(svctype), &mut result__).from_abi::<INetFwService>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetFwServices {
    type Vtable = INetFwServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79649bb4_903e_421b_94c9_79848e79f6ee);
}
impl ::core::convert::From<INetFwServices> for ::windows::core::IUnknown {
    fn from(value: INetFwServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetFwServices> for ::windows::core::IUnknown {
    fn from(value: &INetFwServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetFwServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetFwServices> for super::super::System::Com::IDispatch {
    fn from(value: INetFwServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetFwServices> for super::super::System::Com::IDispatch {
    fn from(value: &INetFwServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetFwServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetFwServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingConfiguration(pub ::windows::core::IUnknown);
impl INetSharingConfiguration {
    pub unsafe fn SharingEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SharingConnectionType(&self) -> ::windows::core::Result<SHARINGCONNECTIONTYPE> {
        let mut result__: <SHARINGCONNECTIONTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SHARINGCONNECTIONTYPE>(result__)
    }
    pub unsafe fn DisableSharing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    pub unsafe fn InternetFirewallEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn DisableInternetFirewall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnableInternetFirewall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPortMappingCollection> {
        let mut result__: <INetSharingPortMappingCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<INetSharingPortMappingCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPortMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: Param5, etargettype: ICS_TARGETTYPE) -> ::windows::core::Result<INetSharingPortMapping> {
        let mut result__: <INetSharingPortMapping as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(ucipprotocol), ::core::mem::transmute(usexternalport), ::core::mem::transmute(usinternalport), ::core::mem::transmute(dwoptions), bstrtargetnameoripaddress.into_param().abi(), ::core::mem::transmute(etargettype), &mut result__).from_abi::<INetSharingPortMapping>(result__)
    }
    pub unsafe fn RemovePortMapping<'a, Param0: ::windows::core::IntoParam<'a, INetSharingPortMapping>>(&self, pmapping: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pmapping.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for INetSharingConfiguration {
    type Vtable = INetSharingConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b6_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<INetSharingConfiguration> for ::windows::core::IUnknown {
    fn from(value: INetSharingConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingConfiguration> for ::windows::core::IUnknown {
    fn from(value: &INetSharingConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingConfiguration> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingConfiguration> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmapping: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingEveryConnectionCollection(pub ::windows::core::IUnknown);
impl INetSharingEveryConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetSharingEveryConnectionCollection {
    type Vtable = INetSharingEveryConnectionCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33c4643c_7811_46fa_a89a_768597bd7223);
}
impl ::core::convert::From<INetSharingEveryConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingEveryConnectionCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingEveryConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingEveryConnectionCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingEveryConnectionCollection> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingEveryConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingEveryConnectionCollection> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingEveryConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingEveryConnectionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingManager(pub ::windows::core::IUnknown);
impl INetSharingManager {
    pub unsafe fn SharingInstalled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPublicConnectionCollection> {
        let mut result__: <INetSharingPublicConnectionCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<INetSharingPublicConnectionCollection>(result__)
    }
    pub unsafe fn EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPrivateConnectionCollection> {
        let mut result__: <INetSharingPrivateConnectionCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<INetSharingPrivateConnectionCollection>(result__)
    }
    pub unsafe fn INetSharingConfigurationForINetConnection<'a, Param0: ::windows::core::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows::core::Result<INetSharingConfiguration> {
        let mut result__: <INetSharingConfiguration as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pnetconnection.into_param().abi(), &mut result__).from_abi::<INetSharingConfiguration>(result__)
    }
    pub unsafe fn EnumEveryConnection(&self) -> ::windows::core::Result<INetSharingEveryConnectionCollection> {
        let mut result__: <INetSharingEveryConnectionCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetSharingEveryConnectionCollection>(result__)
    }
    pub unsafe fn NetConnectionProps<'a, Param0: ::windows::core::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows::core::Result<INetConnectionProps> {
        let mut result__: <INetConnectionProps as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pnetconnection.into_param().abi(), &mut result__).from_abi::<INetConnectionProps>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetSharingManager {
    type Vtable = INetSharingManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b7_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<INetSharingManager> for ::windows::core::IUnknown {
    fn from(value: INetSharingManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingManager> for ::windows::core::IUnknown {
    fn from(value: &INetSharingManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingManager> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingManager> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbinstalled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnetconnection: ::windows::core::RawPtr, ppnetsharingconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnetconnection: ::windows::core::RawPtr, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingPortMapping(pub ::windows::core::IUnknown);
impl INetSharingPortMapping {
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Properties(&self) -> ::windows::core::Result<INetSharingPortMappingProps> {
        let mut result__: <INetSharingPortMappingProps as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INetSharingPortMappingProps>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetSharingPortMapping {
    type Vtable = INetSharingPortMapping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b1_1cd3_11d1_b1c5_00805fc1270e);
}
impl ::core::convert::From<INetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: INetSharingPortMapping) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPortMapping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingPortMapping> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingPortMapping> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppnspmp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingPortMappingCollection(pub ::windows::core::IUnknown);
impl INetSharingPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetSharingPortMappingCollection {
    type Vtable = INetSharingPortMappingCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02e4a2de_da20_4e34_89c8_ac22275a010b);
}
impl ::core::convert::From<INetSharingPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingPortMappingCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPortMappingCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingPortMappingCollection> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingPortMappingCollection> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingPortMappingProps(pub ::windows::core::IUnknown);
impl INetSharingPortMappingProps {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn IPProtocol(&self) -> ::windows::core::Result<u8> {
        let mut result__: <u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Options(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetSharingPortMappingProps {
    type Vtable = INetSharingPortMappingProps_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24b7e9b5_e38f_4685_851b_00892cf5f940);
}
impl ::core::convert::From<INetSharingPortMappingProps> for ::windows::core::IUnknown {
    fn from(value: INetSharingPortMappingProps) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingPortMappingProps> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPortMappingProps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingPortMappingProps> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingPortMappingProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingPortMappingProps> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingPortMappingProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingProps_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucipprot: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pusport: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pusport: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwoptions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrtargetname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrtargetipaddress: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbool: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingPrivateConnectionCollection(pub ::windows::core::IUnknown);
impl INetSharingPrivateConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetSharingPrivateConnectionCollection {
    type Vtable = INetSharingPrivateConnectionCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ae69e0_4409_402a_a2cb_e965c727f840);
}
impl ::core::convert::From<INetSharingPrivateConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingPrivateConnectionCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingPrivateConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPrivateConnectionCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingPrivateConnectionCollection> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingPrivateConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingPrivateConnectionCollection> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingPrivateConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPrivateConnectionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetSharingPublicConnectionCollection(pub ::windows::core::IUnknown);
impl INetSharingPublicConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for INetSharingPublicConnectionCollection {
    type Vtable = INetSharingPublicConnectionCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d7a6355_f372_4971_a149_bfc927be762a);
}
impl ::core::convert::From<INetSharingPublicConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingPublicConnectionCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetSharingPublicConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPublicConnectionCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetSharingPublicConnectionCollection> for super::super::System::Com::IDispatch {
    fn from(value: INetSharingPublicConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetSharingPublicConnectionCollection> for super::super::System::Com::IDispatch {
    fn from(value: &INetSharingPublicConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPublicConnectionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStaticPortMapping(pub ::windows::core::IUnknown);
impl IStaticPortMapping {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExternalIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InternalClient(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditInternalClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrinternalclient.into_param().abi()).ok()
    }
    pub unsafe fn Enable(&self, vb: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(vb)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(linternalport)).ok()
    }
}
unsafe impl ::windows::core::Interface for IStaticPortMapping {
    type Vtable = IStaticPortMapping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f10711f_729b_41e5_93b8_f21d0f818df1);
}
impl ::core::convert::From<IStaticPortMapping> for ::windows::core::IUnknown {
    fn from(value: IStaticPortMapping) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStaticPortMapping> for ::windows::core::IUnknown {
    fn from(value: &IStaticPortMapping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStaticPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStaticPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStaticPortMapping> for super::super::System::Com::IDispatch {
    fn from(value: IStaticPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStaticPortMapping> for super::super::System::Com::IDispatch {
    fn from(value: &IStaticPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IStaticPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IStaticPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vb: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, linternalport: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStaticPortMappingCollection(pub ::windows::core::IUnknown);
impl IStaticPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows::core::Result<IStaticPortMapping> {
        let mut result__: <IStaticPortMapping as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), &mut result__).from_abi::<IStaticPortMapping>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1, linternalport: i32, bstrinternalclient: Param3, benabled: i16, bstrdescription: Param5) -> ::windows::core::Result<IStaticPortMapping> {
        let mut result__: <IStaticPortMapping as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::core::mem::transmute(benabled), bstrdescription.into_param().abi(), &mut result__).from_abi::<IStaticPortMapping>(result__)
    }
}
unsafe impl ::windows::core::Interface for IStaticPortMappingCollection {
    type Vtable = IStaticPortMappingCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd1f3e77_66d6_4664_82c7_36dbb641d0f1);
}
impl ::core::convert::From<IStaticPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: IStaticPortMappingCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStaticPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: &IStaticPortMappingCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStaticPortMappingCollection> for super::super::System::Com::IDispatch {
    fn from(value: IStaticPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStaticPortMappingCollection> for super::super::System::Com::IDispatch {
    fn from(value: &IStaticPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUPnPNAT(pub ::windows::core::IUnknown);
impl IUPnPNAT {
    pub unsafe fn StaticPortMappingCollection(&self) -> ::windows::core::Result<IStaticPortMappingCollection> {
        let mut result__: <IStaticPortMappingCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IStaticPortMappingCollection>(result__)
    }
    pub unsafe fn DynamicPortMappingCollection(&self) -> ::windows::core::Result<IDynamicPortMappingCollection> {
        let mut result__: <IDynamicPortMappingCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDynamicPortMappingCollection>(result__)
    }
    pub unsafe fn NATEventManager(&self) -> ::windows::core::Result<INATEventManager> {
        let mut result__: <INATEventManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<INATEventManager>(result__)
    }
}
unsafe impl ::windows::core::Interface for IUPnPNAT {
    type Vtable = IUPnPNAT_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb171c812_cc76_485a_94d8_b6b3a2794e99);
}
impl ::core::convert::From<IUPnPNAT> for ::windows::core::IUnknown {
    fn from(value: IUPnPNAT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUPnPNAT> for ::windows::core::IUnknown {
    fn from(value: &IUPnPNAT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPNAT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUPnPNAT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPNAT> for super::super::System::Com::IDispatch {
    fn from(value: IUPnPNAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPNAT> for super::super::System::Com::IDispatch {
    fn from(value: &IUPnPNAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IUPnPNAT {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IUPnPNAT {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPNAT_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppspms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdpms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppnem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCONMGR_ENUM_FLAGS(pub i32);
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(0i32);
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(1i32);
impl ::core::convert::From<i32> for NETCONMGR_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETCONMGR_ENUM_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCONUI_CONNECT_FLAGS(pub i32);
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(0i32);
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(1i32);
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(2i32);
impl ::core::convert::From<i32> for NETCONUI_CONNECT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETCONUI_CONNECT_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCON_CHARACTERISTIC_FLAGS(pub i32);
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(0i32);
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1i32);
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2i32);
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4i32);
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8i32);
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32i32);
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(64i32);
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(128i32);
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(256i32);
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(512i32);
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1024i32);
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2048i32);
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4096i32);
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8192i32);
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(16384i32);
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32768i32);
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(65536i32);
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(131072i32);
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(262144i32);
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(983040i32);
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(15728640i32);
impl ::core::convert::From<i32> for NETCON_CHARACTERISTIC_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETCON_CHARACTERISTIC_FLAGS {
    type Abi = Self;
}
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCON_MEDIATYPE(pub i32);
pub const NCM_NONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(0i32);
pub const NCM_DIRECT: NETCON_MEDIATYPE = NETCON_MEDIATYPE(1i32);
pub const NCM_ISDN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(2i32);
pub const NCM_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(3i32);
pub const NCM_PHONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(4i32);
pub const NCM_TUNNEL: NETCON_MEDIATYPE = NETCON_MEDIATYPE(5i32);
pub const NCM_PPPOE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(6i32);
pub const NCM_BRIDGE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(7i32);
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(8i32);
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = NETCON_MEDIATYPE(9i32);
impl ::core::convert::From<i32> for NETCON_MEDIATYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETCON_MEDIATYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NETCON_PROPERTIES {
    pub guidId: ::windows::core::GUID,
    pub pszwName: super::super::Foundation::PWSTR,
    pub pszwDeviceName: super::super::Foundation::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: ::windows::core::GUID,
    pub clsidUiObject: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETCON_PROPERTIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NETCON_PROPERTIES").field("guidId", &self.guidId).field("pszwName", &self.pszwName).field("pszwDeviceName", &self.pszwDeviceName).field("Status", &self.Status).field("MediaType", &self.MediaType).field("dwCharacter", &self.dwCharacter).field("clsidThisObject", &self.clsidThisObject).field("clsidUiObject", &self.clsidUiObject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETCON_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.pszwName == other.pszwName && self.pszwDeviceName == other.pszwDeviceName && self.Status == other.Status && self.MediaType == other.MediaType && self.dwCharacter == other.dwCharacter && self.clsidThisObject == other.clsidThisObject && self.clsidUiObject == other.clsidUiObject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETCON_PROPERTIES {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCON_STATUS(pub i32);
pub const NCS_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(0i32);
pub const NCS_CONNECTING: NETCON_STATUS = NETCON_STATUS(1i32);
pub const NCS_CONNECTED: NETCON_STATUS = NETCON_STATUS(2i32);
pub const NCS_DISCONNECTING: NETCON_STATUS = NETCON_STATUS(3i32);
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = NETCON_STATUS(4i32);
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = NETCON_STATUS(5i32);
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = NETCON_STATUS(6i32);
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(7i32);
pub const NCS_AUTHENTICATING: NETCON_STATUS = NETCON_STATUS(8i32);
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = NETCON_STATUS(9i32);
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = NETCON_STATUS(10i32);
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = NETCON_STATUS(11i32);
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = NETCON_STATUS(12i32);
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = NETCON_STATUS(13i32);
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = NETCON_STATUS(14i32);
pub const NCS_CONNECT_FAILED: NETCON_STATUS = NETCON_STATUS(15i32);
impl ::core::convert::From<i32> for NETCON_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETCON_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCON_TYPE(pub i32);
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = NETCON_TYPE(0i32);
pub const NCT_INBOUND: NETCON_TYPE = NETCON_TYPE(1i32);
pub const NCT_INTERNET: NETCON_TYPE = NETCON_TYPE(2i32);
pub const NCT_LAN: NETCON_TYPE = NETCON_TYPE(3i32);
pub const NCT_PHONE: NETCON_TYPE = NETCON_TYPE(4i32);
pub const NCT_TUNNEL: NETCON_TYPE = NETCON_TYPE(5i32);
pub const NCT_BRIDGE: NETCON_TYPE = NETCON_TYPE(6i32);
impl ::core::convert::From<i32> for NETCON_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETCON_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETISO_ERROR_TYPE(pub i32);
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(0i32);
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(1i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(2i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(3i32);
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(4i32);
impl ::core::convert::From<i32> for NETISO_ERROR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETISO_ERROR_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETISO_FLAG(pub i32);
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = NETISO_FLAG(1i32);
pub const NETISO_FLAG_MAX: NETISO_FLAG = NETISO_FLAG(2i32);
impl ::core::convert::From<i32> for NETISO_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NETISO_FLAG {
    type Abi = Self;
}
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_ACTION(pub i32);
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = NET_FW_ACTION(0i32);
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = NET_FW_ACTION(1i32);
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = NET_FW_ACTION(2i32);
impl ::core::convert::From<i32> for NET_FW_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_ACTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_AUTHENTICATE_TYPE(pub i32);
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(0i32);
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(1i32);
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(2i32);
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(3i32);
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(4i32);
impl ::core::convert::From<i32> for NET_FW_AUTHENTICATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_AUTHENTICATE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(pub i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(0i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(1i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(3i32);
impl ::core::convert::From<i32> for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_EDGE_TRAVERSAL_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_IP_PROTOCOL(pub i32);
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(6i32);
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(17i32);
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(256i32);
impl ::core::convert::From<i32> for NET_FW_IP_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_IP_PROTOCOL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_IP_VERSION(pub i32);
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = NET_FW_IP_VERSION(0i32);
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = NET_FW_IP_VERSION(1i32);
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = NET_FW_IP_VERSION(2i32);
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = NET_FW_IP_VERSION(3i32);
impl ::core::convert::From<i32> for NET_FW_IP_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_IP_VERSION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_MODIFY_STATE(pub i32);
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(0i32);
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(1i32);
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(2i32);
impl ::core::convert::From<i32> for NET_FW_MODIFY_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_MODIFY_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_POLICY_TYPE(pub i32);
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(0i32);
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(1i32);
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(2i32);
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(3i32);
impl ::core::convert::From<i32> for NET_FW_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_POLICY_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_PROFILE_TYPE(pub i32);
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(0i32);
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(1i32);
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(2i32);
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(3i32);
impl ::core::convert::From<i32> for NET_FW_PROFILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_PROFILE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_PROFILE_TYPE2(pub i32);
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(1i32);
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2i32);
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(4i32);
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2147483647i32);
impl ::core::convert::From<i32> for NET_FW_PROFILE_TYPE2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_PROFILE_TYPE2 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_RULE_CATEGORY(pub i32);
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(0i32);
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(1i32);
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(2i32);
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(3i32);
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(4i32);
impl ::core::convert::From<i32> for NET_FW_RULE_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_RULE_CATEGORY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_RULE_DIRECTION(pub i32);
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(1i32);
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(2i32);
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(3i32);
impl ::core::convert::From<i32> for NET_FW_RULE_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_RULE_DIRECTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_SCOPE(pub i32);
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = NET_FW_SCOPE(0i32);
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = NET_FW_SCOPE(1i32);
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = NET_FW_SCOPE(2i32);
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = NET_FW_SCOPE(3i32);
impl ::core::convert::From<i32> for NET_FW_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_SCOPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_SERVICE_TYPE(pub i32);
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(0i32);
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(1i32);
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(2i32);
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(3i32);
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(4i32);
impl ::core::convert::From<i32> for NET_FW_SERVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NET_FW_SERVICE_TYPE {
    type Abi = Self;
}
pub const NetFwAuthorizedApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(wszservername: Param0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername: super::super::Foundation::PWSTR, netisoerror: *mut NETISO_ERROR_TYPE) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.into_param().abi(), ::core::mem::transmute(netisoerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationEnumAppContainers(::core::mem::transmute(flags), ::core::mem::transmute(pdwnumpublicappcs), ::core::mem::transmute(pppublicappcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationFreeAppContainers(::core::mem::transmute(ppublicappcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationGetAppContainerConfig(::core::mem::transmute(pdwnumpublicappcs), ::core::mem::transmute(appcontainersids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: *const ::core::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationRegisterForAppContainerChanges(::core::mem::transmute(flags), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(registrationobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationSetAppContainerConfig(::core::mem::transmute(dwnumpublicappcs), ::core::mem::transmute(appcontainersids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(applicationcontainersid: Param0, packagefullname: Param1, packagefolder: Param2, displayname: Param3, bbinariesfullycomputed: Param4, binaries: *const super::super::Foundation::PWSTR, binariescount: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid: super::super::Foundation::PSID, packagefullname: super::super::Foundation::PWSTR, packagefolder: super::super::Foundation::PWSTR, displayname: super::super::Foundation::PWSTR, bbinariesfullycomputed: super::super::Foundation::BOOL, binaries: *const super::super::Foundation::PWSTR, binariescount: u32) -> ::windows::core::HRESULT;
        }
        NetworkIsolationSetupAppContainerBinaries(applicationcontainersid.into_param().abi(), packagefullname.into_param().abi(), packagefolder.into_param().abi(), displayname.into_param().abi(), bbinariesfullycomputed.into_param().abi(), ::core::mem::transmute(binaries), ::core::mem::transmute(binariescount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(registrationobject: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(NetworkIsolationUnregisterForAppContainerChanges(registrationobject.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PAC_CHANGES_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddress: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows::core::GUID) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows::core::GUID, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = ::core::option::Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows::core::GUID, updatedaddresses: super::super::Foundation::PWSTR, append: super::super::Foundation::BOOL) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PNETISO_EDP_ID_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, wszenterpriseid: super::super::Foundation::PWSTR, dwerr: u32)>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHARINGCONNECTIONTYPE(pub i32);
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(0i32);
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(1i32);
impl ::core::convert::From<i32> for SHARINGCONNECTIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SHARINGCONNECTIONTYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(pub i32);
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(0i32);
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(1i32);
impl ::core::convert::From<i32> for SHARINGCONNECTION_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SHARINGCONNECTION_ENUM_FLAGS {
    type Abi = Self;
}
pub const S_OBJECT_NO_LONGER_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(2i32 as _);
pub const UPnPNAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows::core::GUID,
    pub keyword: super::super::Foundation::PWSTR,
    pub flags: u32,
    pub addresses: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS0").field("id", &self.id).field("keyword", &self.keyword).field("flags", &self.flags).field("addresses", &self.addresses).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.keyword == other.keyword && self.flags == other.flags && self.addresses == other.addresses
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: _tag_FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0").field("dynamicKeywordAddress", &self.dynamicKeywordAddress).field("next", &self.next).field("schemaVersion", &self.schemaVersion).field("originType", &self.originType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        self.dynamicKeywordAddress == other.dynamicKeywordAddress && self.next == other.next && self.schemaVersion == other.schemaVersion && self.originType == other.originType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(pub i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(2i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(3i32);
impl ::core::convert::From<i32> for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(pub i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(1i32);
impl ::core::convert::From<i32> for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(pub i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(0i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(2i32);
impl ::core::convert::From<i32> for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    type Abi = Self;
}
