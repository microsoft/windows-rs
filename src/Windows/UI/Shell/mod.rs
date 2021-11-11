#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Shell`*"]
pub struct AdaptiveCardBuilder {}
impl AdaptiveCardBuilder {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<IAdaptiveCard>(result__)
        })
    }
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Shell`*"]
pub struct IAdaptiveCard(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdaptiveCard {
    type Vtable = IAdaptiveCard_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d0568c_a274_41cd_82a8_989d40b9b05e);
}
impl IAdaptiveCard {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveCard {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72d0568c-a274-41cd-82a8-989d40b9b05e}");
}
impl ::core::convert::From<IAdaptiveCard> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveCard) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveCard) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveCard) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveCard) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAdaptiveCard {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCard_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Shell`*"]
pub struct IAdaptiveCardBuilderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdaptiveCardBuilderStatics {
    type Vtable = IAdaptiveCardBuilderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x766d8f08_d3fe_4347_a0bc_b9ea9a6dc28e);
}
impl IAdaptiveCardBuilderStatics {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<IAdaptiveCard>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e}");
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows::core::IUnknown {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows::core::IUnknown {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows::core::IInspectable {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows::core::IInspectable {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardBuilderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecurityAppManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISecurityAppManager {
    type Vtable = ISecurityAppManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityAppManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: SecurityAppKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailsuri: ::windows::core::RawPtr, registerperuser: bool, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::WindowId) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ShareWindowCommand) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ShareWindowCommand) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShareWindowCommandSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShareWindowCommandSourceStatics {
    type Vtable = IShareWindowCommandSourceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0eb6656_9cac_517c_b6c7_8ef715084295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics_abi(
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
pub struct ITaskbarManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITaskbarManager {
    type Vtable = ITaskbarManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITaskbarManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITaskbarManager2 {
    type Vtable = ITaskbarManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f0a06e_7b02_4911_918c_dee0bbd20ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, secondarytile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_StartScreen")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITaskbarManagerStatics {
    type Vtable = ITaskbarManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb32ab74_de52_4fe6_b7b6_95ff9f8395df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: SecurityAppKind = SecurityAppKind(0i32);
}
impl ::core::convert::From<i32> for SecurityAppKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SecurityAppKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SecurityAppKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
}
impl ::windows::core::DefaultType for SecurityAppKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SecurityAppManager(pub ::windows::core::IInspectable);
impl SecurityAppManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SecurityAppManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn Register<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, kind: SecurityAppKind, displayname: Param1, detailsuri: Param2, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, displayname.into_param().abi(), detailsuri.into_param().abi(), registerperuser, &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Unregister<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, kind: SecurityAppKind, guidregistration: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), kind, guidregistration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn UpdateState<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, kind: SecurityAppKind, guidregistration: Param1, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: Param4) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), kind, guidregistration.into_param().abi(), state, substatus, detailsuri.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SecurityAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.SecurityAppManager;{96ac500c-aed4-561d-bde8-953520343a2d})");
}
unsafe impl ::windows::core::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
impl ::windows::core::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
impl ::core::convert::From<SecurityAppManager> for ::windows::core::IUnknown {
    fn from(value: SecurityAppManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows::core::IUnknown {
    fn from(value: &SecurityAppManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SecurityAppManager> for ::windows::core::IInspectable {
    fn from(value: SecurityAppManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows::core::IInspectable {
    fn from(value: &SecurityAppManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SecurityAppManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SecurityAppManager {}
unsafe impl ::core::marker::Sync for SecurityAppManager {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct SecurityAppManagerContract(pub u8);
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: SecurityAppState = SecurityAppState(0i32);
    pub const Enabled: SecurityAppState = SecurityAppState(1i32);
}
impl ::core::convert::From<i32> for SecurityAppState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SecurityAppState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SecurityAppState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
}
impl ::windows::core::DefaultType for SecurityAppState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: SecurityAppSubstatus = SecurityAppSubstatus(0i32);
    pub const NoActionNeeded: SecurityAppSubstatus = SecurityAppSubstatus(1i32);
    pub const ActionRecommended: SecurityAppSubstatus = SecurityAppSubstatus(2i32);
    pub const ActionNeeded: SecurityAppSubstatus = SecurityAppSubstatus(3i32);
}
impl ::core::convert::From<i32> for SecurityAppSubstatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SecurityAppSubstatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
}
impl ::windows::core::DefaultType for SecurityAppSubstatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: ShareWindowCommand = ShareWindowCommand(0i32);
    pub const StartSharing: ShareWindowCommand = ShareWindowCommand(1i32);
    pub const StopSharing: ShareWindowCommand = ShareWindowCommand(2i32);
}
impl ::core::convert::From<i32> for ShareWindowCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ShareWindowCommand {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
}
impl ::windows::core::DefaultType for ShareWindowCommand {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ShareWindowCommandEventArgs(pub ::windows::core::IInspectable);
impl ShareWindowCommandEventArgs {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Command(&self) -> ::windows::core::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__: ShareWindowCommand = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ShareWindowCommand>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandEventArgs;{4578dc09-a523-5756-a995-e4feb991fff0})");
}
unsafe impl ::windows::core::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
impl ::windows::core::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandEventArgs {}
unsafe impl ::core::marker::Sync for ShareWindowCommandEventArgs {}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ShareWindowCommandSource(pub ::windows::core::IInspectable);
impl ShareWindowCommandSource {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn ReportCommandChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn CommandRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn RemoveCommandRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn CommandInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn RemoveCommandInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<ShareWindowCommandSource> {
        Self::IShareWindowCommandSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ShareWindowCommandSource>(result__)
        })
    }
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandSource;{cb3b7ae3-6b9c-561e-bccc-61e68e0abfef})");
}
unsafe impl ::windows::core::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
impl ::windows::core::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows::core::IUnknown {
    fn from(value: ShareWindowCommandSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows::core::IUnknown {
    fn from(value: &ShareWindowCommandSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows::core::IInspectable {
    fn from(value: ShareWindowCommandSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows::core::IInspectable {
    fn from(value: &ShareWindowCommandSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ShareWindowCommandSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandSource {}
unsafe impl ::core::marker::Sync for ShareWindowCommandSource {}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TaskbarManager(pub ::windows::core::IInspectable);
impl TaskbarManager {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn IsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn IsPinningAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn IsCurrentAppPinnedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    #[doc = "*Required features: `UI_Shell`, `ApplicationModel_Core`, `Foundation`*"]
    pub fn IsAppListEntryPinnedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn RequestPinCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    #[doc = "*Required features: `UI_Shell`, `ApplicationModel_Core`, `Foundation`*"]
    pub fn RequestPinAppListEntryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn GetDefault() -> ::windows::core::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TaskbarManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn IsSecondaryTilePinnedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    #[doc = "*Required features: `UI_Shell`, `Foundation`, `UI_StartScreen`*"]
    pub fn RequestPinSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::StartScreen::SecondaryTile>>(&self, secondarytile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), secondarytile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn TryUnpinSecondaryTileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tileid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TaskbarManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.TaskbarManager;{87490a19-1ad9-49f4-b2e8-86738dc5ac40})");
}
unsafe impl ::windows::core::Interface for TaskbarManager {
    type Vtable = ITaskbarManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
impl ::windows::core::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
impl ::core::convert::From<TaskbarManager> for ::windows::core::IUnknown {
    fn from(value: TaskbarManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows::core::IUnknown {
    fn from(value: &TaskbarManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TaskbarManager> for ::windows::core::IInspectable {
    fn from(value: TaskbarManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows::core::IInspectable {
    fn from(value: &TaskbarManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TaskbarManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TaskbarManager {}
unsafe impl ::core::marker::Sync for TaskbarManager {}
