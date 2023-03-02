#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct ICustomGameControllerFactory(::windows::core::IUnknown);
impl ICustomGameControllerFactory {
    pub fn CreateGameController<P0>(&self, provider: P0) -> ::windows::core::Result<::windows::core::IInspectable>
    where
        P0: ::windows::core::TryIntoParam<IGameControllerProvider>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).CreateGameController)(::windows::core::Interface::as_raw(this), provider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn OnGameControllerAdded<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::IGameController>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnGameControllerAdded)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn OnGameControllerRemoved<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::IGameController>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OnGameControllerRemoved)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
::windows::imp::interface_hierarchy!(ICustomGameControllerFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{69a0ae5e-758e-4cbe-ace6-62155fe9126f}");
}
unsafe impl ::windows::core::Interface for ICustomGameControllerFactory {
    type Vtable = ICustomGameControllerFactory_Vtbl;
}
impl ::core::clone::Clone for ICustomGameControllerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICustomGameControllerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a0ae5e_758e_4cbe_ace6_62155fe9126f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameControllerFactoryManagerStatics {
    type Vtable = IGameControllerFactoryManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IGameControllerFactoryManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameControllerFactoryManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36cb66e3_d0a1_4986_a24c_40b137deba9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, interfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameControllerFactoryManagerStatics2 {
    type Vtable = IGameControllerFactoryManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IGameControllerFactoryManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameControllerFactoryManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeace5644_19df_4115_b32a_2793e2aea3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
::windows::imp::interface_hierarchy!(IGameControllerInputSink, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1ff6f922-c640-4c78-a820-9a715c558bcb}");
}
unsafe impl ::windows::core::Interface for IGameControllerInputSink {
    type Vtable = IGameControllerInputSink_Vtbl;
}
impl ::core::clone::Clone for IGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameControllerInputSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ff6f922_c640_4c78_a820_9a715c558bcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IGameControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IGameControllerProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e6d73982-2996-4559-b16c-3e57d46e58d6}");
}
unsafe impl ::windows::core::Interface for IGameControllerProvider {
    type Vtable = IGameControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d73982_2996_4559_b16c_3e57d46e58d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IGipFirmwareUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGipFirmwareUpdateResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b794d32_8553_4292_8e03_e16651a2f8bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
        let this = &::windows::core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows::imp::interface_hierarchy!(IGipGameControllerInputSink, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IGameControllerInputSink> for IGipGameControllerInputSink {}
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
impl ::windows::core::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a2108abf-09f1-43bc-a140-80f899ec36fb}");
}
unsafe impl ::windows::core::Interface for IGipGameControllerInputSink {
    type Vtable = IGipGameControllerInputSink_Vtbl;
}
impl ::core::clone::Clone for IGipGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGipGameControllerInputSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2108abf_09f1_43bc_a140_80f899ec36fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OnKeyReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGipGameControllerProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IGipGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGipGameControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
        let this = &::windows::core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows::imp::interface_hierarchy!(IHidGameControllerInputSink, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IGameControllerInputSink> for IHidGameControllerInputSink {}
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
impl ::windows::core::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{f754c322-182d-40e4-a126-fcee4ffa1e31}");
}
unsafe impl ::windows::core::Interface for IHidGameControllerInputSink {
    type Vtable = IHidGameControllerInputSink_Vtbl;
}
impl ::core::clone::Clone for IHidGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHidGameControllerInputSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf754c322_182d_40e4_a126_fcee4ffa1e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OnInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidGameControllerProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IHidGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHidGameControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
        let this = &::windows::core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputResumed)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OnInputSuspended)(::windows::core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows::imp::interface_hierarchy!(IXusbGameControllerInputSink, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IGameControllerInputSink> for IXusbGameControllerInputSink {}
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
impl ::windows::core::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{b2ac1d95-6ecb-42b3-8aab-025401ca4712}");
}
unsafe impl ::windows::core::Interface for IXusbGameControllerInputSink {
    type Vtable = IXusbGameControllerInputSink_Vtbl;
}
impl ::core::clone::Clone for IXusbGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXusbGameControllerInputSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2ac1d95_6ecb_42b3_8aab_025401ca4712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OnInputReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXusbGameControllerProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IXusbGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IXusbGameControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e2971eb_0efb_48b4_808b_837643b2f216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetVibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
pub struct GameControllerFactoryManager;
impl GameControllerFactoryManager {
    pub fn RegisterCustomFactoryForGipInterface<P0>(factory: P0, interfaceid: ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterCustomFactoryForGipInterface)(::windows::core::Interface::as_raw(this), factory.try_into_param()?.abi(), interfaceid).ok() })
    }
    pub fn RegisterCustomFactoryForHardwareId<P0>(factory: P0, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterCustomFactoryForHardwareId)(::windows::core::Interface::as_raw(this), factory.try_into_param()?.abi(), hardwarevendorid, hardwareproductid).ok() })
    }
    pub fn RegisterCustomFactoryForXusbType<P0>(factory: P0, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RegisterCustomFactoryForXusbType)(::windows::core::Interface::as_raw(this), factory.try_into_param()?.abi(), xusbtype, xusbsubtype).ok() })
    }
    pub fn TryGetFactoryControllerFromGameController<P0, P1>(factory: P0, gamecontroller: P1) -> ::windows::core::Result<super::IGameController>
    where
        P0: ::windows::core::TryIntoParam<ICustomGameControllerFactory>,
        P1: ::windows::core::TryIntoParam<super::IGameController>,
    {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IGameController>();
            (::windows::core::Interface::vtable(this).TryGetFactoryControllerFromGameController)(::windows::core::Interface::as_raw(this), factory.try_into_param()?.abi(), gamecontroller.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameControllerFactoryManagerStatics<R, F: FnOnce(&IGameControllerFactoryManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGameControllerFactoryManagerStatics2<R, F: FnOnce(&IGameControllerFactoryManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(::windows::core::IUnknown);
impl GipFirmwareUpdateResult {
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ExtendedErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FinalComponentId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).FinalComponentId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GipFirmwareUpdateStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipFirmwareUpdateResult;{6b794d32-8553-4292-8e03-e16651a2f8bc})");
}
impl ::core::clone::Clone for GipFirmwareUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GipFirmwareUpdateResult {
    const IID: ::windows::core::GUID = <IGipFirmwareUpdateResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
::windows::imp::interface_hierarchy!(GipFirmwareUpdateResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GipFirmwareUpdateResult {}
unsafe impl ::core::marker::Sync for GipFirmwareUpdateResult {}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct GipGameControllerProvider(::windows::core::IUnknown);
impl GipGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
    pub fn UpdateFirmwareAsync<P0>(&self, firmwareimage: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>();
            (::windows::core::Interface::vtable(this).UpdateFirmwareAsync)(::windows::core::Interface::as_raw(this), firmwareimage.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for GipGameControllerProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipGameControllerProvider;{dbcf1e19-1af5-45a8-bf02-a0ee50c823fc})");
}
impl ::core::clone::Clone for GipGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GipGameControllerProvider {
    const IID: ::windows::core::GUID = <IGipGameControllerProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
::windows::imp::interface_hierarchy!(GipGameControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IGameControllerProvider> for GipGameControllerProvider {}
unsafe impl ::core::marker::Send for GipGameControllerProvider {}
unsafe impl ::core::marker::Sync for GipGameControllerProvider {}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct HidGameControllerProvider(::windows::core::IUnknown);
impl HidGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).UsageId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).UsagePage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
impl ::windows::core::RuntimeType for HidGameControllerProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.HidGameControllerProvider;{95ce3af4-abf0-4b68-a081-3b7de73ff0e7})");
}
impl ::core::clone::Clone for HidGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for HidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for HidGameControllerProvider {
    const IID: ::windows::core::GUID = <IHidGameControllerProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
::windows::imp::interface_hierarchy!(HidGameControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IGameControllerProvider> for HidGameControllerProvider {}
unsafe impl ::core::marker::Send for HidGameControllerProvider {}
unsafe impl ::core::marker::Sync for HidGameControllerProvider {}
#[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
#[repr(transparent)]
pub struct XusbGameControllerProvider(::windows::core::IUnknown);
impl XusbGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).FirmwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GameControllerVersionInfo>();
            (::windows::core::Interface::vtable(this).HardwareVersionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVibration)(::windows::core::Interface::as_raw(this), lowfrequencymotorspeed, highfrequencymotorspeed).ok() }
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
impl ::windows::core::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.XusbGameControllerProvider;{6e2971eb-0efb-48b4-808b-837643b2f216})");
}
impl ::core::clone::Clone for XusbGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for XusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for XusbGameControllerProvider {
    const IID: ::windows::core::GUID = <IXusbGameControllerProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
::windows::imp::interface_hierarchy!(XusbGameControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IGameControllerProvider> for XusbGameControllerProvider {}
unsafe impl ::core::marker::Send for XusbGameControllerProvider {}
unsafe impl ::core::marker::Sync for XusbGameControllerProvider {}
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
impl ::windows::core::TypeKind for GipFirmwareUpdateStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GipFirmwareUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)");
}
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
impl ::windows::core::TypeKind for GipMessageClass {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GipMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipMessageClass").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GipMessageClass {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)");
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
impl ::windows::core::TypeKind for XusbDeviceSubtype {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XusbDeviceSubtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceSubtype").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)");
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
impl ::windows::core::TypeKind for XusbDeviceType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XusbDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for XusbDeviceType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)");
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
impl ::windows::core::TypeKind for GameControllerVersionInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)");
}
impl ::core::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor && self.Build == other.Build && self.Revision == other.Revision
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
impl ::windows::core::TypeKind for GipFirmwareUpdateProgress {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)");
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.PercentCompleted == other.PercentCompleted && self.CurrentComponentId == other.CurrentComponentId
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateProgress {}
impl ::core::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
