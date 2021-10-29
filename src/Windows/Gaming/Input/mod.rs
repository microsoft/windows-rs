#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ArcadeStick(::windows::runtime::IInspectable);
impl ArcadeStick {
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: ArcadeStickButtons) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::runtime::Result<ArcadeStickReading> {
        let this = self;
        unsafe {
            let mut result__: ArcadeStickReading = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ArcadeStickReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ArcadeStickAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ArcadeStick>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveArcadeStickAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn ArcadeStickRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ArcadeStick>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveArcadeStickRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ArcadeSticks() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ArcadeStick>>(result__)
        })
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn FromGameController<'a, Param0: ::windows::runtime::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::runtime::Result<ArcadeStick> {
        Self::IArcadeStickStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<ArcadeStick>(result__)
        })
    }
    pub fn IArcadeStickStatics<R, F: FnOnce(&IArcadeStickStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ArcadeStick, IArcadeStickStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IArcadeStickStatics2<R, F: FnOnce(&IArcadeStickStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ArcadeStick, IArcadeStickStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ArcadeStick {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ArcadeStick;{b14a539d-befb-4c81-8051-15ecf3b13036})");
}
unsafe impl ::windows::runtime::Interface for ArcadeStick {
    type Vtable = IArcadeStick_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2974438301, 48891, 19585, [128, 81, 21, 236, 243, 177, 48, 54]);
}
impl ::windows::runtime::RuntimeName for ArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.ArcadeStick";
}
impl ::std::convert::From<ArcadeStick> for ::windows::runtime::IUnknown {
    fn from(value: ArcadeStick) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ArcadeStick> for ::windows::runtime::IUnknown {
    fn from(value: &ArcadeStick) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ArcadeStick> for ::windows::runtime::IInspectable {
    fn from(value: ArcadeStick) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ArcadeStick> for ::windows::runtime::IInspectable {
    fn from(value: &ArcadeStick) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ArcadeStick> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ArcadeStick) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ArcadeStick> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ArcadeStick) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for &ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::std::convert::TryInto::<IGameController>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<ArcadeStick> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ArcadeStick) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ArcadeStick> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ArcadeStick) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &ArcadeStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ArcadeStick {}
unsafe impl ::std::marker::Sync for ArcadeStick {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for ArcadeStickButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ArcadeStickButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ArcadeStickButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ArcadeStickButtons;u4)");
}
impl ::std::ops::BitOr for ArcadeStickButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ArcadeStickButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ArcadeStickButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ArcadeStickButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ArcadeStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ArcadeStickReading {
    pub Timestamp: u64,
    pub Buttons: ArcadeStickButtons,
}
impl ArcadeStickReading {}
impl ::std::default::Default for ArcadeStickReading {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ArcadeStickReading {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ArcadeStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).finish()
    }
}
impl ::std::cmp::PartialEq for ArcadeStickReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons
    }
}
impl ::std::cmp::Eq for ArcadeStickReading {}
unsafe impl ::windows::runtime::Abi for ArcadeStickReading {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ArcadeStickReading {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.ArcadeStickReading;u8;enum(Windows.Gaming.Input.ArcadeStickButtons;u4))");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FlightStick(::windows::runtime::IInspectable);
impl FlightStick {
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn HatSwitchKind(&self) -> ::windows::runtime::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__: GameControllerSwitchKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: FlightStickButtons) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::runtime::Result<FlightStickReading> {
        let this = self;
        unsafe {
            let mut result__: FlightStickReading = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FlightStickReading>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlightStickAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<FlightStick>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFlightStickAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn FlightStickRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<FlightStick>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFlightStickRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FlightSticks() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<FlightStick>> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<FlightStick>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows::runtime::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::runtime::Result<FlightStick> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<FlightStick>(result__)
        })
    }
    pub fn IFlightStickStatics<R, F: FnOnce(&IFlightStickStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FlightStick, IFlightStickStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FlightStick {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.FlightStick;{b4a2c01c-b83b-4459-a1a9-97b03c33da7c})");
}
unsafe impl ::windows::runtime::Interface for FlightStick {
    type Vtable = IFlightStick_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3030564892, 47163, 17497, [161, 169, 151, 176, 60, 51, 218, 124]);
}
impl ::windows::runtime::RuntimeName for FlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.FlightStick";
}
impl ::std::convert::From<FlightStick> for ::windows::runtime::IUnknown {
    fn from(value: FlightStick) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FlightStick> for ::windows::runtime::IUnknown {
    fn from(value: &FlightStick) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FlightStick> for ::windows::runtime::IInspectable {
    fn from(value: FlightStick) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FlightStick> for ::windows::runtime::IInspectable {
    fn from(value: &FlightStick) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<FlightStick> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FlightStick) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FlightStick> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FlightStick) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for &FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::std::convert::TryInto::<IGameController>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<FlightStick> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FlightStick) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FlightStick> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FlightStick) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &FlightStick {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for FlightStick {}
unsafe impl ::std::marker::Sync for FlightStick {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: FlightStickButtons = FlightStickButtons(0u32);
    pub const FirePrimary: FlightStickButtons = FlightStickButtons(1u32);
    pub const FireSecondary: FlightStickButtons = FlightStickButtons(2u32);
}
impl ::std::convert::From<u32> for FlightStickButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FlightStickButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FlightStickButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.FlightStickButtons;u4)");
}
impl ::std::ops::BitOr for FlightStickButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FlightStickButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FlightStickButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FlightStickButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FlightStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl ::std::default::Default for FlightStickReading {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FlightStickReading {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FlightStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("HatSwitch", &self.HatSwitch).field("Roll", &self.Roll).field("Pitch", &self.Pitch).field("Yaw", &self.Yaw).field("Throttle", &self.Throttle).finish()
    }
}
impl ::std::cmp::PartialEq for FlightStickReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons && self.HatSwitch == other.HatSwitch && self.Roll == other.Roll && self.Pitch == other.Pitch && self.Yaw == other.Yaw && self.Throttle == other.Throttle
    }
}
impl ::std::cmp::Eq for FlightStickReading {}
unsafe impl ::windows::runtime::Abi for FlightStickReading {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FlightStickReading {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.FlightStickReading;u8;enum(Windows.Gaming.Input.FlightStickButtons;u4);enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4);f8;f8;f8;f8)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for GameControllerButtonLabel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameControllerButtonLabel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameControllerButtonLabel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerButtonLabel;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: GameControllerSwitchKind = GameControllerSwitchKind(0i32);
    pub const FourWay: GameControllerSwitchKind = GameControllerSwitchKind(1i32);
    pub const EightWay: GameControllerSwitchKind = GameControllerSwitchKind(2i32);
}
impl ::std::convert::From<i32> for GameControllerSwitchKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameControllerSwitchKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameControllerSwitchKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchKind;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for GameControllerSwitchPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameControllerSwitchPosition {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameControllerSwitchPosition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Gamepad(::windows::runtime::IInspectable);
impl Gamepad {
    pub fn Vibration(&self) -> ::windows::runtime::Result<GamepadVibration> {
        let this = self;
        unsafe {
            let mut result__: GamepadVibration = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GamepadVibration>(result__)
        }
    }
    pub fn SetVibration<'a, Param0: ::windows::runtime::IntoParam<'a, GamepadVibration>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetCurrentReading(&self) -> ::windows::runtime::Result<GamepadReading> {
        let this = self;
        unsafe {
            let mut result__: GamepadReading = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GamepadReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GamepadAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<Gamepad>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGamepadAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn GamepadRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<Gamepad>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGamepadRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gamepads() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<Gamepad>> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Gamepad>>(result__)
        })
    }
    pub fn GetButtonLabel(&self, button: GamepadButtons) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = &::windows::runtime::Interface::cast::<IGamepad2>(self)?;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn FromGameController<'a, Param0: ::windows::runtime::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::runtime::Result<Gamepad> {
        Self::IGamepadStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<Gamepad>(result__)
        })
    }
    pub fn IGamepadStatics<R, F: FnOnce(&IGamepadStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Gamepad, IGamepadStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGamepadStatics2<R, F: FnOnce(&IGamepadStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Gamepad, IGamepadStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Gamepad {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Gamepad;{bc7bb43c-0a69-3903-9e9d-a50f86a45de5})");
}
unsafe impl ::windows::runtime::Interface for Gamepad {
    type Vtable = IGamepad_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3162223676, 2665, 14595, [158, 157, 165, 15, 134, 164, 93, 229]);
}
impl ::windows::runtime::RuntimeName for Gamepad {
    const NAME: &'static str = "Windows.Gaming.Input.Gamepad";
}
impl ::std::convert::From<Gamepad> for ::windows::runtime::IUnknown {
    fn from(value: Gamepad) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Gamepad> for ::windows::runtime::IUnknown {
    fn from(value: &Gamepad) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Gamepad> for ::windows::runtime::IInspectable {
    fn from(value: Gamepad) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Gamepad> for ::windows::runtime::IInspectable {
    fn from(value: &Gamepad) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<Gamepad> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Gamepad) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Gamepad> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Gamepad) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for &Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::std::convert::TryInto::<IGameController>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<Gamepad> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Gamepad) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Gamepad> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Gamepad) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &Gamepad {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Gamepad {}
unsafe impl ::std::marker::Sync for Gamepad {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for GamepadButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GamepadButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GamepadButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GamepadButtons;u4)");
}
impl ::std::ops::BitOr for GamepadButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GamepadButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GamepadButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GamepadButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GamepadButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl ::std::default::Default for GamepadReading {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GamepadReading {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for GamepadReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons && self.LeftTrigger == other.LeftTrigger && self.RightTrigger == other.RightTrigger && self.LeftThumbstickX == other.LeftThumbstickX && self.LeftThumbstickY == other.LeftThumbstickY && self.RightThumbstickX == other.RightThumbstickX && self.RightThumbstickY == other.RightThumbstickY
    }
}
impl ::std::cmp::Eq for GamepadReading {}
unsafe impl ::windows::runtime::Abi for GamepadReading {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GamepadReading {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadReading;u8;enum(Windows.Gaming.Input.GamepadButtons;u4);f8;f8;f8;f8;f8;f8)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GamepadVibration {
    pub LeftMotor: f64,
    pub RightMotor: f64,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
}
impl GamepadVibration {}
impl ::std::default::Default for GamepadVibration {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GamepadVibration {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GamepadVibration").field("LeftMotor", &self.LeftMotor).field("RightMotor", &self.RightMotor).field("LeftTrigger", &self.LeftTrigger).field("RightTrigger", &self.RightTrigger).finish()
    }
}
impl ::std::cmp::PartialEq for GamepadVibration {
    fn eq(&self, other: &Self) -> bool {
        self.LeftMotor == other.LeftMotor && self.RightMotor == other.RightMotor && self.LeftTrigger == other.LeftTrigger && self.RightTrigger == other.RightTrigger
    }
}
impl ::std::cmp::Eq for GamepadVibration {}
unsafe impl ::windows::runtime::Abi for GamepadVibration {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GamepadVibration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadVibration;f8;f8;f8;f8)");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct GamingInputPreviewContract(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Headset(::windows::runtime::IInspectable);
impl Headset {
    pub fn CaptureDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RenderDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Headset {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Headset;{3fd156ef-6925-3fa8-9181-029c5223ae3b})");
}
unsafe impl ::windows::runtime::Interface for Headset {
    type Vtable = IHeadset_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1070683887, 26917, 16296, [145, 129, 2, 156, 82, 35, 174, 59]);
}
impl ::windows::runtime::RuntimeName for Headset {
    const NAME: &'static str = "Windows.Gaming.Input.Headset";
}
impl ::std::convert::From<Headset> for ::windows::runtime::IUnknown {
    fn from(value: Headset) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Headset> for ::windows::runtime::IUnknown {
    fn from(value: &Headset) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Headset {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Headset {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Headset> for ::windows::runtime::IInspectable {
    fn from(value: Headset) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Headset> for ::windows::runtime::IInspectable {
    fn from(value: &Headset) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Headset {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Headset {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<Headset> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Headset) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Headset> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Headset) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for Headset {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &Headset {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Headset {}
unsafe impl ::std::marker::Sync for Headset {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IArcadeStick(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IArcadeStick {
    type Vtable = IArcadeStick_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2974438301, 48891, 19585, [128, 81, 21, 236, 243, 177, 48, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStick_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ArcadeStickReading) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IArcadeStickStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IArcadeStickStatics {
    type Vtable = IArcadeStickStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1547155656, 14257, 19160, [148, 88, 32, 15, 26, 48, 1, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IArcadeStickStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IArcadeStickStatics2 {
    type Vtable = IArcadeStickStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1387648836, 48006, 17498, [181, 156, 89, 111, 14, 42, 73, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamecontroller: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IFlightStick(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFlightStick {
    type Vtable = IFlightStick_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3030564892, 47163, 17497, [161, 169, 151, 176, 60, 51, 218, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStick_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameControllerSwitchKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FlightStickReading) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IFlightStickStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFlightStickStatics {
    type Vtable = IFlightStickStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427411530, 65228, 17246, [131, 220, 92, 236, 138, 24, 165, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStickStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamecontroller: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGameController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameController {
    type Vtable = IGameController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(464479522, 24420, 17093, [130, 103, 185, 254, 34, 21, 191, 189]);
}
impl IGameController {
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGameController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1baf6522-5f64-42c5-8267-b9fe2215bfbd}");
}
impl ::std::convert::From<IGameController> for ::windows::runtime::IUnknown {
    fn from(value: IGameController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGameController> for ::windows::runtime::IUnknown {
    fn from(value: &IGameController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IGameController> for ::windows::runtime::IInspectable {
    fn from(value: IGameController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameController> for ::windows::runtime::IInspectable {
    fn from(value: &IGameController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGameControllerBatteryInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameControllerBatteryInfo {
    type Vtable = IGameControllerBatteryInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3706504833, 14691, 19878, [149, 93, 85, 63, 59, 111, 97, 97]);
}
impl IGameControllerBatteryInfo {
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGameControllerBatteryInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{dcecc681-3963-4da6-955d-553f3b6f6161}");
}
impl ::std::convert::From<IGameControllerBatteryInfo> for ::windows::runtime::IUnknown {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGameControllerBatteryInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IGameControllerBatteryInfo> for ::windows::runtime::IInspectable {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGameControllerBatteryInfo> for ::windows::runtime::IInspectable {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IGameControllerBatteryInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerBatteryInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Power")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Power"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGamepad(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGamepad {
    type Vtable = IGamepad_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3162223676, 2665, 14595, [158, 157, 165, 15, 134, 164, 93, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GamepadVibration) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: GamepadVibration) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GamepadReading) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGamepad2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGamepad2 {
    type Vtable = IGamepad2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1008110013, 22805, 16965, [176, 192, 200, 159, 174, 3, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGamepadStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGamepadStatics {
    type Vtable = IGamepadStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2344412457, 54428, 14825, [149, 96, 228, 125, 222, 150, 183, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGamepadStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGamepadStatics2 {
    type Vtable = IGamepadStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1114074565, 2134, 18372, [146, 19, 179, 149, 80, 76, 58, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamecontroller: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHeadset(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHeadset {
    type Vtable = IHeadset_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1070683887, 26917, 16296, [145, 129, 2, 156, 82, 35, 174, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadset_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRacingWheel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRacingWheel {
    type Vtable = IRacingWheel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4115031407, 57606, 19586, [169, 15, 85, 64, 18, 144, 75, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Gaming_Input_ForceFeedback")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Gaming_Input_ForceFeedback"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RacingWheelReading) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRacingWheelStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRacingWheelStatics {
    type Vtable = IRacingWheelStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(985738453, 22555, 18742, [159, 148, 105, 241, 230, 81, 76, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRacingWheelStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRacingWheelStatics2 {
    type Vtable = IRacingWheelStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3865492650, 60925, 17187, [169, 246, 60, 56, 64, 72, 209, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamecontroller: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRawGameController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRawGameController {
    type Vtable = IRawGameController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2091740561, 42977, 20337, [154, 120, 51, 233, 197, 223, 234, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRawGameController2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRawGameController2 {
    type Vtable = IRawGameController2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1136705589, 47987, 18262, [167, 135, 62, 214, 190, 166, 23, 189]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Haptics", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRawGameControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRawGameControllerStatics {
    type Vtable = IRawGameControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3951888274, 59738, 19225, [175, 199, 10, 89, 248, 191, 117, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamecontroller: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUINavigationController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUINavigationController {
    type Vtable = IUINavigationController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3853447133, 62734, 19029, [140, 220, 211, 50, 41, 84, 129, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UINavigationReading) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUINavigationControllerStatics {
    type Vtable = IUINavigationControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(789877514, 63224, 19016, [141, 137, 148, 120, 108, 202, 12, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUINavigationControllerStatics2 {
    type Vtable = IUINavigationControllerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3771410659, 45579, 19211, [158, 212, 243, 213, 60, 236, 13, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gamecontroller: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for OptionalUINavigationButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OptionalUINavigationButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for OptionalUINavigationButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4)");
}
impl ::std::ops::BitOr for OptionalUINavigationButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OptionalUINavigationButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OptionalUINavigationButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OptionalUINavigationButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OptionalUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RacingWheel(::windows::runtime::IInspectable);
impl RacingWheel {
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn HasClutch(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasHandbrake(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasPatternShifter(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPatternShifterGear(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MaxWheelAngle(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    pub fn WheelMotor(&self) -> ::windows::runtime::Result<ForceFeedback::ForceFeedbackMotor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ForceFeedback::ForceFeedbackMotor>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: RacingWheelButtons) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::runtime::Result<RacingWheelReading> {
        let this = self;
        unsafe {
            let mut result__: RacingWheelReading = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RacingWheelReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RacingWheelAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<RacingWheel>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRacingWheelAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RacingWheelRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<RacingWheel>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRacingWheelRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RacingWheels() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RacingWheel>>(result__)
        })
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn FromGameController<'a, Param0: ::windows::runtime::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::runtime::Result<RacingWheel> {
        Self::IRacingWheelStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<RacingWheel>(result__)
        })
    }
    pub fn IRacingWheelStatics<R, F: FnOnce(&IRacingWheelStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RacingWheel, IRacingWheelStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRacingWheelStatics2<R, F: FnOnce(&IRacingWheelStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RacingWheel, IRacingWheelStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RacingWheel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RacingWheel;{f546656f-e106-4c82-a90f-554012904b85})");
}
unsafe impl ::windows::runtime::Interface for RacingWheel {
    type Vtable = IRacingWheel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4115031407, 57606, 19586, [169, 15, 85, 64, 18, 144, 75, 133]);
}
impl ::windows::runtime::RuntimeName for RacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.RacingWheel";
}
impl ::std::convert::From<RacingWheel> for ::windows::runtime::IUnknown {
    fn from(value: RacingWheel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RacingWheel> for ::windows::runtime::IUnknown {
    fn from(value: &RacingWheel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RacingWheel> for ::windows::runtime::IInspectable {
    fn from(value: RacingWheel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RacingWheel> for ::windows::runtime::IInspectable {
    fn from(value: &RacingWheel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RacingWheel> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RacingWheel) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RacingWheel> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RacingWheel) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for &RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::std::convert::TryInto::<IGameController>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<RacingWheel> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RacingWheel) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RacingWheel> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RacingWheel) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &RacingWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RacingWheel {}
unsafe impl ::std::marker::Sync for RacingWheel {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for RacingWheelButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RacingWheelButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RacingWheelButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RacingWheelButtons;u4)");
}
impl ::std::ops::BitOr for RacingWheelButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RacingWheelButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RacingWheelButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RacingWheelButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RacingWheelButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl ::std::default::Default for RacingWheelReading {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RacingWheelReading {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for RacingWheelReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.Buttons == other.Buttons && self.PatternShifterGear == other.PatternShifterGear && self.Wheel == other.Wheel && self.Throttle == other.Throttle && self.Brake == other.Brake && self.Clutch == other.Clutch && self.Handbrake == other.Handbrake
    }
}
impl ::std::cmp::Eq for RacingWheelReading {}
unsafe impl ::windows::runtime::Abi for RacingWheelReading {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RacingWheelReading {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.RacingWheelReading;u8;enum(Windows.Gaming.Input.RacingWheelButtons;u4);i4;f8;f8;f8;f8;f8)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RawGameController(::windows::runtime::IInspectable);
impl RawGameController {
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn AxisCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ButtonCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    pub fn ForceFeedbackMotors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SwitchCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn GetButtonLabel(&self, buttonindex: i32) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), buttonindex, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self, buttonarray: &mut [<bool as ::windows::runtime::Abi>::DefaultType], switcharray: &mut [<GameControllerSwitchPosition as ::windows::runtime::Abi>::DefaultType], axisarray: &mut [<f64 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), buttonarray.len() as u32, ::std::mem::transmute_copy(&buttonarray), switcharray.len() as u32, ::std::mem::transmute_copy(&switcharray), axisarray.len() as u32, ::std::mem::transmute_copy(&axisarray), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn GetSwitchKind(&self, switchindex: i32) -> ::windows::runtime::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__: GameControllerSwitchKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), switchindex, &mut result__).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RawGameControllerAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<RawGameController>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawGameControllerAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RawGameControllerRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<RawGameController>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawGameControllerRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RawGameControllers() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<RawGameController>> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RawGameController>>(result__)
        })
    }
    pub fn FromGameController<'a, Param0: ::windows::runtime::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::runtime::Result<RawGameController> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<RawGameController>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    pub fn SimpleHapticsControllers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>> {
        let this = &::windows::runtime::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>>(result__)
        }
    }
    pub fn NonRoamableId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IRawGameControllerStatics<R, F: FnOnce(&IRawGameControllerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RawGameController, IRawGameControllerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RawGameController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RawGameController;{7cad6d91-a7e1-4f71-9a78-33e9c5dfea62})");
}
unsafe impl ::windows::runtime::Interface for RawGameController {
    type Vtable = IRawGameController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2091740561, 42977, 20337, [154, 120, 51, 233, 197, 223, 234, 98]);
}
impl ::windows::runtime::RuntimeName for RawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.RawGameController";
}
impl ::std::convert::From<RawGameController> for ::windows::runtime::IUnknown {
    fn from(value: RawGameController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RawGameController> for ::windows::runtime::IUnknown {
    fn from(value: &RawGameController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RawGameController> for ::windows::runtime::IInspectable {
    fn from(value: RawGameController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RawGameController> for ::windows::runtime::IInspectable {
    fn from(value: &RawGameController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RawGameController> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RawGameController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RawGameController> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RawGameController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for &RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::std::convert::TryInto::<IGameController>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<RawGameController> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RawGameController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RawGameController> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RawGameController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &RawGameController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RawGameController {}
unsafe impl ::std::marker::Sync for RawGameController {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for RequiredUINavigationButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RequiredUINavigationButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RequiredUINavigationButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4)");
}
impl ::std::ops::BitOr for RequiredUINavigationButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RequiredUINavigationButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RequiredUINavigationButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RequiredUINavigationButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RequiredUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UINavigationController(::windows::runtime::IInspectable);
impl UINavigationController {
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Headset(&self) -> ::windows::runtime::Result<Headset> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::runtime::Result<UINavigationReading> {
        let this = self;
        unsafe {
            let mut result__: UINavigationReading = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UINavigationReading>(result__)
        }
    }
    pub fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> ::windows::runtime::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__: GameControllerButtonLabel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), button, &mut result__).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UINavigationControllerAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<UINavigationController>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUINavigationControllerAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn UINavigationControllerRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<UINavigationController>>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUINavigationControllerRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UINavigationControllers() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UINavigationController>>(result__)
        })
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::runtime::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::runtime::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn FromGameController<'a, Param0: ::windows::runtime::IntoParam<'a, IGameController>>(gamecontroller: Param0) -> ::windows::runtime::Result<UINavigationController> {
        Self::IUINavigationControllerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), gamecontroller.into_param().abi(), &mut result__).from_abi::<UINavigationController>(result__)
        })
    }
    pub fn IUINavigationControllerStatics<R, F: FnOnce(&IUINavigationControllerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UINavigationController, IUINavigationControllerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUINavigationControllerStatics2<R, F: FnOnce(&IUINavigationControllerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UINavigationController, IUINavigationControllerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UINavigationController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.UINavigationController;{e5aeefdd-f50e-4a55-8cdc-d33229548175})");
}
unsafe impl ::windows::runtime::Interface for UINavigationController {
    type Vtable = IUINavigationController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3853447133, 62734, 19029, [140, 220, 211, 50, 41, 84, 129, 117]);
}
impl ::windows::runtime::RuntimeName for UINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.UINavigationController";
}
impl ::std::convert::From<UINavigationController> for ::windows::runtime::IUnknown {
    fn from(value: UINavigationController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UINavigationController> for ::windows::runtime::IUnknown {
    fn from(value: &UINavigationController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UINavigationController> for ::windows::runtime::IInspectable {
    fn from(value: UINavigationController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UINavigationController> for ::windows::runtime::IInspectable {
    fn from(value: &UINavigationController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<UINavigationController> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UINavigationController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UINavigationController> for IGameController {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UINavigationController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameController> for &UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameController> {
        ::std::convert::TryInto::<IGameController>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<UINavigationController> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UINavigationController) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UINavigationController> for IGameControllerBatteryInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UINavigationController) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IGameControllerBatteryInfo> for &UINavigationController {
    fn into_param(self) -> ::windows::runtime::Param<'a, IGameControllerBatteryInfo> {
        ::std::convert::TryInto::<IGameControllerBatteryInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for UINavigationController {}
unsafe impl ::std::marker::Sync for UINavigationController {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct UINavigationReading {
    pub Timestamp: u64,
    pub RequiredButtons: RequiredUINavigationButtons,
    pub OptionalButtons: OptionalUINavigationButtons,
}
impl UINavigationReading {}
impl ::std::default::Default for UINavigationReading {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UINavigationReading {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UINavigationReading").field("Timestamp", &self.Timestamp).field("RequiredButtons", &self.RequiredButtons).field("OptionalButtons", &self.OptionalButtons).finish()
    }
}
impl ::std::cmp::PartialEq for UINavigationReading {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.RequiredButtons == other.RequiredButtons && self.OptionalButtons == other.OptionalButtons
    }
}
impl ::std::cmp::Eq for UINavigationReading {}
unsafe impl ::windows::runtime::Abi for UINavigationReading {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UINavigationReading {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.UINavigationReading;u8;enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4);enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4))");
}
