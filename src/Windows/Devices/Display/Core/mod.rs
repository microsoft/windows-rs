#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayAdapter(pub ::windows::core::IInspectable);
impl DisplayAdapter {
    #[cfg(feature = "Graphics")]
    pub fn Id(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    pub fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SourceCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PciVendorId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PciDeviceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PciSubSystemId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PciRevision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::DisplayAdapterId>>(id: Param0) -> ::windows::core::Result<DisplayAdapter> {
        Self::IDisplayAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<DisplayAdapter>(result__)
        })
    }
    pub fn IDisplayAdapterStatics<R, F: FnOnce(&IDisplayAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayAdapter, IDisplayAdapterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayAdapter;{a56f5287-f000-5f2e-b5ac-3783a2b69af5})");
}
unsafe impl ::windows::core::Interface for DisplayAdapter {
    type Vtable = IDisplayAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa56f5287_f000_5f2e_b5ac_3783a2b69af5);
}
impl ::windows::core::RuntimeName for DisplayAdapter {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayAdapter";
}
impl ::core::convert::From<DisplayAdapter> for ::windows::core::IUnknown {
    fn from(value: DisplayAdapter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayAdapter> for ::windows::core::IUnknown {
    fn from(value: &DisplayAdapter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayAdapter> for ::windows::core::IInspectable {
    fn from(value: DisplayAdapter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayAdapter> for ::windows::core::IInspectable {
    fn from(value: &DisplayAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayAdapter {}
unsafe impl ::core::marker::Sync for DisplayAdapter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: DisplayBitsPerChannel = DisplayBitsPerChannel(0u32);
    pub const Bpc6: DisplayBitsPerChannel = DisplayBitsPerChannel(1u32);
    pub const Bpc8: DisplayBitsPerChannel = DisplayBitsPerChannel(2u32);
    pub const Bpc10: DisplayBitsPerChannel = DisplayBitsPerChannel(4u32);
    pub const Bpc12: DisplayBitsPerChannel = DisplayBitsPerChannel(8u32);
    pub const Bpc14: DisplayBitsPerChannel = DisplayBitsPerChannel(16u32);
    pub const Bpc16: DisplayBitsPerChannel = DisplayBitsPerChannel(32u32);
}
impl ::core::convert::From<u32> for DisplayBitsPerChannel {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayBitsPerChannel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayBitsPerChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayBitsPerChannel;u4)");
}
impl ::windows::core::DefaultType for DisplayBitsPerChannel {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayBitsPerChannel {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayBitsPerChannel {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayBitsPerChannel {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayBitsPerChannel {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayBitsPerChannel {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayDevice(pub ::windows::core::IInspectable);
impl DisplayDevice {
    pub fn CreateScanoutSource<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplaySource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplaySource>(result__)
        }
    }
    pub fn CreatePrimary<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, DisplayPrimaryDescription>>(&self, target: Param0, desc: Param1) -> ::windows::core::Result<DisplaySurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), target.into_param().abi(), desc.into_param().abi(), &mut result__).from_abi::<DisplaySurface>(result__)
        }
    }
    pub fn CreateTaskPool(&self) -> ::windows::core::Result<DisplayTaskPool> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTaskPool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicFence<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, target: Param0, offsetfromvblank: Param1) -> ::windows::core::Result<DisplayFence> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi(), offsetfromvblank.into_param().abi(), &mut result__).from_abi::<DisplayFence>(result__)
        }
    }
    pub fn WaitForVBlank<'a, Param0: ::windows::core::IntoParam<'a, DisplaySource>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    pub fn CreateSimpleScanout<'a, Param0: ::windows::core::IntoParam<'a, DisplaySource>, Param1: ::windows::core::IntoParam<'a, DisplaySurface>>(&self, psource: Param0, psurface: Param1, subresourceindex: u32, syncinterval: u32) -> ::windows::core::Result<DisplayScanout> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), psource.into_param().abi(), psurface.into_param().abi(), subresourceindex, syncinterval, &mut result__).from_abi::<DisplayScanout>(result__)
        }
    }
    pub fn IsCapabilitySupported(&self, capability: DisplayDeviceCapability) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), capability, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))]
    pub fn CreateSimpleScanoutWithDirtyRectsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, DisplaySource>, Param1: ::windows::core::IntoParam<'a, DisplaySurface>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32>>>(&self, source: Param0, surface: Param1, subresourceindex: u32, syncinterval: u32, dirtyrects: Param4, options: DisplayScanoutOptions) -> ::windows::core::Result<DisplayScanout> {
        let this = &::windows::core::Interface::cast::<IDisplayDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), surface.into_param().abi(), subresourceindex, syncinterval, dirtyrects.into_param().abi(), options, &mut result__).from_abi::<DisplayScanout>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayDevice;{a4c9b62c-335f-5731-8cb4-c1ccd4731070})");
}
unsafe impl ::windows::core::Interface for DisplayDevice {
    type Vtable = IDisplayDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c9b62c_335f_5731_8cb4_c1ccd4731070);
}
impl ::windows::core::RuntimeName for DisplayDevice {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayDevice";
}
impl ::core::convert::From<DisplayDevice> for ::windows::core::IUnknown {
    fn from(value: DisplayDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayDevice> for ::windows::core::IUnknown {
    fn from(value: &DisplayDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayDevice> for ::windows::core::IInspectable {
    fn from(value: DisplayDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayDevice> for ::windows::core::IInspectable {
    fn from(value: &DisplayDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayDevice {}
unsafe impl ::core::marker::Sync for DisplayDevice {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: DisplayDeviceCapability = DisplayDeviceCapability(0i32);
}
impl ::core::convert::From<i32> for DisplayDeviceCapability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayDeviceCapability {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayDeviceCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayDeviceCapability;i4)");
}
impl ::windows::core::DefaultType for DisplayDeviceCapability {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayFence(pub ::windows::core::IInspectable);
impl DisplayFence {}
unsafe impl ::windows::core::RuntimeType for DisplayFence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayFence;{04dcf9ef-3406-5700-8fec-77eba4c5a74b})");
}
unsafe impl ::windows::core::Interface for DisplayFence {
    type Vtable = IDisplayFence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04dcf9ef_3406_5700_8fec_77eba4c5a74b);
}
impl ::windows::core::RuntimeName for DisplayFence {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayFence";
}
impl ::core::convert::From<DisplayFence> for ::windows::core::IUnknown {
    fn from(value: DisplayFence) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayFence> for ::windows::core::IUnknown {
    fn from(value: &DisplayFence) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayFence> for ::windows::core::IInspectable {
    fn from(value: DisplayFence) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayFence> for ::windows::core::IInspectable {
    fn from(value: &DisplayFence) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayFence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayFence {}
unsafe impl ::core::marker::Sync for DisplayFence {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayManager(pub ::windows::core::IInspectable);
impl DisplayManager {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentTargets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentAdapters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>>(result__)
        }
    }
    pub fn TryAcquireTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayManagerResult> {
        let this = self;
        unsafe {
            let mut result__: DisplayManagerResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayManagerResult>(result__)
        }
    }
    pub fn ReleaseTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() }
    }
    pub fn TryReadCurrentStateForAllTargets(&self) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndReadCurrentState<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DisplayTarget>>>(&self, targets: Param0) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), targets.into_param().abi(), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndCreateEmptyState<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DisplayTarget>>>(&self, targets: Param0) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), targets.into_param().abi(), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndCreateSubstate<'a, Param0: ::windows::core::IntoParam<'a, DisplayState>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DisplayTarget>>>(&self, existingstate: Param0, targets: Param1) -> ::windows::core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), existingstate.into_param().abi(), targets.into_param().abi(), &mut result__).from_abi::<DisplayManagerResultWithState>(result__)
        }
    }
    pub fn CreateDisplayDevice<'a, Param0: ::windows::core::IntoParam<'a, DisplayAdapter>>(&self, adapter: Param0) -> ::windows::core::Result<DisplayDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), adapter.into_param().abi(), &mut result__).from_abi::<DisplayDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Enabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Disabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PathsFailedOrInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePathsFailedOrInvalidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Create(options: DisplayManagerOptions) -> ::windows::core::Result<DisplayManager> {
        Self::IDisplayManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<DisplayManager>(result__)
        })
    }
    pub fn IDisplayManagerStatics<R, F: FnOnce(&IDisplayManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayManager, IDisplayManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManager;{4ed9245b-15ec-56e2-9072-7fe5084a31a7})");
}
unsafe impl ::windows::core::Interface for DisplayManager {
    type Vtable = IDisplayManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ed9245b_15ec_56e2_9072_7fe5084a31a7);
}
impl ::windows::core::RuntimeName for DisplayManager {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManager";
}
impl ::core::convert::From<DisplayManager> for ::windows::core::IUnknown {
    fn from(value: DisplayManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayManager> for ::windows::core::IUnknown {
    fn from(value: &DisplayManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayManager> for ::windows::core::IInspectable {
    fn from(value: DisplayManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayManager> for ::windows::core::IInspectable {
    fn from(value: &DisplayManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayManagerChangedEventArgs(pub ::windows::core::IInspectable);
impl DisplayManagerChangedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerChangedEventArgs;{6abfa285-6cca-5731-bcdc-42e5d2f5c50f})");
}
unsafe impl ::windows::core::Interface for DisplayManagerChangedEventArgs {
    type Vtable = IDisplayManagerChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6abfa285_6cca_5731_bcdc_42e5d2f5c50f);
}
impl ::windows::core::RuntimeName for DisplayManagerChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerChangedEventArgs";
}
impl ::core::convert::From<DisplayManagerChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayManagerChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayManagerChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayManagerChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayManagerChangedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayManagerDisabledEventArgs(pub ::windows::core::IInspectable);
impl DisplayManagerDisabledEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerDisabledEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs;{8726dde4-6793-5973-a11f-5ffbc93fdb90})");
}
unsafe impl ::windows::core::Interface for DisplayManagerDisabledEventArgs {
    type Vtable = IDisplayManagerDisabledEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8726dde4_6793_5973_a11f_5ffbc93fdb90);
}
impl ::windows::core::RuntimeName for DisplayManagerDisabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs";
}
impl ::core::convert::From<DisplayManagerDisabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerDisabledEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayManagerDisabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerDisabledEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayManagerDisabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerDisabledEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayManagerDisabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerDisabledEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerDisabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayManagerDisabledEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerDisabledEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayManagerEnabledEventArgs(pub ::windows::core::IInspectable);
impl DisplayManagerEnabledEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerEnabledEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs;{f0cf3f6f-42fa-59a2-b297-26e1713de848})");
}
unsafe impl ::windows::core::Interface for DisplayManagerEnabledEventArgs {
    type Vtable = IDisplayManagerEnabledEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0cf3f6f_42fa_59a2_b297_26e1713de848);
}
impl ::windows::core::RuntimeName for DisplayManagerEnabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs";
}
impl ::core::convert::From<DisplayManagerEnabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerEnabledEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayManagerEnabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerEnabledEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayManagerEnabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerEnabledEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayManagerEnabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerEnabledEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayManagerEnabledEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerEnabledEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayManagerOptions(pub u32);
impl DisplayManagerOptions {
    pub const None: DisplayManagerOptions = DisplayManagerOptions(0u32);
    pub const EnforceSourceOwnership: DisplayManagerOptions = DisplayManagerOptions(1u32);
    pub const VirtualRefreshRateAware: DisplayManagerOptions = DisplayManagerOptions(2u32);
}
impl ::core::convert::From<u32> for DisplayManagerOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayManagerOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayManagerOptions;u4)");
}
impl ::windows::core::DefaultType for DisplayManagerOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayManagerOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayManagerOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayManagerOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayManagerOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayManagerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(pub ::windows::core::IInspectable);
impl DisplayManagerPathsFailedOrInvalidatedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs;{03a65659-1dec-5c15-b2a2-8fe9129869fe})");
}
unsafe impl ::windows::core::Interface for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    type Vtable = IDisplayManagerPathsFailedOrInvalidatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03a65659_1dec_5c15_b2a2_8fe9129869fe);
}
impl ::windows::core::RuntimeName for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs";
}
impl ::core::convert::From<DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayManagerPathsFailedOrInvalidatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerPathsFailedOrInvalidatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: DisplayManagerResult = DisplayManagerResult(0i32);
    pub const UnknownFailure: DisplayManagerResult = DisplayManagerResult(1i32);
    pub const TargetAccessDenied: DisplayManagerResult = DisplayManagerResult(2i32);
    pub const TargetStale: DisplayManagerResult = DisplayManagerResult(3i32);
    pub const RemoteSessionNotSupported: DisplayManagerResult = DisplayManagerResult(4i32);
}
impl ::core::convert::From<i32> for DisplayManagerResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayManagerResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayManagerResult;i4)");
}
impl ::windows::core::DefaultType for DisplayManagerResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayManagerResultWithState(pub ::windows::core::IInspectable);
impl DisplayManagerResultWithState {
    pub fn ErrorCode(&self) -> ::windows::core::Result<DisplayManagerResult> {
        let this = self;
        unsafe {
            let mut result__: DisplayManagerResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayManagerResult>(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<DisplayState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayManagerResultWithState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayManagerResultWithState;{8e656aa6-6614-54be-bfef-4994547f7be1})");
}
unsafe impl ::windows::core::Interface for DisplayManagerResultWithState {
    type Vtable = IDisplayManagerResultWithState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e656aa6_6614_54be_bfef_4994547f7be1);
}
impl ::windows::core::RuntimeName for DisplayManagerResultWithState {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerResultWithState";
}
impl ::core::convert::From<DisplayManagerResultWithState> for ::windows::core::IUnknown {
    fn from(value: DisplayManagerResultWithState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayManagerResultWithState> for ::windows::core::IUnknown {
    fn from(value: &DisplayManagerResultWithState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayManagerResultWithState> for ::windows::core::IInspectable {
    fn from(value: DisplayManagerResultWithState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayManagerResultWithState> for ::windows::core::IInspectable {
    fn from(value: &DisplayManagerResultWithState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayManagerResultWithState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayManagerResultWithState {}
unsafe impl ::core::marker::Sync for DisplayManagerResultWithState {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayModeInfo(pub ::windows::core::IInspectable);
impl DisplayModeInfo {
    #[cfg(feature = "Graphics")]
    pub fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::SizeInt32>(result__)
        }
    }
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate> {
        let this = self;
        unsafe {
            let mut result__: DisplayPresentationRate = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPresentationRate>(result__)
        }
    }
    pub fn IsInterlaced(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetWireFormatSupportedBitsPerChannel(&self, encoding: DisplayWireFormatPixelEncoding) -> ::windows::core::Result<DisplayBitsPerChannel> {
        let this = self;
        unsafe {
            let mut result__: DisplayBitsPerChannel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), encoding, &mut result__).from_abi::<DisplayBitsPerChannel>(result__)
        }
    }
    pub fn IsWireFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, DisplayWireFormat>>(&self, wireformat: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), wireformat.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PhysicalPresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate> {
        let this = &::windows::core::Interface::cast::<IDisplayModeInfo2>(self)?;
        unsafe {
            let mut result__: DisplayPresentationRate = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPresentationRate>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayModeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayModeInfo;{48d513a0-f79b-5a74-a05e-da821f470868})");
}
unsafe impl ::windows::core::Interface for DisplayModeInfo {
    type Vtable = IDisplayModeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48d513a0_f79b_5a74_a05e_da821f470868);
}
impl ::windows::core::RuntimeName for DisplayModeInfo {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayModeInfo";
}
impl ::core::convert::From<DisplayModeInfo> for ::windows::core::IUnknown {
    fn from(value: DisplayModeInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayModeInfo> for ::windows::core::IUnknown {
    fn from(value: &DisplayModeInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayModeInfo> for ::windows::core::IInspectable {
    fn from(value: DisplayModeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayModeInfo> for ::windows::core::IInspectable {
    fn from(value: &DisplayModeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayModeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayModeInfo {}
unsafe impl ::core::marker::Sync for DisplayModeInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: DisplayModeQueryOptions = DisplayModeQueryOptions(0u32);
    pub const OnlyPreferredResolution: DisplayModeQueryOptions = DisplayModeQueryOptions(1u32);
}
impl ::core::convert::From<u32> for DisplayModeQueryOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayModeQueryOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayModeQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayModeQueryOptions;u4)");
}
impl ::windows::core::DefaultType for DisplayModeQueryOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayModeQueryOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayModeQueryOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayModeQueryOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayModeQueryOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayModeQueryOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayPath(pub ::windows::core::IInspectable);
impl DisplayPath {
    pub fn View(&self) -> ::windows::core::Result<DisplayView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayView>(result__)
        }
    }
    pub fn Target(&self) -> ::windows::core::Result<DisplayTarget> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTarget>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<DisplayPathStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayPathStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPathStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SetSourceResolution<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetSourcePixelFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStereo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SetTargetResolution<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn PresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<DisplayPresentationRate>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetPresentationRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<DisplayPresentationRate>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsInterlaced(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetIsInterlaced<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn WireFormat(&self) -> ::windows::core::Result<DisplayWireFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormat>(result__)
        }
    }
    pub fn SetWireFormat<'a, Param0: ::windows::core::IntoParam<'a, DisplayWireFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<DisplayRotation> {
        let this = self;
        unsafe {
            let mut result__: DisplayRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayRotation>(result__)
        }
    }
    pub fn SetRotation(&self, value: DisplayRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Scaling(&self) -> ::windows::core::Result<DisplayPathScaling> {
        let this = self;
        unsafe {
            let mut result__: DisplayPathScaling = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPathScaling>(result__)
        }
    }
    pub fn SetScaling(&self, value: DisplayPathScaling) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindModes(&self, flags: DisplayModeQueryOptions) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), flags, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>>(result__)
        }
    }
    pub fn ApplyPropertiesFromMode<'a, Param0: ::windows::core::IntoParam<'a, DisplayModeInfo>>(&self, moderesult: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), moderesult.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn PhysicalPresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>> {
        let this = &::windows::core::Interface::cast::<IDisplayPath2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<DisplayPresentationRate>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetPhysicalPresentationRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<DisplayPresentationRate>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplayPath2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPath {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayPath;{b3dfd64a-7460-5cde-811b-d5ae9f3d9f84})");
}
unsafe impl ::windows::core::Interface for DisplayPath {
    type Vtable = IDisplayPath_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3dfd64a_7460_5cde_811b_d5ae9f3d9f84);
}
impl ::windows::core::RuntimeName for DisplayPath {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayPath";
}
impl ::core::convert::From<DisplayPath> for ::windows::core::IUnknown {
    fn from(value: DisplayPath) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayPath> for ::windows::core::IUnknown {
    fn from(value: &DisplayPath) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayPath> for ::windows::core::IInspectable {
    fn from(value: DisplayPath) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayPath> for ::windows::core::IInspectable {
    fn from(value: &DisplayPath) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayPath {}
unsafe impl ::core::marker::Sync for DisplayPath {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: DisplayPathScaling = DisplayPathScaling(0i32);
    pub const Centered: DisplayPathScaling = DisplayPathScaling(1i32);
    pub const Stretched: DisplayPathScaling = DisplayPathScaling(2i32);
    pub const AspectRatioStretched: DisplayPathScaling = DisplayPathScaling(3i32);
    pub const Custom: DisplayPathScaling = DisplayPathScaling(4i32);
    pub const DriverPreferred: DisplayPathScaling = DisplayPathScaling(5i32);
}
impl ::core::convert::From<i32> for DisplayPathScaling {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayPathScaling {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayPathScaling {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPathScaling;i4)");
}
impl ::windows::core::DefaultType for DisplayPathScaling {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: DisplayPathStatus = DisplayPathStatus(0i32);
    pub const Succeeded: DisplayPathStatus = DisplayPathStatus(1i32);
    pub const Pending: DisplayPathStatus = DisplayPathStatus(2i32);
    pub const Failed: DisplayPathStatus = DisplayPathStatus(3i32);
    pub const FailedAsync: DisplayPathStatus = DisplayPathStatus(4i32);
    pub const InvalidatedAsync: DisplayPathStatus = DisplayPathStatus(5i32);
}
impl ::core::convert::From<i32> for DisplayPathStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayPathStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayPathStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPathStatus;i4)");
}
impl ::windows::core::DefaultType for DisplayPathStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: DisplayPresentStatus = DisplayPresentStatus(0i32);
    pub const SourceStatusPreventedPresent: DisplayPresentStatus = DisplayPresentStatus(1i32);
    pub const ScanoutInvalid: DisplayPresentStatus = DisplayPresentStatus(2i32);
    pub const SourceInvalid: DisplayPresentStatus = DisplayPresentStatus(3i32);
    pub const DeviceInvalid: DisplayPresentStatus = DisplayPresentStatus(4i32);
    pub const UnknownFailure: DisplayPresentStatus = DisplayPresentStatus(5i32);
}
impl ::core::convert::From<i32> for DisplayPresentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayPresentStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayPresentStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPresentStatus;i4)");
}
impl ::windows::core::DefaultType for DisplayPresentStatus {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct DisplayPresentationRate {
    pub VerticalSyncRate: super::super::super::Foundation::Numerics::Rational,
    pub VerticalSyncsPerPresentation: i32,
}
#[cfg(feature = "Foundation_Numerics")]
impl DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for DisplayPresentationRate {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for DisplayPresentationRate {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DisplayPresentationRate").field("VerticalSyncRate", &self.VerticalSyncRate).field("VerticalSyncsPerPresentation", &self.VerticalSyncsPerPresentation).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for DisplayPresentationRate {
    fn eq(&self, other: &Self) -> bool {
        self.VerticalSyncRate == other.VerticalSyncRate && self.VerticalSyncsPerPresentation == other.VerticalSyncsPerPresentation
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for DisplayPresentationRate {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for DisplayPresentationRate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Display.Core.DisplayPresentationRate;struct(Windows.Foundation.Numerics.Rational;u4;u4);i4)");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for DisplayPresentationRate {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayPrimaryDescription(pub ::windows::core::IInspectable);
impl DisplayPrimaryDescription {
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn ColorSpace(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXColorSpace> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXColorSpace = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXColorSpace>(result__)
        }
    }
    pub fn IsStereo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn MultisampleDescription(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateInstance<'a, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>>(width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: Param5) -> ::windows::core::Result<DisplayPrimaryDescription> {
        Self::IDisplayPrimaryDescriptionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), width, height, pixelformat, colorspace, isstereo, multisampledescription.into_param().abi(), &mut result__).from_abi::<DisplayPrimaryDescription>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, Param6: ::windows::core::IntoParam<'a, super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>>(
        extraproperties: Param0,
        width: u32,
        height: u32,
        pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat,
        colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace,
        isstereo: bool,
        multisampledescription: Param6,
    ) -> ::windows::core::Result<DisplayPrimaryDescription> {
        Self::IDisplayPrimaryDescriptionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), extraproperties.into_param().abi(), width, height, pixelformat, colorspace, isstereo, multisampledescription.into_param().abi(), &mut result__).from_abi::<DisplayPrimaryDescription>(result__)
        })
    }
    pub fn IDisplayPrimaryDescriptionFactory<R, F: FnOnce(&IDisplayPrimaryDescriptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayPrimaryDescription, IDisplayPrimaryDescriptionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDisplayPrimaryDescriptionStatics<R, F: FnOnce(&IDisplayPrimaryDescriptionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayPrimaryDescription, IDisplayPrimaryDescriptionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayPrimaryDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayPrimaryDescription;{872591d2-d533-50ff-a85e-06696194b77c})");
}
unsafe impl ::windows::core::Interface for DisplayPrimaryDescription {
    type Vtable = IDisplayPrimaryDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x872591d2_d533_50ff_a85e_06696194b77c);
}
impl ::windows::core::RuntimeName for DisplayPrimaryDescription {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayPrimaryDescription";
}
impl ::core::convert::From<DisplayPrimaryDescription> for ::windows::core::IUnknown {
    fn from(value: DisplayPrimaryDescription) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayPrimaryDescription> for ::windows::core::IUnknown {
    fn from(value: &DisplayPrimaryDescription) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayPrimaryDescription> for ::windows::core::IInspectable {
    fn from(value: DisplayPrimaryDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayPrimaryDescription> for ::windows::core::IInspectable {
    fn from(value: &DisplayPrimaryDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayPrimaryDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayPrimaryDescription {}
unsafe impl ::core::marker::Sync for DisplayPrimaryDescription {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: DisplayRotation = DisplayRotation(0i32);
    pub const Clockwise90Degrees: DisplayRotation = DisplayRotation(1i32);
    pub const Clockwise180Degrees: DisplayRotation = DisplayRotation(2i32);
    pub const Clockwise270Degrees: DisplayRotation = DisplayRotation(3i32);
}
impl ::core::convert::From<i32> for DisplayRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayRotation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayRotation;i4)");
}
impl ::windows::core::DefaultType for DisplayRotation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayScanout(pub ::windows::core::IInspectable);
impl DisplayScanout {}
unsafe impl ::windows::core::RuntimeType for DisplayScanout {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayScanout;{e3051828-1ba5-50e7-8a39-bb1fd2f4f8b9})");
}
unsafe impl ::windows::core::Interface for DisplayScanout {
    type Vtable = IDisplayScanout_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3051828_1ba5_50e7_8a39_bb1fd2f4f8b9);
}
impl ::windows::core::RuntimeName for DisplayScanout {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayScanout";
}
impl ::core::convert::From<DisplayScanout> for ::windows::core::IUnknown {
    fn from(value: DisplayScanout) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayScanout> for ::windows::core::IUnknown {
    fn from(value: &DisplayScanout) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayScanout> for ::windows::core::IInspectable {
    fn from(value: DisplayScanout) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayScanout> for ::windows::core::IInspectable {
    fn from(value: &DisplayScanout) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayScanout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayScanout {}
unsafe impl ::core::marker::Sync for DisplayScanout {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: DisplayScanoutOptions = DisplayScanoutOptions(0u32);
    pub const AllowTearing: DisplayScanoutOptions = DisplayScanoutOptions(2u32);
}
impl ::core::convert::From<u32> for DisplayScanoutOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayScanoutOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayScanoutOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayScanoutOptions;u4)");
}
impl ::windows::core::DefaultType for DisplayScanoutOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayScanoutOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayScanoutOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayScanoutOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayScanoutOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayScanoutOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplaySource(pub ::windows::core::IInspectable);
impl DisplaySource {
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    pub fn SourceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetMetadata<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<DisplaySourceStatus> {
        let this = &::windows::core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe {
            let mut result__: DisplaySourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplaySourceStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplaySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplaySource;{ecd15fc1-eadc-51bc-971d-3bc628db2dd4})");
}
unsafe impl ::windows::core::Interface for DisplaySource {
    type Vtable = IDisplaySource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecd15fc1_eadc_51bc_971d_3bc628db2dd4);
}
impl ::windows::core::RuntimeName for DisplaySource {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplaySource";
}
impl ::core::convert::From<DisplaySource> for ::windows::core::IUnknown {
    fn from(value: DisplaySource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplaySource> for ::windows::core::IUnknown {
    fn from(value: &DisplaySource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplaySource> for ::windows::core::IInspectable {
    fn from(value: DisplaySource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplaySource> for ::windows::core::IInspectable {
    fn from(value: &DisplaySource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplaySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplaySource {}
unsafe impl ::core::marker::Sync for DisplaySource {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: DisplaySourceStatus = DisplaySourceStatus(0i32);
    pub const PoweredOff: DisplaySourceStatus = DisplaySourceStatus(1i32);
    pub const Invalid: DisplaySourceStatus = DisplaySourceStatus(2i32);
    pub const OwnedByAnotherDevice: DisplaySourceStatus = DisplaySourceStatus(3i32);
    pub const Unowned: DisplaySourceStatus = DisplaySourceStatus(4i32);
}
impl ::core::convert::From<i32> for DisplaySourceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplaySourceStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplaySourceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplaySourceStatus;i4)");
}
impl ::windows::core::DefaultType for DisplaySourceStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayState(pub ::windows::core::IInspectable);
impl DisplayState {
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsStale(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Targets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayView>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayView>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    pub fn ConnectTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayPath>(result__)
        }
    }
    pub fn ConnectTargetToView<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, DisplayView>>(&self, target: Param0, view: Param1) -> ::windows::core::Result<DisplayPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), target.into_param().abi(), view.into_param().abi(), &mut result__).from_abi::<DisplayPath>(result__)
        }
    }
    pub fn CanConnectTargetToView<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>, Param1: ::windows::core::IntoParam<'a, DisplayView>>(&self, target: Param0, view: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), target.into_param().abi(), view.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetViewForTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayView>(result__)
        }
    }
    pub fn GetPathForTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<DisplayPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<DisplayPath>(result__)
        }
    }
    pub fn DisconnectTarget<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, target: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), target.into_param().abi()).ok() }
    }
    pub fn TryFunctionalize(&self, options: DisplayStateFunctionalizeOptions) -> ::windows::core::Result<DisplayStateOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<DisplayStateOperationResult>(result__)
        }
    }
    pub fn TryApply(&self, options: DisplayStateApplyOptions) -> ::windows::core::Result<DisplayStateOperationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<DisplayStateOperationResult>(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<DisplayState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayState;{08129321-11b5-5cb2-99f8-e90b479a8a1d})");
}
unsafe impl ::windows::core::Interface for DisplayState {
    type Vtable = IDisplayState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08129321_11b5_5cb2_99f8_e90b479a8a1d);
}
impl ::windows::core::RuntimeName for DisplayState {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayState";
}
impl ::core::convert::From<DisplayState> for ::windows::core::IUnknown {
    fn from(value: DisplayState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayState> for ::windows::core::IUnknown {
    fn from(value: &DisplayState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayState> for ::windows::core::IInspectable {
    fn from(value: DisplayState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayState> for ::windows::core::IInspectable {
    fn from(value: &DisplayState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayState {}
unsafe impl ::core::marker::Sync for DisplayState {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: DisplayStateApplyOptions = DisplayStateApplyOptions(0u32);
    pub const FailIfStateChanged: DisplayStateApplyOptions = DisplayStateApplyOptions(1u32);
    pub const ForceReapply: DisplayStateApplyOptions = DisplayStateApplyOptions(2u32);
    pub const ForceModeEnumeration: DisplayStateApplyOptions = DisplayStateApplyOptions(4u32);
}
impl ::core::convert::From<u32> for DisplayStateApplyOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayStateApplyOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayStateApplyOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateApplyOptions;u4)");
}
impl ::windows::core::DefaultType for DisplayStateApplyOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayStateApplyOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayStateApplyOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayStateApplyOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayStateApplyOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayStateApplyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: DisplayStateFunctionalizeOptions = DisplayStateFunctionalizeOptions(0u32);
    pub const FailIfStateChanged: DisplayStateFunctionalizeOptions = DisplayStateFunctionalizeOptions(1u32);
    pub const ValidateTopologyOnly: DisplayStateFunctionalizeOptions = DisplayStateFunctionalizeOptions(2u32);
}
impl ::core::convert::From<u32> for DisplayStateFunctionalizeOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayStateFunctionalizeOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayStateFunctionalizeOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateFunctionalizeOptions;u4)");
}
impl ::windows::core::DefaultType for DisplayStateFunctionalizeOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayStateFunctionalizeOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayStateFunctionalizeOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayStateOperationResult(pub ::windows::core::IInspectable);
impl DisplayStateOperationResult {
    pub fn Status(&self) -> ::windows::core::Result<DisplayStateOperationStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayStateOperationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayStateOperationStatus>(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayStateOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayStateOperationResult;{fcadbfdf-dc27-5638-b7f2-ebdfa4f7ea93})");
}
unsafe impl ::windows::core::Interface for DisplayStateOperationResult {
    type Vtable = IDisplayStateOperationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcadbfdf_dc27_5638_b7f2_ebdfa4f7ea93);
}
impl ::windows::core::RuntimeName for DisplayStateOperationResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayStateOperationResult";
}
impl ::core::convert::From<DisplayStateOperationResult> for ::windows::core::IUnknown {
    fn from(value: DisplayStateOperationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayStateOperationResult> for ::windows::core::IUnknown {
    fn from(value: &DisplayStateOperationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayStateOperationResult> for ::windows::core::IInspectable {
    fn from(value: DisplayStateOperationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayStateOperationResult> for ::windows::core::IInspectable {
    fn from(value: &DisplayStateOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayStateOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayStateOperationResult {}
unsafe impl ::core::marker::Sync for DisplayStateOperationResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: DisplayStateOperationStatus = DisplayStateOperationStatus(0i32);
    pub const PartialFailure: DisplayStateOperationStatus = DisplayStateOperationStatus(1i32);
    pub const UnknownFailure: DisplayStateOperationStatus = DisplayStateOperationStatus(2i32);
    pub const TargetOwnershipLost: DisplayStateOperationStatus = DisplayStateOperationStatus(3i32);
    pub const SystemStateChanged: DisplayStateOperationStatus = DisplayStateOperationStatus(4i32);
    pub const TooManyPathsForAdapter: DisplayStateOperationStatus = DisplayStateOperationStatus(5i32);
    pub const ModesNotSupported: DisplayStateOperationStatus = DisplayStateOperationStatus(6i32);
    pub const RemoteSessionNotSupported: DisplayStateOperationStatus = DisplayStateOperationStatus(7i32);
}
impl ::core::convert::From<i32> for DisplayStateOperationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayStateOperationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayStateOperationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateOperationStatus;i4)");
}
impl ::windows::core::DefaultType for DisplayStateOperationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplaySurface(pub ::windows::core::IInspectable);
impl DisplaySurface {}
unsafe impl ::windows::core::RuntimeType for DisplaySurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplaySurface;{594f6cc6-139a-56d6-a4b1-15fe2cb76adb})");
}
unsafe impl ::windows::core::Interface for DisplaySurface {
    type Vtable = IDisplaySurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f6cc6_139a_56d6_a4b1_15fe2cb76adb);
}
impl ::windows::core::RuntimeName for DisplaySurface {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplaySurface";
}
impl ::core::convert::From<DisplaySurface> for ::windows::core::IUnknown {
    fn from(value: DisplaySurface) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplaySurface> for ::windows::core::IUnknown {
    fn from(value: &DisplaySurface) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplaySurface> for ::windows::core::IInspectable {
    fn from(value: DisplaySurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplaySurface> for ::windows::core::IInspectable {
    fn from(value: &DisplaySurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplaySurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplaySurface {}
unsafe impl ::core::marker::Sync for DisplaySurface {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayTarget(pub ::windows::core::IInspectable);
impl DisplayTarget {
    pub fn Adapter(&self) -> ::windows::core::Result<DisplayAdapter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayAdapter>(result__)
        }
    }
    pub fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AdapterRelativeId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsVirtualModeEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsVirtualTopologyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn UsageKind(&self) -> ::windows::core::Result<super::DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayMonitorUsageKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DisplayMonitorUsageKind>(result__)
        }
    }
    pub fn MonitorPersistence(&self) -> ::windows::core::Result<DisplayTargetPersistence> {
        let this = self;
        unsafe {
            let mut result__: DisplayTargetPersistence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTargetPersistence>(result__)
        }
    }
    pub fn StableMonitorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TryGetMonitor(&self) -> ::windows::core::Result<super::DisplayMonitor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DisplayMonitor>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    pub fn IsStale(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsSame<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, othertarget: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), othertarget.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, DisplayTarget>>(&self, othertarget: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), othertarget.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTarget;{aec57c6f-47b4-546b-987c-e73fa791fe3a})");
}
unsafe impl ::windows::core::Interface for DisplayTarget {
    type Vtable = IDisplayTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec57c6f_47b4_546b_987c_e73fa791fe3a);
}
impl ::windows::core::RuntimeName for DisplayTarget {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTarget";
}
impl ::core::convert::From<DisplayTarget> for ::windows::core::IUnknown {
    fn from(value: DisplayTarget) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayTarget> for ::windows::core::IUnknown {
    fn from(value: &DisplayTarget) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayTarget> for ::windows::core::IInspectable {
    fn from(value: DisplayTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayTarget> for ::windows::core::IInspectable {
    fn from(value: &DisplayTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayTarget {}
unsafe impl ::core::marker::Sync for DisplayTarget {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: DisplayTargetPersistence = DisplayTargetPersistence(0i32);
    pub const BootPersisted: DisplayTargetPersistence = DisplayTargetPersistence(1i32);
    pub const TemporaryPersisted: DisplayTargetPersistence = DisplayTargetPersistence(2i32);
    pub const PathPersisted: DisplayTargetPersistence = DisplayTargetPersistence(3i32);
}
impl ::core::convert::From<i32> for DisplayTargetPersistence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayTargetPersistence {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayTargetPersistence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayTargetPersistence;i4)");
}
impl ::windows::core::DefaultType for DisplayTargetPersistence {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayTask(pub ::windows::core::IInspectable);
impl DisplayTask {
    pub fn SetScanout<'a, Param0: ::windows::core::IntoParam<'a, DisplayScanout>>(&self, scanout: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), scanout.into_param().abi()).ok() }
    }
    pub fn SetWait<'a, Param0: ::windows::core::IntoParam<'a, DisplayFence>>(&self, readyfence: Param0, readyfencevalue: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), readyfence.into_param().abi(), readyfencevalue).ok() }
    }
    pub fn SetSignal<'a, Param1: ::windows::core::IntoParam<'a, DisplayFence>>(&self, signalkind: DisplayTaskSignalKind, fence: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDisplayTask2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), signalkind, fence.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTask;{5e087448-135b-5bb0-bf63-637f84227c7a})");
}
unsafe impl ::windows::core::Interface for DisplayTask {
    type Vtable = IDisplayTask_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e087448_135b_5bb0_bf63_637f84227c7a);
}
impl ::windows::core::RuntimeName for DisplayTask {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTask";
}
impl ::core::convert::From<DisplayTask> for ::windows::core::IUnknown {
    fn from(value: DisplayTask) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayTask> for ::windows::core::IUnknown {
    fn from(value: &DisplayTask) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayTask> for ::windows::core::IInspectable {
    fn from(value: DisplayTask) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayTask> for ::windows::core::IInspectable {
    fn from(value: &DisplayTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayTask {}
unsafe impl ::core::marker::Sync for DisplayTask {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayTaskPool(pub ::windows::core::IInspectable);
impl DisplayTaskPool {
    pub fn CreateTask(&self) -> ::windows::core::Result<DisplayTask> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayTask>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExecuteTask<'a, Param0: ::windows::core::IntoParam<'a, DisplayTask>>(&self, task: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), task.into_param().abi()).ok() }
    }
    pub fn TryExecuteTask<'a, Param0: ::windows::core::IntoParam<'a, DisplayTask>>(&self, task: Param0) -> ::windows::core::Result<DisplayTaskResult> {
        let this = &::windows::core::Interface::cast::<IDisplayTaskPool2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), task.into_param().abi(), &mut result__).from_abi::<DisplayTaskResult>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTaskPool {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTaskPool;{c676253d-237d-5548-aafa-3e517fefef1c})");
}
unsafe impl ::windows::core::Interface for DisplayTaskPool {
    type Vtable = IDisplayTaskPool_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc676253d_237d_5548_aafa_3e517fefef1c);
}
impl ::windows::core::RuntimeName for DisplayTaskPool {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTaskPool";
}
impl ::core::convert::From<DisplayTaskPool> for ::windows::core::IUnknown {
    fn from(value: DisplayTaskPool) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayTaskPool> for ::windows::core::IUnknown {
    fn from(value: &DisplayTaskPool) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayTaskPool> for ::windows::core::IInspectable {
    fn from(value: DisplayTaskPool) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayTaskPool> for ::windows::core::IInspectable {
    fn from(value: &DisplayTaskPool) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTaskPool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayTaskPool {}
unsafe impl ::core::marker::Sync for DisplayTaskPool {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayTaskResult(pub ::windows::core::IInspectable);
impl DisplayTaskResult {
    pub fn PresentStatus(&self) -> ::windows::core::Result<DisplayPresentStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplayPresentStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayPresentStatus>(result__)
        }
    }
    pub fn PresentId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SourceStatus(&self) -> ::windows::core::Result<DisplaySourceStatus> {
        let this = self;
        unsafe {
            let mut result__: DisplaySourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplaySourceStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayTaskResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayTaskResult;{6fbc7d67-f9b1-55e0-9d88-d3a5197a3f59})");
}
unsafe impl ::windows::core::Interface for DisplayTaskResult {
    type Vtable = IDisplayTaskResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fbc7d67_f9b1_55e0_9d88_d3a5197a3f59);
}
impl ::windows::core::RuntimeName for DisplayTaskResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTaskResult";
}
impl ::core::convert::From<DisplayTaskResult> for ::windows::core::IUnknown {
    fn from(value: DisplayTaskResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayTaskResult> for ::windows::core::IUnknown {
    fn from(value: &DisplayTaskResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayTaskResult> for ::windows::core::IInspectable {
    fn from(value: DisplayTaskResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayTaskResult> for ::windows::core::IInspectable {
    fn from(value: &DisplayTaskResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayTaskResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayTaskResult {}
unsafe impl ::core::marker::Sync for DisplayTaskResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayTaskSignalKind(pub i32);
impl DisplayTaskSignalKind {
    pub const OnPresentFlipAway: DisplayTaskSignalKind = DisplayTaskSignalKind(0i32);
    pub const OnPresentFlipTo: DisplayTaskSignalKind = DisplayTaskSignalKind(1i32);
}
impl ::core::convert::From<i32> for DisplayTaskSignalKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayTaskSignalKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayTaskSignalKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayTaskSignalKind;i4)");
}
impl ::windows::core::DefaultType for DisplayTaskSignalKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayView(pub ::windows::core::IInspectable);
impl DisplayView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Paths(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayPath>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<DisplayPath>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn ContentResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub fn SetContentResolution<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetPrimaryPath<'a, Param0: ::windows::core::IntoParam<'a, DisplayPath>>(&self, path: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), path.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayView;{b0c98ca1-b759-5b59-b1ad-f0786aa9e53d})");
}
unsafe impl ::windows::core::Interface for DisplayView {
    type Vtable = IDisplayView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0c98ca1_b759_5b59_b1ad_f0786aa9e53d);
}
impl ::windows::core::RuntimeName for DisplayView {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayView";
}
impl ::core::convert::From<DisplayView> for ::windows::core::IUnknown {
    fn from(value: DisplayView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayView> for ::windows::core::IUnknown {
    fn from(value: &DisplayView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayView> for ::windows::core::IInspectable {
    fn from(value: DisplayView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayView> for ::windows::core::IInspectable {
    fn from(value: &DisplayView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayView {}
unsafe impl ::core::marker::Sync for DisplayView {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DisplayWireFormat(pub ::windows::core::IInspectable);
impl DisplayWireFormat {
    pub fn PixelEncoding(&self) -> ::windows::core::Result<DisplayWireFormatPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatPixelEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatPixelEncoding>(result__)
        }
    }
    pub fn BitsPerChannel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ColorSpace(&self) -> ::windows::core::Result<DisplayWireFormatColorSpace> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatColorSpace = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatColorSpace>(result__)
        }
    }
    pub fn Eotf(&self) -> ::windows::core::Result<DisplayWireFormatEotf> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatEotf = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatEotf>(result__)
        }
    }
    pub fn HdrMetadata(&self) -> ::windows::core::Result<DisplayWireFormatHdrMetadata> {
        let this = self;
        unsafe {
            let mut result__: DisplayWireFormatHdrMetadata = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayWireFormatHdrMetadata>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    pub fn CreateInstance(pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat> {
        Self::IDisplayWireFormatFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata, &mut result__).from_abi::<DisplayWireFormat>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>>(
        extraproperties: Param0,
        pixelencoding: DisplayWireFormatPixelEncoding,
        bitsperchannel: i32,
        colorspace: DisplayWireFormatColorSpace,
        eotf: DisplayWireFormatEotf,
        hdrmetadata: DisplayWireFormatHdrMetadata,
    ) -> ::windows::core::Result<DisplayWireFormat> {
        Self::IDisplayWireFormatStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), extraproperties.into_param().abi(), pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata, &mut result__).from_abi::<DisplayWireFormat>(result__)
        })
    }
    pub fn IDisplayWireFormatFactory<R, F: FnOnce(&IDisplayWireFormatFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayWireFormat, IDisplayWireFormatFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDisplayWireFormatStatics<R, F: FnOnce(&IDisplayWireFormatStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DisplayWireFormat, IDisplayWireFormatStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.Core.DisplayWireFormat;{1acc967d-872c-5a38-bbb9-1d4872b76255})");
}
unsafe impl ::windows::core::Interface for DisplayWireFormat {
    type Vtable = IDisplayWireFormat_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1acc967d_872c_5a38_bbb9_1d4872b76255);
}
impl ::windows::core::RuntimeName for DisplayWireFormat {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayWireFormat";
}
impl ::core::convert::From<DisplayWireFormat> for ::windows::core::IUnknown {
    fn from(value: DisplayWireFormat) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DisplayWireFormat> for ::windows::core::IUnknown {
    fn from(value: &DisplayWireFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DisplayWireFormat> for ::windows::core::IInspectable {
    fn from(value: DisplayWireFormat) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DisplayWireFormat> for ::windows::core::IInspectable {
    fn from(value: &DisplayWireFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DisplayWireFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DisplayWireFormat {}
unsafe impl ::core::marker::Sync for DisplayWireFormat {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: DisplayWireFormatColorSpace = DisplayWireFormatColorSpace(0i32);
    pub const BT2020: DisplayWireFormatColorSpace = DisplayWireFormatColorSpace(1i32);
    pub const ProfileDefinedWideColorGamut: DisplayWireFormatColorSpace = DisplayWireFormatColorSpace(2i32);
}
impl ::core::convert::From<i32> for DisplayWireFormatColorSpace {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatColorSpace {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatColorSpace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatColorSpace;i4)");
}
impl ::windows::core::DefaultType for DisplayWireFormatColorSpace {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: DisplayWireFormatEotf = DisplayWireFormatEotf(0i32);
    pub const HdrSmpte2084: DisplayWireFormatEotf = DisplayWireFormatEotf(1i32);
}
impl ::core::convert::From<i32> for DisplayWireFormatEotf {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatEotf {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatEotf {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatEotf;i4)");
}
impl ::windows::core::DefaultType for DisplayWireFormatEotf {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(0i32);
    pub const Hdr10: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(1i32);
    pub const Hdr10Plus: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(2i32);
    pub const DolbyVisionLowLatency: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(3i32);
}
impl ::core::convert::From<i32> for DisplayWireFormatHdrMetadata {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatHdrMetadata {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatHdrMetadata {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatHdrMetadata;i4)");
}
impl ::windows::core::DefaultType for DisplayWireFormatHdrMetadata {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(0i32);
    pub const Ycc444: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(1i32);
    pub const Ycc422: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(2i32);
    pub const Ycc420: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(3i32);
    pub const Intensity: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(4i32);
}
impl ::core::convert::From<i32> for DisplayWireFormatPixelEncoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayWireFormatPixelEncoding {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayWireFormatPixelEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatPixelEncoding;i4)");
}
impl ::windows::core::DefaultType for DisplayWireFormatPixelEncoding {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayAdapter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayAdapter {
    type Vtable = IDisplayAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa56f5287_f000_5f2e_b5ac_3783a2b69af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayAdapterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayAdapterStatics {
    type Vtable = IDisplayAdapterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dac3cda_481f_5469_8470_82c4ba680a28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAdapterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: super::super::super::Graphics::DisplayAdapterId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayDevice {
    type Vtable = IDisplayDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c9b62c_335f_5731_8cb4_c1ccd4731070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, desc: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, offsetfromvblank: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, psurface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capability: DisplayDeviceCapability, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayDevice2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayDevice2 {
    type Vtable = IDisplayDevice2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fefe50c_0940_54bd_a02f_f9c7a536ad60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, subresourceindex: u32, syncinterval: u32, dirtyrects: ::windows::core::RawPtr, options: DisplayScanoutOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayFence(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayFence {
    type Vtable = IDisplayFence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04dcf9ef_3406_5700_8fec_77eba4c5a74b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayFence_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManager {
    type Vtable = IDisplayManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ed9245b_15ec_56e2_9072_7fe5084a31a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, existingstate: ::windows::core::RawPtr, targets: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManagerChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManagerChangedEventArgs {
    type Vtable = IDisplayManagerChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6abfa285_6cca_5731_bcdc_42e5d2f5c50f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManagerDisabledEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManagerDisabledEventArgs {
    type Vtable = IDisplayManagerDisabledEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8726dde4_6793_5973_a11f_5ffbc93fdb90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerDisabledEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManagerEnabledEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManagerEnabledEventArgs {
    type Vtable = IDisplayManagerEnabledEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0cf3f6f_42fa_59a2_b297_26e1713de848);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerEnabledEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    type Vtable = IDisplayManagerPathsFailedOrInvalidatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03a65659_1dec_5c15_b2a2_8fe9129869fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManagerResultWithState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManagerResultWithState {
    type Vtable = IDisplayManagerResultWithState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e656aa6_6614_54be_bfef_4994547f7be1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerResultWithState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayManagerResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayManagerStatics {
    type Vtable = IDisplayManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b6b9446_b999_5535_9d69_53f092c780a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: DisplayManagerOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayModeInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayModeInfo {
    type Vtable = IDisplayModeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48d513a0_f79b_5a74_a05e_da821f470868);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayModeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encoding: DisplayWireFormatPixelEncoding, result__: *mut DisplayBitsPerChannel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wireformat: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayModeInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayModeInfo2 {
    type Vtable = IDisplayModeInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc86fa386_0ddb_5473_bfb0_4b7807b5f909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayModeInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayPresentationRate) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayPath(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayPath {
    type Vtable = IDisplayPath_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3dfd64a_7460_5cde_811b_d5ae9f3d9f84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPath_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayPathStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: DisplayRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayPathScaling) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: DisplayPathScaling) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: DisplayModeQueryOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, moderesult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayPath2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayPath2 {
    type Vtable = IDisplayPath2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf32459c5_e994_570b_9ec8_ef42c35a8547);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPath2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescription(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayPrimaryDescription {
    type Vtable = IDisplayPrimaryDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x872591d2_d533_50ff_a85e_06696194b77c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXColorSpace) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescriptionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayPrimaryDescriptionFactory {
    type Vtable = IDisplayPrimaryDescriptionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a6aff7b_3637_5c46_b479_76d576216e65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescriptionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescriptionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayPrimaryDescriptionStatics {
    type Vtable = IDisplayPrimaryDescriptionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe60e4cfb_36c9_56dd_8fa1_6ff8c4e0ff07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPrimaryDescriptionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, extraproperties: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayScanout(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayScanout {
    type Vtable = IDisplayScanout_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3051828_1ba5_50e7_8a39_bb1fd2f4f8b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayScanout_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplaySource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplaySource {
    type Vtable = IDisplaySource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecd15fc1_eadc_51bc_971d_3bc628db2dd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplaySource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplaySource2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplaySource2 {
    type Vtable = IDisplaySource2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71e18952_b321_5af4_bfe8_03fbea31e40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplaySource2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayState {
    type Vtable = IDisplayState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08129321_11b5_5cb2_99f8_e90b479a8a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: DisplayStateFunctionalizeOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: DisplayStateApplyOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayStateOperationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayStateOperationResult {
    type Vtable = IDisplayStateOperationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcadbfdf_dc27_5638_b7f2_ebdfa4f7ea93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayStateOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayStateOperationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplaySurface(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplaySurface {
    type Vtable = IDisplaySurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f6cc6_139a_56d6_a4b1_15fe2cb76adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplaySurface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayTarget(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayTarget {
    type Vtable = IDisplayTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec57c6f_47b4_546b_987c_e73fa791fe3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::DisplayMonitorUsageKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayTargetPersistence) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, othertarget: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayTask(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayTask {
    type Vtable = IDisplayTask_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e087448_135b_5bb0_bf63_637f84227c7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTask_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scanout: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readyfence: ::windows::core::RawPtr, readyfencevalue: u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayTask2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayTask2 {
    type Vtable = IDisplayTask2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0957ea19_bd55_55de_9267_c97b61e71c37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTask2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalkind: DisplayTaskSignalKind, fence: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayTaskPool(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayTaskPool {
    type Vtable = IDisplayTaskPool_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc676253d_237d_5548_aafa_3e517fefef1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTaskPool_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayTaskPool2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayTaskPool2 {
    type Vtable = IDisplayTaskPool2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46b879b6_5d17_5955_a872_eb38003db586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTaskPool2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayTaskResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayTaskResult {
    type Vtable = IDisplayTaskResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fbc7d67_f9b1_55e0_9d88_d3a5197a3f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayTaskResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayPresentStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplaySourceStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayView {
    type Vtable = IDisplayView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0c98ca1_b759_5b59_b1ad_f0786aa9e53d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayWireFormat(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayWireFormat {
    type Vtable = IDisplayWireFormat_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1acc967d_872c_5a38_bbb9_1d4872b76255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayWireFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayWireFormatPixelEncoding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayWireFormatColorSpace) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayWireFormatEotf) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DisplayWireFormatHdrMetadata) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayWireFormatFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayWireFormatFactory {
    type Vtable = IDisplayWireFormatFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2efc8d5_09d6_55e6_ad22_9014b3d25229);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayWireFormatFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDisplayWireFormatStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDisplayWireFormatStatics {
    type Vtable = IDisplayWireFormatStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc575a22d_c3e6_5f7a_bdfb_87c6ab8661d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayWireFormatStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, extraproperties: ::windows::core::RawPtr, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
