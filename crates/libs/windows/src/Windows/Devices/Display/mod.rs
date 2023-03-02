#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayMonitor {
    type Vtable = IDisplayMonitor_Vtbl;
}
impl ::core::clone::Clone for IDisplayMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ConnectionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorConnectionKind) -> ::windows::core::HRESULT,
    pub PhysicalConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorPhysicalConnectorKind) -> ::windows::core::HRESULT,
    pub DisplayAdapterDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub DisplayAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    DisplayAdapterId: usize,
    pub DisplayAdapterTargetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorUsageKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub NativeResolutionInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    NativeResolutionInRawPixels: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalSizeInInches: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalSizeInInches: usize,
    pub RawDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RedPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedPrimary: usize,
    #[cfg(feature = "Foundation")]
    pub GreenPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GreenPrimary: usize,
    #[cfg(feature = "Foundation")]
    pub BluePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BluePrimary: usize,
    #[cfg(feature = "Foundation")]
    pub WhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhitePoint: usize,
    pub MaxLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub GetDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptorkind: DisplayMonitorDescriptorKind, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayMonitor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayMonitor2 {
    type Vtable = IDisplayMonitor2_Vtbl;
}
impl ::core::clone::Clone for IDisplayMonitor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayMonitor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023018e6_cb23_5830_96df_a7bf6e602577);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsDolbyVisionSupportedInHdrMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayMonitorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayMonitorStatics {
    type Vtable = IDisplayMonitorStatics_Vtbl;
}
impl ::core::clone::Clone for IDisplayMonitorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayMonitorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eae698f_a228_4c05_821d_b695d667de8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromInterfaceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinterfaceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromInterfaceIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
pub struct DisplayMonitor(::windows::core::IUnknown);
impl DisplayMonitor {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConnectionKind(&self) -> ::windows::core::Result<DisplayMonitorConnectionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayMonitorConnectionKind>();
            (::windows::core::Interface::vtable(this).ConnectionKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhysicalConnector(&self) -> ::windows::core::Result<DisplayMonitorPhysicalConnectorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayMonitorPhysicalConnectorKind>();
            (::windows::core::Interface::vtable(this).PhysicalConnector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayAdapterDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayAdapterDeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn DisplayAdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::DisplayAdapterId>();
            (::windows::core::Interface::vtable(this).DisplayAdapterId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayAdapterTargetId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).DisplayAdapterTargetId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsageKind(&self) -> ::windows::core::Result<DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayMonitorUsageKind>();
            (::windows::core::Interface::vtable(this).UsageKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn NativeResolutionInRawPixels(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::SizeInt32>();
            (::windows::core::Interface::vtable(this).NativeResolutionInRawPixels)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Size>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::Size>>();
            (::windows::core::Interface::vtable(this).PhysicalSizeInInches)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawDpiX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).RawDpiX)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawDpiY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).RawDpiY)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).RedPrimary)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).GreenPrimary)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).BluePrimary)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).WhitePoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).MaxLuminanceInNits)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).MinLuminanceInNits)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f32>();
            (::windows::core::Interface::vtable(this).MaxAverageFullFrameLuminanceInNits)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDescriptor)(::windows::core::Interface::as_raw(this), descriptorkind, ::windows::core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn IsDolbyVisionSupportedInHdrMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IDisplayMonitor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDolbyVisionSupportedInHdrMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DisplayMonitor>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromInterfaceIdAsync(deviceinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DisplayMonitor>>();
            (::windows::core::Interface::vtable(this).FromInterfaceIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceinterfaceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayMonitorStatics<R, F: FnOnce(&IDisplayMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DisplayMonitor, IDisplayMonitorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DisplayMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayMonitor {}
impl ::core::fmt::Debug for DisplayMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayMonitor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.DisplayMonitor;{1f6b15d4-1d01-4c51-87e2-6f954a772b59})");
}
impl ::core::clone::Clone for DisplayMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DisplayMonitor {
    type Vtable = IDisplayMonitor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DisplayMonitor {
    const IID: ::windows::core::GUID = <IDisplayMonitor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.DisplayMonitor";
}
::windows::imp::interface_hierarchy!(DisplayMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DisplayMonitor {}
unsafe impl ::core::marker::Sync for DisplayMonitor {}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: Self = Self(0i32);
    pub const Wired: Self = Self(1i32);
    pub const Wireless: Self = Self(2i32);
    pub const Virtual: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayMonitorConnectionKind {}
impl ::core::clone::Clone for DisplayMonitorConnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorConnectionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DisplayMonitorConnectionKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DisplayMonitorConnectionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorConnectionKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayMonitorConnectionKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorConnectionKind;i4)");
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: Self = Self(0i32);
    pub const DisplayId: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayMonitorDescriptorKind {}
impl ::core::clone::Clone for DisplayMonitorDescriptorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorDescriptorKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DisplayMonitorDescriptorKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DisplayMonitorDescriptorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorDescriptorKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayMonitorDescriptorKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorDescriptorKind;i4)");
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: Self = Self(0i32);
    pub const HD15: Self = Self(1i32);
    pub const AnalogTV: Self = Self(2i32);
    pub const Dvi: Self = Self(3i32);
    pub const Hdmi: Self = Self(4i32);
    pub const Lvds: Self = Self(5i32);
    pub const Sdi: Self = Self(6i32);
    pub const DisplayPort: Self = Self(7i32);
}
impl ::core::marker::Copy for DisplayMonitorPhysicalConnectorKind {}
impl ::core::clone::Clone for DisplayMonitorPhysicalConnectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorPhysicalConnectorKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DisplayMonitorPhysicalConnectorKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DisplayMonitorPhysicalConnectorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorPhysicalConnectorKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayMonitorPhysicalConnectorKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorPhysicalConnectorKind;i4)");
}
#[doc = "*Required features: `\"Devices_Display\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: Self = Self(0i32);
    pub const HeadMounted: Self = Self(1i32);
    pub const SpecialPurpose: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayMonitorUsageKind {}
impl ::core::clone::Clone for DisplayMonitorUsageKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorUsageKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DisplayMonitorUsageKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DisplayMonitorUsageKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorUsageKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayMonitorUsageKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorUsageKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
