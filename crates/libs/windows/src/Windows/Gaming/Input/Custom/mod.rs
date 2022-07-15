#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
pub struct GameControllerFactoryManager;
impl GameControllerFactoryManager {
    pub fn RegisterCustomFactoryForGipInterface<'a, P0, E0>(factory: P0, interfaceid: ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, ICustomGameControllerFactory>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterCustomFactoryForGipInterface)(::windows::core::Interface::as_raw(this), factory.try_into().map_err(|e| e.into())?.abi(), interfaceid).ok() })
    }
    pub fn RegisterCustomFactoryForHardwareId<'a, P0, E0>(factory: P0, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, ICustomGameControllerFactory>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterCustomFactoryForHardwareId)(::windows::core::Interface::as_raw(this), factory.try_into().map_err(|e| e.into())?.abi(), hardwarevendorid, hardwareproductid).ok() })
    }
    pub fn RegisterCustomFactoryForXusbType<'a, P0, E0>(factory: P0, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, ICustomGameControllerFactory>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterCustomFactoryForXusbType)(::windows::core::Interface::as_raw(this), factory.try_into().map_err(|e| e.into())?.abi(), xusbtype, xusbsubtype).ok() })
    }
    pub fn TryGetFactoryControllerFromGameController<'a, P0, E0, P1, E1>(factory: P0, gamecontroller: P1) -> ::windows::core::Result<super::IGameController>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, ICustomGameControllerFactory>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::IGameController>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetFactoryControllerFromGameController)(::windows::core::Interface::as_raw(this), factory.try_into().map_err(|e| e.into())?.abi(), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::IGameController>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameControllerFactoryManagerStatics<R, F: FnOnce(&IGameControllerFactoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGameControllerFactoryManagerStatics2<R, F: FnOnce(&IGameControllerFactoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for GameControllerVersionInfo {}
impl ::core::clone::Clone for GameControllerVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GameControllerVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GameControllerVersionInfo").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows::core::Abi for GameControllerVersionInfo {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GameControllerVersionInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for GameControllerVersionInfo {}
impl ::core::default::Default for GameControllerVersionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl ::core::marker::Copy for GipFirmwareUpdateProgress {}
impl ::core::clone::Clone for GipFirmwareUpdateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GipFirmwareUpdateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GipFirmwareUpdateProgress").field("PercentCompleted", &self.PercentCompleted).field("CurrentComponentId", &self.CurrentComponentId).finish()
    }
}
unsafe impl ::windows::core::Abi for GipFirmwareUpdateProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GipFirmwareUpdateProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateProgress {}
impl ::core::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(::windows::core::IUnknown);
impl GipFirmwareUpdateResult {
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedErrorCode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FinalComponentId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FinalComponentId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GipFirmwareUpdateStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for GipFirmwareUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateResult {}
impl ::core::fmt::Debug for GipFirmwareUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipFirmwareUpdateResult;{6b794d32-8553-4292-8e03-e16651a2f8bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
    const IID: ::windows::core::GUID = <IGipFirmwareUpdateResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
impl ::core::convert::From<GipFirmwareUpdateResult> for ::windows::core::IUnknown {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for ::windows::core::IUnknown {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for &::windows::core::IUnknown {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<GipFirmwareUpdateResult> for ::windows::core::IInspectable {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for ::windows::core::IInspectable {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for &::windows::core::IInspectable {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for GipFirmwareUpdateResult {}
unsafe impl ::core::marker::Sync for GipFirmwareUpdateResult {}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for GipFirmwareUpdateStatus {}
impl ::core::clone::Clone for GipFirmwareUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GipFirmwareUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GipFirmwareUpdateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GipFirmwareUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct GipGameControllerProvider(::windows::core::IUnknown);
impl GipGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendMessage)(::windows::core::Interface::as_raw(this), messageclass, messageid, messagebuffer.len() as u32, messagebuffer.as_ptr()).ok() }
    }
    pub fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[u8], responsemessagebuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendReceiveMessage)(::windows::core::Interface::as_raw(this), messageclass, messageid, requestmessagebuffer.len() as u32, requestmessagebuffer.as_ptr(), responsemessagebuffer.len() as u32, responsemessagebuffer.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UpdateFirmwareAsync<'a, P0, E0>(&self, firmwareimage: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UpdateFirmwareAsync)(::windows::core::Interface::as_raw(this), firmwareimage.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>(result__)
        }
    }
}
impl ::core::clone::Clone for GipGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GipGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GipGameControllerProvider {}
impl ::core::fmt::Debug for GipGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GipGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipGameControllerProvider;{dbcf1e19-1af5-45a8-bf02-a0ee50c823fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = <IGipGameControllerProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
impl ::core::convert::From<GipGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: GipGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &GipGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for &::windows::core::IUnknown {
    fn from(value: &GipGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<GipGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: GipGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &GipGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for &::windows::core::IInspectable {
    fn from(value: &GipGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&GipGameControllerProvider> for ::windows::core::InParam<'a, IGameControllerProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GipGameControllerProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GipGameControllerProvider {}
unsafe impl ::core::marker::Sync for GipGameControllerProvider {}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
impl ::core::marker::Copy for GipMessageClass {}
impl ::core::clone::Clone for GipMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GipMessageClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GipMessageClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for GipMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipMessageClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GipMessageClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct HidGameControllerProvider(::windows::core::IUnknown);
impl HidGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetFeatureReport)(::windows::core::Interface::as_raw(this), reportid, reportbuffer.len() as u32, reportbuffer.as_mut_ptr()).ok() }
    }
    pub fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendFeatureReport)(::windows::core::Interface::as_raw(this), reportid, reportbuffer.len() as u32, reportbuffer.as_ptr()).ok() }
    }
    pub fn SendOutputReport(&self, reportid: u8, reportbuffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendOutputReport)(::windows::core::Interface::as_raw(this), reportid, reportbuffer.len() as u32, reportbuffer.as_ptr()).ok() }
    }
}
impl ::core::clone::Clone for HidGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidGameControllerProvider {}
impl ::core::fmt::Debug for HidGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.HidGameControllerProvider;{95ce3af4-abf0-4b68-a081-3b7de73ff0e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = <IHidGameControllerProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
impl ::core::convert::From<HidGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: HidGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &HidGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for &::windows::core::IUnknown {
    fn from(value: &HidGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<HidGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: HidGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &HidGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for &::windows::core::IInspectable {
    fn from(value: &HidGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&HidGameControllerProvider> for ::windows::core::InParam<'a, IGameControllerProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HidGameControllerProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HidGameControllerProvider {}
unsafe impl ::core::marker::Sync for HidGameControllerProvider {}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct ICustomGameControllerFactory(::windows::core::IUnknown);
impl ICustomGameControllerFactory {
    pub fn CreateGameController<'a, P0, E0>(&self, provider: P0) -> ::windows::core::Result<::windows::core::IInspectable>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameControllerProvider>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateGameController)(::windows::core::Interface::as_raw(this), provider.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn OnGameControllerAdded<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnGameControllerAdded)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn OnGameControllerRemoved<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnGameControllerRemoved)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::convert::From<ICustomGameControllerFactory> for ::windows::core::IUnknown {
    fn from(value: ICustomGameControllerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICustomGameControllerFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICustomGameControllerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomGameControllerFactory> for ::windows::core::IUnknown {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICustomGameControllerFactory> for ::windows::core::IInspectable {
    fn from(value: ICustomGameControllerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICustomGameControllerFactory> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICustomGameControllerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomGameControllerFactory> for ::windows::core::IInspectable {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICustomGameControllerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomGameControllerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomGameControllerFactory {}
impl ::core::fmt::Debug for ICustomGameControllerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomGameControllerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{69a0ae5e-758e-4cbe-ace6-62155fe9126f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICustomGameControllerFactory {
    type Vtable = ICustomGameControllerFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a0ae5e_758e_4cbe_ace6_62155fe9126f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameControllerFactoryManagerStatics {
    type Vtable = IGameControllerFactoryManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36cb66e3_d0a1_4986_a24c_40b137deba9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, interfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameControllerFactoryManagerStatics2 {
    type Vtable = IGameControllerFactoryManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeace5644_19df_4115_b32a_2793e2aea3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TryGetFactoryControllerFromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct IGameControllerInputSink(::windows::core::IUnknown);
impl IGameControllerInputSink {
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameControllerInputSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameControllerInputSink> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerInputSink {}
impl ::core::fmt::Debug for IGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1ff6f922-c640-4c78-a820-9a715c558bcb}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGameControllerInputSink {
    type Vtable = IGameControllerInputSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ff6f922_c640_4c78_a820_9a715c558bcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OnInputResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT,
    pub OnInputSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct IGameControllerProvider(::windows::core::IUnknown);
impl IGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: IGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameControllerProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &IGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: IGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameControllerProvider> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &IGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerProvider {}
impl ::core::fmt::Debug for IGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e6d73982-2996-4559-b16c-3e57d46e58d6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGameControllerProvider {
    type Vtable = IGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d73982_2996_4559_b16c_3e57d46e58d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FirmwareVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HardwareVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGipFirmwareUpdateResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b794d32_8553_4292_8e03_e16651a2f8bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub FinalComponentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GipFirmwareUpdateStatus) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct IGipGameControllerInputSink(::windows::core::IUnknown);
impl IGipGameControllerInputSink {
    pub fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnKeyReceived)(::windows::core::Interface::as_raw(this), timestamp, keycode, ispressed).ok() }
    }
    pub fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnMessageReceived)(::windows::core::Interface::as_raw(this), timestamp, messageclass, messageid, sequenceid, messagebuffer.len() as u32, messagebuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IGipGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IGipGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGipGameControllerInputSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGipGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGipGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IGipGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IGipGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGipGameControllerInputSink> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IGipGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGipGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IGipGameControllerInputSink> for ::windows::core::InParam<'a, IGameControllerInputSink> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IGipGameControllerInputSink) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IGipGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGipGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGipGameControllerInputSink {}
impl ::core::fmt::Debug for IGipGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGipGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a2108abf-09f1-43bc-a140-80f899ec36fb}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGipGameControllerInputSink {
    type Vtable = IGipGameControllerInputSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2108abf_09f1_43bc_a140_80f899ec36fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OnKeyReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGipGameControllerProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT,
    pub SendReceiveMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UpdateFirmwareAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firmwareimage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UpdateFirmwareAsync: usize,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct IHidGameControllerInputSink(::windows::core::IUnknown);
impl IHidGameControllerInputSink {
    pub fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnInputReportReceived)(::windows::core::Interface::as_raw(this), timestamp, reportid, reportbuffer.len() as u32, reportbuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IHidGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IHidGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHidGameControllerInputSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHidGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHidGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IHidGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IHidGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHidGameControllerInputSink> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IHidGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHidGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IHidGameControllerInputSink> for ::windows::core::InParam<'a, IGameControllerInputSink> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IHidGameControllerInputSink) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IHidGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHidGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHidGameControllerInputSink {}
impl ::core::fmt::Debug for IHidGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHidGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f754c322-182d-40e4-a126-fcee4ffa1e31}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IHidGameControllerInputSink {
    type Vtable = IHidGameControllerInputSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf754c322_182d_40e4_a126_fcee4ffa1e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OnInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidGameControllerProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GetFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub SendFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
    pub SendOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct IXusbGameControllerInputSink(::windows::core::IUnknown);
impl IXusbGameControllerInputSink {
    pub fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnInputReceived)(::windows::core::Interface::as_raw(this), timestamp, reportid, inputbuffer.len() as u32, inputbuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IXusbGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXusbGameControllerInputSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXusbGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXusbGameControllerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IXusbGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXusbGameControllerInputSink> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IXusbGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXusbGameControllerInputSink> for ::windows::core::IInspectable {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl<'a> ::core::convert::TryFrom<&IXusbGameControllerInputSink> for ::windows::core::InParam<'a, IGameControllerInputSink> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IXusbGameControllerInputSink) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IXusbGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXusbGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXusbGameControllerInputSink {}
impl ::core::fmt::Debug for IXusbGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXusbGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b2ac1d95-6ecb-42b3-8aab-025401ca4712}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IXusbGameControllerInputSink {
    type Vtable = IXusbGameControllerInputSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2ac1d95_6ecb_42b3_8aab_025401ca4712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OnInputReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXusbGameControllerProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e2971eb_0efb_48b4_808b_837643b2f216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetVibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
impl ::core::marker::Copy for XusbDeviceSubtype {}
impl ::core::clone::Clone for XusbDeviceSubtype {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XusbDeviceSubtype {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XusbDeviceSubtype {
    type Abi = Self;
}
impl ::core::fmt::Debug for XusbDeviceSubtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceSubtype").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
impl ::core::marker::Copy for XusbDeviceType {}
impl ::core::clone::Clone for XusbDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XusbDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XusbDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for XusbDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XusbDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct XusbGameControllerProvider(::windows::core::IUnknown);
impl XusbGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVibration)(::windows::core::Interface::as_raw(this), lowfrequencymotorspeed, highfrequencymotorspeed).ok() }
    }
}
impl ::core::clone::Clone for XusbGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XusbGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XusbGameControllerProvider {}
impl ::core::fmt::Debug for XusbGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.XusbGameControllerProvider;{6e2971eb-0efb-48b4-808b-837643b2f216})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = <IXusbGameControllerProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
impl ::core::convert::From<XusbGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: XusbGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &XusbGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for &::windows::core::IUnknown {
    fn from(value: &XusbGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XusbGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: XusbGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &XusbGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for &::windows::core::IInspectable {
    fn from(value: &XusbGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&XusbGameControllerProvider> for ::windows::core::InParam<'a, IGameControllerProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &XusbGameControllerProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for XusbGameControllerProvider {}
unsafe impl ::core::marker::Sync for XusbGameControllerProvider {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
