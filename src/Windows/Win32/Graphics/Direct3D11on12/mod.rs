#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3D11On12CreateDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
    pdevice: Param0,
    flags: u32,
    pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL,
    featurelevels: u32,
    ppcommandqueues: *const ::core::option::Option<::windows::runtime::IUnknown>,
    numqueues: u32,
    nodemask: u32,
    ppdevice: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>,
    ppimmediatecontext: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>,
    pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3D11On12CreateDevice(pdevice: ::windows::runtime::RawPtr, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const ::windows::runtime::RawPtr, numqueues: u32, nodemask: u32, ppdevice: *mut ::windows::runtime::RawPtr, ppimmediatecontext: *mut ::windows::runtime::RawPtr, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::runtime::HRESULT;
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
unsafe impl ::windows::runtime::Abi for D3D11_RESOURCE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID3D11On12Device(pub ::windows::runtime::IUnknown);
impl ID3D11On12Device {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
}
unsafe impl ::windows::runtime::Interface for ID3D11On12Device {
    type Vtable = ID3D11On12Device_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2237734515, 28841, 18702, [150, 20, 169, 227, 2, 119, 121, 4]);
}
impl ::core::convert::From<ID3D11On12Device> for ::windows::runtime::IUnknown {
    fn from(value: ID3D11On12Device) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID3D11On12Device> for ::windows::runtime::IUnknown {
    fn from(value: &ID3D11On12Device) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ID3D11On12Device {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ID3D11On12Device {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource12: ::windows::runtime::RawPtr, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::runtime::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *const ::windows::runtime::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *const ::windows::runtime::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID3D11On12Device1(pub ::windows::runtime::IUnknown);
impl ID3D11On12Device1 {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
    pub unsafe fn GetD3D12Device<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ID3D11On12Device1 {
    type Vtable = ID3D11On12Device1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3182841332, 59951, 19568, [184, 97, 170, 171, 18, 88, 187, 93]);
}
impl ::core::convert::From<ID3D11On12Device1> for ::windows::runtime::IUnknown {
    fn from(value: ID3D11On12Device1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID3D11On12Device1> for ::windows::runtime::IUnknown {
    fn from(value: &ID3D11On12Device1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ID3D11On12Device1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ID3D11On12Device1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ID3D11On12Device> for ID3D11On12Device1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ID3D11On12Device> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ID3D11On12Device> for &ID3D11On12Device1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ID3D11On12Device> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource12: ::windows::runtime::RawPtr, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::runtime::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *const ::windows::runtime::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *const ::windows::runtime::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ID3D11On12Device2(pub ::windows::runtime::IUnknown);
impl ID3D11On12Device2 {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateWrappedResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, presource12: Param0, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), presource12.into_param().abi(), ::core::mem::transmute(pflags11), ::core::mem::transmute(instate), ::core::mem::transmute(outstate), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`*"]
    pub unsafe fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppresources), ::core::mem::transmute(numresources)))
    }
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`*"]
    pub unsafe fn GetD3D12Device<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3D11Resource>, Param1: ::windows::runtime::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>, T: ::windows::runtime::Interface>(&self, presource11: Param0, pcommandqueue: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), presource11.into_param().abi(), pcommandqueue.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3D11Resource>>(&self, presource11: Param0, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presource11.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ID3D11On12Device2 {
    type Vtable = ID3D11On12Device2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3700486961, 18240, 17402, [134, 110, 103, 241, 44, 181, 130, 35]);
}
impl ::core::convert::From<ID3D11On12Device2> for ::windows::runtime::IUnknown {
    fn from(value: ID3D11On12Device2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ID3D11On12Device2> for ::windows::runtime::IUnknown {
    fn from(value: &ID3D11On12Device2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ID3D11On12Device2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ID3D11On12Device1> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ID3D11On12Device1> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ID3D11On12Device1> for &ID3D11On12Device2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ID3D11On12Device1> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, ID3D11On12Device> for ID3D11On12Device2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ID3D11On12Device> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ID3D11On12Device> for &ID3D11On12Device2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ID3D11On12Device> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11On12Device2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource12: ::windows::runtime::RawPtr, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::runtime::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *const ::windows::runtime::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *const ::windows::runtime::RawPtr, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource11: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presource11: ::windows::runtime::RawPtr, numsync: u32, psignalvalues: *const u64, ppfences: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const ::windows::runtime::RawPtr, numqueues: u32, param6: u32, param7: *mut ::windows::runtime::RawPtr, param8: *mut ::windows::runtime::RawPtr, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::runtime::HRESULT;
