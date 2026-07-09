#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CURSOR_ID(pub u32);
pub const IP_CURSOR_DOWN: u32 = 1;
pub const IP_INVERTED: u32 = 2;
pub const IP_MARGIN: u32 = 4;
pub const MAX_PACKET_BUTTON_COUNT: u32 = 32;
pub const MAX_PACKET_PROPERTY_COUNT: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PACKET_DESCRIPTION {
    pub cbPacketSize: u32,
    pub cPacketProperties: u32,
    pub pPacketProperties: *mut PACKET_PROPERTY,
    pub cButtons: u32,
    pub pguidButtons: *mut windows_core::GUID,
}
impl Default for PACKET_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PACKET_PROPERTY {
    pub guid: windows_core::GUID,
    pub PropertyMetrics: PROPERTY_METRICS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPACKET_DESCRIPTION(pub *mut PACKET_DESCRIPTION);
impl PPACKET_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPACKET_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPACKET_PROPERTY(pub *mut PACKET_PROPERTY);
impl PPACKET_PROPERTY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPACKET_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROPERTY_METRICS(pub *mut PROPERTY_METRICS);
impl PPROPERTY_METRICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROPERTY_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROPERTY_UNITS(pub *mut PROPERTY_UNITS);
impl PPROPERTY_UNITS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROPERTY_UNITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPERTY_METRICS {
    pub nLogicalMin: i32,
    pub nLogicalMax: i32,
    pub Units: PROPERTY_UNITS,
    pub fResolution: f32,
}
pub type PROPERTY_UNITS = i32;
pub const PROPERTY_UNITS_AMPERE: PROPERTY_UNITS = 15;
pub const PROPERTY_UNITS_CANDELA: PROPERTY_UNITS = 16;
pub const PROPERTY_UNITS_CENTIMETERS: PROPERTY_UNITS = 2;
pub const PROPERTY_UNITS_DEFAULT: PROPERTY_UNITS = 0;
pub const PROPERTY_UNITS_DEGREES: PROPERTY_UNITS = 3;
pub const PROPERTY_UNITS_ENGLINEAR: PROPERTY_UNITS = 10;
pub const PROPERTY_UNITS_ENGROTATION: PROPERTY_UNITS = 11;
pub const PROPERTY_UNITS_FAHRENHEIT: PROPERTY_UNITS = 14;
pub const PROPERTY_UNITS_GRAMS: PROPERTY_UNITS = 7;
pub const PROPERTY_UNITS_INCHES: PROPERTY_UNITS = 1;
pub const PROPERTY_UNITS_KELVIN: PROPERTY_UNITS = 13;
pub const PROPERTY_UNITS_POUNDS: PROPERTY_UNITS = 6;
pub const PROPERTY_UNITS_RADIANS: PROPERTY_UNITS = 4;
pub const PROPERTY_UNITS_SECONDS: PROPERTY_UNITS = 5;
pub const PROPERTY_UNITS_SILINEAR: PROPERTY_UNITS = 8;
pub const PROPERTY_UNITS_SIROTATION: PROPERTY_UNITS = 9;
pub const PROPERTY_UNITS_SLUGS: PROPERTY_UNITS = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STROKE_RANGE {
    pub iStrokeBegin: u32,
    pub iStrokeEnd: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SYSTEM_EVENT(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_EVENT_DATA {
    pub bModifier: u8,
    pub wKey: u16,
    pub xPos: i32,
    pub yPos: i32,
    pub bCursorMode: u8,
    pub dwButtonState: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TABLET_CONTEXT_ID(pub u32);
pub const TABLET_DISABLE_FLICKFALLBACKKEYS: u32 = 1048576;
pub const TABLET_DISABLE_FLICKS: u32 = 65536;
pub const TABLET_DISABLE_PENBARRELFEEDBACK: u32 = 16;
pub const TABLET_DISABLE_PENTAPFEEDBACK: u32 = 8;
pub const TABLET_DISABLE_PRESSANDHOLD: u32 = 1;
pub const TABLET_DISABLE_SMOOTHSCROLLING: u32 = 524288;
pub const TABLET_DISABLE_TOUCHSWITCH: u32 = 32768;
pub const TABLET_DISABLE_TOUCHUIFORCEOFF: u32 = 512;
pub const TABLET_DISABLE_TOUCHUIFORCEON: u32 = 256;
pub const TABLET_ENABLE_FLICKLEARNINGMODE: u32 = 262144;
pub const TABLET_ENABLE_FLICKSONCONTEXT: u32 = 131072;
pub const TABLET_ENABLE_MULTITOUCHDATA: u32 = 16777216;
pub const WM_TABLET_ADDED: u32 = 712;
pub const WM_TABLET_DEFBASE: u32 = 704;
pub const WM_TABLET_DELETED: u32 = 713;
pub const WM_TABLET_FLICK: u32 = 715;
pub const WM_TABLET_MAXOFFSET: u32 = 32;
pub const WM_TABLET_QUERYSYSTEMGESTURESTATUS: u32 = 716;
