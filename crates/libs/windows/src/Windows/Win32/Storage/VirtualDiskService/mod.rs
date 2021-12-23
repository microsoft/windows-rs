#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_VdsLoader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c38ed61_d565_4728_aeee_c80952f0ecde);
pub const CLSID_VdsService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1933cb_86f6_4a98_8628_01be94c9a575);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IEnumVdsObject(::windows::core::IUnknown);
impl IEnumVdsObject {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Next(&self, celt: u32, ppobjectarray: *mut ::core::option::Option<::windows::core::IUnknown>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppobjectarray), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
}
impl ::core::convert::From<IEnumVdsObject> for ::windows::core::IUnknown {
    fn from(value: IEnumVdsObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumVdsObject> for ::windows::core::IUnknown {
    fn from(value: &IEnumVdsObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumVdsObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumVdsObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumVdsObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumVdsObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumVdsObject {}
impl ::core::fmt::Debug for IEnumVdsObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumVdsObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumVdsObject {
    type Vtable = IEnumVdsObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x118610b7_8d94_4030_b5b8_500889788e4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumVdsObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsAdmin(::windows::core::IUnknown);
impl IVdsAdmin {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0, providerclsid: Param1, pwszname: Param2, r#type: VDS_PROVIDER_TYPE, pwszmachinename: Param4, pwszversion: Param5, guidversionid: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), providerid.into_param().abi(), providerclsid.into_param().abi(), pwszname.into_param().abi(), ::core::mem::transmute(r#type), pwszmachinename.into_param().abi(), pwszversion.into_param().abi(), guidversionid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn UnregisterProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, providerid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), providerid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IVdsAdmin> for ::windows::core::IUnknown {
    fn from(value: IVdsAdmin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsAdmin> for ::windows::core::IUnknown {
    fn from(value: &IVdsAdmin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsAdmin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsAdmin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsAdmin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdmin {}
impl ::core::fmt::Debug for IVdsAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdmin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdmin {
    type Vtable = IVdsAdminVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd188e97d_85aa_4d33_abc6_26299a10ffc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdminVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: super::super::Foundation::PWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, guidversionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsAdviseSink(::windows::core::IUnknown);
impl IVdsAdviseSink {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn OnNotify(&self, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumberofnotifications), ::core::mem::transmute(pnotificationarray)).ok()
    }
}
impl ::core::convert::From<IVdsAdviseSink> for ::windows::core::IUnknown {
    fn from(value: IVdsAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &IVdsAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdviseSink {}
impl ::core::fmt::Debug for IVdsAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdviseSink {
    type Vtable = IVdsAdviseSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8326cd1d_cf59_4936_b786_5efc08798e25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdviseSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsAsync(::windows::core::IUnknown);
impl IVdsAsync {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Wait(&self, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(phrresult), ::core::mem::transmute(pasyncout)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryStatus(&self, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(phrresult), ::core::mem::transmute(pulpercentcompleted)).ok()
    }
}
impl ::core::convert::From<IVdsAsync> for ::windows::core::IUnknown {
    fn from(value: IVdsAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsAsync> for ::windows::core::IUnknown {
    fn from(value: &IVdsAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsAsync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAsync {}
impl ::core::fmt::Debug for IVdsAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAsync {
    type Vtable = IVdsAsyncVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d23b6d_5a55_4492_9889_397a3c2d2dbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAsyncVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsController(::windows::core::IUnknown);
impl IVdsController {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_CONTROLLER_PROP> {
        let mut result__: VDS_CONTROLLER_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_CONTROLLER_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsSubSystem>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPortProperties(&self, sportnumber: i16) -> ::windows::core::Result<VDS_PORT_PROP> {
        let mut result__: VDS_PORT_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(sportnumber), ::core::mem::transmute(&mut result__)).from_abi::<VDS_PORT_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn FlushCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn InvalidateCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IVdsController> for ::windows::core::IUnknown {
    fn from(value: IVdsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsController> for ::windows::core::IUnknown {
    fn from(value: &IVdsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsController {}
impl ::core::fmt::Debug for IVdsController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsController {
    type Vtable = IVdsControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb53d96e_dffb_474a_a078_790d1e2bc082);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsControllerControllerPort(::windows::core::IUnknown);
impl IVdsControllerControllerPort {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryControllerPorts(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
}
impl ::core::convert::From<IVdsControllerControllerPort> for ::windows::core::IUnknown {
    fn from(value: IVdsControllerControllerPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsControllerControllerPort> for ::windows::core::IUnknown {
    fn from(value: &IVdsControllerControllerPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsControllerControllerPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsControllerControllerPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsControllerControllerPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsControllerControllerPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsControllerControllerPort {}
impl ::core::fmt::Debug for IVdsControllerControllerPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsControllerControllerPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsControllerControllerPort {
    type Vtable = IVdsControllerControllerPortVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca5d735f_6bae_42c0_b30e_f2666045ce71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerControllerPortVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsControllerPort(::windows::core::IUnknown);
impl IVdsControllerPort {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_PORT_PROP> {
        let mut result__: VDS_PORT_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_PORT_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetController(&self) -> ::windows::core::Result<IVdsController> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsController>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetStatus(&self, status: VDS_PORT_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IVdsControllerPort> for ::windows::core::IUnknown {
    fn from(value: IVdsControllerPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsControllerPort> for ::windows::core::IUnknown {
    fn from(value: &IVdsControllerPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsControllerPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsControllerPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsControllerPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsControllerPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsControllerPort {}
impl ::core::fmt::Debug for IVdsControllerPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsControllerPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsControllerPort {
    type Vtable = IVdsControllerPortVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18691d0d_4e7f_43e8_92e4_cf44beeed11c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerPortVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontroller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsDrive(::windows::core::IUnknown);
impl IVdsDrive {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_DRIVE_PROP> {
        let mut result__: VDS_DRIVE_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_DRIVE_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsSubSystem>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppextentarray), ::core::mem::transmute(plnumberofextents)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IVdsDrive> for ::windows::core::IUnknown {
    fn from(value: IVdsDrive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsDrive> for ::windows::core::IUnknown {
    fn from(value: &IVdsDrive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsDrive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsDrive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsDrive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsDrive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDrive {}
impl ::core::fmt::Debug for IVdsDrive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDrive").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDrive {
    type Vtable = IVdsDriveVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff24efa4_aade_4b6b_898b_eaa6a20887c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDriveVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsDrive2(::windows::core::IUnknown);
impl IVdsDrive2 {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties2(&self) -> ::windows::core::Result<VDS_DRIVE_PROP2> {
        let mut result__: VDS_DRIVE_PROP2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_DRIVE_PROP2>(result__)
    }
}
impl ::core::convert::From<IVdsDrive2> for ::windows::core::IUnknown {
    fn from(value: IVdsDrive2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsDrive2> for ::windows::core::IUnknown {
    fn from(value: &IVdsDrive2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsDrive2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsDrive2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsDrive2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsDrive2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDrive2 {}
impl ::core::fmt::Debug for IVdsDrive2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDrive2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDrive2 {
    type Vtable = IVdsDrive2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60b5a730_addf_4436_8ca7_5769e2d1ffa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDrive2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsHwProvider(::windows::core::IUnknown);
impl IVdsHwProvider {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QuerySubSystems(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Reenumerate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IVdsHwProvider> for ::windows::core::IUnknown {
    fn from(value: IVdsHwProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsHwProvider> for ::windows::core::IUnknown {
    fn from(value: &IVdsHwProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsHwProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsHwProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsHwProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsHwProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProvider {}
impl ::core::fmt::Debug for IVdsHwProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProvider {
    type Vtable = IVdsHwProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd99bdaae_b13a_4178_9fdb_e27f16b4603e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsHwProviderPrivate(::windows::core::IUnknown);
impl IVdsHwProviderPrivate {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryIfCreatedLun<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicepath: Param0, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszdevicepath.into_param().abi(), ::core::mem::transmute(pvdsluninformation), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IVdsHwProviderPrivate> for ::windows::core::IUnknown {
    fn from(value: IVdsHwProviderPrivate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsHwProviderPrivate> for ::windows::core::IUnknown {
    fn from(value: &IVdsHwProviderPrivate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsHwProviderPrivate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsHwProviderPrivate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsHwProviderPrivate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsHwProviderPrivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderPrivate {}
impl ::core::fmt::Debug for IVdsHwProviderPrivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderPrivate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderPrivate {
    type Vtable = IVdsHwProviderPrivateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98f17bf3_9f33_4f12_8714_8b4075092c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderPrivateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicepath: super::super::Foundation::PWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsHwProviderPrivateMpio(::windows::core::IUnknown);
impl IVdsHwProviderPrivateMpio {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetAllPathStatusesFromHbaPort<'a, Param0: ::windows::core::IntoParam<'a, VDS_HBAPORT_PROP>>(&self, hbaportprop: Param0, status: VDS_PATH_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hbaportprop.into_param().abi(), ::core::mem::transmute(status)).ok()
    }
}
impl ::core::convert::From<IVdsHwProviderPrivateMpio> for ::windows::core::IUnknown {
    fn from(value: IVdsHwProviderPrivateMpio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsHwProviderPrivateMpio> for ::windows::core::IUnknown {
    fn from(value: &IVdsHwProviderPrivateMpio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsHwProviderPrivateMpio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsHwProviderPrivateMpio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsHwProviderPrivateMpio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsHwProviderPrivateMpio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderPrivateMpio {}
impl ::core::fmt::Debug for IVdsHwProviderPrivateMpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderPrivateMpio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderPrivateMpio {
    type Vtable = IVdsHwProviderPrivateMpioVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x310a7715_ac2b_4c6f_9827_3d742f351676);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderPrivateMpioVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsHwProviderStoragePools(::windows::core::IUnknown);
impl IVdsHwProviderStoragePools {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulflags), ::core::mem::transmute(ullremainingfreespace), ::core::mem::transmute(ppoolattributes), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLunInStoragePool<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: Param2, pwszunmaskinglist: Param3, phints2: *const VDS_HINTS2) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(ullsizeinbytes), storagepoolid.into_param().abi(), pwszunmaskinglist.into_param().abi(), ::core::mem::transmute(phints2), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSizeInStoragePool<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, r#type: VDS_LUN_TYPE, storagepoolid: Param1, phints2: *const VDS_HINTS2) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), storagepoolid.into_param().abi(), ::core::mem::transmute(phints2), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IVdsHwProviderStoragePools> for ::windows::core::IUnknown {
    fn from(value: IVdsHwProviderStoragePools) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsHwProviderStoragePools> for ::windows::core::IUnknown {
    fn from(value: &IVdsHwProviderStoragePools) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsHwProviderStoragePools {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsHwProviderStoragePools {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsHwProviderStoragePools {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsHwProviderStoragePools {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderStoragePools {}
impl ::core::fmt::Debug for IVdsHwProviderStoragePools {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderStoragePools").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderStoragePools {
    type Vtable = IVdsHwProviderStoragePoolsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5b5937a_f188_4c79_b86c_11c920ad11b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderStoragePoolsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsHwProviderType(::windows::core::IUnknown);
impl IVdsHwProviderType {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProviderType(&self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__: VDS_HWPROVIDER_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_HWPROVIDER_TYPE>(result__)
    }
}
impl ::core::convert::From<IVdsHwProviderType> for ::windows::core::IUnknown {
    fn from(value: IVdsHwProviderType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsHwProviderType> for ::windows::core::IUnknown {
    fn from(value: &IVdsHwProviderType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsHwProviderType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsHwProviderType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsHwProviderType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsHwProviderType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderType {}
impl ::core::fmt::Debug for IVdsHwProviderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderType {
    type Vtable = IVdsHwProviderTypeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e0f5166_542d_4fc6_947a_012174240b7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderTypeVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsHwProviderType2(::windows::core::IUnknown);
impl IVdsHwProviderType2 {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProviderType2(&self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__: VDS_HWPROVIDER_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_HWPROVIDER_TYPE>(result__)
    }
}
impl ::core::convert::From<IVdsHwProviderType2> for ::windows::core::IUnknown {
    fn from(value: IVdsHwProviderType2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsHwProviderType2> for ::windows::core::IUnknown {
    fn from(value: &IVdsHwProviderType2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsHwProviderType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsHwProviderType2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsHwProviderType2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsHwProviderType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderType2 {}
impl ::core::fmt::Debug for IVdsHwProviderType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderType2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderType2 {
    type Vtable = IVdsHwProviderType2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8190236f_c4d0_4e81_8011_d69512fcc984);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderType2Vtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsIscsiPortal(::windows::core::IUnknown);
impl IVdsIscsiPortal {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_ISCSI_PORTAL_PROP> {
        let mut result__: VDS_ISCSI_PORTAL_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_ISCSI_PORTAL_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsSubSystem>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAssociatedPortalGroups(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptunneladdress), ::core::mem::transmute(pdestinationaddress)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinitiatorportaladdress), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinitiatorportaladdress), ::core::mem::transmute(ullsecurityflags), ::core::mem::transmute(pipseckey)).ok()
    }
}
impl ::core::convert::From<IVdsIscsiPortal> for ::windows::core::IUnknown {
    fn from(value: IVdsIscsiPortal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsIscsiPortal> for ::windows::core::IUnknown {
    fn from(value: &IVdsIscsiPortal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsIscsiPortal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsIscsiPortal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsIscsiPortal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsIscsiPortal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiPortal {}
impl ::core::fmt::Debug for IVdsIscsiPortal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiPortal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiPortal {
    type Vtable = IVdsIscsiPortalVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa1499d_ec85_4a8a_a47b_ff69201fcd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortalVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsIscsiPortalGroup(::windows::core::IUnknown);
impl IVdsIscsiPortalGroup {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_ISCSI_PORTALGROUP_PROP> {
        let mut result__: VDS_ISCSI_PORTALGROUP_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_ISCSI_PORTALGROUP_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<IVdsIscsiTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsIscsiTarget>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAssociatedPortals(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn AddPortal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, portalid: Param0) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), portalid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn RemovePortal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, portalid: Param0) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), portalid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
}
impl ::core::convert::From<IVdsIscsiPortalGroup> for ::windows::core::IUnknown {
    fn from(value: IVdsIscsiPortalGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsIscsiPortalGroup> for ::windows::core::IUnknown {
    fn from(value: &IVdsIscsiPortalGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsIscsiPortalGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsIscsiPortalGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsIscsiPortalGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsIscsiPortalGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiPortalGroup {}
impl ::core::fmt::Debug for IVdsIscsiPortalGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiPortalGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiPortalGroup {
    type Vtable = IVdsIscsiPortalGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfef5f89d_a3dd_4b36_bf28_e7dde045c593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortalGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsIscsiTarget(::windows::core::IUnknown);
impl IVdsIscsiTarget {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_ISCSI_TARGET_PROP> {
        let mut result__: VDS_ISCSI_TARGET_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_ISCSI_TARGET_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsSubSystem>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryPortalGroups(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn CreatePortalGroup(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfriendlyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszfriendlyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSharedSecret<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptargetsharedsecret), pwszinitiatorname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RememberInitiatorSharedSecret<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinitiatorname: Param0, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszinitiatorname.into_param().abi(), ::core::mem::transmute(pinitiatorsharedsecret)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppwszinitiatorlist), ::core::mem::transmute(plnumberofinitiators)).ok()
    }
}
impl ::core::convert::From<IVdsIscsiTarget> for ::windows::core::IUnknown {
    fn from(value: IVdsIscsiTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsIscsiTarget> for ::windows::core::IUnknown {
    fn from(value: &IVdsIscsiTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsIscsiTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsIscsiTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsIscsiTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsIscsiTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiTarget {}
impl ::core::fmt::Debug for IVdsIscsiTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiTarget {
    type Vtable = IVdsIscsiTargetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa8f5055_83e5_4bcc_aa73_19851a36a849);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiTargetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinitiatorname: super::super::Foundation::PWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLun(::windows::core::IUnknown);
impl IVdsLun {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_LUN_PROP> {
        let mut result__: VDS_LUN_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_LUN_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsSubSystem>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdentificationData(&self) -> ::windows::core::Result<VDS_LUN_INFORMATION> {
        let mut result__: VDS_LUN_INFORMATION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_LUN_INFORMATION>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryActiveControllers(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullnumberofbytestoadd), ::core::mem::transmute(pdriveidarray), ::core::mem::transmute(lnumberofdrives), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Shrink(&self, ullnumberofbytestoremove: u64) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ullnumberofbytestoremove), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryPlexes(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn AddPlex<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, lunid: Param0) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), lunid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn RemovePlex<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, plexid: Param0) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), plexid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Recover(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszunmaskinglist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszunmaskinglist.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn AssociateControllers(&self, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactivecontrolleridarray), ::core::mem::transmute(lnumberofactivecontrollers), ::core::mem::transmute(pinactivecontrolleridarray), ::core::mem::transmute(lnumberofinactivecontrollers)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints(&self) -> ::windows::core::Result<VDS_HINTS> {
        let mut result__: VDS_HINTS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_HINTS>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(phints)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetStatus(&self, status: VDS_LUN_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryMaxLunExtendSize(&self, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdriveidarray), ::core::mem::transmute(lnumberofdrives), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IVdsLun> for ::windows::core::IUnknown {
    fn from(value: IVdsLun) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLun> for ::windows::core::IUnknown {
    fn from(value: &IVdsLun) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLun {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLun {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLun {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLun {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLun {}
impl ::core::fmt::Debug for IVdsLun {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLun").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLun {
    type Vtable = IVdsLunVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3540a9c7_e60f_4111_a840_8bba6c2c83d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lunid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszunmaskinglist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLun2(::windows::core::IUnknown);
impl IVdsLun2 {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints2(&self) -> ::windows::core::Result<VDS_HINTS2> {
        let mut result__: VDS_HINTS2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_HINTS2>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(phints2)).ok()
    }
}
impl ::core::convert::From<IVdsLun2> for ::windows::core::IUnknown {
    fn from(value: IVdsLun2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLun2> for ::windows::core::IUnknown {
    fn from(value: &IVdsLun2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLun2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLun2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLun2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLun2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLun2 {}
impl ::core::fmt::Debug for IVdsLun2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLun2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLun2 {
    type Vtable = IVdsLun2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b3a735_9efb_499a_8071_4394d9ee6fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLun2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLunControllerPorts(::windows::core::IUnknown);
impl IVdsLunControllerPorts {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn AssociateControllerPorts(&self, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactivecontrollerportidarray), ::core::mem::transmute(lnumberofactivecontrollerports), ::core::mem::transmute(pinactivecontrollerportidarray), ::core::mem::transmute(lnumberofinactivecontrollerports)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryActiveControllerPorts(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
}
impl ::core::convert::From<IVdsLunControllerPorts> for ::windows::core::IUnknown {
    fn from(value: IVdsLunControllerPorts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLunControllerPorts> for ::windows::core::IUnknown {
    fn from(value: &IVdsLunControllerPorts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLunControllerPorts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLunControllerPorts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLunControllerPorts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLunControllerPorts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunControllerPorts {}
impl ::core::fmt::Debug for IVdsLunControllerPorts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunControllerPorts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunControllerPorts {
    type Vtable = IVdsLunControllerPortsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x451fe266_da6d_406a_bb60_82e534f85aeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunControllerPortsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLunIscsi(::windows::core::IUnknown);
impl IVdsLunIscsi {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn AssociateTargets(&self, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptargetidarray), ::core::mem::transmute(lnumberoftargets)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAssociatedTargets(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
}
impl ::core::convert::From<IVdsLunIscsi> for ::windows::core::IUnknown {
    fn from(value: IVdsLunIscsi) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLunIscsi> for ::windows::core::IUnknown {
    fn from(value: &IVdsLunIscsi) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLunIscsi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLunIscsi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLunIscsi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLunIscsi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunIscsi {}
impl ::core::fmt::Debug for IVdsLunIscsi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunIscsi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunIscsi {
    type Vtable = IVdsLunIscsiVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d7c1e64_b59b_45ae_b86a_2c2cc6a42067);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunIscsiVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLunMpio(::windows::core::IUnknown);
impl IVdsLunMpio {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppaths), ::core::mem::transmute(plnumberofpaths)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppolicy), ::core::mem::transmute(pppaths), ::core::mem::transmute(plnumberofpaths)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy), ::core::mem::transmute(ppaths), ::core::mem::transmute(lnumberofpaths)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSupportedLbPolicies(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IVdsLunMpio> for ::windows::core::IUnknown {
    fn from(value: IVdsLunMpio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLunMpio> for ::windows::core::IUnknown {
    fn from(value: &IVdsLunMpio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLunMpio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLunMpio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLunMpio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLunMpio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunMpio {}
impl ::core::fmt::Debug for IVdsLunMpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunMpio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunMpio {
    type Vtable = IVdsLunMpioVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c5fbae3_333a_48a1_a982_33c15788cde3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunMpioVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLunNaming(::windows::core::IUnknown);
impl IVdsLunNaming {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfriendlyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszfriendlyname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IVdsLunNaming> for ::windows::core::IUnknown {
    fn from(value: IVdsLunNaming) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLunNaming> for ::windows::core::IUnknown {
    fn from(value: &IVdsLunNaming) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLunNaming {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLunNaming {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLunNaming {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLunNaming {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunNaming {}
impl ::core::fmt::Debug for IVdsLunNaming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunNaming").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunNaming {
    type Vtable = IVdsLunNamingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x907504cb_6b4e_4d88_a34d_17ba661fbb06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunNamingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLunNumber(::windows::core::IUnknown);
impl IVdsLunNumber {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetLunNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IVdsLunNumber> for ::windows::core::IUnknown {
    fn from(value: IVdsLunNumber) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLunNumber> for ::windows::core::IUnknown {
    fn from(value: &IVdsLunNumber) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLunNumber {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLunNumber {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLunNumber {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLunNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunNumber {}
impl ::core::fmt::Debug for IVdsLunNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunNumber").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunNumber {
    type Vtable = IVdsLunNumberVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3f95e46_54b3_41f9_b678_0f1871443a08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunNumberVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsLunPlex(::windows::core::IUnknown);
impl IVdsLunPlex {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_LUN_PLEX_PROP> {
        let mut result__: VDS_LUN_PLEX_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_LUN_PLEX_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetLun(&self) -> ::windows::core::Result<IVdsLun> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsLun>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppextentarray), ::core::mem::transmute(plnumberofextents)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints(&self) -> ::windows::core::Result<VDS_HINTS> {
        let mut result__: VDS_HINTS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_HINTS>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(phints)).ok()
    }
}
impl ::core::convert::From<IVdsLunPlex> for ::windows::core::IUnknown {
    fn from(value: IVdsLunPlex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsLunPlex> for ::windows::core::IUnknown {
    fn from(value: &IVdsLunPlex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsLunPlex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsLunPlex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsLunPlex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsLunPlex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunPlex {}
impl ::core::fmt::Debug for IVdsLunPlex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunPlex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunPlex {
    type Vtable = IVdsLunPlexVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ee1a790_5d2e_4abb_8c99_c481e8be2138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunPlexVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplun: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsMaintenance(::windows::core::IUnknown);
impl IVdsMaintenance {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(operation)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(operation)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(operation), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IVdsMaintenance> for ::windows::core::IUnknown {
    fn from(value: IVdsMaintenance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsMaintenance> for ::windows::core::IUnknown {
    fn from(value: &IVdsMaintenance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsMaintenance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsMaintenance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsMaintenance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsMaintenance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsMaintenance {}
impl ::core::fmt::Debug for IVdsMaintenance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsMaintenance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsMaintenance {
    type Vtable = IVdsMaintenanceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaebeef3_8523_47ed_a2b9_05cecce2a1ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsMaintenanceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsProvider(::windows::core::IUnknown);
impl IVdsProvider {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_PROVIDER_PROP> {
        let mut result__: VDS_PROVIDER_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_PROVIDER_PROP>(result__)
    }
}
impl ::core::convert::From<IVdsProvider> for ::windows::core::IUnknown {
    fn from(value: IVdsProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsProvider> for ::windows::core::IUnknown {
    fn from(value: &IVdsProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsProvider {}
impl ::core::fmt::Debug for IVdsProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsProvider {
    type Vtable = IVdsProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c5e575_7984_4e81_a56b_431f5f92ae42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsProviderPrivate(::windows::core::IUnknown);
impl IVdsProviderPrivate {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, objectid: Param0, r#type: VDS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), objectid.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pwszmachinename: Param0, pcallbackobject: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwszmachinename.into_param().abi(), pcallbackobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bforceunload: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bforceunload.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IVdsProviderPrivate> for ::windows::core::IUnknown {
    fn from(value: IVdsProviderPrivate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsProviderPrivate> for ::windows::core::IUnknown {
    fn from(value: &IVdsProviderPrivate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsProviderPrivate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsProviderPrivate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsProviderPrivate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsProviderPrivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsProviderPrivate {}
impl ::core::fmt::Debug for IVdsProviderPrivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsProviderPrivate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsProviderPrivate {
    type Vtable = IVdsProviderPrivateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11f3cd41_b7e8_48ff_9472_9dff018aa292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderPrivateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmachinename: super::super::Foundation::PWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsProviderSupport(::windows::core::IUnknown);
impl IVdsProviderSupport {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetVersionSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IVdsProviderSupport> for ::windows::core::IUnknown {
    fn from(value: IVdsProviderSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsProviderSupport> for ::windows::core::IUnknown {
    fn from(value: &IVdsProviderSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsProviderSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsProviderSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsProviderSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsProviderSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsProviderSupport {}
impl ::core::fmt::Debug for IVdsProviderSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsProviderSupport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsProviderSupport {
    type Vtable = IVdsProviderSupportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1732be13_e8f9_4a03_bfbc_5f616aa66ce1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderSupportVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsStoragePool(::windows::core::IUnknown);
impl IVdsStoragePool {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsProvider>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_STORAGE_POOL_PROP> {
        let mut result__: VDS_STORAGE_POOL_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_STORAGE_POOL_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes(&self) -> ::windows::core::Result<VDS_POOL_ATTRIBUTES> {
        let mut result__: VDS_POOL_ATTRIBUTES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_POOL_ATTRIBUTES>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppextentarray), ::core::mem::transmute(plnumberofextents)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAllocatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryAllocatedStoragePools(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
}
impl ::core::convert::From<IVdsStoragePool> for ::windows::core::IUnknown {
    fn from(value: IVdsStoragePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsStoragePool> for ::windows::core::IUnknown {
    fn from(value: &IVdsStoragePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsStoragePool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsStoragePool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsStoragePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsStoragePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsStoragePool {}
impl ::core::fmt::Debug for IVdsStoragePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsStoragePool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsStoragePool {
    type Vtable = IVdsStoragePoolVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932ca8cf_0eb3_4ba8_9620_22665d7f8450);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsStoragePoolVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsSubSystem(::windows::core::IUnknown);
impl IVdsSubSystem {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_SUB_SYSTEM_PROP> {
        let mut result__: VDS_SUB_SYSTEM_PROP = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_SUB_SYSTEM_PROP>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IVdsProvider>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryControllers(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryDrives(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> ::windows::core::Result<IVdsDrive> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(sbusnumber), ::core::mem::transmute(sslotnumber), ::core::mem::transmute(&mut result__)).from_abi::<IVdsDrive>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn Reenumerate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetControllerStatus(&self, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ponlinecontrolleridarray), ::core::mem::transmute(lnumberofonlinecontrollers), ::core::mem::transmute(pofflinecontrolleridarray), ::core::mem::transmute(lnumberofofflinecontrollers)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLun<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: Param4, phints: *const VDS_HINTS) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(ullsizeinbytes), ::core::mem::transmute(pdriveidarray), ::core::mem::transmute(lnumberofdrives), pwszunmaskinglist.into_param().abi(), ::core::mem::transmute(phints), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn ReplaceDrive<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, drivetobereplaced: Param0, replacementdrive: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), drivetobereplaced.into_param().abi(), replacementdrive.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pdriveidarray), ::core::mem::transmute(lnumberofdrives), ::core::mem::transmute(phints), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IVdsSubSystem> for ::windows::core::IUnknown {
    fn from(value: IVdsSubSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsSubSystem> for ::windows::core::IUnknown {
    fn from(value: &IVdsSubSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsSubSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsSubSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsSubSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsSubSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystem {}
impl ::core::fmt::Debug for IVdsSubSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystem {
    type Vtable = IVdsSubSystemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fcee2d3_6d90_4f91_80e2_a5c7caaca9d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsSubSystem2(::windows::core::IUnknown);
impl IVdsSubSystem2 {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties2(&self) -> ::windows::core::Result<VDS_SUB_SYSTEM_PROP2> {
        let mut result__: VDS_SUB_SYSTEM_PROP2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<VDS_SUB_SYSTEM_PROP2>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows::core::Result<IVdsDrive> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sbusnumber), ::core::mem::transmute(sslotnumber), ::core::mem::transmute(ulenclosurenumber), ::core::mem::transmute(&mut result__)).from_abi::<IVdsDrive>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLun2<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: Param4, phints2: *const VDS_HINTS2) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(ullsizeinbytes), ::core::mem::transmute(pdriveidarray), ::core::mem::transmute(lnumberofdrives), pwszunmaskinglist.into_param().abi(), ::core::mem::transmute(phints2), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pdriveidarray), ::core::mem::transmute(lnumberofdrives), ::core::mem::transmute(phints2), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IVdsSubSystem2> for ::windows::core::IUnknown {
    fn from(value: IVdsSubSystem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsSubSystem2> for ::windows::core::IUnknown {
    fn from(value: &IVdsSubSystem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsSubSystem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsSubSystem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsSubSystem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsSubSystem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystem2 {}
impl ::core::fmt::Debug for IVdsSubSystem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystem2 {
    type Vtable = IVdsSubSystem2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe666735_7800_4a77_9d9c_40f85b87e292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystem2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsSubSystemInterconnect(::windows::core::IUnknown);
impl IVdsSubSystemInterconnect {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn GetSupportedInterconnects(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IVdsSubSystemInterconnect> for ::windows::core::IUnknown {
    fn from(value: IVdsSubSystemInterconnect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsSubSystemInterconnect> for ::windows::core::IUnknown {
    fn from(value: &IVdsSubSystemInterconnect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsSubSystemInterconnect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsSubSystemInterconnect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsSubSystemInterconnect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsSubSystemInterconnect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemInterconnect {}
impl ::core::fmt::Debug for IVdsSubSystemInterconnect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemInterconnect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemInterconnect {
    type Vtable = IVdsSubSystemInterconnectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6fa560_c141_477b_83ba_0b6c38f7febf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemInterconnectVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsSubSystemIscsi(::windows::core::IUnknown);
impl IVdsSubSystemIscsi {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryTargets(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn QueryPortals(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumVdsObject>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsziscsiname: Param0, pwszfriendlyname: Param1) -> ::windows::core::Result<IVdsAsync> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwsziscsiname.into_param().abi(), pwszfriendlyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IVdsAsync>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
    pub unsafe fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pipseckey)).ok()
    }
}
impl ::core::convert::From<IVdsSubSystemIscsi> for ::windows::core::IUnknown {
    fn from(value: IVdsSubSystemIscsi) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsSubSystemIscsi> for ::windows::core::IUnknown {
    fn from(value: &IVdsSubSystemIscsi) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsSubSystemIscsi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsSubSystemIscsi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsSubSystemIscsi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsSubSystemIscsi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemIscsi {}
impl ::core::fmt::Debug for IVdsSubSystemIscsi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemIscsi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemIscsi {
    type Vtable = IVdsSubSystemIscsiVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0027346f_40d0_4b45_8cec_5906dc0380c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemIscsiVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsziscsiname: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
#[repr(transparent)]
pub struct IVdsSubSystemNaming(::windows::core::IUnknown);
impl IVdsSubSystemNaming {
    #[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfriendlyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszfriendlyname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IVdsSubSystemNaming> for ::windows::core::IUnknown {
    fn from(value: IVdsSubSystemNaming) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVdsSubSystemNaming> for ::windows::core::IUnknown {
    fn from(value: &IVdsSubSystemNaming) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVdsSubSystemNaming {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVdsSubSystemNaming {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVdsSubSystemNaming {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVdsSubSystemNaming {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemNaming {}
impl ::core::fmt::Debug for IVdsSubSystemNaming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemNaming").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemNaming {
    type Vtable = IVdsSubSystemNamingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d70faa3_9cd4_4900_aa20_6981b6aafc75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemNamingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const MAX_FS_NAME_SIZE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT {
    fn clone(&self) -> Self {
        Self { r#type: self.r#type, Anonymous: self.Anonymous.clone() }
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_2,
    pub cv: ::core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_5>,
    pub bvp: ::core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_0>,
    pub sv: VDS_ASYNC_OUTPUT_0_7,
    pub cl: ::core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_1>,
    pub ct: ::core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_4>,
    pub cpg: ::core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_3>,
    pub cvd: ::core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_6>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ASYNC_OUTPUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub pVolumeUnk: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_0 {
    fn clone(&self) -> Self {
        Self { pVolumeUnk: self.pVolumeUnk.clone() }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_0").field("pVolumeUnk", &self.pVolumeUnk).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pVolumeUnk == other.pVolumeUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_0 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pLunUnk: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_1 {
    fn clone(&self) -> Self {
        Self { pLunUnk: self.pLunUnk.clone() }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_1").field("pLunUnk", &self.pLunUnk).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pLunUnk == other.pLunUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_1 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub ullOffset: u64,
    pub volumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_2 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_2").field("ullOffset", &self.ullOffset).field("volumeId", &self.volumeId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ASYNC_OUTPUT_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_2 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub pPortalGroupUnk: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_3 {
    fn clone(&self) -> Self {
        Self { pPortalGroupUnk: self.pPortalGroupUnk.clone() }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_3").field("pPortalGroupUnk", &self.pPortalGroupUnk).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_3 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.pPortalGroupUnk == other.pPortalGroupUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_3 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pTargetUnk: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_4 {
    fn clone(&self) -> Self {
        Self { pTargetUnk: self.pTargetUnk.clone() }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_4").field("pTargetUnk", &self.pTargetUnk).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_4 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.pTargetUnk == other.pTargetUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_4 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pVolumeUnk: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_5 {
    fn clone(&self) -> Self {
        Self { pVolumeUnk: self.pVolumeUnk.clone() }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_5").field("pVolumeUnk", &self.pVolumeUnk).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_5 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.pVolumeUnk == other.pVolumeUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_5 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pVDiskUnk: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_6 {
    fn clone(&self) -> Self {
        Self { pVDiskUnk: self.pVDiskUnk.clone() }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_6").field("pVDiskUnk", &self.pVDiskUnk).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_6 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.pVDiskUnk == other.pVDiskUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_6 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub ullReclaimedBytes: u64,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_7 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_7").field("ullReclaimedBytes", &self.ullReclaimedBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ASYNC_OUTPUT_0_7 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ASYNC_OUTPUT_0_7>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_7 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_ASYNC_OUTPUT_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_UNKNOWN: VDS_ASYNC_OUTPUT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CREATEVOLUME: VDS_ASYNC_OUTPUT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_EXTENDVOLUME: VDS_ASYNC_OUTPUT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_SHRINKVOLUME: VDS_ASYNC_OUTPUT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_RECOVERPACK: VDS_ASYNC_OUTPUT_TYPE = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_REPLACEDISK: VDS_ASYNC_OUTPUT_TYPE = 9i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CREATEPARTITION: VDS_ASYNC_OUTPUT_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CLEAN: VDS_ASYNC_OUTPUT_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CREATELUN: VDS_ASYNC_OUTPUT_TYPE = 50i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_ADDLUNPLEX: VDS_ASYNC_OUTPUT_TYPE = 52i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_REMOVELUNPLEX: VDS_ASYNC_OUTPUT_TYPE = 53i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_EXTENDLUN: VDS_ASYNC_OUTPUT_TYPE = 54i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_SHRINKLUN: VDS_ASYNC_OUTPUT_TYPE = 55i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_RECOVERLUN: VDS_ASYNC_OUTPUT_TYPE = 56i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_LOGINTOTARGET: VDS_ASYNC_OUTPUT_TYPE = 60i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: VDS_ASYNC_OUTPUT_TYPE = 61i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CREATETARGET: VDS_ASYNC_OUTPUT_TYPE = 62i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = 63i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_DELETETARGET: VDS_ASYNC_OUTPUT_TYPE = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_ADDPORTAL: VDS_ASYNC_OUTPUT_TYPE = 65i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_REMOVEPORTAL: VDS_ASYNC_OUTPUT_TYPE = 66i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = 67i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_FORMAT: VDS_ASYNC_OUTPUT_TYPE = 101i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_CREATE_VDISK: VDS_ASYNC_OUTPUT_TYPE = 200i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_ATTACH_VDISK: VDS_ASYNC_OUTPUT_TYPE = 201i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_COMPACT_VDISK: VDS_ASYNC_OUTPUT_TYPE = 202i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_MERGE_VDISK: VDS_ASYNC_OUTPUT_TYPE = 203i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ASYNCOUT_EXPAND_VDISK: VDS_ASYNC_OUTPUT_TYPE = 204i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: VDS_NF_CONTROLLER,
    pub controllerId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_CONTROLLER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_CONTROLLER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_CONTROLLER_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_CONTROLLER_NOTIFICATION").field("ulEvent", &self.ulEvent).field("controllerId", &self.controllerId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_CONTROLLER_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_CONTROLLER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_CONTROLLER_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_CONTROLLER_NOTIFICATION {}
impl ::core::default::Default for VDS_CONTROLLER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_CONTROLLER_PROP {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_CONTROLLER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_CONTROLLER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_CONTROLLER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_CONTROLLER_PROP").field("id", &self.id).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("status", &self.status).field("health", &self.health).field("sNumberOfPorts", &self.sNumberOfPorts).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_CONTROLLER_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_CONTROLLER_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_CONTROLLER_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_CONTROLLER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_CONTROLLER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_CONTROLLER_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_CS_UNKNOWN: VDS_CONTROLLER_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_CS_ONLINE: VDS_CONTROLLER_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_CS_NOT_READY: VDS_CONTROLLER_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_CS_OFFLINE: VDS_CONTROLLER_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_CS_FAILED: VDS_CONTROLLER_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_CS_REMOVED: VDS_CONTROLLER_STATUS = 8i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: VDS_NF_DISK,
    pub diskId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DISK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DISK_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DISK_NOTIFICATION").field("ulEvent", &self.ulEvent).field("diskId", &self.diskId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_DISK_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_DISK_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_DISK_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_DISK_NOTIFICATION {}
impl ::core::default::Default for VDS_DISK_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_EXTENT {
    pub id: ::windows::core::GUID,
    pub LunId: ::windows::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_DRIVE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_EXTENT").field("id", &self.id).field("LunId", &self.LunId).field("ullSize", &self.ullSize).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_DRIVE_EXTENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_DRIVE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_DRIVE_EXTENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_DRIVE_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRF_HOTSPARE: VDS_DRIVE_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRF_ASSIGNED: VDS_DRIVE_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRF_UNASSIGNED: VDS_DRIVE_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRF_HOTSPARE_IN_USE: VDS_DRIVE_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRF_HOTSPARE_STANDBY: VDS_DRIVE_FLAG = 16i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_LETTER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DRIVE_LETTER_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_LETTER_NOTIFICATION").field("ulEvent", &self.ulEvent).field("wcLetter", &self.wcLetter).field("volumeId", &self.volumeId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_DRIVE_LETTER_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_DRIVE_LETTER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_DRIVE_LETTER_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::core::default::Default for VDS_DRIVE_LETTER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: VDS_NF_DRIVE,
    pub driveId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DRIVE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_NOTIFICATION").field("ulEvent", &self.ulEvent).field("driveId", &self.driveId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_DRIVE_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_DRIVE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_DRIVE_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_DRIVE_NOTIFICATION {}
impl ::core::default::Default for VDS_DRIVE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_PROP {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_DRIVE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_PROP").field("id", &self.id).field("ullSize", &self.ullSize).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("ulFlags", &self.ulFlags).field("status", &self.status).field("health", &self.health).field("sInternalBusNumber", &self.sInternalBusNumber).field("sSlotNumber", &self.sSlotNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_DRIVE_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_DRIVE_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_DRIVE_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_DRIVE_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_DRIVE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_PROP2 {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_DRIVE_PROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_PROP2")
            .field("id", &self.id)
            .field("ullSize", &self.ullSize)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sInternalBusNumber", &self.sInternalBusNumber)
            .field("sSlotNumber", &self.sSlotNumber)
            .field("ulEnclosureNumber", &self.ulEnclosureNumber)
            .field("busType", &self.busType)
            .field("ulSpindleSpeed", &self.ulSpindleSpeed)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_DRIVE_PROP2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_DRIVE_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_DRIVE_PROP2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_DRIVE_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_DRIVE_PROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_DRIVE_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRS_UNKNOWN: VDS_DRIVE_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRS_ONLINE: VDS_DRIVE_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRS_NOT_READY: VDS_DRIVE_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRS_OFFLINE: VDS_DRIVE_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRS_FAILED: VDS_DRIVE_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_DRS_REMOVED: VDS_DRIVE_STATUS = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212249i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ACTIVE_PARTITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212232i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ADDRESSES_INCOMPLETELY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211517i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ALIGN_BEYOND_FIRST_CYLINDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211949i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ALIGN_IS_ZERO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211888i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ALIGN_NOT_A_POWER_OF_TWO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211889i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211948i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ALIGN_NOT_ZERO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211947i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212285i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ANOTHER_CALL_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212284i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ASSOCIATED_LUNS_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211509i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ASSOCIATED_PORTALS_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211508i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ASYNC_OBJECT_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212210i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BAD_BOOT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211898i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BAD_COOKIE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212271i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BAD_LABEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212247i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BAD_PNP_MESSAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212017i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BAD_PROVIDER_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212223i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BAD_REVISION_NUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211880i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BLOCK_CLUSTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210749i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BOOT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211257i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BOOT_PAGEFILE_DRIVE_LETTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210994i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_BOOT_PARTITION_NUMBER_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212234i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CACHE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211946i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CANCEL_TOO_LATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212276i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CANNOT_CLEAR_VOLUME_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211945i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CANNOT_EXTEND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212274i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CANNOT_SHRINK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212002i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CANT_INVALIDATE_FVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211886i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CANT_QUICK_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212246i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLEAN_WITH_BOOTBACKING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210743i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLEAN_WITH_CRITICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210990i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLEAN_WITH_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210992i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLEAN_WITH_OEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210991i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLUSTER_COUNT_BEYOND_32BITS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212240i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLUSTER_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212241i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CLUSTER_SIZE_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212242i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_COMPRESSION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210984i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CONFIG_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211976i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CORRUPT_EXTENT_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212021i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CORRUPT_NOTIFICATION_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211990i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CORRUPT_PARTITION_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212023i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CORRUPT_VOLUME_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212029i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CRASHDUMP_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211250i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_CRITICAL_PLEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211906i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DELETE_WITH_BOOTBACKING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210745i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DELETE_WITH_CRITICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210993i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DEVICE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212269i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_BEING_CLEANED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211944i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_CONFIGURATION_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211975i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211974i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_CONFIGURATION_UPDATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211973i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_DYNAMIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211972i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_HAS_BANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210748i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_IN_USE_BY_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212212i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_IO_FAILING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211968i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_IS_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211254i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_IS_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211253i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211969i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_CONVERTIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211943i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_CONVERTIBLE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210971i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212268i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_FOUND_IN_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211987i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212206i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212265i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_LOADED_TO_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212217i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212031i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211883i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_NOT_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212213i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_PNP_REG_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212203i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_REMOVEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211942i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISK_REMOVEABLE_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211941i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DISTINCT_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211909i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DMADMIN_CORRUPT_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212252i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DMADMIN_METHOD_CALL_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212256i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212261i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DRIVER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212027i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DRIVER_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212004i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DRIVER_NO_PACK_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212019i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DRIVER_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211971i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DRIVE_LETTER_NOT_FREE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211940i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DUPLICATE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211986i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DUP_EMPTY_PACK_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212020i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211967i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_EXTEND_FILE_SYSTEM_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212186i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211939i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_EXTEND_TOO_MANY_CLUSTERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210968i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_EXTEND_UNKNOWN_FILESYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210967i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212011i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_EXTENT_SIZE_LESS_THAN_MIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212237i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FAILED_TO_OFFLINE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211881i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FAILED_TO_ONLINE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211882i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FAT32_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210987i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FAT_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210986i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211966i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FLAG_ALREADY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211911i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FORMAT_CRITICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210989i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210985i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FORMAT_WITH_BOOTBACKING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210744i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_FS_NOT_DETERMINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211885i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_GET_SAN_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211259i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_GPT_ATTRIBUTES_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211965i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_HIBERNATION_FILE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211251i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_IA64_BOOT_MIRRORED_TO_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212198i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_IMPORT_SET_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212207i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INCOMPATIBLE_FILE_SYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212251i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INCOMPATIBLE_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212250i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211260i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211248i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INITIALIZED_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212287i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INITIALIZE_NOT_CALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212286i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INITIATOR_ADAPTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211008i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211513i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212216i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_BLOCK_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211982i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212007i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_DISK_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211994i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_DRIVE_LETTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211938i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_DRIVE_LETTER_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211937i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_ENUMERATOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212028i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_EXTENT_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211993i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_FS_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211936i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_FS_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211935i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_IP_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210997i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_ISCSI_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210980i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_ISCSI_TARGET_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211005i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_MEMBER_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211998i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_MEMBER_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211996i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_OBJECT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211934i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_OPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212267i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212006i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PARTITION_LAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211933i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PARTITION_STYLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211932i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PARTITION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211931i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210981i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PLEX_BLOCK_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211978i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PLEX_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211999i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PLEX_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211988i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PLEX_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211997i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PLEX_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211979i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PORT_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211006i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PROVIDER_CLSID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211930i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PROVIDER_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211929i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PROVIDER_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211928i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PROVIDER_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211927i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PROVIDER_VERSION_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211926i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_PROVIDER_VERSION_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211925i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_QUERY_PROVIDER_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211924i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_SECTOR_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211984i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_SERVICE_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211923i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_SHRINK_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211241i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212282i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210747i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_STRIPE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211995i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_VOLUME_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211922i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_VOLUME_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211954i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_INVALID_VOLUME_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211899i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_IO_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212245i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_CHAP_SECRET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210998i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_GET_IKE_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211003i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_GROUP_PRESHARE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210999i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_INITIATOR_NODE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211000i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_LOGIN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211512i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_LOGOUT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211511i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_LOGOUT_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211504i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_SESSION_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211510i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ISCSI_SET_IKE_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211002i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LAST_VALID_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211985i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LBN_REMAP_ENABLED_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212202i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LDM_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212191i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LEGACY_VOLUME_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212230i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LOG_UPDATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211897i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DISK_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211239i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DISK_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211240i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DISK_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211238i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DISK_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211237i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DISK_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210978i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DYNAMIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210976i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_DYNAMIC_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210975i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211234i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211236i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211235i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_SHRINK_GPT_HEADER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210974i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_LUN_UPDATE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210977i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MAX_USABLE_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212184i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MEDIA_WRITE_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212248i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MEMBER_IS_HEALTHY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211964i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MEMBER_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211958i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MEMBER_REGENERATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211963i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MEMBER_SIZE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212010i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MIGRATE_OPEN_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212228i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MIRROR_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210973i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MISSING_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212204i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MULTIPLE_DISCOVERY_DOMAINS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211506i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_MULTIPLE_PACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212001i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NAME_NOT_UNIQUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211519i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212229i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NOT_AN_UNALLOCATED_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212264i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NOT_ENOUGH_DRIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212272i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NOT_ENOUGH_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212273i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212288i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_DISCOVERY_DOMAIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211507i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_DISKS_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212258i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_DISK_PATHNAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211505i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_DRIVELETTER_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212201i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_EXTENTS_FOR_PLEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211980i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_EXTENTS_FOR_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212218i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_FREE_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212233i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_HEALTHY_DISKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211977i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_IMPORT_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211501i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_MAINTENANCE_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210750i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212270i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_PNP_DISK_ARRIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212016i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_PNP_DISK_REMOVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212014i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_PNP_VOLUME_ARRIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212015i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_PNP_VOLUME_REMOVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212013i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_POOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210752i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_POOL_CREATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210751i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_SOFTWARE_PROVIDERS_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212032i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_VALID_LOG_COPIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211894i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_VOLUME_LAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212030i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NO_VOLUME_PATHNAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211503i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_NTFS_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210988i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OBJECT_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212277i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OBJECT_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212259i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212283i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OBJECT_OUT_OF_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212205i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OBJECT_STATUS_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212239i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OFFLINE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210970i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ONE_EXTENT_PER_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211983i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_ONLINE_PACK_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212188i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OPERATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212275i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OPERATION_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212278i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212279i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PACK_NAME_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211962i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PACK_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212208i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PACK_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212220i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PACK_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212000i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PAGEFILE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211252i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_LDM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211891i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212281i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_MSR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211892i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_NON_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211907i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_NOT_CYLINDER_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211970i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212280i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_NOT_OEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211921i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_OF_UNKNOWN_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212231i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211920i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PARTITION_STYLE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211919i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PATH_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212266i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PLEX_IS_HEALTHY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211961i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PLEX_LAST_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211960i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PLEX_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211959i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PLEX_NOT_LOADED_TO_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211893i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PLEX_REGENERATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211957i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PLEX_SIZE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211981i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_CACHE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212257i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_CACHE_OUTOFSYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211502i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_EXITING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212012i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212222i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_INITIALIZATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212260i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211918i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212214i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212254i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_PROVIDER_VOL_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212253i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_RAID5_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210972i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211900i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REBOOT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210996i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REFS_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210746i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REPAIR_VOLUMESTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212192i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212224i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_RETRY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212189i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REVERT_ON_CLOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212200i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REVERT_ON_CLOSE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212190i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_REVERT_ON_CLOSE_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212199i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SECTOR_SIZE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211229i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SECURITY_INCOMPLETELY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211515i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SET_SAN_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211258i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211004i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_DIRTY_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211878i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_EXTEND_UNALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210496i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211887i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_LUN_NOT_UNMASKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210979i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_OVER_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211242i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_SIZE_LESS_THAN_MIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211917i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211916i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_UNKNOWN_FILESYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210966i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SHRINK_USER_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211879i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SOURCE_IS_TARGET_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211992i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SUBSYSTEM_ID_IS_NULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211001i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_SYSTEM_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211247i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_TARGET_PACK_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212003i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_TARGET_PORTAL_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211007i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211514i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212193i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_UNABLE_TO_FIND_BOOT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211261i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_UNABLE_TO_FIND_SYSTEM_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211249i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211955i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_UNRECOVERABLE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212263i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_UNRECOVERABLE_PROVIDER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211915i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VDISK_INVALID_OP_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210982i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VDISK_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210983i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VDISK_PATHNAME_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210969i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_ALREADY_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210956i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_ALREADY_COMPACTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210958i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_ALREADY_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210955i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_ALREADY_MERGING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210957i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_DISK_ALREADY_EXPANDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210959i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_DISK_ALREADY_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210960i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_DISK_IS_COMPACTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210963i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_DISK_IS_EXPANDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210964i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_DISK_IS_MERGING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210962i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_DISK_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210965i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_IS_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210961i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_IS_BEING_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210953i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_IS_BEING_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210952i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VD_NOT_ATTACHED_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210954i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211991i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_EXTEND_FVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211230i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_EXTEND_FVE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211232i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_EXTEND_FVE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211233i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_EXTEND_FVE_RECOVERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211231i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210995i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_HAS_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212194i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_HIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211914i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212238i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212025i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211953i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_MIRRORED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211896i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_NOT_A_MIRROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212219i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_NOT_FOUND_IN_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211908i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_NOT_HEALTHY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212226i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_NOT_MOUNTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212209i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_NOT_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212227i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_NOT_RETAINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211952i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_ON_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212005i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_PERMANENTLY_DISMOUNTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212195i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_REGENERATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211904i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_RETAINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211951i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SHRINK_FVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211243i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SHRINK_FVE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211245i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SHRINK_FVE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211246i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SHRINK_FVE_RECOVERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211244i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SIMPLE_SPANNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211895i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SPANS_DISKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212225i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_SYNCHRONIZING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211905i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_TEMPORARILY_DISMOUNTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212196i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212243i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_E_VOLUME_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212244i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: VDS_NF_FILE_SYSTEM,
    pub volumeId: ::windows::core::GUID,
    pub dwPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_FILE_SYSTEM_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).field("dwPercentCompleted", &self.dwPercentCompleted).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_FILE_SYSTEM_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_FILE_SYSTEM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_FILE_SYSTEM_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::core::default::Default for VDS_FILE_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_FILE_SYSTEM_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_UNKNOWN: VDS_FILE_SYSTEM_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_RAW: VDS_FILE_SYSTEM_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_FAT: VDS_FILE_SYSTEM_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_FAT32: VDS_FILE_SYSTEM_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_NTFS: VDS_FILE_SYSTEM_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_CDFS: VDS_FILE_SYSTEM_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_UDF: VDS_FILE_SYSTEM_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_EXFAT: VDS_FILE_SYSTEM_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_CSVFS: VDS_FILE_SYSTEM_TYPE = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_FST_REFS: VDS_FILE_SYSTEM_TYPE = 9i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_HBAPORT_PROP {
    pub id: ::windows::core::GUID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
impl ::core::marker::Copy for VDS_HBAPORT_PROP {}
impl ::core::clone::Clone for VDS_HBAPORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_HBAPORT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_HBAPORT_PROP").field("id", &self.id).field("wwnNode", &self.wwnNode).field("wwnPort", &self.wwnPort).field("type", &self.r#type).field("status", &self.status).field("ulPortSpeed", &self.ulPortSpeed).field("ulSupportedPortSpeed", &self.ulSupportedPortSpeed).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_HBAPORT_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_HBAPORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_HBAPORT_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_HBAPORT_PROP {}
impl ::core::default::Default for VDS_HBAPORT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_HBAPORT_SPEED_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HSF_UNKNOWN: VDS_HBAPORT_SPEED_FLAG = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HSF_1GBIT: VDS_HBAPORT_SPEED_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HSF_2GBIT: VDS_HBAPORT_SPEED_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HSF_10GBIT: VDS_HBAPORT_SPEED_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HSF_4GBIT: VDS_HBAPORT_SPEED_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HSF_NOT_NEGOTIATED: VDS_HBAPORT_SPEED_FLAG = 32768i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_HBAPORT_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_UNKNOWN: VDS_HBAPORT_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_ONLINE: VDS_HBAPORT_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_OFFLINE: VDS_HBAPORT_STATUS = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_BYPASSED: VDS_HBAPORT_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_DIAGNOSTICS: VDS_HBAPORT_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_LINKDOWN: VDS_HBAPORT_STATUS = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_ERROR: VDS_HBAPORT_STATUS = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPS_LOOPBACK: VDS_HBAPORT_STATUS = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_HBAPORT_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_UNKNOWN: VDS_HBAPORT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_OTHER: VDS_HBAPORT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_NOTPRESENT: VDS_HBAPORT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_NPORT: VDS_HBAPORT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_NLPORT: VDS_HBAPORT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_FLPORT: VDS_HBAPORT_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_FPORT: VDS_HBAPORT_TYPE = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_EPORT: VDS_HBAPORT_TYPE = 9i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_GPORT: VDS_HBAPORT_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_LPORT: VDS_HBAPORT_TYPE = 20i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HPT_PTP: VDS_HBAPORT_TYPE = 21i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_HEALTH = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_UNKNOWN: VDS_HEALTH = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_HEALTHY: VDS_HEALTH = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_REBUILDING: VDS_HEALTH = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_STALE: VDS_HEALTH = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_FAILING: VDS_HEALTH = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_FAILING_REDUNDANCY: VDS_HEALTH = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_FAILED_REDUNDANCY: VDS_HEALTH = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_FAILED_REDUNDANCY_FAILING: VDS_HEALTH = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_FAILED: VDS_HEALTH = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_REPLACED: VDS_HEALTH = 9i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_PENDING_FAILURE: VDS_HEALTH = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_H_DEGRADED: VDS_HEALTH = 11i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_HINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_HINTS")
            .field("ullHintMask", &self.ullHintMask)
            .field("ullExpectedMaximumSize", &self.ullExpectedMaximumSize)
            .field("ulOptimalReadSize", &self.ulOptimalReadSize)
            .field("ulOptimalReadAlignment", &self.ulOptimalReadAlignment)
            .field("ulOptimalWriteSize", &self.ulOptimalWriteSize)
            .field("ulOptimalWriteAlignment", &self.ulOptimalWriteAlignment)
            .field("ulMaximumDriveCount", &self.ulMaximumDriveCount)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("bFastCrashRecoveryRequired", &self.bFastCrashRecoveryRequired)
            .field("bMostlyReads", &self.bMostlyReads)
            .field("bOptimizeForSequentialReads", &self.bOptimizeForSequentialReads)
            .field("bOptimizeForSequentialWrites", &self.bOptimizeForSequentialWrites)
            .field("bRemapEnabled", &self.bRemapEnabled)
            .field("bReadBackVerifyEnabled", &self.bReadBackVerifyEnabled)
            .field("bWriteThroughCachingEnabled", &self.bWriteThroughCachingEnabled)
            .field("bHardwareChecksumEnabled", &self.bHardwareChecksumEnabled)
            .field("bIsYankable", &self.bIsYankable)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_HINTS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_HINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_HINTS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_HINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub bAllocateHotSpare: super::super::Foundation::BOOL,
    pub bUseMirroredCache: super::super::Foundation::BOOL,
    pub bReadCachingEnabled: super::super::Foundation::BOOL,
    pub bWriteCachingEnabled: super::super::Foundation::BOOL,
    pub bMediaScanEnabled: super::super::Foundation::BOOL,
    pub bConsistencyCheckEnabled: super::super::Foundation::BOOL,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub bReserved3: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_HINTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_HINTS2")
            .field("ullHintMask", &self.ullHintMask)
            .field("ullExpectedMaximumSize", &self.ullExpectedMaximumSize)
            .field("ulOptimalReadSize", &self.ulOptimalReadSize)
            .field("ulOptimalReadAlignment", &self.ulOptimalReadAlignment)
            .field("ulOptimalWriteSize", &self.ulOptimalWriteSize)
            .field("ulOptimalWriteAlignment", &self.ulOptimalWriteAlignment)
            .field("ulMaximumDriveCount", &self.ulMaximumDriveCount)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ulReserved3", &self.ulReserved3)
            .field("bFastCrashRecoveryRequired", &self.bFastCrashRecoveryRequired)
            .field("bMostlyReads", &self.bMostlyReads)
            .field("bOptimizeForSequentialReads", &self.bOptimizeForSequentialReads)
            .field("bOptimizeForSequentialWrites", &self.bOptimizeForSequentialWrites)
            .field("bRemapEnabled", &self.bRemapEnabled)
            .field("bReadBackVerifyEnabled", &self.bReadBackVerifyEnabled)
            .field("bWriteThroughCachingEnabled", &self.bWriteThroughCachingEnabled)
            .field("bHardwareChecksumEnabled", &self.bHardwareChecksumEnabled)
            .field("bIsYankable", &self.bIsYankable)
            .field("bAllocateHotSpare", &self.bAllocateHotSpare)
            .field("bUseMirroredCache", &self.bUseMirroredCache)
            .field("bReadCachingEnabled", &self.bReadCachingEnabled)
            .field("bWriteCachingEnabled", &self.bWriteCachingEnabled)
            .field("bMediaScanEnabled", &self.bMediaScanEnabled)
            .field("bConsistencyCheckEnabled", &self.bConsistencyCheckEnabled)
            .field("BusType", &self.BusType)
            .field("bReserved1", &self.bReserved1)
            .field("bReserved2", &self.bReserved2)
            .field("bReserved3", &self.bReserved3)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_HINTS2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_HINTS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_HINTS2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_HINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_ALLOCATEHOTSPARE: i32 = 512i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_BUSTYPE: i32 = 1024i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_CONSISTENCYCHECKENABLED: i32 = 32768i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: i32 = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_HARDWARECHECKSUMENABLED: i32 = 128i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_ISYANKABLE: i32 = 256i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_MEDIASCANENABLED: i32 = 16384i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_MOSTLYREADS: i32 = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: i32 = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: i32 = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_READBACKVERIFYENABLED: i32 = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_READCACHINGENABLED: i32 = 4096i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_REMAPENABLED: i32 = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_USEMIRROREDCACHE: i32 = 2048i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_WRITECACHINGENABLED: i32 = 8192i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: i32 = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_HWPROVIDER_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HWT_UNKNOWN: VDS_HWPROVIDER_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HWT_PCI_RAID: VDS_HWPROVIDER_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HWT_FIBRE_CHANNEL: VDS_HWPROVIDER_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HWT_ISCSI: VDS_HWPROVIDER_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HWT_SAS: VDS_HWPROVIDER_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_HWT_HYBRID: VDS_HWPROVIDER_TYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl ::core::marker::Copy for VDS_INTERCONNECT {}
impl ::core::clone::Clone for VDS_INTERCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_INTERCONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_INTERCONNECT").field("m_addressType", &self.m_addressType).field("m_cbPort", &self.m_cbPort).field("m_pbPort", &self.m_pbPort).field("m_cbAddress", &self.m_cbAddress).field("m_pbAddress", &self.m_pbAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_INTERCONNECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_INTERCONNECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_INTERCONNECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_INTERCONNECT {}
impl ::core::default::Default for VDS_INTERCONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_INTERCONNECT_ADDRESS_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IA_UNKNOWN: VDS_INTERCONNECT_ADDRESS_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IA_FCFS: VDS_INTERCONNECT_ADDRESS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IA_FCPH: VDS_INTERCONNECT_ADDRESS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IA_FCPH3: VDS_INTERCONNECT_ADDRESS_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IA_MAC: VDS_INTERCONNECT_ADDRESS_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IA_SCSI: VDS_INTERCONNECT_ADDRESS_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_INTERCONNECT_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ITF_PCI_RAID: VDS_INTERCONNECT_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ITF_FIBRE_CHANNEL: VDS_INTERCONNECT_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ITF_ISCSI: VDS_INTERCONNECT_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ITF_SAS: VDS_INTERCONNECT_FLAG = 8i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl ::core::marker::Copy for VDS_IPADDRESS {}
impl ::core::clone::Clone for VDS_IPADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_IPADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_IPADDRESS").field("type", &self.r#type).field("ipv4Address", &self.ipv4Address).field("ipv6Address", &self.ipv6Address).field("ulIpv6FlowInfo", &self.ulIpv6FlowInfo).field("ulIpv6ScopeId", &self.ulIpv6ScopeId).field("wszTextAddress", &self.wszTextAddress).field("ulPort", &self.ulPort).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_IPADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_IPADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_IPADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_IPADDRESS {}
impl ::core::default::Default for VDS_IPADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_IPADDRESS_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPT_TEXT: VDS_IPADDRESS_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPT_IPV4: VDS_IPADDRESS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPT_IPV6: VDS_IPADDRESS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPT_EMPTY: VDS_IPADDRESS_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_ISCSI_AUTH_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IAT_NONE: VDS_ISCSI_AUTH_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IAT_CHAP: VDS_ISCSI_AUTH_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IAT_MUTUAL_CHAP: VDS_ISCSI_AUTH_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: ::windows::core::GUID,
    pub pwszName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_INITIATOR_ADAPTER_PROP").field("id", &self.id).field("pwszName", &self.pwszName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_INITIATOR_ADAPTER_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: ::windows::core::GUID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_INITIATOR_PORTAL_PROP").field("id", &self.id).field("address", &self.address).field("ulPortIndex", &self.ulPortIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_INITIATOR_PORTAL_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::core::default::Default for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_ISCSI_IPSEC_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_VALID: VDS_ISCSI_IPSEC_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_IKE: VDS_ISCSI_IPSEC_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_MAIN_MODE: VDS_ISCSI_IPSEC_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_AGGRESSIVE_MODE: VDS_ISCSI_IPSEC_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_PFS_ENABLE: VDS_ISCSI_IPSEC_FLAG = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = 64i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_IPSEC_KEY {}
impl ::core::clone::Clone for VDS_ISCSI_IPSEC_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_IPSEC_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_IPSEC_KEY").field("pKey", &self.pKey).field("ulKeySize", &self.ulKeySize).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ISCSI_IPSEC_KEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_IPSEC_KEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_IPSEC_KEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_IPSEC_KEY {}
impl ::core::default::Default for VDS_ISCSI_IPSEC_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_ISCSI_LOGIN_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ILF_REQUIRE_IPSEC: VDS_ISCSI_LOGIN_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ILF_MULTIPATH_ENABLED: VDS_ISCSI_LOGIN_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_ISCSI_LOGIN_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ILT_MANUAL: VDS_ISCSI_LOGIN_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ILT_PERSISTENT: VDS_ISCSI_LOGIN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_ILT_BOOT: VDS_ISCSI_LOGIN_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: ::windows::core::GUID,
    pub tag: u16,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTALGROUP_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTALGROUP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_PORTALGROUP_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_PORTALGROUP_PROP").field("id", &self.id).field("tag", &self.tag).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ISCSI_PORTALGROUP_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_PORTALGROUP_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_PORTALGROUP_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_PORTALGROUP_PROP {}
impl ::core::default::Default for VDS_ISCSI_PORTALGROUP_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: ::windows::core::GUID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_PORTAL_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_PORTAL_PROP").field("id", &self.id).field("address", &self.address).field("status", &self.status).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ISCSI_PORTAL_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_PORTAL_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_PORTAL_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_PORTAL_PROP {}
impl ::core::default::Default for VDS_ISCSI_PORTAL_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_ISCSI_PORTAL_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPS_UNKNOWN: VDS_ISCSI_PORTAL_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPS_ONLINE: VDS_ISCSI_PORTAL_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPS_NOT_READY: VDS_ISCSI_PORTAL_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPS_OFFLINE: VDS_ISCSI_PORTAL_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_IPS_FAILED: VDS_ISCSI_PORTAL_STATUS = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_SHARED_SECRET {}
impl ::core::clone::Clone for VDS_ISCSI_SHARED_SECRET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_SHARED_SECRET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_SHARED_SECRET").field("pSharedSecret", &self.pSharedSecret).field("ulSharedSecretSize", &self.ulSharedSecretSize).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_ISCSI_SHARED_SECRET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_SHARED_SECRET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_SHARED_SECRET>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_SHARED_SECRET {}
impl ::core::default::Default for VDS_ISCSI_SHARED_SECRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: ::windows::core::GUID,
    pub pwszIscsiName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub bChapEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_ISCSI_TARGET_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_ISCSI_TARGET_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_TARGET_PROP").field("id", &self.id).field("pwszIscsiName", &self.pwszIscsiName).field("pwszFriendlyName", &self.pwszFriendlyName).field("bChapEnabled", &self.bChapEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_ISCSI_TARGET_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_ISCSI_TARGET_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_ISCSI_TARGET_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_ISCSI_TARGET_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LOADBALANCE_POLICY_ENUM = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_UNKNOWN: VDS_LOADBALANCE_POLICY_ENUM = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_FAILOVER: VDS_LOADBALANCE_POLICY_ENUM = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_ROUND_ROBIN: VDS_LOADBALANCE_POLICY_ENUM = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: VDS_LOADBALANCE_POLICY_ENUM = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: VDS_LOADBALANCE_POLICY_ENUM = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_WEIGHTED_PATHS: VDS_LOADBALANCE_POLICY_ENUM = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_LEAST_BLOCKS: VDS_LOADBALANCE_POLICY_ENUM = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBP_VENDOR_SPECIFIC: VDS_LOADBALANCE_POLICY_ENUM = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LUN_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_LBN_REMAP_ENABLED: VDS_LUN_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: VDS_LUN_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: VDS_LUN_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: VDS_LUN_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_READ_CACHE_ENABLED: VDS_LUN_FLAG = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_WRITE_CACHE_ENABLED: VDS_LUN_FLAG = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_MEDIA_SCAN_ENABLED: VDS_LUN_FLAG = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: VDS_LUN_FLAG = 128i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LF_SNAPSHOT: VDS_LUN_FLAG = 256i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: super::super::Foundation::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut u8,
    pub m_szProductId: *mut u8,
    pub m_szProductRevision: *mut u8,
    pub m_szSerialNumber: *mut u8,
    pub m_diskSignature: ::windows::core::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_LUN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_LUN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_INFORMATION")
            .field("m_version", &self.m_version)
            .field("m_DeviceType", &self.m_DeviceType)
            .field("m_DeviceTypeModifier", &self.m_DeviceTypeModifier)
            .field("m_bCommandQueueing", &self.m_bCommandQueueing)
            .field("m_BusType", &self.m_BusType)
            .field("m_szVendorId", &self.m_szVendorId)
            .field("m_szProductId", &self.m_szProductId)
            .field("m_szProductRevision", &self.m_szProductRevision)
            .field("m_szSerialNumber", &self.m_szSerialNumber)
            .field("m_diskSignature", &self.m_diskSignature)
            .field("m_deviceIdDescriptor", &self.m_deviceIdDescriptor)
            .field("m_cInterconnects", &self.m_cInterconnects)
            .field("m_rgInterconnects", &self.m_rgInterconnects)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_LUN_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_LUN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_LUN_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_LUN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: VDS_NF_LUN,
    pub LunId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_LUN_NOTIFICATION {}
impl ::core::clone::Clone for VDS_LUN_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_LUN_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_NOTIFICATION").field("ulEvent", &self.ulEvent).field("LunId", &self.LunId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_LUN_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_LUN_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_LUN_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_LUN_NOTIFICATION {}
impl ::core::default::Default for VDS_LUN_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LUN_PLEX_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPF_LBN_REMAP_ENABLED: VDS_LUN_PLEX_FLAG = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_LUN_PLEX_PROP {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_LUN_PLEX_PROP {}
impl ::core::clone::Clone for VDS_LUN_PLEX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_LUN_PLEX_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_PLEX_PROP").field("id", &self.id).field("ullSize", &self.ullSize).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("ulFlags", &self.ulFlags).field("ulStripeSize", &self.ulStripeSize).field("sRebuildPriority", &self.sRebuildPriority).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_LUN_PLEX_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_LUN_PLEX_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_LUN_PLEX_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_LUN_PLEX_PROP {}
impl ::core::default::Default for VDS_LUN_PLEX_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LUN_PLEX_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPS_UNKNOWN: VDS_LUN_PLEX_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPS_ONLINE: VDS_LUN_PLEX_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPS_NOT_READY: VDS_LUN_PLEX_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPS_OFFLINE: VDS_LUN_PLEX_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPS_FAILED: VDS_LUN_PLEX_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LUN_PLEX_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_UNKNOWN: VDS_LUN_PLEX_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_SIMPLE: VDS_LUN_PLEX_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_SPAN: VDS_LUN_PLEX_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_STRIPE: VDS_LUN_PLEX_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_PARITY: VDS_LUN_PLEX_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID2: VDS_LUN_PLEX_TYPE = 15i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID3: VDS_LUN_PLEX_TYPE = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID4: VDS_LUN_PLEX_TYPE = 17i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID5: VDS_LUN_PLEX_TYPE = 18i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID6: VDS_LUN_PLEX_TYPE = 19i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID03: VDS_LUN_PLEX_TYPE = 21i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID05: VDS_LUN_PLEX_TYPE = 22i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID10: VDS_LUN_PLEX_TYPE = 23i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID15: VDS_LUN_PLEX_TYPE = 24i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID30: VDS_LUN_PLEX_TYPE = 25i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID50: VDS_LUN_PLEX_TYPE = 26i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID53: VDS_LUN_PLEX_TYPE = 28i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LPT_RAID60: VDS_LUN_PLEX_TYPE = 29i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_PROP {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub pwszUnmaskingList: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_LUN_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_LUN_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_LUN_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_PROP").field("id", &self.id).field("ullSize", &self.ullSize).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("pwszUnmaskingList", &self.pwszUnmaskingList).field("ulFlags", &self.ulFlags).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("sRebuildPriority", &self.sRebuildPriority).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_LUN_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_LUN_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_LUN_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_LUN_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_LUN_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LUN_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LS_UNKNOWN: VDS_LUN_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LS_ONLINE: VDS_LUN_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LS_NOT_READY: VDS_LUN_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LS_OFFLINE: VDS_LUN_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LS_FAILED: VDS_LUN_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_LUN_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_UNKNOWN: VDS_LUN_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_DEFAULT: VDS_LUN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_FAULT_TOLERANT: VDS_LUN_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_NON_FAULT_TOLERANT: VDS_LUN_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_SIMPLE: VDS_LUN_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_SPAN: VDS_LUN_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_STRIPE: VDS_LUN_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_MIRROR: VDS_LUN_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_PARITY: VDS_LUN_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID2: VDS_LUN_TYPE = 15i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID3: VDS_LUN_TYPE = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID4: VDS_LUN_TYPE = 17i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID5: VDS_LUN_TYPE = 18i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID6: VDS_LUN_TYPE = 19i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID01: VDS_LUN_TYPE = 20i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID03: VDS_LUN_TYPE = 21i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID05: VDS_LUN_TYPE = 22i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID10: VDS_LUN_TYPE = 23i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID15: VDS_LUN_TYPE = 24i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID30: VDS_LUN_TYPE = 25i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID50: VDS_LUN_TYPE = 26i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID51: VDS_LUN_TYPE = 27i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID53: VDS_LUN_TYPE = 28i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID60: VDS_LUN_TYPE = 29i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LT_RAID61: VDS_LUN_TYPE = 30i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_MAINTENANCE_OPERATION = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const BlinkLight: VDS_MAINTENANCE_OPERATION = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const BeepAlarm: VDS_MAINTENANCE_OPERATION = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const SpinDown: VDS_MAINTENANCE_OPERATION = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const SpinUp: VDS_MAINTENANCE_OPERATION = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const Ping: VDS_MAINTENANCE_OPERATION = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_MOUNT_POINT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_MOUNT_POINT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_MOUNT_POINT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_MOUNT_POINT_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_MOUNT_POINT_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_MOUNT_POINT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_MOUNT_POINT_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_MOUNT_POINT_NOTIFICATION {}
impl ::core::default::Default for VDS_MOUNT_POINT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_CONTROLLER = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_CONTROLLER_ARRIVE: VDS_NF_CONTROLLER = 103u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_CONTROLLER_DEPART: VDS_NF_CONTROLLER = 104u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_CONTROLLER_MODIFY: VDS_NF_CONTROLLER = 350u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_CONTROLLER_REMOVED: VDS_NF_CONTROLLER = 351u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_DISK = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DISK_ARRIVE: VDS_NF_DISK = 8u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DISK_DEPART: VDS_NF_DISK = 9u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DISK_MODIFY: VDS_NF_DISK = 10u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_DRIVE = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DRIVE_ARRIVE: VDS_NF_DRIVE = 105u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DRIVE_DEPART: VDS_NF_DRIVE = 106u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DRIVE_MODIFY: VDS_NF_DRIVE = 107u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DRIVE_REMOVED: VDS_NF_DRIVE = 354u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_FILE_SYSTEM = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_FILE_SYSTEM_MODIFY: VDS_NF_FILE_SYSTEM = 203u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: VDS_NF_FILE_SYSTEM = 204u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_LUN = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_LUN_ARRIVE: VDS_NF_LUN = 108u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_LUN_DEPART: VDS_NF_LUN = 109u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_LUN_MODIFY: VDS_NF_LUN = 110u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_PACK = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PACK_ARRIVE: VDS_NF_PACK = 1u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PACK_DEPART: VDS_NF_PACK = 2u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PACK_MODIFY: VDS_NF_PACK = 3u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PARTITION_DEPART: u32 = 12u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PARTITION_MODIFY: u32 = 13u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NF_PORT = u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORT_ARRIVE: VDS_NF_PORT = 121u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORT_DEPART: VDS_NF_PORT = 122u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORT_MODIFY: VDS_NF_PORT = 352u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORT_REMOVED: VDS_NF_PORT = 353u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORTAL_DEPART: u32 = 124u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_PORTAL_MODIFY: u32 = 125u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_TARGET_ARRIVE: u32 = 126u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_TARGET_DEPART: u32 = 127u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_TARGET_MODIFY: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_VOLUME_DEPART: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_VOLUME_MODIFY: u32 = 6u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl ::core::marker::Copy for VDS_NOTIFICATION {}
impl ::core::clone::Clone for VDS_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VDS_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_NOTIFICATION {}
impl ::core::default::Default for VDS_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl ::core::marker::Copy for VDS_NOTIFICATION_0 {}
impl ::core::clone::Clone for VDS_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VDS_NOTIFICATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_NOTIFICATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_NOTIFICATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_NOTIFICATION_0 {}
impl ::core::default::Default for VDS_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_NOTIFICATION_TARGET_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_UNKNOWN: VDS_NOTIFICATION_TARGET_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_PACK: VDS_NOTIFICATION_TARGET_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_VOLUME: VDS_NOTIFICATION_TARGET_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_DISK: VDS_NOTIFICATION_TARGET_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_PARTITION: VDS_NOTIFICATION_TARGET_TYPE = 60i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_DRIVE_LETTER: VDS_NOTIFICATION_TARGET_TYPE = 61i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_FILE_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = 62i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_MOUNT_POINT: VDS_NOTIFICATION_TARGET_TYPE = 63i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_SUB_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = 30i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_CONTROLLER: VDS_NOTIFICATION_TARGET_TYPE = 31i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_DRIVE: VDS_NOTIFICATION_TARGET_TYPE = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_LUN: VDS_NOTIFICATION_TARGET_TYPE = 33i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_PORT: VDS_NOTIFICATION_TARGET_TYPE = 35i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_PORTAL: VDS_NOTIFICATION_TARGET_TYPE = 36i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_TARGET: VDS_NOTIFICATION_TARGET_TYPE = 37i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_PORTAL_GROUP: VDS_NOTIFICATION_TARGET_TYPE = 38i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_NTT_SERVICE: VDS_NOTIFICATION_TARGET_TYPE = 200i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_OBJECT_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_UNKNOWN: VDS_OBJECT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_PROVIDER: VDS_OBJECT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_PACK: VDS_OBJECT_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_VOLUME: VDS_OBJECT_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_VOLUME_PLEX: VDS_OBJECT_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_DISK: VDS_OBJECT_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_SUB_SYSTEM: VDS_OBJECT_TYPE = 30i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_CONTROLLER: VDS_OBJECT_TYPE = 31i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_DRIVE: VDS_OBJECT_TYPE = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_LUN: VDS_OBJECT_TYPE = 33i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_LUN_PLEX: VDS_OBJECT_TYPE = 34i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_PORT: VDS_OBJECT_TYPE = 35i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_PORTAL: VDS_OBJECT_TYPE = 36i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_TARGET: VDS_OBJECT_TYPE = 37i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_PORTAL_GROUP: VDS_OBJECT_TYPE = 38i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_STORAGE_POOL: VDS_OBJECT_TYPE = 39i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_HBAPORT: VDS_OBJECT_TYPE = 90i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_INIT_ADAPTER: VDS_OBJECT_TYPE = 91i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_INIT_PORTAL: VDS_OBJECT_TYPE = 92i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_ASYNC: VDS_OBJECT_TYPE = 100i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_ENUM: VDS_OBJECT_TYPE = 101i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_VDISK: VDS_OBJECT_TYPE = 200i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_OT_OPEN_VDISK: VDS_OBJECT_TYPE = 201i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: VDS_NF_PACK,
    pub packId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PACK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PACK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PACK_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PACK_NOTIFICATION").field("ulEvent", &self.ulEvent).field("packId", &self.packId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_PACK_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PACK_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PACK_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PACK_NOTIFICATION {}
impl ::core::default::Default for VDS_PACK_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: ::windows::core::GUID,
    pub ullOffset: u64,
}
impl ::core::marker::Copy for VDS_PARTITION_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PARTITION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PARTITION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PARTITION_NOTIFICATION").field("ulEvent", &self.ulEvent).field("diskId", &self.diskId).field("ullOffset", &self.ullOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_PARTITION_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PARTITION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PARTITION_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PARTITION_NOTIFICATION {}
impl ::core::default::Default for VDS_PARTITION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
impl ::core::marker::Copy for VDS_PATH_ID {}
impl ::core::clone::Clone for VDS_PATH_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PATH_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PATH_ID").field("ullSourceId", &self.ullSourceId).field("ullPathId", &self.ullPathId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_PATH_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PATH_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PATH_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PATH_ID {}
impl ::core::default::Default for VDS_PATH_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous1: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl ::core::marker::Copy for VDS_PATH_INFO {}
impl ::core::clone::Clone for VDS_PATH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VDS_PATH_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PATH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PATH_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PATH_INFO {}
impl ::core::default::Default for VDS_PATH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: ::windows::core::GUID,
    pub targetPortalId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_0 {}
impl ::core::clone::Clone for VDS_PATH_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VDS_PATH_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PATH_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PATH_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PATH_INFO_0 {}
impl ::core::default::Default for VDS_PATH_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: ::windows::core::GUID,
    pub initiatorAdapterId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_1 {}
impl ::core::clone::Clone for VDS_PATH_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VDS_PATH_INFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PATH_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PATH_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PATH_INFO_1 {}
impl ::core::default::Default for VDS_PATH_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl ::core::marker::Copy for VDS_PATH_INFO_2 {}
impl ::core::clone::Clone for VDS_PATH_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VDS_PATH_INFO_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PATH_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PATH_INFO_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PATH_INFO_2 {}
impl ::core::default::Default for VDS_PATH_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: super::super::Foundation::BOOL,
    pub ulWeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PATH_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_PATH_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PATH_POLICY").field("pathId", &self.pathId).field("bPrimaryPath", &self.bPrimaryPath).field("ulWeight", &self.ulWeight).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_PATH_POLICY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_PATH_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PATH_POLICY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PATH_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_PATH_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_MPS_UNKNOWN: VDS_PATH_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_MPS_ONLINE: VDS_PATH_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_MPS_FAILED: VDS_PATH_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_MPS_STANDBY: VDS_PATH_STATUS = 7i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: super::super::Foundation::PWSTR,
    pub bSpinDown: super::super::Foundation::BOOL,
    pub bIsThinProvisioned: super::super::Foundation::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: super::super::Foundation::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_POOL_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_POOL_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_POOL_ATTRIBUTES")
            .field("ullAttributeMask", &self.ullAttributeMask)
            .field("raidType", &self.raidType)
            .field("busType", &self.busType)
            .field("pwszIntendedUsage", &self.pwszIntendedUsage)
            .field("bSpinDown", &self.bSpinDown)
            .field("bIsThinProvisioned", &self.bIsThinProvisioned)
            .field("ullProvisionedSpace", &self.ullProvisionedSpace)
            .field("bNoSinglePointOfFailure", &self.bNoSinglePointOfFailure)
            .field("ulDataRedundancyMax", &self.ulDataRedundancyMax)
            .field("ulDataRedundancyMin", &self.ulDataRedundancyMin)
            .field("ulDataRedundancyDefault", &self.ulDataRedundancyDefault)
            .field("ulPackageRedundancyMax", &self.ulPackageRedundancyMax)
            .field("ulPackageRedundancyMin", &self.ulPackageRedundancyMin)
            .field("ulPackageRedundancyDefault", &self.ulPackageRedundancyDefault)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("ulStripeSizeMax", &self.ulStripeSizeMax)
            .field("ulStripeSizeMin", &self.ulStripeSizeMin)
            .field("ulDefaultStripeSize", &self.ulDefaultStripeSize)
            .field("ulNumberOfColumns", &self.ulNumberOfColumns)
            .field("ulNumberOfColumnsMax", &self.ulNumberOfColumnsMax)
            .field("ulNumberOfColumnsMin", &self.ulNumberOfColumnsMin)
            .field("ulDefaultNumberofColumns", &self.ulDefaultNumberofColumns)
            .field("ulDataAvailabilityHint", &self.ulDataAvailabilityHint)
            .field("ulAccessRandomnessHint", &self.ulAccessRandomnessHint)
            .field("ulAccessDirectionHint", &self.ulAccessDirectionHint)
            .field("ulAccessSizeHint", &self.ulAccessSizeHint)
            .field("ulAccessLatencyHint", &self.ulAccessLatencyHint)
            .field("ulAccessBandwidthWeightHint", &self.ulAccessBandwidthWeightHint)
            .field("ulStorageCostHint", &self.ulStorageCostHint)
            .field("ulStorageEfficiencyHint", &self.ulStorageEfficiencyHint)
            .field("ulNumOfCustomAttributes", &self.ulNumOfCustomAttributes)
            .field("pPoolCustomAttributes", &self.pPoolCustomAttributes)
            .field("bReserved1", &self.bReserved1)
            .field("bReserved2", &self.bReserved2)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ullReserved1", &self.ullReserved1)
            .field("ullReserved2", &self.ullReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_POOL_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_POOL_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_POOL_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_POOL_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: i32 = 16777216i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: i32 = 2097152i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: i32 = 8388608i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: i32 = 1048576i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: i32 = 4194304i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: i32 = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_BUSTYPE: i32 = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: i32 = 134217728i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: i32 = 524288i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: i32 = 128i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: i32 = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: i32 = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: i32 = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS: i32 = 32768i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: i32 = 262144i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: i32 = 65536i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: i32 = 131072i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: i32 = 1024i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: i32 = 256i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: i32 = 512i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_RAIDTYPE: i32 = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: i32 = 33554432i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: i32 = 67108864i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: i32 = 2048i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: i32 = 16384i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: i32 = 4096i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: i32 = 8192i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_POOL_ATTRIB_THIN_PROVISION: i32 = 8i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_POOL_CUSTOM_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_POOL_CUSTOM_ATTRIBUTES").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_POOL_CUSTOM_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_POOL_CUSTOM_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_POOL_CUSTOM_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_GROUP_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORTAL_GROUP_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORTAL_GROUP_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portalGroupId", &self.portalGroupId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_PORTAL_GROUP_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PORTAL_GROUP_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PORTAL_GROUP_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::core::default::Default for VDS_PORTAL_GROUP_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORTAL_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORTAL_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portalId", &self.portalId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_PORTAL_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PORTAL_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PORTAL_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PORTAL_NOTIFICATION {}
impl ::core::default::Default for VDS_PORTAL_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: VDS_NF_PORT,
    pub portId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PORT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORT_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portId", &self.portId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_PORT_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_PORT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PORT_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_PORT_NOTIFICATION {}
impl ::core::default::Default for VDS_PORT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PORT_PROP {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub status: VDS_PORT_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PORT_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_PORT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORT_PROP").field("id", &self.id).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("status", &self.status).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_PORT_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_PORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PORT_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_PORT_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PORT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_PORT_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PRS_UNKNOWN: VDS_PORT_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PRS_ONLINE: VDS_PORT_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PRS_NOT_READY: VDS_PORT_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PRS_OFFLINE: VDS_PORT_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PRS_FAILED: VDS_PORT_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PRS_REMOVED: VDS_PORT_STATUS = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_PROVIDER_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_DYNAMIC: VDS_PROVIDER_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: VDS_PROVIDER_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: VDS_PROVIDER_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: VDS_PROVIDER_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: VDS_PROVIDER_FLAG = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_SUPPORT_DYNAMIC: VDS_PROVIDER_FLAG = -2147483648i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: VDS_PROVIDER_FLAG = 1073741824i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_SUPPORT_DYNAMIC_1394: VDS_PROVIDER_FLAG = 536870912i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_SUPPORT_MIRROR: VDS_PROVIDER_FLAG = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PF_SUPPORT_RAID5: VDS_PROVIDER_FLAG = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_PROVIDER_LBSUPPORT_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_FAILOVER: VDS_PROVIDER_LBSUPPORT_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_ROUND_ROBIN: VDS_PROVIDER_LBSUPPORT_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: VDS_PROVIDER_LBSUPPORT_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: VDS_PROVIDER_LBSUPPORT_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_WEIGHTED_PATHS: VDS_PROVIDER_LBSUPPORT_FLAG = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_LEAST_BLOCKS: VDS_PROVIDER_LBSUPPORT_FLAG = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_LBF_VENDOR_SPECIFIC: VDS_PROVIDER_LBSUPPORT_FLAG = 64i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PROVIDER_PROP {
    pub id: ::windows::core::GUID,
    pub pwszName: super::super::Foundation::PWSTR,
    pub guidVersionId: ::windows::core::GUID,
    pub pwszVersion: super::super::Foundation::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PROVIDER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_PROVIDER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PROVIDER_PROP").field("id", &self.id).field("pwszName", &self.pwszName).field("guidVersionId", &self.guidVersionId).field("pwszVersion", &self.pwszVersion).field("type", &self.r#type).field("ulFlags", &self.ulFlags).field("ulStripeSizeFlags", &self.ulStripeSizeFlags).field("sRebuildPriority", &self.sRebuildPriority).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_PROVIDER_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_PROVIDER_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_PROVIDER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_PROVIDER_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PT_UNKNOWN: VDS_PROVIDER_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PT_SOFTWARE: VDS_PROVIDER_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PT_HARDWARE: VDS_PROVIDER_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PT_VIRTUALDISK: VDS_PROVIDER_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_PT_MAX: VDS_PROVIDER_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_RAID_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_UNKNOWN: VDS_RAID_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID0: VDS_RAID_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID1: VDS_RAID_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID2: VDS_RAID_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID3: VDS_RAID_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID4: VDS_RAID_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID5: VDS_RAID_TYPE = 15i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID6: VDS_RAID_TYPE = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID01: VDS_RAID_TYPE = 17i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID03: VDS_RAID_TYPE = 18i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID05: VDS_RAID_TYPE = 19i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID10: VDS_RAID_TYPE = 20i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID15: VDS_RAID_TYPE = 21i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID30: VDS_RAID_TYPE = 22i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID50: VDS_RAID_TYPE = 23i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID51: VDS_RAID_TYPE = 24i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID53: VDS_RAID_TYPE = 25i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID60: VDS_RAID_TYPE = 26i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RT_RAID61: VDS_RAID_TYPE = 27i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_RECOVER_ACTION = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RA_UNKNOWN: VDS_RECOVER_ACTION = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RA_REFRESH: VDS_RECOVER_ACTION = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_RA_RESTART: VDS_RECOVER_ACTION = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
impl ::core::marker::Copy for VDS_SERVICE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SERVICE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SERVICE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SERVICE_NOTIFICATION").field("ulEvent", &self.ulEvent).field("action", &self.action).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_SERVICE_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_SERVICE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_SERVICE_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_SERVICE_NOTIFICATION {}
impl ::core::default::Default for VDS_SERVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_STORAGE_BUS_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeUnknown: VDS_STORAGE_BUS_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeScsi: VDS_STORAGE_BUS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeAtapi: VDS_STORAGE_BUS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeAta: VDS_STORAGE_BUS_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusType1394: VDS_STORAGE_BUS_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeSsa: VDS_STORAGE_BUS_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeFibre: VDS_STORAGE_BUS_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeUsb: VDS_STORAGE_BUS_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeRAID: VDS_STORAGE_BUS_TYPE = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeiScsi: VDS_STORAGE_BUS_TYPE = 9i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeSas: VDS_STORAGE_BUS_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeSata: VDS_STORAGE_BUS_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeSd: VDS_STORAGE_BUS_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeMmc: VDS_STORAGE_BUS_TYPE = 13i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeMax: VDS_STORAGE_BUS_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeVirtual: VDS_STORAGE_BUS_TYPE = 14i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeFileBackedVirtual: VDS_STORAGE_BUS_TYPE = 15i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeSpaces: VDS_STORAGE_BUS_TYPE = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeNVMe: VDS_STORAGE_BUS_TYPE = 17i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeScm: VDS_STORAGE_BUS_TYPE = 18i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeUfs: VDS_STORAGE_BUS_TYPE = 19i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSBusTypeMaxReserved: VDS_STORAGE_BUS_TYPE = 127i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl ::core::marker::Copy for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_DEVICE_ID_DESCRIPTOR").field("m_version", &self.m_version).field("m_cIdentifiers", &self.m_cIdentifiers).field("m_rgIdentifiers", &self.m_rgIdentifiers).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_STORAGE_DEVICE_ID_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::default::Default for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl ::core::marker::Copy for VDS_STORAGE_IDENTIFIER {}
impl ::core::clone::Clone for VDS_STORAGE_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_STORAGE_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_IDENTIFIER").field("m_CodeSet", &self.m_CodeSet).field("m_Type", &self.m_Type).field("m_cbIdentifier", &self.m_cbIdentifier).field("m_rgbIdentifier", &self.m_rgbIdentifier).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_STORAGE_IDENTIFIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_STORAGE_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_STORAGE_IDENTIFIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_STORAGE_IDENTIFIER {}
impl ::core::default::Default for VDS_STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_STORAGE_IDENTIFIER_CODE_SET = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdCodeSetReserved: VDS_STORAGE_IDENTIFIER_CODE_SET = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdCodeSetBinary: VDS_STORAGE_IDENTIFIER_CODE_SET = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdCodeSetAscii: VDS_STORAGE_IDENTIFIER_CODE_SET = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdCodeSetUtf8: VDS_STORAGE_IDENTIFIER_CODE_SET = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_STORAGE_IDENTIFIER_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeVendorSpecific: VDS_STORAGE_IDENTIFIER_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeVendorId: VDS_STORAGE_IDENTIFIER_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeEUI64: VDS_STORAGE_IDENTIFIER_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeFCPHName: VDS_STORAGE_IDENTIFIER_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypePortRelative: VDS_STORAGE_IDENTIFIER_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeTargetPortGroup: VDS_STORAGE_IDENTIFIER_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeLogicalUnitGroup: VDS_STORAGE_IDENTIFIER_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: VDS_STORAGE_IDENTIFIER_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDSStorageIdTypeScsiNameString: VDS_STORAGE_IDENTIFIER_TYPE = 8i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_POOL_DRIVE_EXTENT").field("id", &self.id).field("ullSize", &self.ullSize).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_STORAGE_POOL_DRIVE_EXTENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_STORAGE_POOL_DRIVE_EXTENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: ::windows::core::GUID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_STORAGE_POOL_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_STORAGE_POOL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_STORAGE_POOL_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_POOL_PROP").field("id", &self.id).field("status", &self.status).field("health", &self.health).field("type", &self.r#type).field("pwszName", &self.pwszName).field("pwszDescription", &self.pwszDescription).field("ullTotalConsumedSpace", &self.ullTotalConsumedSpace).field("ullTotalManagedSpace", &self.ullTotalManagedSpace).field("ullRemainingFreeSpace", &self.ullRemainingFreeSpace).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_STORAGE_POOL_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_STORAGE_POOL_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_STORAGE_POOL_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_STORAGE_POOL_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_STORAGE_POOL_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_STORAGE_POOL_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPS_UNKNOWN: VDS_STORAGE_POOL_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPS_ONLINE: VDS_STORAGE_POOL_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPS_NOT_READY: VDS_STORAGE_POOL_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPS_OFFLINE: VDS_STORAGE_POOL_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_STORAGE_POOL_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPT_UNKNOWN: VDS_STORAGE_POOL_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPT_PRIMORDIAL: VDS_STORAGE_POOL_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SPT_CONCRETE: VDS_STORAGE_POOL_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_SUB_SYSTEM_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_LUN_MASKING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_LUN_PLEXING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_LUN_REMAPPING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: VDS_SUB_SYSTEM_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: VDS_SUB_SYSTEM_FLAG = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_RADIUS_CAPABLE: VDS_SUB_SYSTEM_FLAG = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: VDS_SUB_SYSTEM_FLAG = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 128i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = 512i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = 1024i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: VDS_SUB_SYSTEM_FLAG = 2048i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_SPAN_LUNS: VDS_SUB_SYSTEM_FLAG = 4096i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: VDS_SUB_SYSTEM_FLAG = 8192i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: VDS_SUB_SYSTEM_FLAG = 16384i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_PARITY_LUNS: VDS_SUB_SYSTEM_FLAG = 32768i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_AUTH_CHAP: VDS_SUB_SYSTEM_FLAG = 65536i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: VDS_SUB_SYSTEM_FLAG = 131072i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: VDS_SUB_SYSTEM_FLAG = 262144i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_LUN_NUMBER: VDS_SUB_SYSTEM_FLAG = 524288i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: VDS_SUB_SYSTEM_FLAG = 1048576i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_READ_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 2097152i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_WRITE_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 4194304i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_MEDIA_SCAN_CAPABLE: VDS_SUB_SYSTEM_FLAG = 8388608i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: VDS_SUB_SYSTEM_FLAG = 16777216i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SUB_SYSTEM_NOTIFICATION").field("ulEvent", &self.ulEvent).field("subSystemId", &self.subSystemId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_SUB_SYSTEM_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_SUB_SYSTEM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_SUB_SYSTEM_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::core::default::Default for VDS_SUB_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SUB_SYSTEM_PROP")
            .field("id", &self.id)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sNumberOfInternalBuses", &self.sNumberOfInternalBuses)
            .field("sMaxNumberOfSlotsEachBus", &self.sMaxNumberOfSlotsEachBus)
            .field("sMaxNumberOfControllers", &self.sMaxNumberOfControllers)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_SUB_SYSTEM_PROP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_SUB_SYSTEM_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_SUB_SYSTEM_PROP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_SUB_SYSTEM_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_SUB_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_PROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SUB_SYSTEM_PROP2")
            .field("id", &self.id)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("ulSupportedRaidTypeFlags", &self.ulSupportedRaidTypeFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sNumberOfInternalBuses", &self.sNumberOfInternalBuses)
            .field("sMaxNumberOfSlotsEachBus", &self.sMaxNumberOfSlotsEachBus)
            .field("sMaxNumberOfControllers", &self.sMaxNumberOfControllers)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .field("ulNumberOfEnclosures", &self.ulNumberOfEnclosures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VDS_SUB_SYSTEM_PROP2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_SUB_SYSTEM_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_SUB_SYSTEM_PROP2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_SUB_SYSTEM_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_SUB_SYSTEM_PROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_SUB_SYSTEM_STATUS = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SSS_UNKNOWN: VDS_SUB_SYSTEM_STATUS = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SSS_ONLINE: VDS_SUB_SYSTEM_STATUS = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SSS_NOT_READY: VDS_SUB_SYSTEM_STATUS = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SSS_OFFLINE: VDS_SUB_SYSTEM_STATUS = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SSS_FAILED: VDS_SUB_SYSTEM_STATUS = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SSS_PARTIALLY_MANAGED: VDS_SUB_SYSTEM_STATUS = 9i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID2_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID3_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID4_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID5_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID6_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 16i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID01_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 32i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID03_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 64i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID05_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 128i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID10_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 256i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID15_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 512i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID30_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 1024i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID50_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 2048i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID51_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 4096i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID53_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 8192i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID60_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 16384i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_SF_SUPPORTS_RAID61_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 32768i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_ACCESS_PATH_NOT_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(279108i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(272148i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_BOOT_PARTITION_NUMBER_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(271414i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_DEFAULT_PLEX_MEMBER_IDS: ::windows::core::HRESULT = ::windows::core::HRESULT(271640i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_DISK_DISMOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(272393i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_DISK_IS_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(271624i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_DISK_MOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(272392i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_DISK_PARTIALLY_CLEANED: ::windows::core::HRESULT = ::windows::core::HRESULT(271386i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_DISMOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271735i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_EXTEND_FILE_SYSTEM_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271461i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_FS_LOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(271747i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_GPT_BOOT_MIRRORED_TO_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212183i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_IA64_BOOT_MIRRORED_TO_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(271450i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(271437i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_ISCSI_LOGIN_ALREAD_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(272386i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(272385i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(272384i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_MBR_BOOT_MIRRORED_TO_GPT: ::windows::core::HRESULT = ::windows::core::HRESULT(271463i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_NAME_TRUNCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(272128i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_NONCONFORMANT_PARTITION_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(271626i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_NO_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(271639i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_PLEX_NOT_LOADED_TO_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(271755i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_PROPERTIES_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(272149i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_PROVIDER_ERROR_LOADING_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(271393i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_REMOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271736i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_RESYNC_NOTIFICATION_TASK_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271738i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_STATUSES_INCOMPLETELY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(272130i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_SYSTEM_PARTITION: ::windows::core::HRESULT = ::windows::core::HRESULT(271630i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES: ::windows::core::HRESULT = ::windows::core::HRESULT(271451i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_UPDATE_BOOTFILE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271412i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_VOLUME_COMPRESS_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271427i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_VSS_FLUSH_AND_HOLD_WRITES: ::windows::core::HRESULT = ::windows::core::HRESULT(271745i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_VSS_RELEASE_WRITES: ::windows::core::HRESULT = ::windows::core::HRESULT(271746i32);
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_S_WINPE_BOOTENTRY: ::windows::core::HRESULT = ::windows::core::HRESULT(271758i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_TARGET_NOTIFICATION {}
impl ::core::clone::Clone for VDS_TARGET_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_TARGET_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_TARGET_NOTIFICATION").field("ulEvent", &self.ulEvent).field("targetId", &self.targetId).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_TARGET_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_TARGET_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_TARGET_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_TARGET_NOTIFICATION {}
impl ::core::default::Default for VDS_TARGET_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_TRANSITION_STATE = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_TS_UNKNOWN: VDS_TRANSITION_STATE = 0i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_TS_STABLE: VDS_TRANSITION_STATE = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_TS_EXTENDING: VDS_TRANSITION_STATE = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_TS_SHRINKING: VDS_TRANSITION_STATE = 3i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_TS_RECONFIGING: VDS_TRANSITION_STATE = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_TS_RESTRIPING: VDS_TRANSITION_STATE = 5i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub type VDS_VERSION_SUPPORT_FLAG = i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_VSF_1_0: VDS_VERSION_SUPPORT_FLAG = 1i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_VSF_1_1: VDS_VERSION_SUPPORT_FLAG = 2i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_VSF_2_0: VDS_VERSION_SUPPORT_FLAG = 4i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_VSF_2_1: VDS_VERSION_SUPPORT_FLAG = 8i32;
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VDS_VSF_3_0: VDS_VERSION_SUPPORT_FLAG = 16i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows::core::GUID,
    pub plexId: ::windows::core::GUID,
    pub ulPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_VOLUME_NOTIFICATION {}
impl ::core::clone::Clone for VDS_VOLUME_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_VOLUME_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_VOLUME_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).field("plexId", &self.plexId).field("ulPercentCompleted", &self.ulPercentCompleted).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_VOLUME_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_VOLUME_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_VOLUME_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_VOLUME_NOTIFICATION {}
impl ::core::default::Default for VDS_VOLUME_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl ::core::marker::Copy for VDS_WWN {}
impl ::core::clone::Clone for VDS_WWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_WWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_WWN").field("rguchWwn", &self.rguchWwn).finish()
    }
}
unsafe impl ::windows::core::Abi for VDS_WWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VDS_WWN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VDS_WWN>()) == 0 }
    }
}
impl ::core::cmp::Eq for VDS_WWN {}
impl ::core::default::Default for VDS_WWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_VirtualDiskService'*"]
pub const VER_VDS_LUN_INFORMATION: u32 = 1u32;
