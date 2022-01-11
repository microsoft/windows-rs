#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_ACTION_SPECIFIC_BASE: u32 = 600u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_ACTION_SPECIFIC_MAX: u32 = 899u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_DEVICE_INTERNAL_ERROR: u32 = 501u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_INVALID_ACTION: u32 = 401u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_INVALID_ARG: u32 = 402u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_INVALID_SEQUENCE_NUMBER: u32 = 403u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const FAULT_INVALID_VARIABLE: u32 = 404u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HSWDEVICE(pub isize);
impl HSWDEVICE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HSWDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSWDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSWDEVICE {}
impl ::core::fmt::Debug for HSWDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSWDEVICE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HSWDEVICE {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPAddressFamilyControl(::windows::core::IUnknown);
impl IUPnPAddressFamilyControl {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn SetAddressFamily(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn GetAddressFamily(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IUPnPAddressFamilyControl> for ::windows::core::IUnknown {
    fn from(value: IUPnPAddressFamilyControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPAddressFamilyControl> for ::windows::core::IUnknown {
    fn from(value: &IUPnPAddressFamilyControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPAddressFamilyControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPAddressFamilyControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPAddressFamilyControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPAddressFamilyControl {}
impl ::core::fmt::Debug for IUPnPAddressFamilyControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPAddressFamilyControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPAddressFamilyControl {
    type Vtable = IUPnPAddressFamilyControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bf6178_694e_459f_a5a6_191ea0ffa1c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAddressFamilyControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPAsyncResult(::windows::core::IUnknown);
impl IUPnPAsyncResult {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn AsyncOperationComplete(&self, ullrequestid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid)).ok()
    }
}
impl ::core::convert::From<IUPnPAsyncResult> for ::windows::core::IUnknown {
    fn from(value: IUPnPAsyncResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPAsyncResult> for ::windows::core::IUnknown {
    fn from(value: &IUPnPAsyncResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPAsyncResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPAsyncResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPAsyncResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPAsyncResult {}
impl ::core::fmt::Debug for IUPnPAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPAsyncResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPAsyncResult {
    type Vtable = IUPnPAsyncResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d65fd08_d13e_4274_9c8b_dd8d028c8644);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPAsyncResultVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDescriptionDocument(::windows::core::IUnknown);
impl IUPnPDescriptionDocument {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn ReadyState(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, bstrurl: Param0, punkcallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrurl.into_param().abi(), punkcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn LoadResult(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn RootDevice(&self) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevice>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceByUDN<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevice>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDescriptionDocument> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDescriptionDocument> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUPnPDescriptionDocument> for ::windows::core::IUnknown {
    fn from(value: IUPnPDescriptionDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDescriptionDocument> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDescriptionDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDescriptionDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDescriptionDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDescriptionDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDescriptionDocument {}
impl ::core::fmt::Debug for IUPnPDescriptionDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDescriptionDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDescriptionDocument {
    type Vtable = IUPnPDescriptionDocumentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d1c1b2_7daa_4c9e_9595_7f82ed206d1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plreadystate: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppuddevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDescriptionDocumentCallback(::windows::core::IUnknown);
impl IUPnPDescriptionDocumentCallback {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn LoadComplete(&self, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrloadresult)).ok()
    }
}
impl ::core::convert::From<IUPnPDescriptionDocumentCallback> for ::windows::core::IUnknown {
    fn from(value: IUPnPDescriptionDocumentCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDescriptionDocumentCallback> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDescriptionDocumentCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDescriptionDocumentCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDescriptionDocumentCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDescriptionDocumentCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDescriptionDocumentCallback {}
impl ::core::fmt::Debug for IUPnPDescriptionDocumentCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDescriptionDocumentCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDescriptionDocumentCallback {
    type Vtable = IUPnPDescriptionDocumentCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77394c69_5486_40d6_9bc3_4991983e02da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDescriptionDocumentCallbackVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrloadresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDevice(::windows::core::IUnknown);
impl IUPnPDevice {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn IsRootDevice(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn RootDevice(&self) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevice>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn ParentDevice(&self) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevice>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn HasChildren(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Children(&self) -> ::windows::core::Result<IUPnPDevices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevices>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueDeviceName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Type(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentationURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ManufacturerName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ManufacturerURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModelName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModelNumber(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModelURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UPC(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IconURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrencodingformat: Param0, lsizex: i32, lsizey: i32, lbitdepth: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), bstrencodingformat.into_param().abi(), ::core::mem::transmute(lsizex), ::core::mem::transmute(lsizey), ::core::mem::transmute(lbitdepth), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Services(&self) -> ::windows::core::Result<IUPnPServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPServices>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDevice> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDevice> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUPnPDevice> for ::windows::core::IUnknown {
    fn from(value: IUPnPDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDevice> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDevice {}
impl ::core::fmt::Debug for IUPnPDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDevice {
    type Vtable = IUPnPDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d44d0d1_98c9_4889_acd1_f9d674bf2221);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudrootdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuddeviceparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarb: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppudchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrencodingformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lsizex: i32, lsizey: i32, lbitdepth: i32, pbstriconurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppusservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceControl(::windows::core::IUnknown);
impl IUPnPDeviceControl {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrxmldesc: Param0, bstrdeviceidentifier: Param1, bstrinitstring: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), bstrdeviceidentifier.into_param().abi(), bstrinitstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetServiceObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0, bstrserviceid: Param1) -> ::windows::core::Result<super::super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), bstrserviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::IDispatch>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceControl> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceControl> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceControl {}
impl ::core::fmt::Debug for IUPnPDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceControl {
    type Vtable = IUPnPDeviceControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810ba_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdispservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceControlHttpHeaders(::windows::core::IUnknown);
impl IUPnPDeviceControlHttpHeaders {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdditionalResponseHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceControlHttpHeaders> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceControlHttpHeaders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceControlHttpHeaders> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceControlHttpHeaders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceControlHttpHeaders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceControlHttpHeaders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceControlHttpHeaders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceControlHttpHeaders {}
impl ::core::fmt::Debug for IUPnPDeviceControlHttpHeaders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceControlHttpHeaders").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceControlHttpHeaders {
    type Vtable = IUPnPDeviceControlHttpHeadersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810bb_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceControlHttpHeadersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpresponseheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccess(::windows::core::IUnknown);
impl IUPnPDeviceDocumentAccess {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceDocumentAccess> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceDocumentAccess> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceDocumentAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceDocumentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceDocumentAccess {}
impl ::core::fmt::Debug for IUPnPDeviceDocumentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceDocumentAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceDocumentAccess {
    type Vtable = IUPnPDeviceDocumentAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7772804_3287_418e_9072_cf2b47238981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceDocumentAccessEx(::windows::core::IUnknown);
impl IUPnPDeviceDocumentAccessEx {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPDeviceDocumentAccessEx> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceDocumentAccessEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceDocumentAccessEx> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceDocumentAccessEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceDocumentAccessEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceDocumentAccessEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceDocumentAccessEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceDocumentAccessEx {}
impl ::core::fmt::Debug for IUPnPDeviceDocumentAccessEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceDocumentAccessEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceDocumentAccessEx {
    type Vtable = IUPnPDeviceDocumentAccessExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4bc4050_6178_4bd1_a4b8_6398321f3247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceDocumentAccessExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocument: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinder(::windows::core::IUnknown);
impl IUPnPDeviceFinder {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindByType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrtypeuri: Param0, dwflags: u32) -> ::windows::core::Result<IUPnPDevices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrtypeuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevices>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAsyncFind<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, bstrtypeuri: Param0, dwflags: u32, punkdevicefindercallback: Param2) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrtypeuri.into_param().abi(), ::core::mem::transmute(dwflags), punkdevicefindercallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn StartAsyncFind(&self, lfinddata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn CancelAsyncFind(&self, lfinddata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindByUDN<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevice>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDeviceFinder> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDeviceFinder> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUPnPDeviceFinder> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceFinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceFinder> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceFinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceFinder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceFinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinder {}
impl ::core::fmt::Debug for IUPnPDeviceFinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceFinder {
    type Vtable = IUPnPDeviceFinderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadda3d55_6f72_4319_bff9_18600a539b10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, pdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypeuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32, punkdevicefindercallback: *mut ::core::ffi::c_void, plfinddata: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinderAddCallbackWithInterface(::windows::core::IUnknown);
impl IUPnPDeviceFinderAddCallbackWithInterface {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn DeviceAddedWithInterface<'a, Param1: ::windows::core::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata), pdevice.into_param().abi(), ::core::mem::transmute(pguidinterface)).ok()
    }
}
impl ::core::convert::From<IUPnPDeviceFinderAddCallbackWithInterface> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceFinderAddCallbackWithInterface> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceFinderAddCallbackWithInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceFinderAddCallbackWithInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceFinderAddCallbackWithInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinderAddCallbackWithInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinderAddCallbackWithInterface {}
impl ::core::fmt::Debug for IUPnPDeviceFinderAddCallbackWithInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinderAddCallbackWithInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceFinderAddCallbackWithInterface {
    type Vtable = IUPnPDeviceFinderAddCallbackWithInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x983dfc0b_1796_44df_8975_ca545b620ee5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderAddCallbackWithInterfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr, pguidinterface: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceFinderCallback(::windows::core::IUnknown);
impl IUPnPDeviceFinderCallback {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn DeviceAdded<'a, Param1: ::windows::core::IntoParam<'a, IUPnPDevice>>(&self, lfinddata: i32, pdevice: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata), pdevice.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceRemoved<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, lfinddata: i32, bstrudn: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata), bstrudn.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn SearchComplete(&self, lfinddata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfinddata)).ok()
    }
}
impl ::core::convert::From<IUPnPDeviceFinderCallback> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceFinderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceFinderCallback> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceFinderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceFinderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceFinderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceFinderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceFinderCallback {}
impl ::core::fmt::Debug for IUPnPDeviceFinderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceFinderCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceFinderCallback {
    type Vtable = IUPnPDeviceFinderCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x415a984a_88b3_49f3_92af_0508bedf0d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceFinderCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfinddata: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDeviceProvider(::windows::core::IUnknown);
impl IUPnPDeviceProvider {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrinitstring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrinitstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUPnPDeviceProvider> for ::windows::core::IUnknown {
    fn from(value: IUPnPDeviceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDeviceProvider> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDeviceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDeviceProvider {}
impl ::core::fmt::Debug for IUPnPDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDeviceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDeviceProvider {
    type Vtable = IUPnPDeviceProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b8_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDeviceProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPDevices(::windows::core::IUnknown);
impl IUPnPDevices {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrudn: Param0) -> ::windows::core::Result<IUPnPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrudn.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPDevice>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPDevices> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPDevices> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUPnPDevices> for ::windows::core::IUnknown {
    fn from(value: IUPnPDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPDevices> for ::windows::core::IUnknown {
    fn from(value: &IUPnPDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPDevices {}
impl ::core::fmt::Debug for IUPnPDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPDevices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPDevices {
    type Vtable = IUPnPDevicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdbc0c73_bda3_4c66_ac4f_f2d96fdad68c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPDevicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPEventSink(::windows::core::IUnknown);
impl IUPnPEventSink {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn OnStateChanged(&self, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cchanges), ::core::mem::transmute(rgdispidchanges)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnStateChangedSafe<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, varsadispidchanges: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), varsadispidchanges.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPEventSink> for ::windows::core::IUnknown {
    fn from(value: IUPnPEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPEventSink> for ::windows::core::IUnknown {
    fn from(value: &IUPnPEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPEventSink {}
impl ::core::fmt::Debug for IUPnPEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPEventSink {
    type Vtable = IUPnPEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b4_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, rgdispidchanges: *const i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsadispidchanges: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPEventSource(::windows::core::IUnknown);
impl IUPnPEventSource {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pessubscriber.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::core::IntoParam<'a, IUPnPEventSink>>(&self, pessubscriber: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pessubscriber.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPEventSource> for ::windows::core::IUnknown {
    fn from(value: IUPnPEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPEventSource> for ::windows::core::IUnknown {
    fn from(value: &IUPnPEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPEventSource {}
impl ::core::fmt::Debug for IUPnPEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPEventSource {
    type Vtable = IUPnPEventSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b5_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPEventSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pessubscriber: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPHttpHeaderControl(::windows::core::IUnknown);
impl IUPnPHttpHeaderControl {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRequestHeaders<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrhttpheaders: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrhttpheaders.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPHttpHeaderControl> for ::windows::core::IUnknown {
    fn from(value: IUPnPHttpHeaderControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPHttpHeaderControl> for ::windows::core::IUnknown {
    fn from(value: &IUPnPHttpHeaderControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPHttpHeaderControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPHttpHeaderControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPHttpHeaderControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPHttpHeaderControl {}
impl ::core::fmt::Debug for IUPnPHttpHeaderControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPHttpHeaderControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPHttpHeaderControl {
    type Vtable = IUPnPHttpHeaderControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0405af4f_8b5c_447c_80f2_b75984a31f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPHttpHeaderControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhttpheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPRegistrar(::windows::core::IUnknown);
impl IUPnPRegistrar {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrxmldesc: Param0, bstrprogiddevicecontrolclass: Param1, bstrinitstring: Param2, bstrcontainerid: Param3, bstrresourcepath: Param4, nlifetime: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterRunningDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrxmldesc: Param0, punkdevicecontrol: Param1, bstrinitstring: Param2, bstrresourcepath: Param3, nlifetime: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrprovidername: Param0, bstrprogidproviderclass: Param1, bstrinitstring: Param2, bstrcontainerid: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrprovidername.into_param().abi(), bstrprogidproviderclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUniqueDeviceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrtemplateudn: Param1) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrtemplateudn.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bstrdeviceidentifier: Param0, fpermanent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), fpermanent.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrprovidername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrprovidername.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPRegistrar> for ::windows::core::IUnknown {
    fn from(value: IUPnPRegistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPRegistrar> for ::windows::core::IUnknown {
    fn from(value: &IUPnPRegistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPRegistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPRegistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPRegistrar {}
impl ::core::fmt::Debug for IUPnPRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPRegistrar {
    type Vtable = IUPnPRegistrarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b6_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRegistrarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32, pbstrdeviceidentifier: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogidproviderclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrtemplateudn: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrudn: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fpermanent: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPRemoteEndpointInfo(::windows::core::IUnknown);
impl IUPnPRemoteEndpointInfo {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDwordValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrvaluename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrvaluename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGuidValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvaluename: Param0) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrvaluename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IUPnPRemoteEndpointInfo> for ::windows::core::IUnknown {
    fn from(value: IUPnPRemoteEndpointInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPRemoteEndpointInfo> for ::windows::core::IUnknown {
    fn from(value: &IUPnPRemoteEndpointInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPRemoteEndpointInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPRemoteEndpointInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPRemoteEndpointInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPRemoteEndpointInfo {}
impl ::core::fmt::Debug for IUPnPRemoteEndpointInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPRemoteEndpointInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPRemoteEndpointInfo {
    type Vtable = IUPnPRemoteEndpointInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc92eb863_0269_4aff_9c72_75321bba2952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPRemoteEndpointInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvaluename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pguidvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPReregistrar(::windows::core::IUnknown);
impl IUPnPReregistrar {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReregisterDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrxmldesc: Param1, bstrprogiddevicecontrolclass: Param2, bstrinitstring: Param3, bstrcontainerid: Param4, bstrresourcepath: Param5, nlifetime: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), bstrprogiddevicecontrolclass.into_param().abi(), bstrinitstring.into_param().abi(), bstrcontainerid.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReregisterRunningDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdeviceidentifier: Param0, bstrxmldesc: Param1, punkdevicecontrol: Param2, bstrinitstring: Param3, bstrresourcepath: Param4, nlifetime: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrdeviceidentifier.into_param().abi(), bstrxmldesc.into_param().abi(), punkdevicecontrol.into_param().abi(), bstrinitstring.into_param().abi(), bstrresourcepath.into_param().abi(), ::core::mem::transmute(nlifetime)).ok()
    }
}
impl ::core::convert::From<IUPnPReregistrar> for ::windows::core::IUnknown {
    fn from(value: IUPnPReregistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPReregistrar> for ::windows::core::IUnknown {
    fn from(value: &IUPnPReregistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPReregistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPReregistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPReregistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPReregistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPReregistrar {}
impl ::core::fmt::Debug for IUPnPReregistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPReregistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPReregistrar {
    type Vtable = IUPnPReregistrarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b7_73b2_11d4_bf42_00b0d0118b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPReregistrarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrprogiddevicecontrolclass: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcontainerid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrxmldesc: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, punkdevicecontrol: *mut ::core::ffi::c_void, bstrinitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrresourcepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nlifetime: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPService(::windows::core::IUnknown);
impl IUPnPService {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QueryStateVariable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrvariablename: Param0) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrvariablename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InvokeAction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, bstractionname: Param0, vinactionargs: Param1, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), ::core::mem::transmute(pvoutactionargs), ::core::mem::transmute(pvretval)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeIdentifier(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn AddCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), punkcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn LastTransportStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPService> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPService> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPService {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPService {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUPnPService> for ::windows::core::IUnknown {
    fn from(value: IUPnPService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPService> for ::windows::core::IUnknown {
    fn from(value: &IUPnPService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPService {}
impl ::core::fmt::Debug for IUPnPService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPService {
    type Vtable = IUPnPServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa295019c_dc65_47dd_90dc_7fe918a1ab44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPServiceAsync(::windows::core::IUnknown);
impl IUPnPServiceAsync {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginInvokeAction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::VARIANT>, Param2: ::windows::core::IntoParam<'a, IUPnPAsyncResult>>(&self, bstractionname: Param0, vinactionargs: Param1, pasyncresult: Param2) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstractionname.into_param().abi(), vinactionargs.into_param().abi(), pasyncresult.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndInvokeAction(&self, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(pvoutactionargs), ::core::mem::transmute(pvretval)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginQueryStateVariable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, IUPnPAsyncResult>>(&self, bstrvariablename: Param0, pasyncresult: Param1) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrvariablename.into_param().abi(), pasyncresult.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndQueryStateVariable(&self, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn BeginSubscribeToEvents<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, IUPnPAsyncResult>>(&self, punkcallback: Param0, pasyncresult: Param1) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punkcallback.into_param().abi(), pasyncresult.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn EndSubscribeToEvents(&self, ullrequestid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn BeginSCPDDownload<'a, Param0: ::windows::core::IntoParam<'a, IUPnPAsyncResult>>(&self, pasyncresult: Param0) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pasyncresult.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndSCPDDownload(&self, ullrequestid: u64) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn CancelAsyncOperation(&self, ullrequestid: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullrequestid)).ok()
    }
}
impl ::core::convert::From<IUPnPServiceAsync> for ::windows::core::IUnknown {
    fn from(value: IUPnPServiceAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceAsync> for ::windows::core::IUnknown {
    fn from(value: &IUPnPServiceAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPServiceAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPServiceAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceAsync {}
impl ::core::fmt::Debug for IUPnPServiceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPServiceAsync {
    type Vtable = IUPnPServiceAsyncVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x098bdaf5_5ec1_49e7_a260_b3a11dd8680c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceAsyncVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstractionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vinactionargs: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvoutactionargs: *mut super::super::super::System::Com::VARIANT, pvretval: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvariablename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkcallback: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pullrequestid: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64, pbstrscpddoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullrequestid: u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPServiceCallback(::windows::core::IUnknown);
impl IUPnPServiceCallback {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StateVariableChanged<'a, Param0: ::windows::core::IntoParam<'a, IUPnPService>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::VARIANT>>(&self, pus: Param0, pcwszstatevarname: Param1, vavalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pus.into_param().abi(), pcwszstatevarname.into_param().abi(), vavalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn ServiceInstanceDied<'a, Param0: ::windows::core::IntoParam<'a, IUPnPService>>(&self, pus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pus.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IUPnPServiceCallback> for ::windows::core::IUnknown {
    fn from(value: IUPnPServiceCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceCallback> for ::windows::core::IUnknown {
    fn from(value: &IUPnPServiceCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPServiceCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPServiceCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceCallback {}
impl ::core::fmt::Debug for IUPnPServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPServiceCallback {
    type Vtable = IUPnPServiceCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31fadca9_ab73_464b_b67d_5c1d0f83c8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr, pcwszstatevarname: super::super::super::Foundation::PWSTR, vavalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPServiceDocumentAccess(::windows::core::IUnknown);
impl IUPnPServiceDocumentAccess {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IUPnPServiceDocumentAccess> for ::windows::core::IUnknown {
    fn from(value: IUPnPServiceDocumentAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceDocumentAccess> for ::windows::core::IUnknown {
    fn from(value: &IUPnPServiceDocumentAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPServiceDocumentAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceDocumentAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceDocumentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceDocumentAccess {}
impl ::core::fmt::Debug for IUPnPServiceDocumentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceDocumentAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPServiceDocumentAccess {
    type Vtable = IUPnPServiceDocumentAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21905529_0a5e_4589_825d_7e6d87ea6998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceDocumentAccessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdoc: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPServiceEnumProperty(::windows::core::IUnknown);
impl IUPnPServiceEnumProperty {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn SetServiceEnumProperty(&self, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask)).ok()
    }
}
impl ::core::convert::From<IUPnPServiceEnumProperty> for ::windows::core::IUnknown {
    fn from(value: IUPnPServiceEnumProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServiceEnumProperty> for ::windows::core::IUnknown {
    fn from(value: &IUPnPServiceEnumProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPServiceEnumProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServiceEnumProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServiceEnumProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServiceEnumProperty {}
impl ::core::fmt::Debug for IUPnPServiceEnumProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServiceEnumProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPServiceEnumProperty {
    type Vtable = IUPnPServiceEnumPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38873b37_91bb_49f4_b249_2e8efbb8a816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServiceEnumPropertyVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[repr(transparent)]
pub struct IUPnPServices(::windows::core::IUnknown);
impl IUPnPServices {
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrserviceid: Param0) -> ::windows::core::Result<IUPnPService> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrserviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IUPnPService>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUPnPServices> for super::super::super::System::Com::IDispatch {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUPnPServices> for super::super::super::System::Com::IDispatch {
    fn from(value: &IUPnPServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for IUPnPServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &IUPnPServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUPnPServices> for ::windows::core::IUnknown {
    fn from(value: IUPnPServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUPnPServices> for ::windows::core::IUnknown {
    fn from(value: &IUPnPServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUPnPServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUPnPServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUPnPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUPnPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUPnPServices {}
impl ::core::fmt::Debug for IUPnPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUPnPServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUPnPServices {
    type Vtable = IUPnPServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f8c8e9e_9a7a_4dc8_bc41_ff31fa374956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub type SW_DEVICE_CAPABILITIES = i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceCapabilitiesNone: SW_DEVICE_CAPABILITIES = 0i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceCapabilitiesRemovable: SW_DEVICE_CAPABILITIES = 1i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceCapabilitiesSilentInstall: SW_DEVICE_CAPABILITIES = 2i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceCapabilitiesNoDisplayInUI: SW_DEVICE_CAPABILITIES = 4i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceCapabilitiesDriverRequired: SW_DEVICE_CAPABILITIES = 8i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type SW_DEVICE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hswdevice: HSWDEVICE, createresult: ::windows::core::HRESULT, pcontext: *const ::core::ffi::c_void, pszdeviceinstanceid: super::super::super::Foundation::PWSTR)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SW_DEVICE_CREATE_INFO {
    pub cbSize: u32,
    pub pszInstanceId: super::super::super::Foundation::PWSTR,
    pub pszzHardwareIds: super::super::super::Foundation::PWSTR,
    pub pszzCompatibleIds: super::super::super::Foundation::PWSTR,
    pub pContainerId: *mut ::windows::core::GUID,
    pub CapabilityFlags: u32,
    pub pszDeviceDescription: super::super::super::Foundation::PWSTR,
    pub pszDeviceLocation: super::super::super::Foundation::PWSTR,
    pub pSecurityDescriptor: *mut super::super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SW_DEVICE_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for SW_DEVICE_CREATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SW_DEVICE_CREATE_INFO").field("cbSize", &self.cbSize).field("pszInstanceId", &self.pszInstanceId).field("pszzHardwareIds", &self.pszzHardwareIds).field("pszzCompatibleIds", &self.pszzCompatibleIds).field("pContainerId", &self.pContainerId).field("CapabilityFlags", &self.CapabilityFlags).field("pszDeviceDescription", &self.pszDeviceDescription).field("pszDeviceLocation", &self.pszDeviceLocation).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for SW_DEVICE_CREATE_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for SW_DEVICE_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SW_DEVICE_CREATE_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for SW_DEVICE_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for SW_DEVICE_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub type SW_DEVICE_LIFETIME = i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceLifetimeHandle: SW_DEVICE_LIFETIME = 0i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceLifetimeParentPresent: SW_DEVICE_LIFETIME = 1i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const SWDeviceLifetimeMax: SW_DEVICE_LIFETIME = 2i32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[inline]
pub unsafe fn SwDeviceClose<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceClose(hswdevice: HSWDEVICE);
        }
        SwDeviceClose(hswdevice.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Devices_Properties', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SwDeviceCreate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszenumeratorname: Param0, pszparentdeviceinstance: Param1, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: SW_DEVICE_CREATE_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceCreate(pszenumeratorname: super::super::super::Foundation::PWSTR, pszparentdeviceinstance: super::super::super::Foundation::PWSTR, pcreateinfo: *const SW_DEVICE_CREATE_INFO, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phswdevice: *mut isize) -> ::windows::core::HRESULT;
        }
        let mut result__: isize = ::core::mem::zeroed();
        SwDeviceCreate(pszenumeratorname.into_param().abi(), pszparentdeviceinstance.into_param().abi(), ::core::mem::transmute(pcreateinfo), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[inline]
pub unsafe fn SwDeviceGetLifetime<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0) -> ::windows::core::Result<SW_DEVICE_LIFETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceGetLifetime(hswdevice: HSWDEVICE, plifetime: *mut SW_DEVICE_LIFETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: SW_DEVICE_LIFETIME = ::core::mem::zeroed();
        SwDeviceGetLifetime(hswdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<SW_DEVICE_LIFETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDeviceInterfacePropertySet<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfacePropertySet(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::core::HRESULT;
        }
        SwDeviceInterfacePropertySet(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDeviceInterfaceRegister<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(hswdevice: Param0, pinterfaceclassguid: *const ::windows::core::GUID, pszreferencestring: Param2, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: Param5) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceRegister(hswdevice: HSWDEVICE, pinterfaceclassguid: *const ::windows::core::GUID, pszreferencestring: super::super::super::Foundation::PWSTR, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY, fenabled: super::super::super::Foundation::BOOL, ppszdeviceinterfaceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::super::Foundation::PWSTR = ::core::mem::zeroed();
        SwDeviceInterfaceRegister(hswdevice.into_param().abi(), ::core::mem::transmute(pinterfaceclassguid), pszreferencestring.into_param().abi(), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties), fenabled.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwDeviceInterfaceSetState<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(hswdevice: Param0, pszdeviceinterfaceid: Param1, fenabled: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceInterfaceSetState(hswdevice: HSWDEVICE, pszdeviceinterfaceid: super::super::super::Foundation::PWSTR, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        SwDeviceInterfaceSetState(hswdevice.into_param().abi(), pszdeviceinterfaceid.into_param().abi(), fenabled.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SwDevicePropertySet<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDevicePropertySet(hswdevice: HSWDEVICE, cpropertycount: u32, pproperties: *const super::super::Properties::DEVPROPERTY) -> ::windows::core::HRESULT;
        }
        SwDevicePropertySet(hswdevice.into_param().abi(), ::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[inline]
pub unsafe fn SwDeviceSetLifetime<'a, Param0: ::windows::core::IntoParam<'a, HSWDEVICE>>(hswdevice: Param0, lifetime: SW_DEVICE_LIFETIME) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwDeviceSetLifetime(hswdevice: HSWDEVICE, lifetime: SW_DEVICE_LIFETIME) -> ::windows::core::HRESULT;
        }
        SwDeviceSetLifetime(hswdevice.into_param().abi(), ::core::mem::transmute(lifetime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
#[inline]
pub unsafe fn SwMemFree(pmem: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwMemFree(pmem: *const ::core::ffi::c_void);
        }
        SwMemFree(::core::mem::transmute(pmem))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_ADDRESSFAMILY_BOTH: u32 = 3u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_ADDRESSFAMILY_IPv4: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_ADDRESSFAMILY_IPv6: u32 = 2u32;
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_ACTION_REQUEST_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220976i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_ACTION_SPECIFIC_BASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220736i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DEVICE_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DEVICE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220972i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DEVICE_NODE_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DEVICE_NOTREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180494i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DEVICE_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180495i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DEVICE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220969i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DUPLICATE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180511i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_DUPLICATE_SERVICE_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180510i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_ERROR_PROCESSING_RESPONSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220970i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_EVENT_SUBSCRIPTION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220223i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_ICON_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_ICON_NODE_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_ACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220985i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_ARGUMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220984i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_DESCRIPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180509i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220224i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_ICON: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180507i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_ROOT_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180505i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180508i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_VARIABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220973i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_INVALID_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180506i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_OUT_OF_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220983i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_PROTOCOL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220971i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_REQUIRED_ELEMENT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180512i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_ROOT_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_SERVICE_ELEMENT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_SERVICE_NODE_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_SUFFIX_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180504i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_TRANSPORT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220975i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_URLBASE_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180503i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_VALUE_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180496i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_E_VARIABLE_VALUE_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220974i32);
#[doc = "*Required features: 'Win32_Devices_Enumeration_Pnp'*"]
pub const UPNP_SERVICE_DELAY_SCPD_AND_SUBSCRIPTION: u32 = 1u32;
pub const UPnPDescriptionDocument: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d8a9b47_3a28_4ce2_8a4b_bd34e45bceeb);
pub const UPnPDescriptionDocumentEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33fd0563_d81a_4393_83cc_0195b1da2f91);
pub const UPnPDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa32552c5_ba61_457a_b59a_a2561e125e33);
pub const UPnPDeviceFinder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2085f28_feb7_404a_b8e7_e659bdeaaa02);
pub const UPnPDeviceFinderEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x181b54fc_380b_4a75_b3f1_4ac45e9605b0);
pub const UPnPDevices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e84ffd_ad3c_40a4_b835_0882ebcbaaa8);
pub const UPnPRegistrar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204810b9_73b2_11d4_bf42_00b0d0118b56);
pub const UPnPRemoteEndpointInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e5e84e9_4049_4244_b728_2d24227157c7);
pub const UPnPService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc624ba95_fbcb_4409_8c03_8cceec533ef1);
pub const UPnPServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0bc4b4a_a406_4efc_932f_b8546b8100cc);
