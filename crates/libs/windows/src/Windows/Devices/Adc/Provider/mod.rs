#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Devices_Adc_Provider'*"]
#[repr(transparent)]
pub struct IAdcControllerProvider(::windows::core::IUnknown);
impl IAdcControllerProvider {
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn ChannelCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn ResolutionInBits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn MinValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn MaxValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn ChannelMode(&self) -> ::windows::core::Result<ProviderAdcChannelMode> {
        let this = self;
        unsafe {
            let mut result__: ProviderAdcChannelMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProviderAdcChannelMode>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), channelmode, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn AcquireChannel(&self, channel: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), channel).ok() }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn ReleaseChannel(&self, channel: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), channel).ok() }
    }
    #[doc = "*Required features: 'Devices_Adc_Provider'*"]
    pub fn ReadValue(&self, channelnumber: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), channelnumber, &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::convert::From<IAdcControllerProvider> for ::windows::core::IInspectable {
    fn from(value: IAdcControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdcControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &IAdcControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdcControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAdcControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdcControllerProvider> for ::windows::core::IUnknown {
    fn from(value: IAdcControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdcControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &IAdcControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdcControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAdcControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdcControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdcControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdcControllerProvider {}
impl ::core::fmt::Debug for IAdcControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdcControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdcControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{be545828-816d-4de5-a048-aba06958aaa8}");
}
unsafe impl ::windows::core::Interface for IAdcControllerProvider {
    type Vtable = IAdcControllerProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe545828_816d_4de5_a048_aba06958aaa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderAdcChannelMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Devices_Adc_Provider'*"]
#[repr(transparent)]
pub struct IAdcProvider(::windows::core::IUnknown);
impl IAdcProvider {
    #[doc = "*Required features: 'Devices_Adc_Provider', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>(result__)
        }
    }
}
impl ::core::convert::From<IAdcProvider> for ::windows::core::IInspectable {
    fn from(value: IAdcProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdcProvider> for ::windows::core::IInspectable {
    fn from(value: &IAdcProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdcProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAdcProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdcProvider> for ::windows::core::IUnknown {
    fn from(value: IAdcProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdcProvider> for ::windows::core::IUnknown {
    fn from(value: &IAdcProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdcProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAdcProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdcProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdcProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdcProvider {}
impl ::core::fmt::Debug for IAdcProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdcProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAdcProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{28953668-9359-4c57-bc88-e275e81638c9}");
}
unsafe impl ::windows::core::Interface for IAdcProvider {
    type Vtable = IAdcProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28953668_9359_4c57_bc88_e275e81638c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Devices_Adc_Provider'*"]
#[repr(transparent)]
pub struct ProviderAdcChannelMode(pub i32);
impl ProviderAdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderAdcChannelMode {}
impl ::core::clone::Clone for ProviderAdcChannelMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ProviderAdcChannelMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ProviderAdcChannelMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProviderAdcChannelMode {}
impl ::core::fmt::Debug for ProviderAdcChannelMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderAdcChannelMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProviderAdcChannelMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.Provider.ProviderAdcChannelMode;i4)");
}
impl ::windows::core::DefaultType for ProviderAdcChannelMode {
    type DefaultType = Self;
}
