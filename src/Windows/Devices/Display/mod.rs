#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[doc = "*Required features: `Devices_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayMonitor(pub ::windows::runtime::IInspectable);
impl DisplayMonitor {
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn ConnectionKind(&self) -> ::windows::runtime::Result<DisplayMonitorConnectionKind> {
        let this = self;
        unsafe {
            let mut result__: DisplayMonitorConnectionKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayMonitorConnectionKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn PhysicalConnector(&self) -> ::windows::runtime::Result<DisplayMonitorPhysicalConnectorKind> {
        let this = self;
        unsafe {
            let mut result__: DisplayMonitorPhysicalConnectorKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayMonitorPhysicalConnectorKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn DisplayAdapterDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Graphics")]
    #[doc = "*Required features: `Devices_Display`, `Graphics`*"]
    pub fn DisplayAdapterId(&self) -> ::windows::runtime::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn DisplayAdapterTargetId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn UsageKind(&self) -> ::windows::runtime::Result<DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__: DisplayMonitorUsageKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayMonitorUsageKind>(result__)
        }
    }
    #[cfg(feature = "Graphics")]
    #[doc = "*Required features: `Devices_Display`, `Graphics`*"]
    pub fn NativeResolutionInRawPixels(&self) -> ::windows::runtime::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn PhysicalSizeInInches(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::Size>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Size>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn RawDpiX(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn RawDpiY(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn RedPrimary(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn GreenPrimary(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn BluePrimary(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn WhitePoint(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn MaxLuminanceInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn MinLuminanceInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), descriptorkind, ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DisplayMonitor>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Display`, `Foundation`*"]
    pub fn FromInterfaceIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceinterfaceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceinterfaceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DisplayMonitor>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Display`*"]
    pub fn IsDolbyVisionSupportedInHdrMode(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDisplayMonitor2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IDisplayMonitorStatics<R, F: FnOnce(&IDisplayMonitorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DisplayMonitor, IDisplayMonitorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.DisplayMonitor;{1f6b15d4-1d01-4c51-87e2-6f954a772b59})");
}
unsafe impl ::windows::runtime::Interface for DisplayMonitor {
    type Vtable = IDisplayMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
}
impl ::windows::runtime::RuntimeName for DisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.DisplayMonitor";
}
impl ::core::convert::From<DisplayMonitor> for ::windows::runtime::IUnknown {
    fn from(value: DisplayMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DisplayMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayMonitor> for ::windows::runtime::IInspectable {
    fn from(value: DisplayMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayMonitor> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DisplayMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayMonitor {}
unsafe impl ::core::marker::Sync for DisplayMonitor {}
#[doc = "*Required features: `Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(0i32);
    pub const Wired: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(1i32);
    pub const Wireless: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(2i32);
    pub const Virtual: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(3i32);
}
impl ::core::convert::From<i32> for DisplayMonitorConnectionKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayMonitorConnectionKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayMonitorConnectionKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorConnectionKind;i4)");
}
impl ::windows::runtime::DefaultType for DisplayMonitorConnectionKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: DisplayMonitorDescriptorKind = DisplayMonitorDescriptorKind(0i32);
    pub const DisplayId: DisplayMonitorDescriptorKind = DisplayMonitorDescriptorKind(1i32);
}
impl ::core::convert::From<i32> for DisplayMonitorDescriptorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayMonitorDescriptorKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayMonitorDescriptorKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorDescriptorKind;i4)");
}
impl ::windows::runtime::DefaultType for DisplayMonitorDescriptorKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(0i32);
    pub const HD15: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(1i32);
    pub const AnalogTV: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(2i32);
    pub const Dvi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(3i32);
    pub const Hdmi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(4i32);
    pub const Lvds: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(5i32);
    pub const Sdi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(6i32);
    pub const DisplayPort: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(7i32);
}
impl ::core::convert::From<i32> for DisplayMonitorPhysicalConnectorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayMonitorPhysicalConnectorKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayMonitorPhysicalConnectorKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorPhysicalConnectorKind;i4)");
}
impl ::windows::runtime::DefaultType for DisplayMonitorPhysicalConnectorKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: DisplayMonitorUsageKind = DisplayMonitorUsageKind(0i32);
    pub const HeadMounted: DisplayMonitorUsageKind = DisplayMonitorUsageKind(1i32);
    pub const SpecialPurpose: DisplayMonitorUsageKind = DisplayMonitorUsageKind(2i32);
}
impl ::core::convert::From<i32> for DisplayMonitorUsageKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayMonitorUsageKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayMonitorUsageKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorUsageKind;i4)");
}
impl ::windows::runtime::DefaultType for DisplayMonitorUsageKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayMonitor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayMonitor {
    type Vtable = IDisplayMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayMonitorConnectionKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayMonitorPhysicalConnectorKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DisplayMonitorUsageKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptorkind: DisplayMonitorDescriptorKind, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayMonitor2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayMonitor2 {
    type Vtable = IDisplayMonitor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x023018e6_cb23_5830_96df_a7bf6e602577);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayMonitorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayMonitorStatics {
    type Vtable = IDisplayMonitorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6eae698f_a228_4c05_821d_b695d667de8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceinterfaceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
