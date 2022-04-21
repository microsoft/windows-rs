pub const GUID_DEVINTERFACE_PWM_CONTROLLER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1619151692, data2: 61137, data3: 19612, data4: [180, 156, 27, 150, 20, 97, 168, 25] };
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const GUID_DEVINTERFACE_PWM_CONTROLLER_WSZ: &str = "{60824B4C-EED1-4C9C-B49C-1B961461A819}";
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
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub type PWM_POLARITY = i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_ACTIVE_HIGH: PWM_POLARITY = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Pwm\"`*"]
pub const PWM_ACTIVE_LOW: PWM_POLARITY = 1i32;
