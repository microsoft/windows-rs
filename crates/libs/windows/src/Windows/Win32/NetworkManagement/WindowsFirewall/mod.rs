#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type ICS_TARGETTYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const ICSTT_NAME: ICS_TARGETTYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IDynamicPortMapping(::windows::core::IUnknown);
impl IDynamicPortMapping {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExternalIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn ExternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn InternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InternalClient(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn LeaseDuration(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lleasedurationdesired), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditInternalClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), bstrinternalclient.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enable(&self, vb: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(vb)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(linternalport)).ok()
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
impl ::core::convert::From<IDynamicPortMapping> for ::windows::core::IUnknown {
    fn from(value: IDynamicPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDynamicPortMapping> for ::windows::core::IUnknown {
    fn from(value: &IDynamicPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDynamicPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDynamicPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDynamicPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDynamicPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicPortMapping {}
impl ::core::fmt::Debug for IDynamicPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicPortMapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDynamicPortMapping {
    type Vtable = IDynamicPortMappingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fc80282_23b6_4378_9a27_cd8f17c9400c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IDynamicPortMappingCollection(::windows::core::IUnknown);
impl IDynamicPortMappingCollection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows::core::Result<IDynamicPortMapping> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDynamicPortMapping>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremotehost: Param0, lexternalport: i32, bstrprotocol: Param2, linternalport: i32, bstrinternalclient: Param4, benabled: i16, bstrdescription: Param6, lleaseduration: i32) -> ::windows::core::Result<IDynamicPortMapping> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrremotehost.into_param().abi(), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::core::mem::transmute(benabled), bstrdescription.into_param().abi(), ::core::mem::transmute(lleaseduration), ::core::mem::transmute(&mut result__)).from_abi::<IDynamicPortMapping>(result__)
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
impl ::core::convert::From<IDynamicPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: IDynamicPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDynamicPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: &IDynamicPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDynamicPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDynamicPortMappingCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDynamicPortMappingCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDynamicPortMappingCollection {}
impl ::core::fmt::Debug for IDynamicPortMappingCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDynamicPortMappingCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDynamicPortMappingCollection {
    type Vtable = IDynamicPortMappingCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb60de00f_156e_4e8d_9ec1_3a2342c10899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IEnumNetConnection(::windows::core::IUnknown);
impl IEnumNetConnection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetConnection>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetConnection {}
impl ::core::fmt::Debug for IEnumNetConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetConnection {
    type Vtable = IEnumNetConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a0_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IEnumNetSharingEveryConnection(::windows::core::IUnknown);
impl IEnumNetSharingEveryConnection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingEveryConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetSharingEveryConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingEveryConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingEveryConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingEveryConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingEveryConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingEveryConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetSharingEveryConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingEveryConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingEveryConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingEveryConnection {}
impl ::core::fmt::Debug for IEnumNetSharingEveryConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingEveryConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingEveryConnection {
    type Vtable = IEnumNetSharingEveryConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b8_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingEveryConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IEnumNetSharingPortMapping(::windows::core::IUnknown);
impl IEnumNetSharingPortMapping {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPortMapping> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetSharingPortMapping>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingPortMapping {}
impl ::core::fmt::Debug for IEnumNetSharingPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingPortMapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingPortMapping {
    type Vtable = IEnumNetSharingPortMappingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b0_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPortMappingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IEnumNetSharingPrivateConnection(::windows::core::IUnknown);
impl IEnumNetSharingPrivateConnection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPrivateConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetSharingPrivateConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingPrivateConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingPrivateConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingPrivateConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingPrivateConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingPrivateConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetSharingPrivateConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingPrivateConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingPrivateConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingPrivateConnection {}
impl ::core::fmt::Debug for IEnumNetSharingPrivateConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingPrivateConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingPrivateConnection {
    type Vtable = IEnumNetSharingPrivateConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b5_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPrivateConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IEnumNetSharingPublicConnection(::windows::core::IUnknown);
impl IEnumNetSharingPublicConnection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgvar), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPublicConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetSharingPublicConnection>(result__)
    }
}
impl ::core::convert::From<IEnumNetSharingPublicConnection> for ::windows::core::IUnknown {
    fn from(value: IEnumNetSharingPublicConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetSharingPublicConnection> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetSharingPublicConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetSharingPublicConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetSharingPublicConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetSharingPublicConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetSharingPublicConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetSharingPublicConnection {}
impl ::core::fmt::Debug for IEnumNetSharingPublicConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetSharingPublicConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumNetSharingPublicConnection {
    type Vtable = IEnumNetSharingPublicConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b4_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPublicConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INATEventManager(::windows::core::IUnknown);
impl INATEventManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetExternalIPAddressCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetNumberOfEntriesCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
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
impl ::core::convert::From<INATEventManager> for ::windows::core::IUnknown {
    fn from(value: INATEventManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INATEventManager> for ::windows::core::IUnknown {
    fn from(value: &INATEventManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INATEventManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INATEventManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INATEventManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INATEventManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INATEventManager {}
impl ::core::fmt::Debug for INATEventManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INATEventManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INATEventManager {
    type Vtable = INATEventManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624bd588_9060_4109_b0b0_1adbbcac32df);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATEventManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INATExternalIPAddressCallback(::windows::core::IUnknown);
impl INATExternalIPAddressCallback {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NewExternalIPAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnewexternalipaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrnewexternalipaddress.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INATExternalIPAddressCallback> for ::windows::core::IUnknown {
    fn from(value: INATExternalIPAddressCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INATExternalIPAddressCallback> for ::windows::core::IUnknown {
    fn from(value: &INATExternalIPAddressCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INATExternalIPAddressCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INATExternalIPAddressCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INATExternalIPAddressCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INATExternalIPAddressCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INATExternalIPAddressCallback {}
impl ::core::fmt::Debug for INATExternalIPAddressCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INATExternalIPAddressCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INATExternalIPAddressCallback {
    type Vtable = INATExternalIPAddressCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c416740_a34e_446f_ba06_abd04c3149ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATExternalIPAddressCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INATNumberOfEntriesCallback(::windows::core::IUnknown);
impl INATNumberOfEntriesCallback {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewnumberofentries)).ok()
    }
}
impl ::core::convert::From<INATNumberOfEntriesCallback> for ::windows::core::IUnknown {
    fn from(value: INATNumberOfEntriesCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INATNumberOfEntriesCallback> for ::windows::core::IUnknown {
    fn from(value: &INATNumberOfEntriesCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INATNumberOfEntriesCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INATNumberOfEntriesCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INATNumberOfEntriesCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INATNumberOfEntriesCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INATNumberOfEntriesCallback {}
impl ::core::fmt::Debug for INATNumberOfEntriesCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INATNumberOfEntriesCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INATNumberOfEntriesCallback {
    type Vtable = INATNumberOfEntriesCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc83a0a74_91ee_41b6_b67a_67e0f00bbd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATNumberOfEntriesCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows::core::HRESULT);
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INET_FIREWALL_AC_BINARIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INET_FIREWALL_AC_BINARIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_AC_BINARIES").field("count", &self.count).field("binaries", &self.binaries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_BINARIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_BINARIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_BINARIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for INET_FIREWALL_AC_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_AC_CAPABILITIES").field("count", &self.count).field("capabilities", &self.capabilities).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CAPABILITIES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_CAPABILITIES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CHANGE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_CHANGE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_AC_CHANGE_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CHANGE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_AC_CHANGE_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type INET_FIREWALL_AC_CHANGE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type INET_FIREWALL_AC_CREATION_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
impl ::core::marker::Copy for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_APP_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for INET_FIREWALL_APP_CONTAINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_APP_CONTAINER").field("appContainerSid", &self.appContainerSid).field("userSid", &self.userSid).field("appContainerName", &self.appContainerName).field("displayName", &self.displayName).field("description", &self.description).field("capabilities", &self.capabilities).field("binaries", &self.binaries).field("workingDirectory", &self.workingDirectory).field("packageFullName", &self.packageFullName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for INET_FIREWALL_APP_CONTAINER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_APP_CONTAINER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INET_FIREWALL_APP_CONTAINER>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetConnection(::windows::core::IUnknown);
impl INetConnection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Duplicate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwduplicatename: Param0) -> ::windows::core::Result<INetConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszwduplicatename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetConnection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<*mut NETCON_PROPERTIES> {
        let mut result__: *mut NETCON_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut NETCON_PROPERTIES>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn GetUiObjectClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Rename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwnewname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszwnewname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetConnection> for ::windows::core::IUnknown {
    fn from(value: INetConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnection> for ::windows::core::IUnknown {
    fn from(value: &INetConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnection {}
impl ::core::fmt::Debug for INetConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetConnection {
    type Vtable = INetConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a1_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwduplicatename: super::super::Foundation::PWSTR, ppcon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetConnectionConnectUi(::windows::core::IUnknown);
impl INetConnectionConnectUi {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetConnection<'a, Param0: ::windows::core::IntoParam<'a, INetConnection>>(&self, pcon: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcon.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Disconnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<INetConnectionConnectUi> for ::windows::core::IUnknown {
    fn from(value: INetConnectionConnectUi) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnectionConnectUi> for ::windows::core::IUnknown {
    fn from(value: &INetConnectionConnectUi) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnectionConnectUi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetConnectionConnectUi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnectionConnectUi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnectionConnectUi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnectionConnectUi {}
impl ::core::fmt::Debug for INetConnectionConnectUi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnectionConnectUi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetConnectionConnectUi {
    type Vtable = INetConnectionConnectUiVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a3_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionConnectUiVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcon: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetConnectionManager(::windows::core::IUnknown);
impl INetConnectionManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows::core::Result<IEnumNetConnection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetConnection>(result__)
    }
}
impl ::core::convert::From<INetConnectionManager> for ::windows::core::IUnknown {
    fn from(value: INetConnectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnectionManager> for ::windows::core::IUnknown {
    fn from(value: &INetConnectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnectionManager {}
impl ::core::fmt::Debug for INetConnectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetConnectionManager {
    type Vtable = INetConnectionManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a2_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionManagerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetConnectionProps(::windows::core::IUnknown);
impl INetConnectionProps {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Guid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<NETCON_STATUS> {
        let mut result__: NETCON_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NETCON_STATUS>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<NETCON_MEDIATYPE> {
        let mut result__: NETCON_MEDIATYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NETCON_MEDIATYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Characteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl ::core::convert::From<INetConnectionProps> for ::windows::core::IUnknown {
    fn from(value: INetConnectionProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetConnectionProps> for ::windows::core::IUnknown {
    fn from(value: &INetConnectionProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetConnectionProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetConnectionProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetConnectionProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetConnectionProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetConnectionProps {}
impl ::core::fmt::Debug for INetConnectionProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetConnectionProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetConnectionProps {
    type Vtable = INetConnectionPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4277c95_ce5b_463d_8167_5662d9bcaa72);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwAuthorizedApplication(::windows::core::IUnknown);
impl INetFwAuthorizedApplication {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessImageFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProcessImageFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: NET_FW_IP_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: NET_FW_SCOPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
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
impl ::core::convert::From<INetFwAuthorizedApplication> for ::windows::core::IUnknown {
    fn from(value: INetFwAuthorizedApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwAuthorizedApplication> for ::windows::core::IUnknown {
    fn from(value: &INetFwAuthorizedApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwAuthorizedApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwAuthorizedApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwAuthorizedApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwAuthorizedApplication {}
impl ::core::fmt::Debug for INetFwAuthorizedApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwAuthorizedApplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwAuthorizedApplication {
    type Vtable = INetFwAuthorizedApplicationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e64ffa_c2c5_444e_a301_fb5e00018050);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplicationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwAuthorizedApplications(::windows::core::IUnknown);
impl INetFwAuthorizedApplications {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, INetFwAuthorizedApplication>>(&self, app: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), app.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<INetFwAuthorizedApplication> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imagefilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetFwAuthorizedApplication>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl ::core::convert::From<INetFwAuthorizedApplications> for ::windows::core::IUnknown {
    fn from(value: INetFwAuthorizedApplications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwAuthorizedApplications> for ::windows::core::IUnknown {
    fn from(value: &INetFwAuthorizedApplications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwAuthorizedApplications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwAuthorizedApplications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwAuthorizedApplications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwAuthorizedApplications {}
impl ::core::fmt::Debug for INetFwAuthorizedApplications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwAuthorizedApplications").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwAuthorizedApplications {
    type Vtable = INetFwAuthorizedApplicationsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x644efd52_ccf9_486c_97a2_39f352570b30);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplicationsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, app: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwIcmpSettings(::windows::core::IUnknown);
impl INetFwIcmpSettings {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowOutboundDestinationUnreachable(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowRedirect(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowRedirect(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowInboundEchoRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowInboundEchoRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowOutboundTimeExceeded(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowOutboundParameterProblem(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowOutboundParameterProblem(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowOutboundSourceQuench(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowOutboundSourceQuench(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowInboundRouterRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowInboundRouterRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowInboundTimestampRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowInboundTimestampRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowInboundMaskRequest(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowInboundMaskRequest(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAllowOutboundPacketTooBig(&self, allow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(allow)).ok()
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
impl ::core::convert::From<INetFwIcmpSettings> for ::windows::core::IUnknown {
    fn from(value: INetFwIcmpSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwIcmpSettings> for ::windows::core::IUnknown {
    fn from(value: &INetFwIcmpSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwIcmpSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwIcmpSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwIcmpSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwIcmpSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwIcmpSettings {}
impl ::core::fmt::Debug for INetFwIcmpSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwIcmpSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwIcmpSettings {
    type Vtable = INetFwIcmpSettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6207b2e_7cdd_426a_951e_5e1cbc5afead);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwMgr(::windows::core::IUnknown);
impl INetFwMgr {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn LocalPolicy(&self) -> ::windows::core::Result<INetFwPolicy> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwPolicy>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn CurrentProfileType(&self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__: NET_FW_PROFILE_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn RestoreDefaults(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsPortAllowed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: Param3, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), imagefilename.into_param().abi(), ::core::mem::transmute(ipversion), ::core::mem::transmute(portnumber), localaddress.into_param().abi(), ::core::mem::transmute(ipprotocol), ::core::mem::transmute(allowed), ::core::mem::transmute(restricted)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsIcmpTypeAllowed<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, ipversion: NET_FW_IP_VERSION, localaddress: Param1, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion), localaddress.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(allowed), ::core::mem::transmute(restricted)).ok()
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
impl ::core::convert::From<INetFwMgr> for ::windows::core::IUnknown {
    fn from(value: INetFwMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwMgr> for ::windows::core::IUnknown {
    fn from(value: &INetFwMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwMgr {}
impl ::core::fmt::Debug for INetFwMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwMgr {
    type Vtable = INetFwMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7898af5_cac4_4632_a2ec_da06e5111af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwOpenPort(::windows::core::IUnknown);
impl INetFwOpenPort {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: NET_FW_IP_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<NET_FW_IP_PROTOCOL> {
        let mut result__: NET_FW_IP_PROTOCOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_IP_PROTOCOL>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Port(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetPort(&self, portnumber: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(portnumber)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: NET_FW_SCOPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn BuiltIn(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
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
impl ::core::convert::From<INetFwOpenPort> for ::windows::core::IUnknown {
    fn from(value: INetFwOpenPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwOpenPort> for ::windows::core::IUnknown {
    fn from(value: &INetFwOpenPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwOpenPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwOpenPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwOpenPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwOpenPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwOpenPort {}
impl ::core::fmt::Debug for INetFwOpenPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwOpenPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwOpenPort {
    type Vtable = INetFwOpenPortVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0483ba0_47ff_4d9c_a6d6_7741d0b195f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPortVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, builtin: *mut i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwOpenPorts(::windows::core::IUnknown);
impl INetFwOpenPorts {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, INetFwOpenPort>>(&self, port: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), port.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(portnumber), ::core::mem::transmute(ipprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<INetFwOpenPort> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(portnumber), ::core::mem::transmute(ipprotocol), ::core::mem::transmute(&mut result__)).from_abi::<INetFwOpenPort>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl ::core::convert::From<INetFwOpenPorts> for ::windows::core::IUnknown {
    fn from(value: INetFwOpenPorts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwOpenPorts> for ::windows::core::IUnknown {
    fn from(value: &INetFwOpenPorts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwOpenPorts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwOpenPorts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwOpenPorts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwOpenPorts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwOpenPorts {}
impl ::core::fmt::Debug for INetFwOpenPorts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwOpenPorts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwOpenPorts {
    type Vtable = INetFwOpenPortsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e9d7fa_e07e_430a_b19a_090ce82d92e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPortsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, port: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwPolicy(::windows::core::IUnknown);
impl INetFwPolicy {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn CurrentProfile(&self) -> ::windows::core::Result<INetFwProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows::core::Result<INetFwProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<INetFwProfile>(result__)
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
impl ::core::convert::From<INetFwPolicy> for ::windows::core::IUnknown {
    fn from(value: INetFwPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwPolicy> for ::windows::core::IUnknown {
    fn from(value: &INetFwPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwPolicy {}
impl ::core::fmt::Debug for INetFwPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwPolicy {
    type Vtable = INetFwPolicyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd46d2478_9ac9_4008_9dc7_5563ce5536cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwPolicy2(::windows::core::IUnknown);
impl INetFwPolicy2 {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn CurrentProfileTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetFirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExcludedInterfaces<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), interfaces.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetBlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(block)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetNotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Rules(&self) -> ::windows::core::Result<INetFwRules> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwRules>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn ServiceRestriction(&self) -> ::windows::core::Result<INetFwServiceRestriction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwServiceRestriction>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableRuleGroup<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1, enable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask), group.into_param().abi(), ::core::mem::transmute(enable)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRuleGroupEnabled<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, profiletypesbitmask: i32, group: Param1) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask), group.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: NET_FW_ACTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetDefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: NET_FW_ACTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetDefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletype), ::core::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRuleGroupCurrentlyEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, group: Param0) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), group.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn LocalPolicyModifyState(&self) -> ::windows::core::Result<NET_FW_MODIFY_STATE> {
        let mut result__: NET_FW_MODIFY_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_MODIFY_STATE>(result__)
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
impl ::core::convert::From<INetFwPolicy2> for ::windows::core::IUnknown {
    fn from(value: INetFwPolicy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwPolicy2> for ::windows::core::IUnknown {
    fn from(value: &INetFwPolicy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwPolicy2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwPolicy2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwPolicy2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwPolicy2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwPolicy2 {}
impl ::core::fmt::Debug for INetFwPolicy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwPolicy2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwPolicy2 {
    type Vtable = INetFwPolicy2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98325047_c671_4174_8d81_defcd3f03186);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicerestriction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwProduct(::windows::core::IUnknown);
impl INetFwProduct {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RuleCategories(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetRuleCategories<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, rulecategories: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), rulecategories.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, displayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), displayname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathToSignedProductExe(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
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
impl ::core::convert::From<INetFwProduct> for ::windows::core::IUnknown {
    fn from(value: INetFwProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwProduct> for ::windows::core::IUnknown {
    fn from(value: &INetFwProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwProduct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwProduct {}
impl ::core::fmt::Debug for INetFwProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwProduct").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwProduct {
    type Vtable = INetFwProductVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71881699_18f4_458b_b892_3ffce5e07f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProductVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwProducts(::windows::core::IUnknown);
impl INetFwProducts {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Register<'a, Param0: ::windows::core::IntoParam<'a, INetFwProduct>>(&self, product: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), product.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<INetFwProduct> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<INetFwProduct>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl ::core::convert::From<INetFwProducts> for ::windows::core::IUnknown {
    fn from(value: INetFwProducts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwProducts> for ::windows::core::IUnknown {
    fn from(value: &INetFwProducts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwProducts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwProducts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwProducts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwProducts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwProducts {}
impl ::core::fmt::Debug for INetFwProducts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwProducts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwProducts {
    type Vtable = INetFwProductsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39eb36e0_2097_40bd_8af2_63a13b525362);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProductsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, product: ::windows::core::RawPtr, registration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, product: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwProfile(::windows::core::IUnknown);
impl INetFwProfile {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__: NET_FW_PROFILE_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_PROFILE_TYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn FirewallEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetFirewallEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn ExceptionsNotAllowed(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetExceptionsNotAllowed(&self, notallowed: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(notallowed)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn NotificationsDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetNotificationsDisabled(&self, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(disabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn RemoteAdminSettings(&self) -> ::windows::core::Result<INetFwRemoteAdminSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwRemoteAdminSettings>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn IcmpSettings(&self) -> ::windows::core::Result<INetFwIcmpSettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwIcmpSettings>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows::core::Result<INetFwOpenPorts> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwOpenPorts>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Services(&self) -> ::windows::core::Result<INetFwServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwServices>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn AuthorizedApplications(&self) -> ::windows::core::Result<INetFwAuthorizedApplications> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwAuthorizedApplications>(result__)
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
impl ::core::convert::From<INetFwProfile> for ::windows::core::IUnknown {
    fn from(value: INetFwProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwProfile> for ::windows::core::IUnknown {
    fn from(value: &INetFwProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwProfile {}
impl ::core::fmt::Debug for INetFwProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwProfile {
    type Vtable = INetFwProfileVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x174a0dda_e9f9_449d_993b_21ab667ca456);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfileVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notallowed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notallowed: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmpsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, services: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwRemoteAdminSettings(::windows::core::IUnknown);
impl INetFwRemoteAdminSettings {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: NET_FW_IP_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: NET_FW_SCOPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
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
impl ::core::convert::From<INetFwRemoteAdminSettings> for ::windows::core::IUnknown {
    fn from(value: INetFwRemoteAdminSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRemoteAdminSettings> for ::windows::core::IUnknown {
    fn from(value: &INetFwRemoteAdminSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwRemoteAdminSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwRemoteAdminSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwRemoteAdminSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwRemoteAdminSettings {}
impl ::core::fmt::Debug for INetFwRemoteAdminSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRemoteAdminSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwRemoteAdminSettings {
    type Vtable = INetFwRemoteAdminSettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4becddf_6f73_4a83_b832_9c66874cd20e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwRule(::windows::core::IUnknown);
impl INetFwRule {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: NET_FW_RULE_DIRECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dir)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EdgeTraversal(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: NET_FW_ACTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(action)).ok()
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
impl ::core::convert::From<INetFwRule> for ::windows::core::IUnknown {
    fn from(value: INetFwRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRule> for ::windows::core::IUnknown {
    fn from(value: &INetFwRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwRule {}
impl ::core::fmt::Debug for INetFwRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwRule {
    type Vtable = INetFwRuleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf230d27_baba_4e42_aced_f524f22cfce2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRuleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwRule2(::windows::core::IUnknown);
impl INetFwRule2 {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: NET_FW_RULE_DIRECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dir)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EdgeTraversal(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: NET_FW_ACTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(loptions)).ok()
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
impl ::core::convert::From<INetFwRule2> for ::windows::core::IUnknown {
    fn from(value: INetFwRule2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRule2> for ::windows::core::IUnknown {
    fn from(value: &INetFwRule2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwRule2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwRule2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwRule2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwRule2 {}
impl ::core::fmt::Debug for INetFwRule2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRule2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwRule2 {
    type Vtable = INetFwRule2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c27c8da_189b_4dde_89f7_8b39a316782c);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwRule3(::windows::core::IUnknown);
impl INetFwRule3 {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, desc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), desc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, imagefilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), servicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemotePorts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, portnumbers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), portnumbers.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), localaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIcmpTypesAndCodes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, icmptypesandcodes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), icmptypesandcodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__: NET_FW_RULE_DIRECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_RULE_DIRECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dir)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, interfaces: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), interfaces.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceTypes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfacetypes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGrouping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), context.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(profiletypesbitmask)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EdgeTraversal(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION> {
        let mut result__: NET_FW_ACTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_ACTION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(action)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(loptions)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalAppPackageId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalAppPackageId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszpackageid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), wszpackageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserOwner(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserOwner<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserowner: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), wszuserowner.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserAuthorizedList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteUserAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteUserAuthorizedList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteMachineAuthorizedList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, wszuserauthlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), wszuserauthlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SecureFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(loptions)).ok()
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
impl ::core::convert::From<INetFwRule3> for ::windows::core::IUnknown {
    fn from(value: INetFwRule3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRule3> for ::windows::core::IUnknown {
    fn from(value: &INetFwRule3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwRule3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwRule3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwRule3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwRule3 {}
impl ::core::fmt::Debug for INetFwRule3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRule3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwRule3 {
    type Vtable = INetFwRule3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb21563ff_d696_4222_ab46_4e89b73ab34a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpackageid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserowner: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwRules(::windows::core::IUnknown);
impl INetFwRules {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, INetFwRule>>(&self, rule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), rule.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<INetFwRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetFwRule>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl ::core::convert::From<INetFwRules> for ::windows::core::IUnknown {
    fn from(value: INetFwRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwRules> for ::windows::core::IUnknown {
    fn from(value: &INetFwRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwRules {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwRules {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwRules {}
impl ::core::fmt::Debug for INetFwRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwRules").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwRules {
    type Vtable = INetFwRulesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c4c6277_5027_441e_afae_ca1f542da009);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRulesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwService(::windows::core::IUnknown);
impl INetFwService {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<NET_FW_SERVICE_TYPE> {
        let mut result__: NET_FW_SERVICE_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_SERVICE_TYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Customized(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION> {
        let mut result__: NET_FW_IP_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_IP_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ipversion)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE> {
        let mut result__: NET_FW_SCOPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<NET_FW_SCOPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(scope)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteAddresses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, remoteaddrs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows::core::Result<INetFwOpenPorts> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwOpenPorts>(result__)
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
impl ::core::convert::From<INetFwService> for ::windows::core::IUnknown {
    fn from(value: INetFwService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwService> for ::windows::core::IUnknown {
    fn from(value: &INetFwService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwService {}
impl ::core::fmt::Debug for INetFwService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwService {
    type Vtable = INetFwServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79fd57c8_908e_4a36_9888_d5b3f0a444cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customized: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwServiceRestriction(::windows::core::IUnknown);
impl INetFwServiceRestriction {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestrictService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, appname: Param1, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), servicename.into_param().abi(), appname.into_param().abi(), ::core::mem::transmute(restrictservice), ::core::mem::transmute(servicesidrestricted)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceRestricted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, appname: Param1) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), servicename.into_param().abi(), appname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Rules(&self) -> ::windows::core::Result<INetFwRules> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetFwRules>(result__)
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
impl ::core::convert::From<INetFwServiceRestriction> for ::windows::core::IUnknown {
    fn from(value: INetFwServiceRestriction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwServiceRestriction> for ::windows::core::IUnknown {
    fn from(value: &INetFwServiceRestriction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwServiceRestriction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwServiceRestriction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwServiceRestriction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwServiceRestriction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwServiceRestriction {}
impl ::core::fmt::Debug for INetFwServiceRestriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwServiceRestriction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwServiceRestriction {
    type Vtable = INetFwServiceRestrictionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8267bbe3_f890_491c_b7b6_2db1ef0e5d2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestrictionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetFwServices(::windows::core::IUnknown);
impl INetFwServices {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> ::windows::core::Result<INetFwService> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(svctype), ::core::mem::transmute(&mut result__)).from_abi::<INetFwService>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl ::core::convert::From<INetFwServices> for ::windows::core::IUnknown {
    fn from(value: INetFwServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetFwServices> for ::windows::core::IUnknown {
    fn from(value: &INetFwServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetFwServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetFwServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetFwServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetFwServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetFwServices {}
impl ::core::fmt::Debug for INetFwServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetFwServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetFwServices {
    type Vtable = INetFwServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79649bb4_903e_421b_94c9_79848e79f6ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingConfiguration(::windows::core::IUnknown);
impl INetSharingConfiguration {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SharingEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SharingConnectionType(&self) -> ::windows::core::Result<SHARINGCONNECTIONTYPE> {
        let mut result__: SHARINGCONNECTIONTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<SHARINGCONNECTIONTYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn DisableSharing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn InternetFirewallEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn DisableInternetFirewall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnableInternetFirewall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPortMappingCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingPortMappingCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPortMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: Param5, etargettype: ICS_TARGETTYPE) -> ::windows::core::Result<INetSharingPortMapping> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(ucipprotocol), ::core::mem::transmute(usexternalport), ::core::mem::transmute(usinternalport), ::core::mem::transmute(dwoptions), bstrtargetnameoripaddress.into_param().abi(), ::core::mem::transmute(etargettype), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingPortMapping>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn RemovePortMapping<'a, Param0: ::windows::core::IntoParam<'a, INetSharingPortMapping>>(&self, pmapping: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pmapping.into_param().abi()).ok()
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
impl ::core::convert::From<INetSharingConfiguration> for ::windows::core::IUnknown {
    fn from(value: INetSharingConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingConfiguration> for ::windows::core::IUnknown {
    fn from(value: &INetSharingConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingConfiguration {}
impl ::core::fmt::Debug for INetSharingConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingConfiguration {
    type Vtable = INetSharingConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b6_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmapping: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingEveryConnectionCollection(::windows::core::IUnknown);
impl INetSharingEveryConnectionCollection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
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
impl ::core::convert::From<INetSharingEveryConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingEveryConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingEveryConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingEveryConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingEveryConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingEveryConnectionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingEveryConnectionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingEveryConnectionCollection {}
impl ::core::fmt::Debug for INetSharingEveryConnectionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingEveryConnectionCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingEveryConnectionCollection {
    type Vtable = INetSharingEveryConnectionCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33c4643c_7811_46fa_a89a_768597bd7223);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingEveryConnectionCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingManager(::windows::core::IUnknown);
impl INetSharingManager {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn SharingInstalled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPublicConnectionCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingPublicConnectionCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPrivateConnectionCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingPrivateConnectionCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn INetSharingConfigurationForINetConnection<'a, Param0: ::windows::core::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows::core::Result<INetSharingConfiguration> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pnetconnection.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingConfiguration>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EnumEveryConnection(&self) -> ::windows::core::Result<INetSharingEveryConnectionCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingEveryConnectionCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn NetConnectionProps<'a, Param0: ::windows::core::IntoParam<'a, INetConnection>>(&self, pnetconnection: Param0) -> ::windows::core::Result<INetConnectionProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pnetconnection.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetConnectionProps>(result__)
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
impl ::core::convert::From<INetSharingManager> for ::windows::core::IUnknown {
    fn from(value: INetSharingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingManager> for ::windows::core::IUnknown {
    fn from(value: &INetSharingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingManager {}
impl ::core::fmt::Debug for INetSharingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingManager {
    type Vtable = INetSharingManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b7_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbinstalled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppnetsharingconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingPortMapping(::windows::core::IUnknown);
impl INetSharingPortMapping {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<INetSharingPortMappingProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetSharingPortMappingProps>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
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
impl ::core::convert::From<INetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: INetSharingPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingPortMapping> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingPortMapping {}
impl ::core::fmt::Debug for INetSharingPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPortMapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingPortMapping {
    type Vtable = INetSharingPortMappingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956b1_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnspmp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingPortMappingCollection(::windows::core::IUnknown);
impl INetSharingPortMappingCollection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
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
impl ::core::convert::From<INetSharingPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingPortMappingCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingPortMappingCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingPortMappingCollection {}
impl ::core::fmt::Debug for INetSharingPortMappingCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPortMappingCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingPortMappingCollection {
    type Vtable = INetSharingPortMappingCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02e4a2de_da20_4e34_89c8_ac22275a010b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingPortMappingProps(::windows::core::IUnknown);
impl INetSharingPortMappingProps {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn IPProtocol(&self) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn ExternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn InternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Options(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
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
impl ::core::convert::From<INetSharingPortMappingProps> for ::windows::core::IUnknown {
    fn from(value: INetSharingPortMappingProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingPortMappingProps> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPortMappingProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingPortMappingProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingPortMappingProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingPortMappingProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingPortMappingProps {}
impl ::core::fmt::Debug for INetSharingPortMappingProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPortMappingProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingPortMappingProps {
    type Vtable = INetSharingPortMappingPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24b7e9b5_e38f_4685_851b_00892cf5f940);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingPrivateConnectionCollection(::windows::core::IUnknown);
impl INetSharingPrivateConnectionCollection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
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
impl ::core::convert::From<INetSharingPrivateConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingPrivateConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingPrivateConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPrivateConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingPrivateConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingPrivateConnectionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingPrivateConnectionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingPrivateConnectionCollection {}
impl ::core::fmt::Debug for INetSharingPrivateConnectionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPrivateConnectionCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingPrivateConnectionCollection {
    type Vtable = INetSharingPrivateConnectionCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ae69e0_4409_402a_a2cb_e965c727f840);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPrivateConnectionCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct INetSharingPublicConnectionCollection(::windows::core::IUnknown);
impl INetSharingPublicConnectionCollection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
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
impl ::core::convert::From<INetSharingPublicConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: INetSharingPublicConnectionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetSharingPublicConnectionCollection> for ::windows::core::IUnknown {
    fn from(value: &INetSharingPublicConnectionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetSharingPublicConnectionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetSharingPublicConnectionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetSharingPublicConnectionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetSharingPublicConnectionCollection {}
impl ::core::fmt::Debug for INetSharingPublicConnectionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetSharingPublicConnectionCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetSharingPublicConnectionCollection {
    type Vtable = INetSharingPublicConnectionCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d7a6355_f372_4971_a149_bfc927be762a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPublicConnectionCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IStaticPortMapping(::windows::core::IUnknown);
impl IStaticPortMapping {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExternalIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn ExternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn InternalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InternalClient(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditInternalClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternalclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrinternalclient.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Enable(&self, vb: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(vb)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(linternalport)).ok()
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
impl ::core::convert::From<IStaticPortMapping> for ::windows::core::IUnknown {
    fn from(value: IStaticPortMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStaticPortMapping> for ::windows::core::IUnknown {
    fn from(value: &IStaticPortMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStaticPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStaticPortMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStaticPortMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStaticPortMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStaticPortMapping {}
impl ::core::fmt::Debug for IStaticPortMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStaticPortMapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStaticPortMapping {
    type Vtable = IStaticPortMappingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f10711f_729b_41e5_93b8_f21d0f818df1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IStaticPortMappingCollection(::windows::core::IUnknown);
impl IStaticPortMappingCollection {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows::core::Result<IStaticPortMapping> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IStaticPortMapping>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lexternalport: i32, bstrprotocol: Param1, linternalport: i32, bstrinternalclient: Param3, benabled: i16, bstrdescription: Param5) -> ::windows::core::Result<IStaticPortMapping> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lexternalport), bstrprotocol.into_param().abi(), ::core::mem::transmute(linternalport), bstrinternalclient.into_param().abi(), ::core::mem::transmute(benabled), bstrdescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IStaticPortMapping>(result__)
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
impl ::core::convert::From<IStaticPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: IStaticPortMappingCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStaticPortMappingCollection> for ::windows::core::IUnknown {
    fn from(value: &IStaticPortMappingCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStaticPortMappingCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStaticPortMappingCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStaticPortMappingCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStaticPortMappingCollection {}
impl ::core::fmt::Debug for IStaticPortMappingCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStaticPortMappingCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStaticPortMappingCollection {
    type Vtable = IStaticPortMappingCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd1f3e77_66d6_4664_82c7_36dbb641d0f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
#[repr(transparent)]
pub struct IUPnPNAT(::windows::core::IUnknown);
impl IUPnPNAT {
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn StaticPortMappingCollection(&self) -> ::windows::core::Result<IStaticPortMappingCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IStaticPortMappingCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn DynamicPortMappingCollection(&self) -> ::windows::core::Result<IDynamicPortMappingCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDynamicPortMappingCollection>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
    pub unsafe fn NATEventManager(&self) -> ::windows::core::Result<INATEventManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INATEventManager>(result__)
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
impl ::core::convert::From<IUPnPNAT> for ::windows::core::IUnknown {
    fn from(value: IUPnPNAT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPNAT> for ::windows::core::IUnknown {
    fn from(value: &IUPnPNAT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPNAT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPNAT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPNAT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPNAT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPNAT {}
impl ::core::fmt::Debug for IUPnPNAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPNAT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPNAT {
    type Vtable = IUPnPNATVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb171c812_cc76_485a_94d8_b6b3a2794e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPNATVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdpms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETCONMGR_ENUM_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETCONUI_CONNECT_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETCON_CHARACTERISTIC_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = 512i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = 1024i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = 2048i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = 4096i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = 8192i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = 16384i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = 32768i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = 65536i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = 131072i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = 262144i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = 983040i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = 15728640i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETCON_MEDIATYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_NONE: NETCON_MEDIATYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_DIRECT: NETCON_MEDIATYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_ISDN: NETCON_MEDIATYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_LAN: NETCON_MEDIATYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_PHONE: NETCON_MEDIATYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_TUNNEL: NETCON_MEDIATYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_PPPOE: NETCON_MEDIATYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_BRIDGE: NETCON_MEDIATYPE = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = 9i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
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
impl ::core::marker::Copy for NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETCON_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETCON_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETCON_PROPERTIES").field("guidId", &self.guidId).field("pszwName", &self.pszwName).field("pszwDeviceName", &self.pszwDeviceName).field("Status", &self.Status).field("MediaType", &self.MediaType).field("dwCharacter", &self.dwCharacter).field("clsidThisObject", &self.clsidThisObject).field("clsidUiObject", &self.clsidUiObject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETCON_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETCON_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETCON_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETCON_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_DISCONNECTED: NETCON_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_CONNECTING: NETCON_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_CONNECTED: NETCON_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_DISCONNECTING: NETCON_STATUS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = 7i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_AUTHENTICATING: NETCON_STATUS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = 9i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = 10i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = 11i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = 12i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = 13i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = 14i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCS_CONNECT_FAILED: NETCON_STATUS = 15i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETCON_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_INBOUND: NETCON_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_INTERNET: NETCON_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_LAN: NETCON_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_PHONE: NETCON_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_TUNNEL: NETCON_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NCT_BRIDGE: NETCON_TYPE = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETISO_ERROR_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NETISO_FLAG = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_FLAG_MAX: NETISO_FLAG = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_ACTION = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_AUTHENTICATE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_EDGE_TRAVERSAL_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_IP_PROTOCOL = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = 6i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = 17i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_IP_VERSION = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_MODIFY_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_POLICY_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_PROFILE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_PROFILE_TYPE2 = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = 2147483647i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_RULE_CATEGORY = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_RULE_DIRECTION = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_SCOPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type NET_FW_SERVICE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = 4i32;
pub const NetFwAuthorizedApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PAC_CHANGES_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddress: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows::core::GUID) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows::core::GUID, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = ::core::option::Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows::core::GUID, updatedaddresses: super::super::Foundation::PWSTR, append: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PNETISO_EDP_ID_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, wszenterpriseid: super::super::Foundation::PWSTR, dwerr: u32)>;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type SHARINGCONNECTIONTYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type SHARINGCONNECTION_ENUM_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const S_OBJECT_NO_LONGER_VALID: ::windows::core::HRESULT = ::windows::core::HRESULT(2i32);
pub const UPnPNAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows::core::GUID,
    pub keyword: super::super::Foundation::PWSTR,
    pub flags: u32,
    pub addresses: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS0").field("id", &self.id).field("keyword", &self.keyword).field("flags", &self.flags).field("addresses", &self.addresses).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_tag_FW_DYNAMIC_KEYWORD_ADDRESS0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: _tag_FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0").field("dynamicKeywordAddress", &self.dynamicKeywordAddress).field("next", &self.next).field("schemaVersion", &self.schemaVersion).field("originType", &self.originType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub type _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_WindowsFirewall'*"]
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 2i32;
