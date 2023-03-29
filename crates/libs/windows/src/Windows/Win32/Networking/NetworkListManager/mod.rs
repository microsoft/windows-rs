#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEnumNetworkConnections(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworkConnections {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Ole::IEnumVARIANT>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetworkConnection>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__ = ::windows::core::zeroed::<IEnumNetworkConnections>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IEnumNetworkConnections, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEnumNetworkConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IEnumNetworkConnections {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00006_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworkConnections_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEnumNetworks(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworks {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Ole::IEnumVARIANT>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetwork>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetworks> {
        let mut result__ = ::windows::core::zeroed::<IEnumNetworks>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IEnumNetworks, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEnumNetworks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IEnumNetworks {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00003_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetworks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetwork(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetwork {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, sznetworknewname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), sznetworknewname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), szdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetNetworkId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetNetworkId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE> {
        let mut result__ = ::windows::core::zeroed::<NLM_DOMAIN_TYPE>();
        (::windows::core::Interface::vtable(self).GetDomainType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__ = ::windows::core::zeroed::<IEnumNetworkConnections>();
        (::windows::core::Interface::vtable(self).GetNetworkConnections)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTimeCreatedAndConnected)(::windows::core::Interface::as_raw(self), pdwlowdatetimecreated, pdwhighdatetimecreated, pdwlowdatetimeconnected, pdwhighdatetimeconnected).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsConnectedToInternet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__ = ::windows::core::zeroed::<NLM_CONNECTIVITY>();
        (::windows::core::Interface::vtable(self).GetConnectivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<NLM_NETWORK_CATEGORY> {
        let mut result__ = ::windows::core::zeroed::<NLM_NETWORK_CATEGORY>();
        (::windows::core::Interface::vtable(self).GetCategory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCategory)(::windows::core::Interface::as_raw(self), newcategory).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(INetwork, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for INetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00002_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psznetworkname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznetworknewname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetNetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    pub GetTimeCreatedAndConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnectedToInternet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetwork2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetwork2 {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, sznetworknewname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), sznetworknewname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), szdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetNetworkId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetNetworkId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE> {
        let mut result__ = ::windows::core::zeroed::<NLM_DOMAIN_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetDomainType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__ = ::windows::core::zeroed::<IEnumNetworkConnections>();
        (::windows::core::Interface::vtable(self).base__.GetNetworkConnections)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTimeCreatedAndConnected)(::windows::core::Interface::as_raw(self), pdwlowdatetimecreated, pdwhighdatetimecreated, pdwlowdatetimeconnected, pdwhighdatetimeconnected).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsConnectedToInternet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__ = ::windows::core::zeroed::<NLM_CONNECTIVITY>();
        (::windows::core::Interface::vtable(self).base__.GetConnectivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<NLM_NETWORK_CATEGORY> {
        let mut result__ = ::windows::core::zeroed::<NLM_NETWORK_CATEGORY>();
        (::windows::core::Interface::vtable(self).base__.GetCategory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCategory)(::windows::core::Interface::as_raw(self), newcategory).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsDomainAuthenticatedBy)(::windows::core::Interface::as_raw(self), domainauthenticationkind, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(INetwork2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, INetwork);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INetwork2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INetwork2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INetwork2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetwork2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for INetwork2 {
    type Vtable = INetwork2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetwork2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for INetwork2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5550abb_3391_4310_804f_25dcc325ed81);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetwork2_Vtbl {
    pub base__: INetwork_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDomainAuthenticatedBy: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetworkConnection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork(&self) -> ::windows::core::Result<INetwork> {
        let mut result__ = ::windows::core::zeroed::<INetwork>();
        (::windows::core::Interface::vtable(self).GetNetwork)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsConnectedToInternet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__ = ::windows::core::zeroed::<NLM_CONNECTIVITY>();
        (::windows::core::Interface::vtable(self).GetConnectivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetConnectionId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetAdapterId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE> {
        let mut result__ = ::windows::core::zeroed::<NLM_DOMAIN_TYPE>();
        (::windows::core::Interface::vtable(self).GetDomainType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(INetworkConnection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetworkConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for INetworkConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00005_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnectedToInternet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetworkConnection2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork(&self) -> ::windows::core::Result<INetwork> {
        let mut result__ = ::windows::core::zeroed::<INetwork>();
        (::windows::core::Interface::vtable(self).base__.GetNetwork)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsConnectedToInternet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__ = ::windows::core::zeroed::<NLM_CONNECTIVITY>();
        (::windows::core::Interface::vtable(self).base__.GetConnectivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetConnectionId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetAdapterId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE> {
        let mut result__ = ::windows::core::zeroed::<NLM_DOMAIN_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetDomainType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsDomainAuthenticatedBy)(::windows::core::Interface::as_raw(self), domainauthenticationkind, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(INetworkConnection2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, INetworkConnection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INetworkConnection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INetworkConnection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INetworkConnection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkConnection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for INetworkConnection2 {
    type Vtable = INetworkConnection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetworkConnection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for INetworkConnection2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00e676ed_5a35_4738_92eb_8581738d0f0a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnection2_Vtbl {
    pub base__: INetworkConnection_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDomainAuthenticatedBy: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkConnectionCost(::windows::core::IUnknown);
impl INetworkConnectionCost {
    pub unsafe fn GetCost(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCost)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataPlanStatus)(::windows::core::Interface::as_raw(self), pdataplanstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkConnectionCost, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkConnectionCost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkConnectionCost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb0000a_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn ConnectionCostChanged(&self, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionCostChanged)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(connectionid), newcost).ok()
    }
    pub unsafe fn ConnectionDataPlanStatusChanged(&self, connectionid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectionDataPlanStatusChanged)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(connectionid)).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkConnectionCostEvents, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkConnectionCostEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkConnectionCostEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb0000b_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionCostEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectionCostChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::HRESULT,
    pub ConnectionDataPlanStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkConnectionEvents(::windows::core::IUnknown);
impl INetworkConnectionEvents {
    pub unsafe fn NetworkConnectionConnectivityChanged(&self, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkConnectionConnectivityChanged)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(connectionid), newconnectivity).ok()
    }
    pub unsafe fn NetworkConnectionPropertyChanged(&self, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkConnectionPropertyChanged)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(connectionid), flags).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkConnectionEvents, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkConnectionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkConnectionEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00007_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkConnectionEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NetworkConnectionConnectivityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub NetworkConnectionPropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkCostManager(::windows::core::IUnknown);
impl INetworkCostManager {
    pub unsafe fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCost)(::windows::core::Interface::as_raw(self), pcost, pdestipaddr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataPlanStatus)(::windows::core::Interface::as_raw(self), pdataplanstatus, pdestipaddr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDestinationAddresses<P0>(&self, pdestipaddrlist: &[NLM_SOCKADDR], bappend: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetDestinationAddresses)(::windows::core::Interface::as_raw(self), pdestipaddrlist.len() as _, ::core::mem::transmute(pdestipaddrlist.as_ptr()), bappend.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkCostManager, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkCostManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkCostManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00008_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDataPlanStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDestinationAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDestinationAddresses: usize,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkCostManagerEvents(::windows::core::IUnknown);
impl INetworkCostManagerEvents {
    pub unsafe fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CostChanged)(::windows::core::Interface::as_raw(self), newcost, pdestaddr).ok()
    }
    pub unsafe fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DataPlanStatusChanged)(::windows::core::Interface::as_raw(self), pdestaddr).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkCostManagerEvents, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkCostManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkCostManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00009_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkCostManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CostChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
    pub DataPlanStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkEvents(::windows::core::IUnknown);
impl INetworkEvents {
    pub unsafe fn NetworkAdded(&self, networkid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkAdded)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(networkid)).ok()
    }
    pub unsafe fn NetworkDeleted(&self, networkid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkDeleted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(networkid)).ok()
    }
    pub unsafe fn NetworkConnectivityChanged(&self, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkConnectivityChanged)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(networkid), newconnectivity).ok()
    }
    pub unsafe fn NetworkPropertyChanged(&self, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NetworkPropertyChanged)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(networkid), flags).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkEvents, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00004_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> ::windows::core::Result<IEnumNetworks> {
        let mut result__ = ::windows::core::zeroed::<IEnumNetworks>();
        (::windows::core::Interface::vtable(self).GetNetworks)(::windows::core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork(&self, gdnetworkid: ::windows::core::GUID) -> ::windows::core::Result<INetwork> {
        let mut result__ = ::windows::core::zeroed::<INetwork>();
        (::windows::core::Interface::vtable(self).GetNetwork)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(gdnetworkid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections> {
        let mut result__ = ::windows::core::zeroed::<IEnumNetworkConnections>();
        (::windows::core::Interface::vtable(self).GetNetworkConnections)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnection(&self, gdnetworkconnectionid: ::windows::core::GUID) -> ::windows::core::Result<INetworkConnection> {
        let mut result__ = ::windows::core::zeroed::<INetworkConnection>();
        (::windows::core::Interface::vtable(self).GetNetworkConnection)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(gdnetworkconnectionid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnectedToInternet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsConnectedToInternet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsConnected)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY> {
        let mut result__ = ::windows::core::zeroed::<NLM_CONNECTIVITY>();
        (::windows::core::Interface::vtable(self).GetConnectivity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSimulatedProfileInfo)(::windows::core::Interface::as_raw(self), psimulatedinfo).ok()
    }
    pub unsafe fn ClearSimulatedProfileInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearSimulatedProfileInfo)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(INetworkListManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
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
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetworkListManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for INetworkListManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00000_570f_4a9b_8d69_199fdba5723b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows::core::GUID, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows::core::GUID, ppnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnectedToInternet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnectedToInternet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
    pub SetSimulatedProfileInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::HRESULT,
    pub ClearSimulatedProfileInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
pub struct INetworkListManagerEvents(::windows::core::IUnknown);
impl INetworkListManagerEvents {
    pub unsafe fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectivityChanged)(::windows::core::Interface::as_raw(self), newconnectivity).ok()
    }
}
::windows::imp::interface_hierarchy!(INetworkListManagerEvents, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for INetworkListManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkListManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00001_570f_4a9b_8d69_199fdba5723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkListManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectivityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_AllowMerge: ::windows::core::PCWSTR = ::windows::core::w!("NA_AllowMerge");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_CategoryReadOnly: ::windows::core::PCWSTR = ::windows::core::w!("NA_CategoryReadOnly");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_CategorySetByPolicy: ::windows::core::PCWSTR = ::windows::core::w!("NA_CategorySetByPolicy");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DescriptionReadOnly: ::windows::core::PCWSTR = ::windows::core::w!("NA_DescriptionReadOnly");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DescriptionSetByPolicy: ::windows::core::PCWSTR = ::windows::core::w!("NA_DescriptionSetByPolicy");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_DomainAuthenticationFailed: ::windows::core::PCWSTR = ::windows::core::w!("NA_DomainAuthenticationFailed");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_IconReadOnly: ::windows::core::PCWSTR = ::windows::core::w!("NA_IconReadOnly");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_IconSetByPolicy: ::windows::core::PCWSTR = ::windows::core::w!("NA_IconSetByPolicy");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_InternetConnectivityV4: ::windows::core::PCWSTR = ::windows::core::w!("NA_InternetConnectivityV4");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_InternetConnectivityV6: ::windows::core::PCWSTR = ::windows::core::w!("NA_InternetConnectivityV6");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NameReadOnly: ::windows::core::PCWSTR = ::windows::core::w!("NA_NameReadOnly");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NameSetByPolicy: ::windows::core::PCWSTR = ::windows::core::w!("NA_NameSetByPolicy");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NA_NetworkClass: ::windows::core::PCWSTR = ::windows::core::w!("NA_NetworkClass");
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NetworkListManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb00c01_570f_4a9b_8d69_199fdba5723b);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_CONNECTION_COST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_CONNECTION_COST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_CONNECTION_COST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_CONNECTION_PROPERTY_CHANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_CONNECTION_PROPERTY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_CONNECTION_PROPERTY_CHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_CONNECTIVITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_CONNECTIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_CONNECTIVITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NLM_DOMAIN_AUTHENTICATION_KIND(pub i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_AUTHENTICATION_KIND_NONE: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(0i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_AUTHENTICATION_KIND_LDAP: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(1i32);
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
pub const NLM_DOMAIN_AUTHENTICATION_KIND_TLS: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(2i32);
impl ::core::marker::Copy for NLM_DOMAIN_AUTHENTICATION_KIND {}
impl ::core::clone::Clone for NLM_DOMAIN_AUTHENTICATION_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLM_DOMAIN_AUTHENTICATION_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NLM_DOMAIN_AUTHENTICATION_KIND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_DOMAIN_AUTHENTICATION_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_DOMAIN_AUTHENTICATION_KIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_DOMAIN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_DOMAIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_DOMAIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_ENUM_NETWORK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_ENUM_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_ENUM_NETWORK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_INTERNET_CONNECTIVITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_INTERNET_CONNECTIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_INTERNET_CONNECTIVITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_NETWORK_CATEGORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_NETWORK_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_NETWORK_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_NETWORK_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for NLM_NETWORK_PROPERTY_CHANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NLM_NETWORK_PROPERTY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_PROPERTY_CHANGE").field(&self.0).finish()
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
impl ::windows::core::TypeKind for NLM_DATAPLAN_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NLM_DATAPLAN_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceGuid == other.InterfaceGuid && self.UsageData == other.UsageData && self.DataLimitInMegabytes == other.DataLimitInMegabytes && self.InboundBandwidthInKbps == other.InboundBandwidthInKbps && self.OutboundBandwidthInKbps == other.OutboundBandwidthInKbps && self.NextBillingCycle == other.NextBillingCycle && self.MaxTransferSizeInMegabytes == other.MaxTransferSizeInMegabytes && self.Reserved == other.Reserved
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
impl ::windows::core::TypeKind for NLM_SIMULATED_PROFILE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NLM_SIMULATED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProfileName == other.ProfileName && self.cost == other.cost && self.UsageInMegabytes == other.UsageInMegabytes && self.DataLimitInMegabytes == other.DataLimitInMegabytes
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
impl ::windows::core::TypeKind for NLM_SOCKADDR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NLM_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NLM_SOCKADDR {}
impl ::core::default::Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for NLM_USAGE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NLM_USAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UsageInMegabytes == other.UsageInMegabytes && self.LastSyncTime == other.LastSyncTime
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
