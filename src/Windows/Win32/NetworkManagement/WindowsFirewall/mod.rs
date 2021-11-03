#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ICS_TARGETTYPE(pub i32);
pub const ICSTT_NAME: ICS_TARGETTYPE = ICS_TARGETTYPE(0i32);
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = ICS_TARGETTYPE(1i32);
impl ::std::convert::From<i32> for ICS_TARGETTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ICS_TARGETTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDynamicPortMapping(::windows::runtime::IUnknown);
impl IDynamicPortMapping {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ExternalIPAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteHost(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn ExternalPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Protocol(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn InternalPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn InternalClient(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn LeaseDuration(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(lleasedurationdesired), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn EditInternalClient<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), bstrinternalclient.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enable(&self, vb: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(vb)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn EditDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(linternalport)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDynamicPortMapping {
    type Vtable = IDynamicPortMapping_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1338507906, 9142, 17272, [154, 39, 205, 143, 23, 201, 64, 12]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IDynamicPortMapping> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IDynamicPortMapping) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IDynamicPortMapping> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IDynamicPortMapping) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IDynamicPortMapping {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IDynamicPortMapping {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrinternalclient: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vb: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linternalport: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDynamicPortMappingCollection(::windows::runtime::IUnknown);
impl IDynamicPortMappingCollection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows::runtime::Result<IDynamicPortMapping> {
        let mut result__: <IDynamicPortMapping as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::std::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), &mut result__).from_abi::<IDynamicPortMapping>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::std::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrremotehost: Param0,
        lexternalport: i32,
        bstrprotocol: Param2,
        linternalport: i32,
        bstrinternalclient: Param4,
        benabled: i16,
        bstrdescription: Param6,
        lleaseduration: i32,
    ) -> ::windows::runtime::Result<IDynamicPortMapping> {
        let mut result__: <IDynamicPortMapping as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            bstrremotehost.into_param().abi(),
            ::std::mem::transmute(lexternalport),
            bstrprotocol.into_param().abi(),
            ::std::mem::transmute(linternalport),
            bstrinternalclient.into_param().abi(),
            ::std::mem::transmute(benabled),
            bstrdescription.into_param().abi(),
            ::std::mem::transmute(lleaseduration),
            &mut result__,
        )
        .from_abi::<IDynamicPortMapping>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDynamicPortMappingCollection {
    type Vtable = IDynamicPortMappingCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3054362639, 5486, 20109, [158, 193, 58, 35, 66, 193, 8, 153]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IDynamicPortMappingCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IDynamicPortMappingCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IDynamicPortMappingCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IDynamicPortMappingCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrremotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrremotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrremotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        lexternalport: i32,
        bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        linternalport: i32,
        bstrinternalclient: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        benabled: i16,
        bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        lleaseduration: i32,
        ppdpm: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IEnumNetConnection(::windows::runtime::IUnknown);
impl IEnumNetConnection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<INetConnection>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetConnection> {
        let mut result__: <IEnumNetConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetConnection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetConnection {
    type Vtable = IEnumNetConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226080, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IEnumNetSharingEveryConnection(::windows::runtime::IUnknown);
impl IEnumNetSharingEveryConnection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgvar), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetSharingEveryConnection> {
        let mut result__: <IEnumNetSharingEveryConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingEveryConnection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetSharingEveryConnection {
    type Vtable = IEnumNetSharingEveryConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226104, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingEveryConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IEnumNetSharingPortMapping(::windows::runtime::IUnknown);
impl IEnumNetSharingPortMapping {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgvar), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetSharingPortMapping> {
        let mut result__: <IEnumNetSharingPortMapping as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingPortMapping>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetSharingPortMapping {
    type Vtable = IEnumNetSharingPortMapping_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226096, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IEnumNetSharingPrivateConnection(::windows::runtime::IUnknown);
impl IEnumNetSharingPrivateConnection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgvar), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetSharingPrivateConnection> {
        let mut result__: <IEnumNetSharingPrivateConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingPrivateConnection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetSharingPrivateConnection {
    type Vtable = IEnumNetSharingPrivateConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226101, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPrivateConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IEnumNetSharingPublicConnection(::windows::runtime::IUnknown);
impl IEnumNetSharingPublicConnection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgvar), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetSharingPublicConnection> {
        let mut result__: <IEnumNetSharingPublicConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetSharingPublicConnection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetSharingPublicConnection {
    type Vtable = IEnumNetSharingPublicConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226100, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPublicConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgvar: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INATEventManager(::windows::runtime::IUnknown);
impl INATEventManager {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetExternalIPAddressCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetNumberOfEntriesCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INATEventManager {
    type Vtable = INATEventManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1649137032, 36960, 16649, [176, 176, 26, 219, 188, 172, 50, 223]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INATEventManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INATEventManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INATEventManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INATEventManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INATEventManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INATEventManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INATEventManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INATExternalIPAddressCallback(::windows::runtime::IUnknown);
impl INATExternalIPAddressCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn NewExternalIPAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnewexternalipaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrnewexternalipaddress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INATExternalIPAddressCallback {
    type Vtable = INATExternalIPAddressCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2621531968, 41806, 17519, [186, 6, 171, 208, 76, 49, 73, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATExternalIPAddressCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnewexternalipaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INATNumberOfEntriesCallback(::windows::runtime::IUnknown);
impl INATNumberOfEntriesCallback {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lnewnumberofentries)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INATNumberOfEntriesCallback {
    type Vtable = INATNumberOfEntriesCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3359246964, 37358, 16822, [182, 122, 103, 224, 240, 11, 189, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATNumberOfEntriesCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnewnumberofentries: i32) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for INET_FIREWALL_AC_BINARIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_FIREWALL_AC_BINARIES").field("count", &self.count).field("binaries", &self.binaries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INET_FIREWALL_AC_BINARIES {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.binaries == other.binaries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_AC_BINARIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for INET_FIREWALL_AC_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_FIREWALL_AC_CAPABILITIES").field("count", &self.count).field("capabilities", &self.capabilities).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for INET_FIREWALL_AC_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.capabilities == other.capabilities
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_AC_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
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
impl ::std::default::Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for INET_FIREWALL_AC_CHANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_AC_CHANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for INET_FIREWALL_AC_CHANGE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_AC_CHANGE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(pub i32);
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(0i32);
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(1i32);
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(3i32);
impl ::std::convert::From<i32> for INET_FIREWALL_AC_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_AC_CHANGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(pub i32);
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(0i32);
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(1i32);
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(2i32);
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(4i32);
impl ::std::convert::From<i32> for INET_FIREWALL_AC_CREATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_AC_CREATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
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
impl ::std::default::Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for INET_FIREWALL_APP_CONTAINER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INET_FIREWALL_APP_CONTAINER")
            .field("appContainerSid", &self.appContainerSid)
            .field("userSid", &self.userSid)
            .field("appContainerName", &self.appContainerName)
            .field("displayName", &self.displayName)
            .field("description", &self.description)
            .field("capabilities", &self.capabilities)
            .field("binaries", &self.binaries)
            .field("workingDirectory", &self.workingDirectory)
            .field("packageFullName", &self.packageFullName)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for INET_FIREWALL_APP_CONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.appContainerSid == other.appContainerSid && self.userSid == other.userSid && self.appContainerName == other.appContainerName && self.displayName == other.displayName && self.description == other.description && self.capabilities == other.capabilities && self.binaries == other.binaries && self.workingDirectory == other.workingDirectory && self.packageFullName == other.packageFullName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for INET_FIREWALL_APP_CONTAINER {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetConnection(::windows::runtime::IUnknown);
impl INetConnection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Connect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Duplicate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwduplicatename: Param0) -> ::windows::runtime::Result<INetConnection> {
        let mut result__: <INetConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszwduplicatename.into_param().abi(), &mut result__).from_abi::<INetConnection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<*mut NETCON_PROPERTIES> {
        let mut result__: <*mut NETCON_PROPERTIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut NETCON_PROPERTIES>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn GetUiObjectClassId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Rename<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwnewname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszwnewname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetConnection {
    type Vtable = INetConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226081, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszwduplicatename: super::super::Foundation::PWSTR, ppcon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetConnectionConnectUi(::windows::runtime::IUnknown);
impl INetConnectionConnectUi {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetConnection<'a, Param0: ::windows::runtime::IntoParam<'a, INetConnection>>(&self, pcon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Disconnect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetConnectionConnectUi {
    type Vtable = INetConnectionConnectUi_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226083, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionConnectUi_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetConnectionManager(::windows::runtime::IUnknown);
impl INetConnectionManager {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows::runtime::Result<IEnumNetConnection> {
        let mut result__: <IEnumNetConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), &mut result__).from_abi::<IEnumNetConnection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetConnectionManager {
    type Vtable = INetConnectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226082, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetConnectionProps(::windows::runtime::IUnknown);
impl INetConnectionProps {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Guid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn DeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<NETCON_STATUS> {
        let mut result__: <NETCON_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NETCON_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn MediaType(&self) -> ::windows::runtime::Result<NETCON_MEDIATYPE> {
        let mut result__: <NETCON_MEDIATYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NETCON_MEDIATYPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Characteristics(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetConnectionProps {
    type Vtable = INetConnectionProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4096228501, 52827, 17981, [129, 103, 86, 98, 217, 188, 170, 114]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetConnectionProps> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetConnectionProps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetConnectionProps> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetConnectionProps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetConnectionProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetConnectionProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrguid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdevicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut NETCON_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwAuthorizedApplication(::windows::runtime::IUnknown);
impl INetFwAuthorizedApplication {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ProcessImageFileName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetProcessImageFileName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn IpVersion(&self) -> ::windows::runtime::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Scope(&self) -> ::windows::runtime::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwAuthorizedApplication {
    type Vtable = INetFwAuthorizedApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3051769850, 49861, 17486, [163, 1, 251, 94, 0, 1, 128, 80]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwAuthorizedApplication> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwAuthorizedApplication) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwAuthorizedApplication> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwAuthorizedApplication) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwAuthorizedApplications(::windows::runtime::IUnknown);
impl INetFwAuthorizedApplications {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, INetFwAuthorizedApplication>>(&self, app: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), app.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::runtime::Result<INetFwAuthorizedApplication> {
        let mut result__: <INetFwAuthorizedApplication as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), imagefilename.into_param().abi(), &mut result__).from_abi::<INetFwAuthorizedApplication>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwAuthorizedApplications {
    type Vtable = INetFwAuthorizedApplications_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1682898258, 52473, 18540, [151, 162, 57, 243, 82, 87, 11, 48]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwAuthorizedApplications> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwAuthorizedApplications) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwAuthorizedApplications> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwAuthorizedApplications) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplications_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, app: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwIcmpSettings(::windows::runtime::IUnknown);
impl INetFwIcmpSettings {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowOutboundDestinationUnreachable(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowRedirect(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowRedirect(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowInboundEchoRequest(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowInboundEchoRequest(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowOutboundTimeExceeded(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowOutboundParameterProblem(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowOutboundParameterProblem(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowOutboundSourceQuench(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowOutboundSourceQuench(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowInboundRouterRequest(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowInboundRouterRequest(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowInboundTimestampRequest(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowInboundTimestampRequest(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowInboundMaskRequest(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowInboundMaskRequest(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAllowOutboundPacketTooBig(&self, allow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(allow)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwIcmpSettings {
    type Vtable = INetFwIcmpSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2787146542, 31965, 17002, [149, 30, 94, 28, 188, 90, 254, 173]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwIcmpSettings> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwIcmpSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwIcmpSettings> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwIcmpSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwIcmpSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwIcmpSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allow: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwMgr(::windows::runtime::IUnknown);
impl INetFwMgr {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn LocalPolicy(&self) -> ::windows::runtime::Result<INetFwPolicy> {
        let mut result__: <INetFwPolicy as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwPolicy>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn CurrentProfileType(&self) -> ::windows::runtime::Result<NET_FW_PROFILE_TYPE> {
        let mut result__: <NET_FW_PROFILE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn RestoreDefaults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsPortAllowed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: Param3, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), imagefilename.into_param().abi(), ::std::mem::transmute(ipversion), ::std::mem::transmute(portnumber), localaddress.into_param().abi(), ::std::mem::transmute(ipprotocol), ::std::mem::transmute(allowed), ::std::mem::transmute(restricted)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn IsIcmpTypeAllowed<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, ipversion: NET_FW_IP_VERSION, localaddress: Param1, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ipversion), localaddress.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(allowed), ::std::mem::transmute(restricted)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwMgr {
    type Vtable = INetFwMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4152986357, 51908, 17970, [162, 236, 218, 6, 229, 17, 26, 242]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwMgr> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwMgr) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwMgr> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwMgr) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localpolicy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, restricted: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: NET_FW_IP_VERSION, localaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, restricted: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwOpenPort(::windows::runtime::IUnknown);
impl INetFwOpenPort {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn IpVersion(&self) -> ::windows::runtime::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Protocol(&self) -> ::windows::runtime::Result<NET_FW_IP_PROTOCOL> {
        let mut result__: <NET_FW_IP_PROTOCOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_PROTOCOL>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ipprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Port(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetPort(&self, portnumber: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(portnumber)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Scope(&self) -> ::windows::runtime::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn BuiltIn(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwOpenPort {
    type Vtable = INetFwOpenPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3762830240, 18431, 19868, [166, 214, 119, 65, 208, 177, 149, 247]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwOpenPort> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwOpenPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwOpenPort> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwOpenPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwOpenPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwOpenPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumber: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumber: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, builtin: *mut i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwOpenPorts(::windows::runtime::IUnknown);
impl INetFwOpenPorts {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, INetFwOpenPort>>(&self, port: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), port.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(portnumber), ::std::mem::transmute(ipprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::runtime::Result<INetFwOpenPort> {
        let mut result__: <INetFwOpenPort as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(portnumber), ::std::mem::transmute(ipprotocol), &mut result__).from_abi::<INetFwOpenPort>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwOpenPorts {
    type Vtable = INetFwOpenPorts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3236550650, 57470, 17162, [177, 154, 9, 12, 232, 45, 146, 226]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwOpenPorts> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwOpenPorts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwOpenPorts> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwOpenPorts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwOpenPorts {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwOpenPorts {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPorts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, port: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwPolicy(::windows::runtime::IUnknown);
impl INetFwPolicy {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn CurrentProfile(&self) -> ::windows::runtime::Result<INetFwProfile> {
        let mut result__: <INetFwProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwProfile>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows::runtime::Result<INetFwProfile> {
        let mut result__: <INetFwProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<INetFwProfile>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwPolicy {
    type Vtable = INetFwPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3563922552, 39625, 16392, [157, 199, 85, 99, 206, 85, 54, 204]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwPolicy> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwPolicy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwPolicy> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwPolicy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwPolicy {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwPolicy2(::windows::runtime::IUnknown);
impl INetFwPolicy2 {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn CurrentProfileTypes(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetFirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetExcludedInterfaces<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), interfaces.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetBlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), ::std::mem::transmute(block)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetNotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), ::std::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), ::std::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Rules(&self) -> ::windows::runtime::Result<INetFwRules> {
        let mut result__: <INetFwRules as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwRules>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn ServiceRestriction(&self) -> ::windows::runtime::Result<INetFwServiceRestriction> {
        let mut result__: <INetFwServiceRestriction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwServiceRestriction>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn EnableRuleGroup<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1, enable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletypesbitmask), group.into_param().abi(), ::std::mem::transmute(enable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn IsRuleGroupEnabled<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletypesbitmask), group.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetDefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), ::std::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::runtime::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetDefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletype), ::std::mem::transmute(action)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn IsRuleGroupCurrentlyEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, group: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), group.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn LocalPolicyModifyState(&self) -> ::windows::runtime::Result<NET_FW_MODIFY_STATE> {
        let mut result__: <NET_FW_MODIFY_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_MODIFY_STATE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwPolicy2 {
    type Vtable = INetFwPolicy2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2553434183, 50801, 16756, [141, 129, 222, 252, 211, 240, 49, 134]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwPolicy2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwPolicy2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwPolicy2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwPolicy2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwPolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwPolicy2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rules: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicerestriction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: i32, group: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: i32, group: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, group: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwProduct(::windows::runtime::IUnknown);
impl INetFwProduct {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn RuleCategories(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetRuleCategories<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, rulecategories: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), rulecategories.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, displayname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), displayname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn PathToSignedProductExe(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwProduct {
    type Vtable = INetFwProduct_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904744089, 6388, 17803, [184, 146, 63, 252, 229, 224, 127, 117]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwProduct> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwProduct) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwProduct> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwProduct) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwProduct {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwProduct {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rulecategories: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rulecategories: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwProducts(::windows::runtime::IUnknown);
impl INetFwProducts {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Register<'a, Param0: ::windows::runtime::IntoParam<'a, INetFwProduct>>(&self, product: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), product.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<INetFwProduct> {
        let mut result__: <INetFwProduct as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<INetFwProduct>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwProducts {
    type Vtable = INetFwProducts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(971716320, 8343, 16573, [138, 242, 99, 161, 59, 82, 83, 98]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwProducts> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwProducts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwProducts> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwProducts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwProducts {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwProducts {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProducts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, product: ::windows::runtime::RawPtr, registration: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, product: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwProfile(::windows::runtime::IUnknown);
impl INetFwProfile {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<NET_FW_PROFILE_TYPE> {
        let mut result__: <NET_FW_PROFILE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn FirewallEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetFirewallEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn ExceptionsNotAllowed(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetExceptionsNotAllowed(&self, notallowed: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(notallowed)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn NotificationsDisabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetNotificationsDisabled(&self, disabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn RemoteAdminSettings(&self) -> ::windows::runtime::Result<INetFwRemoteAdminSettings> {
        let mut result__: <INetFwRemoteAdminSettings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwRemoteAdminSettings>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn IcmpSettings(&self) -> ::windows::runtime::Result<INetFwIcmpSettings> {
        let mut result__: <INetFwIcmpSettings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwIcmpSettings>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows::runtime::Result<INetFwOpenPorts> {
        let mut result__: <INetFwOpenPorts as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwOpenPorts>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Services(&self) -> ::windows::runtime::Result<INetFwServices> {
        let mut result__: <INetFwServices as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwServices>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn AuthorizedApplications(&self) -> ::windows::runtime::Result<INetFwAuthorizedApplications> {
        let mut result__: <INetFwAuthorizedApplications as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwAuthorizedApplications>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwProfile {
    type Vtable = INetFwProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(390729178, 59897, 17565, [153, 59, 33, 171, 102, 124, 164, 86]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwProfile> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwProfile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwProfile> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwProfile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notallowed: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notallowed: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteadminsettings: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmpsettings: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, openports: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, services: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, apps: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwRemoteAdminSettings(::windows::runtime::IUnknown);
impl INetFwRemoteAdminSettings {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn IpVersion(&self) -> ::windows::runtime::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Scope(&self) -> ::windows::runtime::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwRemoteAdminSettings {
    type Vtable = INetFwRemoteAdminSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3569274335, 28531, 19075, [184, 50, 156, 102, 135, 76, 210, 14]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwRemoteAdminSettings> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwRemoteAdminSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwRemoteAdminSettings> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwRemoteAdminSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwRule(::windows::runtime::IUnknown);
impl INetFwRule {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ApplicationName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ServiceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Protocol(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(protocol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalPorts(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemotePorts(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Direction(&self) -> ::windows::runtime::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: <NET_FW_RULE_DIRECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Interfaces(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Grouping(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Profiles(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletypesbitmask)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EdgeTraversal(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Action(&self) -> ::windows::runtime::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(action)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwRule {
    type Vtable = INetFwRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2938309927, 47802, 20034, [172, 237, 245, 36, 242, 44, 252, 226]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmptypesandcodes: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmptypesandcodes: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dir: NET_FW_RULE_DIRECTION) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaces: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaces: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfacetypes: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfacetypes: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: *mut NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: NET_FW_ACTION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwRule2(::windows::runtime::IUnknown);
impl INetFwRule2 {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ApplicationName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ServiceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Protocol(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(protocol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalPorts(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemotePorts(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Direction(&self) -> ::windows::runtime::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: <NET_FW_RULE_DIRECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Interfaces(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Grouping(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Profiles(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletypesbitmask)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EdgeTraversal(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Action(&self) -> ::windows::runtime::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), ::std::mem::transmute(loptions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwRule2 {
    type Vtable = INetFwRule2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2619853018, 6299, 19934, [137, 247, 139, 57, 163, 22, 120, 44]);
}
impl ::std::convert::From<INetFwRule2> for INetFwRule {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetFwRule2> for INetFwRule {
    fn from(value: &INetFwRule2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INetFwRule> for INetFwRule2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INetFwRule> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INetFwRule>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INetFwRule> for &INetFwRule2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INetFwRule> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INetFwRule>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwRule2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwRule2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwRule2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwRule2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwRule2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmptypesandcodes: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmptypesandcodes: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dir: NET_FW_RULE_DIRECTION) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaces: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaces: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfacetypes: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfacetypes: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: *mut NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loptions: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loptions: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwRule3(::windows::runtime::IUnknown);
impl INetFwRule3 {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ApplicationName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ServiceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Protocol(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(protocol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalPorts(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemotePorts(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Direction(&self) -> ::windows::runtime::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: <NET_FW_RULE_DIRECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(dir)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Interfaces(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Grouping(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Profiles(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(profiletypesbitmask)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EdgeTraversal(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Action(&self) -> ::windows::runtime::Result<NET_FW_ACTION> {
        let mut result__: <NET_FW_ACTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), ::std::mem::transmute(loptions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalAppPackageId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalAppPackageId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszpackageid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), wszpackageid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalUserOwner(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalUserOwner<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserowner: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), wszuserowner.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn LocalUserAuthorizedList(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalUserAuthorizedList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteUserAuthorizedList(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteUserAuthorizedList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteMachineAuthorizedList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SecureFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), ::std::mem::transmute(loptions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetFwRule3 {
    type Vtable = INetFwRule3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2987746303, 54934, 16930, [171, 70, 78, 137, 183, 58, 179, 74]);
}
impl ::std::convert::From<INetFwRule3> for INetFwRule2 {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetFwRule3> for INetFwRule2 {
    fn from(value: &INetFwRule3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INetFwRule2> for INetFwRule3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INetFwRule2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INetFwRule2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INetFwRule2> for &INetFwRule3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INetFwRule2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INetFwRule2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<INetFwRule3> for INetFwRule {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetFwRule3> for INetFwRule {
    fn from(value: &INetFwRule3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INetFwRule> for INetFwRule3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INetFwRule> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INetFwRule>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INetFwRule> for &INetFwRule3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INetFwRule> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INetFwRule>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwRule3> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwRule3> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwRule3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwRule3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwRule3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imagefilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portnumbers: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmptypesandcodes: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icmptypesandcodes: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dir: NET_FW_RULE_DIRECTION) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaces: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaces: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfacetypes: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfacetypes: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profiletypesbitmask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: *mut NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: NET_FW_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loptions: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loptions: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszpackageid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszpackageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserowner: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserowner: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserauthlist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserauthlist: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserauthlist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserauthlist: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserauthlist: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszuserauthlist: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loptions: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loptions: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwRules(::windows::runtime::IUnknown);
impl INetFwRules {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, INetFwRule>>(&self, rule: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), rule.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<INetFwRule> {
        let mut result__: <INetFwRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<INetFwRule>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwRules {
    type Vtable = INetFwRules_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2622251639, 20519, 17438, [175, 174, 202, 31, 84, 45, 160, 9]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwRules> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwRules) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwRules> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwRules) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRules_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rule: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwService(::windows::runtime::IUnknown);
impl INetFwService {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<NET_FW_SERVICE_TYPE> {
        let mut result__: <NET_FW_SERVICE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SERVICE_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Customized(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn IpVersion(&self) -> ::windows::runtime::Result<NET_FW_IP_VERSION> {
        let mut result__: <NET_FW_IP_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Scope(&self) -> ::windows::runtime::Result<NET_FW_SCOPE> {
        let mut result__: <NET_FW_SCOPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(scope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows::runtime::Result<INetFwOpenPorts> {
        let mut result__: <INetFwOpenPorts as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwOpenPorts>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwService {
    type Vtable = INetFwService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2046646216, 37006, 18998, [152, 136, 213, 179, 240, 164, 68, 207]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwService> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwService) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwService> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwService) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customized: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ipversion: NET_FW_IP_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: *mut NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scope: NET_FW_SCOPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteaddrs: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, openports: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwServiceRestriction(::windows::runtime::IUnknown);
impl INetFwServiceRestriction {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn RestrictService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, appname: Param1, restrictservice: i16, servicesidrestricted: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), servicename.into_param().abi(), appname.into_param().abi(), ::std::mem::transmute(restrictservice), ::std::mem::transmute(servicesidrestricted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ServiceRestricted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, appname: Param1) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), servicename.into_param().abi(), appname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Rules(&self) -> ::windows::runtime::Result<INetFwRules> {
        let mut result__: <INetFwRules as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetFwRules>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwServiceRestriction {
    type Vtable = INetFwServiceRestriction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2187836387, 63632, 18716, [183, 182, 45, 177, 239, 14, 93, 43]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwServiceRestriction> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwServiceRestriction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwServiceRestriction> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwServiceRestriction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwServiceRestriction {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwServiceRestriction {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestriction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rules: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetFwServices(::windows::runtime::IUnknown);
impl INetFwServices {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> ::windows::runtime::Result<INetFwService> {
        let mut result__: <INetFwService as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(svctype), &mut result__).from_abi::<INetFwService>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetFwServices {
    type Vtable = INetFwServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2036636596, 36926, 16923, [148, 201, 121, 132, 142, 121, 246, 238]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetFwServices> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetFwServices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetFwServices> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetFwServices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetFwServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetFwServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingConfiguration(::windows::runtime::IUnknown);
impl INetSharingConfiguration {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SharingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SharingConnectionType(&self) -> ::windows::runtime::Result<SHARINGCONNECTIONTYPE> {
        let mut result__: <SHARINGCONNECTIONTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<SHARINGCONNECTIONTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn DisableSharing(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn InternetFirewallEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn DisableInternetFirewall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnableInternetFirewall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::runtime::Result<INetSharingPortMappingCollection> {
        let mut result__: <INetSharingPortMappingCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), &mut result__).from_abi::<INetSharingPortMappingCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn AddPortMapping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: Param5, etargettype: ICS_TARGETTYPE) -> ::windows::runtime::Result<INetSharingPortMapping> {
        let mut result__: <INetSharingPortMapping as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrname.into_param().abi(),
            ::std::mem::transmute(ucipprotocol),
            ::std::mem::transmute(usexternalport),
            ::std::mem::transmute(usinternalport),
            ::std::mem::transmute(dwoptions),
            bstrtargetnameoripaddress.into_param().abi(),
            ::std::mem::transmute(etargettype),
            &mut result__,
        )
        .from_abi::<INetSharingPortMapping>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn RemovePortMapping<'a, Param0: ::windows::runtime::IntoParam<'a, INetSharingPortMapping>>(&self, pmapping: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pmapping.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingConfiguration {
    type Vtable = INetSharingConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226102, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingConfiguration> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingConfiguration> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: SHARINGCONNECTIONTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmapping: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingEveryConnectionCollection(::windows::runtime::IUnknown);
impl INetSharingEveryConnectionCollection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingEveryConnectionCollection {
    type Vtable = INetSharingEveryConnectionCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(868508732, 30737, 18170, [168, 154, 118, 133, 151, 189, 114, 35]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingEveryConnectionCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingEveryConnectionCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingEveryConnectionCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingEveryConnectionCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingEveryConnectionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingManager(::windows::runtime::IUnknown);
impl INetSharingManager {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn SharingInstalled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::runtime::Result<INetSharingPublicConnectionCollection> {
        let mut result__: <INetSharingPublicConnectionCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), &mut result__).from_abi::<INetSharingPublicConnectionCollection>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::runtime::Result<INetSharingPrivateConnectionCollection> {
        let mut result__: <INetSharingPrivateConnectionCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), &mut result__).from_abi::<INetSharingPrivateConnectionCollection>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn INetSharingConfigurationForINetConnection<'a, Param0: ::windows::runtime::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows::runtime::Result<INetSharingConfiguration> {
        let mut result__: <INetSharingConfiguration as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pnetconnection.into_param().abi(), &mut result__).from_abi::<INetSharingConfiguration>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EnumEveryConnection(&self) -> ::windows::runtime::Result<INetSharingEveryConnectionCollection> {
        let mut result__: <INetSharingEveryConnectionCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetSharingEveryConnectionCollection>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn NetConnectionProps<'a, Param0: ::windows::runtime::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows::runtime::Result<INetConnectionProps> {
        let mut result__: <INetConnectionProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pnetconnection.into_param().abi(), &mut result__).from_abi::<INetConnectionProps>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingManager {
    type Vtable = INetSharingManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226103, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbinstalled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnetconnection: ::windows::runtime::RawPtr, ppnetsharingconfiguration: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcoll: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnetconnection: ::windows::runtime::RawPtr, ppprops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingPortMapping(::windows::runtime::IUnknown);
impl INetSharingPortMapping {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Disable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enable(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Properties(&self) -> ::windows::runtime::Result<INetSharingPortMappingProps> {
        let mut result__: <INetSharingPortMappingProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetSharingPortMappingProps>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingPortMapping {
    type Vtable = INetSharingPortMapping_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230226097, 7379, 4561, [177, 197, 0, 128, 95, 193, 39, 14]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingPortMapping> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingPortMapping) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingPortMapping> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingPortMapping) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingPortMapping {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingPortMapping {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnspmp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingPortMappingCollection(::windows::runtime::IUnknown);
impl INetSharingPortMappingCollection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingPortMappingCollection {
    type Vtable = INetSharingPortMappingCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(48538334, 55840, 20020, [137, 200, 172, 34, 39, 90, 1, 11]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingPortMappingCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingPortMappingCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingPortMappingCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingPortMappingCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingPortMappingProps(::windows::runtime::IUnknown);
impl INetSharingPortMappingProps {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn IPProtocol(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn ExternalPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn InternalPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Options(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn TargetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn TargetIPAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingPortMappingProps {
    type Vtable = INetSharingPortMappingProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(616032693, 58255, 18053, [133, 27, 0, 137, 44, 245, 249, 64]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingPortMappingProps> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingPortMappingProps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingPortMappingProps> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingPortMappingProps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pucipprot: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pusport: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pusport: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwoptions: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtargetname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtargetipaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingPrivateConnectionCollection(::windows::runtime::IUnknown);
impl INetSharingPrivateConnectionCollection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingPrivateConnectionCollection {
    type Vtable = INetSharingPrivateConnectionCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(950954464, 17417, 16426, [162, 203, 233, 101, 199, 39, 248, 64]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingPrivateConnectionCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingPrivateConnectionCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingPrivateConnectionCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingPrivateConnectionCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPrivateConnectionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INetSharingPublicConnectionCollection(::windows::runtime::IUnknown);
impl INetSharingPublicConnectionCollection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetSharingPublicConnectionCollection {
    type Vtable = INetSharingPublicConnectionCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2105172821, 62322, 18801, [161, 73, 191, 201, 39, 190, 118, 42]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetSharingPublicConnectionCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetSharingPublicConnectionCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetSharingPublicConnectionCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetSharingPublicConnectionCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPublicConnectionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IStaticPortMapping(::windows::runtime::IUnknown);
impl IStaticPortMapping {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn ExternalIPAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn ExternalPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn InternalPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Protocol(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn InternalClient(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn EditInternalClient<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bstrinternalclient.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Enable(&self, vb: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(vb)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn EditDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(linternalport)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStaticPortMapping {
    type Vtable = IStaticPortMapping_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1863348511, 29339, 16869, [147, 184, 242, 29, 15, 129, 141, 241]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IStaticPortMapping> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IStaticPortMapping) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IStaticPortMapping> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IStaticPortMapping) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IStaticPortMapping {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IStaticPortMapping {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrinternalclient: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vb: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linternalport: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IStaticPortMappingCollection(::windows::runtime::IUnknown);
impl IStaticPortMappingCollection {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows::runtime::Result<IStaticPortMapping> {
        let mut result__: <IStaticPortMapping as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), &mut result__).from_abi::<IStaticPortMapping>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    pub unsafe fn Add<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1, linternalport: i32, bstrinternalclient: Param3, benabled: i16, bstrdescription: Param5) -> ::windows::runtime::Result<IStaticPortMapping> {
        let mut result__: <IStaticPortMapping as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::std::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::std::mem::transmute(benabled), bstrdescription.into_param().abi(), &mut result__).from_abi::<IStaticPortMapping>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStaticPortMappingCollection {
    type Vtable = IStaticPortMappingCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3441376887, 26326, 18020, [130, 199, 54, 219, 182, 65, 208, 241]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IStaticPortMappingCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IStaticPortMappingCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IStaticPortMappingCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IStaticPortMappingCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lexternalport: i32, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lexternalport: i32, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lexternalport: i32, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUPnPNAT(::windows::runtime::IUnknown);
impl IUPnPNAT {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn StaticPortMappingCollection(&self) -> ::windows::runtime::Result<IStaticPortMappingCollection> {
        let mut result__: <IStaticPortMappingCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IStaticPortMappingCollection>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn DynamicPortMappingCollection(&self) -> ::windows::runtime::Result<IDynamicPortMappingCollection> {
        let mut result__: <IDynamicPortMappingCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDynamicPortMappingCollection>(result__)
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
    pub unsafe fn NATEventManager(&self) -> ::windows::runtime::Result<INATEventManager> {
        let mut result__: <INATEventManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INATEventManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUPnPNAT {
    type Vtable = IUPnPNAT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2977024018, 52342, 18522, [148, 216, 182, 179, 162, 121, 78, 153]);
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IUPnPNAT> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IUPnPNAT) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IUPnPNAT> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IUPnPNAT) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IUPnPNAT {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IUPnPNAT {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPNAT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppspms: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdpms: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCONMGR_ENUM_FLAGS(pub i32);
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(0i32);
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(1i32);
impl ::std::convert::From<i32> for NETCONMGR_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETCONMGR_ENUM_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCONUI_CONNECT_FLAGS(pub i32);
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(0i32);
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(1i32);
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(2i32);
impl ::std::convert::From<i32> for NETCONUI_CONNECT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETCONUI_CONNECT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for NETCON_CHARACTERISTIC_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETCON_CHARACTERISTIC_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for NETCON_MEDIATYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETCON_MEDIATYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
pub struct NETCON_PROPERTIES {
    pub guidId: ::windows::runtime::GUID,
    pub pszwName: super::super::Foundation::PWSTR,
    pub pszwDeviceName: super::super::Foundation::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: ::windows::runtime::GUID,
    pub clsidUiObject: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NETCON_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETCON_PROPERTIES")
            .field("guidId", &self.guidId)
            .field("pszwName", &self.pszwName)
            .field("pszwDeviceName", &self.pszwDeviceName)
            .field("Status", &self.Status)
            .field("MediaType", &self.MediaType)
            .field("dwCharacter", &self.dwCharacter)
            .field("clsidThisObject", &self.clsidThisObject)
            .field("clsidUiObject", &self.clsidUiObject)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NETCON_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.pszwName == other.pszwName && self.pszwDeviceName == other.pszwDeviceName && self.Status == other.Status && self.MediaType == other.MediaType && self.dwCharacter == other.dwCharacter && self.clsidThisObject == other.clsidThisObject && self.clsidUiObject == other.clsidUiObject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NETCON_PROPERTIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for NETCON_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETCON_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETCON_TYPE(pub i32);
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = NETCON_TYPE(0i32);
pub const NCT_INBOUND: NETCON_TYPE = NETCON_TYPE(1i32);
pub const NCT_INTERNET: NETCON_TYPE = NETCON_TYPE(2i32);
pub const NCT_LAN: NETCON_TYPE = NETCON_TYPE(3i32);
pub const NCT_PHONE: NETCON_TYPE = NETCON_TYPE(4i32);
pub const NCT_TUNNEL: NETCON_TYPE = NETCON_TYPE(5i32);
pub const NCT_BRIDGE: NETCON_TYPE = NETCON_TYPE(6i32);
impl ::std::convert::From<i32> for NETCON_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETCON_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETISO_ERROR_TYPE(pub i32);
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(0i32);
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(1i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(2i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(3i32);
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(4i32);
impl ::std::convert::From<i32> for NETISO_ERROR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETISO_ERROR_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETISO_FLAG(pub i32);
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = NETISO_FLAG(1i32);
pub const NETISO_FLAG_MAX: NETISO_FLAG = NETISO_FLAG(2i32);
impl ::std::convert::From<i32> for NETISO_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETISO_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_ACTION(pub i32);
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = NET_FW_ACTION(0i32);
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = NET_FW_ACTION(1i32);
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = NET_FW_ACTION(2i32);
impl ::std::convert::From<i32> for NET_FW_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_AUTHENTICATE_TYPE(pub i32);
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(0i32);
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(1i32);
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(2i32);
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(3i32);
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(4i32);
impl ::std::convert::From<i32> for NET_FW_AUTHENTICATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_AUTHENTICATE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(pub i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(0i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(1i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(3i32);
impl ::std::convert::From<i32> for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_EDGE_TRAVERSAL_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_IP_PROTOCOL(pub i32);
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(6i32);
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(17i32);
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(256i32);
impl ::std::convert::From<i32> for NET_FW_IP_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_IP_PROTOCOL {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_IP_VERSION(pub i32);
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = NET_FW_IP_VERSION(0i32);
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = NET_FW_IP_VERSION(1i32);
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = NET_FW_IP_VERSION(2i32);
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = NET_FW_IP_VERSION(3i32);
impl ::std::convert::From<i32> for NET_FW_IP_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_IP_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_MODIFY_STATE(pub i32);
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(0i32);
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(1i32);
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(2i32);
impl ::std::convert::From<i32> for NET_FW_MODIFY_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_MODIFY_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_POLICY_TYPE(pub i32);
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(0i32);
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(1i32);
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(2i32);
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(3i32);
impl ::std::convert::From<i32> for NET_FW_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_POLICY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_PROFILE_TYPE(pub i32);
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(0i32);
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(1i32);
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(2i32);
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(3i32);
impl ::std::convert::From<i32> for NET_FW_PROFILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_PROFILE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_PROFILE_TYPE2(pub i32);
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(1i32);
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2i32);
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(4i32);
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2147483647i32);
impl ::std::convert::From<i32> for NET_FW_PROFILE_TYPE2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_PROFILE_TYPE2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_RULE_CATEGORY(pub i32);
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(0i32);
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(1i32);
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(2i32);
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(3i32);
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(4i32);
impl ::std::convert::From<i32> for NET_FW_RULE_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_RULE_CATEGORY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_RULE_DIRECTION(pub i32);
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(1i32);
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(2i32);
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(3i32);
impl ::std::convert::From<i32> for NET_FW_RULE_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_RULE_DIRECTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_SCOPE(pub i32);
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = NET_FW_SCOPE(0i32);
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = NET_FW_SCOPE(1i32);
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = NET_FW_SCOPE(2i32);
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = NET_FW_SCOPE(3i32);
impl ::std::convert::From<i32> for NET_FW_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_SCOPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NET_FW_SERVICE_TYPE(pub i32);
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(0i32);
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(1i32);
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(2i32);
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(3i32);
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(4i32);
impl ::std::convert::From<i32> for NET_FW_SERVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_FW_SERVICE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NetFwAuthorizedApplication: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3969402547, 10082, 19051, [162, 20, 106, 203, 96, 52, 98, 210]);
pub const NetFwMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810346818, 28217, 16600, [148, 58, 185, 19, 196, 12, 156, 212]);
pub const NetFwOpenPort: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(212157894, 14253, 19052, [191, 146, 159, 118, 16, 6, 126, 245]);
pub const NetFwPolicy2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803433343, 27361, 16812, [129, 122, 246, 249, 33, 102, 215, 221]);
pub const NetFwProduct: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2641649368, 50452, 19741, [191, 66, 117, 31, 237, 45, 90, 199]);
pub const NetFwProducts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3424192411, 33394, 19827, [187, 112, 205, 181, 51, 82, 123, 97]);
pub const NetFwRule: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(744211518, 13161, 19507, [171, 12, 190, 148, 105, 103, 122, 244]);
pub const NetSharingManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1550041517, 14678, 20472, [132, 134, 64, 3, 71, 88, 49, 91]);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(wszservername: Param0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername: super::super::Foundation::PWSTR, netisoerror: *mut NETISO_ERROR_TYPE) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.into_param().abi(), ::std::mem::transmute(netisoerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationEnumAppContainers(::std::mem::transmute(flags), ::std::mem::transmute(pdwnumpublicappcs), ::std::mem::transmute(pppublicappcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationFreeAppContainers(::std::mem::transmute(ppublicappcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationGetAppContainerConfig(::std::mem::transmute(pdwnumpublicappcs), ::std::mem::transmute(appcontainersids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: ::std::option::Option<PAC_CHANGES_CALLBACK_FN>, context: *const ::std::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationRegisterForAppContainerChanges(::std::mem::transmute(flags), ::std::mem::transmute(callback), ::std::mem::transmute(context), ::std::mem::transmute(registrationobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationSetAppContainerConfig(::std::mem::transmute(dwnumpublicappcs), ::std::mem::transmute(appcontainersids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    applicationcontainersid: Param0,
    packagefullname: Param1,
    packagefolder: Param2,
    displayname: Param3,
    bbinariesfullycomputed: Param4,
    binaries: *const super::super::Foundation::PWSTR,
    binariescount: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid: super::super::Foundation::PSID, packagefullname: super::super::Foundation::PWSTR, packagefolder: super::super::Foundation::PWSTR, displayname: super::super::Foundation::PWSTR, bbinariesfullycomputed: super::super::Foundation::BOOL, binaries: *const super::super::Foundation::PWSTR, binariescount: u32) -> ::windows::runtime::HRESULT;
        }
        NetworkIsolationSetupAppContainerBinaries(applicationcontainersid.into_param().abi(), packagefullname.into_param().abi(), packagefolder.into_param().abi(), displayname.into_param().abi(), bbinariesfullycomputed.into_param().abi(), ::std::mem::transmute(binaries), ::std::mem::transmute(binariescount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(registrationobject: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(NetworkIsolationUnregisterForAppContainerChanges(registrationobject.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PAC_CHANGES_CALLBACK_FN = unsafe extern "system" fn(context: *const ::std::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = unsafe extern "system" fn(dynamickeywordaddress: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = unsafe extern "system" fn(dynamickeywordaddressid: ::windows::runtime::GUID) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = unsafe extern "system" fn(dynamickeywordaddressid: ::windows::runtime::GUID, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = unsafe extern "system" fn(dynamickeywordaddressdata: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = unsafe extern "system" fn(dynamickeywordaddressid: ::windows::runtime::GUID, updatedaddresses: super::super::Foundation::PWSTR, append: super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PNETISO_EDP_ID_CALLBACK_FN = unsafe extern "system" fn(context: *mut ::std::ffi::c_void, wszenterpriseid: super::super::Foundation::PWSTR, dwerr: u32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHARINGCONNECTIONTYPE(pub i32);
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(0i32);
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(1i32);
impl ::std::convert::From<i32> for SHARINGCONNECTIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHARINGCONNECTIONTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(pub i32);
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(0i32);
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(1i32);
impl ::std::convert::From<i32> for SHARINGCONNECTION_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHARINGCONNECTION_ENUM_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const S_OBJECT_NO_LONGER_VALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(2i32 as _);
pub const UPnPNAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2921201834, 16341, 16444, [138, 39, 43, 189, 195, 12, 208, 225]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows::runtime::GUID,
    pub keyword: super::super::Foundation::PWSTR,
    pub flags: u32,
    pub addresses: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS0").field("id", &self.id).field("keyword", &self.keyword).field("flags", &self.flags).field("addresses", &self.addresses).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.keyword == other.keyword && self.flags == other.flags && self.addresses == other.addresses
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: _tag_FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0").field("dynamicKeywordAddress", &self.dynamicKeywordAddress).field("next", &self.next).field("schemaVersion", &self.schemaVersion).field("originType", &self.originType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        self.dynamicKeywordAddress == other.dynamicKeywordAddress && self.next == other.next && self.schemaVersion == other.schemaVersion && self.originType == other.originType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(pub i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(2i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(3i32);
impl ::std::convert::From<i32> for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(pub i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(1i32);
impl ::std::convert::From<i32> for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(pub i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(0i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(2i32);
impl ::std::convert::From<i32> for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
