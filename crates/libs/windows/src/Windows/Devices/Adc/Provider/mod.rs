#[doc = "*Required features: `\"Devices_Adc_Provider\"`*"]
#[repr(transparent)]
pub struct IAdcControllerProvider(::windows_core::IUnknown);
impl IAdcControllerProvider {
    pub fn ChannelCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolutionInBits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolutionInBits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChannelMode(&self) -> ::windows_core::Result<ProviderAdcChannelMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChannelMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelModeSupported)(::windows_core::Interface::as_raw(this), channelmode, &mut result__).from_abi(result__)
        }
    }
    pub fn AcquireChannel(&self, channel: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcquireChannel)(::windows_core::Interface::as_raw(this), channel).ok() }
    }
    pub fn ReleaseChannel(&self, channel: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReleaseChannel)(::windows_core::Interface::as_raw(this), channel).ok() }
    }
    pub fn ReadValue(&self, channelnumber: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadValue)(::windows_core::Interface::as_raw(this), channelnumber, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAdcControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IAdcControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{be545828-816d-4de5-a048-aba06958aaa8}");
}
unsafe impl ::windows_core::Interface for IAdcControllerProvider {
    type Vtable = IAdcControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IAdcControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdcControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe545828_816d_4de5_a048_aba06958aaa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ResolutionInBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ChannelMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> ::windows_core::HRESULT,
    pub SetChannelMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderAdcChannelMode) -> ::windows_core::HRESULT,
    pub IsChannelModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AcquireChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows_core::HRESULT,
    pub ReleaseChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows_core::HRESULT,
    pub ReadValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Adc_Provider\"`*"]
#[repr(transparent)]
pub struct IAdcProvider(::windows_core::IUnknown);
impl IAdcProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetControllers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAdcProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IAdcProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{28953668-9359-4c57-bc88-e275e81638c9}");
}
unsafe impl ::windows_core::Interface for IAdcProvider {
    type Vtable = IAdcProvider_Vtbl;
}
impl ::core::clone::Clone for IAdcProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdcProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28953668_9359_4c57_bc88_e275e81638c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[doc = "*Required features: `\"Devices_Adc_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ProviderAdcChannelMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProviderAdcChannelMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProviderAdcChannelMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderAdcChannelMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProviderAdcChannelMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.Provider.ProviderAdcChannelMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
