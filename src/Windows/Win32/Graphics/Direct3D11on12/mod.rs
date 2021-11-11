#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3D11On12CreateDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(
    pdevice: Param0,
    flags: u32,
    pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL,
    featurelevels: u32,
    ppcommandqueues: *const ::core::option::Option<::windows::core::IUnknown>,
    numqueues: u32,
    nodemask: u32,
    ppdevice: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>,
    ppimmediatecontext: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>,
    pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D11On12CreateDevice(pdevice: ::windows::core::RawPtr, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const ::windows::core::RawPtr, numqueues: u32, nodemask: u32, ppdevice: *mut ::windows::core::RawPtr, ppimmediatecontext: *mut ::windows::core::RawPtr, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT;
        }
        D3D11On12CreateDevice(
            pdevice.into_param().abi(),
            ::core::mem::transmute(flags),
            ::core::mem::transmute(pfeaturelevels),
            ::core::mem::transmute(featurelevels),
            ::core::mem::transmute(ppcommandqueues),
            ::core::mem::transmute(numqueues),
            ::core::mem::transmute(nodemask),
            ::core::mem::transmute(ppdevice),
            ::core::mem::transmute(ppimmediatecontext),
            ::core::mem::transmute(pchosenfeaturelevel),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl D3D11_RESOURCE_FLAGS {}
impl ::core::default::Default for D3D11_RESOURCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for D3D11_RESOURCE_FLAGS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("D3D11_RESOURCE_FLAGS").field("BindFlags", &self.BindFlags).field("MiscFlags", &self.MiscFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("StructureByteStride", &self.StructureByteStride).finish()
    }
}
impl ::core::cmp::PartialEq for D3D11_RESOURCE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.BindFlags == other.BindFlags && self.MiscFlags == other.MiscFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.StructureByteStride == other.StructureByteStride
    }
}
impl ::core::cmp::Eq for D3D11_RESOURCE_FLAGS {}
unsafe impl ::windows::core::Abi for D3D11_RESOURCE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID3D11On12Device(pub ::windows::core::IUnknown);
impl ID3D11On12Device {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
}
unsafe impl ::windows::core::Interface for ID3D11On12Device {
    type Vtable = ID3D11On12Device_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85611e73_70a9_490e_9614_a9e302777904);
}
impl ::core::convert::From<ID3D11On12Device> for ::windows::core::IUnknown {
    fn from(value: ID3D11On12Device) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID3D11On12Device> for ::windows::core::IUnknown {
    fn from(value: &ID3D11On12Device) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D11On12Device {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ID3D11On12Device {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource12: ::windows::core::RawPtr, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresources: *const ::windows::core::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresources: *const ::windows::core::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID3D11On12Device1(pub ::windows::core::IUnknown);
impl ID3D11On12Device1 {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
    pub unsafe fn GetD3D12Device<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ID3D11On12Device1 {
    type Vtable = ID3D11On12Device1_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdb64df4_ea2f_4c70_b861_aaab1258bb5d);
}
impl ::core::convert::From<ID3D11On12Device1> for ::windows::core::IUnknown {
    fn from(value: ID3D11On12Device1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ::windows::core::IUnknown {
    fn from(value: &ID3D11On12Device1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D11On12Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ID3D11On12Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ID3D11On12Device1> for ID3D11On12Device {
    fn from(value: ID3D11On12Device1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ID3D11On12Device {
    fn from(value: &ID3D11On12Device1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D11On12Device> for ID3D11On12Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D11On12Device> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D11On12Device> for &ID3D11On12Device1 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D11On12Device> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device1_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource12: ::windows::core::RawPtr, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresources: *const ::windows::core::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresources: *const ::windows::core::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID3D11On12Device2(pub ::windows::core::IUnknown);
impl ID3D11On12Device2 {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
    pub unsafe fn GetD3D12Device<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows::core::IntoParam<'a, super::Direct3D11::ID3D11Resource>, Param1: ::windows::core::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>, T: ::windows::core::Interface>(&self, presource11: Param0, pcommandqueue: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), presource11.into_param().abi(), pcommandqueue.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows::core::IntoParam<'a, super::Direct3D11::ID3D11Resource>>(&self, presource11: Param0, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presource11.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
unsafe impl ::windows::core::Interface for ID3D11On12Device2 {
    type Vtable = ID3D11On12Device2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc90f331_4740_43fa_866e_67f12cb58223);
}
impl ::core::convert::From<ID3D11On12Device2> for ::windows::core::IUnknown {
    fn from(value: ID3D11On12Device2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ::windows::core::IUnknown {
    fn from(value: &ID3D11On12Device2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ID3D11On12Device2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ID3D11On12Device1 {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ID3D11On12Device1 {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D11On12Device1> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D11On12Device1> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D11On12Device1> for &ID3D11On12Device2 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D11On12Device1> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ID3D11On12Device2> for ID3D11On12Device {
    fn from(value: ID3D11On12Device2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ID3D11On12Device {
    fn from(value: &ID3D11On12Device2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D11On12Device> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D11On12Device> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ID3D11On12Device> for &ID3D11On12Device2 {
    fn into_param(self) -> ::windows::core::Param<'a, ID3D11On12Device> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource12: ::windows::core::RawPtr, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresources: *const ::windows::core::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresources: *const ::windows::core::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource11: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presource11: ::windows::core::RawPtr, numsync: u32, psignalvalues: *const u64, ppfences: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = unsafe extern "system" fn(param0: ::windows::core::RawPtr, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const ::windows::core::RawPtr, numqueues: u32, param6: u32, param7: *mut ::windows::core::RawPtr, param8: *mut ::windows::core::RawPtr, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::core::HRESULT;
