#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEnumNetworkConnections(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworkConnections {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetworkConnection>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetworkConnections>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEnumNetworkConnections> for ::windows::core::IUnknown {
    fn from(value: IEnumNetworkConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEnumNetworkConnections> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetworkConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetworkConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetworkConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEnumNetworkConnections> for super::super::System::Com::IDispatch {
    fn from(value: IEnumNetworkConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEnumNetworkConnections> for super::super::System::Com::IDispatch {
    fn from(value: &IEnumNetworkConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IEnumNetworkConnections {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IEnumNetworkConnections {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEnumNetworkConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumNetworkConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumNetworkConnections {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumNetworkConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetworkConnections").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IEnumNetworkConnections {
    type Vtable = IEnumNetworkConnections_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00006_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworkConnections_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEnumNetworks(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworks {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetwork>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetworks> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetworks>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEnumNetworks> for ::windows::core::IUnknown {
    fn from(value: IEnumNetworks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEnumNetworks> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetworks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetworks {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumNetworks {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEnumNetworks> for super::super::System::Com::IDispatch {
    fn from(value: IEnumNetworks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEnumNetworks> for super::super::System::Com::IDispatch {
    fn from(value: &IEnumNetworks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IEnumNetworks {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IEnumNetworks {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEnumNetworks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumNetworks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumNetworks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumNetworks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetworks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IEnumNetworks {
    type Vtable = IEnumNetworks_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00003_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworks_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetwork(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetwork {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sznetworknewname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), sznetworknewname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, szdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), szdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetNetworkId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE> {
        let mut result__: NLM_DOMAIN_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDomainType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_DOMAIN_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkConnections)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetworkConnections>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTimeCreatedAndConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwlowdatetimecreated), ::core::mem::transmute(pdwhighdatetimecreated), ::core::mem::transmute(pdwlowdatetimeconnected), ::core::mem::transmute(pdwhighdatetimeconnected)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnectedToInternet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__: NLM_CONNECTIVITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectivity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_CONNECTIVITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<NLM_NETWORK_CATEGORY> {
        let mut result__: NLM_NETWORK_CATEGORY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_NETWORK_CATEGORY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(newcategory)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetwork> for ::windows::core::IUnknown {
    fn from(value: INetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetwork> for ::windows::core::IUnknown {
    fn from(value: &INetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetwork> for super::super::System::Com::IDispatch {
    fn from(value: INetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetwork> for super::super::System::Com::IDispatch {
    fn from(value: &INetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetwork {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a INetwork {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetwork").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for INetwork {
    type Vtable = INetwork_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00002_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetwork_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psznetworkname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznetworknewname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub GetNetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    pub GetTimeCreatedAndConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::HRESULT,
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetworkConnection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork(&self) -> ::windows::core::Result<INetwork> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetwork)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnectedToInternet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__: NLM_CONNECTIVITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectivity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_CONNECTIVITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetConnectionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAdapterId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE> {
        let mut result__: NLM_DOMAIN_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDomainType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_DOMAIN_TYPE>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetworkConnection> for ::windows::core::IUnknown {
    fn from(value: INetworkConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetworkConnection> for ::windows::core::IUnknown {
    fn from(value: &INetworkConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetworkConnection> for super::super::System::Com::IDispatch {
    fn from(value: INetworkConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetworkConnection> for super::super::System::Com::IDispatch {
    fn from(value: &INetworkConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetworkConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a INetworkConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetworkConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INetworkConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INetworkConnection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INetworkConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkConnection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for INetworkConnection {
    type Vtable = INetworkConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00005_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnection_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkConnectionCost(::windows::core::IUnknown);
impl INetworkConnectionCost {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetCost(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCost)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPlanStatus(&self) -> ::windows::core::Result<NLM_DATAPLAN_STATUS> {
        let mut result__: NLM_DATAPLAN_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataPlanStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_DATAPLAN_STATUS>(result__)
    }
}
impl ::core::convert::From<INetworkConnectionCost> for ::windows::core::IUnknown {
    fn from(value: INetworkConnectionCost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkConnectionCost> for ::windows::core::IUnknown {
    fn from(value: &INetworkConnectionCost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkConnectionCost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkConnectionCost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkConnectionCost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkConnectionCost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkConnectionCost {}
impl ::core::fmt::Debug for INetworkConnectionCost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkConnectionCost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkConnectionCost {
    type Vtable = INetworkConnectionCost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb0000a_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCost_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataPlanStatus: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkConnectionCostEvents(::windows::core::IUnknown);
impl INetworkConnectionCostEvents {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn ConnectionCostChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, connectionid: Param0, newcost: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionCostChanged)(::core::mem::transmute_copy(self), connectionid.into_param().abi(), ::core::mem::transmute(newcost)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn ConnectionDataPlanStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, connectionid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionDataPlanStatusChanged)(::core::mem::transmute_copy(self), connectionid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetworkConnectionCostEvents> for ::windows::core::IUnknown {
    fn from(value: INetworkConnectionCostEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkConnectionCostEvents> for ::windows::core::IUnknown {
    fn from(value: &INetworkConnectionCostEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkConnectionCostEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkConnectionCostEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkConnectionCostEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkConnectionCostEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkConnectionCostEvents {}
impl ::core::fmt::Debug for INetworkConnectionCostEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkConnectionCostEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkConnectionCostEvents {
    type Vtable = INetworkConnectionCostEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb0000b_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCostEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ConnectionCostChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::HRESULT,
    pub ConnectionDataPlanStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkConnectionEvents(::windows::core::IUnknown);
impl INetworkConnectionEvents {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn NetworkConnectionConnectivityChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, connectionid: Param0, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkConnectionConnectivityChanged)(::core::mem::transmute_copy(self), connectionid.into_param().abi(), ::core::mem::transmute(newconnectivity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn NetworkConnectionPropertyChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, connectionid: Param0, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkConnectionPropertyChanged)(::core::mem::transmute_copy(self), connectionid.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<INetworkConnectionEvents> for ::windows::core::IUnknown {
    fn from(value: INetworkConnectionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkConnectionEvents> for ::windows::core::IUnknown {
    fn from(value: &INetworkConnectionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkConnectionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkConnectionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkConnectionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkConnectionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkConnectionEvents {}
impl ::core::fmt::Debug for INetworkConnectionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkConnectionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkConnectionEvents {
    type Vtable = INetworkConnectionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00007_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub NetworkConnectionConnectivityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub NetworkConnectionPropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkCostManager(::windows::core::IUnknown);
impl INetworkCostManager {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCost)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcost), ::core::mem::transmute(pdestipaddr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataPlanStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdataplanstatus), ::core::mem::transmute(pdestipaddr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn SetDestinationAddresses(&self, pdestipaddrlist: &[NLM_SOCKADDR], bappend: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDestinationAddresses)(::core::mem::transmute_copy(self), pdestipaddrlist.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pdestipaddrlist)), ::core::mem::transmute(bappend)).ok()
    }
}
impl ::core::convert::From<INetworkCostManager> for ::windows::core::IUnknown {
    fn from(value: INetworkCostManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkCostManager> for ::windows::core::IUnknown {
    fn from(value: &INetworkCostManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkCostManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkCostManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkCostManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkCostManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkCostManager {}
impl ::core::fmt::Debug for INetworkCostManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkCostManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkCostManager {
    type Vtable = INetworkCostManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00008_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataPlanStatus: usize,
    pub SetDestinationAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkCostManagerEvents(::windows::core::IUnknown);
impl INetworkCostManagerEvents {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CostChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newcost), ::core::mem::transmute(pdestaddr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DataPlanStatusChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdestaddr)).ok()
    }
}
impl ::core::convert::From<INetworkCostManagerEvents> for ::windows::core::IUnknown {
    fn from(value: INetworkCostManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkCostManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &INetworkCostManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkCostManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkCostManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkCostManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkCostManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkCostManagerEvents {}
impl ::core::fmt::Debug for INetworkCostManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkCostManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkCostManagerEvents {
    type Vtable = INetworkCostManagerEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00009_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CostChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
    pub DataPlanStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkEvents(::windows::core::IUnknown);
impl INetworkEvents {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn NetworkAdded<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, networkid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkAdded)(::core::mem::transmute_copy(self), networkid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn NetworkDeleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, networkid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkDeleted)(::core::mem::transmute_copy(self), networkid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn NetworkConnectivityChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, networkid: Param0, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkConnectivityChanged)(::core::mem::transmute_copy(self), networkid.into_param().abi(), ::core::mem::transmute(newconnectivity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn NetworkPropertyChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, networkid: Param0, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkPropertyChanged)(::core::mem::transmute_copy(self), networkid.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<INetworkEvents> for ::windows::core::IUnknown {
    fn from(value: INetworkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkEvents> for ::windows::core::IUnknown {
    fn from(value: &INetworkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkEvents {}
impl ::core::fmt::Debug for INetworkEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkEvents {
    type Vtable = INetworkEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00004_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub NetworkAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub NetworkDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub NetworkConnectivityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub NetworkPropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetworkListManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetworkListManager {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> ::windows::core::Result<IEnumNetworks> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworks)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetworks>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, gdnetworkid: Param0) -> ::windows::core::Result<INetwork> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetwork)(::core::mem::transmute_copy(self), gdnetworkid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetwork>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkConnections)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetworkConnections>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnection<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, gdnetworkconnectionid: Param0) -> ::windows::core::Result<INetworkConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkConnection)(::core::mem::transmute_copy(self), gdnetworkconnectionid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetworkConnection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnectedToInternet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__: NLM_CONNECTIVITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectivity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NLM_CONNECTIVITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSimulatedProfileInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(psimulatedinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn ClearSimulatedProfileInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearSimulatedProfileInfo)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetworkListManager> for ::windows::core::IUnknown {
    fn from(value: INetworkListManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetworkListManager> for ::windows::core::IUnknown {
    fn from(value: &INetworkListManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkListManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkListManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetworkListManager> for super::super::System::Com::IDispatch {
    fn from(value: INetworkListManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetworkListManager> for super::super::System::Com::IDispatch {
    fn from(value: &INetworkListManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for INetworkListManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a INetworkListManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetworkListManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INetworkListManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INetworkListManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INetworkListManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkListManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for INetworkListManager {
    type Vtable = INetworkListManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00000_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows::core::GUID, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows::core::GUID, ppnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnection: usize,
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub SetSimulatedProfileInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::HRESULT,
    pub ClearSimulatedProfileInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkListManagerEvents(::windows::core::IUnknown);
impl INetworkListManagerEvents {
    #[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
    pub unsafe fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectivityChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newconnectivity)).ok()
    }
}
impl ::core::convert::From<INetworkListManagerEvents> for ::windows::core::IUnknown {
    fn from(value: INetworkListManagerEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetworkListManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &INetworkListManagerEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkListManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkListManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetworkListManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetworkListManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetworkListManagerEvents {}
impl ::core::fmt::Debug for INetworkListManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkListManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetworkListManagerEvents {
    type Vtable = INetworkListManagerEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00001_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManagerEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ConnectivityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_AllowMerge: &'static str = "NA_AllowMerge";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_CategoryReadOnly: &'static str = "NA_CategoryReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_CategorySetByPolicy: &'static str = "NA_CategorySetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DescriptionReadOnly: &'static str = "NA_DescriptionReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DescriptionSetByPolicy: &'static str = "NA_DescriptionSetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DomainAuthenticationFailed: &'static str = "NA_DomainAuthenticationFailed";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_IconReadOnly: &'static str = "NA_IconReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_IconSetByPolicy: &'static str = "NA_IconSetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_InternetConnectivityV4: &'static str = "NA_InternetConnectivityV4";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_InternetConnectivityV6: &'static str = "NA_InternetConnectivityV6";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NameReadOnly: &'static str = "NA_NameReadOnly";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NameSetByPolicy: &'static str = "NA_NameSetByPolicy";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NetworkClass: &'static str = "NA_NetworkClass";
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_CONNECTION_COST(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = NLM_CONNECTION_COST(0i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(2i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = NLM_CONNECTION_COST(4i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(65536i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(131072i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = NLM_CONNECTION_COST(262144i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(524288i32);
impl ::core::marker::Copy for NLM_CONNECTION_COST {}
impl ::core::clone::Clone for NLM_CONNECTION_COST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_CONNECTION_COST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_CONNECTION_COST {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_CONNECTION_COST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_CONNECTION_COST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_CONNECTION_PROPERTY_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = NLM_CONNECTION_PROPERTY_CHANGE(1i32);
impl ::core::marker::Copy for NLM_CONNECTION_PROPERTY_CHANGE {}
impl ::core::clone::Clone for NLM_CONNECTION_PROPERTY_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_CONNECTION_PROPERTY_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_CONNECTION_PROPERTY_CHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_CONNECTION_PROPERTY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_CONNECTION_PROPERTY_CHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_CONNECTIVITY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = NLM_CONNECTIVITY(0i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(2i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(16i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(32i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(64i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(256i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(512i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1024i32);
impl ::core::marker::Copy for NLM_CONNECTIVITY {}
impl ::core::clone::Clone for NLM_CONNECTIVITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_CONNECTIVITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_CONNECTIVITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_CONNECTIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_CONNECTIVITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: ::windows::core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NLM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NLM_DATAPLAN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NLM_DATAPLAN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLM_DATAPLAN_STATUS").field("InterfaceGuid", &self.InterfaceGuid).field("UsageData", &self.UsageData).field("DataLimitInMegabytes", &self.DataLimitInMegabytes).field("InboundBandwidthInKbps", &self.InboundBandwidthInKbps).field("OutboundBandwidthInKbps", &self.OutboundBandwidthInKbps).field("NextBillingCycle", &self.NextBillingCycle).field("MaxTransferSizeInMegabytes", &self.MaxTransferSizeInMegabytes).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NLM_DATAPLAN_STATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NLM_DATAPLAN_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NLM_DATAPLAN_STATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NLM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NLM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_DOMAIN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(2i32);
impl ::core::marker::Copy for NLM_DOMAIN_TYPE {}
impl ::core::clone::Clone for NLM_DOMAIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_DOMAIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_DOMAIN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_DOMAIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_DOMAIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_ENUM_NETWORK(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(2i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(3i32);
impl ::core::marker::Copy for NLM_ENUM_NETWORK {}
impl ::core::clone::Clone for NLM_ENUM_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_ENUM_NETWORK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_ENUM_NETWORK {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_ENUM_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_ENUM_NETWORK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_INTERNET_CONNECTIVITY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(2i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(4i32);
impl ::core::marker::Copy for NLM_INTERNET_CONNECTIVITY {}
impl ::core::clone::Clone for NLM_INTERNET_CONNECTIVITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_INTERNET_CONNECTIVITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_INTERNET_CONNECTIVITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_INTERNET_CONNECTIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_INTERNET_CONNECTIVITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_NETWORK_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(2i32);
impl ::core::marker::Copy for NLM_NETWORK_CATEGORY {}
impl ::core::clone::Clone for NLM_NETWORK_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_NETWORK_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_NETWORK_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_NETWORK_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_NETWORK_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(3i32);
impl ::core::marker::Copy for NLM_NETWORK_CLASS {}
impl ::core::clone::Clone for NLM_NETWORK_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_NETWORK_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_NETWORK_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_NETWORK_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLM_NETWORK_PROPERTY_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(4i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(8i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(16i32);
impl ::core::marker::Copy for NLM_NETWORK_PROPERTY_CHANGE {}
impl ::core::clone::Clone for NLM_NETWORK_PROPERTY_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_NETWORK_PROPERTY_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NLM_NETWORK_PROPERTY_CHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NLM_NETWORK_PROPERTY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_PROPERTY_CHANGE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl ::core::marker::Copy for NLM_SIMULATED_PROFILE_INFO {}
impl ::core::clone::Clone for NLM_SIMULATED_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLM_SIMULATED_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLM_SIMULATED_PROFILE_INFO").field("ProfileName", &self.ProfileName).field("cost", &self.cost).field("UsageInMegabytes", &self.UsageInMegabytes).field("DataLimitInMegabytes", &self.DataLimitInMegabytes).finish()
    }
}
unsafe impl ::windows::core::Abi for NLM_SIMULATED_PROFILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NLM_SIMULATED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NLM_SIMULATED_PROFILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for NLM_SIMULATED_PROFILE_INFO {}
impl ::core::default::Default for NLM_SIMULATED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl ::core::marker::Copy for NLM_SOCKADDR {}
impl ::core::clone::Clone for NLM_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLM_SOCKADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLM_SOCKADDR").field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for NLM_SOCKADDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NLM_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NLM_SOCKADDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NLM_SOCKADDR {}
impl ::core::default::Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NLM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NLM_USAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NLM_USAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLM_USAGE_DATA").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NLM_USAGE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NLM_USAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NLM_USAGE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NLM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NLM_USAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const NetworkListManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00c01_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
