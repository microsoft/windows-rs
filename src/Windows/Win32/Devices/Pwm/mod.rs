#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const GUID_DEVINTERFACE_PWM_CONTROLLER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1619151692,
        61137,
        19612,
        [180, 156, 27, 150, 20, 97, 168, 25],
    );
pub const IOCTL_PWM_CONTROLLER_GET_ACTUAL_PERIOD: u32 = 262148u32;
pub const IOCTL_PWM_CONTROLLER_GET_INFO: u32 = 262144u32;
pub const IOCTL_PWM_CONTROLLER_SET_DESIRED_PERIOD: u32 = 294920u32;
pub const IOCTL_PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 262544u32;
pub const IOCTL_PWM_PIN_GET_POLARITY: u32 = 262552u32;
pub const IOCTL_PWM_PIN_IS_STARTED: u32 = 262568u32;
pub const IOCTL_PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 295316u32;
pub const IOCTL_PWM_PIN_SET_POLARITY: u32 = 295324u32;
pub const IOCTL_PWM_PIN_START: u32 = 295331u32;
pub const IOCTL_PWM_PIN_STOP: u32 = 295335u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
impl PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
impl ::std::default::Default for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT")
            .field("ActualPeriod", &self.ActualPeriod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::std::cmp::Eq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
unsafe impl ::windows::runtime::Abi for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_CONTROLLER_INFO {
    pub Size: usize,
    pub PinCount: u32,
    pub MinimumPeriod: u64,
    pub MaximumPeriod: u64,
}
impl PWM_CONTROLLER_INFO {}
impl ::std::default::Default for PWM_CONTROLLER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_CONTROLLER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_INFO")
            .field("Size", &self.Size)
            .field("PinCount", &self.PinCount)
            .field("MinimumPeriod", &self.MinimumPeriod)
            .field("MaximumPeriod", &self.MaximumPeriod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_CONTROLLER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.PinCount == other.PinCount
            && self.MinimumPeriod == other.MinimumPeriod
            && self.MaximumPeriod == other.MaximumPeriod
    }
}
impl ::std::cmp::Eq for PWM_CONTROLLER_INFO {}
unsafe impl ::windows::runtime::Abi for PWM_CONTROLLER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    pub DesiredPeriod: u64,
}
impl PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
impl ::std::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT")
            .field("DesiredPeriod", &self.DesiredPeriod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DesiredPeriod == other.DesiredPeriod
    }
}
impl ::std::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
unsafe impl ::windows::runtime::Abi for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
impl PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
impl ::std::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT")
            .field("ActualPeriod", &self.ActualPeriod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::std::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
unsafe impl ::windows::runtime::Abi for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PWM_IOCTL_ID_CONTROLLER_GET_ACTUAL_PERIOD: i32 = 1i32;
pub const PWM_IOCTL_ID_CONTROLLER_GET_INFO: i32 = 0i32;
pub const PWM_IOCTL_ID_CONTROLLER_SET_DESIRED_PERIOD: i32 = 2i32;
pub const PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 100i32;
pub const PWM_IOCTL_ID_PIN_GET_POLARITY: i32 = 102i32;
pub const PWM_IOCTL_ID_PIN_IS_STARTED: i32 = 106i32;
pub const PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 101i32;
pub const PWM_IOCTL_ID_PIN_SET_POLARITY: i32 = 103i32;
pub const PWM_IOCTL_ID_PIN_START: i32 = 104i32;
pub const PWM_IOCTL_ID_PIN_STOP: i32 = 105i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    pub Percentage: u64,
}
impl PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
impl ::std::default::Default for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT")
            .field("Percentage", &self.Percentage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::std::cmp::Eq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
unsafe impl ::windows::runtime::Abi for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_PIN_GET_POLARITY_OUTPUT {
    pub Polarity: PWM_POLARITY,
}
impl PWM_PIN_GET_POLARITY_OUTPUT {}
impl ::std::default::Default for PWM_PIN_GET_POLARITY_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_PIN_GET_POLARITY_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_PIN_GET_POLARITY_OUTPUT")
            .field("Polarity", &self.Polarity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_PIN_GET_POLARITY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::std::cmp::Eq for PWM_PIN_GET_POLARITY_OUTPUT {}
unsafe impl ::windows::runtime::Abi for PWM_PIN_GET_POLARITY_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PWM_PIN_IS_STARTED_OUTPUT {
    pub IsStarted: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl PWM_PIN_IS_STARTED_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PWM_PIN_IS_STARTED_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PWM_PIN_IS_STARTED_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_PIN_IS_STARTED_OUTPUT")
            .field("IsStarted", &self.IsStarted)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PWM_PIN_IS_STARTED_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.IsStarted == other.IsStarted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PWM_PIN_IS_STARTED_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PWM_PIN_IS_STARTED_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    pub Percentage: u64,
}
impl PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
impl ::std::default::Default for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT")
            .field("Percentage", &self.Percentage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::std::cmp::Eq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
unsafe impl ::windows::runtime::Abi for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PWM_PIN_SET_POLARITY_INPUT {
    pub Polarity: PWM_POLARITY,
}
impl PWM_PIN_SET_POLARITY_INPUT {}
impl ::std::default::Default for PWM_PIN_SET_POLARITY_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PWM_PIN_SET_POLARITY_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PWM_PIN_SET_POLARITY_INPUT")
            .field("Polarity", &self.Polarity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PWM_PIN_SET_POLARITY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::std::cmp::Eq for PWM_PIN_SET_POLARITY_INPUT {}
unsafe impl ::windows::runtime::Abi for PWM_PIN_SET_POLARITY_INPUT {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct PWM_POLARITY(pub i32);
pub const PWM_ACTIVE_HIGH: PWM_POLARITY = PWM_POLARITY(0i32);
pub const PWM_ACTIVE_LOW: PWM_POLARITY = PWM_POLARITY(1i32);
impl ::std::convert::From<i32> for PWM_POLARITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PWM_POLARITY {
    type Abi = Self;
    type DefaultType = Self;
}
