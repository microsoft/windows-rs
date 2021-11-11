#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ArcadeStick(pub ::windows::core::IInspectable);
impl ArcadeStick {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetButtonLabel(&self, button: ArcadeStickButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<ArcadeStickReading> {
        let this = self;
        unsafe {
            let mut result__: ArcadeStickReading = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ArcadeStickReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn ArcadeStickAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ArcadeStick>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveArcadeStickAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn ArcadeStickRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ArcadeStick>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveArcadeStickRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`*"]
    pub fn ArcadeSticks() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ArcadeStick>>(result__)
        })
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn FromGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::core::Result<ArcadeStick> {
        Self::IArcadeStickStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<ArcadeStick>(result__)
        })
    }
    pub fn IArcadeStickStatics<R, F: FnOnce(&IArcadeStickStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ArcadeStick, IArcadeStickStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IArcadeStickStatics2<R, F: FnOnce(&IArcadeStickStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ArcadeStick, IArcadeStickStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ArcadeStick {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ArcadeStick;{b14a539d-befb-4c81-8051-15ecf3b13036})");
}
unsafe impl ::windows::core::Interface for ArcadeStick {
    type Vtable = IArcadeStick_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14a539d_befb_4c81_8051_15ecf3b13036);
}
impl ::windows::core::RuntimeName for ArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.ArcadeStick";
}
impl ::core::convert::From<ArcadeStick> for ::windows::core::IUnknown {
    fn from(value: ArcadeStick) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ArcadeStick> for ::windows::core::IUnknown {
    fn from(value: &ArcadeStick) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ArcadeStick> for ::windows::core::IInspectable {
    fn from(value: ArcadeStick) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ArcadeStick> for ::windows::core::IInspectable {
    fn from(value: &ArcadeStick) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ArcadeStick> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: ArcadeStick) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ArcadeStick> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: &ArcadeStick) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for &ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ArcadeStick> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: ArcadeStick) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ArcadeStick> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &ArcadeStick) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &ArcadeStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ArcadeStick {}
unsafe impl ::core::marker::Sync for ArcadeStick {}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ArcadeStickButtons(pub u32);
impl ArcadeStickButtons {
    pub const None: ArcadeStickButtons = ArcadeStickButtons(0u32);
    pub const StickUp: ArcadeStickButtons = ArcadeStickButtons(1u32);
    pub const StickDown: ArcadeStickButtons = ArcadeStickButtons(2u32);
    pub const StickLeft: ArcadeStickButtons = ArcadeStickButtons(4u32);
    pub const StickRight: ArcadeStickButtons = ArcadeStickButtons(8u32);
    pub const Action1: ArcadeStickButtons = ArcadeStickButtons(16u32);
    pub const Action2: ArcadeStickButtons = ArcadeStickButtons(32u32);
    pub const Action3: ArcadeStickButtons = ArcadeStickButtons(64u32);
    pub const Action4: ArcadeStickButtons = ArcadeStickButtons(128u32);
    pub const Action5: ArcadeStickButtons = ArcadeStickButtons(256u32);
    pub const Action6: ArcadeStickButtons = ArcadeStickButtons(512u32);
    pub const Special1: ArcadeStickButtons = ArcadeStickButtons(1024u32);
    pub const Special2: ArcadeStickButtons = ArcadeStickButtons(2048u32);
}
impl ::core::convert::From<u32> for ArcadeStickButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ArcadeStickButtons {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ArcadeStickButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ArcadeStickButtons;u4)");
}
impl ::windows::core::DefaultType for ArcadeStickButtons {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ArcadeStickButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ArcadeStickButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ArcadeStickButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ArcadeStickButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ArcadeStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct ArcadeStickReading {
    pub Timestamp: u64,
    pub Buttons: ArcadeStickButtons,
}
impl ArcadeStickReading {}
impl ::core::default::Default for ArcadeStickReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ArcadeStickReading {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ArcadeStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).finish()
    }
}
impl ::core::cmp::PartialEq for ArcadeStickReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons
    }
}
impl ::core::cmp::Eq for ArcadeStickReading {}
unsafe impl ::windows::core::Abi for ArcadeStickReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ArcadeStickReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.ArcadeStickReading;u8;enum(Windows.Gaming.Input.ArcadeStickButtons;u4))");
}
impl ::windows::core::DefaultType for ArcadeStickReading {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FlightStick(pub ::windows::core::IInspectable);
impl FlightStick {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn HatSwitchKind(&self) -> ::windows::core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__: GameControllerSwitchKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetButtonLabel(&self, button: FlightStickButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<FlightStickReading> {
        let this = self;
        unsafe {
            let mut result__: FlightStickReading = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FlightStickReading>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn FlightStickAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<FlightStick>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveFlightStickAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn FlightStickRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<FlightStick>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveFlightStickRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`*"]
    pub fn FlightSticks() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FlightStick>> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<FlightStick>>(result__)
        })
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn FromGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::core::Result<FlightStick> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<FlightStick>(result__)
        })
    }
    pub fn IFlightStickStatics<R, F: FnOnce(&IFlightStickStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FlightStick, IFlightStickStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FlightStick {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.FlightStick;{b4a2c01c-b83b-4459-a1a9-97b03c33da7c})");
}
unsafe impl ::windows::core::Interface for FlightStick {
    type Vtable = IFlightStick_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4a2c01c_b83b_4459_a1a9_97b03c33da7c);
}
impl ::windows::core::RuntimeName for FlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.FlightStick";
}
impl ::core::convert::From<FlightStick> for ::windows::core::IUnknown {
    fn from(value: FlightStick) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FlightStick> for ::windows::core::IUnknown {
    fn from(value: &FlightStick) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FlightStick> for ::windows::core::IInspectable {
    fn from(value: FlightStick) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FlightStick> for ::windows::core::IInspectable {
    fn from(value: &FlightStick) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<FlightStick> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: FlightStick) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FlightStick> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: &FlightStick) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for &FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FlightStick> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: FlightStick) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FlightStick> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &FlightStick) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &FlightStick {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FlightStick {}
unsafe impl ::core::marker::Sync for FlightStick {}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: FlightStickButtons = FlightStickButtons(0u32);
    pub const FirePrimary: FlightStickButtons = FlightStickButtons(1u32);
    pub const FireSecondary: FlightStickButtons = FlightStickButtons(2u32);
}
impl ::core::convert::From<u32> for FlightStickButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FlightStickButtons {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FlightStickButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.FlightStickButtons;u4)");
}
impl ::windows::core::DefaultType for FlightStickButtons {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for FlightStickButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for FlightStickButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for FlightStickButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for FlightStickButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for FlightStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct FlightStickReading {
    pub Timestamp: u64,
    pub Buttons: FlightStickButtons,
    pub HatSwitch: GameControllerSwitchPosition,
    pub Roll: f64,
    pub Pitch: f64,
    pub Yaw: f64,
    pub Throttle: f64,
}
impl FlightStickReading {}
impl ::core::default::Default for FlightStickReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FlightStickReading {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FlightStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("HatSwitch", &self.HatSwitch).field("Roll", &self.Roll).field("Pitch", &self.Pitch).field("Yaw", &self.Yaw).field("Throttle", &self.Throttle).finish()
    }
}
impl ::core::cmp::PartialEq for FlightStickReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons && self.HatSwitch == other.HatSwitch && self.Roll == other.Roll && self.Pitch == other.Pitch && self.Yaw == other.Yaw && self.Throttle == other.Throttle
    }
}
impl ::core::cmp::Eq for FlightStickReading {}
unsafe impl ::windows::core::Abi for FlightStickReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FlightStickReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.FlightStickReading;u8;enum(Windows.Gaming.Input.FlightStickButtons;u4);enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4);f8;f8;f8;f8)");
}
impl ::windows::core::DefaultType for FlightStickReading {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameControllerButtonLabel(pub i32);
impl GameControllerButtonLabel {
    pub const None: GameControllerButtonLabel = GameControllerButtonLabel(0i32);
    pub const XboxBack: GameControllerButtonLabel = GameControllerButtonLabel(1i32);
    pub const XboxStart: GameControllerButtonLabel = GameControllerButtonLabel(2i32);
    pub const XboxMenu: GameControllerButtonLabel = GameControllerButtonLabel(3i32);
    pub const XboxView: GameControllerButtonLabel = GameControllerButtonLabel(4i32);
    pub const XboxUp: GameControllerButtonLabel = GameControllerButtonLabel(5i32);
    pub const XboxDown: GameControllerButtonLabel = GameControllerButtonLabel(6i32);
    pub const XboxLeft: GameControllerButtonLabel = GameControllerButtonLabel(7i32);
    pub const XboxRight: GameControllerButtonLabel = GameControllerButtonLabel(8i32);
    pub const XboxA: GameControllerButtonLabel = GameControllerButtonLabel(9i32);
    pub const XboxB: GameControllerButtonLabel = GameControllerButtonLabel(10i32);
    pub const XboxX: GameControllerButtonLabel = GameControllerButtonLabel(11i32);
    pub const XboxY: GameControllerButtonLabel = GameControllerButtonLabel(12i32);
    pub const XboxLeftBumper: GameControllerButtonLabel = GameControllerButtonLabel(13i32);
    pub const XboxLeftTrigger: GameControllerButtonLabel = GameControllerButtonLabel(14i32);
    pub const XboxLeftStickButton: GameControllerButtonLabel = GameControllerButtonLabel(15i32);
    pub const XboxRightBumper: GameControllerButtonLabel = GameControllerButtonLabel(16i32);
    pub const XboxRightTrigger: GameControllerButtonLabel = GameControllerButtonLabel(17i32);
    pub const XboxRightStickButton: GameControllerButtonLabel = GameControllerButtonLabel(18i32);
    pub const XboxPaddle1: GameControllerButtonLabel = GameControllerButtonLabel(19i32);
    pub const XboxPaddle2: GameControllerButtonLabel = GameControllerButtonLabel(20i32);
    pub const XboxPaddle3: GameControllerButtonLabel = GameControllerButtonLabel(21i32);
    pub const XboxPaddle4: GameControllerButtonLabel = GameControllerButtonLabel(22i32);
    pub const Mode: GameControllerButtonLabel = GameControllerButtonLabel(23i32);
    pub const Select: GameControllerButtonLabel = GameControllerButtonLabel(24i32);
    pub const Menu: GameControllerButtonLabel = GameControllerButtonLabel(25i32);
    pub const View: GameControllerButtonLabel = GameControllerButtonLabel(26i32);
    pub const Back: GameControllerButtonLabel = GameControllerButtonLabel(27i32);
    pub const Start: GameControllerButtonLabel = GameControllerButtonLabel(28i32);
    pub const Options: GameControllerButtonLabel = GameControllerButtonLabel(29i32);
    pub const Share: GameControllerButtonLabel = GameControllerButtonLabel(30i32);
    pub const Up: GameControllerButtonLabel = GameControllerButtonLabel(31i32);
    pub const Down: GameControllerButtonLabel = GameControllerButtonLabel(32i32);
    pub const Left: GameControllerButtonLabel = GameControllerButtonLabel(33i32);
    pub const Right: GameControllerButtonLabel = GameControllerButtonLabel(34i32);
    pub const LetterA: GameControllerButtonLabel = GameControllerButtonLabel(35i32);
    pub const LetterB: GameControllerButtonLabel = GameControllerButtonLabel(36i32);
    pub const LetterC: GameControllerButtonLabel = GameControllerButtonLabel(37i32);
    pub const LetterL: GameControllerButtonLabel = GameControllerButtonLabel(38i32);
    pub const LetterR: GameControllerButtonLabel = GameControllerButtonLabel(39i32);
    pub const LetterX: GameControllerButtonLabel = GameControllerButtonLabel(40i32);
    pub const LetterY: GameControllerButtonLabel = GameControllerButtonLabel(41i32);
    pub const LetterZ: GameControllerButtonLabel = GameControllerButtonLabel(42i32);
    pub const Cross: GameControllerButtonLabel = GameControllerButtonLabel(43i32);
    pub const Circle: GameControllerButtonLabel = GameControllerButtonLabel(44i32);
    pub const Square: GameControllerButtonLabel = GameControllerButtonLabel(45i32);
    pub const Triangle: GameControllerButtonLabel = GameControllerButtonLabel(46i32);
    pub const LeftBumper: GameControllerButtonLabel = GameControllerButtonLabel(47i32);
    pub const LeftTrigger: GameControllerButtonLabel = GameControllerButtonLabel(48i32);
    pub const LeftStickButton: GameControllerButtonLabel = GameControllerButtonLabel(49i32);
    pub const Left1: GameControllerButtonLabel = GameControllerButtonLabel(50i32);
    pub const Left2: GameControllerButtonLabel = GameControllerButtonLabel(51i32);
    pub const Left3: GameControllerButtonLabel = GameControllerButtonLabel(52i32);
    pub const RightBumper: GameControllerButtonLabel = GameControllerButtonLabel(53i32);
    pub const RightTrigger: GameControllerButtonLabel = GameControllerButtonLabel(54i32);
    pub const RightStickButton: GameControllerButtonLabel = GameControllerButtonLabel(55i32);
    pub const Right1: GameControllerButtonLabel = GameControllerButtonLabel(56i32);
    pub const Right2: GameControllerButtonLabel = GameControllerButtonLabel(57i32);
    pub const Right3: GameControllerButtonLabel = GameControllerButtonLabel(58i32);
    pub const Paddle1: GameControllerButtonLabel = GameControllerButtonLabel(59i32);
    pub const Paddle2: GameControllerButtonLabel = GameControllerButtonLabel(60i32);
    pub const Paddle3: GameControllerButtonLabel = GameControllerButtonLabel(61i32);
    pub const Paddle4: GameControllerButtonLabel = GameControllerButtonLabel(62i32);
    pub const Plus: GameControllerButtonLabel = GameControllerButtonLabel(63i32);
    pub const Minus: GameControllerButtonLabel = GameControllerButtonLabel(64i32);
    pub const DownLeftArrow: GameControllerButtonLabel = GameControllerButtonLabel(65i32);
    pub const DialLeft: GameControllerButtonLabel = GameControllerButtonLabel(66i32);
    pub const DialRight: GameControllerButtonLabel = GameControllerButtonLabel(67i32);
    pub const Suspension: GameControllerButtonLabel = GameControllerButtonLabel(68i32);
}
impl ::core::convert::From<i32> for GameControllerButtonLabel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameControllerButtonLabel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameControllerButtonLabel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerButtonLabel;i4)");
}
impl ::windows::core::DefaultType for GameControllerButtonLabel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: GameControllerSwitchKind = GameControllerSwitchKind(0i32);
    pub const FourWay: GameControllerSwitchKind = GameControllerSwitchKind(1i32);
    pub const EightWay: GameControllerSwitchKind = GameControllerSwitchKind(2i32);
}
impl ::core::convert::From<i32> for GameControllerSwitchKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameControllerSwitchKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameControllerSwitchKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchKind;i4)");
}
impl ::windows::core::DefaultType for GameControllerSwitchKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameControllerSwitchPosition(pub i32);
impl GameControllerSwitchPosition {
    pub const Center: GameControllerSwitchPosition = GameControllerSwitchPosition(0i32);
    pub const Up: GameControllerSwitchPosition = GameControllerSwitchPosition(1i32);
    pub const UpRight: GameControllerSwitchPosition = GameControllerSwitchPosition(2i32);
    pub const Right: GameControllerSwitchPosition = GameControllerSwitchPosition(3i32);
    pub const DownRight: GameControllerSwitchPosition = GameControllerSwitchPosition(4i32);
    pub const Down: GameControllerSwitchPosition = GameControllerSwitchPosition(5i32);
    pub const DownLeft: GameControllerSwitchPosition = GameControllerSwitchPosition(6i32);
    pub const Left: GameControllerSwitchPosition = GameControllerSwitchPosition(7i32);
    pub const UpLeft: GameControllerSwitchPosition = GameControllerSwitchPosition(8i32);
}
impl ::core::convert::From<i32> for GameControllerSwitchPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameControllerSwitchPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameControllerSwitchPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4)");
}
impl ::windows::core::DefaultType for GameControllerSwitchPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Gamepad(pub ::windows::core::IInspectable);
impl Gamepad {
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Vibration(&self) -> ::windows::core::Result<GamepadVibration> {
        let this = self;
        unsafe {
            let mut result__: GamepadVibration = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GamepadVibration>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn SetVibration<'a, Param0: ::windows::core::IntoParam<'a, GamepadVibration>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<GamepadReading> {
        let this = self;
        unsafe {
            let mut result__: GamepadReading = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GamepadReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn GamepadAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<Gamepad>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveGamepadAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn GamepadRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<Gamepad>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveGamepadRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`*"]
    pub fn Gamepads() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Gamepad>> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Gamepad>>(result__)
        })
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetButtonLabel(&self, button: GamepadButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = &::windows::core::Interface::cast::<IGamepad2>(self)?;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn FromGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::core::Result<Gamepad> {
        Self::IGamepadStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<Gamepad>(result__)
        })
    }
    pub fn IGamepadStatics<R, F: FnOnce(&IGamepadStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Gamepad, IGamepadStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGamepadStatics2<R, F: FnOnce(&IGamepadStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Gamepad, IGamepadStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Gamepad {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Gamepad;{bc7bb43c-0a69-3903-9e9d-a50f86a45de5})");
}
unsafe impl ::windows::core::Interface for Gamepad {
    type Vtable = IGamepad_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc7bb43c_0a69_3903_9e9d_a50f86a45de5);
}
impl ::windows::core::RuntimeName for Gamepad {
    const NAME: &'static str = "Windows.Gaming.Input.Gamepad";
}
impl ::core::convert::From<Gamepad> for ::windows::core::IUnknown {
    fn from(value: Gamepad) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Gamepad> for ::windows::core::IUnknown {
    fn from(value: &Gamepad) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Gamepad> for ::windows::core::IInspectable {
    fn from(value: Gamepad) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Gamepad> for ::windows::core::IInspectable {
    fn from(value: &Gamepad) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Gamepad> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: Gamepad) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Gamepad> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: &Gamepad) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for &Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<Gamepad> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: Gamepad) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Gamepad> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &Gamepad) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &Gamepad {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Gamepad {}
unsafe impl ::core::marker::Sync for Gamepad {}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GamepadButtons(pub u32);
impl GamepadButtons {
    pub const None: GamepadButtons = GamepadButtons(0u32);
    pub const Menu: GamepadButtons = GamepadButtons(1u32);
    pub const View: GamepadButtons = GamepadButtons(2u32);
    pub const A: GamepadButtons = GamepadButtons(4u32);
    pub const B: GamepadButtons = GamepadButtons(8u32);
    pub const X: GamepadButtons = GamepadButtons(16u32);
    pub const Y: GamepadButtons = GamepadButtons(32u32);
    pub const DPadUp: GamepadButtons = GamepadButtons(64u32);
    pub const DPadDown: GamepadButtons = GamepadButtons(128u32);
    pub const DPadLeft: GamepadButtons = GamepadButtons(256u32);
    pub const DPadRight: GamepadButtons = GamepadButtons(512u32);
    pub const LeftShoulder: GamepadButtons = GamepadButtons(1024u32);
    pub const RightShoulder: GamepadButtons = GamepadButtons(2048u32);
    pub const LeftThumbstick: GamepadButtons = GamepadButtons(4096u32);
    pub const RightThumbstick: GamepadButtons = GamepadButtons(8192u32);
    pub const Paddle1: GamepadButtons = GamepadButtons(16384u32);
    pub const Paddle2: GamepadButtons = GamepadButtons(32768u32);
    pub const Paddle3: GamepadButtons = GamepadButtons(65536u32);
    pub const Paddle4: GamepadButtons = GamepadButtons(131072u32);
}
impl ::core::convert::From<u32> for GamepadButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GamepadButtons {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GamepadButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GamepadButtons;u4)");
}
impl ::windows::core::DefaultType for GamepadButtons {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for GamepadButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GamepadButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GamepadButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GamepadButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GamepadButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct GamepadReading {
    pub Timestamp: u64,
    pub Buttons: GamepadButtons,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
    pub LeftThumbstickX: f64,
    pub LeftThumbstickY: f64,
    pub RightThumbstickX: f64,
    pub RightThumbstickY: f64,
}
impl GamepadReading {}
impl ::core::default::Default for GamepadReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GamepadReading {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GamepadReading")
            .field("Timestamp", &self.Timestamp)
            .field("Buttons", &self.Buttons)
            .field("LeftTrigger", &self.LeftTrigger)
            .field("RightTrigger", &self.RightTrigger)
            .field("LeftThumbstickX", &self.LeftThumbstickX)
            .field("LeftThumbstickY", &self.LeftThumbstickY)
            .field("RightThumbstickX", &self.RightThumbstickX)
            .field("RightThumbstickY", &self.RightThumbstickY)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GamepadReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons && self.LeftTrigger == other.LeftTrigger && self.RightTrigger == other.RightTrigger && self.LeftThumbstickX == other.LeftThumbstickX && self.LeftThumbstickY == other.LeftThumbstickY && self.RightThumbstickX == other.RightThumbstickX && self.RightThumbstickY == other.RightThumbstickY
    }
}
impl ::core::cmp::Eq for GamepadReading {}
unsafe impl ::windows::core::Abi for GamepadReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GamepadReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadReading;u8;enum(Windows.Gaming.Input.GamepadButtons;u4);f8;f8;f8;f8;f8;f8)");
}
impl ::windows::core::DefaultType for GamepadReading {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct GamepadVibration {
    pub LeftMotor: f64,
    pub RightMotor: f64,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
}
impl GamepadVibration {}
impl ::core::default::Default for GamepadVibration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GamepadVibration {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GamepadVibration").field("LeftMotor", &self.LeftMotor).field("RightMotor", &self.RightMotor).field("LeftTrigger", &self.LeftTrigger).field("RightTrigger", &self.RightTrigger).finish()
    }
}
impl ::core::cmp::PartialEq for GamepadVibration {
    fn eq(&self, other: &Self) -> bool {
        self.LeftMotor == other.LeftMotor && self.RightMotor == other.RightMotor && self.LeftTrigger == other.LeftTrigger && self.RightTrigger == other.RightTrigger
    }
}
impl ::core::cmp::Eq for GamepadVibration {}
unsafe impl ::windows::core::Abi for GamepadVibration {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GamepadVibration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadVibration;f8;f8;f8;f8)");
}
impl ::windows::core::DefaultType for GamepadVibration {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct GamingInputPreviewContract(pub u8);
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Headset(pub ::windows::core::IInspectable);
impl Headset {
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn CaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn RenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Headset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Headset;{3fd156ef-6925-3fa8-9181-029c5223ae3b})");
}
unsafe impl ::windows::core::Interface for Headset {
    type Vtable = IHeadset_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd156ef_6925_3fa8_9181_029c5223ae3b);
}
impl ::windows::core::RuntimeName for Headset {
    const NAME: &'static str = "Windows.Gaming.Input.Headset";
}
impl ::core::convert::From<Headset> for ::windows::core::IUnknown {
    fn from(value: Headset) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Headset> for ::windows::core::IUnknown {
    fn from(value: &Headset) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Headset {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Headset {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Headset> for ::windows::core::IInspectable {
    fn from(value: Headset) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Headset> for ::windows::core::IInspectable {
    fn from(value: &Headset) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Headset {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Headset {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Headset> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: Headset) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Headset> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &Headset) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for Headset {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &Headset {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Headset {}
unsafe impl ::core::marker::Sync for Headset {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IArcadeStick(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IArcadeStick {
    type Vtable = IArcadeStick_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14a539d_befb_4c81_8051_15ecf3b13036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStick_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ArcadeStickReading) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IArcadeStickStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IArcadeStickStatics {
    type Vtable = IArcadeStickStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c37b8c8_37b1_4ad8_9458_200f1a30018e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IArcadeStickStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IArcadeStickStatics2 {
    type Vtable = IArcadeStickStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52b5d744_bb86_445a_b59c_596f0e2a49df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlightStick(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlightStick {
    type Vtable = IFlightStick_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4a2c01c_b83b_4459_a1a9_97b03c33da7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStick_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FlightStickReading) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFlightStickStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFlightStickStatics {
    type Vtable = IFlightStickStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5514924a_fecc_435e_83dc_5cec8a18a520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStickStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct IGameController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameController {
    type Vtable = IGameController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1baf6522_5f64_42c5_8267_b9fe2215bfbd);
}
impl IGameController {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGameController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1baf6522-5f64-42c5-8267-b9fe2215bfbd}");
}
impl ::core::convert::From<IGameController> for ::windows::core::IUnknown {
    fn from(value: IGameController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGameController> for ::windows::core::IUnknown {
    fn from(value: &IGameController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGameController> for ::windows::core::IInspectable {
    fn from(value: IGameController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameController> for ::windows::core::IInspectable {
    fn from(value: &IGameController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct IGameControllerBatteryInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameControllerBatteryInfo {
    type Vtable = IGameControllerBatteryInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcecc681_3963_4da6_955d_553f3b6f6161);
}
impl IGameControllerBatteryInfo {
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGameControllerBatteryInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dcecc681-3963-4da6-955d-553f3b6f6161}");
}
impl ::core::convert::From<IGameControllerBatteryInfo> for ::windows::core::IUnknown {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGameControllerBatteryInfo> for ::windows::core::IUnknown {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGameControllerBatteryInfo> for ::windows::core::IInspectable {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGameControllerBatteryInfo> for ::windows::core::IInspectable {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerBatteryInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Power")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGamepad(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGamepad {
    type Vtable = IGamepad_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc7bb43c_0a69_3903_9e9d_a50f86a45de5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GamepadVibration) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GamepadVibration) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GamepadReading) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGamepad2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGamepad2 {
    type Vtable = IGamepad2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c1689bd_5915_4245_b0c0_c89fae0308ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGamepadStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGamepadStatics {
    type Vtable = IGamepadStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bbce529_d49c_39e9_9560_e47dde96b7c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGamepadStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGamepadStatics2 {
    type Vtable = IGamepadStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42676dc5_0856_47c4_9213_b395504c3a3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHeadset(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHeadset {
    type Vtable = IHeadset_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd156ef_6925_3fa8_9181_029c5223ae3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadset_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRacingWheel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRacingWheel {
    type Vtable = IRacingWheel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf546656f_e106_4c82_a90f_554012904b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Gaming_Input_ForceFeedback")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_ForceFeedback"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RacingWheelReading) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRacingWheelStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRacingWheelStatics {
    type Vtable = IRacingWheelStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ac12cd5_581b_4936_9f94_69f1e6514c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRacingWheelStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRacingWheelStatics2 {
    type Vtable = IRacingWheelStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe666bcaa_edfd_4323_a9f6_3c384048d1ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRawGameController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRawGameController {
    type Vtable = IRawGameController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cad6d91_a7e1_4f71_9a78_33e9c5dfea62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRawGameController2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRawGameController2 {
    type Vtable = IRawGameController2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c0c035_bb73_4756_a787_3ed6bea617bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Haptics", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRawGameControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRawGameControllerStatics {
    type Vtable = IRawGameControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb8d0792_e95a_4b19_afc7_0a59f8bf759e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUINavigationController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUINavigationController {
    type Vtable = IUINavigationController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5aeefdd_f50e_4a55_8cdc_d33229548175);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UINavigationReading) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUINavigationControllerStatics {
    type Vtable = IUINavigationControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f14930a_f6f8_4a48_8d89_94786cca0c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUINavigationControllerStatics2 {
    type Vtable = IUINavigationControllerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0cb28e3_b20b_4b0b_9ed4_f3d53cec0de4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OptionalUINavigationButtons(pub u32);
impl OptionalUINavigationButtons {
    pub const None: OptionalUINavigationButtons = OptionalUINavigationButtons(0u32);
    pub const Context1: OptionalUINavigationButtons = OptionalUINavigationButtons(1u32);
    pub const Context2: OptionalUINavigationButtons = OptionalUINavigationButtons(2u32);
    pub const Context3: OptionalUINavigationButtons = OptionalUINavigationButtons(4u32);
    pub const Context4: OptionalUINavigationButtons = OptionalUINavigationButtons(8u32);
    pub const PageUp: OptionalUINavigationButtons = OptionalUINavigationButtons(16u32);
    pub const PageDown: OptionalUINavigationButtons = OptionalUINavigationButtons(32u32);
    pub const PageLeft: OptionalUINavigationButtons = OptionalUINavigationButtons(64u32);
    pub const PageRight: OptionalUINavigationButtons = OptionalUINavigationButtons(128u32);
    pub const ScrollUp: OptionalUINavigationButtons = OptionalUINavigationButtons(256u32);
    pub const ScrollDown: OptionalUINavigationButtons = OptionalUINavigationButtons(512u32);
    pub const ScrollLeft: OptionalUINavigationButtons = OptionalUINavigationButtons(1024u32);
    pub const ScrollRight: OptionalUINavigationButtons = OptionalUINavigationButtons(2048u32);
}
impl ::core::convert::From<u32> for OptionalUINavigationButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OptionalUINavigationButtons {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OptionalUINavigationButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4)");
}
impl ::windows::core::DefaultType for OptionalUINavigationButtons {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for OptionalUINavigationButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for OptionalUINavigationButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for OptionalUINavigationButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for OptionalUINavigationButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for OptionalUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RacingWheel(pub ::windows::core::IInspectable);
impl RacingWheel {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn HasClutch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn HasHandbrake(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn HasPatternShifter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn MaxPatternShifterGear(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn MaxWheelAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    #[doc = "*Required features: `Gaming_Input`, `Gaming_Input_ForceFeedback`*"]
    pub fn WheelMotor(&self) -> ::windows::core::Result<ForceFeedback::ForceFeedbackMotor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedback::ForceFeedbackMotor>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetButtonLabel(&self, button: RacingWheelButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<RacingWheelReading> {
        let this = self;
        unsafe {
            let mut result__: RacingWheelReading = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RacingWheelReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RacingWheelAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<RacingWheel>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveRacingWheelAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RacingWheelRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<RacingWheel>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveRacingWheelRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`*"]
    pub fn RacingWheels() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RacingWheel>>(result__)
        })
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn FromGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::core::Result<RacingWheel> {
        Self::IRacingWheelStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<RacingWheel>(result__)
        })
    }
    pub fn IRacingWheelStatics<R, F: FnOnce(&IRacingWheelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RacingWheel, IRacingWheelStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRacingWheelStatics2<R, F: FnOnce(&IRacingWheelStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RacingWheel, IRacingWheelStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RacingWheel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RacingWheel;{f546656f-e106-4c82-a90f-554012904b85})");
}
unsafe impl ::windows::core::Interface for RacingWheel {
    type Vtable = IRacingWheel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf546656f_e106_4c82_a90f_554012904b85);
}
impl ::windows::core::RuntimeName for RacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.RacingWheel";
}
impl ::core::convert::From<RacingWheel> for ::windows::core::IUnknown {
    fn from(value: RacingWheel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RacingWheel> for ::windows::core::IUnknown {
    fn from(value: &RacingWheel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RacingWheel> for ::windows::core::IInspectable {
    fn from(value: RacingWheel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RacingWheel> for ::windows::core::IInspectable {
    fn from(value: &RacingWheel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RacingWheel> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: RacingWheel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RacingWheel> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: &RacingWheel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for &RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RacingWheel> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: RacingWheel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RacingWheel> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &RacingWheel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &RacingWheel {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RacingWheel {}
unsafe impl ::core::marker::Sync for RacingWheel {}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RacingWheelButtons(pub u32);
impl RacingWheelButtons {
    pub const None: RacingWheelButtons = RacingWheelButtons(0u32);
    pub const PreviousGear: RacingWheelButtons = RacingWheelButtons(1u32);
    pub const NextGear: RacingWheelButtons = RacingWheelButtons(2u32);
    pub const DPadUp: RacingWheelButtons = RacingWheelButtons(4u32);
    pub const DPadDown: RacingWheelButtons = RacingWheelButtons(8u32);
    pub const DPadLeft: RacingWheelButtons = RacingWheelButtons(16u32);
    pub const DPadRight: RacingWheelButtons = RacingWheelButtons(32u32);
    pub const Button1: RacingWheelButtons = RacingWheelButtons(64u32);
    pub const Button2: RacingWheelButtons = RacingWheelButtons(128u32);
    pub const Button3: RacingWheelButtons = RacingWheelButtons(256u32);
    pub const Button4: RacingWheelButtons = RacingWheelButtons(512u32);
    pub const Button5: RacingWheelButtons = RacingWheelButtons(1024u32);
    pub const Button6: RacingWheelButtons = RacingWheelButtons(2048u32);
    pub const Button7: RacingWheelButtons = RacingWheelButtons(4096u32);
    pub const Button8: RacingWheelButtons = RacingWheelButtons(8192u32);
    pub const Button9: RacingWheelButtons = RacingWheelButtons(16384u32);
    pub const Button10: RacingWheelButtons = RacingWheelButtons(32768u32);
    pub const Button11: RacingWheelButtons = RacingWheelButtons(65536u32);
    pub const Button12: RacingWheelButtons = RacingWheelButtons(131072u32);
    pub const Button13: RacingWheelButtons = RacingWheelButtons(262144u32);
    pub const Button14: RacingWheelButtons = RacingWheelButtons(524288u32);
    pub const Button15: RacingWheelButtons = RacingWheelButtons(1048576u32);
    pub const Button16: RacingWheelButtons = RacingWheelButtons(2097152u32);
}
impl ::core::convert::From<u32> for RacingWheelButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RacingWheelButtons {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RacingWheelButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RacingWheelButtons;u4)");
}
impl ::windows::core::DefaultType for RacingWheelButtons {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for RacingWheelButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RacingWheelButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RacingWheelButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RacingWheelButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RacingWheelButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct RacingWheelReading {
    pub Timestamp: u64,
    pub Buttons: RacingWheelButtons,
    pub PatternShifterGear: i32,
    pub Wheel: f64,
    pub Throttle: f64,
    pub Brake: f64,
    pub Clutch: f64,
    pub Handbrake: f64,
}
impl RacingWheelReading {}
impl ::core::default::Default for RacingWheelReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RacingWheelReading {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RacingWheelReading")
            .field("Timestamp", &self.Timestamp)
            .field("Buttons", &self.Buttons)
            .field("PatternShifterGear", &self.PatternShifterGear)
            .field("Wheel", &self.Wheel)
            .field("Throttle", &self.Throttle)
            .field("Brake", &self.Brake)
            .field("Clutch", &self.Clutch)
            .field("Handbrake", &self.Handbrake)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RacingWheelReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons && self.PatternShifterGear == other.PatternShifterGear && self.Wheel == other.Wheel && self.Throttle == other.Throttle && self.Brake == other.Brake && self.Clutch == other.Clutch && self.Handbrake == other.Handbrake
    }
}
impl ::core::cmp::Eq for RacingWheelReading {}
unsafe impl ::windows::core::Abi for RacingWheelReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RacingWheelReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.RacingWheelReading;u8;enum(Windows.Gaming.Input.RacingWheelButtons;u4);i4;f8;f8;f8;f8;f8)");
}
impl ::windows::core::DefaultType for RacingWheelReading {
    type DefaultType = Self;
}
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RawGameController(pub ::windows::core::IInspectable);
impl RawGameController {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn AxisCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn ButtonCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`, `Gaming_Input_ForceFeedback`*"]
    pub fn ForceFeedbackMotors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn SwitchCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetButtonLabel(&self, buttonindex: i32) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), buttonindex, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetCurrentReading(&self, buttonarray: &mut [<bool as ::windows::core::DefaultType>::DefaultType], switcharray: &mut [<GameControllerSwitchPosition as ::windows::core::DefaultType>::DefaultType], axisarray: &mut [<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), buttonarray.len() as u32, ::core::mem::transmute_copy(&buttonarray), switcharray.len() as u32, ::core::mem::transmute_copy(&switcharray), axisarray.len() as u32, ::core::mem::transmute_copy(&axisarray), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetSwitchKind(&self, switchindex: i32) -> ::windows::core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__: GameControllerSwitchKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), switchindex, &mut result__).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RawGameControllerAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<RawGameController>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveRawGameControllerAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RawGameControllerRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<RawGameController>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveRawGameControllerRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`*"]
    pub fn RawGameControllers() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RawGameController>> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RawGameController>>(result__)
        })
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn FromGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::core::Result<RawGameController> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<RawGameController>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Haptics`, `Foundation_Collections`*"]
    pub fn SimpleHapticsControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>> {
        let this = &::windows::core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IRawGameControllerStatics<R, F: FnOnce(&IRawGameControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RawGameController, IRawGameControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RawGameController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RawGameController;{7cad6d91-a7e1-4f71-9a78-33e9c5dfea62})");
}
unsafe impl ::windows::core::Interface for RawGameController {
    type Vtable = IRawGameController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cad6d91_a7e1_4f71_9a78_33e9c5dfea62);
}
impl ::windows::core::RuntimeName for RawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.RawGameController";
}
impl ::core::convert::From<RawGameController> for ::windows::core::IUnknown {
    fn from(value: RawGameController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RawGameController> for ::windows::core::IUnknown {
    fn from(value: &RawGameController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RawGameController> for ::windows::core::IInspectable {
    fn from(value: RawGameController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RawGameController> for ::windows::core::IInspectable {
    fn from(value: &RawGameController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<RawGameController> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: RawGameController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RawGameController> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: &RawGameController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for &RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RawGameController> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: RawGameController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RawGameController> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &RawGameController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &RawGameController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RawGameController {}
unsafe impl ::core::marker::Sync for RawGameController {}
#[doc = "*Required features: `Gaming_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RequiredUINavigationButtons(pub u32);
impl RequiredUINavigationButtons {
    pub const None: RequiredUINavigationButtons = RequiredUINavigationButtons(0u32);
    pub const Menu: RequiredUINavigationButtons = RequiredUINavigationButtons(1u32);
    pub const View: RequiredUINavigationButtons = RequiredUINavigationButtons(2u32);
    pub const Accept: RequiredUINavigationButtons = RequiredUINavigationButtons(4u32);
    pub const Cancel: RequiredUINavigationButtons = RequiredUINavigationButtons(8u32);
    pub const Up: RequiredUINavigationButtons = RequiredUINavigationButtons(16u32);
    pub const Down: RequiredUINavigationButtons = RequiredUINavigationButtons(32u32);
    pub const Left: RequiredUINavigationButtons = RequiredUINavigationButtons(64u32);
    pub const Right: RequiredUINavigationButtons = RequiredUINavigationButtons(128u32);
}
impl ::core::convert::From<u32> for RequiredUINavigationButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RequiredUINavigationButtons {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RequiredUINavigationButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4)");
}
impl ::windows::core::DefaultType for RequiredUINavigationButtons {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for RequiredUINavigationButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RequiredUINavigationButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RequiredUINavigationButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RequiredUINavigationButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RequiredUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Gaming_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UINavigationController(pub ::windows::core::IInspectable);
impl UINavigationController {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`, `System`*"]
    pub fn UserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUserChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Gaming_Input`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<UINavigationReading> {
        let this = self;
        unsafe {
            let mut result__: UINavigationReading = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UINavigationReading>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn UINavigationControllerAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<UINavigationController>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUINavigationControllerAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn UINavigationControllerRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<UINavigationController>>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation`*"]
    pub fn RemoveUINavigationControllerRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Gaming_Input`, `Foundation_Collections`*"]
    pub fn UINavigationControllers() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UINavigationController>>(result__)
        })
    }
    #[cfg(feature = "Devices_Power")]
    #[doc = "*Required features: `Gaming_Input`, `Devices_Power`*"]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[doc = "*Required features: `Gaming_Input`*"]
    pub fn FromGameController<'a, Param0: ::windows::core::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::core::Result<UINavigationController> {
        Self::IUINavigationControllerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<UINavigationController>(result__)
        })
    }
    pub fn IUINavigationControllerStatics<R, F: FnOnce(&IUINavigationControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UINavigationController, IUINavigationControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUINavigationControllerStatics2<R, F: FnOnce(&IUINavigationControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UINavigationController, IUINavigationControllerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UINavigationController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.UINavigationController;{e5aeefdd-f50e-4a55-8cdc-d33229548175})");
}
unsafe impl ::windows::core::Interface for UINavigationController {
    type Vtable = IUINavigationController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5aeefdd_f50e_4a55_8cdc_d33229548175);
}
impl ::windows::core::RuntimeName for UINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.UINavigationController";
}
impl ::core::convert::From<UINavigationController> for ::windows::core::IUnknown {
    fn from(value: UINavigationController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UINavigationController> for ::windows::core::IUnknown {
    fn from(value: &UINavigationController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UINavigationController> for ::windows::core::IInspectable {
    fn from(value: UINavigationController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UINavigationController> for ::windows::core::IInspectable {
    fn from(value: &UINavigationController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<UINavigationController> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: UINavigationController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UINavigationController> for IGameController {
    type Error = ::windows::core::Error;
    fn try_from(value: &UINavigationController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameController> for &UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameController> {
        ::core::convert::TryInto::<IGameController>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<UINavigationController> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: UINavigationController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UINavigationController> for IGameControllerBatteryInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &UINavigationController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGameControllerBatteryInfo> for &UINavigationController {
    fn into_param(self) -> ::windows::core::Param<'a, IGameControllerBatteryInfo> {
        ::core::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UINavigationController {}
unsafe impl ::core::marker::Sync for UINavigationController {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Gaming_Input`*"]
pub struct UINavigationReading {
    pub Timestamp: u64,
    pub RequiredButtons: RequiredUINavigationButtons,
    pub OptionalButtons: OptionalUINavigationButtons,
}
impl UINavigationReading {}
impl ::core::default::Default for UINavigationReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for UINavigationReading {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("UINavigationReading").field("Timestamp", &self.Timestamp).field("RequiredButtons", &self.RequiredButtons).field("OptionalButtons", &self.OptionalButtons).finish()
    }
}
impl ::core::cmp::PartialEq for UINavigationReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.RequiredButtons == other.RequiredButtons && self.OptionalButtons == other.OptionalButtons
    }
}
impl ::core::cmp::Eq for UINavigationReading {}
unsafe impl ::windows::core::Abi for UINavigationReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UINavigationReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.UINavigationReading;u8;enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4);enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4))");
}
impl ::windows::core::DefaultType for UINavigationReading {
    type DefaultType = Self;
}
