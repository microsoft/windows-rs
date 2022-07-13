#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Direct3D11\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3D11On12CreateDevice<'a, P0>(pdevice: P0, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], ppcommandqueues: &[::core::option::Option<::windows::core::IUnknown>], nodemask: u32, ppdevice: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>, ppimmediatecontext: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3D11On12CreateDevice(pdevice: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const *mut ::core::ffi::c_void, numqueues: u32, nodemask: u32, ppdevice: *mut *mut ::core::ffi::c_void, ppimmediatecontext: *mut *mut ::core::ffi::c_void, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT;
    }
    D3D11On12CreateDevice(pdevice.into().abi(), flags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfeaturelevels)), pfeaturelevels.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppcommandqueues)), ppcommandqueues.len() as _, nodemask, ::core::mem::transmute(ppdevice), ::core::mem::transmute(ppimmediatecontext), ::core::mem::transmute(pchosenfeaturelevel)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`*"]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl ::core::marker::Copy for D3D11_RESOURCE_FLAGS {}
impl ::core::clone::Clone for D3D11_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D11_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RESOURCE_FLAGS").field("BindFlags", &self.BindFlags).field("MiscFlags", &self.MiscFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("StructureByteStride", &self.StructureByteStride).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D11_RESOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D11_RESOURCE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D11_RESOURCE_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D11_RESOURCE_FLAGS {}
impl ::core::default::Default for D3D11_RESOURCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`*"]
#[repr(transparent)]
pub struct ID3D11On12Device(::windows::core::IUnknown);
impl ID3D11On12Device {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<'a, P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Interface::vtable(self).CreateWrappedResource)(::windows::core::Interface::as_raw(self), presource12.into().abi(), ::core::mem::transmute(pflags11), instate, outstate, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows::core::Interface::vtable(self).ReleaseWrappedResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows::core::Interface::vtable(self).AcquireWrappedResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
}
impl ::core::convert::From<ID3D11On12Device> for ::windows::core::IUnknown {
    fn from(value: ID3D11On12Device) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D11On12Device> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D11On12Device) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device> for ::windows::core::IUnknown {
    fn from(value: &ID3D11On12Device) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D11On12Device {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device {}
impl ::core::fmt::Debug for ID3D11On12Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device {}
unsafe impl ::core::marker::Sync for ID3D11On12Device {}
unsafe impl ::windows::core::Interface for ID3D11On12Device {
    type Vtable = ID3D11On12Device_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85611e73_70a9_490e_9614_a9e302777904);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateWrappedResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource12: *mut ::core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateWrappedResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub ReleaseWrappedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    ReleaseWrappedResources: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub AcquireWrappedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    AcquireWrappedResources: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`*"]
#[repr(transparent)]
pub struct ID3D11On12Device1(::windows::core::IUnknown);
impl ID3D11On12Device1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<'a, P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Interface::vtable(self).base__.CreateWrappedResource)(::windows::core::Interface::as_raw(self), presource12.into().abi(), ::core::mem::transmute(pflags11), instate, outstate, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows::core::Interface::vtable(self).base__.ReleaseWrappedResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows::core::Interface::vtable(self).base__.AcquireWrappedResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    pub unsafe fn GetD3D12Device<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetD3D12Device)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ID3D11On12Device1> for ::windows::core::IUnknown {
    fn from(value: ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D11On12Device1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ::windows::core::IUnknown {
    fn from(value: &ID3D11On12Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D11On12Device1> for ID3D11On12Device {
    fn from(value: ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D11On12Device1> for &'a ID3D11On12Device {
    fn from(value: &'a ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ID3D11On12Device {
    fn from(value: &ID3D11On12Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D11On12Device1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device1 {}
impl ::core::fmt::Debug for ID3D11On12Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device1").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device1 {}
unsafe impl ::core::marker::Sync for ID3D11On12Device1 {}
unsafe impl ::windows::core::Interface for ID3D11On12Device1 {
    type Vtable = ID3D11On12Device1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdb64df4_ea2f_4c70_b861_aaab1258bb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device1_Vtbl {
    pub base__: ID3D11On12Device_Vtbl,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`*"]
#[repr(transparent)]
pub struct ID3D11On12Device2(::windows::core::IUnknown);
impl ID3D11On12Device2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateWrappedResource<'a, P0, T>(&self, presource12: P0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CreateWrappedResource)(::windows::core::Interface::as_raw(self), presource12.into().abi(), ::core::mem::transmute(pflags11), instate, outstate, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows::core::Interface::vtable(self).base__.base__.ReleaseWrappedResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: &[::core::option::Option<super::Direct3D11::ID3D11Resource>]) {
        (::windows::core::Interface::vtable(self).base__.base__.AcquireWrappedResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppresources)), ppresources.len() as _)
    }
    pub unsafe fn GetD3D12Device<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base__.GetD3D12Device)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn UnwrapUnderlyingResource<'a, P0, P1, T>(&self, presource11: P0, pcommandqueue: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D11::ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D12::ID3D12CommandQueue>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).UnwrapUnderlyingResource)(::windows::core::Interface::as_raw(self), presource11.into().abi(), pcommandqueue.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn ReturnUnderlyingResource<'a, P0>(&self, presource11: P0, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Direct3D11::ID3D11Resource>>,
    {
        (::windows::core::Interface::vtable(self).ReturnUnderlyingResource)(::windows::core::Interface::as_raw(self), presource11.into().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ::windows::core::IUnknown {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D11On12Device2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ::windows::core::IUnknown {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ID3D11On12Device {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D11On12Device2> for &'a ID3D11On12Device {
    fn from(value: &'a ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ID3D11On12Device {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ID3D11On12Device1 {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ID3D11On12Device2> for &'a ID3D11On12Device1 {
    fn from(value: &'a ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ID3D11On12Device1 {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ID3D11On12Device2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ID3D11On12Device2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11On12Device2 {}
impl ::core::fmt::Debug for ID3D11On12Device2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11On12Device2").field(&self.0).finish()
    }
}
unsafe impl ::core::marker::Send for ID3D11On12Device2 {}
unsafe impl ::core::marker::Sync for ID3D11On12Device2 {}
unsafe impl ::windows::core::Interface for ID3D11On12Device2 {
    type Vtable = ID3D11On12Device2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc90f331_4740_43fa_866e_67f12cb58223);
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device2_Vtbl {
    pub base__: ID3D11On12Device1_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource11: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource11: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *const u64, ppfences: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    ReturnUnderlyingResource: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Direct3D11\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<::windows::core::IUnknown>, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const ::core::option::Option<::windows::core::IUnknown>, numqueues: u32, param6: u32, param7: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>, param8: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
