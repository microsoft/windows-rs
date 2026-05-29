/// UI Automation properties applied to every widget kind via
/// [`Modifiers::accessibility`](super::Modifiers::accessibility).
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct AccessibilityModifiers {
    pub automation_name: Option<String>,
    pub automation_id: Option<String>,
    pub help_text: Option<String>,
    pub live_setting: Option<LiveSetting>,
    pub heading_level: Option<HeadingLevel>,
}

impl AccessibilityModifiers {
    pub fn is_empty(&self) -> bool {
        self.automation_name.is_none()
            && self.automation_id.is_none()
            && self.help_text.is_none()
            && self.live_setting.is_none()
            && self.heading_level.is_none()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LiveSetting {
    Off,
    Polite,
    Assertive,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}
