#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub struct GameControllerFactoryManager {}
impl GameControllerFactoryManager {
    pub fn RegisterCustomFactoryForGipInterface<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ICustomGameControllerFactory>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        factory: Param0,
        interfaceid: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                factory.into_param().abi(),
                interfaceid.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn RegisterCustomFactoryForHardwareId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ICustomGameControllerFactory>,
    >(
        factory: Param0,
        hardwarevendorid: u16,
        hardwareproductid: u16,
    ) -> ::windows::runtime::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                factory.into_param().abi(),
                hardwarevendorid,
                hardwareproductid,
            )
            .ok()
        })
    }
    pub fn RegisterCustomFactoryForXusbType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ICustomGameControllerFactory>,
    >(
        factory: Param0,
        xusbtype: XusbDeviceType,
        xusbsubtype: XusbDeviceSubtype,
    ) -> ::windows::runtime::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                factory.into_param().abi(),
                xusbtype,
                xusbsubtype,
            )
            .ok()
        })
    }
    pub fn TryGetFactoryControllerFromGameController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ICustomGameControllerFactory>,
        Param1: ::windows::runtime::IntoParam<'a, super::IGameController>,
    >(
        factory: Param0,
        gamecontroller: Param1,
    ) -> ::windows::runtime::Result<super::IGameController> {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                factory.into_param().abi(),
                gamecontroller.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::IGameController>(result__)
        })
    }
    pub fn IGameControllerFactoryManagerStatics<
        R,
        F: FnOnce(&IGameControllerFactoryManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GameControllerFactoryManager,
            IGameControllerFactoryManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGameControllerFactoryManagerStatics2<
        R,
        F: FnOnce(&IGameControllerFactoryManagerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GameControllerFactoryManager,
            IGameControllerFactoryManagerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl GameControllerVersionInfo {}
impl ::std::default::Default for GameControllerVersionInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GameControllerVersionInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GameControllerVersionInfo")
            .field("Major", &self.Major)
            .field("Minor", &self.Minor)
            .field("Build", &self.Build)
            .field("Revision", &self.Revision)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major
            && self.Minor == other.Minor
            && self.Build == other.Build
            && self.Revision == other.Revision
    }
}
impl ::std::cmp::Eq for GameControllerVersionInfo {}
unsafe impl ::windows::runtime::Abi for GameControllerVersionInfo {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)",
    );
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl GipFirmwareUpdateProgress {}
impl ::std::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GipFirmwareUpdateProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GipFirmwareUpdateProgress")
            .field("PercentCompleted", &self.PercentCompleted)
            .field("CurrentComponentId", &self.CurrentComponentId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.PercentCompleted == other.PercentCompleted
            && self.CurrentComponentId == other.CurrentComponentId
    }
}
impl ::std::cmp::Eq for GipFirmwareUpdateProgress {}
unsafe impl ::windows::runtime::Abi for GipFirmwareUpdateProgress {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GipFirmwareUpdateResult(::windows::runtime::IInspectable);
impl GipFirmwareUpdateResult {
    pub fn ExtendedErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn FinalComponentId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::runtime::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__: GipFirmwareUpdateStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GipFirmwareUpdateStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Gaming.Input.Custom.GipFirmwareUpdateResult;{6b794d32-8553-4292-8e03-e16651a2f8bc})" ) ;
}
unsafe impl ::windows::runtime::Interface for GipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1803111730,
        34131,
        17042,
        [142, 3, 225, 102, 81, 162, 248, 188],
    );
}
impl ::windows::runtime::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
impl ::std::convert::From<GipFirmwareUpdateResult> for ::windows::runtime::IUnknown {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GipFirmwareUpdateResult> for ::windows::runtime::IUnknown {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GipFirmwareUpdateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &GipFirmwareUpdateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GipFirmwareUpdateResult> for ::windows::runtime::IInspectable {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GipFirmwareUpdateResult> for ::windows::runtime::IInspectable {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GipFirmwareUpdateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GipFirmwareUpdateResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GipFirmwareUpdateResult {}
unsafe impl ::std::marker::Sync for GipFirmwareUpdateResult {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(0i32);
    pub const UpToDate: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(1i32);
    pub const Failed: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(2i32);
}
impl ::std::convert::From<i32> for GipFirmwareUpdateStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GipFirmwareUpdateStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GipGameControllerProvider(::windows::runtime::IInspectable);
impl GipGameControllerProvider {
    pub fn SendMessage(
        &self,
        messageclass: GipMessageClass,
        messageid: u8,
        messagebuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                messageclass,
                messageid,
                messagebuffer.len() as u32,
                ::std::mem::transmute(messagebuffer.as_ptr()),
            )
            .ok()
        }
    }
    pub fn SendReceiveMessage(
        &self,
        messageclass: GipMessageClass,
        messageid: u8,
        requestmessagebuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
        responsemessagebuffer: &mut [<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                messageclass,
                messageid,
                requestmessagebuffer.len() as u32,
                ::std::mem::transmute(requestmessagebuffer.as_ptr()),
                responsemessagebuffer.len() as u32,
                ::std::mem::transmute_copy(&responsemessagebuffer),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UpdateFirmwareAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>,
    >(
        &self,
        firmwareimage: Param0,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IAsyncOperationWithProgress<
            GipFirmwareUpdateResult,
            GipFirmwareUpdateProgress,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                firmwareimage.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<
                GipFirmwareUpdateResult,
                GipFirmwareUpdateProgress,
            >>(result__)
        }
    }
    pub fn FirmwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GipGameControllerProvider {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Gaming.Input.Custom.GipGameControllerProvider;{dbcf1e19-1af5-45a8-bf02-a0ee50c823fc})" ) ;
}
unsafe impl ::windows::runtime::Interface for GipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3687783961,
        6901,
        17832,
        [191, 2, 160, 238, 80, 200, 35, 252],
    );
}
impl ::windows::runtime::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
impl ::std::convert::From<GipGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: GipGameControllerProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GipGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: &GipGameControllerProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GipGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &GipGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<GipGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: GipGameControllerProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GipGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: &GipGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GipGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GipGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<GipGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GipGameControllerProvider) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&GipGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GipGameControllerProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerProvider> for GipGameControllerProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerProvider> for &GipGameControllerProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerProvider> {
        ::std::convert::TryInto::<IGameControllerProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for GipGameControllerProvider {}
unsafe impl ::std::marker::Sync for GipGameControllerProvider {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: GipMessageClass = GipMessageClass(0i32);
    pub const LowLatency: GipMessageClass = GipMessageClass(1i32);
    pub const StandardLatency: GipMessageClass = GipMessageClass(2i32);
}
impl ::std::convert::From<i32> for GipMessageClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GipMessageClass {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GipMessageClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HidGameControllerProvider(::windows::runtime::IInspectable);
impl HidGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn GetFeatureReport(
        &self,
        reportid: u8,
        reportbuffer: &mut [<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                reportid,
                reportbuffer.len() as u32,
                ::std::mem::transmute_copy(&reportbuffer),
            )
            .ok()
        }
    }
    pub fn SendFeatureReport(
        &self,
        reportid: u8,
        reportbuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                reportid,
                reportbuffer.len() as u32,
                ::std::mem::transmute(reportbuffer.as_ptr()),
            )
            .ok()
        }
    }
    pub fn SendOutputReport(
        &self,
        reportid: u8,
        reportbuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                reportid,
                reportbuffer.len() as u32,
                ::std::mem::transmute(reportbuffer.as_ptr()),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidGameControllerProvider {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Gaming.Input.Custom.HidGameControllerProvider;{95ce3af4-abf0-4b68-a081-3b7de73ff0e7})" ) ;
}
unsafe impl ::windows::runtime::Interface for HidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2513320692,
        44016,
        19304,
        [160, 129, 59, 125, 231, 63, 240, 231],
    );
}
impl ::windows::runtime::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
impl ::std::convert::From<HidGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: HidGameControllerProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: &HidGameControllerProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HidGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HidGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HidGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: HidGameControllerProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: &HidGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HidGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HidGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<HidGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HidGameControllerProvider) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&HidGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HidGameControllerProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerProvider> for HidGameControllerProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerProvider> for &HidGameControllerProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerProvider> {
        ::std::convert::TryInto::<IGameControllerProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HidGameControllerProvider {}
unsafe impl ::std::marker::Sync for HidGameControllerProvider {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICustomGameControllerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomGameControllerFactory {
    type Vtable = ICustomGameControllerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1772138078,
        30094,
        19646,
        [172, 230, 98, 21, 95, 233, 18, 111],
    );
}
impl ICustomGameControllerFactory {
    pub fn CreateGameController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IGameControllerProvider>,
    >(
        &self,
        provider: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                provider.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn OnGameControllerAdded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::IGameController>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGameControllerRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::IGameController>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{69a0ae5e-758e-4cbe-ace6-62155fe9126f}");
}
impl ::std::convert::From<ICustomGameControllerFactory> for ::windows::runtime::IUnknown {
    fn from(value: ICustomGameControllerFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICustomGameControllerFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICustomGameControllerFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICustomGameControllerFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICustomGameControllerFactory> for ::windows::runtime::IInspectable {
    fn from(value: ICustomGameControllerFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICustomGameControllerFactory> for ::windows::runtime::IInspectable {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ICustomGameControllerFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ICustomGameControllerFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        provider: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameControllerFactoryManagerStatics {
    type Vtable = IGameControllerFactoryManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        919299811,
        53409,
        18822,
        [162, 76, 64, 177, 55, 222, 186, 158],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        factory: ::windows::runtime::RawPtr,
        interfaceid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        factory: ::windows::runtime::RawPtr,
        hardwarevendorid: u16,
        hardwareproductid: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        factory: ::windows::runtime::RawPtr,
        xusbtype: XusbDeviceType,
        xusbsubtype: XusbDeviceSubtype,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameControllerFactoryManagerStatics2 {
    type Vtable = IGameControllerFactoryManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3939391044,
        6623,
        16661,
        [179, 42, 39, 147, 226, 174, 163, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        factory: ::windows::runtime::RawPtr,
        gamecontroller: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGameControllerInputSink(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameControllerInputSink {
    type Vtable = IGameControllerInputSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        536279330,
        50752,
        19576,
        [168, 32, 154, 113, 92, 85, 139, 203],
    );
}
impl IGameControllerInputSink {
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{1ff6f922-c640-4c78-a820-9a715c558bcb}");
}
impl ::std::convert::From<IGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: IGameControllerInputSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: &IGameControllerInputSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: IGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: &IGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGameControllerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameControllerProvider {
    type Vtable = IGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3872864642,
        10646,
        17753,
        [177, 108, 62, 87, 212, 110, 88, 214],
    );
}
impl IGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGameControllerProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{e6d73982-2996-4559-b16c-3e57d46e58d6}");
}
impl ::std::convert::From<IGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: IGameControllerProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IGameControllerProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: IGameControllerProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut GameControllerVersionInfo,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut GameControllerVersionInfo,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1803111730,
        34131,
        17042,
        [142, 3, 225, 102, 81, 162, 248, 188],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut GipFirmwareUpdateStatus,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGipGameControllerInputSink(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGipGameControllerInputSink {
    type Vtable = IGipGameControllerInputSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2718993087,
        2545,
        17340,
        [161, 64, 128, 248, 153, 236, 54, 251],
    );
}
impl IGipGameControllerInputSink {
    pub fn OnKeyReceived(
        &self,
        timestamp: u64,
        keycode: u8,
        ispressed: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
                keycode,
                ispressed,
            )
            .ok()
        }
    }
    pub fn OnMessageReceived(
        &self,
        timestamp: u64,
        messageclass: GipMessageClass,
        messageid: u8,
        sequenceid: u8,
        messagebuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                timestamp,
                messageclass,
                messageid,
                sequenceid,
                messagebuffer.len() as u32,
                ::std::mem::transmute(messagebuffer.as_ptr()),
            )
            .ok()
        }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{a2108abf-09f1-43bc-a140-80f899ec36fb}");
}
impl ::std::convert::From<IGipGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: IGipGameControllerInputSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGipGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IGipGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IGipGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IGipGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: IGipGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGipGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IGipGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IGipGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IGipGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IGipGameControllerInputSink) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IGipGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IGipGameControllerInputSink) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerInputSink>
    for IGipGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerInputSink> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerInputSink>
    for &IGipGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerInputSink> {
        ::std::convert::TryInto::<IGameControllerInputSink>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u64,
        keycode: u8,
        ispressed: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u64,
        messageclass: GipMessageClass,
        messageid: u8,
        sequenceid: u8,
        messageBuffer_array_size: u32,
        messagebuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IGipGameControllerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3687783961,
        6901,
        17832,
        [191, 2, 160, 238, 80, 200, 35, 252],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        messageclass: GipMessageClass,
        messageid: u8,
        messageBuffer_array_size: u32,
        messagebuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        messageclass: GipMessageClass,
        messageid: u8,
        requestMessageBuffer_array_size: u32,
        requestmessagebuffer: *const u8,
        responseMessageBuffer_array_size: u32,
        responsemessagebuffer: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        firmwareimage: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IHidGameControllerInputSink(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidGameControllerInputSink {
    type Vtable = IHidGameControllerInputSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4149527330,
        6189,
        16612,
        [161, 38, 252, 238, 79, 250, 30, 49],
    );
}
impl IHidGameControllerInputSink {
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
    pub fn OnInputReportReceived(
        &self,
        timestamp: u64,
        reportid: u8,
        reportbuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
                reportid,
                reportbuffer.len() as u32,
                ::std::mem::transmute(reportbuffer.as_ptr()),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{f754c322-182d-40e4-a126-fcee4ffa1e31}");
}
impl ::std::convert::From<IHidGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: IHidGameControllerInputSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IHidGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IHidGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IHidGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IHidGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: IHidGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IHidGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IHidGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IHidGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IHidGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IHidGameControllerInputSink) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IHidGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IHidGameControllerInputSink) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerInputSink>
    for IHidGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerInputSink> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerInputSink>
    for &IHidGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerInputSink> {
        ::std::convert::TryInto::<IGameControllerInputSink>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u64,
        reportid: u8,
        reportBuffer_array_size: u32,
        reportbuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHidGameControllerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2513320692,
        44016,
        19304,
        [160, 129, 59, 125, 231, 63, 240, 231],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reportid: u8,
        reportBuffer_array_size: u32,
        reportbuffer: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reportid: u8,
        reportBuffer_array_size: u32,
        reportbuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reportid: u8,
        reportBuffer_array_size: u32,
        reportbuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXusbGameControllerInputSink(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXusbGameControllerInputSink {
    type Vtable = IXusbGameControllerInputSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2997624213,
        28363,
        17075,
        [138, 171, 2, 84, 1, 202, 71, 18],
    );
}
impl IXusbGameControllerInputSink {
    pub fn OnInputReceived(
        &self,
        timestamp: u64,
        reportid: u8,
        inputbuffer: &[<u8 as ::windows::runtime::Abi>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
                reportid,
                inputbuffer.len() as u32,
                ::std::mem::transmute(inputbuffer.as_ptr()),
            )
            .ok()
        }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                timestamp,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{b2ac1d95-6ecb-42b3-8aab-025401ca4712}");
}
impl ::std::convert::From<IXusbGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXusbGameControllerInputSink> for ::windows::runtime::IUnknown {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IXusbGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IXusbGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXusbGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IXusbGameControllerInputSink> for ::windows::runtime::IInspectable {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IXusbGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IXusbGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IXusbGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IXusbGameControllerInputSink) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IXusbGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IXusbGameControllerInputSink) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerInputSink>
    for IXusbGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerInputSink> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerInputSink>
    for &IXusbGameControllerInputSink
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerInputSink> {
        ::std::convert::TryInto::<IGameControllerInputSink>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: u64,
        reportid: u8,
        inputBuffer_array_size: u32,
        inputbuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1848209899,
        3835,
        18612,
        [128, 139, 131, 118, 67, 178, 242, 22],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lowfrequencymotorspeed: f64,
        highfrequencymotorspeed: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for XusbDeviceSubtype {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XusbDeviceSubtype {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: XusbDeviceType = XusbDeviceType(0i32);
    pub const Gamepad: XusbDeviceType = XusbDeviceType(1i32);
}
impl ::std::convert::From<i32> for XusbDeviceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XusbDeviceType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XusbDeviceType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct XusbGameControllerProvider(::windows::runtime::IInspectable);
impl XusbGameControllerProvider {
    pub fn SetVibration(
        &self,
        lowfrequencymotorspeed: f64,
        highfrequencymotorspeed: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                lowfrequencymotorspeed,
                highfrequencymotorspeed,
            )
            .ok()
        }
    }
    pub fn FirmwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::runtime::Result<GameControllerVersionInfo> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: GameControllerVersionInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Gaming.Input.Custom.XusbGameControllerProvider;{6e2971eb-0efb-48b4-808b-837643b2f216})" ) ;
}
unsafe impl ::windows::runtime::Interface for XusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1848209899,
        3835,
        18612,
        [128, 139, 131, 118, 67, 178, 242, 22],
    );
}
impl ::windows::runtime::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
impl ::std::convert::From<XusbGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: XusbGameControllerProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XusbGameControllerProvider> for ::windows::runtime::IUnknown {
    fn from(value: &XusbGameControllerProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for XusbGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &XusbGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<XusbGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: XusbGameControllerProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XusbGameControllerProvider> for ::windows::runtime::IInspectable {
    fn from(value: &XusbGameControllerProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for XusbGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a XusbGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<XusbGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XusbGameControllerProvider) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&XusbGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XusbGameControllerProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerProvider> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerProvider>
    for &XusbGameControllerProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerProvider> {
        ::std::convert::TryInto::<IGameControllerProvider>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for XusbGameControllerProvider {}
unsafe impl ::std::marker::Sync for XusbGameControllerProvider {}
