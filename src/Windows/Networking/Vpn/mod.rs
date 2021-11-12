#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnAppId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnAppId {
    type Vtable = IVpnAppId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b06a635_5c58_41d9_94a7_bfbcf1d8ca54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnAppId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnAppIdType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnAppIdType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnAppIdFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnAppIdFactory {
    type Vtable = IVpnAppIdFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46adfd2a_0aab_4fdb_821d_d3ddc919788b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnAppIdFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: VpnAppIdType, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannel {
    type Vtable = IVpnChannel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ac78d07_d1a8_4303_a091_c8d2e0915bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainoutertunneltransport: ::windows::core::RawPtr, optionaloutertunneltransport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, routescope: ::windows::core::RawPtr, namespacescope: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: ::windows::core::RawPtr, optionaloutertunneltransport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: VpnDataPathType, vpnpacketbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customprompt: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tunneltransport: ::windows::core::RawPtr, usetls12: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannel2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannel2 {
    type Vtable = IVpnChannel2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2255d165_993b_4629_ad60_f1c3f3537f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, custompromptelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credtype: VpnCredentialType, credoptions: u32, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credtype: VpnCredentialType, credoptions: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credtype: VpnCredentialType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        assignedclientipv4list: ::windows::core::RawPtr,
        assignedclientipv6list: ::windows::core::RawPtr,
        vpninterfaceid: ::windows::core::RawPtr,
        assignedroutes: ::windows::core::RawPtr,
        assignednamespace: ::windows::core::RawPtr,
        mtusize: u32,
        maxframesize: u32,
        reserved: bool,
        mainoutertunneltransport: ::windows::core::RawPtr,
        optionaloutertunneltransport: ::windows::core::RawPtr,
        assignedtrafficfilters: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannel4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannel4 {
    type Vtable = IVpnChannel4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7266ede_2937_419d_9570_486aebb81803);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transport: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, assignedclientipv4addresses: ::windows::core::RawPtr, assignedclientipv6addresses: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, transports: ::windows::core::RawPtr, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transport: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transport: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut super::Sockets::ControlChannelTriggerStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannel5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannel5 {
    type Vtable = IVpnChannel5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde7a0992_8384_4fbc_882c_1fd23124cd3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, decapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannel6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannel6 {
    type Vtable = IVpnChannel6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55843696_bd63_49c5_abca_5da77885551a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharedcontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannelActivityEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannelActivityEventArgs {
    type Vtable = IVpnChannelActivityEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa36c88f2_afdc_4775_855d_d4ac0a35fc55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelActivityEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannelActivityStateChangedArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannelActivityStateChangedArgs {
    type Vtable = IVpnChannelActivityStateChangedArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d750565_fdc0_4bbe_a23b_45fffc6d97a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelActivityStateChangedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannelConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannelConfiguration {
    type Vtable = IVpnChannelConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e2ddca2_2012_4fe4_b179_8c652c6d107e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnChannelConfiguration2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannelConfiguration2 {
    type Vtable = IVpnChannelConfiguration2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf30b574c_7824_471c_a118_63dbc93ae4c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnChannelStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnChannelStatics {
    type Vtable = IVpnChannelStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88eb062d_e818_4ffd_98a6_363e3736c95d);
}
impl IVpnChannelStatics {
    pub fn ProcessEventAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, thirdpartyplugin: Param0, event: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), thirdpartyplugin.into_param().abi(), event.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnChannelStatics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{88eb062d-e818-4ffd-98a6-363e3736c95d}");
}
impl ::core::convert::From<IVpnChannelStatics> for ::windows::core::IUnknown {
    fn from(value: IVpnChannelStatics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnChannelStatics> for ::windows::core::IUnknown {
    fn from(value: &IVpnChannelStatics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnChannelStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnChannelStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnChannelStatics> for ::windows::core::IInspectable {
    fn from(value: IVpnChannelStatics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnChannelStatics> for ::windows::core::IInspectable {
    fn from(value: &IVpnChannelStatics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnChannelStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnChannelStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, thirdpartyplugin: ::windows::core::RawPtr, event: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnCredential(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCredential {
    type Vtable = IVpnCredential_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e78af3_a46d_404b_8729_1832522853ac);
}
impl IVpnCredential {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn CertificateCredential(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    pub fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnCredential {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b7e78af3-a46d-404b-8729-1832522853ac}");
}
impl ::core::convert::From<IVpnCredential> for ::windows::core::IUnknown {
    fn from(value: IVpnCredential) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnCredential> for ::windows::core::IUnknown {
    fn from(value: &IVpnCredential) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnCredential> for ::windows::core::IInspectable {
    fn from(value: IVpnCredential) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnCredential> for ::windows::core::IInspectable {
    fn from(value: &IVpnCredential) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCredential_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomCheckBox(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomCheckBox {
    type Vtable = IVpnCustomCheckBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43878753_03c5_4e61_93d7_a957714c4282);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomCheckBox_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomComboBox(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomComboBox {
    type Vtable = IVpnCustomComboBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a24158e_dba1_4c6f_8270_dcf3c9761c4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomComboBox_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomEditBox(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomEditBox {
    type Vtable = IVpnCustomEditBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3002d9a0_cfbf_4c0b_8f3c_66f503c20b39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomEditBox_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomErrorBox(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomErrorBox {
    type Vtable = IVpnCustomErrorBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ec4efb2_c942_42af_b223_588b48328721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomErrorBox_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnCustomPrompt(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomPrompt {
    type Vtable = IVpnCustomPrompt_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b2ebe7b_87d5_433c_b4f6_eee6aa68a244);
}
impl IVpnCustomPrompt {
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnCustomPrompt {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9b2ebe7b-87d5-433c-b4f6-eee6aa68a244}");
}
impl ::core::convert::From<IVpnCustomPrompt> for ::windows::core::IUnknown {
    fn from(value: IVpnCustomPrompt) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnCustomPrompt> for ::windows::core::IUnknown {
    fn from(value: &IVpnCustomPrompt) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnCustomPrompt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnCustomPrompt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnCustomPrompt> for ::windows::core::IInspectable {
    fn from(value: IVpnCustomPrompt) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnCustomPrompt> for ::windows::core::IInspectable {
    fn from(value: &IVpnCustomPrompt) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnCustomPrompt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnCustomPrompt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPrompt_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomPromptBooleanInput(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomPromptBooleanInput {
    type Vtable = IVpnCustomPromptBooleanInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4c9a69e_ff47_4527_9f27_a49292019979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptBooleanInput_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnCustomPromptElement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomPromptElement {
    type Vtable = IVpnCustomPromptElement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73bd5638_6f04_404d_93dd_50a44924a38b);
}
impl IVpnCustomPromptElement {
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnCustomPromptElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{73bd5638-6f04-404d-93dd-50a44924a38b}");
}
impl ::core::convert::From<IVpnCustomPromptElement> for ::windows::core::IUnknown {
    fn from(value: IVpnCustomPromptElement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnCustomPromptElement> for ::windows::core::IUnknown {
    fn from(value: &IVpnCustomPromptElement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnCustomPromptElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnCustomPromptElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnCustomPromptElement> for ::windows::core::IInspectable {
    fn from(value: IVpnCustomPromptElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnCustomPromptElement> for ::windows::core::IInspectable {
    fn from(value: &IVpnCustomPromptElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnCustomPromptElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnCustomPromptElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptElement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomPromptOptionSelector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomPromptOptionSelector {
    type Vtable = IVpnCustomPromptOptionSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b8f34d9_8ec1_4e95_9a4e_7ba64d38f330);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptOptionSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomPromptText(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomPromptText {
    type Vtable = IVpnCustomPromptText_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bc8bdee_3a42_49a3_abdd_07b2edea752d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptText_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomPromptTextInput(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomPromptTextInput {
    type Vtable = IVpnCustomPromptTextInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9da9c75_913c_47d5_88ba_48fc48930235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptTextInput_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnCustomTextBox(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnCustomTextBox {
    type Vtable = IVpnCustomTextBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaa4c3ca_8f23_4d36_91f1_76d937827942);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomTextBox_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnDomainNameAssignment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnDomainNameAssignment {
    type Vtable = IVpnDomainNameAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4135b141_ccdb_49b5_9401_039a8ae767e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameAssignment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnDomainNameInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnDomainNameInfo {
    type Vtable = IVpnDomainNameInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad2eb82f_ea8e_4f7a_843e_1a87e32e1b9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnDomainNameType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnDomainNameType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnDomainNameInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnDomainNameInfo2 {
    type Vtable = IVpnDomainNameInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab871151_6c53_4828_9883_d886de104407);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnDomainNameInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnDomainNameInfoFactory {
    type Vtable = IVpnDomainNameInfoFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2507bb75_028f_4688_8d3a_c4531df37da8);
}
impl IVpnDomainNameInfoFactory {
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnDomainNameInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::HostName>>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::HostName>>>(&self, name: Param0, nametype: VpnDomainNameType, dnsserverlist: Param2, proxyserverlist: Param3) -> ::windows::core::Result<VpnDomainNameInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), nametype, dnsserverlist.into_param().abi(), proxyserverlist.into_param().abi(), &mut result__).from_abi::<VpnDomainNameInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnDomainNameInfoFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2507bb75-028f-4688-8d3a-c4531df37da8}");
}
impl ::core::convert::From<IVpnDomainNameInfoFactory> for ::windows::core::IUnknown {
    fn from(value: IVpnDomainNameInfoFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnDomainNameInfoFactory> for ::windows::core::IUnknown {
    fn from(value: &IVpnDomainNameInfoFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnDomainNameInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnDomainNameInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnDomainNameInfoFactory> for ::windows::core::IInspectable {
    fn from(value: IVpnDomainNameInfoFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnDomainNameInfoFactory> for ::windows::core::IInspectable {
    fn from(value: &IVpnDomainNameInfoFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnDomainNameInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnDomainNameInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnForegroundActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnForegroundActivatedEventArgs {
    type Vtable = IVpnForegroundActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85b465b0_cadb_4d70_ac92_543a24dc9ebc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnForegroundActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnForegroundActivationOperation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnForegroundActivationOperation {
    type Vtable = IVpnForegroundActivationOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e010d57_f17a_4bd5_9b6d_f984f1297d3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnForegroundActivationOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnInterfaceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnInterfaceId {
    type Vtable = IVpnInterfaceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d1011);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnInterfaceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id_array_size: *mut u32, id: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnInterfaceIdFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnInterfaceIdFactory {
    type Vtable = IVpnInterfaceIdFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d1000);
}
impl IVpnInterfaceIdFactory {
    pub fn CreateVpnInterfaceId(&self, address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), address.len() as u32, ::core::mem::transmute(address.as_ptr()), &mut result__).from_abi::<VpnInterfaceId>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnInterfaceIdFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9e2ddca2-1712-4ce4-b179-8c652c6d1000}");
}
impl ::core::convert::From<IVpnInterfaceIdFactory> for ::windows::core::IUnknown {
    fn from(value: IVpnInterfaceIdFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnInterfaceIdFactory> for ::windows::core::IUnknown {
    fn from(value: &IVpnInterfaceIdFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnInterfaceIdFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnInterfaceIdFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnInterfaceIdFactory> for ::windows::core::IInspectable {
    fn from(value: IVpnInterfaceIdFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnInterfaceIdFactory> for ::windows::core::IInspectable {
    fn from(value: &IVpnInterfaceIdFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnInterfaceIdFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnInterfaceIdFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnInterfaceIdFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, address_array_size: u32, address: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnManagementAgent(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnManagementAgent {
    type Vtable = IVpnManagementAgent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x193696cd_a5c4_4abe_852b_785be4cb3e34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnManagementAgent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnNamespaceAssignment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnNamespaceAssignment {
    type Vtable = IVpnNamespaceAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f7db18_307d_4c0e_bd62_8fa270bbadd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNamespaceAssignment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnNamespaceInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnNamespaceInfo {
    type Vtable = IVpnNamespaceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30edfb43_444f_44c5_8167_a35a91f1af94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNamespaceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnNamespaceInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnNamespaceInfoFactory {
    type Vtable = IVpnNamespaceInfoFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3e951a_b0ce_442b_acbb_5f99b202c31c);
}
impl IVpnNamespaceInfoFactory {
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnNamespaceInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<super::HostName>>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<super::HostName>>>(&self, name: Param0, dnsserverlist: Param1, proxyserverlist: Param2) -> ::windows::core::Result<VpnNamespaceInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), dnsserverlist.into_param().abi(), proxyserverlist.into_param().abi(), &mut result__).from_abi::<VpnNamespaceInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnNamespaceInfoFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cb3e951a-b0ce-442b-acbb-5f99b202c31c}");
}
impl ::core::convert::From<IVpnNamespaceInfoFactory> for ::windows::core::IUnknown {
    fn from(value: IVpnNamespaceInfoFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnNamespaceInfoFactory> for ::windows::core::IUnknown {
    fn from(value: &IVpnNamespaceInfoFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnNamespaceInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnNamespaceInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnNamespaceInfoFactory> for ::windows::core::IInspectable {
    fn from(value: IVpnNamespaceInfoFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnNamespaceInfoFactory> for ::windows::core::IInspectable {
    fn from(value: &IVpnNamespaceInfoFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnNamespaceInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnNamespaceInfoFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNamespaceInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnNativeProfile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnNativeProfile {
    type Vtable = IVpnNativeProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4aee29e_6417_4333_9842_f0a66db69802);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNativeProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnNativeProtocolType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnNativeProtocolType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnNativeProfile2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnNativeProfile2 {
    type Vtable = IVpnNativeProfile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fec2467_cdb5_4ac7_b5a3_0afb5ec47682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNativeProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPacketBuffer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPacketBuffer {
    type Vtable = IVpnPacketBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f891fc_4d5c_4a63_b70d_4e307eacce55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPacketBuffer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPacketBuffer2 {
    type Vtable = IVpnPacketBuffer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x665e91f0_8805_4bf5_a619_2e84882e6b4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBuffer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPacketBuffer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPacketBuffer3 {
    type Vtable = IVpnPacketBuffer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe256072f_107b_4c40_b127_5bc53e0ad960);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBuffer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnPacketBufferFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPacketBufferFactory {
    type Vtable = IVpnPacketBufferFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d9999);
}
impl IVpnPacketBufferFactory {
    pub fn CreateVpnPacketBuffer<'a, Param0: ::windows::core::IntoParam<'a, VpnPacketBuffer>>(&self, parentbuffer: Param0, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), parentbuffer.into_param().abi(), offset, length, &mut result__).from_abi::<VpnPacketBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnPacketBufferFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9e2ddca2-1712-4ce4-b179-8c652c6d9999}");
}
impl ::core::convert::From<IVpnPacketBufferFactory> for ::windows::core::IUnknown {
    fn from(value: IVpnPacketBufferFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnPacketBufferFactory> for ::windows::core::IUnknown {
    fn from(value: &IVpnPacketBufferFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnPacketBufferFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnPacketBufferFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnPacketBufferFactory> for ::windows::core::IInspectable {
    fn from(value: IVpnPacketBufferFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnPacketBufferFactory> for ::windows::core::IInspectable {
    fn from(value: &IVpnPacketBufferFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnPacketBufferFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnPacketBufferFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBufferFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parentbuffer: ::windows::core::RawPtr, offset: u32, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPacketBufferList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPacketBufferList {
    type Vtable = IVpnPacketBufferList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f891fc_4d5c_4a63_b70d_4e307eacce77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBufferList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPacketBufferList2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPacketBufferList2 {
    type Vtable = IVpnPacketBufferList2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e7acfe5_ea1e_482a_8d98_c065f57d89ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBufferList2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPickedCredential(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPickedCredential {
    type Vtable = IVpnPickedCredential_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a793ac7_8854_4e52_ad97_24dd9a842bce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPickedCredential_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnPlugIn(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPlugIn {
    type Vtable = IVpnPlugIn_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb78d07_d0a8_4703_a091_c8c2c0915bc4);
}
impl IVpnPlugIn {
    pub fn Connect<'a, Param0: ::windows::core::IntoParam<'a, VpnChannel>>(&self, channel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), channel.into_param().abi()).ok() }
    }
    pub fn Disconnect<'a, Param0: ::windows::core::IntoParam<'a, VpnChannel>>(&self, channel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), channel.into_param().abi()).ok() }
    }
    pub fn GetKeepAlivePayload<'a, Param0: ::windows::core::IntoParam<'a, VpnChannel>>(&self, channel: Param0, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), channel.into_param().abi(), keepalivepacket as *mut _ as _).ok() }
    }
    pub fn Encapsulate<'a, Param0: ::windows::core::IntoParam<'a, VpnChannel>, Param1: ::windows::core::IntoParam<'a, VpnPacketBufferList>, Param2: ::windows::core::IntoParam<'a, VpnPacketBufferList>>(&self, channel: Param0, packets: Param1, encapulatedpackets: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), channel.into_param().abi(), packets.into_param().abi(), encapulatedpackets.into_param().abi()).ok() }
    }
    pub fn Decapsulate<'a, Param0: ::windows::core::IntoParam<'a, VpnChannel>, Param1: ::windows::core::IntoParam<'a, VpnPacketBuffer>, Param2: ::windows::core::IntoParam<'a, VpnPacketBufferList>, Param3: ::windows::core::IntoParam<'a, VpnPacketBufferList>>(&self, channel: Param0, encapbuffer: Param1, decapsulatedpackets: Param2, controlpacketstosend: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), channel.into_param().abi(), encapbuffer.into_param().abi(), decapsulatedpackets.into_param().abi(), controlpacketstosend.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnPlugIn {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ceb78d07-d0a8-4703-a091-c8c2c0915bc4}");
}
impl ::core::convert::From<IVpnPlugIn> for ::windows::core::IUnknown {
    fn from(value: IVpnPlugIn) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnPlugIn> for ::windows::core::IUnknown {
    fn from(value: &IVpnPlugIn) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnPlugIn> for ::windows::core::IInspectable {
    fn from(value: IVpnPlugIn) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnPlugIn> for ::windows::core::IInspectable {
    fn from(value: &IVpnPlugIn) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPlugIn_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, channel: ::windows::core::RawPtr, keepalivepacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, channel: ::windows::core::RawPtr, packets: ::windows::core::RawPtr, encapulatedpackets: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, channel: ::windows::core::RawPtr, encapbuffer: ::windows::core::RawPtr, decapsulatedpackets: ::windows::core::RawPtr, controlpacketstosend: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPlugInProfile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPlugInProfile {
    type Vtable = IVpnPlugInProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0edf0da4_4f00_4589_8d7b_4bf988f6542c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPlugInProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnPlugInProfile2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnPlugInProfile2 {
    type Vtable = IVpnPlugInProfile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x611c4892_cf94_4ad6_ba99_00f4ff34565e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPlugInProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnProfile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnProfile {
    type Vtable = IVpnProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7875b751_b0d7_43db_8a93_d3fe2479e56a);
}
impl IVpnProfile {
    pub fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetProfileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnAppId>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>(result__)
        }
    }
    pub fn RememberCredentials(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7875b751-b0d7-43db-8a93-d3fe2479e56a}");
}
impl ::core::convert::From<IVpnProfile> for ::windows::core::IUnknown {
    fn from(value: IVpnProfile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnProfile> for ::windows::core::IUnknown {
    fn from(value: &IVpnProfile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnProfile> for ::windows::core::IInspectable {
    fn from(value: IVpnProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnProfile> for ::windows::core::IInspectable {
    fn from(value: &IVpnProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnRoute(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnRoute {
    type Vtable = IVpnRoute_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5731b83_0969_4699_938e_7776db29cfb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnRoute_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnRouteAssignment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnRouteAssignment {
    type Vtable = IVpnRouteAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb64de22_ce39_4a76_9550_f61039f80e48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnRouteAssignment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVpnRouteFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnRouteFactory {
    type Vtable = IVpnRouteFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdeab5ff_45cf_4b99_83fb_db3bc2672b02);
}
impl IVpnRouteFactory {
    pub fn CreateVpnRoute<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>>(&self, address: Param0, prefixsize: u8) -> ::windows::core::Result<VpnRoute> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), address.into_param().abi(), prefixsize, &mut result__).from_abi::<VpnRoute>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVpnRouteFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bdeab5ff-45cf-4b99-83fb-db3bc2672b02}");
}
impl ::core::convert::From<IVpnRouteFactory> for ::windows::core::IUnknown {
    fn from(value: IVpnRouteFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVpnRouteFactory> for ::windows::core::IUnknown {
    fn from(value: &IVpnRouteFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVpnRouteFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVpnRouteFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVpnRouteFactory> for ::windows::core::IInspectable {
    fn from(value: IVpnRouteFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVpnRouteFactory> for ::windows::core::IInspectable {
    fn from(value: &IVpnRouteFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVpnRouteFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVpnRouteFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnRouteFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, address: ::windows::core::RawPtr, prefixsize: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnSystemHealth(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnSystemHealth {
    type Vtable = IVpnSystemHealth_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99a8f8af_c0ee_4e75_817a_f231aee5123d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnSystemHealth_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnTrafficFilter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnTrafficFilter {
    type Vtable = IVpnTrafficFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f691b60_6c9f_47f5_ac36_bb1b042e2c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnTrafficFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnIPProtocol) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnIPProtocol) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnTrafficFilterAssignment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnTrafficFilterAssignment {
    type Vtable = IVpnTrafficFilterAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ccd45c_e664_471e_89cd_601603b9e0f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnTrafficFilterAssignment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVpnTrafficFilterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVpnTrafficFilterFactory {
    type Vtable = IVpnTrafficFilterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x480d41d5_7f99_474c_86ee_96df168318f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnTrafficFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnAppId(pub ::windows::core::IInspectable);
impl VpnAppId {
    pub fn Type(&self) -> ::windows::core::Result<VpnAppIdType> {
        let this = self;
        unsafe {
            let mut result__: VpnAppIdType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnAppIdType>(result__)
        }
    }
    pub fn SetType(&self, value: VpnAppIdType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(r#type: VpnAppIdType, value: Param1) -> ::windows::core::Result<VpnAppId> {
        Self::IVpnAppIdFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, value.into_param().abi(), &mut result__).from_abi::<VpnAppId>(result__)
        })
    }
    pub fn IVpnAppIdFactory<R, F: FnOnce(&IVpnAppIdFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnAppId, IVpnAppIdFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnAppId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnAppId;{7b06a635-5c58-41d9-94a7-bfbcf1d8ca54})");
}
unsafe impl ::windows::core::Interface for VpnAppId {
    type Vtable = IVpnAppId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b06a635_5c58_41d9_94a7_bfbcf1d8ca54);
}
impl ::windows::core::RuntimeName for VpnAppId {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnAppId";
}
impl ::core::convert::From<VpnAppId> for ::windows::core::IUnknown {
    fn from(value: VpnAppId) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnAppId> for ::windows::core::IUnknown {
    fn from(value: &VpnAppId) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnAppId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnAppId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnAppId> for ::windows::core::IInspectable {
    fn from(value: VpnAppId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnAppId> for ::windows::core::IInspectable {
    fn from(value: &VpnAppId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnAppId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnAppId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnAppId {}
unsafe impl ::core::marker::Sync for VpnAppId {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnAppIdType(pub i32);
impl VpnAppIdType {
    pub const PackageFamilyName: VpnAppIdType = VpnAppIdType(0i32);
    pub const FullyQualifiedBinaryName: VpnAppIdType = VpnAppIdType(1i32);
    pub const FilePath: VpnAppIdType = VpnAppIdType(2i32);
}
impl ::core::convert::From<i32> for VpnAppIdType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnAppIdType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnAppIdType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnAppIdType;i4)");
}
impl ::windows::core::DefaultType for VpnAppIdType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnAuthenticationMethod(pub i32);
impl VpnAuthenticationMethod {
    pub const Mschapv2: VpnAuthenticationMethod = VpnAuthenticationMethod(0i32);
    pub const Eap: VpnAuthenticationMethod = VpnAuthenticationMethod(1i32);
    pub const Certificate: VpnAuthenticationMethod = VpnAuthenticationMethod(2i32);
    pub const PresharedKey: VpnAuthenticationMethod = VpnAuthenticationMethod(3i32);
}
impl ::core::convert::From<i32> for VpnAuthenticationMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnAuthenticationMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnAuthenticationMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnAuthenticationMethod;i4)");
}
impl ::windows::core::DefaultType for VpnAuthenticationMethod {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnChannel(pub ::windows::core::IInspectable);
impl VpnChannel {
    pub fn AssociateTransport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, mainoutertunneltransport: Param0, optionaloutertunneltransport: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mainoutertunneltransport.into_param().abi(), optionaloutertunneltransport.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Start<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>,
        Param2: ::windows::core::IntoParam<'a, VpnInterfaceId>,
        Param3: ::windows::core::IntoParam<'a, VpnRouteAssignment>,
        Param4: ::windows::core::IntoParam<'a, VpnNamespaceAssignment>,
        Param8: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param9: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        assignedclientipv4list: Param0,
        assignedclientipv6list: Param1,
        vpninterfaceid: Param2,
        routescope: Param3,
        namespacescope: Param4,
        mtusize: u32,
        maxframesize: u32,
        optimizeforlowcostnetwork: bool,
        mainoutertunneltransport: Param8,
        optionaloutertunneltransport: Param9,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                assignedclientipv4list.into_param().abi(),
                assignedclientipv6list.into_param().abi(),
                vpninterfaceid.into_param().abi(),
                routescope.into_param().abi(),
                namespacescope.into_param().abi(),
                mtusize,
                maxframesize,
                optimizeforlowcostnetwork,
                mainoutertunneltransport.into_param().abi(),
                optionaloutertunneltransport.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn RequestCredentials<'a, Param3: ::windows::core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: Param3) -> ::windows::core::Result<VpnPickedCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), credtype, isretry, issinglesignoncredential, certificate.into_param().abi(), &mut result__).from_abi::<VpnPickedCredential>(result__)
        }
    }
    pub fn RequestVpnPacketBuffer(&self, r#type: VpnDataPathType, vpnpacketbuffer: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), r#type, vpnpacketbuffer as *mut _ as _).ok() }
    }
    pub fn LogDiagnosticMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, message: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), message.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows::core::Result<VpnChannelConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnChannelConfiguration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ActivityChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivityChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn SetPlugInContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PlugInContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SystemHealth(&self) -> ::windows::core::Result<VpnSystemHealth> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnSystemHealth>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestCustomPrompt<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt>>>(&self, customprompt: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), customprompt.into_param().abi()).ok() }
    }
    pub fn SetErrorMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, message: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), message.into_param().abi()).ok() }
    }
    pub fn SetAllowedSslTlsVersions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, tunneltransport: Param0, usetls12: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), tunneltransport.into_param().abi(), usetls12).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithMainTransport<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>,
        Param2: ::windows::core::IntoParam<'a, VpnInterfaceId>,
        Param3: ::windows::core::IntoParam<'a, VpnRouteAssignment>,
        Param4: ::windows::core::IntoParam<'a, VpnDomainNameAssignment>,
        Param8: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        assignedclientipv4list: Param0,
        assignedclientipv6list: Param1,
        vpninterfaceid: Param2,
        assignedroutes: Param3,
        assigneddomainname: Param4,
        mtusize: u32,
        maxframesize: u32,
        reserved: bool,
        mainoutertunneltransport: Param8,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                assignedclientipv4list.into_param().abi(),
                assignedclientipv6list.into_param().abi(),
                vpninterfaceid.into_param().abi(),
                assignedroutes.into_param().abi(),
                assigneddomainname.into_param().abi(),
                mtusize,
                maxframesize,
                reserved,
                mainoutertunneltransport.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartExistingTransports<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>, Param2: ::windows::core::IntoParam<'a, VpnInterfaceId>, Param3: ::windows::core::IntoParam<'a, VpnRouteAssignment>, Param4: ::windows::core::IntoParam<'a, VpnDomainNameAssignment>>(
        &self,
        assignedclientipv4list: Param0,
        assignedclientipv6list: Param1,
        vpninterfaceid: Param2,
        assignedroutes: Param3,
        assigneddomainname: Param4,
        mtusize: u32,
        maxframesize: u32,
        reserved: bool,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), assignedclientipv4list.into_param().abi(), assignedclientipv6list.into_param().abi(), vpninterfaceid.into_param().abi(), assignedroutes.into_param().abi(), assigneddomainname.into_param().abi(), mtusize, maxframesize, reserved).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ActivityStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivityStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetVpnSendPacketBuffer(&self) -> ::windows::core::Result<VpnPacketBuffer> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnPacketBuffer>(result__)
        }
    }
    pub fn GetVpnReceivePacketBuffer(&self) -> ::windows::core::Result<VpnPacketBuffer> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnPacketBuffer>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestCustomPromptAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement>>>(&self, custompromptelement: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), custompromptelement.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub fn RequestCredentialsWithCertificateAsync<'a, Param2: ::windows::core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, credtype: VpnCredentialType, credoptions: u32, certificate: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), credtype, credoptions, certificate.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnCredential>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCredentialsWithOptionsAsync(&self, credtype: VpnCredentialType, credoptions: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), credtype, credoptions, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnCredential>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCredentialsSimpleAsync(&self, credtype: VpnCredentialType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), credtype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnCredential>>(result__)
        }
    }
    pub fn TerminateConnection<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, message: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), message.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithTrafficFilter<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<super::HostName>>,
        Param2: ::windows::core::IntoParam<'a, VpnInterfaceId>,
        Param3: ::windows::core::IntoParam<'a, VpnRouteAssignment>,
        Param4: ::windows::core::IntoParam<'a, VpnDomainNameAssignment>,
        Param8: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param9: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param10: ::windows::core::IntoParam<'a, VpnTrafficFilterAssignment>,
    >(
        &self,
        assignedclientipv4list: Param0,
        assignedclientipv6list: Param1,
        vpninterfaceid: Param2,
        assignedroutes: Param3,
        assignednamespace: Param4,
        mtusize: u32,
        maxframesize: u32,
        reserved: bool,
        mainoutertunneltransport: Param8,
        optionaloutertunneltransport: Param9,
        assignedtrafficfilters: Param10,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                assignedclientipv4list.into_param().abi(),
                assignedclientipv6list.into_param().abi(),
                vpninterfaceid.into_param().abi(),
                assignedroutes.into_param().abi(),
                assignednamespace.into_param().abi(),
                mtusize,
                maxframesize,
                reserved,
                mainoutertunneltransport.into_param().abi(),
                optionaloutertunneltransport.into_param().abi(),
                assignedtrafficfilters.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ProcessEventAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(thirdpartyplugin: Param0, event: Param1) -> ::windows::core::Result<()> {
        Self::IVpnChannelStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), thirdpartyplugin.into_param().abi(), event.into_param().abi()).ok() })
    }
    pub fn AddAndAssociateTransport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, transport: Param0, context: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), transport.into_param().abi(), context.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithMultipleTransports<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::HostName>>,
        Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::HostName>>,
        Param2: ::windows::core::IntoParam<'a, VpnInterfaceId>,
        Param3: ::windows::core::IntoParam<'a, VpnRouteAssignment>,
        Param4: ::windows::core::IntoParam<'a, VpnDomainNameAssignment>,
        Param8: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>,
        Param9: ::windows::core::IntoParam<'a, VpnTrafficFilterAssignment>,
    >(
        &self,
        assignedclientipv4addresses: Param0,
        assignedclientipv6addresses: Param1,
        vpninterfaceid: Param2,
        assignedroutes: Param3,
        assignednamespace: Param4,
        mtusize: u32,
        maxframesize: u32,
        reserved: bool,
        transports: Param8,
        assignedtrafficfilters: Param9,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                assignedclientipv4addresses.into_param().abi(),
                assignedclientipv6addresses.into_param().abi(),
                vpninterfaceid.into_param().abi(),
                assignedroutes.into_param().abi(),
                assignednamespace.into_param().abi(),
                mtusize,
                maxframesize,
                reserved,
                transports.into_param().abi(),
                assignedtrafficfilters.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReplaceAndAssociateTransport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, transport: Param0, context: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), transport.into_param().abi(), context.into_param().abi()).ok() }
    }
    pub fn StartReconnectingTransport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, transport: Param0, context: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), transport.into_param().abi(), context.into_param().abi()).ok() }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn GetSlotTypeForTransportContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, context: Param0) -> ::windows::core::Result<super::Sockets::ControlChannelTriggerStatus> {
        let this = &::windows::core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe {
            let mut result__: super::Sockets::ControlChannelTriggerStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::Sockets::ControlChannelTriggerStatus>(result__)
        }
    }
    pub fn CurrentRequestTransportContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn AppendVpnReceivePacketBuffer<'a, Param0: ::windows::core::IntoParam<'a, VpnPacketBuffer>>(&self, decapsulatedpacketbuffer: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), decapsulatedpacketbuffer.into_param().abi()).ok() }
    }
    pub fn AppendVpnSendPacketBuffer<'a, Param0: ::windows::core::IntoParam<'a, VpnPacketBuffer>>(&self, encapsulatedpacketbuffer: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), encapsulatedpacketbuffer.into_param().abi()).ok() }
    }
    pub fn FlushVpnReceivePacketBuffers(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn FlushVpnSendPacketBuffers(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivateForeground<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, packagerelativeappid: Param0, sharedcontext: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IVpnChannel6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagerelativeappid.into_param().abi(), sharedcontext.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn IVpnChannelStatics<R, F: FnOnce(&IVpnChannelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnChannel, IVpnChannelStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannel;{4ac78d07-d1a8-4303-a091-c8d2e0915bc3})");
}
unsafe impl ::windows::core::Interface for VpnChannel {
    type Vtable = IVpnChannel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ac78d07_d1a8_4303_a091_c8d2e0915bc3);
}
impl ::windows::core::RuntimeName for VpnChannel {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannel";
}
impl ::core::convert::From<VpnChannel> for ::windows::core::IUnknown {
    fn from(value: VpnChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnChannel> for ::windows::core::IUnknown {
    fn from(value: &VpnChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnChannel> for ::windows::core::IInspectable {
    fn from(value: VpnChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnChannel> for ::windows::core::IInspectable {
    fn from(value: &VpnChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnChannel {}
unsafe impl ::core::marker::Sync for VpnChannel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnChannelActivityEventArgs(pub ::windows::core::IInspectable);
impl VpnChannelActivityEventArgs {
    pub fn Type(&self) -> ::windows::core::Result<VpnChannelActivityEventType> {
        let this = self;
        unsafe {
            let mut result__: VpnChannelActivityEventType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnChannelActivityEventType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnChannelActivityEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannelActivityEventArgs;{a36c88f2-afdc-4775-855d-d4ac0a35fc55})");
}
unsafe impl ::windows::core::Interface for VpnChannelActivityEventArgs {
    type Vtable = IVpnChannelActivityEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa36c88f2_afdc_4775_855d_d4ac0a35fc55);
}
impl ::windows::core::RuntimeName for VpnChannelActivityEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelActivityEventArgs";
}
impl ::core::convert::From<VpnChannelActivityEventArgs> for ::windows::core::IUnknown {
    fn from(value: VpnChannelActivityEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnChannelActivityEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VpnChannelActivityEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnChannelActivityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnChannelActivityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnChannelActivityEventArgs> for ::windows::core::IInspectable {
    fn from(value: VpnChannelActivityEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnChannelActivityEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VpnChannelActivityEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnChannelActivityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnChannelActivityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnChannelActivityEventArgs {}
unsafe impl ::core::marker::Sync for VpnChannelActivityEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnChannelActivityEventType(pub i32);
impl VpnChannelActivityEventType {
    pub const Idle: VpnChannelActivityEventType = VpnChannelActivityEventType(0i32);
    pub const Active: VpnChannelActivityEventType = VpnChannelActivityEventType(1i32);
}
impl ::core::convert::From<i32> for VpnChannelActivityEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnChannelActivityEventType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnChannelActivityEventType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnChannelActivityEventType;i4)");
}
impl ::windows::core::DefaultType for VpnChannelActivityEventType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnChannelActivityStateChangedArgs(pub ::windows::core::IInspectable);
impl VpnChannelActivityStateChangedArgs {
    pub fn ActivityState(&self) -> ::windows::core::Result<VpnChannelActivityEventType> {
        let this = self;
        unsafe {
            let mut result__: VpnChannelActivityEventType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnChannelActivityEventType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnChannelActivityStateChangedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs;{3d750565-fdc0-4bbe-a23b-45fffc6d97a1})");
}
unsafe impl ::windows::core::Interface for VpnChannelActivityStateChangedArgs {
    type Vtable = IVpnChannelActivityStateChangedArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d750565_fdc0_4bbe_a23b_45fffc6d97a1);
}
impl ::windows::core::RuntimeName for VpnChannelActivityStateChangedArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs";
}
impl ::core::convert::From<VpnChannelActivityStateChangedArgs> for ::windows::core::IUnknown {
    fn from(value: VpnChannelActivityStateChangedArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnChannelActivityStateChangedArgs> for ::windows::core::IUnknown {
    fn from(value: &VpnChannelActivityStateChangedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnChannelActivityStateChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnChannelActivityStateChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnChannelActivityStateChangedArgs> for ::windows::core::IInspectable {
    fn from(value: VpnChannelActivityStateChangedArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnChannelActivityStateChangedArgs> for ::windows::core::IInspectable {
    fn from(value: &VpnChannelActivityStateChangedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnChannelActivityStateChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnChannelActivityStateChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnChannelActivityStateChangedArgs {}
unsafe impl ::core::marker::Sync for VpnChannelActivityStateChangedArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnChannelConfiguration(pub ::windows::core::IInspectable);
impl VpnChannelConfiguration {
    pub fn ServerServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerHostNameList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::HostName>>(result__)
        }
    }
    pub fn CustomField(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ServerUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IVpnChannelConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnChannelConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannelConfiguration;{0e2ddca2-2012-4fe4-b179-8c652c6d107e})");
}
unsafe impl ::windows::core::Interface for VpnChannelConfiguration {
    type Vtable = IVpnChannelConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e2ddca2_2012_4fe4_b179_8c652c6d107e);
}
impl ::windows::core::RuntimeName for VpnChannelConfiguration {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelConfiguration";
}
impl ::core::convert::From<VpnChannelConfiguration> for ::windows::core::IUnknown {
    fn from(value: VpnChannelConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnChannelConfiguration> for ::windows::core::IUnknown {
    fn from(value: &VpnChannelConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnChannelConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnChannelConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnChannelConfiguration> for ::windows::core::IInspectable {
    fn from(value: VpnChannelConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnChannelConfiguration> for ::windows::core::IInspectable {
    fn from(value: &VpnChannelConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnChannelConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnChannelConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnChannelConfiguration {}
unsafe impl ::core::marker::Sync for VpnChannelConfiguration {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnChannelRequestCredentialsOptions(pub u32);
impl VpnChannelRequestCredentialsOptions {
    pub const None: VpnChannelRequestCredentialsOptions = VpnChannelRequestCredentialsOptions(0u32);
    pub const Retrying: VpnChannelRequestCredentialsOptions = VpnChannelRequestCredentialsOptions(1u32);
    pub const UseForSingleSignIn: VpnChannelRequestCredentialsOptions = VpnChannelRequestCredentialsOptions(2u32);
}
impl ::core::convert::From<u32> for VpnChannelRequestCredentialsOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnChannelRequestCredentialsOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnChannelRequestCredentialsOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnChannelRequestCredentialsOptions;u4)");
}
impl ::windows::core::DefaultType for VpnChannelRequestCredentialsOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for VpnChannelRequestCredentialsOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for VpnChannelRequestCredentialsOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCredential(pub ::windows::core::IInspectable);
impl VpnCredential {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn CertificateCredential(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    pub fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCredential {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCredential;{b7e78af3-a46d-404b-8729-1832522853ac})");
}
unsafe impl ::windows::core::Interface for VpnCredential {
    type Vtable = IVpnCredential_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e78af3_a46d_404b_8729_1832522853ac);
}
impl ::windows::core::RuntimeName for VpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCredential";
}
impl ::core::convert::From<VpnCredential> for ::windows::core::IUnknown {
    fn from(value: VpnCredential) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCredential> for ::windows::core::IUnknown {
    fn from(value: &VpnCredential) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCredential> for ::windows::core::IInspectable {
    fn from(value: VpnCredential) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCredential> for ::windows::core::IInspectable {
    fn from(value: &VpnCredential) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VpnCredential> for IVpnCredential {
    fn from(value: VpnCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VpnCredential> for IVpnCredential {
    fn from(value: &VpnCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCredential> for VpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCredential> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCredential> for &VpnCredential {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCredential> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VpnCredential {}
unsafe impl ::core::marker::Sync for VpnCredential {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnCredentialType(pub i32);
impl VpnCredentialType {
    pub const UsernamePassword: VpnCredentialType = VpnCredentialType(0i32);
    pub const UsernameOtpPin: VpnCredentialType = VpnCredentialType(1i32);
    pub const UsernamePasswordAndPin: VpnCredentialType = VpnCredentialType(2i32);
    pub const UsernamePasswordChange: VpnCredentialType = VpnCredentialType(3i32);
    pub const SmartCard: VpnCredentialType = VpnCredentialType(4i32);
    pub const ProtectedCertificate: VpnCredentialType = VpnCredentialType(5i32);
    pub const UnProtectedCertificate: VpnCredentialType = VpnCredentialType(6i32);
}
impl ::core::convert::From<i32> for VpnCredentialType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnCredentialType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnCredentialType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnCredentialType;i4)");
}
impl ::windows::core::DefaultType for VpnCredentialType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomCheckBox(pub ::windows::core::IInspectable);
impl VpnCustomCheckBox {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomCheckBox, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetInitialCheckState(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InitialCheckState(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Checked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomCheckBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomCheckBox;{43878753-03c5-4e61-93d7-a957714c4282})");
}
unsafe impl ::windows::core::Interface for VpnCustomCheckBox {
    type Vtable = IVpnCustomCheckBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43878753_03c5_4e61_93d7_a957714c4282);
}
impl ::windows::core::RuntimeName for VpnCustomCheckBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomCheckBox";
}
impl ::core::convert::From<VpnCustomCheckBox> for ::windows::core::IUnknown {
    fn from(value: VpnCustomCheckBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomCheckBox> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomCheckBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomCheckBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomCheckBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomCheckBox> for ::windows::core::IInspectable {
    fn from(value: VpnCustomCheckBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomCheckBox> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomCheckBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomCheckBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomCheckBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomCheckBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomCheckBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomCheckBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomCheckBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for VpnCustomCheckBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for &VpnCustomCheckBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::core::convert::TryInto::<IVpnCustomPrompt>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomCheckBox {}
unsafe impl ::core::marker::Sync for VpnCustomCheckBox {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomComboBox(pub ::windows::core::IInspectable);
impl VpnCustomComboBox {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomComboBox, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetOptionsText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionsText(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Selected(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomComboBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomComboBox;{9a24158e-dba1-4c6f-8270-dcf3c9761c4c})");
}
unsafe impl ::windows::core::Interface for VpnCustomComboBox {
    type Vtable = IVpnCustomComboBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a24158e_dba1_4c6f_8270_dcf3c9761c4c);
}
impl ::windows::core::RuntimeName for VpnCustomComboBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomComboBox";
}
impl ::core::convert::From<VpnCustomComboBox> for ::windows::core::IUnknown {
    fn from(value: VpnCustomComboBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomComboBox> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomComboBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomComboBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomComboBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomComboBox> for ::windows::core::IInspectable {
    fn from(value: VpnCustomComboBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomComboBox> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomComboBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomComboBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomComboBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomComboBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomComboBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomComboBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomComboBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for VpnCustomComboBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for &VpnCustomComboBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::core::convert::TryInto::<IVpnCustomPrompt>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomComboBox {}
unsafe impl ::core::marker::Sync for VpnCustomComboBox {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomEditBox(pub ::windows::core::IInspectable);
impl VpnCustomEditBox {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomEditBox, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetDefaultText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DefaultText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNoEcho(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NoEcho(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomEditBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomEditBox;{3002d9a0-cfbf-4c0b-8f3c-66f503c20b39})");
}
unsafe impl ::windows::core::Interface for VpnCustomEditBox {
    type Vtable = IVpnCustomEditBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3002d9a0_cfbf_4c0b_8f3c_66f503c20b39);
}
impl ::windows::core::RuntimeName for VpnCustomEditBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomEditBox";
}
impl ::core::convert::From<VpnCustomEditBox> for ::windows::core::IUnknown {
    fn from(value: VpnCustomEditBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomEditBox> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomEditBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomEditBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomEditBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomEditBox> for ::windows::core::IInspectable {
    fn from(value: VpnCustomEditBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomEditBox> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomEditBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomEditBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomEditBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomEditBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomEditBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomEditBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomEditBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for VpnCustomEditBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for &VpnCustomEditBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::core::convert::TryInto::<IVpnCustomPrompt>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomEditBox {}
unsafe impl ::core::marker::Sync for VpnCustomEditBox {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomErrorBox(pub ::windows::core::IInspectable);
impl VpnCustomErrorBox {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomErrorBox, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomErrorBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomErrorBox;{9ec4efb2-c942-42af-b223-588b48328721})");
}
unsafe impl ::windows::core::Interface for VpnCustomErrorBox {
    type Vtable = IVpnCustomErrorBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ec4efb2_c942_42af_b223_588b48328721);
}
impl ::windows::core::RuntimeName for VpnCustomErrorBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomErrorBox";
}
impl ::core::convert::From<VpnCustomErrorBox> for ::windows::core::IUnknown {
    fn from(value: VpnCustomErrorBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomErrorBox> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomErrorBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomErrorBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomErrorBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomErrorBox> for ::windows::core::IInspectable {
    fn from(value: VpnCustomErrorBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomErrorBox> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomErrorBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomErrorBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomErrorBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomErrorBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomErrorBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomErrorBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomErrorBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for VpnCustomErrorBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for &VpnCustomErrorBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::core::convert::TryInto::<IVpnCustomPrompt>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomErrorBox {}
unsafe impl ::core::marker::Sync for VpnCustomErrorBox {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomPromptBooleanInput(pub ::windows::core::IInspectable);
impl VpnCustomPromptBooleanInput {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomPromptBooleanInput, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetInitialValue(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InitialValue(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomPromptBooleanInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptBooleanInput;{c4c9a69e-ff47-4527-9f27-a49292019979})");
}
unsafe impl ::windows::core::Interface for VpnCustomPromptBooleanInput {
    type Vtable = IVpnCustomPromptBooleanInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4c9a69e_ff47_4527_9f27_a49292019979);
}
impl ::windows::core::RuntimeName for VpnCustomPromptBooleanInput {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptBooleanInput";
}
impl ::core::convert::From<VpnCustomPromptBooleanInput> for ::windows::core::IUnknown {
    fn from(value: VpnCustomPromptBooleanInput) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomPromptBooleanInput> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomPromptBooleanInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomPromptBooleanInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomPromptBooleanInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomPromptBooleanInput> for ::windows::core::IInspectable {
    fn from(value: VpnCustomPromptBooleanInput) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomPromptBooleanInput> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomPromptBooleanInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomPromptBooleanInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomPromptBooleanInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomPromptBooleanInput> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomPromptBooleanInput) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomPromptBooleanInput> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomPromptBooleanInput) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for VpnCustomPromptBooleanInput {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for &VpnCustomPromptBooleanInput {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::core::convert::TryInto::<IVpnCustomPromptElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomPromptBooleanInput {}
unsafe impl ::core::marker::Sync for VpnCustomPromptBooleanInput {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomPromptOptionSelector(pub ::windows::core::IInspectable);
impl VpnCustomPromptOptionSelector {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomPromptOptionSelector, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SelectedIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomPromptOptionSelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptOptionSelector;{3b8f34d9-8ec1-4e95-9a4e-7ba64d38f330})");
}
unsafe impl ::windows::core::Interface for VpnCustomPromptOptionSelector {
    type Vtable = IVpnCustomPromptOptionSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b8f34d9_8ec1_4e95_9a4e_7ba64d38f330);
}
impl ::windows::core::RuntimeName for VpnCustomPromptOptionSelector {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptOptionSelector";
}
impl ::core::convert::From<VpnCustomPromptOptionSelector> for ::windows::core::IUnknown {
    fn from(value: VpnCustomPromptOptionSelector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomPromptOptionSelector> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomPromptOptionSelector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomPromptOptionSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomPromptOptionSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomPromptOptionSelector> for ::windows::core::IInspectable {
    fn from(value: VpnCustomPromptOptionSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomPromptOptionSelector> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomPromptOptionSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomPromptOptionSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomPromptOptionSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomPromptOptionSelector> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomPromptOptionSelector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomPromptOptionSelector> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomPromptOptionSelector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for VpnCustomPromptOptionSelector {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for &VpnCustomPromptOptionSelector {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::core::convert::TryInto::<IVpnCustomPromptElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomPromptOptionSelector {}
unsafe impl ::core::marker::Sync for VpnCustomPromptOptionSelector {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomPromptText(pub ::windows::core::IInspectable);
impl VpnCustomPromptText {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomPromptText, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomPromptText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptText;{3bc8bdee-3a42-49a3-abdd-07b2edea752d})");
}
unsafe impl ::windows::core::Interface for VpnCustomPromptText {
    type Vtable = IVpnCustomPromptText_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bc8bdee_3a42_49a3_abdd_07b2edea752d);
}
impl ::windows::core::RuntimeName for VpnCustomPromptText {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptText";
}
impl ::core::convert::From<VpnCustomPromptText> for ::windows::core::IUnknown {
    fn from(value: VpnCustomPromptText) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomPromptText> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomPromptText) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomPromptText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomPromptText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomPromptText> for ::windows::core::IInspectable {
    fn from(value: VpnCustomPromptText) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomPromptText> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomPromptText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomPromptText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomPromptText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomPromptText> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomPromptText) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomPromptText> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomPromptText) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for VpnCustomPromptText {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for &VpnCustomPromptText {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::core::convert::TryInto::<IVpnCustomPromptElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomPromptText {}
unsafe impl ::core::marker::Sync for VpnCustomPromptText {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomPromptTextInput(pub ::windows::core::IInspectable);
impl VpnCustomPromptTextInput {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomPromptTextInput, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetPlaceholderText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIsTextHidden(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsTextHidden(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomPromptTextInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptTextInput;{c9da9c75-913c-47d5-88ba-48fc48930235})");
}
unsafe impl ::windows::core::Interface for VpnCustomPromptTextInput {
    type Vtable = IVpnCustomPromptTextInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9da9c75_913c_47d5_88ba_48fc48930235);
}
impl ::windows::core::RuntimeName for VpnCustomPromptTextInput {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptTextInput";
}
impl ::core::convert::From<VpnCustomPromptTextInput> for ::windows::core::IUnknown {
    fn from(value: VpnCustomPromptTextInput) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomPromptTextInput> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomPromptTextInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomPromptTextInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomPromptTextInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomPromptTextInput> for ::windows::core::IInspectable {
    fn from(value: VpnCustomPromptTextInput) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomPromptTextInput> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomPromptTextInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomPromptTextInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomPromptTextInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomPromptTextInput> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomPromptTextInput) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomPromptTextInput> for IVpnCustomPromptElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomPromptTextInput) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for VpnCustomPromptTextInput {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPromptElement> for &VpnCustomPromptTextInput {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPromptElement> {
        ::core::convert::TryInto::<IVpnCustomPromptElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomPromptTextInput {}
unsafe impl ::core::marker::Sync for VpnCustomPromptTextInput {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnCustomTextBox(pub ::windows::core::IInspectable);
impl VpnCustomTextBox {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnCustomTextBox, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetDisplayText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnCustomTextBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomTextBox;{daa4c3ca-8f23-4d36-91f1-76d937827942})");
}
unsafe impl ::windows::core::Interface for VpnCustomTextBox {
    type Vtable = IVpnCustomTextBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaa4c3ca_8f23_4d36_91f1_76d937827942);
}
impl ::windows::core::RuntimeName for VpnCustomTextBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomTextBox";
}
impl ::core::convert::From<VpnCustomTextBox> for ::windows::core::IUnknown {
    fn from(value: VpnCustomTextBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnCustomTextBox> for ::windows::core::IUnknown {
    fn from(value: &VpnCustomTextBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnCustomTextBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnCustomTextBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnCustomTextBox> for ::windows::core::IInspectable {
    fn from(value: VpnCustomTextBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnCustomTextBox> for ::windows::core::IInspectable {
    fn from(value: &VpnCustomTextBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnCustomTextBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnCustomTextBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnCustomTextBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnCustomTextBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnCustomTextBox> for IVpnCustomPrompt {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnCustomTextBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for VpnCustomTextBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnCustomPrompt> for &VpnCustomTextBox {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnCustomPrompt> {
        ::core::convert::TryInto::<IVpnCustomPrompt>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnCustomTextBox {}
unsafe impl ::core::marker::Sync for VpnCustomTextBox {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnDataPathType(pub i32);
impl VpnDataPathType {
    pub const Send: VpnDataPathType = VpnDataPathType(0i32);
    pub const Receive: VpnDataPathType = VpnDataPathType(1i32);
}
impl ::core::convert::From<i32> for VpnDataPathType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnDataPathType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnDataPathType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnDataPathType;i4)");
}
impl ::windows::core::DefaultType for VpnDataPathType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnDomainNameAssignment(pub ::windows::core::IInspectable);
impl VpnDomainNameAssignment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnDomainNameAssignment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetProxyAutoConfigurationUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProxyAutoConfigurationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnDomainNameAssignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnDomainNameAssignment;{4135b141-ccdb-49b5-9401-039a8ae767e9})");
}
unsafe impl ::windows::core::Interface for VpnDomainNameAssignment {
    type Vtable = IVpnDomainNameAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4135b141_ccdb_49b5_9401_039a8ae767e9);
}
impl ::windows::core::RuntimeName for VpnDomainNameAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnDomainNameAssignment";
}
impl ::core::convert::From<VpnDomainNameAssignment> for ::windows::core::IUnknown {
    fn from(value: VpnDomainNameAssignment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnDomainNameAssignment> for ::windows::core::IUnknown {
    fn from(value: &VpnDomainNameAssignment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnDomainNameAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnDomainNameAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnDomainNameAssignment> for ::windows::core::IInspectable {
    fn from(value: VpnDomainNameAssignment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnDomainNameAssignment> for ::windows::core::IInspectable {
    fn from(value: &VpnDomainNameAssignment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnDomainNameAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnDomainNameAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnDomainNameAssignment {}
unsafe impl ::core::marker::Sync for VpnDomainNameAssignment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnDomainNameInfo(pub ::windows::core::IInspectable);
impl VpnDomainNameInfo {
    pub fn SetDomainName<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DomainName(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    pub fn SetDomainNameType(&self, value: VpnDomainNameType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DomainNameType(&self) -> ::windows::core::Result<VpnDomainNameType> {
        let this = self;
        unsafe {
            let mut result__: VpnDomainNameType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnDomainNameType>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::HostName>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::HostName>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnDomainNameInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::HostName>>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::HostName>>>(name: Param0, nametype: VpnDomainNameType, dnsserverlist: Param2, proxyserverlist: Param3) -> ::windows::core::Result<VpnDomainNameInfo> {
        Self::IVpnDomainNameInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), nametype, dnsserverlist.into_param().abi(), proxyserverlist.into_param().abi(), &mut result__).from_abi::<VpnDomainNameInfo>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn WebProxyUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = &::windows::core::Interface::cast::<IVpnDomainNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    pub fn IVpnDomainNameInfoFactory<R, F: FnOnce(&IVpnDomainNameInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnDomainNameInfo, IVpnDomainNameInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnDomainNameInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnDomainNameInfo;{ad2eb82f-ea8e-4f7a-843e-1a87e32e1b9a})");
}
unsafe impl ::windows::core::Interface for VpnDomainNameInfo {
    type Vtable = IVpnDomainNameInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad2eb82f_ea8e_4f7a_843e_1a87e32e1b9a);
}
impl ::windows::core::RuntimeName for VpnDomainNameInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnDomainNameInfo";
}
impl ::core::convert::From<VpnDomainNameInfo> for ::windows::core::IUnknown {
    fn from(value: VpnDomainNameInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnDomainNameInfo> for ::windows::core::IUnknown {
    fn from(value: &VpnDomainNameInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnDomainNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnDomainNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnDomainNameInfo> for ::windows::core::IInspectable {
    fn from(value: VpnDomainNameInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnDomainNameInfo> for ::windows::core::IInspectable {
    fn from(value: &VpnDomainNameInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnDomainNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnDomainNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnDomainNameInfo {}
unsafe impl ::core::marker::Sync for VpnDomainNameInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnDomainNameType(pub i32);
impl VpnDomainNameType {
    pub const Suffix: VpnDomainNameType = VpnDomainNameType(0i32);
    pub const FullyQualified: VpnDomainNameType = VpnDomainNameType(1i32);
    pub const Reserved: VpnDomainNameType = VpnDomainNameType(65535i32);
}
impl ::core::convert::From<i32> for VpnDomainNameType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnDomainNameType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnDomainNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnDomainNameType;i4)");
}
impl ::windows::core::DefaultType for VpnDomainNameType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnForegroundActivatedEventArgs(pub ::windows::core::IInspectable);
impl VpnForegroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SharedContext(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivationOperation(&self) -> ::windows::core::Result<VpnForegroundActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnForegroundActivationOperation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnForegroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnForegroundActivatedEventArgs;{85b465b0-cadb-4d70-ac92-543a24dc9ebc})");
}
unsafe impl ::windows::core::Interface for VpnForegroundActivatedEventArgs {
    type Vtable = IVpnForegroundActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85b465b0_cadb_4d70_ac92_543a24dc9ebc);
}
impl ::windows::core::RuntimeName for VpnForegroundActivatedEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnForegroundActivatedEventArgs";
}
impl ::core::convert::From<VpnForegroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VpnForegroundActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnForegroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VpnForegroundActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnForegroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VpnForegroundActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnForegroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VpnForegroundActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<VpnForegroundActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnForegroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&VpnForegroundActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnForegroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> for VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> for &VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<VpnForegroundActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnForegroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&VpnForegroundActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnForegroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for &VpnForegroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnForegroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VpnForegroundActivatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnForegroundActivationOperation(pub ::windows::core::IInspectable);
impl VpnForegroundActivationOperation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Complete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, result: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), result.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnForegroundActivationOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnForegroundActivationOperation;{9e010d57-f17a-4bd5-9b6d-f984f1297d3c})");
}
unsafe impl ::windows::core::Interface for VpnForegroundActivationOperation {
    type Vtable = IVpnForegroundActivationOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e010d57_f17a_4bd5_9b6d_f984f1297d3c);
}
impl ::windows::core::RuntimeName for VpnForegroundActivationOperation {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnForegroundActivationOperation";
}
impl ::core::convert::From<VpnForegroundActivationOperation> for ::windows::core::IUnknown {
    fn from(value: VpnForegroundActivationOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnForegroundActivationOperation> for ::windows::core::IUnknown {
    fn from(value: &VpnForegroundActivationOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnForegroundActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnForegroundActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnForegroundActivationOperation> for ::windows::core::IInspectable {
    fn from(value: VpnForegroundActivationOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnForegroundActivationOperation> for ::windows::core::IInspectable {
    fn from(value: &VpnForegroundActivationOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnForegroundActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnForegroundActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnForegroundActivationOperation {}
unsafe impl ::core::marker::Sync for VpnForegroundActivationOperation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnIPProtocol(pub i32);
impl VpnIPProtocol {
    pub const None: VpnIPProtocol = VpnIPProtocol(0i32);
    pub const Tcp: VpnIPProtocol = VpnIPProtocol(6i32);
    pub const Udp: VpnIPProtocol = VpnIPProtocol(17i32);
    pub const Icmp: VpnIPProtocol = VpnIPProtocol(1i32);
    pub const Ipv6Icmp: VpnIPProtocol = VpnIPProtocol(58i32);
    pub const Igmp: VpnIPProtocol = VpnIPProtocol(2i32);
    pub const Pgm: VpnIPProtocol = VpnIPProtocol(113i32);
}
impl ::core::convert::From<i32> for VpnIPProtocol {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnIPProtocol {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnIPProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnIPProtocol;i4)");
}
impl ::windows::core::DefaultType for VpnIPProtocol {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnInterfaceId(pub ::windows::core::IInspectable);
impl VpnInterfaceId {
    pub fn GetAddressInfo(&self, id: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.set_abi_len(), id as *mut _ as _).ok() }
    }
    pub fn CreateVpnInterfaceId(address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId> {
        Self::IVpnInterfaceIdFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), address.len() as u32, ::core::mem::transmute(address.as_ptr()), &mut result__).from_abi::<VpnInterfaceId>(result__)
        })
    }
    pub fn IVpnInterfaceIdFactory<R, F: FnOnce(&IVpnInterfaceIdFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnInterfaceId, IVpnInterfaceIdFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnInterfaceId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnInterfaceId;{9e2ddca2-1712-4ce4-b179-8c652c6d1011})");
}
unsafe impl ::windows::core::Interface for VpnInterfaceId {
    type Vtable = IVpnInterfaceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d1011);
}
impl ::windows::core::RuntimeName for VpnInterfaceId {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnInterfaceId";
}
impl ::core::convert::From<VpnInterfaceId> for ::windows::core::IUnknown {
    fn from(value: VpnInterfaceId) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnInterfaceId> for ::windows::core::IUnknown {
    fn from(value: &VpnInterfaceId) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnInterfaceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnInterfaceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnInterfaceId> for ::windows::core::IInspectable {
    fn from(value: VpnInterfaceId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnInterfaceId> for ::windows::core::IInspectable {
    fn from(value: &VpnInterfaceId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnInterfaceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnInterfaceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnInterfaceId {}
unsafe impl ::core::marker::Sync for VpnInterfaceId {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnManagementAgent(pub ::windows::core::IInspectable);
impl VpnManagementAgent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnManagementAgent, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddProfileFromXmlAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xml: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xml.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddProfileFromObjectAsync<'a, Param0: ::windows::core::IntoParam<'a, IVpnProfile>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateProfileFromXmlAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, xml: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), xml.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateProfileFromObjectAsync<'a, Param0: ::windows::core::IntoParam<'a, IVpnProfile>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetProfilesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, IVpnProfile>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, IVpnProfile>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectProfileWithPasswordCredentialAsync<'a, Param0: ::windows::core::IntoParam<'a, IVpnProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, profile: Param0, passwordcredential: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), profile.into_param().abi(), passwordcredential.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DisconnectProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, IVpnProfile>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnManagementAgent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnManagementAgent;{193696cd-a5c4-4abe-852b-785be4cb3e34})");
}
unsafe impl ::windows::core::Interface for VpnManagementAgent {
    type Vtable = IVpnManagementAgent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x193696cd_a5c4_4abe_852b_785be4cb3e34);
}
impl ::windows::core::RuntimeName for VpnManagementAgent {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnManagementAgent";
}
impl ::core::convert::From<VpnManagementAgent> for ::windows::core::IUnknown {
    fn from(value: VpnManagementAgent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnManagementAgent> for ::windows::core::IUnknown {
    fn from(value: &VpnManagementAgent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnManagementAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnManagementAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnManagementAgent> for ::windows::core::IInspectable {
    fn from(value: VpnManagementAgent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnManagementAgent> for ::windows::core::IInspectable {
    fn from(value: &VpnManagementAgent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnManagementAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnManagementAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnManagementAgent {}
unsafe impl ::core::marker::Sync for VpnManagementAgent {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnManagementConnectionStatus(pub i32);
impl VpnManagementConnectionStatus {
    pub const Disconnected: VpnManagementConnectionStatus = VpnManagementConnectionStatus(0i32);
    pub const Disconnecting: VpnManagementConnectionStatus = VpnManagementConnectionStatus(1i32);
    pub const Connected: VpnManagementConnectionStatus = VpnManagementConnectionStatus(2i32);
    pub const Connecting: VpnManagementConnectionStatus = VpnManagementConnectionStatus(3i32);
}
impl ::core::convert::From<i32> for VpnManagementConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnManagementConnectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnManagementConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnManagementConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for VpnManagementConnectionStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnManagementErrorStatus(pub i32);
impl VpnManagementErrorStatus {
    pub const Ok: VpnManagementErrorStatus = VpnManagementErrorStatus(0i32);
    pub const Other: VpnManagementErrorStatus = VpnManagementErrorStatus(1i32);
    pub const InvalidXmlSyntax: VpnManagementErrorStatus = VpnManagementErrorStatus(2i32);
    pub const ProfileNameTooLong: VpnManagementErrorStatus = VpnManagementErrorStatus(3i32);
    pub const ProfileInvalidAppId: VpnManagementErrorStatus = VpnManagementErrorStatus(4i32);
    pub const AccessDenied: VpnManagementErrorStatus = VpnManagementErrorStatus(5i32);
    pub const CannotFindProfile: VpnManagementErrorStatus = VpnManagementErrorStatus(6i32);
    pub const AlreadyDisconnecting: VpnManagementErrorStatus = VpnManagementErrorStatus(7i32);
    pub const AlreadyConnected: VpnManagementErrorStatus = VpnManagementErrorStatus(8i32);
    pub const GeneralAuthenticationFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(9i32);
    pub const EapFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(10i32);
    pub const SmartCardFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(11i32);
    pub const CertificateFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(12i32);
    pub const ServerConfiguration: VpnManagementErrorStatus = VpnManagementErrorStatus(13i32);
    pub const NoConnection: VpnManagementErrorStatus = VpnManagementErrorStatus(14i32);
    pub const ServerConnection: VpnManagementErrorStatus = VpnManagementErrorStatus(15i32);
    pub const UserNamePassword: VpnManagementErrorStatus = VpnManagementErrorStatus(16i32);
    pub const DnsNotResolvable: VpnManagementErrorStatus = VpnManagementErrorStatus(17i32);
    pub const InvalidIP: VpnManagementErrorStatus = VpnManagementErrorStatus(18i32);
}
impl ::core::convert::From<i32> for VpnManagementErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnManagementErrorStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnManagementErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnManagementErrorStatus;i4)");
}
impl ::windows::core::DefaultType for VpnManagementErrorStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnNamespaceAssignment(pub ::windows::core::IInspectable);
impl VpnNamespaceAssignment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnNamespaceAssignment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetNamespaceList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NamespaceList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetProxyAutoConfigUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProxyAutoConfigUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnNamespaceAssignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnNamespaceAssignment;{d7f7db18-307d-4c0e-bd62-8fa270bbadd6})");
}
unsafe impl ::windows::core::Interface for VpnNamespaceAssignment {
    type Vtable = IVpnNamespaceAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f7db18_307d_4c0e_bd62_8fa270bbadd6);
}
impl ::windows::core::RuntimeName for VpnNamespaceAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNamespaceAssignment";
}
impl ::core::convert::From<VpnNamespaceAssignment> for ::windows::core::IUnknown {
    fn from(value: VpnNamespaceAssignment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnNamespaceAssignment> for ::windows::core::IUnknown {
    fn from(value: &VpnNamespaceAssignment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnNamespaceAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnNamespaceAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnNamespaceAssignment> for ::windows::core::IInspectable {
    fn from(value: VpnNamespaceAssignment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnNamespaceAssignment> for ::windows::core::IInspectable {
    fn from(value: &VpnNamespaceAssignment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnNamespaceAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnNamespaceAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnNamespaceAssignment {}
unsafe impl ::core::marker::Sync for VpnNamespaceAssignment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnNamespaceInfo(pub ::windows::core::IInspectable);
impl VpnNamespaceInfo {
    pub fn SetNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Namespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDnsServers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<super::HostName>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::HostName>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetWebProxyServers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<super::HostName>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::HostName>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnNamespaceInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<super::HostName>>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<super::HostName>>>(name: Param0, dnsserverlist: Param1, proxyserverlist: Param2) -> ::windows::core::Result<VpnNamespaceInfo> {
        Self::IVpnNamespaceInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), dnsserverlist.into_param().abi(), proxyserverlist.into_param().abi(), &mut result__).from_abi::<VpnNamespaceInfo>(result__)
        })
    }
    pub fn IVpnNamespaceInfoFactory<R, F: FnOnce(&IVpnNamespaceInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnNamespaceInfo, IVpnNamespaceInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnNamespaceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnNamespaceInfo;{30edfb43-444f-44c5-8167-a35a91f1af94})");
}
unsafe impl ::windows::core::Interface for VpnNamespaceInfo {
    type Vtable = IVpnNamespaceInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30edfb43_444f_44c5_8167_a35a91f1af94);
}
impl ::windows::core::RuntimeName for VpnNamespaceInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNamespaceInfo";
}
impl ::core::convert::From<VpnNamespaceInfo> for ::windows::core::IUnknown {
    fn from(value: VpnNamespaceInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnNamespaceInfo> for ::windows::core::IUnknown {
    fn from(value: &VpnNamespaceInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnNamespaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnNamespaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnNamespaceInfo> for ::windows::core::IInspectable {
    fn from(value: VpnNamespaceInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnNamespaceInfo> for ::windows::core::IInspectable {
    fn from(value: &VpnNamespaceInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnNamespaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnNamespaceInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnNamespaceInfo {}
unsafe impl ::core::marker::Sync for VpnNamespaceInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnNativeProfile(pub ::windows::core::IInspectable);
impl VpnNativeProfile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnNativeProfile, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Servers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn RoutingPolicyType(&self) -> ::windows::core::Result<VpnRoutingPolicyType> {
        let this = self;
        unsafe {
            let mut result__: VpnRoutingPolicyType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnRoutingPolicyType>(result__)
        }
    }
    pub fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NativeProtocolType(&self) -> ::windows::core::Result<VpnNativeProtocolType> {
        let this = self;
        unsafe {
            let mut result__: VpnNativeProtocolType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnNativeProtocolType>(result__)
        }
    }
    pub fn SetNativeProtocolType(&self, value: VpnNativeProtocolType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn UserAuthenticationMethod(&self) -> ::windows::core::Result<VpnAuthenticationMethod> {
        let this = self;
        unsafe {
            let mut result__: VpnAuthenticationMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnAuthenticationMethod>(result__)
        }
    }
    pub fn SetUserAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TunnelAuthenticationMethod(&self) -> ::windows::core::Result<VpnAuthenticationMethod> {
        let this = self;
        unsafe {
            let mut result__: VpnAuthenticationMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnAuthenticationMethod>(result__)
        }
    }
    pub fn SetTunnelAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EapConfiguration(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEapConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetProfileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnAppId>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>(result__)
        }
    }
    pub fn RememberCredentials(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RequireVpnClientAppUI(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnNativeProfile2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnNativeProfile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<VpnManagementConnectionStatus> {
        let this = &::windows::core::Interface::cast::<IVpnNativeProfile2>(self)?;
        unsafe {
            let mut result__: VpnManagementConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnManagementConnectionStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnNativeProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnNativeProfile;{a4aee29e-6417-4333-9842-f0a66db69802})");
}
unsafe impl ::windows::core::Interface for VpnNativeProfile {
    type Vtable = IVpnNativeProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4aee29e_6417_4333_9842_f0a66db69802);
}
impl ::windows::core::RuntimeName for VpnNativeProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNativeProfile";
}
impl ::core::convert::From<VpnNativeProfile> for ::windows::core::IUnknown {
    fn from(value: VpnNativeProfile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnNativeProfile> for ::windows::core::IUnknown {
    fn from(value: &VpnNativeProfile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnNativeProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnNativeProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnNativeProfile> for ::windows::core::IInspectable {
    fn from(value: VpnNativeProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnNativeProfile> for ::windows::core::IInspectable {
    fn from(value: &VpnNativeProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnNativeProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnNativeProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnNativeProfile> for IVpnProfile {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnNativeProfile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnNativeProfile> for IVpnProfile {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnNativeProfile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnProfile> for VpnNativeProfile {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnProfile> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnProfile> for &VpnNativeProfile {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnProfile> {
        ::core::convert::TryInto::<IVpnProfile>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnNativeProfile {}
unsafe impl ::core::marker::Sync for VpnNativeProfile {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnNativeProtocolType(pub i32);
impl VpnNativeProtocolType {
    pub const Pptp: VpnNativeProtocolType = VpnNativeProtocolType(0i32);
    pub const L2tp: VpnNativeProtocolType = VpnNativeProtocolType(1i32);
    pub const IpsecIkev2: VpnNativeProtocolType = VpnNativeProtocolType(2i32);
}
impl ::core::convert::From<i32> for VpnNativeProtocolType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnNativeProtocolType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnNativeProtocolType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnNativeProtocolType;i4)");
}
impl ::windows::core::DefaultType for VpnNativeProtocolType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnPacketBuffer(pub ::windows::core::IInspectable);
impl VpnPacketBuffer {
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::Buffer>(result__)
        }
    }
    pub fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<VpnPacketBufferStatus> {
        let this = self;
        unsafe {
            let mut result__: VpnPacketBufferStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnPacketBufferStatus>(result__)
        }
    }
    pub fn SetTransportAffinity(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TransportAffinity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<VpnAppId> {
        let this = &::windows::core::Interface::cast::<IVpnPacketBuffer2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnAppId>(result__)
        }
    }
    pub fn CreateVpnPacketBuffer<'a, Param0: ::windows::core::IntoParam<'a, VpnPacketBuffer>>(parentbuffer: Param0, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer> {
        Self::IVpnPacketBufferFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), parentbuffer.into_param().abi(), offset, length, &mut result__).from_abi::<VpnPacketBuffer>(result__)
        })
    }
    pub fn SetTransportContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnPacketBuffer3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TransportContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IVpnPacketBuffer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn IVpnPacketBufferFactory<R, F: FnOnce(&IVpnPacketBufferFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnPacketBuffer, IVpnPacketBufferFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnPacketBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPacketBuffer;{c2f891fc-4d5c-4a63-b70d-4e307eacce55})");
}
unsafe impl ::windows::core::Interface for VpnPacketBuffer {
    type Vtable = IVpnPacketBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f891fc_4d5c_4a63_b70d_4e307eacce55);
}
impl ::windows::core::RuntimeName for VpnPacketBuffer {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPacketBuffer";
}
impl ::core::convert::From<VpnPacketBuffer> for ::windows::core::IUnknown {
    fn from(value: VpnPacketBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnPacketBuffer> for ::windows::core::IUnknown {
    fn from(value: &VpnPacketBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnPacketBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnPacketBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnPacketBuffer> for ::windows::core::IInspectable {
    fn from(value: VpnPacketBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnPacketBuffer> for ::windows::core::IInspectable {
    fn from(value: &VpnPacketBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnPacketBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnPacketBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnPacketBuffer {}
unsafe impl ::core::marker::Sync for VpnPacketBuffer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnPacketBufferList(pub ::windows::core::IInspectable);
impl VpnPacketBufferList {
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, VpnPacketBuffer>>(&self, nextvpnpacketbuffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), nextvpnpacketbuffer.into_param().abi()).ok() }
    }
    pub fn AddAtBegin<'a, Param0: ::windows::core::IntoParam<'a, VpnPacketBuffer>>(&self, nextvpnpacketbuffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), nextvpnpacketbuffer.into_param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnPacketBuffer>(result__)
        }
    }
    pub fn RemoveAtBegin(&self) -> ::windows::core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnPacketBuffer>(result__)
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<VpnPacketBufferStatus> {
        let this = self;
        unsafe {
            let mut result__: VpnPacketBufferStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnPacketBufferStatus>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<VpnPacketBuffer>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<VpnPacketBuffer>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<VpnPacketBuffer>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnPacketBufferList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPacketBufferList;{c2f891fc-4d5c-4a63-b70d-4e307eacce77})");
}
unsafe impl ::windows::core::Interface for VpnPacketBufferList {
    type Vtable = IVpnPacketBufferList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f891fc_4d5c_4a63_b70d_4e307eacce77);
}
impl ::windows::core::RuntimeName for VpnPacketBufferList {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPacketBufferList";
}
impl ::core::convert::From<VpnPacketBufferList> for ::windows::core::IUnknown {
    fn from(value: VpnPacketBufferList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnPacketBufferList> for ::windows::core::IUnknown {
    fn from(value: &VpnPacketBufferList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnPacketBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnPacketBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnPacketBufferList> for ::windows::core::IInspectable {
    fn from(value: VpnPacketBufferList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnPacketBufferList> for ::windows::core::IInspectable {
    fn from(value: &VpnPacketBufferList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnPacketBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnPacketBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<VpnPacketBufferList> for super::super::Foundation::Collections::IIterable<VpnPacketBuffer> {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnPacketBufferList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&VpnPacketBufferList> for super::super::Foundation::Collections::IIterable<VpnPacketBuffer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnPacketBufferList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<VpnPacketBuffer>> for VpnPacketBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<VpnPacketBuffer>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<VpnPacketBuffer>> for &VpnPacketBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<VpnPacketBuffer>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<VpnPacketBuffer>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnPacketBufferList {}
unsafe impl ::core::marker::Sync for VpnPacketBufferList {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for VpnPacketBufferList {
    type Item = VpnPacketBuffer;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &VpnPacketBufferList {
    type Item = VpnPacketBuffer;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnPacketBufferStatus(pub i32);
impl VpnPacketBufferStatus {
    pub const Ok: VpnPacketBufferStatus = VpnPacketBufferStatus(0i32);
    pub const InvalidBufferSize: VpnPacketBufferStatus = VpnPacketBufferStatus(1i32);
}
impl ::core::convert::From<i32> for VpnPacketBufferStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnPacketBufferStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnPacketBufferStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnPacketBufferStatus;i4)");
}
impl ::windows::core::DefaultType for VpnPacketBufferStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnPickedCredential(pub ::windows::core::IInspectable);
impl VpnPickedCredential {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    pub fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnPickedCredential {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPickedCredential;{9a793ac7-8854-4e52-ad97-24dd9a842bce})");
}
unsafe impl ::windows::core::Interface for VpnPickedCredential {
    type Vtable = IVpnPickedCredential_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a793ac7_8854_4e52_ad97_24dd9a842bce);
}
impl ::windows::core::RuntimeName for VpnPickedCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPickedCredential";
}
impl ::core::convert::From<VpnPickedCredential> for ::windows::core::IUnknown {
    fn from(value: VpnPickedCredential) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnPickedCredential> for ::windows::core::IUnknown {
    fn from(value: &VpnPickedCredential) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnPickedCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnPickedCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnPickedCredential> for ::windows::core::IInspectable {
    fn from(value: VpnPickedCredential) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnPickedCredential> for ::windows::core::IInspectable {
    fn from(value: &VpnPickedCredential) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnPickedCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnPickedCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnPickedCredential {}
unsafe impl ::core::marker::Sync for VpnPickedCredential {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnPlugInProfile(pub ::windows::core::IInspectable);
impl VpnPlugInProfile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnPlugInProfile, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ServerUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        }
    }
    pub fn CustomConfiguration(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCustomConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn VpnPluginPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetVpnPluginPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetProfileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnAppId>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>(result__)
        }
    }
    pub fn RememberCredentials(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RequireVpnClientAppUI(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<VpnManagementConnectionStatus> {
        let this = &::windows::core::Interface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe {
            let mut result__: VpnManagementConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnManagementConnectionStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnPlugInProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPlugInProfile;{0edf0da4-4f00-4589-8d7b-4bf988f6542c})");
}
unsafe impl ::windows::core::Interface for VpnPlugInProfile {
    type Vtable = IVpnPlugInProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0edf0da4_4f00_4589_8d7b_4bf988f6542c);
}
impl ::windows::core::RuntimeName for VpnPlugInProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPlugInProfile";
}
impl ::core::convert::From<VpnPlugInProfile> for ::windows::core::IUnknown {
    fn from(value: VpnPlugInProfile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnPlugInProfile> for ::windows::core::IUnknown {
    fn from(value: &VpnPlugInProfile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnPlugInProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnPlugInProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnPlugInProfile> for ::windows::core::IInspectable {
    fn from(value: VpnPlugInProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnPlugInProfile> for ::windows::core::IInspectable {
    fn from(value: &VpnPlugInProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnPlugInProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnPlugInProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VpnPlugInProfile> for IVpnProfile {
    type Error = ::windows::core::Error;
    fn try_from(value: VpnPlugInProfile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VpnPlugInProfile> for IVpnProfile {
    type Error = ::windows::core::Error;
    fn try_from(value: &VpnPlugInProfile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnProfile> for VpnPlugInProfile {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnProfile> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVpnProfile> for &VpnPlugInProfile {
    fn into_param(self) -> ::windows::core::Param<'a, IVpnProfile> {
        ::core::convert::TryInto::<IVpnProfile>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VpnPlugInProfile {}
unsafe impl ::core::marker::Sync for VpnPlugInProfile {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnRoute(pub ::windows::core::IInspectable);
impl VpnRoute {
    pub fn SetAddress<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Address(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    pub fn SetPrefixSize(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PrefixSize(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn CreateVpnRoute<'a, Param0: ::windows::core::IntoParam<'a, super::HostName>>(address: Param0, prefixsize: u8) -> ::windows::core::Result<VpnRoute> {
        Self::IVpnRouteFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), address.into_param().abi(), prefixsize, &mut result__).from_abi::<VpnRoute>(result__)
        })
    }
    pub fn IVpnRouteFactory<R, F: FnOnce(&IVpnRouteFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnRoute, IVpnRouteFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnRoute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnRoute;{b5731b83-0969-4699-938e-7776db29cfb3})");
}
unsafe impl ::windows::core::Interface for VpnRoute {
    type Vtable = IVpnRoute_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5731b83_0969_4699_938e_7776db29cfb3);
}
impl ::windows::core::RuntimeName for VpnRoute {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnRoute";
}
impl ::core::convert::From<VpnRoute> for ::windows::core::IUnknown {
    fn from(value: VpnRoute) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnRoute> for ::windows::core::IUnknown {
    fn from(value: &VpnRoute) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnRoute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnRoute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnRoute> for ::windows::core::IInspectable {
    fn from(value: VpnRoute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnRoute> for ::windows::core::IInspectable {
    fn from(value: &VpnRoute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnRoute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnRoute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnRoute {}
unsafe impl ::core::marker::Sync for VpnRoute {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnRouteAssignment(pub ::windows::core::IInspectable);
impl VpnRouteAssignment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnRouteAssignment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv4InclusionRoutes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<VpnRoute>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv6InclusionRoutes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<VpnRoute>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv4InclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv6InclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv4ExclusionRoutes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<VpnRoute>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv6ExclusionRoutes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<VpnRoute>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv4ExclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv6ExclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnRoute>>(result__)
        }
    }
    pub fn SetExcludeLocalSubnets(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExcludeLocalSubnets(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnRouteAssignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnRouteAssignment;{db64de22-ce39-4a76-9550-f61039f80e48})");
}
unsafe impl ::windows::core::Interface for VpnRouteAssignment {
    type Vtable = IVpnRouteAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb64de22_ce39_4a76_9550_f61039f80e48);
}
impl ::windows::core::RuntimeName for VpnRouteAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnRouteAssignment";
}
impl ::core::convert::From<VpnRouteAssignment> for ::windows::core::IUnknown {
    fn from(value: VpnRouteAssignment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnRouteAssignment> for ::windows::core::IUnknown {
    fn from(value: &VpnRouteAssignment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnRouteAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnRouteAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnRouteAssignment> for ::windows::core::IInspectable {
    fn from(value: VpnRouteAssignment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnRouteAssignment> for ::windows::core::IInspectable {
    fn from(value: &VpnRouteAssignment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnRouteAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnRouteAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnRouteAssignment {}
unsafe impl ::core::marker::Sync for VpnRouteAssignment {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VpnRoutingPolicyType(pub i32);
impl VpnRoutingPolicyType {
    pub const SplitRouting: VpnRoutingPolicyType = VpnRoutingPolicyType(0i32);
    pub const ForceAllTrafficOverVpn: VpnRoutingPolicyType = VpnRoutingPolicyType(1i32);
}
impl ::core::convert::From<i32> for VpnRoutingPolicyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VpnRoutingPolicyType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VpnRoutingPolicyType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnRoutingPolicyType;i4)");
}
impl ::windows::core::DefaultType for VpnRoutingPolicyType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnSystemHealth(pub ::windows::core::IInspectable);
impl VpnSystemHealth {
    #[cfg(feature = "Storage_Streams")]
    pub fn StatementOfHealth(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::Buffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnSystemHealth {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnSystemHealth;{99a8f8af-c0ee-4e75-817a-f231aee5123d})");
}
unsafe impl ::windows::core::Interface for VpnSystemHealth {
    type Vtable = IVpnSystemHealth_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99a8f8af_c0ee_4e75_817a_f231aee5123d);
}
impl ::windows::core::RuntimeName for VpnSystemHealth {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnSystemHealth";
}
impl ::core::convert::From<VpnSystemHealth> for ::windows::core::IUnknown {
    fn from(value: VpnSystemHealth) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnSystemHealth> for ::windows::core::IUnknown {
    fn from(value: &VpnSystemHealth) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnSystemHealth {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnSystemHealth {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnSystemHealth> for ::windows::core::IInspectable {
    fn from(value: VpnSystemHealth) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnSystemHealth> for ::windows::core::IInspectable {
    fn from(value: &VpnSystemHealth) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnSystemHealth {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnSystemHealth {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnSystemHealth {}
unsafe impl ::core::marker::Sync for VpnSystemHealth {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnTrafficFilter(pub ::windows::core::IInspectable);
impl VpnTrafficFilter {
    pub fn AppId(&self) -> ::windows::core::Result<VpnAppId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnAppId>(result__)
        }
    }
    pub fn SetAppId<'a, Param0: ::windows::core::IntoParam<'a, VpnAppId>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppClaims(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<VpnIPProtocol> {
        let this = self;
        unsafe {
            let mut result__: VpnIPProtocol = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnIPProtocol>(result__)
        }
    }
    pub fn SetProtocol(&self, value: VpnIPProtocol) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalPortRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemotePortRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalAddressRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoteAddressRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn RoutingPolicyType(&self) -> ::windows::core::Result<VpnRoutingPolicyType> {
        let this = self;
        unsafe {
            let mut result__: VpnRoutingPolicyType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VpnRoutingPolicyType>(result__)
        }
    }
    pub fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, VpnAppId>>(appid: Param0) -> ::windows::core::Result<VpnTrafficFilter> {
        Self::IVpnTrafficFilterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appid.into_param().abi(), &mut result__).from_abi::<VpnTrafficFilter>(result__)
        })
    }
    pub fn IVpnTrafficFilterFactory<R, F: FnOnce(&IVpnTrafficFilterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnTrafficFilter, IVpnTrafficFilterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnTrafficFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnTrafficFilter;{2f691b60-6c9f-47f5-ac36-bb1b042e2c50})");
}
unsafe impl ::windows::core::Interface for VpnTrafficFilter {
    type Vtable = IVpnTrafficFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f691b60_6c9f_47f5_ac36_bb1b042e2c50);
}
impl ::windows::core::RuntimeName for VpnTrafficFilter {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnTrafficFilter";
}
impl ::core::convert::From<VpnTrafficFilter> for ::windows::core::IUnknown {
    fn from(value: VpnTrafficFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnTrafficFilter> for ::windows::core::IUnknown {
    fn from(value: &VpnTrafficFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnTrafficFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnTrafficFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnTrafficFilter> for ::windows::core::IInspectable {
    fn from(value: VpnTrafficFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnTrafficFilter> for ::windows::core::IInspectable {
    fn from(value: &VpnTrafficFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnTrafficFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnTrafficFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnTrafficFilter {}
unsafe impl ::core::marker::Sync for VpnTrafficFilter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VpnTrafficFilterAssignment(pub ::windows::core::IInspectable);
impl VpnTrafficFilterAssignment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VpnTrafficFilterAssignment, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilterList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>(result__)
        }
    }
    pub fn AllowOutbound(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowOutbound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowInbound(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowInbound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VpnTrafficFilterAssignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnTrafficFilterAssignment;{56ccd45c-e664-471e-89cd-601603b9e0f3})");
}
unsafe impl ::windows::core::Interface for VpnTrafficFilterAssignment {
    type Vtable = IVpnTrafficFilterAssignment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ccd45c_e664_471e_89cd_601603b9e0f3);
}
impl ::windows::core::RuntimeName for VpnTrafficFilterAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnTrafficFilterAssignment";
}
impl ::core::convert::From<VpnTrafficFilterAssignment> for ::windows::core::IUnknown {
    fn from(value: VpnTrafficFilterAssignment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VpnTrafficFilterAssignment> for ::windows::core::IUnknown {
    fn from(value: &VpnTrafficFilterAssignment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VpnTrafficFilterAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VpnTrafficFilterAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VpnTrafficFilterAssignment> for ::windows::core::IInspectable {
    fn from(value: VpnTrafficFilterAssignment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VpnTrafficFilterAssignment> for ::windows::core::IInspectable {
    fn from(value: &VpnTrafficFilterAssignment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VpnTrafficFilterAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VpnTrafficFilterAssignment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VpnTrafficFilterAssignment {}
unsafe impl ::core::marker::Sync for VpnTrafficFilterAssignment {}
