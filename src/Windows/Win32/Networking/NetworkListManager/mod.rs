#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumNetworkConnections(::windows::runtime::IUnknown);
impl IEnumNetworkConnections {
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::Automation::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::Automation::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::Automation::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<INetworkConnection>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetworkConnections> {
        let mut result__: <IEnumNetworkConnections as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetworkConnections>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetworkConnections {
    type Vtable = IEnumNetworkConnections_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521862, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<IEnumNetworkConnections> for ::windows::runtime::IUnknown {
    fn from(value: IEnumNetworkConnections) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumNetworkConnections> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumNetworkConnections) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumNetworkConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumNetworkConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IEnumNetworkConnections> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IEnumNetworkConnections) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IEnumNetworkConnections> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IEnumNetworkConnections) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IEnumNetworkConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IEnumNetworkConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworkConnections_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumvar: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumnetwork: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumNetworks(::windows::runtime::IUnknown);
impl IEnumNetworks {
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::Automation::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::Automation::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::Automation::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<INetwork>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumNetworks> {
        let mut result__: <IEnumNetworks as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetworks>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumNetworks {
    type Vtable = IEnumNetworks_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521859, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<IEnumNetworks> for ::windows::runtime::IUnknown {
    fn from(value: IEnumNetworks) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumNetworks> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumNetworks) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumNetworks {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumNetworks {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IEnumNetworks> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IEnumNetworks) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IEnumNetworks> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IEnumNetworks) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IEnumNetworks {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IEnumNetworks {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworks_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumvar: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumnetwork: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetwork(::windows::runtime::IUnknown);
impl INetwork {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sznetworknewname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), sznetworknewname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, szdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), szdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetworkId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetDomainType(&self) -> ::windows::runtime::Result<NLM_DOMAIN_TYPE> {
        let mut result__: <NLM_DOMAIN_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_DOMAIN_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::runtime::Result<IEnumNetworkConnections> {
        let mut result__: <IEnumNetworkConnections as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetworkConnections>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwlowdatetimecreated), ::std::mem::transmute(pdwhighdatetimecreated), ::std::mem::transmute(pdwlowdatetimeconnected), ::std::mem::transmute(pdwhighdatetimeconnected)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetConnectivity(&self) -> ::windows::runtime::Result<NLM_CONNECTIVITY> {
        let mut result__: <NLM_CONNECTIVITY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_CONNECTIVITY>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetCategory(&self) -> ::windows::runtime::Result<NLM_NETWORK_CATEGORY> {
        let mut result__: <NLM_NETWORK_CATEGORY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_NETWORK_CATEGORY>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(newcategory)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetwork {
    type Vtable = INetwork_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521858, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetwork> for ::windows::runtime::IUnknown {
    fn from(value: INetwork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetwork> for ::windows::runtime::IUnknown {
    fn from(value: &INetwork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetwork> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetwork) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetwork> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetwork) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetwork {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetwork_abi(
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
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psznetworkname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sznetworknewname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgdguidnetworkid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumnetworkconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkConnection(::windows::runtime::IUnknown);
impl INetworkConnection {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetwork(&self) -> ::windows::runtime::Result<INetwork> {
        let mut result__: <INetwork as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INetwork>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetConnectivity(&self) -> ::windows::runtime::Result<NLM_CONNECTIVITY> {
        let mut result__: <NLM_CONNECTIVITY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_CONNECTIVITY>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetConnectionId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetAdapterId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetDomainType(&self) -> ::windows::runtime::Result<NLM_DOMAIN_TYPE> {
        let mut result__: <NLM_DOMAIN_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_DOMAIN_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetworkConnection {
    type Vtable = INetworkConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521861, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkConnection> for ::windows::runtime::IUnknown {
    fn from(value: INetworkConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkConnection> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetworkConnection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetworkConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetworkConnection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetworkConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetworkConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetworkConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnection_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnetwork: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgdconnectionid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgdadapterid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkConnectionCost(::windows::runtime::IUnknown);
impl INetworkConnectionCost {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetCost(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
    pub unsafe fn GetDataPlanStatus(&self) -> ::windows::runtime::Result<NLM_DATAPLAN_STATUS> {
        let mut result__: <NLM_DATAPLAN_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_DATAPLAN_STATUS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for INetworkConnectionCost {
    type Vtable = INetworkConnectionCost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521866, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkConnectionCost> for ::windows::runtime::IUnknown {
    fn from(value: INetworkConnectionCost) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkConnectionCost> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkConnectionCost) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkConnectionCost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkConnectionCost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcost: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkConnectionCostEvents(::windows::runtime::IUnknown);
impl INetworkConnectionCostEvents {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn ConnectionCostChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, connectionid: Param0, newcost: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), connectionid.into_param().abi(), ::std::mem::transmute(newcost)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn ConnectionDataPlanStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, connectionid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), connectionid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkConnectionCostEvents {
    type Vtable = INetworkConnectionCostEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521867, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkConnectionCostEvents> for ::windows::runtime::IUnknown {
    fn from(value: INetworkConnectionCostEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkConnectionCostEvents> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkConnectionCostEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkConnectionCostEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkConnectionCostEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCostEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: ::windows::runtime::GUID, newcost: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkConnectionEvents(::windows::runtime::IUnknown);
impl INetworkConnectionEvents {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn NetworkConnectionConnectivityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, connectionid: Param0, newconnectivity: NLM_CONNECTIVITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), connectionid.into_param().abi(), ::std::mem::transmute(newconnectivity)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn NetworkConnectionPropertyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, connectionid: Param0, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), connectionid.into_param().abi(), ::std::mem::transmute(flags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkConnectionEvents {
    type Vtable = INetworkConnectionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521863, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkConnectionEvents> for ::windows::runtime::IUnknown {
    fn from(value: INetworkConnectionEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkConnectionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkConnectionEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkConnectionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkConnectionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: ::windows::runtime::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: ::windows::runtime::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkCostManager(::windows::runtime::IUnknown);
impl INetworkCostManager {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcost), ::std::mem::transmute(pdestipaddr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdataplanstatus), ::std::mem::transmute(pdestipaddr)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn SetDestinationAddresses(&self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(length), ::std::mem::transmute(pdestipaddrlist), ::std::mem::transmute(bappend)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkCostManager {
    type Vtable = INetworkCostManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521864, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkCostManager> for ::windows::runtime::IUnknown {
    fn from(value: INetworkCostManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkCostManager> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkCostManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkCostManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkCostManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkCostManagerEvents(::windows::runtime::IUnknown);
impl INetworkCostManagerEvents {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(newcost), ::std::mem::transmute(pdestaddr)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdestaddr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkCostManagerEvents {
    type Vtable = INetworkCostManagerEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521865, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkCostManagerEvents> for ::windows::runtime::IUnknown {
    fn from(value: INetworkCostManagerEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkCostManagerEvents> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkCostManagerEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkCostManagerEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkCostManagerEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestaddr: *const NLM_SOCKADDR) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkEvents(::windows::runtime::IUnknown);
impl INetworkEvents {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn NetworkAdded<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, networkid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), networkid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn NetworkDeleted<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, networkid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), networkid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn NetworkConnectivityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, networkid: Param0, newconnectivity: NLM_CONNECTIVITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), networkid.into_param().abi(), ::std::mem::transmute(newconnectivity)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn NetworkPropertyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, networkid: Param0, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), networkid.into_param().abi(), ::std::mem::transmute(flags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkEvents {
    type Vtable = INetworkEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521860, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkEvents> for ::windows::runtime::IUnknown {
    fn from(value: INetworkEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkEvents> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkid: ::windows::runtime::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkid: ::windows::runtime::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkListManager(::windows::runtime::IUnknown);
impl INetworkListManager {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> ::windows::runtime::Result<IEnumNetworks> {
        let mut result__: <IEnumNetworks as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(flags), &mut result__).from_abi::<IEnumNetworks>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetwork<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, gdnetworkid: Param0) -> ::windows::runtime::Result<INetwork> {
        let mut result__: <INetwork as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), gdnetworkid.into_param().abi(), &mut result__).from_abi::<INetwork>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::runtime::Result<IEnumNetworkConnections> {
        let mut result__: <IEnumNetworkConnections as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumNetworkConnections>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetNetworkConnection<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, gdnetworkconnectionid: Param0) -> ::windows::runtime::Result<INetworkConnection> {
        let mut result__: <INetworkConnection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), gdnetworkconnectionid.into_param().abi(), &mut result__).from_abi::<INetworkConnection>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn GetConnectivity(&self) -> ::windows::runtime::Result<NLM_CONNECTIVITY> {
        let mut result__: <NLM_CONNECTIVITY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<NLM_CONNECTIVITY>(result__)
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(psimulatedinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn ClearSimulatedProfileInfo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkListManager {
    type Vtable = INetworkListManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521856, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkListManager> for ::windows::runtime::IUnknown {
    fn from(value: INetworkListManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkListManager> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkListManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkListManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkListManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<INetworkListManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: INetworkListManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&INetworkListManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &INetworkListManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for INetworkListManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &INetworkListManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManager_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gdnetworkid: ::windows::runtime::GUID, ppnetwork: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gdnetworkconnectionid: ::windows::runtime::GUID, ppnetworkconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbisconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INetworkListManagerEvents(::windows::runtime::IUnknown);
impl INetworkListManagerEvents {
    #[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
    pub unsafe fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(newconnectivity)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INetworkListManagerEvents {
    type Vtable = INetworkListManagerEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702521857, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
}
impl ::std::convert::From<INetworkListManagerEvents> for ::windows::runtime::IUnknown {
    fn from(value: INetworkListManagerEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INetworkListManagerEvents> for ::windows::runtime::IUnknown {
    fn from(value: &INetworkListManagerEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INetworkListManagerEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INetworkListManagerEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newconnectivity: NLM_CONNECTIVITY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_CONNECTION_COST(pub i32);
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = NLM_CONNECTION_COST(0i32);
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(1i32);
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(2i32);
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = NLM_CONNECTION_COST(4i32);
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(65536i32);
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(131072i32);
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = NLM_CONNECTION_COST(262144i32);
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(524288i32);
impl ::std::convert::From<i32> for NLM_CONNECTION_COST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_CONNECTION_COST {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_CONNECTION_PROPERTY_CHANGE(pub i32);
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = NLM_CONNECTION_PROPERTY_CHANGE(1i32);
impl ::std::convert::From<i32> for NLM_CONNECTION_PROPERTY_CHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_CONNECTION_PROPERTY_CHANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_CONNECTIVITY(pub i32);
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = NLM_CONNECTIVITY(0i32);
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1i32);
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(2i32);
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(16i32);
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(32i32);
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(64i32);
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(256i32);
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(512i32);
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1024i32);
impl ::std::convert::From<i32> for NLM_CONNECTIVITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_CONNECTIVITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: ::windows::runtime::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NLM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NLM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NLM_DATAPLAN_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NLM_DATAPLAN_STATUS")
            .field("InterfaceGuid", &self.InterfaceGuid)
            .field("UsageData", &self.UsageData)
            .field("DataLimitInMegabytes", &self.DataLimitInMegabytes)
            .field("InboundBandwidthInKbps", &self.InboundBandwidthInKbps)
            .field("OutboundBandwidthInKbps", &self.OutboundBandwidthInKbps)
            .field("NextBillingCycle", &self.NextBillingCycle)
            .field("MaxTransferSizeInMegabytes", &self.MaxTransferSizeInMegabytes)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NLM_DATAPLAN_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceGuid == other.InterfaceGuid && self.UsageData == other.UsageData && self.DataLimitInMegabytes == other.DataLimitInMegabytes && self.InboundBandwidthInKbps == other.InboundBandwidthInKbps && self.OutboundBandwidthInKbps == other.OutboundBandwidthInKbps && self.NextBillingCycle == other.NextBillingCycle && self.MaxTransferSizeInMegabytes == other.MaxTransferSizeInMegabytes && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NLM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NLM_DATAPLAN_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_DOMAIN_TYPE(pub i32);
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(0i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(1i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(2i32);
impl ::std::convert::From<i32> for NLM_DOMAIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_DOMAIN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_ENUM_NETWORK(pub i32);
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(1i32);
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(2i32);
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(3i32);
impl ::std::convert::From<i32> for NLM_ENUM_NETWORK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_ENUM_NETWORK {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_INTERNET_CONNECTIVITY(pub i32);
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(1i32);
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(2i32);
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(4i32);
impl ::std::convert::From<i32> for NLM_INTERNET_CONNECTIVITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_INTERNET_CONNECTIVITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_NETWORK_CATEGORY(pub i32);
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(0i32);
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(1i32);
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(2i32);
impl ::std::convert::From<i32> for NLM_NETWORK_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_NETWORK_CATEGORY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_NETWORK_CLASS(pub i32);
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(1i32);
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(2i32);
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(3i32);
impl ::std::convert::From<i32> for NLM_NETWORK_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_NETWORK_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NLM_NETWORK_PROPERTY_CHANGE(pub i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(1i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(2i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(4i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(8i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(16i32);
impl ::std::convert::From<i32> for NLM_NETWORK_PROPERTY_CHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NLM_NETWORK_PROPERTY_CHANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl NLM_SIMULATED_PROFILE_INFO {}
impl ::std::default::Default for NLM_SIMULATED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NLM_SIMULATED_PROFILE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NLM_SIMULATED_PROFILE_INFO").field("ProfileName", &self.ProfileName).field("cost", &self.cost).field("UsageInMegabytes", &self.UsageInMegabytes).field("DataLimitInMegabytes", &self.DataLimitInMegabytes).finish()
    }
}
impl ::std::cmp::PartialEq for NLM_SIMULATED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProfileName == other.ProfileName && self.cost == other.cost && self.UsageInMegabytes == other.UsageInMegabytes && self.DataLimitInMegabytes == other.DataLimitInMegabytes
    }
}
impl ::std::cmp::Eq for NLM_SIMULATED_PROFILE_INFO {}
unsafe impl ::windows::runtime::Abi for NLM_SIMULATED_PROFILE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl NLM_SOCKADDR {}
impl ::std::default::Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NLM_SOCKADDR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NLM_SOCKADDR").field("data", &self.data).finish()
    }
}
impl ::std::cmp::PartialEq for NLM_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::std::cmp::Eq for NLM_SOCKADDR {}
unsafe impl ::windows::runtime::Abi for NLM_SOCKADDR {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_NetworkListManager`, `Win32_Foundation`*"]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl NLM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NLM_USAGE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NLM_USAGE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NLM_USAGE_DATA").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NLM_USAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UsageInMegabytes == other.UsageInMegabytes && self.LastSyncTime == other.LastSyncTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NLM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NLM_USAGE_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NetworkListManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3702524929, 22287, 19099, [141, 105, 25, 159, 219, 165, 114, 59]);
