#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayAdapter(::windows::core::IUnknown);
impl DisplayAdapter {
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn Id(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInterfacePath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SourceCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PciVendorId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PciVendorId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PciDeviceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PciDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PciSubSystemId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PciSubSystemId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PciRevision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PciRevision)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::DisplayAdapterId>>(id: Param0) -> ::windows::core::Result<DisplayAdapter> {
        Self::IDisplayAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromId)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<DisplayAdapter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayAdapterStatics<R, F: FnOnce(&IDisplayAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayAdapter, IDisplayAdapterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayAdapter {}
impl ::core::fmt::Debug for DisplayAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayAdapter;{a56f5287-f000-5f2e-b5ac-3783a2b69af5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayAdapter {
    type Vtable = IDisplayAdapter_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayAdapter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayAdapter {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayAdapter";
}
impl ::core::convert::From<DisplayAdapter> for ::windows::core::IUnknown {
    fn from(value: DisplayAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayAdapter> for ::windows::core::IUnknown {
    fn from(value: &DisplayAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayAdapter> for ::windows::core::IInspectable {
    fn from(value: DisplayAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayAdapter> for ::windows::core::IInspectable {
    fn from(value: &DisplayAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayAdapter {}
unsafe impl ::core::marker::Sync for DisplayAdapter {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: Self = Self(0u32);
    pub const Bpc6: Self = Self(1u32);
    pub const Bpc8: Self = Self(2u32);
    pub const Bpc10: Self = Self(4u32);
    pub const Bpc12: Self = Self(8u32);
    pub const Bpc14: Self = Self(16u32);
    pub const Bpc16: Self = Self(32u32);
}
impl ::core::marker::Copy for DisplayBitsPerChannel {}
impl ::core::clone::Clone for DisplayBitsPerChannel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBitsPerChannel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayBitsPerChannel {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBitsPerChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBitsPerChannel").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayBitsPerChannel {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayBitsPerChannel {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayBitsPerChannel {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayBitsPerChannel {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayBitsPerChannel {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayBitsPerChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayBitsPerChannel;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayDevice(::windows::core::IUnknown);
impl DisplayDevice {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreateScanoutSource<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplaySource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateScanoutSource)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplaySource>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreatePrimary<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, DisplayPrimaryDescription>>(&self, target: Param0, desc: Param1) -> ::windows::core::Result<DisplaySurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePrimary)(::core::mem::transmute_copy(this), target.into_param().abi(), desc.into_param().abi(), &mut result__).from_abi::<DisplaySurface>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreateTaskPool(&self) -> ::windows::core::Result<DisplayTaskPool> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateTaskPool)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTaskPool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicFence<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, target: Param0, offsetfromvblank: Param1) -> ::windows::core::Result<DisplayFence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreatePeriodicFence)(::core::mem::transmute_copy(this), target.into_param().abi(), offsetfromvblank.into_param().abi(), &mut result__).from_abi::<DisplayFence>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn WaitForVBlank<'a, Param0: ::windows::core::IntoParam<'a, DisplaySource>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).WaitForVBlank)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreateSimpleScanout<'a, Param0: ::windows::core::IntoParam<'a, DisplaySource>, Param1: ::windows::core::IntoParam<'a, DisplaySurface>>(&self, psource: Param0, psurface: Param1, subresourceindex: u32, syncinterval: u32) -> ::windows::core::Result<DisplayScanout> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSimpleScanout)(::core::mem::transmute_copy(this), psource.into_param().abi(), psurface.into_param().abi(), subresourceindex, syncinterval, &mut result__).from_abi::<DisplayScanout>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsCapabilitySupported(&self, capability: DisplayDeviceCapability) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCapabilitySupported)(::core::mem::transmute_copy(this), capability, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))]
    pub fn CreateSimpleScanoutWithDirtyRectsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, DisplaySource>, Param1: ::windows::core::IntoParam<'a, DisplaySurface>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32>>>(&self, source: Param0, surface: Param1, subresourceindex: u32, syncinterval: u32, dirtyrects: Param4, options: DisplayScanoutOptions) -> ::windows::core::Result<DisplayScanout> {
        let this = &::windows::core::Interface::cast::<IDisplayDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSimpleScanoutWithDirtyRectsAndOptions)(::core::mem::transmute_copy(this), source.into_param().abi(), surface.into_param().abi(), subresourceindex, syncinterval, dirtyrects.into_param().abi(), options, &mut result__).from_abi::<DisplayScanout>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayDevice {}
impl ::core::fmt::Debug for DisplayDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayDevice;{a4c9b62c-335f-5731-8cb4-c1ccd4731070})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayDevice {
    type Vtable = IDisplayDevice_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayDevice {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayDevice";
}
impl ::core::convert::From<DisplayDevice> for ::windows::core::IUnknown {
    fn from(value: DisplayDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayDevice> for ::windows::core::IUnknown {
    fn from(value: &DisplayDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayDevice> for ::windows::core::IInspectable {
    fn from(value: DisplayDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayDevice> for ::windows::core::IInspectable {
    fn from(value: &DisplayDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayDevice {}
unsafe impl ::core::marker::Sync for DisplayDevice {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayDeviceCapability {}
impl ::core::clone::Clone for DisplayDeviceCapability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayDeviceCapability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayDeviceCapability {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayDeviceCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayDeviceCapability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayDeviceCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayDeviceCapability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayFence(::windows::core::IUnknown);
impl DisplayFence {}
impl ::core::clone::Clone for DisplayFence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayFence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayFence {}
impl ::core::fmt::Debug for DisplayFence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayFence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayFence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayFence;{04dcf9ef-3406-5700-8fec-77eba4c5a74b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayFence {
    type Vtable = IDisplayFence_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayFence as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayFence {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayFence";
}
impl ::core::convert::From<DisplayFence> for ::windows::core::IUnknown {
    fn from(value: DisplayFence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayFence> for ::windows::core::IUnknown {
    fn from(value: &DisplayFence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayFence> for ::windows::core::IInspectable {
    fn from(value: DisplayFence) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayFence> for ::windows::core::IInspectable {
    fn from(value: &DisplayFence) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayFence {}
unsafe impl ::core::marker::Sync for DisplayFence {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManager(::windows::core::IUnknown);
impl DisplayManager {
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentTargets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentTargets)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentAdapters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentAdapters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn TryAcquireTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayManagerResult> {
        let this = self;
        unsafe {
            let mut result__: DisplayManagerResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryAcquireTarget)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayManagerResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ReleaseTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleaseTarget)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn TryReadCurrentStateForAllTargets(&self) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryReadCurrentStateForAllTargets)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndReadCurrentState<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DisplayTarget>>>(&self, targets: Param0) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryAcquireTargetsAndReadCurrentState)(::core::mem::transmute_copy(this), targets.into_param().abi(), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndCreateEmptyState<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DisplayTarget>>>(&self, targets: Param0) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryAcquireTargetsAndCreateEmptyState)(::core::mem::transmute_copy(this), targets.into_param().abi(), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndCreateSubstate<'a, Param0: ::windows::core::IntoParam<'a, DisplayState>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DisplayTarget>>>(&self, existingstate: Param0, targets: Param1) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryAcquireTargetsAndCreateSubstate)(::core::mem::transmute_copy(this), existingstate.into_param().abi(), targets.into_param().abi(), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreateDisplayDevice<'a, Param0: ::windows::core::IntoParam<'a, DisplayAdapter>>(&self, adapter: Param0) -> ::windows::core::Result<DisplayDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDisplayDevice)(::core::mem::transmute_copy(this), adapter.into_param().abi(), &mut result__).from_abi::<DisplayDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Enabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Enabled)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnabled)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Disabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Disabled)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDisabled)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PathsFailedOrInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PathsFailedOrInvalidated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePathsFailedOrInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePathsFailedOrInvalidated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Create(options: DisplayManagerOptions) -> ::windows::core::Result<DisplayManager> {
        Self::IDisplayManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<DisplayManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayManagerStatics<R, F: FnOnce(&IDisplayManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayManager, IDisplayManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayManager {}
impl ::core::fmt::Debug for DisplayManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManager;{4ed9245b-15ec-56e2-9072-7fe5084a31a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayManager {
    type Vtable = IDisplayManager_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayManager {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManager";
}
impl ::core::convert::From<DisplayManager> for ::windows::core::IUnknown {
    fn from(value: DisplayManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManager> for ::windows::core::IUnknown {
    fn from(value: &DisplayManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayManager> for ::windows::core::IInspectable {
    fn from(value: DisplayManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManager> for ::windows::core::IInspectable {
    fn from(value: &DisplayManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DisplayManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DisplayManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DisplayManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DisplayManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DisplayManager {}
unsafe impl ::core::marker::Sync for DisplayManager {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerChangedEventArgs(::windows::core::IUnknown);
impl DisplayManagerChangedEventArgs {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayManagerChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayManagerChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayManagerChangedEventArgs {}
impl ::core::fmt::Debug for DisplayManagerChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerChangedEventArgs;{6abfa285-6cca-5731-bcdc-42e5d2f5c50f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayManagerChangedEventArgs {
    type Vtable = IDisplayManagerChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayManagerChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayManagerChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerChangedEventArgs";
}
impl ::core::convert::From<DisplayManagerChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayManagerChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayManagerChangedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerDisabledEventArgs(::windows::core::IUnknown);
impl DisplayManagerDisabledEventArgs {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayManagerDisabledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayManagerDisabledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayManagerDisabledEventArgs {}
impl ::core::fmt::Debug for DisplayManagerDisabledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerDisabledEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerDisabledEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs;{8726dde4-6793-5973-a11f-5ffbc93fdb90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayManagerDisabledEventArgs {
    type Vtable = IDisplayManagerDisabledEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayManagerDisabledEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayManagerDisabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs";
}
impl ::core::convert::From<DisplayManagerDisabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerDisabledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerDisabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerDisabledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayManagerDisabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerDisabledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerDisabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerDisabledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayManagerDisabledEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerDisabledEventArgs {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerEnabledEventArgs(::windows::core::IUnknown);
impl DisplayManagerEnabledEventArgs {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayManagerEnabledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayManagerEnabledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayManagerEnabledEventArgs {}
impl ::core::fmt::Debug for DisplayManagerEnabledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerEnabledEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerEnabledEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs;{f0cf3f6f-42fa-59a2-b297-26e1713de848})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayManagerEnabledEventArgs {
    type Vtable = IDisplayManagerEnabledEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayManagerEnabledEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayManagerEnabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs";
}
impl ::core::convert::From<DisplayManagerEnabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerEnabledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerEnabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerEnabledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayManagerEnabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerEnabledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerEnabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerEnabledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayManagerEnabledEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerEnabledEventArgs {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayManagerOptions(pub u32);
impl DisplayManagerOptions {
    pub const None: Self = Self(0u32);
    pub const EnforceSourceOwnership: Self = Self(1u32);
    pub const VirtualRefreshRateAware: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayManagerOptions {}
impl ::core::clone::Clone for DisplayManagerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayManagerOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayManagerOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayManagerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayManagerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayManagerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayManagerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayManagerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayManagerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayManagerOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(::windows::core::IUnknown);
impl DisplayManagerPathsFailedOrInvalidatedEventArgs {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
impl ::core::fmt::Debug for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerPathsFailedOrInvalidatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs;{03a65659-1dec-5c15-b2a2-8fe9129869fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    type Vtable = IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayManagerPathsFailedOrInvalidatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs";
}
impl ::core::convert::From<DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const TargetAccessDenied: Self = Self(2i32);
    pub const TargetStale: Self = Self(3i32);
    pub const RemoteSessionNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplayManagerResult {}
impl ::core::clone::Clone for DisplayManagerResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayManagerResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayManagerResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayManagerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayManagerResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerResultWithState(::windows::core::IUnknown);
impl DisplayManagerResultWithState {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<DisplayManagerResult> {
        let this = self;
        unsafe {
            let mut result__: DisplayManagerResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayManagerResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn State(&self) -> ::windows::core::Result<DisplayState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayState>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayManagerResultWithState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayManagerResultWithState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayManagerResultWithState {}
impl ::core::fmt::Debug for DisplayManagerResultWithState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayManagerResultWithState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerResultWithState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerResultWithState;{8e656aa6-6614-54be-bfef-4994547f7be1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayManagerResultWithState {
    type Vtable = IDisplayManagerResultWithState_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayManagerResultWithState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayManagerResultWithState {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerResultWithState";
}
impl ::core::convert::From<DisplayManagerResultWithState> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerResultWithState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerResultWithState> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerResultWithState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayManagerResultWithState> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerResultWithState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayManagerResultWithState> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerResultWithState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayManagerResultWithState {}
unsafe impl ::core::marker::Sync for DisplayManagerResultWithState {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayModeInfo(::windows::core::IUnknown);
impl DisplayModeInfo {
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStereo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourcePixelFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate> {
        let this = self;
        unsafe {
            let mut result__: DisplayPresentationRate = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PresentationRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPresentationRate>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsInterlaced(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInterlaced)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn GetWireFormatSupportedBitsPerChannel(&self, encoding: DisplayWireFormatPixelEncoding) -> ::windows::core::Result<DisplayBitsPerChannel> {
        let this = self;
        unsafe {
            let mut result__: DisplayBitsPerChannel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetWireFormatSupportedBitsPerChannel)(::core::mem::transmute_copy(this), encoding, &mut result__).from_abi::<DisplayBitsPerChannel>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsWireFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, DisplayWireFormat>>(&self, wireformat: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsWireFormatSupported)(::core::mem::transmute_copy(this), wireformat.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PhysicalPresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate> {
        let this = &::windows::core::Interface::cast::<IDisplayModeInfo2>(self)?;
        unsafe {
            let mut result__: DisplayPresentationRate = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalPresentationRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPresentationRate>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayModeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayModeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayModeInfo {}
impl ::core::fmt::Debug for DisplayModeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayModeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayModeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayModeInfo;{48d513a0-f79b-5a74-a05e-da821f470868})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayModeInfo {
    type Vtable = IDisplayModeInfo_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayModeInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayModeInfo {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayModeInfo";
}
impl ::core::convert::From<DisplayModeInfo> for ::windows::core::IUnknown {
    fn from(value: DisplayModeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayModeInfo> for ::windows::core::IUnknown {
    fn from(value: &DisplayModeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayModeInfo> for ::windows::core::IInspectable {
    fn from(value: DisplayModeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayModeInfo> for ::windows::core::IInspectable {
    fn from(value: &DisplayModeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayModeInfo {}
unsafe impl ::core::marker::Sync for DisplayModeInfo {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: Self = Self(0u32);
    pub const OnlyPreferredResolution: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayModeQueryOptions {}
impl ::core::clone::Clone for DisplayModeQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayModeQueryOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayModeQueryOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayModeQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayModeQueryOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayModeQueryOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayModeQueryOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayModeQueryOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayModeQueryOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayModeQueryOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayModeQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayModeQueryOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayPath(::windows::core::IUnknown);
impl DisplayPath {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn View(&self) -> ::windows::core::Result<DisplayView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).View)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayView>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Target(&self) -> ::windows::core::Result<DisplayTarget> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Target)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTarget>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<DisplayPathStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayPathStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPathStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SetSourceResolution<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSourceResolution)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourcePixelFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetSourcePixelFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSourcePixelFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStereo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetIsStereo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsStereo)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SetTargetResolution<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetResolution)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PresentationRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<DisplayPresentationRate>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPresentationRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<DisplayPresentationRate>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPresentationRate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsInterlaced(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInterlaced)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsInterlaced<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInterlaced)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn WireFormat(&self) -> ::windows::core::Result<DisplayWireFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WireFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetWireFormat<'a, Param0: ::windows::core::IntoParam<'a, DisplayWireFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWireFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Rotation(&self) -> ::windows::core::Result<DisplayRotation> {
        let this = self;
        unsafe {
            let mut result__: DisplayRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rotation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayRotation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetRotation(&self, value: DisplayRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Scaling(&self) -> ::windows::core::Result<DisplayPathScaling> {
        let this = self;
        unsafe {
            let mut result__: DisplayPathScaling = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scaling)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPathScaling>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetScaling(&self, value: DisplayPathScaling) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScaling)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindModes(&self, flags: DisplayModeQueryOptions) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindModes)(::core::mem::transmute_copy(this), flags, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ApplyPropertiesFromMode<'a, Param0: ::windows::core::IntoParam<'a, DisplayModeInfo>>(&self, moderesult: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ApplyPropertiesFromMode)(::core::mem::transmute_copy(this), moderesult.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PhysicalPresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>> {
        let this = &::windows::core::Interface::cast::<IDisplayPath2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalPresentationRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<DisplayPresentationRate>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPhysicalPresentationRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<DisplayPresentationRate>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplayPath2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPhysicalPresentationRate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DisplayPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayPath {}
impl ::core::fmt::Debug for DisplayPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPath").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPath {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayPath;{b3dfd64a-7460-5cde-811b-d5ae9f3d9f84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayPath {
    type Vtable = IDisplayPath_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayPath as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayPath {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayPath";
}
impl ::core::convert::From<DisplayPath> for ::windows::core::IUnknown {
    fn from(value: DisplayPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayPath> for ::windows::core::IUnknown {
    fn from(value: &DisplayPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayPath> for ::windows::core::IInspectable {
    fn from(value: DisplayPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayPath> for ::windows::core::IInspectable {
    fn from(value: &DisplayPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayPath {}
unsafe impl ::core::marker::Sync for DisplayPath {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: Self = Self(0i32);
    pub const Centered: Self = Self(1i32);
    pub const Stretched: Self = Self(2i32);
    pub const AspectRatioStretched: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const DriverPreferred: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPathScaling {}
impl ::core::clone::Clone for DisplayPathScaling {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayPathScaling {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayPathScaling {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayPathScaling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPathScaling").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPathScaling {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPathScaling;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const FailedAsync: Self = Self(4i32);
    pub const InvalidatedAsync: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPathStatus {}
impl ::core::clone::Clone for DisplayPathStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayPathStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayPathStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayPathStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPathStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPathStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPathStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: Self = Self(0i32);
    pub const SourceStatusPreventedPresent: Self = Self(1i32);
    pub const ScanoutInvalid: Self = Self(2i32);
    pub const SourceInvalid: Self = Self(3i32);
    pub const DeviceInvalid: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPresentStatus {}
impl ::core::clone::Clone for DisplayPresentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayPresentStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayPresentStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayPresentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPresentStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPresentStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPresentStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct DisplayPresentationRate {
    pub VerticalSyncRate: super::super::super::Foundation::Numerics::Rational,
    pub VerticalSyncsPerPresentation: i32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for DisplayPresentationRate {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for DisplayPresentationRate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayPresentationRate").field("VerticalSyncRate", &self.VerticalSyncRate).field("VerticalSyncsPerPresentation", &self.VerticalSyncsPerPresentation).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for DisplayPresentationRate {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for DisplayPresentationRate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Display.Core.DisplayPresentationRate;struct(Windows.Foundation.Numerics.Rational;u4;u4);i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for DisplayPresentationRate {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DisplayPresentationRate>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for DisplayPresentationRate {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayPrimaryDescription(::windows::core::IUnknown);
impl DisplayPrimaryDescription {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn ColorSpace(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXColorSpace> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXColorSpace = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorSpace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXColorSpace>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStereo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn MultisampleDescription(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MultisampleDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateInstance<'a, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>>(width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: Param5) -> ::windows::core::Result<DisplayPrimaryDescription> {
        Self::IDisplayPrimaryDescriptionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), width, height, pixelformat, colorspace, isstereo, multisampledescription.into_param().abi(), &mut result__).from_abi::<DisplayPrimaryDescription>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, Param6: ::windows::core::IntoParam<'a, super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>>(extraproperties: Param0, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: Param6) -> ::windows::core::Result<DisplayPrimaryDescription> {
        Self::IDisplayPrimaryDescriptionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithProperties)(::core::mem::transmute_copy(this), extraproperties.into_param().abi(), width, height, pixelformat, colorspace, isstereo, multisampledescription.into_param().abi(), &mut result__).from_abi::<DisplayPrimaryDescription>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayPrimaryDescriptionFactory<R, F: FnOnce(&IDisplayPrimaryDescriptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayPrimaryDescription, IDisplayPrimaryDescriptionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IDisplayPrimaryDescriptionStatics<R, F: FnOnce(&IDisplayPrimaryDescriptionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayPrimaryDescription, IDisplayPrimaryDescriptionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayPrimaryDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayPrimaryDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayPrimaryDescription {}
impl ::core::fmt::Debug for DisplayPrimaryDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPrimaryDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPrimaryDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayPrimaryDescription;{872591d2-d533-50ff-a85e-06696194b77c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayPrimaryDescription {
    type Vtable = IDisplayPrimaryDescription_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayPrimaryDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayPrimaryDescription {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayPrimaryDescription";
}
impl ::core::convert::From<DisplayPrimaryDescription> for ::windows::core::IUnknown {
    fn from(value: DisplayPrimaryDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayPrimaryDescription> for ::windows::core::IUnknown {
    fn from(value: &DisplayPrimaryDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayPrimaryDescription> for ::windows::core::IInspectable {
    fn from(value: DisplayPrimaryDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayPrimaryDescription> for ::windows::core::IInspectable {
    fn from(value: &DisplayPrimaryDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayPrimaryDescription {}
unsafe impl ::core::marker::Sync for DisplayPrimaryDescription {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayRotation {}
impl ::core::clone::Clone for DisplayRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayScanout(::windows::core::IUnknown);
impl DisplayScanout {}
impl ::core::clone::Clone for DisplayScanout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayScanout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayScanout {}
impl ::core::fmt::Debug for DisplayScanout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayScanout").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayScanout {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayScanout;{e3051828-1ba5-50e7-8a39-bb1fd2f4f8b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayScanout {
    type Vtable = IDisplayScanout_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayScanout as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayScanout {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayScanout";
}
impl ::core::convert::From<DisplayScanout> for ::windows::core::IUnknown {
    fn from(value: DisplayScanout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayScanout> for ::windows::core::IUnknown {
    fn from(value: &DisplayScanout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayScanout> for ::windows::core::IInspectable {
    fn from(value: DisplayScanout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayScanout> for ::windows::core::IInspectable {
    fn from(value: &DisplayScanout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayScanout {}
unsafe impl ::core::marker::Sync for DisplayScanout {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: Self = Self(0u32);
    pub const AllowTearing: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayScanoutOptions {}
impl ::core::clone::Clone for DisplayScanoutOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayScanoutOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayScanoutOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayScanoutOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayScanoutOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayScanoutOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayScanoutOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayScanoutOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayScanoutOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayScanoutOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayScanoutOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayScanoutOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplaySource(::windows::core::IUnknown);
impl DisplaySource {
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdapterId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SourceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetMetadata<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMetadata)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<DisplaySourceStatus> {
        let this = &::windows::core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe {
            let mut result__: DisplaySourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplaySourceStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DisplaySource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplaySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplaySource {}
impl ::core::fmt::Debug for DisplaySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplaySource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplaySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplaySource;{ecd15fc1-eadc-51bc-971d-3bc628db2dd4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplaySource {
    type Vtable = IDisplaySource_Vtbl;
    const IID: ::windows::core::GUID = <IDisplaySource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplaySource {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplaySource";
}
impl ::core::convert::From<DisplaySource> for ::windows::core::IUnknown {
    fn from(value: DisplaySource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplaySource> for ::windows::core::IUnknown {
    fn from(value: &DisplaySource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplaySource> for ::windows::core::IInspectable {
    fn from(value: DisplaySource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplaySource> for ::windows::core::IInspectable {
    fn from(value: &DisplaySource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplaySource {}
unsafe impl ::core::marker::Sync for DisplaySource {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: Self = Self(0i32);
    pub const PoweredOff: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
    pub const OwnedByAnotherDevice: Self = Self(3i32);
    pub const Unowned: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplaySourceStatus {}
impl ::core::clone::Clone for DisplaySourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplaySourceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplaySourceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplaySourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplaySourceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplaySourceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplaySourceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayState(::windows::core::IUnknown);
impl DisplayState {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsStale(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Targets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Targets)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayView>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Views)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayView>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ConnectTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectTarget)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayPath>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ConnectTargetToView<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, DisplayView>>(&self, target: Param0, view: Param1) -> ::windows::core::Result<DisplayPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectTargetToView)(::core::mem::transmute_copy(this), target.into_param().abi(), view.into_param().abi(), &mut result__).from_abi::<DisplayPath>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CanConnectTargetToView<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, DisplayView>>(&self, target: Param0, view: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanConnectTargetToView)(::core::mem::transmute_copy(this), target.into_param().abi(), view.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn GetViewForTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetViewForTarget)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayView>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn GetPathForTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPathForTarget)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayPath>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn DisconnectTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisconnectTarget)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn TryFunctionalize(&self, options: DisplayStateFunctionalizeOptions) -> ::windows::core::Result<DisplayStateOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryFunctionalize)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<DisplayStateOperationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn TryApply(&self, options: DisplayStateApplyOptions) -> ::windows::core::Result<DisplayStateOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryApply)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<DisplayStateOperationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Clone(&self) -> ::windows::core::Result<DisplayState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Clone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayState>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayState {}
impl ::core::fmt::Debug for DisplayState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayState;{08129321-11b5-5cb2-99f8-e90b479a8a1d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayState {
    type Vtable = IDisplayState_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayState {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayState";
}
impl ::core::convert::From<DisplayState> for ::windows::core::IUnknown {
    fn from(value: DisplayState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayState> for ::windows::core::IUnknown {
    fn from(value: &DisplayState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayState> for ::windows::core::IInspectable {
    fn from(value: DisplayState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayState> for ::windows::core::IInspectable {
    fn from(value: &DisplayState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayState {}
unsafe impl ::core::marker::Sync for DisplayState {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ForceReapply: Self = Self(2u32);
    pub const ForceModeEnumeration: Self = Self(4u32);
}
impl ::core::marker::Copy for DisplayStateApplyOptions {}
impl ::core::clone::Clone for DisplayStateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayStateApplyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayStateApplyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayStateApplyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStateApplyOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayStateApplyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayStateApplyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayStateApplyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayStateApplyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayStateApplyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStateApplyOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateApplyOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ValidateTopologyOnly: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayStateFunctionalizeOptions {}
impl ::core::clone::Clone for DisplayStateFunctionalizeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayStateFunctionalizeOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayStateFunctionalizeOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayStateFunctionalizeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStateFunctionalizeOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayStateFunctionalizeOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayStateFunctionalizeOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStateFunctionalizeOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateFunctionalizeOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayStateOperationResult(::windows::core::IUnknown);
impl DisplayStateOperationResult {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<DisplayStateOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayStateOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayStateOperationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayStateOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayStateOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayStateOperationResult {}
impl ::core::fmt::Debug for DisplayStateOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStateOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStateOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayStateOperationResult;{fcadbfdf-dc27-5638-b7f2-ebdfa4f7ea93})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayStateOperationResult {
    type Vtable = IDisplayStateOperationResult_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayStateOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayStateOperationResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayStateOperationResult";
}
impl ::core::convert::From<DisplayStateOperationResult> for ::windows::core::IUnknown {
    fn from(value: DisplayStateOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayStateOperationResult> for ::windows::core::IUnknown {
    fn from(value: &DisplayStateOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayStateOperationResult> for ::windows::core::IInspectable {
    fn from(value: DisplayStateOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayStateOperationResult> for ::windows::core::IInspectable {
    fn from(value: &DisplayStateOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayStateOperationResult {}
unsafe impl ::core::marker::Sync for DisplayStateOperationResult {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const PartialFailure: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
    pub const TargetOwnershipLost: Self = Self(3i32);
    pub const SystemStateChanged: Self = Self(4i32);
    pub const TooManyPathsForAdapter: Self = Self(5i32);
    pub const ModesNotSupported: Self = Self(6i32);
    pub const RemoteSessionNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for DisplayStateOperationStatus {}
impl ::core::clone::Clone for DisplayStateOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayStateOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayStateOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayStateOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStateOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStateOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplaySurface(::windows::core::IUnknown);
impl DisplaySurface {}
impl ::core::clone::Clone for DisplaySurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplaySurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplaySurface {}
impl ::core::fmt::Debug for DisplaySurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplaySurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplaySurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplaySurface;{594f6cc6-139a-56d6-a4b1-15fe2cb76adb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplaySurface {
    type Vtable = IDisplaySurface_Vtbl;
    const IID: ::windows::core::GUID = <IDisplaySurface as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplaySurface {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplaySurface";
}
impl ::core::convert::From<DisplaySurface> for ::windows::core::IUnknown {
    fn from(value: DisplaySurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplaySurface> for ::windows::core::IUnknown {
    fn from(value: &DisplaySurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplaySurface> for ::windows::core::IInspectable {
    fn from(value: DisplaySurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplaySurface> for ::windows::core::IInspectable {
    fn from(value: &DisplaySurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplaySurface {}
unsafe impl ::core::marker::Sync for DisplaySurface {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayTarget(::windows::core::IUnknown);
impl DisplayTarget {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Adapter(&self) -> ::windows::core::Result<DisplayAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Adapter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayAdapter>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInterfacePath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn AdapterRelativeId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdapterRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsVirtualModeEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVirtualModeEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsVirtualTopologyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVirtualTopologyEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn UsageKind(&self) -> ::windows::core::Result<super::DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayMonitorUsageKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DisplayMonitorUsageKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn MonitorPersistence(&self) -> ::windows::core::Result<DisplayTargetPersistence> {
        let this = self;
        unsafe {
            let mut result__: DisplayTargetPersistence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MonitorPersistence)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTargetPersistence>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn StableMonitorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StableMonitorId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn TryGetMonitor(&self) -> ::windows::core::Result<super::DisplayMonitor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetMonitor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DisplayMonitor>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsStale(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsSame<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, othertarget: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSame)(::core::mem::transmute_copy(this), othertarget.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, othertarget: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), othertarget.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayTarget {}
impl ::core::fmt::Debug for DisplayTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTarget;{aec57c6f-47b4-546b-987c-e73fa791fe3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayTarget {
    type Vtable = IDisplayTarget_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayTarget {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTarget";
}
impl ::core::convert::From<DisplayTarget> for ::windows::core::IUnknown {
    fn from(value: DisplayTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTarget> for ::windows::core::IUnknown {
    fn from(value: &DisplayTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayTarget> for ::windows::core::IInspectable {
    fn from(value: DisplayTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTarget> for ::windows::core::IInspectable {
    fn from(value: &DisplayTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayTarget {}
unsafe impl ::core::marker::Sync for DisplayTarget {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: Self = Self(0i32);
    pub const BootPersisted: Self = Self(1i32);
    pub const TemporaryPersisted: Self = Self(2i32);
    pub const PathPersisted: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayTargetPersistence {}
impl ::core::clone::Clone for DisplayTargetPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayTargetPersistence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayTargetPersistence {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayTargetPersistence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTargetPersistence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTargetPersistence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayTargetPersistence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayTask(::windows::core::IUnknown);
impl DisplayTask {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetScanout<'a, Param0: ::windows::core::IntoParam<'a, DisplayScanout>>(&self, scanout: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScanout)(::core::mem::transmute_copy(this), scanout.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetWait<'a, Param0: ::windows::core::IntoParam<'a, DisplayFence>>(&self, readyfence: Param0, readyfencevalue: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWait)(::core::mem::transmute_copy(this), readyfence.into_param().abi(), readyfencevalue).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetSignal<'a, Param1: ::windows::core::IntoParam<'a, DisplayFence>>(&self, signalkind: DisplayTaskSignalKind, fence: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplayTask2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSignal)(::core::mem::transmute_copy(this), signalkind, fence.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DisplayTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayTask {}
impl ::core::fmt::Debug for DisplayTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTask;{5e087448-135b-5bb0-bf63-637f84227c7a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayTask {
    type Vtable = IDisplayTask_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayTask as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayTask {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTask";
}
impl ::core::convert::From<DisplayTask> for ::windows::core::IUnknown {
    fn from(value: DisplayTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTask> for ::windows::core::IUnknown {
    fn from(value: &DisplayTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayTask> for ::windows::core::IInspectable {
    fn from(value: DisplayTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTask> for ::windows::core::IInspectable {
    fn from(value: &DisplayTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayTask {}
unsafe impl ::core::marker::Sync for DisplayTask {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayTaskPool(::windows::core::IUnknown);
impl DisplayTaskPool {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreateTask(&self) -> ::windows::core::Result<DisplayTask> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateTask)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTask>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ExecuteTask<'a, Param0: ::windows::core::IntoParam<'a, DisplayTask>>(&self, task: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ExecuteTask)(::core::mem::transmute_copy(this), task.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn TryExecuteTask<'a, Param0: ::windows::core::IntoParam<'a, DisplayTask>>(&self, task: Param0) -> ::windows::core::Result<DisplayTaskResult> {
        let this = &::windows::core::Interface::cast::<IDisplayTaskPool2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryExecuteTask)(::core::mem::transmute_copy(this), task.into_param().abi(), &mut result__).from_abi::<DisplayTaskResult>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayTaskPool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayTaskPool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayTaskPool {}
impl ::core::fmt::Debug for DisplayTaskPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTaskPool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTaskPool {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTaskPool;{c676253d-237d-5548-aafa-3e517fefef1c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayTaskPool {
    type Vtable = IDisplayTaskPool_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayTaskPool as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayTaskPool {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTaskPool";
}
impl ::core::convert::From<DisplayTaskPool> for ::windows::core::IUnknown {
    fn from(value: DisplayTaskPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTaskPool> for ::windows::core::IUnknown {
    fn from(value: &DisplayTaskPool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayTaskPool> for ::windows::core::IInspectable {
    fn from(value: DisplayTaskPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTaskPool> for ::windows::core::IInspectable {
    fn from(value: &DisplayTaskPool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayTaskPool {}
unsafe impl ::core::marker::Sync for DisplayTaskPool {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayTaskResult(::windows::core::IUnknown);
impl DisplayTaskResult {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PresentStatus(&self) -> ::windows::core::Result<DisplayPresentStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayPresentStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PresentStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPresentStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PresentId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PresentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SourceStatus(&self) -> ::windows::core::Result<DisplaySourceStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplaySourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplaySourceStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayTaskResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayTaskResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayTaskResult {}
impl ::core::fmt::Debug for DisplayTaskResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTaskResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTaskResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTaskResult;{6fbc7d67-f9b1-55e0-9d88-d3a5197a3f59})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayTaskResult {
    type Vtable = IDisplayTaskResult_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayTaskResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayTaskResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTaskResult";
}
impl ::core::convert::From<DisplayTaskResult> for ::windows::core::IUnknown {
    fn from(value: DisplayTaskResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTaskResult> for ::windows::core::IUnknown {
    fn from(value: &DisplayTaskResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayTaskResult> for ::windows::core::IInspectable {
    fn from(value: DisplayTaskResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayTaskResult> for ::windows::core::IInspectable {
    fn from(value: &DisplayTaskResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayTaskResult {}
unsafe impl ::core::marker::Sync for DisplayTaskResult {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayTaskSignalKind(pub i32);
impl DisplayTaskSignalKind {
    pub const OnPresentFlipAway: Self = Self(0i32);
    pub const OnPresentFlipTo: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayTaskSignalKind {}
impl ::core::clone::Clone for DisplayTaskSignalKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayTaskSignalKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayTaskSignalKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayTaskSignalKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayTaskSignalKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTaskSignalKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayTaskSignalKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayView(::windows::core::IUnknown);
impl DisplayView {
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Paths(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayPath>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Paths)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayPath>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn ContentResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation\"`, `\"Graphics\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SetContentResolution<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentResolution)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn SetPrimaryPath<'a, Param0: ::windows::core::IntoParam<'a, DisplayPath>>(&self, path: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrimaryPath)(::core::mem::transmute_copy(this), path.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayView {}
impl ::core::fmt::Debug for DisplayView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayView;{b0c98ca1-b759-5b59-b1ad-f0786aa9e53d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayView {
    type Vtable = IDisplayView_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayView {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayView";
}
impl ::core::convert::From<DisplayView> for ::windows::core::IUnknown {
    fn from(value: DisplayView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayView> for ::windows::core::IUnknown {
    fn from(value: &DisplayView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayView> for ::windows::core::IInspectable {
    fn from(value: DisplayView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayView> for ::windows::core::IInspectable {
    fn from(value: &DisplayView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayView {}
unsafe impl ::core::marker::Sync for DisplayView {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayWireFormat(::windows::core::IUnknown);
impl DisplayWireFormat {
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn PixelEncoding(&self) -> ::windows::core::Result<DisplayWireFormatPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatPixelEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelEncoding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatPixelEncoding>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn BitsPerChannel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitsPerChannel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn ColorSpace(&self) -> ::windows::core::Result<DisplayWireFormatColorSpace> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatColorSpace = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorSpace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatColorSpace>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn Eotf(&self) -> ::windows::core::Result<DisplayWireFormatEotf> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatEotf = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Eotf)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatEotf>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn HdrMetadata(&self) -> ::windows::core::Result<DisplayWireFormatHdrMetadata> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatHdrMetadata = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HdrMetadata)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatHdrMetadata>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`*"]
    pub fn CreateInstance(pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat> {
        Self::IDisplayWireFormatFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata, &mut result__).from_abi::<DisplayWireFormat>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>>(extraproperties: Param0, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat> {
        Self::IDisplayWireFormatStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithProperties)(::core::mem::transmute_copy(this), extraproperties.into_param().abi(), pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata, &mut result__).from_abi::<DisplayWireFormat>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayWireFormatFactory<R, F: FnOnce(&IDisplayWireFormatFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayWireFormat, IDisplayWireFormatFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IDisplayWireFormatStatics<R, F: FnOnce(&IDisplayWireFormatStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayWireFormat, IDisplayWireFormatStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayWireFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayWireFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayWireFormat {}
impl ::core::fmt::Debug for DisplayWireFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayWireFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayWireFormat;{1acc967d-872c-5a38-bbb9-1d4872b76255})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DisplayWireFormat {
    type Vtable = IDisplayWireFormat_Vtbl;
    const IID: ::windows::core::GUID = <IDisplayWireFormat as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayWireFormat {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayWireFormat";
}
impl ::core::convert::From<DisplayWireFormat> for ::windows::core::IUnknown {
    fn from(value: DisplayWireFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayWireFormat> for ::windows::core::IUnknown {
    fn from(value: &DisplayWireFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayWireFormat> for ::windows::core::IInspectable {
    fn from(value: DisplayWireFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayWireFormat> for ::windows::core::IInspectable {
    fn from(value: &DisplayWireFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayWireFormat {}
unsafe impl ::core::marker::Sync for DisplayWireFormat {}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: Self = Self(0i32);
    pub const BT2020: Self = Self(1i32);
    pub const ProfileDefinedWideColorGamut: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayWireFormatColorSpace {}
impl ::core::clone::Clone for DisplayWireFormatColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayWireFormatColorSpace {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatColorSpace {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayWireFormatColorSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayWireFormatColorSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatColorSpace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatColorSpace;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: Self = Self(0i32);
    pub const HdrSmpte2084: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayWireFormatEotf {}
impl ::core::clone::Clone for DisplayWireFormatEotf {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayWireFormatEotf {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatEotf {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayWireFormatEotf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayWireFormatEotf").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatEotf {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatEotf;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: Self = Self(0i32);
    pub const Hdr10: Self = Self(1i32);
    pub const Hdr10Plus: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayWireFormatHdrMetadata {}
impl ::core::clone::Clone for DisplayWireFormatHdrMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayWireFormatHdrMetadata {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatHdrMetadata {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayWireFormatHdrMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayWireFormatHdrMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatHdrMetadata {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatHdrMetadata;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
    pub const Intensity: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplayWireFormatPixelEncoding {}
impl ::core::clone::Clone for DisplayWireFormatPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayWireFormatPixelEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatPixelEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayWireFormatPixelEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayWireFormatPixelEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatPixelEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatPixelEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAdapter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAdapter {
    type Vtable = IDisplayAdapter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa56f5287_f000_5f2e_b5ac_3783a2b69af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAdapter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Id: usize,
    pub DeviceInterfacePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PciVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PciDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PciSubSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PciRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAdapterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAdapterStatics {
    type Vtable = IDisplayAdapterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dac3cda_481f_5469_8470_82c4ba680a28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAdapterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics")]
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: super::super::super::Graphics::DisplayAdapterId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    FromId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayDevice {
    type Vtable = IDisplayDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c9b62c_335f_5731_8cb4_c1ccd4731070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateScanoutSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreatePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, desc: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTaskPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicFence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, offsetfromvblank: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicFence: usize,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSimpleScanout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psurface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsCapabilitySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capability: DisplayDeviceCapability, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayDevice2 {
    type Vtable = IDisplayDevice2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fefe50c_0940_54bd_a02f_f9c7a536ad60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDevice2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))]
    pub CreateSimpleScanoutWithDirtyRectsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, dirtyrects: ::windows::core::RawPtr, options: DisplayScanoutOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics")))]
    CreateSimpleScanoutWithDirtyRectsAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayFence(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayFence {
    type Vtable = IDisplayFence_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04dcf9ef_3406_5700_8fec_77eba4c5a74b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayFence_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManager {
    type Vtable = IDisplayManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ed9245b_15ec_56e2_9072_7fe5084a31a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentTargets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentAdapters: usize,
    pub TryAcquireTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT,
    pub ReleaseTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryReadCurrentStateForAllTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndReadCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndReadCurrentState: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndCreateEmptyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndCreateEmptyState: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndCreateSubstate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingstate: ::windows::core::RawPtr, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndCreateSubstate: usize,
    pub CreateDisplayDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub Disabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisabled: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PathsFailedOrInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathsFailedOrInvalidated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePathsFailedOrInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePathsFailedOrInvalidated: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManagerChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManagerChangedEventArgs {
    type Vtable = IDisplayManagerChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6abfa285_6cca_5731_bcdc_42e5d2f5c50f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManagerDisabledEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManagerDisabledEventArgs {
    type Vtable = IDisplayManagerDisabledEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8726dde4_6793_5973_a11f_5ffbc93fdb90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerDisabledEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManagerEnabledEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManagerEnabledEventArgs {
    type Vtable = IDisplayManagerEnabledEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0cf3f6f_42fa_59a2_b297_26e1713de848);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerEnabledEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    type Vtable = IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03a65659_1dec_5c15_b2a2_8fe9129869fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManagerResultWithState(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManagerResultWithState {
    type Vtable = IDisplayManagerResultWithState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e656aa6_6614_54be_bfef_4994547f7be1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerResultWithState_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayManagerStatics {
    type Vtable = IDisplayManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b6b9446_b999_5535_9d69_53f092c780a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DisplayManagerOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayModeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayModeInfo {
    type Vtable = IDisplayModeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48d513a0_f79b_5a74_a05e_da821f470868);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayModeInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics")]
    pub SourceResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SourceResolution: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub SourcePixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SourcePixelFormat: usize,
    #[cfg(feature = "Graphics")]
    pub TargetResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    TargetResolution: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PresentationRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PresentationRate: usize,
    pub IsInterlaced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetWireFormatSupportedBitsPerChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoding: DisplayWireFormatPixelEncoding, result__: *mut DisplayBitsPerChannel) -> ::windows::core::HRESULT,
    pub IsWireFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wireformat: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayModeInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayModeInfo2 {
    type Vtable = IDisplayModeInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc86fa386_0ddb_5473_bfb0_4b7807b5f909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayModeInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PhysicalPresentationRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PhysicalPresentationRate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayPath(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayPath {
    type Vtable = IDisplayPath_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3dfd64a_7460_5cde_811b_d5ae9f3d9f84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPath_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub View: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayPathStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SourceResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SourceResolution: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SetSourceResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SetSourceResolution: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SourcePixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SourcePixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetSourcePixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetSourcePixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub TargetResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    TargetResolution: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SetTargetResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SetTargetResolution: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PresentationRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PresentationRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPresentationRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPresentationRate: usize,
    #[cfg(feature = "Foundation")]
    pub IsInterlaced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInterlaced: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsInterlaced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsInterlaced: usize,
    pub WireFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetWireFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayRotation) -> ::windows::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DisplayRotation) -> ::windows::core::HRESULT,
    pub Scaling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayPathScaling) -> ::windows::core::HRESULT,
    pub SetScaling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DisplayPathScaling) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: DisplayModeQueryOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindModes: usize,
    pub ApplyPropertiesFromMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moderesult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayPath2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayPath2 {
    type Vtable = IDisplayPath2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf32459c5_e994_570b_9ec8_ef42c35a8547);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPath2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PhysicalPresentationRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PhysicalPresentationRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPhysicalPresentationRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPhysicalPresentationRate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayPrimaryDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayPrimaryDescription {
    type Vtable = IDisplayPrimaryDescription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x872591d2_d533_50ff_a85e_06696194b77c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescription_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub ColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXColorSpace) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    ColorSpace: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub MultisampleDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    MultisampleDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayPrimaryDescriptionFactory {
    type Vtable = IDisplayPrimaryDescriptionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a6aff7b_3637_5c46_b479_76d576216e65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescriptionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayPrimaryDescriptionStatics {
    type Vtable = IDisplayPrimaryDescriptionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe60e4cfb_36c9_56dd_8fa1_6ff8c4e0ff07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescriptionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extraproperties: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    CreateWithProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayScanout(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayScanout {
    type Vtable = IDisplayScanout_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3051828_1ba5_50e7_8a39_bb1fd2f4f8b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayScanout_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplaySource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplaySource {
    type Vtable = IDisplaySource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecd15fc1_eadc_51bc_971d_3bc628db2dd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplaySource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    pub SourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetMetadata: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplaySource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplaySource2 {
    type Vtable = IDisplaySource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71e18952_b321_5af4_bfe8_03fbea31e40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplaySource2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayState(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayState {
    type Vtable = IDisplayState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08129321_11b5_5cb2_99f8_e90b479a8a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayState_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Targets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Targets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub ConnectTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ConnectTargetToView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CanConnectTargetToView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetViewForTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPathForTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DisconnectTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryFunctionalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DisplayStateFunctionalizeOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TryApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: DisplayStateApplyOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayStateOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayStateOperationResult {
    type Vtable = IDisplayStateOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcadbfdf_dc27_5638_b7f2_ebdfa4f7ea93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayStateOperationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayStateOperationStatus) -> ::windows::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplaySurface(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplaySurface {
    type Vtable = IDisplaySurface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f6cc6_139a_56d6_a4b1_15fe2cb76adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplaySurface_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayTarget {
    type Vtable = IDisplayTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec57c6f_47b4_546b_987c_e73fa791fe3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTarget_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Adapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeviceInterfacePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AdapterRelativeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVirtualModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVirtualTopologyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DisplayMonitorUsageKind) -> ::windows::core::HRESULT,
    pub MonitorPersistence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayTargetPersistence) -> ::windows::core::HRESULT,
    pub StableMonitorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TryGetMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub IsStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayTask(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayTask {
    type Vtable = IDisplayTask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e087448_135b_5bb0_bf63_637f84227c7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTask_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetScanout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanout: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetWait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readyfence: ::windows::core::RawPtr, readyfencevalue: u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayTask2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayTask2 {
    type Vtable = IDisplayTask2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0957ea19_bd55_55de_9267_c97b61e71c37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTask2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetSignal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalkind: DisplayTaskSignalKind, fence: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayTaskPool(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayTaskPool {
    type Vtable = IDisplayTaskPool_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc676253d_237d_5548_aafa_3e517fefef1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTaskPool_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ExecuteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExecuteTask: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayTaskPool2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayTaskPool2 {
    type Vtable = IDisplayTaskPool2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46b879b6_5d17_5955_a872_eb38003db586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTaskPool2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryExecuteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, task: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayTaskResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayTaskResult {
    type Vtable = IDisplayTaskResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fbc7d67_f9b1_55e0_9d88_d3a5197a3f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTaskResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PresentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayPresentStatus) -> ::windows::core::HRESULT,
    pub PresentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SourceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayView {
    type Vtable = IDisplayView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0c98ca1_b759_5b59_b1ad_f0786aa9e53d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Paths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Paths: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub ContentResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    ContentResolution: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SetContentResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SetContentResolution: usize,
    pub SetPrimaryPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayWireFormat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayWireFormat {
    type Vtable = IDisplayWireFormat_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1acc967d_872c_5a38_bbb9_1d4872b76255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayWireFormat_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PixelEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatPixelEncoding) -> ::windows::core::HRESULT,
    pub BitsPerChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatColorSpace) -> ::windows::core::HRESULT,
    pub Eotf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatEotf) -> ::windows::core::HRESULT,
    pub HdrMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayWireFormatHdrMetadata) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayWireFormatFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayWireFormatFactory {
    type Vtable = IDisplayWireFormatFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2efc8d5_09d6_55e6_ad22_9014b3d25229);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayWireFormatFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayWireFormatStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayWireFormatStatics {
    type Vtable = IDisplayWireFormatStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc575a22d_c3e6_5f7a_bdfb_87c6ab8661d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayWireFormatStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extraproperties: ::windows::core::RawPtr, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
