impl ::core::default::Default for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {}
impl ::core::fmt::Debug for PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT").field("ActualPeriod", &self.ActualPeriod).finish()
    }
}
impl ::core::default::Default for PWM_CONTROLLER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PinCount == other.PinCount && self.MinimumPeriod == other.MinimumPeriod && self.MaximumPeriod == other.MaximumPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_INFO {}
impl ::core::fmt::Debug for PWM_CONTROLLER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_INFO").field("Size", &self.Size).field("PinCount", &self.PinCount).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumPeriod", &self.MaximumPeriod).finish()
    }
}
impl ::core::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DesiredPeriod == other.DesiredPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {}
impl ::core::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT").field("DesiredPeriod", &self.DesiredPeriod).finish()
    }
}
impl ::core::default::Default for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ActualPeriod == other.ActualPeriod
    }
}
impl ::core::cmp::Eq for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {}
impl ::core::fmt::Debug for PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT").field("ActualPeriod", &self.ActualPeriod).finish()
    }
}
impl ::core::default::Default for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {}
impl ::core::fmt::Debug for PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT").field("Percentage", &self.Percentage).finish()
    }
}
impl ::core::default::Default for PWM_PIN_GET_POLARITY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_GET_POLARITY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::core::cmp::Eq for PWM_PIN_GET_POLARITY_OUTPUT {}
impl ::core::fmt::Debug for PWM_PIN_GET_POLARITY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_GET_POLARITY_OUTPUT").field("Polarity", &self.Polarity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PWM_PIN_IS_STARTED_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PWM_PIN_IS_STARTED_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_IS_STARTED_OUTPUT").field("IsStarted", &self.IsStarted).finish()
    }
}
impl ::core::default::Default for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {}
impl ::core::fmt::Debug for PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT").field("Percentage", &self.Percentage).finish()
    }
}
impl ::core::default::Default for PWM_PIN_SET_POLARITY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PWM_PIN_SET_POLARITY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Polarity == other.Polarity
    }
}
impl ::core::cmp::Eq for PWM_PIN_SET_POLARITY_INPUT {}
impl ::core::fmt::Debug for PWM_PIN_SET_POLARITY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PWM_PIN_SET_POLARITY_INPUT").field("Polarity", &self.Polarity).finish()
    }
}
impl ::core::default::Default for PWM_POLARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PWM_POLARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PWM_POLARITY").field(&self.0).finish()
    }
}
