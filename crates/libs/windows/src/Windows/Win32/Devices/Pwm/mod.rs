#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const GUID_DEVINTERFACE_PWM_CONTROLLER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60824b4c_eed1_4c9c_b49c_1b961461a819);
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const GUID_DEVINTERFACE_PWM_CONTROLLER_WSZ: ::windows::core::PCWSTR = ::windows::w!("{60824B4C-EED1-4C9C-B49C-1B961461A819}");
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_CONTROLLER_GET_ACTUAL_PERIOD: u32 = 262148u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_CONTROLLER_GET_INFO: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_CONTROLLER_SET_DESIRED_PERIOD: u32 = 294920u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 262544u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_GET_POLARITY: u32 = 262552u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_IS_STARTED: u32 = 262568u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: u32 = 295316u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_SET_POLARITY: u32 = 295324u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_START: u32 = 295331u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const IOCTL_PWM_PIN_STOP: u32 = 295335u32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_CONTROLLER_GET_ACTUAL_PERIOD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_CONTROLLER_GET_INFO: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_CONTROLLER_SET_DESIRED_PERIOD: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_GET_POLARITY: i32 = 102i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_IS_STARTED: i32 = 106i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE: i32 = 101i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_SET_POLARITY: i32 = 103i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_START: i32 = 104i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_IOCTL_ID_PIN_STOP: i32 = 105i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PWM_POLARITY(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_ACTIVE_HIGH: PWM_POLARITY = PWM_POLARITY(0i32);
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_ACTIVE_LOW: PWM_POLARITY = PWM_POLARITY(1i32);
impl ::core::marker::Copy for PWM_POLARITY {}
impl ::core::clone::Clone for PWM_POLARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PWM_POLARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PWM_POLARITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PWM_POLARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PWM_POLARITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
impl ::core::marker::Copy for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
impl ::core::clone::Clone for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT").field("ActualPeriod", &self.ActualPeriod).finish()
    }
}
impl ::windows::core::TypeKind for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
impl ::core::default::Default for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_CONTROLLER_INFO {
    pub Size: usize,
    pub PinCount: u32,
    pub MinimumPeriod: u64,
    pub MaximumPeriod: u64,
}
impl ::core::marker::Copy for PWM_CONTROLLER_INFO {}
impl ::core::clone::Clone for PWM_CONTROLLER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_INFO").field("Size", &self.Size).field("PinCount", &self.PinCount).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumPeriod", &self.MaximumPeriod).finish()
    }
}
impl ::windows::core::TypeKind for PWM_CONTROLLER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PinCount == other.PinCount && self.MinimumPeriod == other.MinimumPeriod && self.MaximumPeriod == other.MaximumPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_INFO {}
impl ::core::default::Default for PWM_CONTROLLER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    pub DesiredPeriod: u64,
}
impl ::core::marker::Copy for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
impl ::core::clone::Clone for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT").field("DesiredPeriod", &self.DesiredPeriod).finish()
    }
}
impl ::windows::core::TypeKind for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DesiredPeriod == other.DesiredPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
impl ::core::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    pub ActualPeriod: u64,
}
impl ::core::marker::Copy for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
impl ::core::clone::Clone for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT").field("ActualPeriod", &self.ActualPeriod).finish()
    }
}
impl ::windows::core::TypeKind for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
impl ::core::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    pub Percentage: u64,
}
impl ::core::marker::Copy for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
impl ::core::clone::Clone for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT").field("Percentage", &self.Percentage).finish()
    }
}
impl ::windows::core::TypeKind for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
impl ::core::default::Default for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_PIN_GET_POLARITY_OUTPUT {
    pub Polarity: PWM_POLARITY,
}
impl ::core::marker::Copy for PWM_PIN_GET_POLARITY_OUTPUT {}
impl ::core::clone::Clone for PWM_PIN_GET_POLARITY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_PIN_GET_POLARITY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_GET_POLARITY_OUTPUT").field("Polarity", &self.Polarity).finish()
    }
}
impl ::windows::core::TypeKind for PWM_PIN_GET_POLARITY_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_PIN_GET_POLARITY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::core::cmp::Eq for PWM_PIN_GET_POLARITY_OUTPUT {}
impl ::core::default::Default for PWM_PIN_GET_POLARITY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PWM_PIN_IS_STARTED_OUTPUT {
    pub IsStarted: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PWM_PIN_IS_STARTED_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PWM_PIN_IS_STARTED_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PWM_PIN_IS_STARTED_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_IS_STARTED_OUTPUT").field("IsStarted", &self.IsStarted).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PWM_PIN_IS_STARTED_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for PWM_PIN_IS_STARTED_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    pub Percentage: u64,
}
impl ::core::marker::Copy for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
impl ::core::clone::Clone for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT").field("Percentage", &self.Percentage).finish()
    }
}
impl ::windows::core::TypeKind for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
impl ::core::default::Default for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub struct PWM_PIN_SET_POLARITY_INPUT {
    pub Polarity: PWM_POLARITY,
}
impl ::core::marker::Copy for PWM_PIN_SET_POLARITY_INPUT {}
impl ::core::clone::Clone for PWM_PIN_SET_POLARITY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PWM_PIN_SET_POLARITY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_SET_POLARITY_INPUT").field("Polarity", &self.Polarity).finish()
    }
}
impl ::windows::core::TypeKind for PWM_PIN_SET_POLARITY_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PWM_PIN_SET_POLARITY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::core::cmp::Eq for PWM_PIN_SET_POLARITY_INPUT {}
impl ::core::default::Default for PWM_PIN_SET_POLARITY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
