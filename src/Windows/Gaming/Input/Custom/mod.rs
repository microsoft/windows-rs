#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct GameControllerFactoryManager {}
impl GameControllerFactoryManager {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn RegisterCustomFactoryForGipInterface<'a, Param0: ::windows::core::IntoParam<'a, ICustomGameControllerFactory>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(factory: Param0, interfaceid: Param1) -> ::windows::core::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), factory.into_param().abi(), interfaceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn RegisterCustomFactoryForHardwareId<'a, Param0: ::windows::core::IntoParam<'a, ICustomGameControllerFactory>>(factory: Param0, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), factory.into_param().abi(), hardwarevendorid, hardwareproductid).ok() })
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn RegisterCustomFactoryForXusbType<'a, Param0: ::windows::core::IntoParam<'a, ICustomGameControllerFactory>>(factory: Param0, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), factory.into_param().abi(), xusbtype, xusbsubtype).ok() })
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn TryGetFactoryControllerFromGameController<'a, Param0: ::windows::core::IntoParam<'a, ICustomGameControllerFactory>, Param1: ::windows::core::IntoParam<'a, super::IGameController>>(factory: Param0, gamecontroller: Param1) -> ::windows::core::Result<super::IGameController> {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), factory.into_param().abi(), gamecontroller.into_param().abi(), &mut result__).from_abi::<super::IGameController>(result__)
        })
    }
    pub fn IGameControllerFactoryManagerStatics<R, F: FnOnce(&IGameControllerFactoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGameControllerFactoryManagerStatics2<R, F: FnOnce(&IGameControllerFactoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl GameControllerVersionInfo {}
impl ::core::default::Default for GameControllerVersionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GameControllerVersionInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GameControllerVersionInfo").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
impl ::core::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor && self.Build == other.Build && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for GameControllerVersionInfo {}
unsafe impl ::windows::core::Abi for GameControllerVersionInfo {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)");
}
impl ::windows::core::DefaultType for GameControllerVersionInfo {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl GipFirmwareUpdateProgress {}
impl ::core::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GipFirmwareUpdateProgress {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GipFirmwareUpdateProgress").field("PercentCompleted", &self.PercentCompleted).field("CurrentComponentId", &self.CurrentComponentId).finish()
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.PercentCompleted == other.PercentCompleted && self.CurrentComponentId == other.CurrentComponentId
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateProgress {}
unsafe impl ::windows::core::Abi for GipFirmwareUpdateProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)");
}
impl ::windows::core::DefaultType for GipFirmwareUpdateProgress {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GipFirmwareUpdateResult(pub ::windows::core::IInspectable);
impl GipFirmwareUpdateResult {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn FinalComponentId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn Status(&self) -> ::windows::core::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__: GipFirmwareUpdateStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GipFirmwareUpdateStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipFirmwareUpdateResult;{6b794d32-8553-4292-8e03-e16651a2f8bc})");
}
unsafe impl ::windows::core::Interface for GipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b794d32_8553_4292_8e03_e16651a2f8bc);
}
impl ::windows::core::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
impl ::core::convert::From<GipFirmwareUpdateResult> for ::windows::core::IUnknown {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for ::windows::core::IUnknown {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GipFirmwareUpdateResult> for ::windows::core::IInspectable {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for ::windows::core::IInspectable {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GipFirmwareUpdateResult {}
unsafe impl ::core::marker::Sync for GipFirmwareUpdateResult {}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(0i32);
    pub const UpToDate: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(1i32);
    pub const Failed: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(2i32);
}
impl ::core::convert::From<i32> for GipFirmwareUpdateStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GipFirmwareUpdateStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)");
}
impl ::windows::core::DefaultType for GipFirmwareUpdateStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GipGameControllerProvider(pub ::windows::core::IInspectable);
impl GipGameControllerProvider {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), messageclass, messageid, messagebuffer.len() as u32, ::core::mem::transmute(messagebuffer.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsemessagebuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), messageclass, messageid, requestmessagebuffer.len() as u32, ::core::mem::transmute(requestmessagebuffer.as_ptr()), responsemessagebuffer.len() as u32, ::core::mem::transmute_copy(&responsemessagebuffer)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Gaming_Input_Custom`, `Foundation`, `Storage_Streams`*"]
    pub fn UpdateFirmwareAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, firmwareimage: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), firmwareimage.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GipGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipGameControllerProvider;{dbcf1e19-1af5-45a8-bf02-a0ee50c823fc})");
}
unsafe impl ::windows::core::Interface for GipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
}
impl ::windows::core::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
impl ::core::convert::From<GipGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: GipGameControllerProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &GipGameControllerProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GipGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GipGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GipGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: GipGameControllerProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &GipGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GipGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GipGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<GipGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: GipGameControllerProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GipGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &GipGameControllerProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerProvider> for GipGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerProvider> for &GipGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerProvider> {
        ::core::convert::TryInto::<IGameControllerProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GipGameControllerProvider {}
unsafe impl ::core::marker::Sync for GipGameControllerProvider {}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: GipMessageClass = GipMessageClass(0i32);
    pub const LowLatency: GipMessageClass = GipMessageClass(1i32);
    pub const StandardLatency: GipMessageClass = GipMessageClass(2i32);
}
impl ::core::convert::From<i32> for GipMessageClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GipMessageClass {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GipMessageClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)");
}
impl ::windows::core::DefaultType for GipMessageClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HidGameControllerProvider(pub ::windows::core::IInspectable);
impl HidGameControllerProvider {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), reportid, reportbuffer.len() as u32, ::core::mem::transmute_copy(&reportbuffer)).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), reportid, reportbuffer.len() as u32, ::core::mem::transmute(reportbuffer.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn SendOutputReport(&self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), reportid, reportbuffer.len() as u32, ::core::mem::transmute(reportbuffer.as_ptr())).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for HidGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.HidGameControllerProvider;{95ce3af4-abf0-4b68-a081-3b7de73ff0e7})");
}
unsafe impl ::windows::core::Interface for HidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
}
impl ::windows::core::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
impl ::core::convert::From<HidGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: HidGameControllerProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &HidGameControllerProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HidGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: HidGameControllerProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &HidGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<HidGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: HidGameControllerProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HidGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &HidGameControllerProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerProvider> for HidGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerProvider> for &HidGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerProvider> {
        ::core::convert::TryInto::<IGameControllerProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HidGameControllerProvider {}
unsafe impl ::core::marker::Sync for HidGameControllerProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct ICustomGameControllerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomGameControllerFactory {
    type Vtable = ICustomGameControllerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a0ae5e_758e_4cbe_ace6_62155fe9126f);
}
impl ICustomGameControllerFactory {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn CreateGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameControllerProvider>>(&self, provider: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnGameControllerAdded<'a, Param0: ::windows::core::IntoParam<'a, super::IGameController>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnGameControllerRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::IGameController>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{69a0ae5e-758e-4cbe-ace6-62155fe9126f}");
}
impl ::core::convert::From<ICustomGameControllerFactory> for ::windows::core::IUnknown {
    fn from(value: ICustomGameControllerFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICustomGameControllerFactory> for ::windows::core::IUnknown {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICustomGameControllerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICustomGameControllerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICustomGameControllerFactory> for ::windows::core::IInspectable {
    fn from(value: ICustomGameControllerFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICustomGameControllerFactory> for ::windows::core::IInspectable {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICustomGameControllerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICustomGameControllerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameControllerFactoryManagerStatics {
    type Vtable = IGameControllerFactoryManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36cb66e3_d0a1_4986_a24c_40b137deba9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, factory: ::windows::core::RawPtr, interfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, factory: ::windows::core::RawPtr, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, factory: ::windows::core::RawPtr, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameControllerFactoryManagerStatics2 {
    type Vtable = IGameControllerFactoryManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeace5644_19df_4115_b32a_2793e2aea3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, factory: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct IGameControllerInputSink(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameControllerInputSink {
    type Vtable = IGameControllerInputSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ff6f922_c640_4c78_a820_9a715c558bcb);
}
impl IGameControllerInputSink {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1ff6f922-c640-4c78-a820-9a715c558bcb}");
}
impl ::core::convert::From<IGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IGameControllerInputSink) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IGameControllerInputSink) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct IGameControllerProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameControllerProvider {
    type Vtable = IGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d73982_2996_4559_b16c_3e57d46e58d6);
}
impl IGameControllerProvider {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e6d73982-2996-4559-b16c-3e57d46e58d6}");
}
impl ::core::convert::From<IGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: IGameControllerProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &IGameControllerProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: IGameControllerProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &IGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b794d32_8553_4292_8e03_e16651a2f8bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GipFirmwareUpdateStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct IGipGameControllerInputSink(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGipGameControllerInputSink {
    type Vtable = IGipGameControllerInputSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2108abf_09f1_43bc_a140_80f899ec36fb);
}
impl IGipGameControllerInputSink {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp, keycode, ispressed).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), timestamp, messageclass, messageid, sequenceid, messagebuffer.len() as u32, ::core::mem::transmute(messagebuffer.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a2108abf-09f1-43bc-a140-80f899ec36fb}");
}
impl ::core::convert::From<IGipGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IGipGameControllerInputSink) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGipGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGipGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGipGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGipGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IGipGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGipGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGipGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGipGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IGipGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::core::Error;
    fn try_from(value: IGipGameControllerInputSink) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IGipGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::core::Error;
    fn try_from(value: &IGipGameControllerInputSink) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerInputSink> for IGipGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerInputSink> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerInputSink> for &IGipGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerInputSink> {
        ::core::convert::TryInto::<IGameControllerInputSink>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGipGameControllerProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, firmwareimage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct IHidGameControllerInputSink(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHidGameControllerInputSink {
    type Vtable = IHidGameControllerInputSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf754c322_182d_40e4_a126_fcee4ffa1e31);
}
impl IHidGameControllerInputSink {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp, reportid, reportbuffer.len() as u32, ::core::mem::transmute(reportbuffer.as_ptr())).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f754c322-182d-40e4-a126-fcee4ffa1e31}");
}
impl ::core::convert::From<IHidGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IHidGameControllerInputSink) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IHidGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHidGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHidGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IHidGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IHidGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHidGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IHidGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IHidGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IHidGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::core::Error;
    fn try_from(value: IHidGameControllerInputSink) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IHidGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::core::Error;
    fn try_from(value: &IHidGameControllerInputSink) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerInputSink> for IHidGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerInputSink> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerInputSink> for &IHidGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerInputSink> {
        ::core::convert::TryInto::<IGameControllerInputSink>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidGameControllerProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input_Custom`*"]
pub struct IXusbGameControllerInputSink(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXusbGameControllerInputSink {
    type Vtable = IXusbGameControllerInputSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2ac1d95_6ecb_42b3_8aab_025401ca4712);
}
impl IXusbGameControllerInputSink {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp, reportid, inputbuffer.len() as u32, ::core::mem::transmute(inputbuffer.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), timestamp).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b2ac1d95-6ecb-42b3-8aab-025401ca4712}");
}
impl ::core::convert::From<IXusbGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IXusbGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IXusbGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXusbGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IXusbGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::core::Error;
    fn try_from(value: IXusbGameControllerInputSink) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXusbGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXusbGameControllerInputSink) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerInputSink> for IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerInputSink> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerInputSink> for &IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerInputSink> {
        ::core::convert::TryInto::<IGameControllerInputSink>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e2971eb_0efb_48b4_808b_837643b2f216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: XusbDeviceSubtype = XusbDeviceSubtype(0i32);
    pub const Gamepad: XusbDeviceSubtype = XusbDeviceSubtype(1i32);
    pub const ArcadePad: XusbDeviceSubtype = XusbDeviceSubtype(2i32);
    pub const ArcadeStick: XusbDeviceSubtype = XusbDeviceSubtype(3i32);
    pub const FlightStick: XusbDeviceSubtype = XusbDeviceSubtype(4i32);
    pub const Wheel: XusbDeviceSubtype = XusbDeviceSubtype(5i32);
    pub const Guitar: XusbDeviceSubtype = XusbDeviceSubtype(6i32);
    pub const GuitarAlternate: XusbDeviceSubtype = XusbDeviceSubtype(7i32);
    pub const GuitarBass: XusbDeviceSubtype = XusbDeviceSubtype(8i32);
    pub const DrumKit: XusbDeviceSubtype = XusbDeviceSubtype(9i32);
    pub const DancePad: XusbDeviceSubtype = XusbDeviceSubtype(10i32);
}
impl ::core::convert::From<i32> for XusbDeviceSubtype {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XusbDeviceSubtype {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)");
}
impl ::windows::core::DefaultType for XusbDeviceSubtype {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: XusbDeviceType = XusbDeviceType(0i32);
    pub const Gamepad: XusbDeviceType = XusbDeviceType(1i32);
}
impl ::core::convert::From<i32> for XusbDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XusbDeviceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XusbDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)");
}
impl ::windows::core::DefaultType for XusbDeviceType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input_Custom`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XusbGameControllerProvider(pub ::windows::core::IInspectable);
impl XusbGameControllerProvider {
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), lowfrequencymotorspeed, highfrequencymotorspeed).ok() }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input_Custom`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.XusbGameControllerProvider;{6e2971eb-0efb-48b4-808b-837643b2f216})");
}
unsafe impl ::windows::core::Interface for XusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e2971eb_0efb_48b4_808b_837643b2f216);
}
impl ::windows::core::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
impl ::core::convert::From<XusbGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: XusbGameControllerProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &XusbGameControllerProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XusbGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XusbGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: XusbGameControllerProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &XusbGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XusbGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XusbGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: XusbGameControllerProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XusbGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &XusbGameControllerProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerProvider> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerProvider> for &XusbGameControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerProvider> {
        ::core::convert::TryInto::<IGameControllerProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XusbGameControllerProvider {}
unsafe impl ::core::marker::Sync for XusbGameControllerProvider {}
