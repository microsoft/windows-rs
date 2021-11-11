#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const GUID_DEVINTERFACE_PWM_CONTROLLER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60824b4c_eed1_4c9c_b49c_1b961461a819);
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_CONTROLLER_GET_ACTUAL_PERIOD: u32 = 262148u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_CONTROLLER_GET_INFO: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_CONTROLLER_SET_DESIRED_PERIOD: u32 = 294920u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 262544u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_GET_POLARITY: u32 = 262552u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_IS_STARTED: u32 = 262568u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 295316u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_SET_POLARITY: u32 = 295324u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_START: u32 = 295331u32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const IOCTL_PWM_PIN_STOP: u32 = 295335u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
impl PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
impl ::core::default::Default for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT").field("ActualPeriod", &self.ActualPeriod).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
unsafe impl ::windows::core::Abi for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_CONTROLLER_INFO {
    pub Size: usize,
    pub PinCount: u32,
    pub MinimumPeriod: u64,
    pub MaximumPeriod: u64,
}
impl PWM_CONTROLLER_INFO {}
impl ::core::default::Default for PWM_CONTROLLER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_INFO").field("Size", &self.Size).field("PinCount", &self.PinCount).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumPeriod", &self.MaximumPeriod).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PinCount == other.PinCount && self.MinimumPeriod == other.MinimumPeriod && self.MaximumPeriod == other.MaximumPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_INFO {}
unsafe impl ::windows::core::Abi for PWM_CONTROLLER_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    pub DesiredPeriod: u64,
}
impl PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
impl ::core::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT").field("DesiredPeriod", &self.DesiredPeriod).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DesiredPeriod == other.DesiredPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
unsafe impl ::windows::core::Abi for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
impl PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
impl ::core::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT").field("ActualPeriod", &self.ActualPeriod).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
unsafe impl ::windows::core::Abi for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_CONTROLLER_GET_ACTUAL_PERIOD: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_CONTROLLER_GET_INFO: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_CONTROLLER_SET_DESIRED_PERIOD: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 100i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_GET_POLARITY: i32 = 102i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_IS_STARTED: i32 = 106i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 101i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_SET_POLARITY: i32 = 103i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_START: i32 = 104i32;
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub const PWM_IOCTL_ID_PIN_STOP: i32 = 105i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    pub Percentage: u64,
}
impl PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
impl ::core::default::Default for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT").field("Percentage", &self.Percentage).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
unsafe impl ::windows::core::Abi for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_PIN_GET_POLARITY_OUTPUT {
    pub Polarity: PWM_POLARITY,
}
impl PWM_PIN_GET_POLARITY_OUTPUT {}
impl ::core::default::Default for PWM_PIN_GET_POLARITY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_PIN_GET_POLARITY_OUTPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_PIN_GET_POLARITY_OUTPUT").field("Polarity", &self.Polarity).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_GET_POLARITY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::core::cmp::Eq for PWM_PIN_GET_POLARITY_OUTPUT {}
unsafe impl ::windows::core::Abi for PWM_PIN_GET_POLARITY_OUTPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Pwm`, `Win32_Foundation`*"]
pub struct PWM_PIN_IS_STARTED_OUTPUT {
    pub IsStarted: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl PWM_PIN_IS_STARTED_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PWM_PIN_IS_STARTED_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PWM_PIN_IS_STARTED_OUTPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_PIN_IS_STARTED_OUTPUT").field("IsStarted", &self.IsStarted).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PWM_PIN_IS_STARTED_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.IsStarted == other.IsStarted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PWM_PIN_IS_STARTED_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PWM_PIN_IS_STARTED_OUTPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    pub Percentage: u64,
}
impl PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
impl ::core::default::Default for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT").field("Percentage", &self.Percentage).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
unsafe impl ::windows::core::Abi for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
pub struct PWM_PIN_SET_POLARITY_INPUT {
    pub Polarity: PWM_POLARITY,
}
impl PWM_PIN_SET_POLARITY_INPUT {}
impl ::core::default::Default for PWM_PIN_SET_POLARITY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PWM_PIN_SET_POLARITY_INPUT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PWM_PIN_SET_POLARITY_INPUT").field("Polarity", &self.Polarity).finish()
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_SET_POLARITY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::core::cmp::Eq for PWM_PIN_SET_POLARITY_INPUT {}
unsafe impl ::windows::core::Abi for PWM_PIN_SET_POLARITY_INPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Pwm`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PWM_POLARITY(pub i32);
pub const PWM_ACTIVE_HIGH: PWM_POLARITY = PWM_POLARITY(0i32);
pub const PWM_ACTIVE_LOW: PWM_POLARITY = PWM_POLARITY(1i32);
impl ::core::convert::From<i32> for PWM_POLARITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PWM_POLARITY {
    type Abi = Self;
}
