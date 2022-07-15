#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct ArcadeStick(::windows::core::IUnknown);
impl ArcadeStick {
    pub fn GetButtonLabel(&self, button: ArcadeStickButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetButtonLabel)(::windows::core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<ArcadeStickReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ArcadeStickReading>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ArcadeStickAdded<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<ArcadeStick>>>,
    {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ArcadeStickAdded)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveArcadeStickAdded(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveArcadeStickAdded)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ArcadeStickRemoved<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<ArcadeStick>>>,
    {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ArcadeStickRemoved)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveArcadeStickRemoved(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveArcadeStickRemoved)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ArcadeSticks() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ArcadeSticks)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ArcadeStick>>(result__)
        })
    }
    pub fn FromGameController<'a, P0, E0>(gamecontroller: P0) -> ::windows::core::Result<ArcadeStick>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IArcadeStickStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromGameController)(::windows::core::Interface::as_raw(this), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<ArcadeStick>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IArcadeStickStatics<R, F: FnOnce(&IArcadeStickStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ArcadeStick, IArcadeStickStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IArcadeStickStatics2<R, F: FnOnce(&IArcadeStickStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ArcadeStick, IArcadeStickStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ArcadeStick {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ArcadeStick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ArcadeStick {}
impl ::core::fmt::Debug for ArcadeStick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ArcadeStick").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ArcadeStick {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ArcadeStick;{b14a539d-befb-4c81-8051-15ecf3b13036})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ArcadeStick {
    type Vtable = IArcadeStick_Vtbl;
    const IID: ::windows::core::GUID = <IArcadeStick as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.ArcadeStick";
}
impl ::core::convert::From<ArcadeStick> for ::windows::core::IUnknown {
    fn from(value: ArcadeStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ArcadeStick> for ::windows::core::IUnknown {
    fn from(value: &ArcadeStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ArcadeStick> for &::windows::core::IUnknown {
    fn from(value: &ArcadeStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ArcadeStick> for ::windows::core::IInspectable {
    fn from(value: ArcadeStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ArcadeStick> for ::windows::core::IInspectable {
    fn from(value: &ArcadeStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ArcadeStick> for &::windows::core::IInspectable {
    fn from(value: &ArcadeStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&ArcadeStick> for ::windows::core::InParam<'a, IGameController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ArcadeStick) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&ArcadeStick> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ArcadeStick) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ArcadeStick {}
unsafe impl ::core::marker::Sync for ArcadeStick {}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ArcadeStickButtons(pub u32);
impl ArcadeStickButtons {
    pub const None: Self = Self(0u32);
    pub const StickUp: Self = Self(1u32);
    pub const StickDown: Self = Self(2u32);
    pub const StickLeft: Self = Self(4u32);
    pub const StickRight: Self = Self(8u32);
    pub const Action1: Self = Self(16u32);
    pub const Action2: Self = Self(32u32);
    pub const Action3: Self = Self(64u32);
    pub const Action4: Self = Self(128u32);
    pub const Action5: Self = Self(256u32);
    pub const Action6: Self = Self(512u32);
    pub const Special1: Self = Self(1024u32);
    pub const Special2: Self = Self(2048u32);
}
impl ::core::marker::Copy for ArcadeStickButtons {}
impl ::core::clone::Clone for ArcadeStickButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ArcadeStickButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ArcadeStickButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for ArcadeStickButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ArcadeStickButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ArcadeStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ArcadeStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ArcadeStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ArcadeStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ArcadeStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ArcadeStickButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ArcadeStickButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct ArcadeStickReading {
    pub Timestamp: u64,
    pub Buttons: ArcadeStickButtons,
}
impl ::core::marker::Copy for ArcadeStickReading {}
impl ::core::clone::Clone for ArcadeStickReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ArcadeStickReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ArcadeStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).finish()
    }
}
unsafe impl ::windows::core::Abi for ArcadeStickReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ArcadeStickReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.ArcadeStickReading;u8;enum(Windows.Gaming.Input.ArcadeStickButtons;u4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ArcadeStickReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ArcadeStickReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for ArcadeStickReading {}
impl ::core::default::Default for ArcadeStickReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct FlightStick(::windows::core::IUnknown);
impl FlightStick {
    pub fn HatSwitchKind(&self) -> ::windows::core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HatSwitchKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: FlightStickButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetButtonLabel)(::windows::core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<FlightStickReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FlightStickReading>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlightStickAdded<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<FlightStick>>>,
    {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FlightStickAdded)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFlightStickAdded(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveFlightStickAdded)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlightStickRemoved<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<FlightStick>>>,
    {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FlightStickRemoved)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFlightStickRemoved(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveFlightStickRemoved)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FlightSticks() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FlightStick>> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FlightSticks)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<FlightStick>>(result__)
        })
    }
    pub fn FromGameController<'a, P0, E0>(gamecontroller: P0) -> ::windows::core::Result<FlightStick>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromGameController)(::windows::core::Interface::as_raw(this), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<FlightStick>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IFlightStickStatics<R, F: FnOnce(&IFlightStickStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FlightStick, IFlightStickStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FlightStick {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FlightStick {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlightStick {}
impl ::core::fmt::Debug for FlightStick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlightStick").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FlightStick {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.FlightStick;{b4a2c01c-b83b-4459-a1a9-97b03c33da7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FlightStick {
    type Vtable = IFlightStick_Vtbl;
    const IID: ::windows::core::GUID = <IFlightStick as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.FlightStick";
}
impl ::core::convert::From<FlightStick> for ::windows::core::IUnknown {
    fn from(value: FlightStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlightStick> for ::windows::core::IUnknown {
    fn from(value: &FlightStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FlightStick> for &::windows::core::IUnknown {
    fn from(value: &FlightStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FlightStick> for ::windows::core::IInspectable {
    fn from(value: FlightStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlightStick> for ::windows::core::IInspectable {
    fn from(value: &FlightStick) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FlightStick> for &::windows::core::IInspectable {
    fn from(value: &FlightStick) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&FlightStick> for ::windows::core::InParam<'a, IGameController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FlightStick) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&FlightStick> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FlightStick) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FlightStick {}
unsafe impl ::core::marker::Sync for FlightStick {}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: Self = Self(0u32);
    pub const FirePrimary: Self = Self(1u32);
    pub const FireSecondary: Self = Self(2u32);
}
impl ::core::marker::Copy for FlightStickButtons {}
impl ::core::clone::Clone for FlightStickButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlightStickButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FlightStickButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for FlightStickButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlightStickButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FlightStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FlightStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FlightStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FlightStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FlightStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for FlightStickButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.FlightStickButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct FlightStickReading {
    pub Timestamp: u64,
    pub Buttons: FlightStickButtons,
    pub HatSwitch: GameControllerSwitchPosition,
    pub Roll: f64,
    pub Pitch: f64,
    pub Yaw: f64,
    pub Throttle: f64,
}
impl ::core::marker::Copy for FlightStickReading {}
impl ::core::clone::Clone for FlightStickReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FlightStickReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FlightStickReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("HatSwitch", &self.HatSwitch).field("Roll", &self.Roll).field("Pitch", &self.Pitch).field("Yaw", &self.Yaw).field("Throttle", &self.Throttle).finish()
    }
}
unsafe impl ::windows::core::Abi for FlightStickReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FlightStickReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.FlightStickReading;u8;enum(Windows.Gaming.Input.FlightStickButtons;u4);enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4);f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for FlightStickReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FlightStickReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for FlightStickReading {}
impl ::core::default::Default for FlightStickReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameControllerButtonLabel(pub i32);
impl GameControllerButtonLabel {
    pub const None: Self = Self(0i32);
    pub const XboxBack: Self = Self(1i32);
    pub const XboxStart: Self = Self(2i32);
    pub const XboxMenu: Self = Self(3i32);
    pub const XboxView: Self = Self(4i32);
    pub const XboxUp: Self = Self(5i32);
    pub const XboxDown: Self = Self(6i32);
    pub const XboxLeft: Self = Self(7i32);
    pub const XboxRight: Self = Self(8i32);
    pub const XboxA: Self = Self(9i32);
    pub const XboxB: Self = Self(10i32);
    pub const XboxX: Self = Self(11i32);
    pub const XboxY: Self = Self(12i32);
    pub const XboxLeftBumper: Self = Self(13i32);
    pub const XboxLeftTrigger: Self = Self(14i32);
    pub const XboxLeftStickButton: Self = Self(15i32);
    pub const XboxRightBumper: Self = Self(16i32);
    pub const XboxRightTrigger: Self = Self(17i32);
    pub const XboxRightStickButton: Self = Self(18i32);
    pub const XboxPaddle1: Self = Self(19i32);
    pub const XboxPaddle2: Self = Self(20i32);
    pub const XboxPaddle3: Self = Self(21i32);
    pub const XboxPaddle4: Self = Self(22i32);
    pub const Mode: Self = Self(23i32);
    pub const Select: Self = Self(24i32);
    pub const Menu: Self = Self(25i32);
    pub const View: Self = Self(26i32);
    pub const Back: Self = Self(27i32);
    pub const Start: Self = Self(28i32);
    pub const Options: Self = Self(29i32);
    pub const Share: Self = Self(30i32);
    pub const Up: Self = Self(31i32);
    pub const Down: Self = Self(32i32);
    pub const Left: Self = Self(33i32);
    pub const Right: Self = Self(34i32);
    pub const LetterA: Self = Self(35i32);
    pub const LetterB: Self = Self(36i32);
    pub const LetterC: Self = Self(37i32);
    pub const LetterL: Self = Self(38i32);
    pub const LetterR: Self = Self(39i32);
    pub const LetterX: Self = Self(40i32);
    pub const LetterY: Self = Self(41i32);
    pub const LetterZ: Self = Self(42i32);
    pub const Cross: Self = Self(43i32);
    pub const Circle: Self = Self(44i32);
    pub const Square: Self = Self(45i32);
    pub const Triangle: Self = Self(46i32);
    pub const LeftBumper: Self = Self(47i32);
    pub const LeftTrigger: Self = Self(48i32);
    pub const LeftStickButton: Self = Self(49i32);
    pub const Left1: Self = Self(50i32);
    pub const Left2: Self = Self(51i32);
    pub const Left3: Self = Self(52i32);
    pub const RightBumper: Self = Self(53i32);
    pub const RightTrigger: Self = Self(54i32);
    pub const RightStickButton: Self = Self(55i32);
    pub const Right1: Self = Self(56i32);
    pub const Right2: Self = Self(57i32);
    pub const Right3: Self = Self(58i32);
    pub const Paddle1: Self = Self(59i32);
    pub const Paddle2: Self = Self(60i32);
    pub const Paddle3: Self = Self(61i32);
    pub const Paddle4: Self = Self(62i32);
    pub const Plus: Self = Self(63i32);
    pub const Minus: Self = Self(64i32);
    pub const DownLeftArrow: Self = Self(65i32);
    pub const DialLeft: Self = Self(66i32);
    pub const DialRight: Self = Self(67i32);
    pub const Suspension: Self = Self(68i32);
}
impl ::core::marker::Copy for GameControllerButtonLabel {}
impl ::core::clone::Clone for GameControllerButtonLabel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameControllerButtonLabel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameControllerButtonLabel {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameControllerButtonLabel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameControllerButtonLabel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameControllerButtonLabel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerButtonLabel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: Self = Self(0i32);
    pub const FourWay: Self = Self(1i32);
    pub const EightWay: Self = Self(2i32);
}
impl ::core::marker::Copy for GameControllerSwitchKind {}
impl ::core::clone::Clone for GameControllerSwitchKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameControllerSwitchKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameControllerSwitchKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameControllerSwitchKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameControllerSwitchKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameControllerSwitchKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameControllerSwitchPosition(pub i32);
impl GameControllerSwitchPosition {
    pub const Center: Self = Self(0i32);
    pub const Up: Self = Self(1i32);
    pub const UpRight: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const DownRight: Self = Self(4i32);
    pub const Down: Self = Self(5i32);
    pub const DownLeft: Self = Self(6i32);
    pub const Left: Self = Self(7i32);
    pub const UpLeft: Self = Self(8i32);
}
impl ::core::marker::Copy for GameControllerSwitchPosition {}
impl ::core::clone::Clone for GameControllerSwitchPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameControllerSwitchPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameControllerSwitchPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameControllerSwitchPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameControllerSwitchPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameControllerSwitchPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct Gamepad(::windows::core::IUnknown);
impl Gamepad {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn Vibration(&self) -> ::windows::core::Result<GamepadVibration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Vibration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GamepadVibration>(result__)
        }
    }
    pub fn SetVibration(&self, value: GamepadVibration) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVibration)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<GamepadReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GamepadReading>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: GamepadButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = &::windows::core::Interface::cast::<IGamepad2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetButtonLabel)(::windows::core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GamepadAdded<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<Gamepad>>>,
    {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GamepadAdded)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGamepadAdded(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveGamepadAdded)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GamepadRemoved<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<Gamepad>>>,
    {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GamepadRemoved)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGamepadRemoved(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveGamepadRemoved)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gamepads() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Gamepad>> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Gamepads)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<Gamepad>>(result__)
        })
    }
    pub fn FromGameController<'a, P0, E0>(gamecontroller: P0) -> ::windows::core::Result<Gamepad>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGamepadStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromGameController)(::windows::core::Interface::as_raw(this), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<Gamepad>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGamepadStatics<R, F: FnOnce(&IGamepadStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Gamepad, IGamepadStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGamepadStatics2<R, F: FnOnce(&IGamepadStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Gamepad, IGamepadStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Gamepad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Gamepad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Gamepad {}
impl ::core::fmt::Debug for Gamepad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Gamepad").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Gamepad {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Gamepad;{bc7bb43c-0a69-3903-9e9d-a50f86a45de5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Gamepad {
    type Vtable = IGamepad_Vtbl;
    const IID: ::windows::core::GUID = <IGamepad as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Gamepad {
    const NAME: &'static str = "Windows.Gaming.Input.Gamepad";
}
impl ::core::convert::From<Gamepad> for ::windows::core::IUnknown {
    fn from(value: Gamepad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Gamepad> for ::windows::core::IUnknown {
    fn from(value: &Gamepad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Gamepad> for &::windows::core::IUnknown {
    fn from(value: &Gamepad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Gamepad> for ::windows::core::IInspectable {
    fn from(value: Gamepad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Gamepad> for ::windows::core::IInspectable {
    fn from(value: &Gamepad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Gamepad> for &::windows::core::IInspectable {
    fn from(value: &Gamepad) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&Gamepad> for ::windows::core::InParam<'a, IGameController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Gamepad) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&Gamepad> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Gamepad) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Gamepad {}
unsafe impl ::core::marker::Sync for Gamepad {}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GamepadButtons(pub u32);
impl GamepadButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const A: Self = Self(4u32);
    pub const B: Self = Self(8u32);
    pub const X: Self = Self(16u32);
    pub const Y: Self = Self(32u32);
    pub const DPadUp: Self = Self(64u32);
    pub const DPadDown: Self = Self(128u32);
    pub const DPadLeft: Self = Self(256u32);
    pub const DPadRight: Self = Self(512u32);
    pub const LeftShoulder: Self = Self(1024u32);
    pub const RightShoulder: Self = Self(2048u32);
    pub const LeftThumbstick: Self = Self(4096u32);
    pub const RightThumbstick: Self = Self(8192u32);
    pub const Paddle1: Self = Self(16384u32);
    pub const Paddle2: Self = Self(32768u32);
    pub const Paddle3: Self = Self(65536u32);
    pub const Paddle4: Self = Self(131072u32);
}
impl ::core::marker::Copy for GamepadButtons {}
impl ::core::clone::Clone for GamepadButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GamepadButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GamepadButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for GamepadButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GamepadButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GamepadButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GamepadButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GamepadButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GamepadButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GamepadButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for GamepadButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GamepadButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
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
impl ::core::marker::Copy for GamepadReading {}
impl ::core::clone::Clone for GamepadReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamepadReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamepadReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("LeftTrigger", &self.LeftTrigger).field("RightTrigger", &self.RightTrigger).field("LeftThumbstickX", &self.LeftThumbstickX).field("LeftThumbstickY", &self.LeftThumbstickY).field("RightThumbstickX", &self.RightThumbstickX).field("RightThumbstickY", &self.RightThumbstickY).finish()
    }
}
unsafe impl ::windows::core::Abi for GamepadReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GamepadReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadReading;u8;enum(Windows.Gaming.Input.GamepadButtons;u4);f8;f8;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GamepadReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamepadReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamepadReading {}
impl ::core::default::Default for GamepadReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct GamepadVibration {
    pub LeftMotor: f64,
    pub RightMotor: f64,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
}
impl ::core::marker::Copy for GamepadVibration {}
impl ::core::clone::Clone for GamepadVibration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GamepadVibration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamepadVibration").field("LeftMotor", &self.LeftMotor).field("RightMotor", &self.RightMotor).field("LeftTrigger", &self.LeftTrigger).field("RightTrigger", &self.RightTrigger).finish()
    }
}
unsafe impl ::windows::core::Abi for GamepadVibration {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GamepadVibration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadVibration;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GamepadVibration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GamepadVibration>()) == 0 }
    }
}
impl ::core::cmp::Eq for GamepadVibration {}
impl ::core::default::Default for GamepadVibration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct Headset(::windows::core::IUnknown);
impl Headset {
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn CaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CaptureDeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RenderDeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for Headset {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Headset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Headset {}
impl ::core::fmt::Debug for Headset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Headset").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Headset {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Headset;{3fd156ef-6925-3fa8-9181-029c5223ae3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Headset {
    type Vtable = IHeadset_Vtbl;
    const IID: ::windows::core::GUID = <IHeadset as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Headset {
    const NAME: &'static str = "Windows.Gaming.Input.Headset";
}
impl ::core::convert::From<Headset> for ::windows::core::IUnknown {
    fn from(value: Headset) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Headset> for ::windows::core::IUnknown {
    fn from(value: &Headset) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Headset> for &::windows::core::IUnknown {
    fn from(value: &Headset) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Headset> for ::windows::core::IInspectable {
    fn from(value: Headset) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Headset> for ::windows::core::IInspectable {
    fn from(value: &Headset) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Headset> for &::windows::core::IInspectable {
    fn from(value: &Headset) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&Headset> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Headset) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Headset {}
unsafe impl ::core::marker::Sync for Headset {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcadeStick(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IArcadeStick {
    type Vtable = IArcadeStick_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14a539d_befb_4c81_8051_15ecf3b13036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStick_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ArcadeStickReading) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcadeStickStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IArcadeStickStatics {
    type Vtable = IArcadeStickStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c37b8c8_37b1_4ad8_9458_200f1a30018e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ArcadeStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArcadeStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveArcadeStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveArcadeStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub ArcadeStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArcadeStickRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveArcadeStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveArcadeStickRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ArcadeSticks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ArcadeSticks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IArcadeStickStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IArcadeStickStatics2 {
    type Vtable = IArcadeStickStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52b5d744_bb86_445a_b59c_596f0e2a49df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcadeStickStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlightStick(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFlightStick {
    type Vtable = IFlightStick_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4a2c01c_b83b_4459_a1a9_97b03c33da7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStick_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HatSwitchKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FlightStickReading) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlightStickStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFlightStickStatics {
    type Vtable = IFlightStickStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5514924a_fecc_435e_83dc_5cec8a18a520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlightStickStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FlightStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlightStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFlightStickAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFlightStickAdded: usize,
    #[cfg(feature = "Foundation")]
    pub FlightStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlightStickRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFlightStickRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFlightStickRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FlightSticks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FlightSticks: usize,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct IGameController(::windows::core::IUnknown);
impl IGameController {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
}
impl ::core::convert::From<IGameController> for ::windows::core::IUnknown {
    fn from(value: IGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameController> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameController> for ::windows::core::IUnknown {
    fn from(value: &IGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IGameController> for ::windows::core::IInspectable {
    fn from(value: IGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameController> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameController> for ::windows::core::IInspectable {
    fn from(value: &IGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameController {}
impl ::core::fmt::Debug for IGameController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGameController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1baf6522-5f64-42c5-8267-b9fe2215bfbd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGameController {
    type Vtable = IGameController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1baf6522_5f64_42c5_8267_b9fe2215bfbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub HeadsetConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadsetConnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeadsetConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeadsetConnected: usize,
    #[cfg(feature = "Foundation")]
    pub HeadsetDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadsetDisconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeadsetDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeadsetDisconnected: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub UserChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    UserChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserChanged: usize,
    pub Headset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsWireless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct IGameControllerBatteryInfo(::windows::core::IUnknown);
impl IGameControllerBatteryInfo {
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
impl ::core::convert::From<IGameControllerBatteryInfo> for ::windows::core::IUnknown {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameControllerBatteryInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGameControllerBatteryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerBatteryInfo> for ::windows::core::IUnknown {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IGameControllerBatteryInfo> for ::windows::core::IInspectable {
    fn from(value: IGameControllerBatteryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGameControllerBatteryInfo> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IGameControllerBatteryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerBatteryInfo> for ::windows::core::IInspectable {
    fn from(value: &IGameControllerBatteryInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGameControllerBatteryInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameControllerBatteryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerBatteryInfo {}
impl ::core::fmt::Debug for IGameControllerBatteryInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerBatteryInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGameControllerBatteryInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dcecc681-3963-4da6-955d-553f3b6f6161}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGameControllerBatteryInfo {
    type Vtable = IGameControllerBatteryInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcecc681_3963_4da6_955d_553f3b6f6161);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerBatteryInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepad(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGamepad {
    type Vtable = IGamepad_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc7bb43c_0a69_3903_9e9d_a50f86a45de5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Vibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GamepadVibration) -> ::windows::core::HRESULT,
    pub SetVibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GamepadVibration) -> ::windows::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GamepadReading) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepad2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGamepad2 {
    type Vtable = IGamepad2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c1689bd_5915_4245_b0c0_c89fae0308ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepad2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepadStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGamepadStatics {
    type Vtable = IGamepadStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bbce529_d49c_39e9_9560_e47dde96b7c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GamepadAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GamepadAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGamepadAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGamepadAdded: usize,
    #[cfg(feature = "Foundation")]
    pub GamepadRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GamepadRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGamepadRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGamepadRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gamepads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gamepads: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGamepadStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGamepadStatics2 {
    type Vtable = IGamepadStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42676dc5_0856_47c4_9213_b395504c3a3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamepadStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHeadset(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHeadset {
    type Vtable = IHeadset_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fd156ef_6925_3fa8_9181_029c5223ae3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeadset_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CaptureDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RenderDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRacingWheel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRacingWheel {
    type Vtable = IRacingWheel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf546656f_e106_4c82_a90f_554012904b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheel_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HasClutch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasHandbrake: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasPatternShifter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxPatternShifterGear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxWheelAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    pub WheelMotor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_ForceFeedback"))]
    WheelMotor: usize,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RacingWheelReading) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRacingWheelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRacingWheelStatics {
    type Vtable = IRacingWheelStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ac12cd5_581b_4936_9f94_69f1e6514c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RacingWheelAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RacingWheelAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRacingWheelAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRacingWheelAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RacingWheelRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RacingWheelRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRacingWheelRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRacingWheelRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RacingWheels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RacingWheels: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRacingWheelStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRacingWheelStatics2 {
    type Vtable = IRacingWheelStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe666bcaa_edfd_4323_a9f6_3c384048d1ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRacingWheelStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawGameController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRawGameController {
    type Vtable = IRawGameController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cad6d91_a7e1_4f71_9a78_33e9c5dfea62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AxisCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ButtonCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    pub ForceFeedbackMotors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback")))]
    ForceFeedbackMotors: usize,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SwitchCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows::core::HRESULT,
    pub GetSwitchKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawGameController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRawGameController2 {
    type Vtable = IRawGameController2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c0c035_bb73_4756_a787_3ed6bea617bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameController2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    pub SimpleHapticsControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Haptics", feature = "Foundation_Collections")))]
    SimpleHapticsControllers: usize,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawGameControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRawGameControllerStatics {
    type Vtable = IRawGameControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb8d0792_e95a_4b19_afc7_0a59f8bf759e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawGameControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RawGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawGameControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawGameControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RawGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawGameControllerRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawGameControllerRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RawGameControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RawGameControllers: usize,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUINavigationController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUINavigationController {
    type Vtable = IUINavigationController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5aeefdd_f50e_4a55_8cdc_d33229548175);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UINavigationReading) -> ::windows::core::HRESULT,
    pub GetOptionalButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
    pub GetRequiredButtonLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUINavigationControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUINavigationControllerStatics {
    type Vtable = IUINavigationControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f14930a_f6f8_4a48_8d89_94786cca0c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub UINavigationControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UINavigationControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUINavigationControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUINavigationControllerAdded: usize,
    #[cfg(feature = "Foundation")]
    pub UINavigationControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UINavigationControllerRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUINavigationControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUINavigationControllerRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UINavigationControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UINavigationControllers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUINavigationControllerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUINavigationControllerStatics2 {
    type Vtable = IUINavigationControllerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0cb28e3_b20b_4b0b_9ed4_f3d53cec0de4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUINavigationControllerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OptionalUINavigationButtons(pub u32);
impl OptionalUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Context1: Self = Self(1u32);
    pub const Context2: Self = Self(2u32);
    pub const Context3: Self = Self(4u32);
    pub const Context4: Self = Self(8u32);
    pub const PageUp: Self = Self(16u32);
    pub const PageDown: Self = Self(32u32);
    pub const PageLeft: Self = Self(64u32);
    pub const PageRight: Self = Self(128u32);
    pub const ScrollUp: Self = Self(256u32);
    pub const ScrollDown: Self = Self(512u32);
    pub const ScrollLeft: Self = Self(1024u32);
    pub const ScrollRight: Self = Self(2048u32);
}
impl ::core::marker::Copy for OptionalUINavigationButtons {}
impl ::core::clone::Clone for OptionalUINavigationButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OptionalUINavigationButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OptionalUINavigationButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for OptionalUINavigationButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OptionalUINavigationButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OptionalUINavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OptionalUINavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OptionalUINavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OptionalUINavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OptionalUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for OptionalUINavigationButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct RacingWheel(::windows::core::IUnknown);
impl RacingWheel {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn HasClutch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasClutch)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasHandbrake(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasHandbrake)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasPatternShifter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasPatternShifter)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPatternShifterGear(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxPatternShifterGear)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxWheelAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxWheelAngle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Gaming_Input_ForceFeedback\"`*"]
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    pub fn WheelMotor(&self) -> ::windows::core::Result<ForceFeedback::ForceFeedbackMotor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WheelMotor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedback::ForceFeedbackMotor>(result__)
        }
    }
    pub fn GetButtonLabel(&self, button: RacingWheelButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetButtonLabel)(::windows::core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<RacingWheelReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RacingWheelReading>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RacingWheelAdded<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<RacingWheel>>>,
    {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RacingWheelAdded)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRacingWheelAdded(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRacingWheelAdded)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RacingWheelRemoved<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<RacingWheel>>>,
    {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RacingWheelRemoved)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRacingWheelRemoved(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRacingWheelRemoved)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RacingWheels() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RacingWheels)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<RacingWheel>>(result__)
        })
    }
    pub fn FromGameController<'a, P0, E0>(gamecontroller: P0) -> ::windows::core::Result<RacingWheel>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRacingWheelStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromGameController)(::windows::core::Interface::as_raw(this), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<RacingWheel>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRacingWheelStatics<R, F: FnOnce(&IRacingWheelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RacingWheel, IRacingWheelStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRacingWheelStatics2<R, F: FnOnce(&IRacingWheelStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RacingWheel, IRacingWheelStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RacingWheel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RacingWheel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RacingWheel {}
impl ::core::fmt::Debug for RacingWheel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RacingWheel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RacingWheel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RacingWheel;{f546656f-e106-4c82-a90f-554012904b85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RacingWheel {
    type Vtable = IRacingWheel_Vtbl;
    const IID: ::windows::core::GUID = <IRacingWheel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.RacingWheel";
}
impl ::core::convert::From<RacingWheel> for ::windows::core::IUnknown {
    fn from(value: RacingWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RacingWheel> for ::windows::core::IUnknown {
    fn from(value: &RacingWheel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RacingWheel> for &::windows::core::IUnknown {
    fn from(value: &RacingWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RacingWheel> for ::windows::core::IInspectable {
    fn from(value: RacingWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RacingWheel> for ::windows::core::IInspectable {
    fn from(value: &RacingWheel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RacingWheel> for &::windows::core::IInspectable {
    fn from(value: &RacingWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&RacingWheel> for ::windows::core::InParam<'a, IGameController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RacingWheel) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&RacingWheel> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RacingWheel) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RacingWheel {}
unsafe impl ::core::marker::Sync for RacingWheel {}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RacingWheelButtons(pub u32);
impl RacingWheelButtons {
    pub const None: Self = Self(0u32);
    pub const PreviousGear: Self = Self(1u32);
    pub const NextGear: Self = Self(2u32);
    pub const DPadUp: Self = Self(4u32);
    pub const DPadDown: Self = Self(8u32);
    pub const DPadLeft: Self = Self(16u32);
    pub const DPadRight: Self = Self(32u32);
    pub const Button1: Self = Self(64u32);
    pub const Button2: Self = Self(128u32);
    pub const Button3: Self = Self(256u32);
    pub const Button4: Self = Self(512u32);
    pub const Button5: Self = Self(1024u32);
    pub const Button6: Self = Self(2048u32);
    pub const Button7: Self = Self(4096u32);
    pub const Button8: Self = Self(8192u32);
    pub const Button9: Self = Self(16384u32);
    pub const Button10: Self = Self(32768u32);
    pub const Button11: Self = Self(65536u32);
    pub const Button12: Self = Self(131072u32);
    pub const Button13: Self = Self(262144u32);
    pub const Button14: Self = Self(524288u32);
    pub const Button15: Self = Self(1048576u32);
    pub const Button16: Self = Self(2097152u32);
}
impl ::core::marker::Copy for RacingWheelButtons {}
impl ::core::clone::Clone for RacingWheelButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RacingWheelButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RacingWheelButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for RacingWheelButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RacingWheelButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RacingWheelButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RacingWheelButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RacingWheelButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RacingWheelButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RacingWheelButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for RacingWheelButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RacingWheelButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
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
impl ::core::marker::Copy for RacingWheelReading {}
impl ::core::clone::Clone for RacingWheelReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RacingWheelReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RacingWheelReading").field("Timestamp", &self.Timestamp).field("Buttons", &self.Buttons).field("PatternShifterGear", &self.PatternShifterGear).field("Wheel", &self.Wheel).field("Throttle", &self.Throttle).field("Brake", &self.Brake).field("Clutch", &self.Clutch).field("Handbrake", &self.Handbrake).finish()
    }
}
unsafe impl ::windows::core::Abi for RacingWheelReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RacingWheelReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.RacingWheelReading;u8;enum(Windows.Gaming.Input.RacingWheelButtons;u4);i4;f8;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for RacingWheelReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RacingWheelReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for RacingWheelReading {}
impl ::core::default::Default for RacingWheelReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct RawGameController(::windows::core::IUnknown);
impl RawGameController {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn AxisCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AxisCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ButtonCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ButtonCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Gaming_Input_ForceFeedback\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    pub fn ForceFeedbackMotors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ForceFeedbackMotors)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>(result__)
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
    pub fn SwitchCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SwitchCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GetButtonLabel(&self, buttonindex: i32) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetButtonLabel)(::windows::core::Interface::as_raw(this), buttonindex, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetCurrentReading(&self, buttonarray: &mut [bool], switcharray: &mut [GameControllerSwitchPosition], axisarray: &mut [f64]) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), buttonarray.len() as u32, buttonarray.as_mut_ptr(), switcharray.len() as u32, switcharray.as_mut_ptr(), axisarray.len() as u32, axisarray.as_mut_ptr(), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn GetSwitchKind(&self, switchindex: i32) -> ::windows::core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetSwitchKind)(::windows::core::Interface::as_raw(this), switchindex, result__.as_mut_ptr()).from_abi::<GameControllerSwitchKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Haptics\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    pub fn SimpleHapticsControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>> {
        let this = &::windows::core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SimpleHapticsControllers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>>(result__)
        }
    }
    pub fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NonRoamableId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RawGameControllerAdded<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<RawGameController>>>,
    {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RawGameControllerAdded)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawGameControllerAdded(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRawGameControllerAdded)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RawGameControllerRemoved<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<RawGameController>>>,
    {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RawGameControllerRemoved)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawGameControllerRemoved(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRawGameControllerRemoved)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RawGameControllers() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RawGameController>> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RawGameControllers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<RawGameController>>(result__)
        })
    }
    pub fn FromGameController<'a, P0, E0>(gamecontroller: P0) -> ::windows::core::Result<RawGameController>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromGameController)(::windows::core::Interface::as_raw(this), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<RawGameController>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRawGameControllerStatics<R, F: FnOnce(&IRawGameControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RawGameController, IRawGameControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RawGameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RawGameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RawGameController {}
impl ::core::fmt::Debug for RawGameController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RawGameController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RawGameController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.RawGameController;{7cad6d91-a7e1-4f71-9a78-33e9c5dfea62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RawGameController {
    type Vtable = IRawGameController_Vtbl;
    const IID: ::windows::core::GUID = <IRawGameController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.RawGameController";
}
impl ::core::convert::From<RawGameController> for ::windows::core::IUnknown {
    fn from(value: RawGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawGameController> for ::windows::core::IUnknown {
    fn from(value: &RawGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RawGameController> for &::windows::core::IUnknown {
    fn from(value: &RawGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RawGameController> for ::windows::core::IInspectable {
    fn from(value: RawGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RawGameController> for ::windows::core::IInspectable {
    fn from(value: &RawGameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RawGameController> for &::windows::core::IInspectable {
    fn from(value: &RawGameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&RawGameController> for ::windows::core::InParam<'a, IGameController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RawGameController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&RawGameController> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RawGameController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RawGameController {}
unsafe impl ::core::marker::Sync for RawGameController {}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RequiredUINavigationButtons(pub u32);
impl RequiredUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const Accept: Self = Self(4u32);
    pub const Cancel: Self = Self(8u32);
    pub const Up: Self = Self(16u32);
    pub const Down: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const Right: Self = Self(128u32);
}
impl ::core::marker::Copy for RequiredUINavigationButtons {}
impl ::core::clone::Clone for RequiredUINavigationButtons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RequiredUINavigationButtons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RequiredUINavigationButtons {
    type Abi = Self;
}
impl ::core::fmt::Debug for RequiredUINavigationButtons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RequiredUINavigationButtons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RequiredUINavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RequiredUINavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RequiredUINavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RequiredUINavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RequiredUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for RequiredUINavigationButtons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Input\"`*"]
#[repr(transparent)]
pub struct UINavigationController(::windows::core::IUnknown);
impl UINavigationController {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetConnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetConnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetConnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadsetDisconnected<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, Headset>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HeadsetDisconnected)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeadsetDisconnected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UserChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> ::windows::core::Result<Headset> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Headset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headset>(result__)
        }
    }
    pub fn IsWireless(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsWireless)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Power\"`*"]
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &::windows::core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetBatteryReport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Power::BatteryReport>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows::core::Result<UINavigationReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentReading)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UINavigationReading>(result__)
        }
    }
    pub fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetOptionalButtonLabel)(::windows::core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    pub fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetRequiredButtonLabel)(::windows::core::Interface::as_raw(this), button, result__.as_mut_ptr()).from_abi::<GameControllerButtonLabel>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UINavigationControllerAdded<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<UINavigationController>>>,
    {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UINavigationControllerAdded)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUINavigationControllerAdded(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveUINavigationControllerAdded)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UINavigationControllerRemoved<'a, P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<UINavigationController>>>,
    {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UINavigationControllerRemoved)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUINavigationControllerRemoved(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveUINavigationControllerRemoved)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UINavigationControllers() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UINavigationControllers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<UINavigationController>>(result__)
        })
    }
    pub fn FromGameController<'a, P0, E0>(gamecontroller: P0) -> ::windows::core::Result<UINavigationController>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IGameController>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IUINavigationControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromGameController)(::windows::core::Interface::as_raw(this), gamecontroller.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<UINavigationController>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUINavigationControllerStatics<R, F: FnOnce(&IUINavigationControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UINavigationController, IUINavigationControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUINavigationControllerStatics2<R, F: FnOnce(&IUINavigationControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UINavigationController, IUINavigationControllerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UINavigationController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UINavigationController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UINavigationController {}
impl ::core::fmt::Debug for UINavigationController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UINavigationController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UINavigationController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.UINavigationController;{e5aeefdd-f50e-4a55-8cdc-d33229548175})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UINavigationController {
    type Vtable = IUINavigationController_Vtbl;
    const IID: ::windows::core::GUID = <IUINavigationController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.UINavigationController";
}
impl ::core::convert::From<UINavigationController> for ::windows::core::IUnknown {
    fn from(value: UINavigationController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UINavigationController> for ::windows::core::IUnknown {
    fn from(value: &UINavigationController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UINavigationController> for &::windows::core::IUnknown {
    fn from(value: &UINavigationController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<UINavigationController> for ::windows::core::IInspectable {
    fn from(value: UINavigationController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UINavigationController> for ::windows::core::IInspectable {
    fn from(value: &UINavigationController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UINavigationController> for &::windows::core::IInspectable {
    fn from(value: &UINavigationController) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&UINavigationController> for ::windows::core::InParam<'a, IGameController> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UINavigationController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl<'a> ::core::convert::TryFrom<&UINavigationController> for ::windows::core::InParam<'a, IGameControllerBatteryInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &UINavigationController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for UINavigationController {}
unsafe impl ::core::marker::Sync for UINavigationController {}
#[repr(C)]
#[doc = "*Required features: `\"Gaming_Input\"`*"]
pub struct UINavigationReading {
    pub Timestamp: u64,
    pub RequiredButtons: RequiredUINavigationButtons,
    pub OptionalButtons: OptionalUINavigationButtons,
}
impl ::core::marker::Copy for UINavigationReading {}
impl ::core::clone::Clone for UINavigationReading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UINavigationReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UINavigationReading").field("Timestamp", &self.Timestamp).field("RequiredButtons", &self.RequiredButtons).field("OptionalButtons", &self.OptionalButtons).finish()
    }
}
unsafe impl ::windows::core::Abi for UINavigationReading {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UINavigationReading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.UINavigationReading;u8;enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4);enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for UINavigationReading {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UINavigationReading>()) == 0 }
    }
}
impl ::core::cmp::Eq for UINavigationReading {}
impl ::core::default::Default for UINavigationReading {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
