#[doc(hidden)]
#[repr(transparent)]
pub struct IControlChannelTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IControlChannelTrigger {
    type Vtable = IControlChannelTrigger_Vtbl;
}
impl ::core::clone::Clone for IControlChannelTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlChannelTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1431a7_ee96_40e8_a199_8703cd969ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ControlChannelTriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServerKeepAliveIntervalInMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetServerKeepAliveIntervalInMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CurrentKeepAliveIntervalInMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TransportObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")]
    pub KeepAliveTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    KeepAliveTrigger: usize,
    #[cfg(feature = "ApplicationModel_Background")]
    pub PushNotificationTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    PushNotificationTrigger: usize,
    pub UsingTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForPushEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerStatus) -> ::windows::core::HRESULT,
    pub DecreaseNetworkKeepAliveInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FlushTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IControlChannelTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IControlChannelTrigger2 {
    type Vtable = IControlChannelTrigger2_Vtbl;
}
impl ::core::clone::Clone for IControlChannelTrigger2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlChannelTrigger2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf00d237_51be_4514_9725_3556e1879580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTrigger2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IControlChannelTriggerEventDetails(::windows::core::IUnknown);
impl IControlChannelTriggerEventDetails {
    pub fn ControlChannelTrigger(&self) -> ::windows::core::Result<ControlChannelTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ControlChannelTrigger>();
            (::windows::core::Interface::vtable(this).ControlChannelTrigger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IControlChannelTriggerEventDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IControlChannelTriggerEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChannelTriggerEventDetails {}
impl ::core::fmt::Debug for IControlChannelTriggerEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChannelTriggerEventDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IControlChannelTriggerEventDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1b36e047-89bb-4236-96ac-71d012bb4869}");
}
unsafe impl ::windows::core::Interface for IControlChannelTriggerEventDetails {
    type Vtable = IControlChannelTriggerEventDetails_Vtbl;
}
impl ::core::clone::Clone for IControlChannelTriggerEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlChannelTriggerEventDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b36e047_89bb_4236_96ac_71d012bb4869);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerEventDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ControlChannelTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IControlChannelTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IControlChannelTriggerFactory {
    type Vtable = IControlChannelTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IControlChannelTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlChannelTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda4b7cf0_8d71_446f_88c3_b95184a2d6cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateControlChannelTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateControlChannelTriggerEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IControlChannelTriggerResetEventDetails(::windows::core::IUnknown);
impl IControlChannelTriggerResetEventDetails {
    pub fn ResetReason(&self) -> ::windows::core::Result<ControlChannelTriggerResetReason> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ControlChannelTriggerResetReason>();
            (::windows::core::Interface::vtable(this).ResetReason)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareSlotReset(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HardwareSlotReset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SoftwareSlotReset(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SoftwareSlotReset)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IControlChannelTriggerResetEventDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IControlChannelTriggerResetEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChannelTriggerResetEventDetails {}
impl ::core::fmt::Debug for IControlChannelTriggerResetEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChannelTriggerResetEventDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IControlChannelTriggerResetEventDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6851038e-8ec4-42fe-9bb2-21e91b7bfcb1}");
}
unsafe impl ::windows::core::Interface for IControlChannelTriggerResetEventDetails {
    type Vtable = IControlChannelTriggerResetEventDetails_Vtbl;
}
impl ::core::clone::Clone for IControlChannelTriggerResetEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlChannelTriggerResetEventDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6851038e_8ec4_42fe_9bb2_21e91b7bfcb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChannelTriggerResetEventDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResetReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> ::windows::core::HRESULT,
    pub HardwareSlotReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SoftwareSlotReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocket {
    type Vtable = IDatagramSocket_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe25bbb_c3bc_4677_8446_ca28a465a3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithEndpointPairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithEndpointPairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BindServiceNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindServiceNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BindEndpointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindEndpointAsync: usize,
    pub JoinMulticastGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetOutputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetOutputStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetOutputStreamWithEndpointPairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetOutputStreamWithEndpointPairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocket2 {
    type Vtable = IDatagramSocket2_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocket2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocket2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd83ba354_9a9d_4185_a20a_1424c9c2a7cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocket2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub BindServiceNameAndAdapterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    BindServiceNameAndAdapterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocket3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocket3 {
    type Vtable = IDatagramSocket3_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocket3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocket3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37544f09_ab92_4306_9ac1_0c381283d9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocket3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CancelIOAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelIOAsync: usize,
    pub EnableTransferOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransferOwnershipWithContextAndKeepAliveTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransferOwnershipWithContextAndKeepAliveTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketControl {
    type Vtable = IDatagramSocketControl_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocketControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52ac3f2e_349a_4135_bb58_b79b2647d390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub QualityOfService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketControl2 {
    type Vtable = IDatagramSocketControl2_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocketControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocketControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ead5c2_979c_4415_82a1_3cfaf646c192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetInboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DontFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDontFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketControl3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketControl3 {
    type Vtable = IDatagramSocketControl3_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocketControl3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocketControl3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4eb8256_1f6d_4598_9b57_d42a001df349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketControl3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MulticastOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMulticastOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketInformation {
    type Vtable = IDatagramSocketInformation_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocketInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f1a569a_55fb_48cd_9706_7a974f7b1585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketMessageReceivedEventArgs {
    type Vtable = IDatagramSocketMessageReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocketMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d107e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataReader: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDatagramSocketStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDatagramSocketStatics {
    type Vtable = IDatagramSocketStatics_Vtbl;
}
impl ::core::clone::Clone for IDatagramSocketStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDatagramSocketStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9c62aee_1494_4a21_bb7e_8589fc751d9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDatagramSocketStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsWithSortOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsWithSortOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocket {
    type Vtable = IMessageWebSocket_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33727d08_34d5_4746_ad7b_8dde5bc2ef88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocket2 {
    type Vtable = IMessageWebSocket2_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocket2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocket2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed0cee7_f9c8_440a_9ad5_737281d9742e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocket2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocket3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocket3 {
    type Vtable = IMessageWebSocket3_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocket3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocket3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59d9defb_71af_4349_8487_911fcf681597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocket3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendNonfinalFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendNonfinalFrameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendFinalFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendFinalFrameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketControl {
    type Vtable = IMessageWebSocketControl_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocketControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8118388a_c629_4f0a_80fb_81fc05538862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxMessageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxMessageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketControl2 {
    type Vtable = IMessageWebSocketControl2_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocketControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocketControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe30fd791_080c_400a_a712_27dfa9e744d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ActualUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualUnsolicitedPongInterval: usize,
    pub ReceiveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MessageWebSocketReceiveMode) -> ::windows::core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MessageWebSocketReceiveMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketMessageReceivedEventArgs {
    type Vtable = IMessageWebSocketMessageReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocketMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x478c22ac_4c4b_42ed_9ed7_1ef9f94fa3d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataReader: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageWebSocketMessageReceivedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageWebSocketMessageReceivedEventArgs2 {
    type Vtable = IMessageWebSocketMessageReceivedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IMessageWebSocketMessageReceivedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageWebSocketMessageReceivedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89ce06fd_dd6f_4a07_87f9_f9eb4d89d83d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageWebSocketMessageReceivedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsMessageComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerMessageWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerMessageWebSocket {
    type Vtable = IServerMessageWebSocket_Vtbl;
}
impl ::core::clone::Clone for IServerMessageWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IServerMessageWebSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3ac9240_813b_5efd_7e11_ae2305fc77f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerMessageWebSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CloseWithStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: u16, reason: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerMessageWebSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerMessageWebSocketControl {
    type Vtable = IServerMessageWebSocketControl_Vtbl;
}
impl ::core::clone::Clone for IServerMessageWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IServerMessageWebSocketControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c2f051_1c1f_587a_4519_2181610192b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerMessageWebSocketControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerMessageWebSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerMessageWebSocketInformation {
    type Vtable = IServerMessageWebSocketInformation_Vtbl;
}
impl ::core::clone::Clone for IServerMessageWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IServerMessageWebSocketInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc32b45f_4448_5505_6cc9_09afa8915f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerMessageWebSocketInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerStreamWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerStreamWebSocket {
    type Vtable = IServerStreamWebSocket_Vtbl;
}
impl ::core::clone::Clone for IServerStreamWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IServerStreamWebSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ced5bbf_74f6_55e4_79df_9132680dfee8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerStreamWebSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CloseWithStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: u16, reason: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServerStreamWebSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IServerStreamWebSocketInformation {
    type Vtable = IServerStreamWebSocketInformation_Vtbl;
}
impl ::core::clone::Clone for IServerStreamWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IServerStreamWebSocketInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc32b45f_4448_5505_6cc9_09aba8915f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerStreamWebSocketInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityContext {
    type Vtable = ISocketActivityContext_Vtbl;
}
impl ::core::clone::Clone for ISocketActivityContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketActivityContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43b04d64_4c85_4396_a637_1d973f6ebd49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityContextFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityContextFactory {
    type Vtable = ISocketActivityContextFactory_Vtbl;
}
impl ::core::clone::Clone for ISocketActivityContextFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketActivityContextFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb99fc3c3_088c_4388_83ae_2525138e049a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityContextFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityInformation {
    type Vtable = ISocketActivityInformation_Vtbl;
}
impl ::core::clone::Clone for ISocketActivityInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketActivityInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d8a42e4_a87e_4b74_9968_185b2511defe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityKind) -> ::windows::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DatagramSocket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StreamSocket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StreamSocketListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityInformationStatics {
    type Vtable = ISocketActivityInformationStatics_Vtbl;
}
impl ::core::clone::Clone for ISocketActivityInformationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketActivityInformationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8570b47a_7e7d_4736_8041_1327a6543c56);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllSockets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllSockets: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityTriggerDetails {
    type Vtable = ISocketActivityTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ISocketActivityTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketActivityTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45f406a7_fc9f_4f81_acad_355fef51e67b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityTriggerReason) -> ::windows::core::HRESULT,
    pub SocketInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketErrorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketErrorStatics {
    type Vtable = ISocketErrorStatics_Vtbl;
}
impl ::core::clone::Clone for ISocketErrorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketErrorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x828337f4_7d56_4d8e_b7b4_a07dd7c1bca9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketErrorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SocketErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocket {
    type Vtable = IStreamSocket_Vtbl;
}
impl ::core::clone::Clone for IStreamSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a22cf3_fc7b_4857_af38_f6e7de6a5b49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithEndpointPairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithEndpointPairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithEndpointPairAndProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointpair: *mut ::core::ffi::c_void, protectionlevel: SocketProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithEndpointPairAndProtectionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectWithProtectionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpgradeToSslAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectionlevel: SocketProtectionLevel, validationhostname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpgradeToSslAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocket2 {
    type Vtable = IStreamSocket2_Vtbl;
}
impl ::core::clone::Clone for IStreamSocket2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocket2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29d0e575_f314_4d09_adf0_0fbd967fbd9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocket2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub ConnectWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    ConnectWithProtectionLevelAndAdapterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocket3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocket3 {
    type Vtable = IStreamSocket3_Vtbl;
}
impl ::core::clone::Clone for IStreamSocket3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocket3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f430b00_9d28_4854_bac3_2301941ec223);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocket3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CancelIOAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelIOAsync: usize,
    pub EnableTransferOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransferOwnershipWithContextAndKeepAliveTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransferOwnershipWithContextAndKeepAliveTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl {
    type Vtable = IStreamSocketControl_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe25adf1_92ab_4af3_9992_0f4c85e36cc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NoDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub KeepAlive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKeepAlive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub QualityOfService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl2 {
    type Vtable = IStreamSocketControl2_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2d09a56_060f_44c1_b8e2_1fbf60bd62c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl3 {
    type Vtable = IStreamSocketControl3_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketControl3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketControl3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc56a444c_4e74_403e_894c_b31cae5c7342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SerializeConnectionAttempts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSerializeConnectionAttempts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketControl4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketControl4 {
    type Vtable = IStreamSocketControl4_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketControl4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketControl4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x964e2b3d_ec27_4888_b3ce_c74b418423ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketControl4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MinProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT,
    pub SetMinProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketProtectionLevel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketInformation {
    type Vtable = IStreamSocketInformation_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b80ae30_5e68_4205_88f0_dc85d2e25ded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoundTripTimeStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundTripTimeStatistics) -> ::windows::core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketInformation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketInformation2 {
    type Vtable = IStreamSocketInformation2_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketInformation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12c28452_4bdc_4ee4_976a_cf130e9d92e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketInformation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListener {
    type Vtable = IStreamSocketListener_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListener {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff513437_df9f_4df0_bf82_0ec5d7b35aae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListener_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BindServiceNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindServiceNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BindEndpointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindEndpointAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListener2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListener2 {
    type Vtable = IStreamSocketListener2_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListener2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListener2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x658dc13e_bb3e_4458_b232_ed1088694b98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListener2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BindServiceNameWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BindServiceNameWithProtectionLevelAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub BindServiceNameWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity")))]
    BindServiceNameWithProtectionLevelAndAdapterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListener3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListener3 {
    type Vtable = IStreamSocketListener3_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListener3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListener3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4798201c_bdf8_4919_8542_28d450e74507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListener3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CancelIOAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelIOAsync: usize,
    pub EnableTransferOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerConnectionReceivedEventArgs {
    type Vtable = IStreamSocketListenerConnectionReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListenerConnectionReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListenerConnectionReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c472ea9_373f_447b_85b1_ddd4548803ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Socket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerControl {
    type Vtable = IStreamSocketListenerControl_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListenerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListenerControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20d8c576_8d8a_4dba_9722_a16c4d984980);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub QualityOfService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerControl2 {
    type Vtable = IStreamSocketListenerControl2_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListenerControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListenerControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x948bb665_2c3e_404b_b8b0_8eb249a2b0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NoDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub KeepAlive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKeepAlive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketListenerInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketListenerInformation {
    type Vtable = IStreamSocketListenerInformation_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketListenerInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketListenerInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe62ba82f_a63a_430b_bf62_29e93e5633b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketListenerInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamSocketStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamSocketStatics {
    type Vtable = IStreamSocketStatics_Vtbl;
}
impl ::core::clone::Clone for IStreamSocketStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamSocketStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa420bc4a_6e2e_4af5_b556_355ae0cd4f29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamSocketStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsWithSortOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remoteservicename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsWithSortOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocket {
    type Vtable = IStreamWebSocket_Vtbl;
}
impl ::core::clone::Clone for IStreamWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamWebSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4a49d8_b289_45bb_97eb_c7525205a843);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocket2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocket2 {
    type Vtable = IStreamWebSocket2_Vtbl;
}
impl ::core::clone::Clone for IStreamWebSocket2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamWebSocket2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa4d08cb_93f5_4678_8236_57cce5417ed5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocket2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerCustomValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerCustomValidationRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocketControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocketControl {
    type Vtable = IStreamWebSocketControl_Vtbl;
}
impl ::core::clone::Clone for IStreamWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamWebSocketControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4f478b1_a45a_48db_953a_645b7d964c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocketControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NoDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStreamWebSocketControl2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStreamWebSocketControl2 {
    type Vtable = IStreamWebSocketControl2_Vtbl;
}
impl ::core::clone::Clone for IStreamWebSocketControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStreamWebSocketControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x215d9f7e_fa58_40da_9f11_a48dafe95037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamWebSocketControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredUnsolicitedPongInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ActualUnsolicitedPongInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualUnsolicitedPongInterval: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IWebSocket(::windows::core::IUnknown);
impl IWebSocket {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestHeader)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(headername), ::core::mem::transmute_copy(headervalue)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, eventhandler: &super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CloseWithStatus)(::windows::core::Interface::as_raw(this), code, ::core::mem::transmute_copy(reason)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IWebSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl windows::core::CanTryInto<super::super::Foundation::IClosable> for IWebSocket {}
impl ::core::cmp::PartialEq for IWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocket {}
impl ::core::fmt::Debug for IWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{f877396f-99b1-4e18-bc08-850c9adf156e}");
}
unsafe impl ::windows::core::Interface for IWebSocket {
    type Vtable = IWebSocket_Vtbl;
}
impl ::core::clone::Clone for IWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf877396f_99b1_4e18_bc08_850c9adf156e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::std::mem::MaybeUninit<::windows::core::HSTRING>, headervalue: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CloseWithStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: u16, reason: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebSocketClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebSocketClosedEventArgs {
    type Vtable = IWebSocketClosedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebSocketClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketClosedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb78d07_d0a8_4703_a091_c8c2c0915bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketClosedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IWebSocketControl(::windows::core::IUnknown);
impl IWebSocketControl {
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).OutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ServerCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServerCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ProxyCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProxyCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).SupportedProtocols)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IWebSocketControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketControl {}
impl ::core::fmt::Debug for IWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebSocketControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{2ec4bdc3-d9a5-455a-9811-de24d45337e9}");
}
unsafe impl ::windows::core::Interface for IWebSocketControl {
    type Vtable = IWebSocketControl_Vtbl;
}
impl ::core::clone::Clone for IWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec4bdc3_d9a5_455a_9811_de24d45337e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedProtocols: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedProtocols: usize,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IWebSocketControl2(::windows::core::IUnknown);
impl IWebSocketControl2 {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).IgnorableServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).OutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ServerCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetServerCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ProxyCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProxyCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).SupportedProtocols)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IWebSocketControl2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IWebSocketControl> for IWebSocketControl2 {}
impl ::core::cmp::PartialEq for IWebSocketControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketControl2 {}
impl ::core::fmt::Debug for IWebSocketControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketControl2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebSocketControl2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{79c3be03-f2ca-461e-af4e-9665bc2d0620}");
}
unsafe impl ::windows::core::Interface for IWebSocketControl2 {
    type Vtable = IWebSocketControl2_Vtbl;
}
impl ::core::clone::Clone for IWebSocketControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c3be03_f2ca_461e_af4e_9665bc2d0620);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebSocketErrorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebSocketErrorStatics {
    type Vtable = IWebSocketErrorStatics_Vtbl;
}
impl ::core::clone::Clone for IWebSocketErrorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketErrorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27cdf35b_1f61_4709_8e02_61283ada4e9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketErrorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IWebSocketInformation(::windows::core::IUnknown);
impl IWebSocketInformation {
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Protocol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IWebSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketInformation {}
impl ::core::fmt::Debug for IWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{5e01e316-c92a-47a5-b25f-07847639d181}");
}
unsafe impl ::windows::core::Interface for IWebSocketInformation {
    type Vtable = IWebSocketInformation_Vtbl;
}
impl ::core::clone::Clone for IWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e01e316_c92a_47a5_b25f_07847639d181);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct IWebSocketInformation2(::windows::core::IUnknown);
impl IWebSocketInformation2 {
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketSslErrorSeverity>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Protocol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IWebSocketInformation2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IWebSocketInformation> for IWebSocketInformation2 {}
impl ::core::cmp::PartialEq for IWebSocketInformation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebSocketInformation2 {}
impl ::core::fmt::Debug for IWebSocketInformation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebSocketInformation2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebSocketInformation2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ce1d39ce-a1b7-4d43-8269-8d5b981bd47a}");
}
unsafe impl ::windows::core::Interface for IWebSocketInformation2 {
    type Vtable = IWebSocketInformation2_Vtbl;
}
impl ::core::clone::Clone for IWebSocketInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketInformation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce1d39ce_a1b7_4d43_8269_8d5b981bd47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketInformation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebSocketServerCustomValidationRequestedEventArgs {
    type Vtable = IWebSocketServerCustomValidationRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebSocketServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebSocketServerCustomValidationRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffeffe48_022a_4ab7_8b36_e10af4640e6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ControlChannelTrigger(::windows::core::IUnknown);
impl ControlChannelTrigger {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ControlChannelTriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ControlChannelTriggerId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServerKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ServerKeepAliveIntervalInMinutes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetServerKeepAliveIntervalInMinutes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServerKeepAliveIntervalInMinutes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).CurrentKeepAliveIntervalInMinutes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).TransportObject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn KeepAliveTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::ApplicationModel::Background::IBackgroundTrigger>();
            (::windows::core::Interface::vtable(this).KeepAliveTrigger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn PushNotificationTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::ApplicationModel::Background::IBackgroundTrigger>();
            (::windows::core::Interface::vtable(this).PushNotificationTrigger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsingTransport<P0>(&self, transport: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UsingTransport)(::windows::core::Interface::as_raw(this), transport.into_param().abi()).ok() }
    }
    pub fn WaitForPushEnabled(&self) -> ::windows::core::Result<ControlChannelTriggerStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ControlChannelTriggerStatus>();
            (::windows::core::Interface::vtable(this).WaitForPushEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DecreaseNetworkKeepAliveInterval(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DecreaseNetworkKeepAliveInterval)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn FlushTransport(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).FlushTransport)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IControlChannelTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsWakeFromLowPowerSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateControlChannelTrigger(channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32) -> ::windows::core::Result<ControlChannelTrigger> {
        Self::IControlChannelTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ControlChannelTrigger>();
            (::windows::core::Interface::vtable(this).CreateControlChannelTrigger)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(channelid), serverkeepaliveintervalinminutes, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateControlChannelTriggerEx(channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType) -> ::windows::core::Result<ControlChannelTrigger> {
        Self::IControlChannelTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ControlChannelTrigger>();
            (::windows::core::Interface::vtable(this).CreateControlChannelTriggerEx)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(channelid), serverkeepaliveintervalinminutes, resourcerequesttype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IControlChannelTriggerFactory<R, F: FnOnce(&IControlChannelTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ControlChannelTrigger, IControlChannelTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ControlChannelTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ControlChannelTrigger {}
impl ::core::fmt::Debug for ControlChannelTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ControlChannelTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ControlChannelTrigger;{7d1431a7-ee96-40e8-a199-8703cd969ec3})");
}
impl ::core::clone::Clone for ControlChannelTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ControlChannelTrigger {
    type Vtable = IControlChannelTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ControlChannelTrigger {
    const IID: ::windows::core::GUID = <IControlChannelTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ControlChannelTrigger {
    const NAME: &'static str = "Windows.Networking.Sockets.ControlChannelTrigger";
}
::windows::imp::interface_hierarchy!(ControlChannelTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for ControlChannelTrigger {}
unsafe impl ::core::marker::Send for ControlChannelTrigger {}
unsafe impl ::core::marker::Sync for ControlChannelTrigger {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct DatagramSocket(::windows::core::IUnknown);
impl DatagramSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DatagramSocket, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> ::windows::core::Result<DatagramSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DatagramSocketControl>();
            (::windows::core::Interface::vtable(this).Control)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Information(&self) -> ::windows::core::Result<DatagramSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DatagramSocketInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self, remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithEndpointPairAsync(&self, endpointpair: &super::EndpointPair) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectWithEndpointPairAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(endpointpair), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BindServiceNameAsync(&self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindServiceNameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localservicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BindEndpointAsync(&self, localhostname: &super::HostName, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindEndpointAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localhostname), ::core::mem::transmute_copy(localservicename), &mut result__).from_abi(result__)
        }
    }
    pub fn JoinMulticastGroup(&self, host: &super::HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).JoinMulticastGroup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(host)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetOutputStreamAsync(&self, remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>();
            (::windows::core::Interface::vtable(this).GetOutputStreamAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetOutputStreamWithEndpointPairAsync(&self, endpointpair: &super::EndpointPair) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>();
            (::windows::core::Interface::vtable(this).GetOutputStreamWithEndpointPairAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(endpointpair), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived(&self, eventhandler: &super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MessageReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMessageReceived)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub fn BindServiceNameAndAdapterAsync(&self, localservicename: &::windows::core::HSTRING, adapter: &super::Connectivity::NetworkAdapter) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindServiceNameAndAdapterAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localservicename), ::core::mem::transmute_copy(adapter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).CancelIOAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnableTransferOwnership(&self, taskid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTransferOwnership)(::windows::core::Interface::as_raw(this), taskid).ok() }
    }
    pub fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTransferOwnershipWithConnectedStandbyAction)(::windows::core::Interface::as_raw(this), taskid, connectedstandbyaction).ok() }
    }
    pub fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnership)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid)).ok() }
    }
    pub fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &SocketActivityContext) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnershipWithContext)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid), ::core::mem::transmute_copy(data)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketid: &::windows::core::HSTRING, data: &SocketActivityContext, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnershipWithContextAndKeepAliveTime)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid), ::core::mem::transmute_copy(data), keepalivetime).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsAsync(remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IDatagramSocketStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>();
            (::windows::core::Interface::vtable(this).GetEndpointPairsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsWithSortOptionsAsync(remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IDatagramSocketStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>();
            (::windows::core::Interface::vtable(this).GetEndpointPairsWithSortOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), sortoptions, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDatagramSocketStatics<R, F: FnOnce(&IDatagramSocketStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DatagramSocket, IDatagramSocketStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DatagramSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocket {}
impl ::core::fmt::Debug for DatagramSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DatagramSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocket;{7fe25bbb-c3bc-4677-8446-ca28a465a3af})");
}
impl ::core::clone::Clone for DatagramSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DatagramSocket {
    type Vtable = IDatagramSocket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DatagramSocket {
    const IID: ::windows::core::GUID = <IDatagramSocket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DatagramSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocket";
}
::windows::imp::interface_hierarchy!(DatagramSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for DatagramSocket {}
unsafe impl ::core::marker::Send for DatagramSocket {}
unsafe impl ::core::marker::Sync for DatagramSocket {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct DatagramSocketControl(::windows::core::IUnknown);
impl DatagramSocketControl {
    pub fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketQualityOfService>();
            (::windows::core::Interface::vtable(this).QualityOfService)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQualityOfService)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).OutboundUnicastHopLimit)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundUnicastHopLimit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).InboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DontFragment(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DontFragment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDontFragment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDontFragment)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MulticastOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocketControl3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).MulticastOnly)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMulticastOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IDatagramSocketControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMulticastOnly)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for DatagramSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocketControl {}
impl ::core::fmt::Debug for DatagramSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocketControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DatagramSocketControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocketControl;{52ac3f2e-349a-4135-bb58-b79b2647d390})");
}
impl ::core::clone::Clone for DatagramSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DatagramSocketControl {
    type Vtable = IDatagramSocketControl_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DatagramSocketControl {
    const IID: ::windows::core::GUID = <IDatagramSocketControl as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DatagramSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketControl";
}
::windows::imp::interface_hierarchy!(DatagramSocketControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DatagramSocketControl {}
unsafe impl ::core::marker::Sync for DatagramSocketControl {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct DatagramSocketInformation(::windows::core::IUnknown);
impl DatagramSocketInformation {
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalPort)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).RemoteAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemotePort)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DatagramSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocketInformation {}
impl ::core::fmt::Debug for DatagramSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DatagramSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocketInformation;{5f1a569a-55fb-48cd-9706-7a974f7b1585})");
}
impl ::core::clone::Clone for DatagramSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DatagramSocketInformation {
    type Vtable = IDatagramSocketInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DatagramSocketInformation {
    const IID: ::windows::core::GUID = <IDatagramSocketInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DatagramSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketInformation";
}
::windows::imp::interface_hierarchy!(DatagramSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DatagramSocketInformation {}
unsafe impl ::core::marker::Sync for DatagramSocketInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct DatagramSocketMessageReceivedEventArgs(::windows::core::IUnknown);
impl DatagramSocketMessageReceivedEventArgs {
    pub fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).RemoteAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemotePort)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::DataReader>();
            (::windows::core::Interface::vtable(this).GetDataReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).GetDataStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DatagramSocketMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DatagramSocketMessageReceivedEventArgs {}
impl ::core::fmt::Debug for DatagramSocketMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DatagramSocketMessageReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DatagramSocketMessageReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs;{9e2ddca2-1712-4ce4-b179-8c652c6d107e})");
}
impl ::core::clone::Clone for DatagramSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DatagramSocketMessageReceivedEventArgs {
    type Vtable = IDatagramSocketMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DatagramSocketMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = <IDatagramSocketMessageReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DatagramSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs";
}
::windows::imp::interface_hierarchy!(DatagramSocketMessageReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DatagramSocketMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for DatagramSocketMessageReceivedEventArgs {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct MessageWebSocket(::windows::core::IUnknown);
impl MessageWebSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MessageWebSocket, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> ::windows::core::Result<MessageWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MessageWebSocketControl>();
            (::windows::core::Interface::vtable(this).Control)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Information(&self) -> ::windows::core::Result<MessageWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MessageWebSocketInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived(&self, eventhandler: &super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MessageReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMessageReceived)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested(&self, eventhandler: &super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocket2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ServerCustomValidationRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocket2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveServerCustomValidationRequested)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendNonfinalFrameAsync<P0>(&self, data: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocket3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>();
            (::windows::core::Interface::vtable(this).SendNonfinalFrameAsync)(::windows::core::Interface::as_raw(this), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendFinalFrameAsync<P0>(&self, data: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocket3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>();
            (::windows::core::Interface::vtable(this).SendFinalFrameAsync)(::windows::core::Interface::as_raw(this), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestHeader)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(headername), ::core::mem::transmute_copy(headervalue)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, eventhandler: &super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).CloseWithStatus)(::windows::core::Interface::as_raw(this), code, ::core::mem::transmute_copy(reason)).ok() }
    }
}
impl ::core::cmp::PartialEq for MessageWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocket {}
impl ::core::fmt::Debug for MessageWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MessageWebSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocket;{33727d08-34d5-4746-ad7b-8dde5bc2ef88})");
}
impl ::core::clone::Clone for MessageWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MessageWebSocket {
    type Vtable = IMessageWebSocket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MessageWebSocket {
    const IID: ::windows::core::GUID = <IMessageWebSocket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocket";
}
::windows::imp::interface_hierarchy!(MessageWebSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for MessageWebSocket {}
impl ::windows::core::CanTryInto<IWebSocket> for MessageWebSocket {}
unsafe impl ::core::marker::Send for MessageWebSocket {}
unsafe impl ::core::marker::Sync for MessageWebSocket {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct MessageWebSocketControl(::windows::core::IUnknown);
impl MessageWebSocketControl {
    pub fn MaxMessageSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxMessageSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxMessageSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxMessageSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageType(&self) -> ::windows::core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMessageType)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).DesiredUnsolicitedPongInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredUnsolicitedPongInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredUnsolicitedPongInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).ActualUnsolicitedPongInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReceiveMode(&self) -> ::windows::core::Result<MessageWebSocketReceiveMode> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MessageWebSocketReceiveMode>();
            (::windows::core::Interface::vtable(this).ReceiveMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReceiveMode(&self, value: MessageWebSocketReceiveMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetReceiveMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ClientCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate(&self, value: &super::super::Security::Cryptography::Certificates::Certificate) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetClientCertificate)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).OutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ServerCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetServerCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ProxyCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProxyCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).SupportedProtocols)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).IgnorableServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MessageWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketControl {}
impl ::core::fmt::Debug for MessageWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MessageWebSocketControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocketControl;{8118388a-c629-4f0a-80fb-81fc05538862})");
}
impl ::core::clone::Clone for MessageWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MessageWebSocketControl {
    type Vtable = IMessageWebSocketControl_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MessageWebSocketControl {
    const IID: ::windows::core::GUID = <IMessageWebSocketControl as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketControl";
}
::windows::imp::interface_hierarchy!(MessageWebSocketControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebSocketControl> for MessageWebSocketControl {}
impl ::windows::core::CanTryInto<IWebSocketControl2> for MessageWebSocketControl {}
unsafe impl ::core::marker::Send for MessageWebSocketControl {}
unsafe impl ::core::marker::Sync for MessageWebSocketControl {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct MessageWebSocketInformation(::windows::core::IUnknown);
impl MessageWebSocketInformation {
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Protocol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketSslErrorSeverity>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MessageWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketInformation {}
impl ::core::fmt::Debug for MessageWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MessageWebSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocketInformation;{5e01e316-c92a-47a5-b25f-07847639d181})");
}
impl ::core::clone::Clone for MessageWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MessageWebSocketInformation {
    type Vtable = IWebSocketInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MessageWebSocketInformation {
    const IID: ::windows::core::GUID = <IWebSocketInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketInformation";
}
::windows::imp::interface_hierarchy!(MessageWebSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebSocketInformation> for MessageWebSocketInformation {}
impl ::windows::core::CanTryInto<IWebSocketInformation2> for MessageWebSocketInformation {}
unsafe impl ::core::marker::Send for MessageWebSocketInformation {}
unsafe impl ::core::marker::Sync for MessageWebSocketInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct MessageWebSocketMessageReceivedEventArgs(::windows::core::IUnknown);
impl MessageWebSocketMessageReceivedEventArgs {
    pub fn MessageType(&self) -> ::windows::core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::DataReader>();
            (::windows::core::Interface::vtable(this).GetDataReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).GetDataStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMessageComplete(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IMessageWebSocketMessageReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsMessageComplete)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MessageWebSocketMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageWebSocketMessageReceivedEventArgs {}
impl ::core::fmt::Debug for MessageWebSocketMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketMessageReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MessageWebSocketMessageReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs;{478c22ac-4c4b-42ed-9ed7-1ef9f94fa3d5})");
}
impl ::core::clone::Clone for MessageWebSocketMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MessageWebSocketMessageReceivedEventArgs {
    type Vtable = IMessageWebSocketMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MessageWebSocketMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMessageWebSocketMessageReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MessageWebSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs";
}
::windows::imp::interface_hierarchy!(MessageWebSocketMessageReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MessageWebSocketMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MessageWebSocketMessageReceivedEventArgs {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ServerMessageWebSocket(::windows::core::IUnknown);
impl ServerMessageWebSocket {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived(&self, value: &super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MessageReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMessageReceived)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Control(&self) -> ::windows::core::Result<ServerMessageWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ServerMessageWebSocketControl>();
            (::windows::core::Interface::vtable(this).Control)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Information(&self) -> ::windows::core::Result<ServerMessageWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ServerMessageWebSocketInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, value: &super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CloseWithStatus)(::windows::core::Interface::as_raw(this), code, ::core::mem::transmute_copy(reason)).ok() }
    }
}
impl ::core::cmp::PartialEq for ServerMessageWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerMessageWebSocket {}
impl ::core::fmt::Debug for ServerMessageWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerMessageWebSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ServerMessageWebSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerMessageWebSocket;{e3ac9240-813b-5efd-7e11-ae2305fc77f1})");
}
impl ::core::clone::Clone for ServerMessageWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ServerMessageWebSocket {
    type Vtable = IServerMessageWebSocket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ServerMessageWebSocket {
    const IID: ::windows::core::GUID = <IServerMessageWebSocket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ServerMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocket";
}
::windows::imp::interface_hierarchy!(ServerMessageWebSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for ServerMessageWebSocket {}
unsafe impl ::core::marker::Send for ServerMessageWebSocket {}
unsafe impl ::core::marker::Sync for ServerMessageWebSocket {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ServerMessageWebSocketControl(::windows::core::IUnknown);
impl ServerMessageWebSocketControl {
    pub fn MessageType(&self) -> ::windows::core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMessageType)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ServerMessageWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerMessageWebSocketControl {}
impl ::core::fmt::Debug for ServerMessageWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerMessageWebSocketControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ServerMessageWebSocketControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerMessageWebSocketControl;{69c2f051-1c1f-587a-4519-2181610192b7})");
}
impl ::core::clone::Clone for ServerMessageWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ServerMessageWebSocketControl {
    type Vtable = IServerMessageWebSocketControl_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ServerMessageWebSocketControl {
    const IID: ::windows::core::GUID = <IServerMessageWebSocketControl as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ServerMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocketControl";
}
::windows::imp::interface_hierarchy!(ServerMessageWebSocketControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ServerMessageWebSocketControl {}
unsafe impl ::core::marker::Sync for ServerMessageWebSocketControl {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ServerMessageWebSocketInformation(::windows::core::IUnknown);
impl ServerMessageWebSocketInformation {
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Protocol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ServerMessageWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerMessageWebSocketInformation {}
impl ::core::fmt::Debug for ServerMessageWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerMessageWebSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ServerMessageWebSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerMessageWebSocketInformation;{fc32b45f-4448-5505-6cc9-09afa8915f5d})");
}
impl ::core::clone::Clone for ServerMessageWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ServerMessageWebSocketInformation {
    type Vtable = IServerMessageWebSocketInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ServerMessageWebSocketInformation {
    const IID: ::windows::core::GUID = <IServerMessageWebSocketInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ServerMessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocketInformation";
}
::windows::imp::interface_hierarchy!(ServerMessageWebSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ServerMessageWebSocketInformation {}
unsafe impl ::core::marker::Sync for ServerMessageWebSocketInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ServerStreamWebSocket(::windows::core::IUnknown);
impl ServerStreamWebSocket {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Information(&self) -> ::windows::core::Result<ServerStreamWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ServerStreamWebSocketInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).InputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, value: &super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CloseWithStatus)(::windows::core::Interface::as_raw(this), code, ::core::mem::transmute_copy(reason)).ok() }
    }
}
impl ::core::cmp::PartialEq for ServerStreamWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerStreamWebSocket {}
impl ::core::fmt::Debug for ServerStreamWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerStreamWebSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ServerStreamWebSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerStreamWebSocket;{2ced5bbf-74f6-55e4-79df-9132680dfee8})");
}
impl ::core::clone::Clone for ServerStreamWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ServerStreamWebSocket {
    type Vtable = IServerStreamWebSocket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ServerStreamWebSocket {
    const IID: ::windows::core::GUID = <IServerStreamWebSocket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ServerStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerStreamWebSocket";
}
::windows::imp::interface_hierarchy!(ServerStreamWebSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for ServerStreamWebSocket {}
unsafe impl ::core::marker::Send for ServerStreamWebSocket {}
unsafe impl ::core::marker::Sync for ServerStreamWebSocket {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct ServerStreamWebSocketInformation(::windows::core::IUnknown);
impl ServerStreamWebSocketInformation {
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Protocol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ServerStreamWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServerStreamWebSocketInformation {}
impl ::core::fmt::Debug for ServerStreamWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerStreamWebSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ServerStreamWebSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.ServerStreamWebSocketInformation;{fc32b45f-4448-5505-6cc9-09aba8915f5d})");
}
impl ::core::clone::Clone for ServerStreamWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ServerStreamWebSocketInformation {
    type Vtable = IServerStreamWebSocketInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ServerStreamWebSocketInformation {
    const IID: ::windows::core::GUID = <IServerStreamWebSocketInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ServerStreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerStreamWebSocketInformation";
}
::windows::imp::interface_hierarchy!(ServerStreamWebSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ServerStreamWebSocketInformation {}
unsafe impl ::core::marker::Sync for ServerStreamWebSocketInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketActivityContext(::windows::core::IUnknown);
impl SocketActivityContext {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(data: P0) -> ::windows::core::Result<SocketActivityContext>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISocketActivityContextFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketActivityContext>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketActivityContextFactory<R, F: FnOnce(&ISocketActivityContextFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SocketActivityContext, ISocketActivityContextFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SocketActivityContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityContext {}
impl ::core::fmt::Debug for SocketActivityContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityContext").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityContext {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.SocketActivityContext;{43b04d64-4c85-4396-a637-1d973f6ebd49})");
}
impl ::core::clone::Clone for SocketActivityContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SocketActivityContext {
    type Vtable = ISocketActivityContext_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SocketActivityContext {
    const IID: ::windows::core::GUID = <ISocketActivityContext as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SocketActivityContext {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityContext";
}
::windows::imp::interface_hierarchy!(SocketActivityContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SocketActivityContext {}
unsafe impl ::core::marker::Sync for SocketActivityContext {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketActivityInformation(::windows::core::IUnknown);
impl SocketActivityInformation {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SocketKind(&self) -> ::windows::core::Result<SocketActivityKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketActivityKind>();
            (::windows::core::Interface::vtable(this).SocketKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Context(&self) -> ::windows::core::Result<SocketActivityContext> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketActivityContext>();
            (::windows::core::Interface::vtable(this).Context)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DatagramSocket(&self) -> ::windows::core::Result<DatagramSocket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DatagramSocket>();
            (::windows::core::Interface::vtable(this).DatagramSocket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreamSocket(&self) -> ::windows::core::Result<StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocket>();
            (::windows::core::Interface::vtable(this).StreamSocket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreamSocketListener(&self) -> ::windows::core::Result<StreamSocketListener> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocketListener>();
            (::windows::core::Interface::vtable(this).StreamSocketListener)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllSockets() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>> {
        Self::ISocketActivityInformationStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>>();
            (::windows::core::Interface::vtable(this).AllSockets)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketActivityInformationStatics<R, F: FnOnce(&ISocketActivityInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SocketActivityInformation, ISocketActivityInformationStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SocketActivityInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityInformation {}
impl ::core::fmt::Debug for SocketActivityInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.SocketActivityInformation;{8d8a42e4-a87e-4b74-9968-185b2511defe})");
}
impl ::core::clone::Clone for SocketActivityInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SocketActivityInformation {
    type Vtable = ISocketActivityInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SocketActivityInformation {
    const IID: ::windows::core::GUID = <ISocketActivityInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SocketActivityInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityInformation";
}
::windows::imp::interface_hierarchy!(SocketActivityInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SocketActivityInformation {}
unsafe impl ::core::marker::Sync for SocketActivityInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct SocketActivityTriggerDetails(::windows::core::IUnknown);
impl SocketActivityTriggerDetails {
    pub fn Reason(&self) -> ::windows::core::Result<SocketActivityTriggerReason> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketActivityTriggerReason>();
            (::windows::core::Interface::vtable(this).Reason)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SocketInformation(&self) -> ::windows::core::Result<SocketActivityInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketActivityInformation>();
            (::windows::core::Interface::vtable(this).SocketInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SocketActivityTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityTriggerDetails {}
impl ::core::fmt::Debug for SocketActivityTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.SocketActivityTriggerDetails;{45f406a7-fc9f-4f81-acad-355fef51e67b})");
}
impl ::core::clone::Clone for SocketActivityTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SocketActivityTriggerDetails {
    type Vtable = ISocketActivityTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SocketActivityTriggerDetails {
    const IID: ::windows::core::GUID = <ISocketActivityTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SocketActivityTriggerDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityTriggerDetails";
}
::windows::imp::interface_hierarchy!(SocketActivityTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SocketActivityTriggerDetails {}
unsafe impl ::core::marker::Sync for SocketActivityTriggerDetails {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
pub struct SocketError;
impl SocketError {
    pub fn GetStatus(hresult: i32) -> ::windows::core::Result<SocketErrorStatus> {
        Self::ISocketErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketErrorStatus>();
            (::windows::core::Interface::vtable(this).GetStatus)(::windows::core::Interface::as_raw(this), hresult, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketErrorStatics<R, F: FnOnce(&ISocketErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SocketError, ISocketErrorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SocketError {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketError";
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocket(::windows::core::IUnknown);
impl StreamSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StreamSocket, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> ::windows::core::Result<StreamSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocketControl>();
            (::windows::core::Interface::vtable(this).Control)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Information(&self) -> ::windows::core::Result<StreamSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocketInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).InputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithEndpointPairAsync(&self, endpointpair: &super::EndpointPair) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectWithEndpointPairAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(endpointpair), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self, remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithEndpointPairAndProtectionLevelAsync(&self, endpointpair: &super::EndpointPair, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectWithEndpointPairAndProtectionLevelAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(endpointpair), protectionlevel, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectWithProtectionLevelAsync(&self, remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectWithProtectionLevelAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), protectionlevel, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpgradeToSslAsync(&self, protectionlevel: SocketProtectionLevel, validationhostname: &super::HostName) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).UpgradeToSslAsync)(::windows::core::Interface::as_raw(this), protectionlevel, ::core::mem::transmute_copy(validationhostname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub fn ConnectWithProtectionLevelAndAdapterAsync(&self, remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &super::Connectivity::NetworkAdapter) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectWithProtectionLevelAndAdapterAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), protectionlevel, ::core::mem::transmute_copy(adapter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).CancelIOAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnableTransferOwnership(&self, taskid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTransferOwnership)(::windows::core::Interface::as_raw(this), taskid).ok() }
    }
    pub fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTransferOwnershipWithConnectedStandbyAction)(::windows::core::Interface::as_raw(this), taskid, connectedstandbyaction).ok() }
    }
    pub fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnership)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid)).ok() }
    }
    pub fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &SocketActivityContext) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnershipWithContext)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid), ::core::mem::transmute_copy(data)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketid: &::windows::core::HSTRING, data: &SocketActivityContext, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocket3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnershipWithContextAndKeepAliveTime)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid), ::core::mem::transmute_copy(data), keepalivetime).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsAsync(remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IStreamSocketStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>();
            (::windows::core::Interface::vtable(this).GetEndpointPairsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsWithSortOptionsAsync(remotehostname: &super::HostName, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>> {
        Self::IStreamSocketStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>();
            (::windows::core::Interface::vtable(this).GetEndpointPairsWithSortOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), ::core::mem::transmute_copy(remoteservicename), sortoptions, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStreamSocketStatics<R, F: FnOnce(&IStreamSocketStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StreamSocket, IStreamSocketStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for StreamSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocket {}
impl ::core::fmt::Debug for StreamSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocket;{69a22cf3-fc7b-4857-af38-f6e7de6a5b49})");
}
impl ::core::clone::Clone for StreamSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocket {
    type Vtable = IStreamSocket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocket {
    const IID: ::windows::core::GUID = <IStreamSocket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocket";
}
::windows::imp::interface_hierarchy!(StreamSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for StreamSocket {}
unsafe impl ::core::marker::Send for StreamSocket {}
unsafe impl ::core::marker::Sync for StreamSocket {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocketControl(::windows::core::IUnknown);
impl StreamSocketControl {
    pub fn NoDelay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).NoDelay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNoDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepAlive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).KeepAlive)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeepAlive)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).OutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketQualityOfService>();
            (::windows::core::Interface::vtable(this).QualityOfService)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQualityOfService)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).OutboundUnicastHopLimit)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundUnicastHopLimit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).IgnorableServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SerializeConnectionAttempts(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SerializeConnectionAttempts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSerializeConnectionAttempts(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSerializeConnectionAttempts)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ClientCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate(&self, value: &super::super::Security::Cryptography::Certificates::Certificate) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetClientCertificate)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MinProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketProtectionLevel>();
            (::windows::core::Interface::vtable(this).MinProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMinProtectionLevel(&self, value: SocketProtectionLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketControl4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMinProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for StreamSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketControl {}
impl ::core::fmt::Debug for StreamSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocketControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketControl;{fe25adf1-92ab-4af3-9992-0f4c85e36cc4})");
}
impl ::core::clone::Clone for StreamSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocketControl {
    type Vtable = IStreamSocketControl_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocketControl {
    const IID: ::windows::core::GUID = <IStreamSocketControl as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketControl";
}
::windows::imp::interface_hierarchy!(StreamSocketControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StreamSocketControl {}
unsafe impl ::core::marker::Sync for StreamSocketControl {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocketInformation(::windows::core::IUnknown);
impl StreamSocketInformation {
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalPort)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).RemoteHostName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).RemoteAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemoteServiceName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemotePort)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoundTripTimeStatistics(&self) -> ::windows::core::Result<RoundTripTimeStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<RoundTripTimeStatistics>();
            (::windows::core::Interface::vtable(this).RoundTripTimeStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketProtectionLevel>();
            (::windows::core::Interface::vtable(this).ProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionKey(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).SessionKey)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketSslErrorSeverity>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StreamSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketInformation {}
impl ::core::fmt::Debug for StreamSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketInformation;{3b80ae30-5e68-4205-88f0-dc85d2e25ded})");
}
impl ::core::clone::Clone for StreamSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocketInformation {
    type Vtable = IStreamSocketInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocketInformation {
    const IID: ::windows::core::GUID = <IStreamSocketInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketInformation";
}
::windows::imp::interface_hierarchy!(StreamSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StreamSocketInformation {}
unsafe impl ::core::marker::Sync for StreamSocketInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocketListener(::windows::core::IUnknown);
impl StreamSocketListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StreamSocketListener, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> ::windows::core::Result<StreamSocketListenerControl> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocketListenerControl>();
            (::windows::core::Interface::vtable(this).Control)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Information(&self) -> ::windows::core::Result<StreamSocketListenerInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocketListenerInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BindServiceNameAsync(&self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindServiceNameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localservicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BindEndpointAsync(&self, localhostname: &super::HostName, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindEndpointAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localhostname), ::core::mem::transmute_copy(localservicename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionReceived(&self, eventhandler: &super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ConnectionReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionReceived)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BindServiceNameWithProtectionLevelAsync(&self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindServiceNameWithProtectionLevelAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localservicename), protectionlevel, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity"))]
    pub fn BindServiceNameWithProtectionLevelAndAdapterAsync(&self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &super::Connectivity::NetworkAdapter) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).BindServiceNameWithProtectionLevelAndAdapterAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(localservicename), protectionlevel, ::core::mem::transmute_copy(adapter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).CancelIOAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnableTransferOwnership(&self, taskid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTransferOwnership)(::windows::core::Interface::as_raw(this), taskid).ok() }
    }
    pub fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableTransferOwnershipWithConnectedStandbyAction)(::windows::core::Interface::as_raw(this), taskid, connectedstandbyaction).ok() }
    }
    pub fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnership)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid)).ok() }
    }
    pub fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &SocketActivityContext) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).TransferOwnershipWithContext)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(socketid), ::core::mem::transmute_copy(data)).ok() }
    }
}
impl ::core::cmp::PartialEq for StreamSocketListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListener {}
impl ::core::fmt::Debug for StreamSocketListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListener").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocketListener {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListener;{ff513437-df9f-4df0-bf82-0ec5d7b35aae})");
}
impl ::core::clone::Clone for StreamSocketListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocketListener {
    type Vtable = IStreamSocketListener_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocketListener {
    const IID: ::windows::core::GUID = <IStreamSocketListener as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocketListener {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListener";
}
::windows::imp::interface_hierarchy!(StreamSocketListener, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for StreamSocketListener {}
unsafe impl ::core::marker::Send for StreamSocketListener {}
unsafe impl ::core::marker::Sync for StreamSocketListener {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocketListenerConnectionReceivedEventArgs(::windows::core::IUnknown);
impl StreamSocketListenerConnectionReceivedEventArgs {
    pub fn Socket(&self) -> ::windows::core::Result<StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamSocket>();
            (::windows::core::Interface::vtable(this).Socket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StreamSocketListenerConnectionReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListenerConnectionReceivedEventArgs {}
impl ::core::fmt::Debug for StreamSocketListenerConnectionReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListenerConnectionReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocketListenerConnectionReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs;{0c472ea9-373f-447b-85b1-ddd4548803ba})");
}
impl ::core::clone::Clone for StreamSocketListenerConnectionReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocketListenerConnectionReceivedEventArgs {
    type Vtable = IStreamSocketListenerConnectionReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocketListenerConnectionReceivedEventArgs {
    const IID: ::windows::core::GUID = <IStreamSocketListenerConnectionReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocketListenerConnectionReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs";
}
::windows::imp::interface_hierarchy!(StreamSocketListenerConnectionReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StreamSocketListenerConnectionReceivedEventArgs {}
unsafe impl ::core::marker::Sync for StreamSocketListenerConnectionReceivedEventArgs {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocketListenerControl(::windows::core::IUnknown);
impl StreamSocketListenerControl {
    pub fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketQualityOfService>();
            (::windows::core::Interface::vtable(this).QualityOfService)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQualityOfService)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NoDelay(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).NoDelay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNoDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepAlive(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).KeepAlive)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetKeepAlive)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).OutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).OutboundUnicastHopLimit)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundUnicastHopLimit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for StreamSocketListenerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListenerControl {}
impl ::core::fmt::Debug for StreamSocketListenerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListenerControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocketListenerControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListenerControl;{20d8c576-8d8a-4dba-9722-a16c4d984980})");
}
impl ::core::clone::Clone for StreamSocketListenerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocketListenerControl {
    type Vtable = IStreamSocketListenerControl_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocketListenerControl {
    const IID: ::windows::core::GUID = <IStreamSocketListenerControl as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocketListenerControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerControl";
}
::windows::imp::interface_hierarchy!(StreamSocketListenerControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StreamSocketListenerControl {}
unsafe impl ::core::marker::Sync for StreamSocketListenerControl {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamSocketListenerInformation(::windows::core::IUnknown);
impl StreamSocketListenerInformation {
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalPort)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StreamSocketListenerInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamSocketListenerInformation {}
impl ::core::fmt::Debug for StreamSocketListenerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamSocketListenerInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamSocketListenerInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamSocketListenerInformation;{e62ba82f-a63a-430b-bf62-29e93e5633b4})");
}
impl ::core::clone::Clone for StreamSocketListenerInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamSocketListenerInformation {
    type Vtable = IStreamSocketListenerInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamSocketListenerInformation {
    const IID: ::windows::core::GUID = <IStreamSocketListenerInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamSocketListenerInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerInformation";
}
::windows::imp::interface_hierarchy!(StreamSocketListenerInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StreamSocketListenerInformation {}
unsafe impl ::core::marker::Sync for StreamSocketListenerInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamWebSocket(::windows::core::IUnknown);
impl StreamWebSocket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StreamWebSocket, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> ::windows::core::Result<StreamWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamWebSocketControl>();
            (::windows::core::Interface::vtable(this).Control)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Information(&self) -> ::windows::core::Result<StreamWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<StreamWebSocketInformation>();
            (::windows::core::Interface::vtable(this).Information)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).InputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerCustomValidationRequested(&self, eventhandler: &super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocket2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ServerCustomValidationRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerCustomValidationRequested(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocket2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveServerCustomValidationRequested)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).OutputStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self, uri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRequestHeader)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(headername), ::core::mem::transmute_copy(headervalue)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed(&self, eventhandler: &super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocket>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).CloseWithStatus)(::windows::core::Interface::as_raw(this), code, ::core::mem::transmute_copy(reason)).ok() }
    }
}
impl ::core::cmp::PartialEq for StreamWebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamWebSocket {}
impl ::core::fmt::Debug for StreamWebSocket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamWebSocket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamWebSocket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamWebSocket;{bd4a49d8-b289-45bb-97eb-c7525205a843})");
}
impl ::core::clone::Clone for StreamWebSocket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamWebSocket {
    type Vtable = IStreamWebSocket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamWebSocket {
    const IID: ::windows::core::GUID = <IStreamWebSocket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocket";
}
::windows::imp::interface_hierarchy!(StreamWebSocket, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for StreamWebSocket {}
impl ::windows::core::CanTryInto<IWebSocket> for StreamWebSocket {}
unsafe impl ::core::marker::Send for StreamWebSocket {}
unsafe impl ::core::marker::Sync for StreamWebSocket {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamWebSocketControl(::windows::core::IUnknown);
impl StreamWebSocketControl {
    pub fn NoDelay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).NoDelay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNoDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).DesiredUnsolicitedPongInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredUnsolicitedPongInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredUnsolicitedPongInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).ActualUnsolicitedPongInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ClientCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate(&self, value: &super::super::Security::Cryptography::Certificates::Certificate) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetClientCertificate)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).OutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ServerCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetServerCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).ProxyCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProxyCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).SupportedProtocols)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).IgnorableServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StreamWebSocketControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamWebSocketControl {}
impl ::core::fmt::Debug for StreamWebSocketControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamWebSocketControl").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamWebSocketControl {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamWebSocketControl;{b4f478b1-a45a-48db-953a-645b7d964c07})");
}
impl ::core::clone::Clone for StreamWebSocketControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamWebSocketControl {
    type Vtable = IStreamWebSocketControl_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamWebSocketControl {
    const IID: ::windows::core::GUID = <IStreamWebSocketControl as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocketControl";
}
::windows::imp::interface_hierarchy!(StreamWebSocketControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebSocketControl> for StreamWebSocketControl {}
impl ::windows::core::CanTryInto<IWebSocketControl2> for StreamWebSocketControl {}
unsafe impl ::core::marker::Send for StreamWebSocketControl {}
unsafe impl ::core::marker::Sync for StreamWebSocketControl {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct StreamWebSocketInformation(::windows::core::IUnknown);
impl StreamWebSocketInformation {
    pub fn LocalAddress(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::HostName>();
            (::windows::core::Interface::vtable(this).LocalAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BandwidthStatistics>();
            (::windows::core::Interface::vtable(this).BandwidthStatistics)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Protocol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketSslErrorSeverity>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &::windows::core::ComInterface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StreamWebSocketInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StreamWebSocketInformation {}
impl ::core::fmt::Debug for StreamWebSocketInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamWebSocketInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StreamWebSocketInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.StreamWebSocketInformation;{5e01e316-c92a-47a5-b25f-07847639d181})");
}
impl ::core::clone::Clone for StreamWebSocketInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StreamWebSocketInformation {
    type Vtable = IWebSocketInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StreamWebSocketInformation {
    const IID: ::windows::core::GUID = <IWebSocketInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocketInformation";
}
::windows::imp::interface_hierarchy!(StreamWebSocketInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebSocketInformation> for StreamWebSocketInformation {}
impl ::windows::core::CanTryInto<IWebSocketInformation2> for StreamWebSocketInformation {}
unsafe impl ::core::marker::Send for StreamWebSocketInformation {}
unsafe impl ::core::marker::Sync for StreamWebSocketInformation {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct WebSocketClosedEventArgs(::windows::core::IUnknown);
impl WebSocketClosedEventArgs {
    pub fn Code(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u16>();
            (::windows::core::Interface::vtable(this).Code)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Reason)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebSocketClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebSocketClosedEventArgs {}
impl ::core::fmt::Debug for WebSocketClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSocketClosedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebSocketClosedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.WebSocketClosedEventArgs;{ceb78d07-d0a8-4703-a091-c8c2c0915bc3})");
}
impl ::core::clone::Clone for WebSocketClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebSocketClosedEventArgs {
    type Vtable = IWebSocketClosedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebSocketClosedEventArgs {
    const IID: ::windows::core::GUID = <IWebSocketClosedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebSocketClosedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketClosedEventArgs";
}
::windows::imp::interface_hierarchy!(WebSocketClosedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebSocketClosedEventArgs {}
unsafe impl ::core::marker::Sync for WebSocketClosedEventArgs {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
pub struct WebSocketError;
impl WebSocketError {
    #[doc = "*Required features: `\"Web\"`*"]
    #[cfg(feature = "Web")]
    pub fn GetStatus(hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus> {
        Self::IWebSocketErrorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Web::WebErrorStatus>();
            (::windows::core::Interface::vtable(this).GetStatus)(::windows::core::Interface::as_raw(this), hresult, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebSocketErrorStatics<R, F: FnOnce(&IWebSocketErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebSocketError, IWebSocketErrorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WebSocketError {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketError";
}
#[doc = "*Required features: `\"Networking_Sockets\"`, `\"ApplicationModel_Background\"`*"]
#[cfg(feature = "ApplicationModel_Background")]
#[repr(transparent)]
pub struct WebSocketKeepAlive(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Background")]
impl WebSocketKeepAlive {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebSocketKeepAlive, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Run<P0>(&self, taskinstance: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::ApplicationModel::Background::IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Run)(::windows::core::Interface::as_raw(this), taskinstance.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::cmp::PartialEq for WebSocketKeepAlive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::cmp::Eq for WebSocketKeepAlive {}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::fmt::Debug for WebSocketKeepAlive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSocketKeepAlive").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::RuntimeType for WebSocketKeepAlive {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.WebSocketKeepAlive;{7d13d534-fd12-43ce-8c22-ea1ff13c06df})");
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::clone::Clone for WebSocketKeepAlive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::windows::core::Interface for WebSocketKeepAlive {
    type Vtable = super::super::ApplicationModel::Background::IBackgroundTask_Vtbl;
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::windows::core::ComInterface for WebSocketKeepAlive {
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Background::IBackgroundTask as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::RuntimeName for WebSocketKeepAlive {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketKeepAlive";
}
#[cfg(feature = "ApplicationModel_Background")]
::windows::imp::interface_hierarchy!(WebSocketKeepAlive, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::CanTryInto<super::super::ApplicationModel::Background::IBackgroundTask> for WebSocketKeepAlive {}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::core::marker::Send for WebSocketKeepAlive {}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl ::core::marker::Sync for WebSocketKeepAlive {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
pub struct WebSocketServerCustomValidationRequestedEventArgs(::windows::core::IUnknown);
impl WebSocketServerCustomValidationRequestedEventArgs {
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Cryptography::Certificates::Certificate>();
            (::windows::core::Interface::vtable(this).ServerCertificate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SocketSslErrorSeverity>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>();
            (::windows::core::Interface::vtable(this).ServerCertificateErrors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).ServerIntermediateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reject)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebSocketServerCustomValidationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebSocketServerCustomValidationRequestedEventArgs {}
impl ::core::fmt::Debug for WebSocketServerCustomValidationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSocketServerCustomValidationRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebSocketServerCustomValidationRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs;{ffeffe48-022a-4ab7-8b36-e10af4640e6b})");
}
impl ::core::clone::Clone for WebSocketServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebSocketServerCustomValidationRequestedEventArgs {
    type Vtable = IWebSocketServerCustomValidationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebSocketServerCustomValidationRequestedEventArgs {
    const IID: ::windows::core::GUID = <IWebSocketServerCustomValidationRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebSocketServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(WebSocketServerCustomValidationRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebSocketServerCustomValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WebSocketServerCustomValidationRequestedEventArgs {}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ControlChannelTriggerResetReason(pub i32);
impl ControlChannelTriggerResetReason {
    pub const FastUserSwitched: Self = Self(0i32);
    pub const LowPowerExit: Self = Self(1i32);
    pub const QuietHoursExit: Self = Self(2i32);
    pub const ApplicationRestart: Self = Self(3i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResetReason {}
impl ::core::clone::Clone for ControlChannelTriggerResetReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ControlChannelTriggerResetReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ControlChannelTriggerResetReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ControlChannelTriggerResetReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerResetReason").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ControlChannelTriggerResetReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerResetReason;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ControlChannelTriggerResourceType(pub i32);
impl ControlChannelTriggerResourceType {
    pub const RequestSoftwareSlot: Self = Self(0i32);
    pub const RequestHardwareSlot: Self = Self(1i32);
}
impl ::core::marker::Copy for ControlChannelTriggerResourceType {}
impl ::core::clone::Clone for ControlChannelTriggerResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ControlChannelTriggerResourceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ControlChannelTriggerResourceType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ControlChannelTriggerResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerResourceType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ControlChannelTriggerResourceType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerResourceType;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ControlChannelTriggerStatus(pub i32);
impl ControlChannelTriggerStatus {
    pub const HardwareSlotRequested: Self = Self(0i32);
    pub const SoftwareSlotAllocated: Self = Self(1i32);
    pub const HardwareSlotAllocated: Self = Self(2i32);
    pub const PolicyError: Self = Self(3i32);
    pub const SystemError: Self = Self(4i32);
    pub const TransportDisconnected: Self = Self(5i32);
    pub const ServiceUnavailable: Self = Self(6i32);
}
impl ::core::marker::Copy for ControlChannelTriggerStatus {}
impl ::core::clone::Clone for ControlChannelTriggerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ControlChannelTriggerStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ControlChannelTriggerStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ControlChannelTriggerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ControlChannelTriggerStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerStatus;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MessageWebSocketReceiveMode(pub i32);
impl MessageWebSocketReceiveMode {
    pub const FullMessage: Self = Self(0i32);
    pub const PartialMessage: Self = Self(1i32);
}
impl ::core::marker::Copy for MessageWebSocketReceiveMode {}
impl ::core::clone::Clone for MessageWebSocketReceiveMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MessageWebSocketReceiveMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MessageWebSocketReceiveMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MessageWebSocketReceiveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageWebSocketReceiveMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MessageWebSocketReceiveMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.MessageWebSocketReceiveMode;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketActivityConnectedStandbyAction(pub i32);
impl SocketActivityConnectedStandbyAction {
    pub const DoNotWake: Self = Self(0i32);
    pub const Wake: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketActivityConnectedStandbyAction {}
impl ::core::clone::Clone for SocketActivityConnectedStandbyAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketActivityConnectedStandbyAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketActivityConnectedStandbyAction {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketActivityConnectedStandbyAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityConnectedStandbyAction").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityConnectedStandbyAction {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityConnectedStandbyAction;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketActivityKind(pub i32);
impl SocketActivityKind {
    pub const None: Self = Self(0i32);
    pub const StreamSocketListener: Self = Self(1i32);
    pub const DatagramSocket: Self = Self(2i32);
    pub const StreamSocket: Self = Self(3i32);
}
impl ::core::marker::Copy for SocketActivityKind {}
impl ::core::clone::Clone for SocketActivityKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketActivityKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketActivityKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketActivityKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityKind;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketActivityTriggerReason(pub i32);
impl SocketActivityTriggerReason {
    pub const None: Self = Self(0i32);
    pub const SocketActivity: Self = Self(1i32);
    pub const ConnectionAccepted: Self = Self(2i32);
    pub const KeepAliveTimerExpired: Self = Self(3i32);
    pub const SocketClosed: Self = Self(4i32);
}
impl ::core::marker::Copy for SocketActivityTriggerReason {}
impl ::core::clone::Clone for SocketActivityTriggerReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketActivityTriggerReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketActivityTriggerReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketActivityTriggerReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityTriggerReason").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityTriggerReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityTriggerReason;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketErrorStatus(pub i32);
impl SocketErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const OperationAborted: Self = Self(1i32);
    pub const HttpInvalidServerResponse: Self = Self(2i32);
    pub const ConnectionTimedOut: Self = Self(3i32);
    pub const AddressFamilyNotSupported: Self = Self(4i32);
    pub const SocketTypeNotSupported: Self = Self(5i32);
    pub const HostNotFound: Self = Self(6i32);
    pub const NoDataRecordOfRequestedType: Self = Self(7i32);
    pub const NonAuthoritativeHostNotFound: Self = Self(8i32);
    pub const ClassTypeNotFound: Self = Self(9i32);
    pub const AddressAlreadyInUse: Self = Self(10i32);
    pub const CannotAssignRequestedAddress: Self = Self(11i32);
    pub const ConnectionRefused: Self = Self(12i32);
    pub const NetworkIsUnreachable: Self = Self(13i32);
    pub const UnreachableHost: Self = Self(14i32);
    pub const NetworkIsDown: Self = Self(15i32);
    pub const NetworkDroppedConnectionOnReset: Self = Self(16i32);
    pub const SoftwareCausedConnectionAbort: Self = Self(17i32);
    pub const ConnectionResetByPeer: Self = Self(18i32);
    pub const HostIsDown: Self = Self(19i32);
    pub const NoAddressesFound: Self = Self(20i32);
    pub const TooManyOpenFiles: Self = Self(21i32);
    pub const MessageTooLong: Self = Self(22i32);
    pub const CertificateExpired: Self = Self(23i32);
    pub const CertificateUntrustedRoot: Self = Self(24i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(25i32);
    pub const CertificateWrongUsage: Self = Self(26i32);
    pub const CertificateRevoked: Self = Self(27i32);
    pub const CertificateNoRevocationCheck: Self = Self(28i32);
    pub const CertificateRevocationServerOffline: Self = Self(29i32);
    pub const CertificateIsInvalid: Self = Self(30i32);
}
impl ::core::marker::Copy for SocketErrorStatus {}
impl ::core::clone::Clone for SocketErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketErrorStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketErrorStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketErrorStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketErrorStatus;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketMessageType(pub i32);
impl SocketMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Utf8: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketMessageType {}
impl ::core::clone::Clone for SocketMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketMessageType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketMessageType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketMessageType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketMessageType;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketProtectionLevel(pub i32);
impl SocketProtectionLevel {
    pub const PlainSocket: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const SslAllowNullEncryption: Self = Self(2i32);
    pub const BluetoothEncryptionAllowNullAuthentication: Self = Self(3i32);
    pub const BluetoothEncryptionWithAuthentication: Self = Self(4i32);
    pub const Ssl3AllowWeakEncryption: Self = Self(5i32);
    pub const Tls10: Self = Self(6i32);
    pub const Tls11: Self = Self(7i32);
    pub const Tls12: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for SocketProtectionLevel {}
impl ::core::clone::Clone for SocketProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketProtectionLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketProtectionLevel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketProtectionLevel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketProtectionLevel;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketQualityOfService(pub i32);
impl SocketQualityOfService {
    pub const Normal: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
}
impl ::core::marker::Copy for SocketQualityOfService {}
impl ::core::clone::Clone for SocketQualityOfService {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketQualityOfService {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketQualityOfService {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketQualityOfService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketQualityOfService").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketQualityOfService {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketQualityOfService;i4)");
}
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SocketSslErrorSeverity(pub i32);
impl SocketSslErrorSeverity {
    pub const None: Self = Self(0i32);
    pub const Ignorable: Self = Self(1i32);
    pub const Fatal: Self = Self(2i32);
}
impl ::core::marker::Copy for SocketSslErrorSeverity {}
impl ::core::clone::Clone for SocketSslErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SocketSslErrorSeverity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SocketSslErrorSeverity {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SocketSslErrorSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketSslErrorSeverity").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketSslErrorSeverity {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketSslErrorSeverity;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
pub struct BandwidthStatistics {
    pub OutboundBitsPerSecond: u64,
    pub InboundBitsPerSecond: u64,
    pub OutboundBitsPerSecondInstability: u64,
    pub InboundBitsPerSecondInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
impl ::core::marker::Copy for BandwidthStatistics {}
impl ::core::clone::Clone for BandwidthStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BandwidthStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BandwidthStatistics").field("OutboundBitsPerSecond", &self.OutboundBitsPerSecond).field("InboundBitsPerSecond", &self.InboundBitsPerSecond).field("OutboundBitsPerSecondInstability", &self.OutboundBitsPerSecondInstability).field("InboundBitsPerSecondInstability", &self.InboundBitsPerSecondInstability).field("OutboundBandwidthPeaked", &self.OutboundBandwidthPeaked).field("InboundBandwidthPeaked", &self.InboundBandwidthPeaked).finish()
    }
}
impl ::windows::core::TypeKind for BandwidthStatistics {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for BandwidthStatistics {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.Sockets.BandwidthStatistics;u8;u8;u8;u8;b1;b1)");
}
impl ::core::cmp::PartialEq for BandwidthStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.OutboundBitsPerSecond == other.OutboundBitsPerSecond && self.InboundBitsPerSecond == other.InboundBitsPerSecond && self.OutboundBitsPerSecondInstability == other.OutboundBitsPerSecondInstability && self.InboundBitsPerSecondInstability == other.InboundBitsPerSecondInstability && self.OutboundBandwidthPeaked == other.OutboundBandwidthPeaked && self.InboundBandwidthPeaked == other.InboundBandwidthPeaked
    }
}
impl ::core::cmp::Eq for BandwidthStatistics {}
impl ::core::default::Default for BandwidthStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_Sockets\"`*"]
pub struct RoundTripTimeStatistics {
    pub Variance: u32,
    pub Max: u32,
    pub Min: u32,
    pub Sum: u32,
}
impl ::core::marker::Copy for RoundTripTimeStatistics {}
impl ::core::clone::Clone for RoundTripTimeStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RoundTripTimeStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RoundTripTimeStatistics").field("Variance", &self.Variance).field("Max", &self.Max).field("Min", &self.Min).field("Sum", &self.Sum).finish()
    }
}
impl ::windows::core::TypeKind for RoundTripTimeStatistics {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for RoundTripTimeStatistics {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.Sockets.RoundTripTimeStatistics;u4;u4;u4;u4)");
}
impl ::core::cmp::PartialEq for RoundTripTimeStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.Variance == other.Variance && self.Max == other.Max && self.Min == other.Min && self.Sum == other.Sum
    }
}
impl ::core::cmp::Eq for RoundTripTimeStatistics {}
impl ::core::default::Default for RoundTripTimeStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
