#[inline]
pub unsafe fn GameInputCreate() -> windows_core::Result<IGameInput> {
    windows_core::link!("gameinput.dll" "system" fn GameInputCreate(gameinput : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GameInputCreate(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const FACILITY_GAMEINPUT: u32 = 906u32;
pub const GAMEINPUT_E_DEVICE_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x838A0001_u32 as _);
pub const GAMEINPUT_E_DEVICE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x838A0002_u32 as _);
pub const GAMEINPUT_E_INSUFFICIENT_FORCE_FEEDBACK_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x838A0006_u32 as _);
pub const GAMEINPUT_E_READING_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x838A0003_u32 as _);
pub const GAMEINPUT_E_REFERENCE_READING_TOO_OLD: windows_core::HRESULT = windows_core::HRESULT(0x838A0004_u32 as _);
pub const GAMEINPUT_E_TIMESTAMP_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x838A0005_u32 as _);
pub const GameInput2WaySwitch: GameInputSwitchKind = GameInputSwitchKind(0i32);
pub const GameInput4WaySwitch: GameInputSwitchKind = GameInputSwitchKind(1i32);
pub const GameInput8WaySwitch: GameInputSwitchKind = GameInputSwitchKind(2i32);
pub const GameInputAbntKeyboard: GameInputKeyboardKind = GameInputKeyboardKind(3i32);
pub const GameInputAnsiKeyboard: GameInputKeyboardKind = GameInputKeyboardKind(0i32);
pub const GameInputApplicationItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(1i32);
pub const GameInputArcadeStickAction1: GameInputArcadeStickButtons = GameInputArcadeStickButtons(64i32);
pub const GameInputArcadeStickAction2: GameInputArcadeStickButtons = GameInputArcadeStickButtons(128i32);
pub const GameInputArcadeStickAction3: GameInputArcadeStickButtons = GameInputArcadeStickButtons(256i32);
pub const GameInputArcadeStickAction4: GameInputArcadeStickButtons = GameInputArcadeStickButtons(512i32);
pub const GameInputArcadeStickAction5: GameInputArcadeStickButtons = GameInputArcadeStickButtons(1024i32);
pub const GameInputArcadeStickAction6: GameInputArcadeStickButtons = GameInputArcadeStickButtons(2048i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputArcadeStickButtons(pub i32);
impl GameInputArcadeStickButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputArcadeStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputArcadeStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputArcadeStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputArcadeStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputArcadeStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputArcadeStickDown: GameInputArcadeStickButtons = GameInputArcadeStickButtons(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputArcadeStickInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub stickUpLabel: GameInputLabel,
    pub stickDownLabel: GameInputLabel,
    pub stickLeftLabel: GameInputLabel,
    pub stickRightLabel: GameInputLabel,
    pub actionButton1Label: GameInputLabel,
    pub actionButton2Label: GameInputLabel,
    pub actionButton3Label: GameInputLabel,
    pub actionButton4Label: GameInputLabel,
    pub actionButton5Label: GameInputLabel,
    pub actionButton6Label: GameInputLabel,
    pub specialButton1Label: GameInputLabel,
    pub specialButton2Label: GameInputLabel,
}
pub const GameInputArcadeStickLeft: GameInputArcadeStickButtons = GameInputArcadeStickButtons(16i32);
pub const GameInputArcadeStickMenu: GameInputArcadeStickButtons = GameInputArcadeStickButtons(1i32);
pub const GameInputArcadeStickNone: GameInputArcadeStickButtons = GameInputArcadeStickButtons(0i32);
pub const GameInputArcadeStickRight: GameInputArcadeStickButtons = GameInputArcadeStickButtons(32i32);
pub const GameInputArcadeStickSpecial1: GameInputArcadeStickButtons = GameInputArcadeStickButtons(4096i32);
pub const GameInputArcadeStickSpecial2: GameInputArcadeStickButtons = GameInputArcadeStickButtons(8192i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputArcadeStickState {
    pub buttons: GameInputArcadeStickButtons,
}
pub const GameInputArcadeStickUp: GameInputArcadeStickButtons = GameInputArcadeStickButtons(4i32);
pub const GameInputArcadeStickView: GameInputArcadeStickButtons = GameInputArcadeStickButtons(2i32);
pub const GameInputArrayItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(2i32);
pub const GameInputAsyncEnumeration: GameInputEnumerationKind = GameInputEnumerationKind(1i32);
pub const GameInputBatteryCharging: GameInputBatteryStatus = GameInputBatteryStatus(3i32);
pub const GameInputBatteryDischarging: GameInputBatteryStatus = GameInputBatteryStatus(1i32);
pub const GameInputBatteryIdle: GameInputBatteryStatus = GameInputBatteryStatus(2i32);
pub const GameInputBatteryNotPresent: GameInputBatteryStatus = GameInputBatteryStatus(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputBatteryState {
    pub chargeRate: f32,
    pub maxChargeRate: f32,
    pub remainingCapacity: f32,
    pub fullChargeCapacity: f32,
    pub status: GameInputBatteryStatus,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputBatteryStatus(pub i32);
pub const GameInputBatteryUnknown: GameInputBatteryStatus = GameInputBatteryStatus(-1i32);
pub const GameInputBlockingEnumeration: GameInputEnumerationKind = GameInputEnumerationKind(2i32);
pub const GameInputBufferedItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(256i32);
pub const GameInputConstantItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputControllerAxisInfo {
    pub mappedInputKinds: GameInputKind,
    pub label: GameInputLabel,
    pub isContinuous: u8,
    pub isNonlinear: u8,
    pub isQuantized: u8,
    pub hasRestValue: u8,
    pub restValue: f32,
    pub resolution: u64,
    pub legacyDInputIndex: u16,
    pub legacyHidIndex: u16,
    pub rawReportIndex: u32,
    pub inputReport: *const GameInputRawDeviceReportInfo,
    pub inputReportItem: *const GameInputRawDeviceReportItemInfo,
}
impl Default for GameInputControllerAxisInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputControllerButtonInfo {
    pub mappedInputKinds: GameInputKind,
    pub label: GameInputLabel,
    pub legacyDInputIndex: u16,
    pub legacyHidIndex: u16,
    pub rawReportIndex: u32,
    pub inputReport: *const GameInputRawDeviceReportInfo,
    pub inputReportItem: *const GameInputRawDeviceReportItemInfo,
}
impl Default for GameInputControllerButtonInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputControllerSwitchInfo {
    pub mappedInputKinds: GameInputKind,
    pub label: GameInputLabel,
    pub positionLabels: [GameInputLabel; 9],
    pub kind: GameInputSwitchKind,
    pub legacyDInputIndex: u16,
    pub legacyHidIndex: u16,
    pub rawReportIndex: u32,
    pub inputReport: *const GameInputRawDeviceReportInfo,
    pub inputReportItem: *const GameInputRawDeviceReportItemInfo,
}
impl Default for GameInputControllerSwitchInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GameInputDefaultFocusPolicy: GameInputFocusPolicy = GameInputFocusPolicy(0i32);
pub const GameInputDefaultItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(0i32);
pub const GameInputDeviceAnyStatus: GameInputDeviceStatus = GameInputDeviceStatus(16777215i32);
pub const GameInputDeviceAudioCapture: GameInputDeviceStatus = GameInputDeviceStatus(16i32);
pub const GameInputDeviceAudioRender: GameInputDeviceStatus = GameInputDeviceStatus(32i32);
pub type GameInputDeviceCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: windows_core::Ref<IGameInputDevice>, timestamp: u64, currentstatus: GameInputDeviceStatus, previousstatus: GameInputDeviceStatus)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputDeviceCapabilities(pub i32);
impl GameInputDeviceCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputDeviceCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputDeviceCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputDeviceCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputDeviceCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputDeviceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputDeviceCapabilityAudio: GameInputDeviceCapabilities = GameInputDeviceCapabilities(1i32);
pub const GameInputDeviceCapabilityNone: GameInputDeviceCapabilities = GameInputDeviceCapabilities(0i32);
pub const GameInputDeviceCapabilityPluginModule: GameInputDeviceCapabilities = GameInputDeviceCapabilities(2i32);
pub const GameInputDeviceCapabilityPowerOff: GameInputDeviceCapabilities = GameInputDeviceCapabilities(4i32);
pub const GameInputDeviceCapabilitySynchronization: GameInputDeviceCapabilities = GameInputDeviceCapabilities(8i32);
pub const GameInputDeviceCapabilityWireless: GameInputDeviceCapabilities = GameInputDeviceCapabilities(16i32);
pub const GameInputDeviceConnected: GameInputDeviceStatus = GameInputDeviceStatus(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputDeviceFamily(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputDeviceInfo {
    pub infoSize: u32,
    pub vendorId: u16,
    pub productId: u16,
    pub revisionNumber: u16,
    pub interfaceNumber: u8,
    pub collectionNumber: u8,
    pub usage: GameInputUsage,
    pub hardwareVersion: GameInputVersion,
    pub firmwareVersion: GameInputVersion,
    pub deviceId: super::super::super::Foundation::APP_LOCAL_DEVICE_ID,
    pub deviceRootId: super::super::super::Foundation::APP_LOCAL_DEVICE_ID,
    pub deviceFamily: GameInputDeviceFamily,
    pub capabilities: GameInputDeviceCapabilities,
    pub supportedInput: GameInputKind,
    pub supportedRumbleMotors: GameInputRumbleMotors,
    pub inputReportCount: u32,
    pub outputReportCount: u32,
    pub featureReportCount: u32,
    pub controllerAxisCount: u32,
    pub controllerButtonCount: u32,
    pub controllerSwitchCount: u32,
    pub touchPointCount: u32,
    pub touchSensorCount: u32,
    pub forceFeedbackMotorCount: u32,
    pub hapticFeedbackMotorCount: u32,
    pub deviceStringCount: u32,
    pub deviceDescriptorSize: u32,
    pub inputReportInfo: *const GameInputRawDeviceReportInfo,
    pub outputReportInfo: *const GameInputRawDeviceReportInfo,
    pub featureReportInfo: *const GameInputRawDeviceReportInfo,
    pub controllerAxisInfo: *const GameInputControllerAxisInfo,
    pub controllerButtonInfo: *const GameInputControllerButtonInfo,
    pub controllerSwitchInfo: *const GameInputControllerSwitchInfo,
    pub keyboardInfo: *const GameInputKeyboardInfo,
    pub mouseInfo: *const GameInputMouseInfo,
    pub touchSensorInfo: *const GameInputTouchSensorInfo,
    pub motionInfo: *const GameInputMotionInfo,
    pub arcadeStickInfo: *const GameInputArcadeStickInfo,
    pub flightStickInfo: *const GameInputFlightStickInfo,
    pub gamepadInfo: *const GameInputGamepadInfo,
    pub racingWheelInfo: *const GameInputRacingWheelInfo,
    pub uiNavigationInfo: *const GameInputUiNavigationInfo,
    pub forceFeedbackMotorInfo: *const GameInputForceFeedbackMotorInfo,
    pub hapticFeedbackMotorInfo: *const GameInputHapticFeedbackMotorInfo,
    pub displayName: *const GameInputString,
    pub deviceStrings: *const GameInputString,
    pub deviceDescriptorData: *const core::ffi::c_void,
    pub supportedSystemButtons: GameInputSystemButtons,
}
impl Default for GameInputDeviceInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GameInputDeviceInputEnabled: GameInputDeviceStatus = GameInputDeviceStatus(2i32);
pub const GameInputDeviceNoStatus: GameInputDeviceStatus = GameInputDeviceStatus(0i32);
pub const GameInputDeviceOutputEnabled: GameInputDeviceStatus = GameInputDeviceStatus(4i32);
pub const GameInputDeviceRawIoEnabled: GameInputDeviceStatus = GameInputDeviceStatus(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputDeviceStatus(pub i32);
impl GameInputDeviceStatus {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputDeviceStatus {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputDeviceStatus {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputDeviceStatus {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputDeviceStatus {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputDeviceStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputDeviceSynchronized: GameInputDeviceStatus = GameInputDeviceStatus(64i32);
pub const GameInputDeviceUserIdle: GameInputDeviceStatus = GameInputDeviceStatus(1048576i32);
pub const GameInputDeviceWireless: GameInputDeviceStatus = GameInputDeviceStatus(128i32);
pub const GameInputDisableBackgroundGuideButton: GameInputFocusPolicy = GameInputFocusPolicy(4i32);
pub const GameInputDisableBackgroundInput: GameInputFocusPolicy = GameInputFocusPolicy(1i32);
pub const GameInputDisableBackgroundShareButton: GameInputFocusPolicy = GameInputFocusPolicy(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputEnumerationKind(pub i32);
pub const GameInputExclusiveForegroundGuideButton: GameInputFocusPolicy = GameInputFocusPolicy(8i32);
pub const GameInputExclusiveForegroundInput: GameInputFocusPolicy = GameInputFocusPolicy(2i32);
pub const GameInputExclusiveForegroundShareButton: GameInputFocusPolicy = GameInputFocusPolicy(32i32);
pub const GameInputFamilyAggregate: GameInputDeviceFamily = GameInputDeviceFamily(0i32);
pub const GameInputFamilyHid: GameInputDeviceFamily = GameInputDeviceFamily(3i32);
pub const GameInputFamilyI8042: GameInputDeviceFamily = GameInputDeviceFamily(4i32);
pub const GameInputFamilyVirtual: GameInputDeviceFamily = GameInputDeviceFamily(-1i32);
pub const GameInputFamilyXbox360: GameInputDeviceFamily = GameInputDeviceFamily(2i32);
pub const GameInputFamilyXboxOne: GameInputDeviceFamily = GameInputDeviceFamily(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputFeedbackAxes(pub i32);
impl GameInputFeedbackAxes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputFeedbackAxes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputFeedbackAxes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputFeedbackAxes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputFeedbackAxes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputFeedbackAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputFeedbackAxisAngularX: GameInputFeedbackAxes = GameInputFeedbackAxes(8i32);
pub const GameInputFeedbackAxisAngularY: GameInputFeedbackAxes = GameInputFeedbackAxes(16i32);
pub const GameInputFeedbackAxisAngularZ: GameInputFeedbackAxes = GameInputFeedbackAxes(32i32);
pub const GameInputFeedbackAxisLinearX: GameInputFeedbackAxes = GameInputFeedbackAxes(1i32);
pub const GameInputFeedbackAxisLinearY: GameInputFeedbackAxes = GameInputFeedbackAxes(2i32);
pub const GameInputFeedbackAxisLinearZ: GameInputFeedbackAxes = GameInputFeedbackAxes(4i32);
pub const GameInputFeedbackAxisNone: GameInputFeedbackAxes = GameInputFeedbackAxes(0i32);
pub const GameInputFeedbackAxisNormal: GameInputFeedbackAxes = GameInputFeedbackAxes(64i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputFeedbackEffectState(pub i32);
pub const GameInputFeedbackPaused: GameInputFeedbackEffectState = GameInputFeedbackEffectState(2i32);
pub const GameInputFeedbackRunning: GameInputFeedbackEffectState = GameInputFeedbackEffectState(1i32);
pub const GameInputFeedbackStopped: GameInputFeedbackEffectState = GameInputFeedbackEffectState(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputFlightStickButtons(pub i32);
impl GameInputFlightStickButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputFlightStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputFlightStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputFlightStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputFlightStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputFlightStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputFlightStickFirePrimary: GameInputFlightStickButtons = GameInputFlightStickButtons(4i32);
pub const GameInputFlightStickFireSecondary: GameInputFlightStickButtons = GameInputFlightStickButtons(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputFlightStickInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub firePrimaryButtonLabel: GameInputLabel,
    pub fireSecondaryButtonLabel: GameInputLabel,
    pub hatSwitchKind: GameInputSwitchKind,
}
pub const GameInputFlightStickMenu: GameInputFlightStickButtons = GameInputFlightStickButtons(1i32);
pub const GameInputFlightStickNone: GameInputFlightStickButtons = GameInputFlightStickButtons(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputFlightStickState {
    pub buttons: GameInputFlightStickButtons,
    pub hatSwitch: GameInputSwitchPosition,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub throttle: f32,
}
pub const GameInputFlightStickView: GameInputFlightStickButtons = GameInputFlightStickButtons(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputFocusPolicy(pub i32);
impl GameInputFocusPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputFocusPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputFocusPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputFocusPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputFocusPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputFocusPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackConditionParams {
    pub magnitude: GameInputForceFeedbackMagnitude,
    pub positiveCoefficient: f32,
    pub negativeCoefficient: f32,
    pub maxPositiveMagnitude: f32,
    pub maxNegativeMagnitude: f32,
    pub deadZone: f32,
    pub bias: f32,
}
pub const GameInputForceFeedbackConstant: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackConstantParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub magnitude: GameInputForceFeedbackMagnitude,
}
pub const GameInputForceFeedbackDamper: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputForceFeedbackEffectKind(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackEnvelope {
    pub attackDuration: u64,
    pub sustainDuration: u64,
    pub releaseDuration: u64,
    pub attackGain: f32,
    pub sustainGain: f32,
    pub releaseGain: f32,
    pub playCount: u32,
    pub repeatDelay: u64,
}
pub const GameInputForceFeedbackFriction: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(8i32);
pub const GameInputForceFeedbackInertia: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(10i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackMagnitude {
    pub linearX: f32,
    pub linearY: f32,
    pub linearZ: f32,
    pub angularX: f32,
    pub angularY: f32,
    pub angularZ: f32,
    pub normal: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackMotorInfo {
    pub supportedAxes: GameInputFeedbackAxes,
    pub location: GameInputLocation,
    pub locationId: u32,
    pub maxSimultaneousEffects: u32,
    pub isConstantEffectSupported: u8,
    pub isRampEffectSupported: u8,
    pub isSineWaveEffectSupported: u8,
    pub isSquareWaveEffectSupported: u8,
    pub isTriangleWaveEffectSupported: u8,
    pub isSawtoothUpWaveEffectSupported: u8,
    pub isSawtoothDownWaveEffectSupported: u8,
    pub isSpringEffectSupported: u8,
    pub isFrictionEffectSupported: u8,
    pub isDamperEffectSupported: u8,
    pub isInertiaEffectSupported: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GameInputForceFeedbackParams {
    pub kind: GameInputForceFeedbackEffectKind,
    pub data: GameInputForceFeedbackParams_0,
}
impl Default for GameInputForceFeedbackParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GameInputForceFeedbackParams_0 {
    pub constant: GameInputForceFeedbackConstantParams,
    pub ramp: GameInputForceFeedbackRampParams,
    pub sineWave: GameInputForceFeedbackPeriodicParams,
    pub squareWave: GameInputForceFeedbackPeriodicParams,
    pub triangleWave: GameInputForceFeedbackPeriodicParams,
    pub sawtoothUpWave: GameInputForceFeedbackPeriodicParams,
    pub sawtoothDownWave: GameInputForceFeedbackPeriodicParams,
    pub spring: GameInputForceFeedbackConditionParams,
    pub friction: GameInputForceFeedbackConditionParams,
    pub damper: GameInputForceFeedbackConditionParams,
    pub inertia: GameInputForceFeedbackConditionParams,
}
impl Default for GameInputForceFeedbackParams_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackPeriodicParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub magnitude: GameInputForceFeedbackMagnitude,
    pub frequency: f32,
    pub phase: f32,
    pub bias: f32,
}
pub const GameInputForceFeedbackRamp: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackRampParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub startMagnitude: GameInputForceFeedbackMagnitude,
    pub endMagnitude: GameInputForceFeedbackMagnitude,
}
pub const GameInputForceFeedbackSawtoothDownWave: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(6i32);
pub const GameInputForceFeedbackSawtoothUpWave: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(5i32);
pub const GameInputForceFeedbackSineWave: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(2i32);
pub const GameInputForceFeedbackSpring: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(7i32);
pub const GameInputForceFeedbackSquareWave: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(3i32);
pub const GameInputForceFeedbackTriangleWave: GameInputForceFeedbackEffectKind = GameInputForceFeedbackEffectKind(4i32);
pub const GameInputGamepadA: GameInputGamepadButtons = GameInputGamepadButtons(4i32);
pub const GameInputGamepadB: GameInputGamepadButtons = GameInputGamepadButtons(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputGamepadButtons(pub i32);
impl GameInputGamepadButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputGamepadButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputGamepadButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputGamepadButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputGamepadButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputGamepadButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputGamepadDPadDown: GameInputGamepadButtons = GameInputGamepadButtons(128i32);
pub const GameInputGamepadDPadLeft: GameInputGamepadButtons = GameInputGamepadButtons(256i32);
pub const GameInputGamepadDPadRight: GameInputGamepadButtons = GameInputGamepadButtons(512i32);
pub const GameInputGamepadDPadUp: GameInputGamepadButtons = GameInputGamepadButtons(64i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputGamepadInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub aButtonLabel: GameInputLabel,
    pub bButtonLabel: GameInputLabel,
    pub xButtonLabel: GameInputLabel,
    pub yButtonLabel: GameInputLabel,
    pub dpadUpLabel: GameInputLabel,
    pub dpadDownLabel: GameInputLabel,
    pub dpadLeftLabel: GameInputLabel,
    pub dpadRightLabel: GameInputLabel,
    pub leftShoulderButtonLabel: GameInputLabel,
    pub rightShoulderButtonLabel: GameInputLabel,
    pub leftThumbstickButtonLabel: GameInputLabel,
    pub rightThumbstickButtonLabel: GameInputLabel,
}
pub const GameInputGamepadLeftShoulder: GameInputGamepadButtons = GameInputGamepadButtons(1024i32);
pub const GameInputGamepadLeftThumbstick: GameInputGamepadButtons = GameInputGamepadButtons(4096i32);
pub const GameInputGamepadMenu: GameInputGamepadButtons = GameInputGamepadButtons(1i32);
pub const GameInputGamepadNone: GameInputGamepadButtons = GameInputGamepadButtons(0i32);
pub const GameInputGamepadRightShoulder: GameInputGamepadButtons = GameInputGamepadButtons(2048i32);
pub const GameInputGamepadRightThumbstick: GameInputGamepadButtons = GameInputGamepadButtons(8192i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputGamepadState {
    pub buttons: GameInputGamepadButtons,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
    pub leftThumbstickX: f32,
    pub leftThumbstickY: f32,
    pub rightThumbstickX: f32,
    pub rightThumbstickY: f32,
}
pub const GameInputGamepadView: GameInputGamepadButtons = GameInputGamepadButtons(2i32);
pub const GameInputGamepadX: GameInputGamepadButtons = GameInputGamepadButtons(16i32);
pub const GameInputGamepadY: GameInputGamepadButtons = GameInputGamepadButtons(32i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputHapticFeedbackMotorInfo {
    pub mappedRumbleMotors: GameInputRumbleMotors,
    pub location: GameInputLocation,
    pub locationId: u32,
    pub waveformCount: u32,
    pub waveformInfo: *const GameInputHapticWaveformInfo,
}
impl Default for GameInputHapticFeedbackMotorInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputHapticFeedbackParams {
    pub waveformIndex: u32,
    pub duration: u64,
    pub intensity: f32,
    pub playCount: u32,
    pub repeatDelay: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputHapticWaveformInfo {
    pub usage: GameInputUsage,
    pub isDurationSupported: u8,
    pub isIntensitySupported: u8,
    pub isRepeatSupported: u8,
    pub isRepeatDelaySupported: u8,
    pub defaultDuration: u64,
}
pub const GameInputIsoKeyboard: GameInputKeyboardKind = GameInputKeyboardKind(1i32);
pub const GameInputJisKeyboard: GameInputKeyboardKind = GameInputKeyboardKind(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputKeyState {
    pub scanCode: u32,
    pub codePoint: u32,
    pub virtualKey: u8,
    pub isDeadKey: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputKeyboardInfo {
    pub kind: GameInputKeyboardKind,
    pub layout: u32,
    pub keyCount: u32,
    pub functionKeyCount: u32,
    pub maxSimultaneousKeys: u32,
    pub platformType: u32,
    pub platformSubtype: u32,
    pub nativeLanguage: *const GameInputString,
}
impl Default for GameInputKeyboardInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputKeyboardKind(pub i32);
pub type GameInputKeyboardLayoutCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: windows_core::Ref<IGameInputDevice>, timestamp: u64, currentlayout: u32, previouslayout: u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputKind(pub i32);
impl GameInputKind {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputKind {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputKind {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputKind {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputKind {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputKind {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputKindArcadeStick: GameInputKind = GameInputKind(65536i32);
pub const GameInputKindController: GameInputKind = GameInputKind(14i32);
pub const GameInputKindControllerAxis: GameInputKind = GameInputKind(2i32);
pub const GameInputKindControllerButton: GameInputKind = GameInputKind(4i32);
pub const GameInputKindControllerSwitch: GameInputKind = GameInputKind(8i32);
pub const GameInputKindFlightStick: GameInputKind = GameInputKind(131072i32);
pub const GameInputKindGamepad: GameInputKind = GameInputKind(262144i32);
pub const GameInputKindKeyboard: GameInputKind = GameInputKind(16i32);
pub const GameInputKindMotion: GameInputKind = GameInputKind(4096i32);
pub const GameInputKindMouse: GameInputKind = GameInputKind(32i32);
pub const GameInputKindRacingWheel: GameInputKind = GameInputKind(524288i32);
pub const GameInputKindRawDeviceReport: GameInputKind = GameInputKind(1i32);
pub const GameInputKindTouch: GameInputKind = GameInputKind(256i32);
pub const GameInputKindUiNavigation: GameInputKind = GameInputKind(16777216i32);
pub const GameInputKindUnknown: GameInputKind = GameInputKind(0i32);
pub const GameInputKsKeyboard: GameInputKeyboardKind = GameInputKeyboardKind(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputLabel(pub i32);
pub const GameInputLabelArrowClockwise: GameInputLabel = GameInputLabel(72i32);
pub const GameInputLabelArrowCounterClockwise: GameInputLabel = GameInputLabel(73i32);
pub const GameInputLabelArrowDown: GameInputLabel = GameInputLabel(65i32);
pub const GameInputLabelArrowDownLLeft: GameInputLabel = GameInputLabel(66i32);
pub const GameInputLabelArrowDownRight: GameInputLabel = GameInputLabel(64i32);
pub const GameInputLabelArrowLeft: GameInputLabel = GameInputLabel(67i32);
pub const GameInputLabelArrowLeftRight: GameInputLabel = GameInputLabel(70i32);
pub const GameInputLabelArrowReturn: GameInputLabel = GameInputLabel(74i32);
pub const GameInputLabelArrowRight: GameInputLabel = GameInputLabel(63i32);
pub const GameInputLabelArrowUp: GameInputLabel = GameInputLabel(61i32);
pub const GameInputLabelArrowUpDown: GameInputLabel = GameInputLabel(69i32);
pub const GameInputLabelArrowUpDownLeftRight: GameInputLabel = GameInputLabel(71i32);
pub const GameInputLabelArrowUpLeft: GameInputLabel = GameInputLabel(68i32);
pub const GameInputLabelArrowUpRight: GameInputLabel = GameInputLabel(62i32);
pub const GameInputLabelBack: GameInputLabel = GameInputLabel(101i32);
pub const GameInputLabelDown: GameInputLabel = GameInputLabel(106i32);
pub const GameInputLabelGuide: GameInputLabel = GameInputLabel(96i32);
pub const GameInputLabelHome: GameInputLabel = GameInputLabel(95i32);
pub const GameInputLabelIconBranding: GameInputLabel = GameInputLabel(75i32);
pub const GameInputLabelIconCircle: GameInputLabel = GameInputLabel(79i32);
pub const GameInputLabelIconCross: GameInputLabel = GameInputLabel(78i32);
pub const GameInputLabelIconDPadDown: GameInputLabel = GameInputLabel(84i32);
pub const GameInputLabelIconDPadLeft: GameInputLabel = GameInputLabel(85i32);
pub const GameInputLabelIconDPadRight: GameInputLabel = GameInputLabel(86i32);
pub const GameInputLabelIconDPadUp: GameInputLabel = GameInputLabel(83i32);
pub const GameInputLabelIconDialClockwise: GameInputLabel = GameInputLabel(87i32);
pub const GameInputLabelIconDialCounterClockwise: GameInputLabel = GameInputLabel(88i32);
pub const GameInputLabelIconHome: GameInputLabel = GameInputLabel(76i32);
pub const GameInputLabelIconMenu: GameInputLabel = GameInputLabel(77i32);
pub const GameInputLabelIconMinus: GameInputLabel = GameInputLabel(93i32);
pub const GameInputLabelIconPlus: GameInputLabel = GameInputLabel(92i32);
pub const GameInputLabelIconSliderLeftRight: GameInputLabel = GameInputLabel(89i32);
pub const GameInputLabelIconSliderUpDown: GameInputLabel = GameInputLabel(90i32);
pub const GameInputLabelIconSquare: GameInputLabel = GameInputLabel(80i32);
pub const GameInputLabelIconStar: GameInputLabel = GameInputLabel(82i32);
pub const GameInputLabelIconSuspension: GameInputLabel = GameInputLabel(94i32);
pub const GameInputLabelIconTriangle: GameInputLabel = GameInputLabel(81i32);
pub const GameInputLabelIconWheelUpDown: GameInputLabel = GameInputLabel(91i32);
pub const GameInputLabelL1: GameInputLabel = GameInputLabel(112i32);
pub const GameInputLabelL2: GameInputLabel = GameInputLabel(113i32);
pub const GameInputLabelL3: GameInputLabel = GameInputLabel(114i32);
pub const GameInputLabelLB: GameInputLabel = GameInputLabel(109i32);
pub const GameInputLabelLSB: GameInputLabel = GameInputLabel(111i32);
pub const GameInputLabelLT: GameInputLabel = GameInputLabel(110i32);
pub const GameInputLabelLeft: GameInputLabel = GameInputLabel(107i32);
pub const GameInputLabelLetterA: GameInputLabel = GameInputLabel(25i32);
pub const GameInputLabelLetterB: GameInputLabel = GameInputLabel(26i32);
pub const GameInputLabelLetterC: GameInputLabel = GameInputLabel(27i32);
pub const GameInputLabelLetterD: GameInputLabel = GameInputLabel(28i32);
pub const GameInputLabelLetterE: GameInputLabel = GameInputLabel(29i32);
pub const GameInputLabelLetterF: GameInputLabel = GameInputLabel(30i32);
pub const GameInputLabelLetterG: GameInputLabel = GameInputLabel(31i32);
pub const GameInputLabelLetterH: GameInputLabel = GameInputLabel(32i32);
pub const GameInputLabelLetterI: GameInputLabel = GameInputLabel(33i32);
pub const GameInputLabelLetterJ: GameInputLabel = GameInputLabel(34i32);
pub const GameInputLabelLetterK: GameInputLabel = GameInputLabel(35i32);
pub const GameInputLabelLetterL: GameInputLabel = GameInputLabel(36i32);
pub const GameInputLabelLetterM: GameInputLabel = GameInputLabel(37i32);
pub const GameInputLabelLetterN: GameInputLabel = GameInputLabel(38i32);
pub const GameInputLabelLetterO: GameInputLabel = GameInputLabel(39i32);
pub const GameInputLabelLetterP: GameInputLabel = GameInputLabel(40i32);
pub const GameInputLabelLetterQ: GameInputLabel = GameInputLabel(41i32);
pub const GameInputLabelLetterR: GameInputLabel = GameInputLabel(42i32);
pub const GameInputLabelLetterS: GameInputLabel = GameInputLabel(43i32);
pub const GameInputLabelLetterT: GameInputLabel = GameInputLabel(44i32);
pub const GameInputLabelLetterU: GameInputLabel = GameInputLabel(45i32);
pub const GameInputLabelLetterV: GameInputLabel = GameInputLabel(46i32);
pub const GameInputLabelLetterW: GameInputLabel = GameInputLabel(47i32);
pub const GameInputLabelLetterX: GameInputLabel = GameInputLabel(48i32);
pub const GameInputLabelLetterY: GameInputLabel = GameInputLabel(49i32);
pub const GameInputLabelLetterZ: GameInputLabel = GameInputLabel(50i32);
pub const GameInputLabelMenu: GameInputLabel = GameInputLabel(99i32);
pub const GameInputLabelMode: GameInputLabel = GameInputLabel(97i32);
pub const GameInputLabelNone: GameInputLabel = GameInputLabel(0i32);
pub const GameInputLabelNumber0: GameInputLabel = GameInputLabel(51i32);
pub const GameInputLabelNumber1: GameInputLabel = GameInputLabel(52i32);
pub const GameInputLabelNumber2: GameInputLabel = GameInputLabel(53i32);
pub const GameInputLabelNumber3: GameInputLabel = GameInputLabel(54i32);
pub const GameInputLabelNumber4: GameInputLabel = GameInputLabel(55i32);
pub const GameInputLabelNumber5: GameInputLabel = GameInputLabel(56i32);
pub const GameInputLabelNumber6: GameInputLabel = GameInputLabel(57i32);
pub const GameInputLabelNumber7: GameInputLabel = GameInputLabel(58i32);
pub const GameInputLabelNumber8: GameInputLabel = GameInputLabel(59i32);
pub const GameInputLabelNumber9: GameInputLabel = GameInputLabel(60i32);
pub const GameInputLabelOptions: GameInputLabel = GameInputLabel(103i32);
pub const GameInputLabelP1: GameInputLabel = GameInputLabel(121i32);
pub const GameInputLabelP2: GameInputLabel = GameInputLabel(122i32);
pub const GameInputLabelP3: GameInputLabel = GameInputLabel(123i32);
pub const GameInputLabelP4: GameInputLabel = GameInputLabel(124i32);
pub const GameInputLabelR1: GameInputLabel = GameInputLabel(118i32);
pub const GameInputLabelR2: GameInputLabel = GameInputLabel(119i32);
pub const GameInputLabelR3: GameInputLabel = GameInputLabel(120i32);
pub const GameInputLabelRB: GameInputLabel = GameInputLabel(115i32);
pub const GameInputLabelRSB: GameInputLabel = GameInputLabel(117i32);
pub const GameInputLabelRT: GameInputLabel = GameInputLabel(116i32);
pub const GameInputLabelRight: GameInputLabel = GameInputLabel(108i32);
pub const GameInputLabelSelect: GameInputLabel = GameInputLabel(98i32);
pub const GameInputLabelShare: GameInputLabel = GameInputLabel(104i32);
pub const GameInputLabelStart: GameInputLabel = GameInputLabel(102i32);
pub const GameInputLabelUnknown: GameInputLabel = GameInputLabel(-1i32);
pub const GameInputLabelUp: GameInputLabel = GameInputLabel(105i32);
pub const GameInputLabelView: GameInputLabel = GameInputLabel(100i32);
pub const GameInputLabelXboxA: GameInputLabel = GameInputLabel(7i32);
pub const GameInputLabelXboxB: GameInputLabel = GameInputLabel(8i32);
pub const GameInputLabelXboxBack: GameInputLabel = GameInputLabel(2i32);
pub const GameInputLabelXboxDPadDown: GameInputLabel = GameInputLabel(12i32);
pub const GameInputLabelXboxDPadLeft: GameInputLabel = GameInputLabel(13i32);
pub const GameInputLabelXboxDPadRight: GameInputLabel = GameInputLabel(14i32);
pub const GameInputLabelXboxDPadUp: GameInputLabel = GameInputLabel(11i32);
pub const GameInputLabelXboxGuide: GameInputLabel = GameInputLabel(1i32);
pub const GameInputLabelXboxLeftShoulder: GameInputLabel = GameInputLabel(15i32);
pub const GameInputLabelXboxLeftStickButton: GameInputLabel = GameInputLabel(17i32);
pub const GameInputLabelXboxLeftTrigger: GameInputLabel = GameInputLabel(16i32);
pub const GameInputLabelXboxMenu: GameInputLabel = GameInputLabel(4i32);
pub const GameInputLabelXboxPaddle1: GameInputLabel = GameInputLabel(21i32);
pub const GameInputLabelXboxPaddle2: GameInputLabel = GameInputLabel(22i32);
pub const GameInputLabelXboxPaddle3: GameInputLabel = GameInputLabel(23i32);
pub const GameInputLabelXboxPaddle4: GameInputLabel = GameInputLabel(24i32);
pub const GameInputLabelXboxRightShoulder: GameInputLabel = GameInputLabel(18i32);
pub const GameInputLabelXboxRightStickButton: GameInputLabel = GameInputLabel(20i32);
pub const GameInputLabelXboxRightTrigger: GameInputLabel = GameInputLabel(19i32);
pub const GameInputLabelXboxStart: GameInputLabel = GameInputLabel(3i32);
pub const GameInputLabelXboxView: GameInputLabel = GameInputLabel(5i32);
pub const GameInputLabelXboxX: GameInputLabel = GameInputLabel(9i32);
pub const GameInputLabelXboxY: GameInputLabel = GameInputLabel(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputLocation(pub i32);
pub const GameInputLocationAxis: GameInputLocation = GameInputLocation(2i32);
pub const GameInputLocationButton: GameInputLocation = GameInputLocation(3i32);
pub const GameInputLocationChassis: GameInputLocation = GameInputLocation(0i32);
pub const GameInputLocationDisplay: GameInputLocation = GameInputLocation(1i32);
pub const GameInputLocationKey: GameInputLocation = GameInputLocation(5i32);
pub const GameInputLocationSwitch: GameInputLocation = GameInputLocation(4i32);
pub const GameInputLocationTouchPad: GameInputLocation = GameInputLocation(6i32);
pub const GameInputLocationUnknown: GameInputLocation = GameInputLocation(-1i32);
pub const GameInputLogicalItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputMotionAccuracy(pub i32);
pub const GameInputMotionAccuracyUnknown: GameInputMotionAccuracy = GameInputMotionAccuracy(-1i32);
pub const GameInputMotionAccurate: GameInputMotionAccuracy = GameInputMotionAccuracy(3i32);
pub const GameInputMotionApproximate: GameInputMotionAccuracy = GameInputMotionAccuracy(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputMotionInfo {
    pub maxAcceleration: f32,
    pub maxAngularVelocity: f32,
    pub maxMagneticFieldStrength: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputMotionState {
    pub accelerationX: f32,
    pub accelerationY: f32,
    pub accelerationZ: f32,
    pub angularVelocityX: f32,
    pub angularVelocityY: f32,
    pub angularVelocityZ: f32,
    pub magneticFieldX: f32,
    pub magneticFieldY: f32,
    pub magneticFieldZ: f32,
    pub orientationW: f32,
    pub orientationX: f32,
    pub orientationY: f32,
    pub orientationZ: f32,
    pub accelerometerAccuracy: GameInputMotionAccuracy,
    pub gyroscopeAccuracy: GameInputMotionAccuracy,
    pub magnetometerAccuracy: GameInputMotionAccuracy,
    pub orientationAccuracy: GameInputMotionAccuracy,
}
pub const GameInputMotionUnavailable: GameInputMotionAccuracy = GameInputMotionAccuracy(0i32);
pub const GameInputMotionUnreliable: GameInputMotionAccuracy = GameInputMotionAccuracy(1i32);
pub const GameInputMouseButton4: GameInputMouseButtons = GameInputMouseButtons(8i32);
pub const GameInputMouseButton5: GameInputMouseButtons = GameInputMouseButtons(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputMouseButtons(pub i32);
impl GameInputMouseButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputMouseButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputMouseButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputMouseButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputMouseButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputMouseButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputMouseInfo {
    pub supportedButtons: GameInputMouseButtons,
    pub sampleRate: u32,
    pub sensorDpi: u32,
    pub hasWheelX: u8,
    pub hasWheelY: u8,
}
pub const GameInputMouseLeftButton: GameInputMouseButtons = GameInputMouseButtons(1i32);
pub const GameInputMouseMiddleButton: GameInputMouseButtons = GameInputMouseButtons(4i32);
pub const GameInputMouseNone: GameInputMouseButtons = GameInputMouseButtons(0i32);
pub const GameInputMouseRightButton: GameInputMouseButtons = GameInputMouseButtons(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputMouseState {
    pub buttons: GameInputMouseButtons,
    pub positionX: i64,
    pub positionY: i64,
    pub wheelX: i64,
    pub wheelY: i64,
}
pub const GameInputMouseWheelTiltLeft: GameInputMouseButtons = GameInputMouseButtons(32i32);
pub const GameInputMouseWheelTiltRight: GameInputMouseButtons = GameInputMouseButtons(64i32);
pub const GameInputNamedArrayItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(4i32);
pub const GameInputNoEnumeration: GameInputEnumerationKind = GameInputEnumerationKind(0i32);
pub const GameInputNonlinearItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(16i32);
pub const GameInputNullableItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(64i32);
pub const GameInputPhysicalItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(0i32);
pub const GameInputPhysicalUnitAcceleration: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(5i32);
pub const GameInputPhysicalUnitAngle: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(10i32);
pub const GameInputPhysicalUnitAngularAcceleration: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(12i32);
pub const GameInputPhysicalUnitAngularMass: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(13i32);
pub const GameInputPhysicalUnitAngularMomentum: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(14i32);
pub const GameInputPhysicalUnitAngularTorque: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(15i32);
pub const GameInputPhysicalUnitAngularVelocity: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(11i32);
pub const GameInputPhysicalUnitElectricCharge: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(17i32);
pub const GameInputPhysicalUnitElectricCurrent: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(16i32);
pub const GameInputPhysicalUnitElectricPotential: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(18i32);
pub const GameInputPhysicalUnitEnergy: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(19i32);
pub const GameInputPhysicalUnitForce: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(8i32);
pub const GameInputPhysicalUnitFrequency: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(2i32);
pub const GameInputPhysicalUnitIlluminance: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(24i32);
pub const GameInputPhysicalUnitLength: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(3i32);
pub const GameInputPhysicalUnitLuminousFlux: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(23i32);
pub const GameInputPhysicalUnitLuminousIntensity: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(22i32);
pub const GameInputPhysicalUnitMass: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(6i32);
pub const GameInputPhysicalUnitMomentum: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(7i32);
pub const GameInputPhysicalUnitNone: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(0i32);
pub const GameInputPhysicalUnitPower: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(20i32);
pub const GameInputPhysicalUnitPressure: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(9i32);
pub const GameInputPhysicalUnitTemperature: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(21i32);
pub const GameInputPhysicalUnitTime: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(1i32);
pub const GameInputPhysicalUnitUnknown: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(-1i32);
pub const GameInputPhysicalUnitVelocity: GameInputRawDevicePhysicalUnitKind = GameInputRawDevicePhysicalUnitKind(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputRacingWheelButtons(pub i32);
impl GameInputRacingWheelButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputRacingWheelButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputRacingWheelButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputRacingWheelButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputRacingWheelButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputRacingWheelButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputRacingWheelDpadDown: GameInputRacingWheelButtons = GameInputRacingWheelButtons(32i32);
pub const GameInputRacingWheelDpadLeft: GameInputRacingWheelButtons = GameInputRacingWheelButtons(64i32);
pub const GameInputRacingWheelDpadRight: GameInputRacingWheelButtons = GameInputRacingWheelButtons(128i32);
pub const GameInputRacingWheelDpadUp: GameInputRacingWheelButtons = GameInputRacingWheelButtons(16i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputRacingWheelInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub previousGearButtonLabel: GameInputLabel,
    pub nextGearButtonLabel: GameInputLabel,
    pub dpadUpLabel: GameInputLabel,
    pub dpadDownLabel: GameInputLabel,
    pub dpadLeftLabel: GameInputLabel,
    pub dpadRightLabel: GameInputLabel,
    pub hasClutch: u8,
    pub hasHandbrake: u8,
    pub hasPatternShifter: u8,
    pub minPatternShifterGear: i32,
    pub maxPatternShifterGear: i32,
    pub maxWheelAngle: f32,
}
pub const GameInputRacingWheelMenu: GameInputRacingWheelButtons = GameInputRacingWheelButtons(1i32);
pub const GameInputRacingWheelNextGear: GameInputRacingWheelButtons = GameInputRacingWheelButtons(8i32);
pub const GameInputRacingWheelNone: GameInputRacingWheelButtons = GameInputRacingWheelButtons(0i32);
pub const GameInputRacingWheelPreviousGear: GameInputRacingWheelButtons = GameInputRacingWheelButtons(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputRacingWheelState {
    pub buttons: GameInputRacingWheelButtons,
    pub patternShifterGear: i32,
    pub wheel: f32,
    pub throttle: f32,
    pub brake: f32,
    pub clutch: f32,
    pub handbrake: f32,
}
pub const GameInputRacingWheelView: GameInputRacingWheelButtons = GameInputRacingWheelButtons(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputRawDeviceItemCollectionInfo {
    pub kind: GameInputRawDeviceItemCollectionKind,
    pub childCount: u32,
    pub siblingCount: u32,
    pub usageCount: u32,
    pub usages: *const GameInputUsage,
    pub parent: *const GameInputRawDeviceItemCollectionInfo,
    pub firstSibling: *const GameInputRawDeviceItemCollectionInfo,
    pub previousSibling: *const GameInputRawDeviceItemCollectionInfo,
    pub nextSibling: *const GameInputRawDeviceItemCollectionInfo,
    pub lastSibling: *const GameInputRawDeviceItemCollectionInfo,
    pub firstChild: *const GameInputRawDeviceItemCollectionInfo,
    pub lastChild: *const GameInputRawDeviceItemCollectionInfo,
}
impl Default for GameInputRawDeviceItemCollectionInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputRawDeviceItemCollectionKind(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputRawDevicePhysicalUnitKind(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputRawDeviceReportInfo {
    pub kind: GameInputRawDeviceReportKind,
    pub id: u32,
    pub size: u32,
    pub itemCount: u32,
    pub items: *const GameInputRawDeviceReportItemInfo,
}
impl Default for GameInputRawDeviceReportInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputRawDeviceReportItemFlags(pub i32);
impl GameInputRawDeviceReportItemFlags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputRawDeviceReportItemFlags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputRawDeviceReportItemFlags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputRawDeviceReportItemFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputRawDeviceReportItemFlags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputRawDeviceReportItemFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameInputRawDeviceReportItemInfo {
    pub bitOffset: u32,
    pub bitSize: u32,
    pub logicalMin: i64,
    pub logicalMax: i64,
    pub physicalMin: f64,
    pub physicalMax: f64,
    pub physicalUnits: GameInputRawDevicePhysicalUnitKind,
    pub rawPhysicalUnits: u32,
    pub rawPhysicalUnitsExponent: i32,
    pub flags: GameInputRawDeviceReportItemFlags,
    pub usageCount: u32,
    pub usages: *const GameInputUsage,
    pub collection: *const GameInputRawDeviceItemCollectionInfo,
    pub itemString: *const GameInputString,
}
impl Default for GameInputRawDeviceReportItemInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputRawDeviceReportKind(pub i32);
pub const GameInputRawFeatureReport: GameInputRawDeviceReportKind = GameInputRawDeviceReportKind(2i32);
pub const GameInputRawInputReport: GameInputRawDeviceReportKind = GameInputRawDeviceReportKind(0i32);
pub const GameInputRawOutputReport: GameInputRawDeviceReportKind = GameInputRawDeviceReportKind(1i32);
pub type GameInputReadingCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, reading: windows_core::Ref<IGameInputReading>, hasoverrunoccurred: bool)>;
pub const GameInputRelativeItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(4i32);
pub const GameInputReportItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(3i32);
pub const GameInputRumbleHighFrequency: GameInputRumbleMotors = GameInputRumbleMotors(2i32);
pub const GameInputRumbleLeftTrigger: GameInputRumbleMotors = GameInputRumbleMotors(4i32);
pub const GameInputRumbleLowFrequency: GameInputRumbleMotors = GameInputRumbleMotors(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputRumbleMotors(pub i32);
impl GameInputRumbleMotors {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputRumbleMotors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputRumbleMotors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputRumbleMotors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputRumbleMotors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputRumbleMotors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputRumbleNone: GameInputRumbleMotors = GameInputRumbleMotors(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputRumbleParams {
    pub lowFrequency: f32,
    pub highFrequency: f32,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
}
pub const GameInputRumbleRightTrigger: GameInputRumbleMotors = GameInputRumbleMotors(8i32);
pub const GameInputStableItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(32i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputString {
    pub sizeInBytes: u32,
    pub codePointCount: u32,
    pub data: windows_core::PCSTR,
}
pub const GameInputSwitchCenter: GameInputSwitchPosition = GameInputSwitchPosition(0i32);
pub const GameInputSwitchDown: GameInputSwitchPosition = GameInputSwitchPosition(5i32);
pub const GameInputSwitchDownLeft: GameInputSwitchPosition = GameInputSwitchPosition(6i32);
pub const GameInputSwitchDownRight: GameInputSwitchPosition = GameInputSwitchPosition(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputSwitchKind(pub i32);
pub const GameInputSwitchLeft: GameInputSwitchPosition = GameInputSwitchPosition(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputSwitchPosition(pub i32);
pub const GameInputSwitchRight: GameInputSwitchPosition = GameInputSwitchPosition(3i32);
pub const GameInputSwitchUp: GameInputSwitchPosition = GameInputSwitchPosition(1i32);
pub const GameInputSwitchUpLeft: GameInputSwitchPosition = GameInputSwitchPosition(8i32);
pub const GameInputSwitchUpRight: GameInputSwitchPosition = GameInputSwitchPosition(2i32);
pub type GameInputSystemButtonCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: windows_core::Ref<IGameInputDevice>, timestamp: u64, currentbuttons: GameInputSystemButtons, previousbuttons: GameInputSystemButtons)>;
pub const GameInputSystemButtonGuide: GameInputSystemButtons = GameInputSystemButtons(1i32);
pub const GameInputSystemButtonNone: GameInputSystemButtons = GameInputSystemButtons(0i32);
pub const GameInputSystemButtonShare: GameInputSystemButtons = GameInputSystemButtons(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputSystemButtons(pub i32);
impl GameInputSystemButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputSystemButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputSystemButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputSystemButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputSystemButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputSystemButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputTouchSensorInfo {
    pub mappedInputKinds: GameInputKind,
    pub label: GameInputLabel,
    pub location: GameInputLocation,
    pub locationId: u32,
    pub resolutionX: u64,
    pub resolutionY: u64,
    pub shape: GameInputTouchShape,
    pub aspectRatio: f32,
    pub orientation: f32,
    pub physicalWidth: f32,
    pub physicalHeight: f32,
    pub maxPressure: f32,
    pub maxProximity: f32,
    pub maxTouchPoints: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputTouchShape(pub i32);
pub const GameInputTouchShape1DIrregular: GameInputTouchShape = GameInputTouchShape(3i32);
pub const GameInputTouchShape1DLinear: GameInputTouchShape = GameInputTouchShape(1i32);
pub const GameInputTouchShape1DRadial: GameInputTouchShape = GameInputTouchShape(2i32);
pub const GameInputTouchShape2DElliptical: GameInputTouchShape = GameInputTouchShape(5i32);
pub const GameInputTouchShape2DIrregular: GameInputTouchShape = GameInputTouchShape(6i32);
pub const GameInputTouchShape2DRectangular: GameInputTouchShape = GameInputTouchShape(4i32);
pub const GameInputTouchShapePoint: GameInputTouchShape = GameInputTouchShape(0i32);
pub const GameInputTouchShapeUnknown: GameInputTouchShape = GameInputTouchShape(-1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputTouchState {
    pub touchId: u64,
    pub sensorIndex: u32,
    pub positionX: f32,
    pub positionY: f32,
    pub pressure: f32,
    pub proximity: f32,
    pub contactRectTop: f32,
    pub contactRectLeft: f32,
    pub contactRectRight: f32,
    pub contactRectBottom: f32,
}
pub const GameInputUiNavigationAccept: GameInputUiNavigationButtons = GameInputUiNavigationButtons(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameInputUiNavigationButtons(pub i32);
impl GameInputUiNavigationButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GameInputUiNavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GameInputUiNavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GameInputUiNavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GameInputUiNavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GameInputUiNavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GameInputUiNavigationCancel: GameInputUiNavigationButtons = GameInputUiNavigationButtons(8i32);
pub const GameInputUiNavigationContext1: GameInputUiNavigationButtons = GameInputUiNavigationButtons(256i32);
pub const GameInputUiNavigationContext2: GameInputUiNavigationButtons = GameInputUiNavigationButtons(512i32);
pub const GameInputUiNavigationContext3: GameInputUiNavigationButtons = GameInputUiNavigationButtons(1024i32);
pub const GameInputUiNavigationContext4: GameInputUiNavigationButtons = GameInputUiNavigationButtons(2048i32);
pub const GameInputUiNavigationDown: GameInputUiNavigationButtons = GameInputUiNavigationButtons(32i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputUiNavigationInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub acceptButtonLabel: GameInputLabel,
    pub cancelButtonLabel: GameInputLabel,
    pub upButtonLabel: GameInputLabel,
    pub downButtonLabel: GameInputLabel,
    pub leftButtonLabel: GameInputLabel,
    pub rightButtonLabel: GameInputLabel,
    pub contextButton1Label: GameInputLabel,
    pub contextButton2Label: GameInputLabel,
    pub contextButton3Label: GameInputLabel,
    pub contextButton4Label: GameInputLabel,
    pub pageUpButtonLabel: GameInputLabel,
    pub pageDownButtonLabel: GameInputLabel,
    pub pageLeftButtonLabel: GameInputLabel,
    pub pageRightButtonLabel: GameInputLabel,
    pub scrollUpButtonLabel: GameInputLabel,
    pub scrollDownButtonLabel: GameInputLabel,
    pub scrollLeftButtonLabel: GameInputLabel,
    pub scrollRightButtonLabel: GameInputLabel,
    pub guideButtonLabel: GameInputLabel,
}
pub const GameInputUiNavigationLeft: GameInputUiNavigationButtons = GameInputUiNavigationButtons(64i32);
pub const GameInputUiNavigationMenu: GameInputUiNavigationButtons = GameInputUiNavigationButtons(1i32);
pub const GameInputUiNavigationNone: GameInputUiNavigationButtons = GameInputUiNavigationButtons(0i32);
pub const GameInputUiNavigationPageDown: GameInputUiNavigationButtons = GameInputUiNavigationButtons(8192i32);
pub const GameInputUiNavigationPageLeft: GameInputUiNavigationButtons = GameInputUiNavigationButtons(16384i32);
pub const GameInputUiNavigationPageRight: GameInputUiNavigationButtons = GameInputUiNavigationButtons(32768i32);
pub const GameInputUiNavigationPageUp: GameInputUiNavigationButtons = GameInputUiNavigationButtons(4096i32);
pub const GameInputUiNavigationRight: GameInputUiNavigationButtons = GameInputUiNavigationButtons(128i32);
pub const GameInputUiNavigationScrollDown: GameInputUiNavigationButtons = GameInputUiNavigationButtons(131072i32);
pub const GameInputUiNavigationScrollLeft: GameInputUiNavigationButtons = GameInputUiNavigationButtons(262144i32);
pub const GameInputUiNavigationScrollRight: GameInputUiNavigationButtons = GameInputUiNavigationButtons(524288i32);
pub const GameInputUiNavigationScrollUp: GameInputUiNavigationButtons = GameInputUiNavigationButtons(65536i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputUiNavigationState {
    pub buttons: GameInputUiNavigationButtons,
}
pub const GameInputUiNavigationUp: GameInputUiNavigationButtons = GameInputUiNavigationButtons(16i32);
pub const GameInputUiNavigationView: GameInputUiNavigationButtons = GameInputUiNavigationButtons(2i32);
pub const GameInputUnknownItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(-1i32);
pub const GameInputUnknownKeyboard: GameInputKeyboardKind = GameInputKeyboardKind(-1i32);
pub const GameInputUnknownSwitchKind: GameInputSwitchKind = GameInputSwitchKind(-1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputUsage {
    pub page: u16,
    pub id: u16,
}
pub const GameInputUsageModifierItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(6i32);
pub const GameInputUsageSwitchItemCollection: GameInputRawDeviceItemCollectionKind = GameInputRawDeviceItemCollectionKind(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub revision: u16,
}
pub const GameInputVolatileItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(128i32);
pub const GameInputWraparoundItem: GameInputRawDeviceReportItemFlags = GameInputRawDeviceReportItemFlags(8i32);
windows_core::imp::define_interface!(IGameInput, IGameInput_Vtbl, 0x11be2a7e_4254_445a_9c09_ffc40f006918);
windows_core::imp::interface_hierarchy!(IGameInput, windows_core::IUnknown);
impl IGameInput {
    pub unsafe fn GetCurrentTimestamp(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTimestamp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCurrentReading<P1>(&self, inputkind: GameInputKind, device: P1) -> windows_core::Result<IGameInputReading>
    where
        P1: windows_core::Param<IGameInputDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentReading)(windows_core::Interface::as_raw(self), inputkind, device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextReading<P0, P2>(&self, referencereading: P0, inputkind: GameInputKind, device: P2) -> windows_core::Result<IGameInputReading>
    where
        P0: windows_core::Param<IGameInputReading>,
        P2: windows_core::Param<IGameInputDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextReading)(windows_core::Interface::as_raw(self), referencereading.param().abi(), inputkind, device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviousReading<P0, P2>(&self, referencereading: P0, inputkind: GameInputKind, device: P2) -> windows_core::Result<IGameInputReading>
    where
        P0: windows_core::Param<IGameInputReading>,
        P2: windows_core::Param<IGameInputDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousReading)(windows_core::Interface::as_raw(self), referencereading.param().abi(), inputkind, device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTemporalReading<P1>(&self, timestamp: u64, device: P1) -> windows_core::Result<IGameInputReading>
    where
        P1: windows_core::Param<IGameInputDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemporalReading)(windows_core::Interface::as_raw(self), timestamp, device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterReadingCallback<P0>(&self, device: P0, inputkind: GameInputKind, analogthreshold: f32, context: Option<*const core::ffi::c_void>, callbackfunc: GameInputReadingCallback, callbacktoken: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGameInputDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterReadingCallback)(windows_core::Interface::as_raw(self), device.param().abi(), inputkind, analogthreshold, context.unwrap_or(core::mem::zeroed()) as _, callbackfunc, callbacktoken.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn RegisterDeviceCallback<P0>(&self, device: P0, inputkind: GameInputKind, statusfilter: GameInputDeviceStatus, enumerationkind: GameInputEnumerationKind, context: Option<*const core::ffi::c_void>, callbackfunc: GameInputDeviceCallback, callbacktoken: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGameInputDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterDeviceCallback)(windows_core::Interface::as_raw(self), device.param().abi(), inputkind, statusfilter, enumerationkind, context.unwrap_or(core::mem::zeroed()) as _, callbackfunc, callbacktoken.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn RegisterSystemButtonCallback<P0>(&self, device: P0, buttonfilter: GameInputSystemButtons, context: Option<*const core::ffi::c_void>, callbackfunc: GameInputSystemButtonCallback, callbacktoken: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGameInputDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterSystemButtonCallback)(windows_core::Interface::as_raw(self), device.param().abi(), buttonfilter, context.unwrap_or(core::mem::zeroed()) as _, callbackfunc, callbacktoken.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn RegisterKeyboardLayoutCallback<P0>(&self, device: P0, context: Option<*const core::ffi::c_void>, callbackfunc: GameInputKeyboardLayoutCallback, callbacktoken: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGameInputDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterKeyboardLayoutCallback)(windows_core::Interface::as_raw(self), device.param().abi(), context.unwrap_or(core::mem::zeroed()) as _, callbackfunc, callbacktoken.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn StopCallback(&self, callbacktoken: u64) {
        unsafe { (windows_core::Interface::vtable(self).StopCallback)(windows_core::Interface::as_raw(self), callbacktoken) }
    }
    pub unsafe fn UnregisterCallback(&self, callbacktoken: u64, timeoutinmicroseconds: u64) -> bool {
        unsafe { (windows_core::Interface::vtable(self).UnregisterCallback)(windows_core::Interface::as_raw(self), callbacktoken, timeoutinmicroseconds) }
    }
    pub unsafe fn CreateDispatcher(&self) -> windows_core::Result<IGameInputDispatcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAggregateDevice(&self, inputkind: GameInputKind) -> windows_core::Result<IGameInputDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAggregateDevice)(windows_core::Interface::as_raw(self), inputkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindDeviceFromId(&self, value: *const super::super::super::Foundation::APP_LOCAL_DEVICE_ID) -> windows_core::Result<IGameInputDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindDeviceFromId)(windows_core::Interface::as_raw(self), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindDeviceFromObject<P0>(&self, value: P0) -> windows_core::Result<IGameInputDevice>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindDeviceFromObject)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindDeviceFromPlatformHandle(&self, value: super::super::super::Foundation::HANDLE) -> windows_core::Result<IGameInputDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindDeviceFromPlatformHandle)(windows_core::Interface::as_raw(self), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindDeviceFromPlatformString<P0>(&self, value: P0) -> windows_core::Result<IGameInputDevice>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindDeviceFromPlatformString)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnableOemDeviceSupport(&self, vendorid: u16, productid: u16, interfacenumber: u8, collectionnumber: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableOemDeviceSupport)(windows_core::Interface::as_raw(self), vendorid, productid, interfacenumber, collectionnumber).ok() }
    }
    pub unsafe fn SetFocusPolicy(&self, policy: GameInputFocusPolicy) {
        unsafe { (windows_core::Interface::vtable(self).SetFocusPolicy)(windows_core::Interface::as_raw(self), policy) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameInput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, GameInputKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GameInputKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GameInputKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTemporalReading: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterReadingCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GameInputKind, f32, *const core::ffi::c_void, GameInputReadingCallback, *mut u64) -> windows_core::HRESULT,
    pub RegisterDeviceCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GameInputKind, GameInputDeviceStatus, GameInputEnumerationKind, *const core::ffi::c_void, GameInputDeviceCallback, *mut u64) -> windows_core::HRESULT,
    pub RegisterSystemButtonCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GameInputSystemButtons, *const core::ffi::c_void, GameInputSystemButtonCallback, *mut u64) -> windows_core::HRESULT,
    pub RegisterKeyboardLayoutCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, GameInputKeyboardLayoutCallback, *mut u64) -> windows_core::HRESULT,
    pub StopCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub UnregisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> bool,
    pub CreateDispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAggregateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, GameInputKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindDeviceFromId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::APP_LOCAL_DEVICE_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindDeviceFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindDeviceFromPlatformHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindDeviceFromPlatformString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableOemDeviceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, u8, u8) -> windows_core::HRESULT,
    pub SetFocusPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, GameInputFocusPolicy),
}
pub trait IGameInput_Impl: windows_core::IUnknownImpl {
    fn GetCurrentTimestamp(&self) -> u64;
    fn GetCurrentReading(&self, inputkind: GameInputKind, device: windows_core::Ref<IGameInputDevice>) -> windows_core::Result<IGameInputReading>;
    fn GetNextReading(&self, referencereading: windows_core::Ref<IGameInputReading>, inputkind: GameInputKind, device: windows_core::Ref<IGameInputDevice>) -> windows_core::Result<IGameInputReading>;
    fn GetPreviousReading(&self, referencereading: windows_core::Ref<IGameInputReading>, inputkind: GameInputKind, device: windows_core::Ref<IGameInputDevice>) -> windows_core::Result<IGameInputReading>;
    fn GetTemporalReading(&self, timestamp: u64, device: windows_core::Ref<IGameInputDevice>) -> windows_core::Result<IGameInputReading>;
    fn RegisterReadingCallback(&self, device: windows_core::Ref<IGameInputDevice>, inputkind: GameInputKind, analogthreshold: f32, context: *const core::ffi::c_void, callbackfunc: GameInputReadingCallback, callbacktoken: *mut u64) -> windows_core::Result<()>;
    fn RegisterDeviceCallback(&self, device: windows_core::Ref<IGameInputDevice>, inputkind: GameInputKind, statusfilter: GameInputDeviceStatus, enumerationkind: GameInputEnumerationKind, context: *const core::ffi::c_void, callbackfunc: GameInputDeviceCallback, callbacktoken: *mut u64) -> windows_core::Result<()>;
    fn RegisterSystemButtonCallback(&self, device: windows_core::Ref<IGameInputDevice>, buttonfilter: GameInputSystemButtons, context: *const core::ffi::c_void, callbackfunc: GameInputSystemButtonCallback, callbacktoken: *mut u64) -> windows_core::Result<()>;
    fn RegisterKeyboardLayoutCallback(&self, device: windows_core::Ref<IGameInputDevice>, context: *const core::ffi::c_void, callbackfunc: GameInputKeyboardLayoutCallback, callbacktoken: *mut u64) -> windows_core::Result<()>;
    fn StopCallback(&self, callbacktoken: u64);
    fn UnregisterCallback(&self, callbacktoken: u64, timeoutinmicroseconds: u64) -> bool;
    fn CreateDispatcher(&self) -> windows_core::Result<IGameInputDispatcher>;
    fn CreateAggregateDevice(&self, inputkind: GameInputKind) -> windows_core::Result<IGameInputDevice>;
    fn FindDeviceFromId(&self, value: *const super::super::super::Foundation::APP_LOCAL_DEVICE_ID) -> windows_core::Result<IGameInputDevice>;
    fn FindDeviceFromObject(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IGameInputDevice>;
    fn FindDeviceFromPlatformHandle(&self, value: super::super::super::Foundation::HANDLE) -> windows_core::Result<IGameInputDevice>;
    fn FindDeviceFromPlatformString(&self, value: &windows_core::PCWSTR) -> windows_core::Result<IGameInputDevice>;
    fn EnableOemDeviceSupport(&self, vendorid: u16, productid: u16, interfacenumber: u8, collectionnumber: u8) -> windows_core::Result<()>;
    fn SetFocusPolicy(&self, policy: GameInputFocusPolicy);
}
impl IGameInput_Vtbl {
    pub const fn new<Identity: IGameInput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentTimestamp<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::GetCurrentTimestamp(this)
            }
        }
        unsafe extern "system" fn GetCurrentReading<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputkind: GameInputKind, device: *mut core::ffi::c_void, reading: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::GetCurrentReading(this, core::mem::transmute_copy(&inputkind), core::mem::transmute_copy(&device)) {
                    Ok(ok__) => {
                        reading.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextReading<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencereading: *mut core::ffi::c_void, inputkind: GameInputKind, device: *mut core::ffi::c_void, reading: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::GetNextReading(this, core::mem::transmute_copy(&referencereading), core::mem::transmute_copy(&inputkind), core::mem::transmute_copy(&device)) {
                    Ok(ok__) => {
                        reading.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviousReading<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencereading: *mut core::ffi::c_void, inputkind: GameInputKind, device: *mut core::ffi::c_void, reading: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::GetPreviousReading(this, core::mem::transmute_copy(&referencereading), core::mem::transmute_copy(&inputkind), core::mem::transmute_copy(&device)) {
                    Ok(ok__) => {
                        reading.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTemporalReading<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, device: *mut core::ffi::c_void, reading: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::GetTemporalReading(this, core::mem::transmute_copy(&timestamp), core::mem::transmute_copy(&device)) {
                    Ok(ok__) => {
                        reading.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterReadingCallback<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void, inputkind: GameInputKind, analogthreshold: f32, context: *const core::ffi::c_void, callbackfunc: GameInputReadingCallback, callbacktoken: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::RegisterReadingCallback(this, core::mem::transmute_copy(&device), core::mem::transmute_copy(&inputkind), core::mem::transmute_copy(&analogthreshold), core::mem::transmute_copy(&context), core::mem::transmute_copy(&callbackfunc), core::mem::transmute_copy(&callbacktoken)).into()
            }
        }
        unsafe extern "system" fn RegisterDeviceCallback<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void, inputkind: GameInputKind, statusfilter: GameInputDeviceStatus, enumerationkind: GameInputEnumerationKind, context: *const core::ffi::c_void, callbackfunc: GameInputDeviceCallback, callbacktoken: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::RegisterDeviceCallback(this, core::mem::transmute_copy(&device), core::mem::transmute_copy(&inputkind), core::mem::transmute_copy(&statusfilter), core::mem::transmute_copy(&enumerationkind), core::mem::transmute_copy(&context), core::mem::transmute_copy(&callbackfunc), core::mem::transmute_copy(&callbacktoken)).into()
            }
        }
        unsafe extern "system" fn RegisterSystemButtonCallback<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void, buttonfilter: GameInputSystemButtons, context: *const core::ffi::c_void, callbackfunc: GameInputSystemButtonCallback, callbacktoken: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::RegisterSystemButtonCallback(this, core::mem::transmute_copy(&device), core::mem::transmute_copy(&buttonfilter), core::mem::transmute_copy(&context), core::mem::transmute_copy(&callbackfunc), core::mem::transmute_copy(&callbacktoken)).into()
            }
        }
        unsafe extern "system" fn RegisterKeyboardLayoutCallback<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void, context: *const core::ffi::c_void, callbackfunc: GameInputKeyboardLayoutCallback, callbacktoken: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::RegisterKeyboardLayoutCallback(this, core::mem::transmute_copy(&device), core::mem::transmute_copy(&context), core::mem::transmute_copy(&callbackfunc), core::mem::transmute_copy(&callbacktoken)).into()
            }
        }
        unsafe extern "system" fn StopCallback<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callbacktoken: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::StopCallback(this, core::mem::transmute_copy(&callbacktoken))
            }
        }
        unsafe extern "system" fn UnregisterCallback<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callbacktoken: u64, timeoutinmicroseconds: u64) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::UnregisterCallback(this, core::mem::transmute_copy(&callbacktoken), core::mem::transmute_copy(&timeoutinmicroseconds))
            }
        }
        unsafe extern "system" fn CreateDispatcher<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::CreateDispatcher(this) {
                    Ok(ok__) => {
                        dispatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAggregateDevice<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputkind: GameInputKind, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::CreateAggregateDevice(this, core::mem::transmute_copy(&inputkind)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindDeviceFromId<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const super::super::super::Foundation::APP_LOCAL_DEVICE_ID, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::FindDeviceFromId(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindDeviceFromObject<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::FindDeviceFromObject(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindDeviceFromPlatformHandle<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::HANDLE, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::FindDeviceFromPlatformHandle(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindDeviceFromPlatformString<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInput_Impl::FindDeviceFromPlatformString(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableOemDeviceSupport<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendorid: u16, productid: u16, interfacenumber: u8, collectionnumber: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::EnableOemDeviceSupport(this, core::mem::transmute_copy(&vendorid), core::mem::transmute_copy(&productid), core::mem::transmute_copy(&interfacenumber), core::mem::transmute_copy(&collectionnumber)).into()
            }
        }
        unsafe extern "system" fn SetFocusPolicy<Identity: IGameInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: GameInputFocusPolicy) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInput_Impl::SetFocusPolicy(this, core::mem::transmute_copy(&policy))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTimestamp: GetCurrentTimestamp::<Identity, OFFSET>,
            GetCurrentReading: GetCurrentReading::<Identity, OFFSET>,
            GetNextReading: GetNextReading::<Identity, OFFSET>,
            GetPreviousReading: GetPreviousReading::<Identity, OFFSET>,
            GetTemporalReading: GetTemporalReading::<Identity, OFFSET>,
            RegisterReadingCallback: RegisterReadingCallback::<Identity, OFFSET>,
            RegisterDeviceCallback: RegisterDeviceCallback::<Identity, OFFSET>,
            RegisterSystemButtonCallback: RegisterSystemButtonCallback::<Identity, OFFSET>,
            RegisterKeyboardLayoutCallback: RegisterKeyboardLayoutCallback::<Identity, OFFSET>,
            StopCallback: StopCallback::<Identity, OFFSET>,
            UnregisterCallback: UnregisterCallback::<Identity, OFFSET>,
            CreateDispatcher: CreateDispatcher::<Identity, OFFSET>,
            CreateAggregateDevice: CreateAggregateDevice::<Identity, OFFSET>,
            FindDeviceFromId: FindDeviceFromId::<Identity, OFFSET>,
            FindDeviceFromObject: FindDeviceFromObject::<Identity, OFFSET>,
            FindDeviceFromPlatformHandle: FindDeviceFromPlatformHandle::<Identity, OFFSET>,
            FindDeviceFromPlatformString: FindDeviceFromPlatformString::<Identity, OFFSET>,
            EnableOemDeviceSupport: EnableOemDeviceSupport::<Identity, OFFSET>,
            SetFocusPolicy: SetFocusPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameInput as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameInput {}
windows_core::imp::define_interface!(IGameInputDevice, IGameInputDevice_Vtbl, 0x31dd86fb_4c1b_408a_868f_439b3cd47125);
windows_core::imp::interface_hierarchy!(IGameInputDevice, windows_core::IUnknown);
impl IGameInputDevice {
    pub unsafe fn GetDeviceInfo(&self) -> *mut GameInputDeviceInfo {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDeviceStatus(&self) -> GameInputDeviceStatus {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBatteryState(&self, state: *mut GameInputBatteryState) {
        unsafe { (windows_core::Interface::vtable(self).GetBatteryState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn CreateForceFeedbackEffect(&self, motorindex: u32, params: *const GameInputForceFeedbackParams) -> windows_core::Result<IGameInputForceFeedbackEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateForceFeedbackEffect)(windows_core::Interface::as_raw(self), motorindex, params, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsForceFeedbackMotorPoweredOn(&self, motorindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsForceFeedbackMotorPoweredOn)(windows_core::Interface::as_raw(self), motorindex) }
    }
    pub unsafe fn SetForceFeedbackMotorGain(&self, motorindex: u32, mastergain: f32) {
        unsafe { (windows_core::Interface::vtable(self).SetForceFeedbackMotorGain)(windows_core::Interface::as_raw(self), motorindex, mastergain) }
    }
    pub unsafe fn SetHapticMotorState(&self, motorindex: u32, params: Option<*const GameInputHapticFeedbackParams>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHapticMotorState)(windows_core::Interface::as_raw(self), motorindex, params.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetRumbleState(&self, params: Option<*const GameInputRumbleParams>) {
        unsafe { (windows_core::Interface::vtable(self).SetRumbleState)(windows_core::Interface::as_raw(self), params.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetInputSynchronizationState(&self, enabled: u8) {
        unsafe { (windows_core::Interface::vtable(self).SetInputSynchronizationState)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn SendInputSynchronizationHint(&self) {
        unsafe { (windows_core::Interface::vtable(self).SendInputSynchronizationHint)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PowerOff(&self) {
        unsafe { (windows_core::Interface::vtable(self).PowerOff)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateRawDeviceReport(&self, reportid: u32, reportkind: GameInputRawDeviceReportKind) -> windows_core::Result<IGameInputRawDeviceReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRawDeviceReport)(windows_core::Interface::as_raw(self), reportid, reportkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRawDeviceFeature(&self, reportid: u32) -> windows_core::Result<IGameInputRawDeviceReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRawDeviceFeature)(windows_core::Interface::as_raw(self), reportid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRawDeviceFeature<P0>(&self, report: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGameInputRawDeviceReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRawDeviceFeature)(windows_core::Interface::as_raw(self), report.param().abi()).ok() }
    }
    pub unsafe fn SendRawDeviceOutput<P0>(&self, report: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGameInputRawDeviceReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendRawDeviceOutput)(windows_core::Interface::as_raw(self), report.param().abi()).ok() }
    }
    pub unsafe fn SendRawDeviceOutputWithResponse<P0>(&self, requestreport: P0) -> windows_core::Result<IGameInputRawDeviceReport>
    where
        P0: windows_core::Param<IGameInputRawDeviceReport>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendRawDeviceOutputWithResponse)(windows_core::Interface::as_raw(self), requestreport.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecuteRawDeviceIoControl(&self, controlcode: u32, inputbuffersize: usize, inputbuffer: Option<*const core::ffi::c_void>, outputbuffersize: usize, outputbuffer: Option<*mut core::ffi::c_void>, outputsize: Option<*mut usize>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecuteRawDeviceIoControl)(windows_core::Interface::as_raw(self), controlcode, inputbuffersize, inputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbuffersize, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn AcquireExclusiveRawDeviceAccess(&self, timeoutinmicroseconds: u64) -> bool {
        unsafe { (windows_core::Interface::vtable(self).AcquireExclusiveRawDeviceAccess)(windows_core::Interface::as_raw(self), timeoutinmicroseconds) }
    }
    pub unsafe fn ReleaseExclusiveRawDeviceAccess(&self) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseExclusiveRawDeviceAccess)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameInputDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut GameInputDeviceInfo,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> GameInputDeviceStatus,
    pub GetBatteryState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputBatteryState),
    pub CreateForceFeedbackEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const GameInputForceFeedbackParams, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsForceFeedbackMotorPoweredOn: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub SetForceFeedbackMotorGain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32),
    pub SetHapticMotorState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const GameInputHapticFeedbackParams) -> windows_core::HRESULT,
    pub SetRumbleState: unsafe extern "system" fn(*mut core::ffi::c_void, *const GameInputRumbleParams),
    pub SetInputSynchronizationState: unsafe extern "system" fn(*mut core::ffi::c_void, u8),
    pub SendInputSynchronizationHint: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PowerOff: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub CreateRawDeviceReport: unsafe extern "system" fn(*mut core::ffi::c_void, u32, GameInputRawDeviceReportKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRawDeviceFeature: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRawDeviceFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendRawDeviceOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendRawDeviceOutputWithResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteRawDeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub AcquireExclusiveRawDeviceAccess: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> bool,
    pub ReleaseExclusiveRawDeviceAccess: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IGameInputDevice_Impl: windows_core::IUnknownImpl {
    fn GetDeviceInfo(&self) -> *mut GameInputDeviceInfo;
    fn GetDeviceStatus(&self) -> GameInputDeviceStatus;
    fn GetBatteryState(&self, state: *mut GameInputBatteryState);
    fn CreateForceFeedbackEffect(&self, motorindex: u32, params: *const GameInputForceFeedbackParams) -> windows_core::Result<IGameInputForceFeedbackEffect>;
    fn IsForceFeedbackMotorPoweredOn(&self, motorindex: u32) -> bool;
    fn SetForceFeedbackMotorGain(&self, motorindex: u32, mastergain: f32);
    fn SetHapticMotorState(&self, motorindex: u32, params: *const GameInputHapticFeedbackParams) -> windows_core::Result<()>;
    fn SetRumbleState(&self, params: *const GameInputRumbleParams);
    fn SetInputSynchronizationState(&self, enabled: u8);
    fn SendInputSynchronizationHint(&self);
    fn PowerOff(&self);
    fn CreateRawDeviceReport(&self, reportid: u32, reportkind: GameInputRawDeviceReportKind) -> windows_core::Result<IGameInputRawDeviceReport>;
    fn GetRawDeviceFeature(&self, reportid: u32) -> windows_core::Result<IGameInputRawDeviceReport>;
    fn SetRawDeviceFeature(&self, report: windows_core::Ref<IGameInputRawDeviceReport>) -> windows_core::Result<()>;
    fn SendRawDeviceOutput(&self, report: windows_core::Ref<IGameInputRawDeviceReport>) -> windows_core::Result<()>;
    fn SendRawDeviceOutputWithResponse(&self, requestreport: windows_core::Ref<IGameInputRawDeviceReport>) -> windows_core::Result<IGameInputRawDeviceReport>;
    fn ExecuteRawDeviceIoControl(&self, controlcode: u32, inputbuffersize: usize, inputbuffer: *const core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void, outputsize: *mut usize) -> windows_core::Result<()>;
    fn AcquireExclusiveRawDeviceAccess(&self, timeoutinmicroseconds: u64) -> bool;
    fn ReleaseExclusiveRawDeviceAccess(&self);
}
impl IGameInputDevice_Vtbl {
    pub const fn new<Identity: IGameInputDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceInfo<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut GameInputDeviceInfo {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::GetDeviceInfo(this)
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> GameInputDeviceStatus {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::GetDeviceStatus(this)
            }
        }
        unsafe extern "system" fn GetBatteryState<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputBatteryState) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::GetBatteryState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn CreateForceFeedbackEffect<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motorindex: u32, params: *const GameInputForceFeedbackParams, effect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInputDevice_Impl::CreateForceFeedbackEffect(this, core::mem::transmute_copy(&motorindex), core::mem::transmute_copy(&params)) {
                    Ok(ok__) => {
                        effect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsForceFeedbackMotorPoweredOn<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motorindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::IsForceFeedbackMotorPoweredOn(this, core::mem::transmute_copy(&motorindex))
            }
        }
        unsafe extern "system" fn SetForceFeedbackMotorGain<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motorindex: u32, mastergain: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SetForceFeedbackMotorGain(this, core::mem::transmute_copy(&motorindex), core::mem::transmute_copy(&mastergain))
            }
        }
        unsafe extern "system" fn SetHapticMotorState<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motorindex: u32, params: *const GameInputHapticFeedbackParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SetHapticMotorState(this, core::mem::transmute_copy(&motorindex), core::mem::transmute_copy(&params)).into()
            }
        }
        unsafe extern "system" fn SetRumbleState<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: *const GameInputRumbleParams) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SetRumbleState(this, core::mem::transmute_copy(&params))
            }
        }
        unsafe extern "system" fn SetInputSynchronizationState<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: u8) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SetInputSynchronizationState(this, core::mem::transmute_copy(&enabled))
            }
        }
        unsafe extern "system" fn SendInputSynchronizationHint<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SendInputSynchronizationHint(this)
            }
        }
        unsafe extern "system" fn PowerOff<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::PowerOff(this)
            }
        }
        unsafe extern "system" fn CreateRawDeviceReport<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportid: u32, reportkind: GameInputRawDeviceReportKind, report: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInputDevice_Impl::CreateRawDeviceReport(this, core::mem::transmute_copy(&reportid), core::mem::transmute_copy(&reportkind)) {
                    Ok(ok__) => {
                        report.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRawDeviceFeature<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportid: u32, report: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInputDevice_Impl::GetRawDeviceFeature(this, core::mem::transmute_copy(&reportid)) {
                    Ok(ok__) => {
                        report.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRawDeviceFeature<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, report: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SetRawDeviceFeature(this, core::mem::transmute_copy(&report)).into()
            }
        }
        unsafe extern "system" fn SendRawDeviceOutput<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, report: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::SendRawDeviceOutput(this, core::mem::transmute_copy(&report)).into()
            }
        }
        unsafe extern "system" fn SendRawDeviceOutputWithResponse<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestreport: *mut core::ffi::c_void, responsereport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInputDevice_Impl::SendRawDeviceOutputWithResponse(this, core::mem::transmute_copy(&requestreport)) {
                    Ok(ok__) => {
                        responsereport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecuteRawDeviceIoControl<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlcode: u32, inputbuffersize: usize, inputbuffer: *const core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void, outputsize: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::ExecuteRawDeviceIoControl(this, core::mem::transmute_copy(&controlcode), core::mem::transmute_copy(&inputbuffersize), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&outputbuffersize), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&outputsize)).into()
            }
        }
        unsafe extern "system" fn AcquireExclusiveRawDeviceAccess<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinmicroseconds: u64) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::AcquireExclusiveRawDeviceAccess(this, core::mem::transmute_copy(&timeoutinmicroseconds))
            }
        }
        unsafe extern "system" fn ReleaseExclusiveRawDeviceAccess<Identity: IGameInputDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDevice_Impl::ReleaseExclusiveRawDeviceAccess(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            GetBatteryState: GetBatteryState::<Identity, OFFSET>,
            CreateForceFeedbackEffect: CreateForceFeedbackEffect::<Identity, OFFSET>,
            IsForceFeedbackMotorPoweredOn: IsForceFeedbackMotorPoweredOn::<Identity, OFFSET>,
            SetForceFeedbackMotorGain: SetForceFeedbackMotorGain::<Identity, OFFSET>,
            SetHapticMotorState: SetHapticMotorState::<Identity, OFFSET>,
            SetRumbleState: SetRumbleState::<Identity, OFFSET>,
            SetInputSynchronizationState: SetInputSynchronizationState::<Identity, OFFSET>,
            SendInputSynchronizationHint: SendInputSynchronizationHint::<Identity, OFFSET>,
            PowerOff: PowerOff::<Identity, OFFSET>,
            CreateRawDeviceReport: CreateRawDeviceReport::<Identity, OFFSET>,
            GetRawDeviceFeature: GetRawDeviceFeature::<Identity, OFFSET>,
            SetRawDeviceFeature: SetRawDeviceFeature::<Identity, OFFSET>,
            SendRawDeviceOutput: SendRawDeviceOutput::<Identity, OFFSET>,
            SendRawDeviceOutputWithResponse: SendRawDeviceOutputWithResponse::<Identity, OFFSET>,
            ExecuteRawDeviceIoControl: ExecuteRawDeviceIoControl::<Identity, OFFSET>,
            AcquireExclusiveRawDeviceAccess: AcquireExclusiveRawDeviceAccess::<Identity, OFFSET>,
            ReleaseExclusiveRawDeviceAccess: ReleaseExclusiveRawDeviceAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameInputDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameInputDevice {}
windows_core::imp::define_interface!(IGameInputDispatcher, IGameInputDispatcher_Vtbl, 0x415eed2e_98cb_42c2_8f28_b94601074e31);
windows_core::imp::interface_hierarchy!(IGameInputDispatcher, windows_core::IUnknown);
impl IGameInputDispatcher {
    pub unsafe fn Dispatch(&self, quotainmicroseconds: u64) -> bool {
        unsafe { (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), quotainmicroseconds) }
    }
    pub unsafe fn OpenWaitHandle(&self) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenWaitHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameInputDispatcher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> bool,
    pub OpenWaitHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
pub trait IGameInputDispatcher_Impl: windows_core::IUnknownImpl {
    fn Dispatch(&self, quotainmicroseconds: u64) -> bool;
    fn OpenWaitHandle(&self) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
}
impl IGameInputDispatcher_Vtbl {
    pub const fn new<Identity: IGameInputDispatcher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Dispatch<Identity: IGameInputDispatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotainmicroseconds: u64) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputDispatcher_Impl::Dispatch(this, core::mem::transmute_copy(&quotainmicroseconds))
            }
        }
        unsafe extern "system" fn OpenWaitHandle<Identity: IGameInputDispatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waithandle: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameInputDispatcher_Impl::OpenWaitHandle(this) {
                    Ok(ok__) => {
                        waithandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Dispatch: Dispatch::<Identity, OFFSET>,
            OpenWaitHandle: OpenWaitHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameInputDispatcher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameInputDispatcher {}
windows_core::imp::define_interface!(IGameInputForceFeedbackEffect, IGameInputForceFeedbackEffect_Vtbl, 0x51bda05e_f742_45d9_b085_9444ae48381d);
windows_core::imp::interface_hierarchy!(IGameInputForceFeedbackEffect, windows_core::IUnknown);
impl IGameInputForceFeedbackEffect {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IGameInputDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetMotorIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetMotorIndex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetGain(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetGain)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetGain(&self, gain: f32) {
        unsafe { (windows_core::Interface::vtable(self).SetGain)(windows_core::Interface::as_raw(self), gain) }
    }
    pub unsafe fn GetParams(&self, params: *mut GameInputForceFeedbackParams) {
        unsafe { (windows_core::Interface::vtable(self).GetParams)(windows_core::Interface::as_raw(self), params as _) }
    }
    pub unsafe fn SetParams(&self, params: *const GameInputForceFeedbackParams) -> bool {
        unsafe { (windows_core::Interface::vtable(self).SetParams)(windows_core::Interface::as_raw(self), params) }
    }
    pub unsafe fn GetState(&self) -> GameInputFeedbackEffectState {
        unsafe { (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetState(&self, state: GameInputFeedbackEffectState) {
        unsafe { (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameInputForceFeedbackEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetMotorIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetGain: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    pub GetParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputForceFeedbackParams),
    pub SetParams: unsafe extern "system" fn(*mut core::ffi::c_void, *const GameInputForceFeedbackParams) -> bool,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void) -> GameInputFeedbackEffectState,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, GameInputFeedbackEffectState),
}
pub trait IGameInputForceFeedbackEffect_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, device: windows_core::OutRef<IGameInputDevice>);
    fn GetMotorIndex(&self) -> u32;
    fn GetGain(&self) -> f32;
    fn SetGain(&self, gain: f32);
    fn GetParams(&self, params: *mut GameInputForceFeedbackParams);
    fn SetParams(&self, params: *const GameInputForceFeedbackParams) -> bool;
    fn GetState(&self) -> GameInputFeedbackEffectState;
    fn SetState(&self, state: GameInputFeedbackEffectState);
}
impl IGameInputForceFeedbackEffect_Vtbl {
    pub const fn new<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::GetDevice(this, core::mem::transmute_copy(&device))
            }
        }
        unsafe extern "system" fn GetMotorIndex<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::GetMotorIndex(this)
            }
        }
        unsafe extern "system" fn GetGain<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::GetGain(this)
            }
        }
        unsafe extern "system" fn SetGain<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gain: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::SetGain(this, core::mem::transmute_copy(&gain))
            }
        }
        unsafe extern "system" fn GetParams<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: *mut GameInputForceFeedbackParams) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::GetParams(this, core::mem::transmute_copy(&params))
            }
        }
        unsafe extern "system" fn SetParams<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: *const GameInputForceFeedbackParams) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::SetParams(this, core::mem::transmute_copy(&params))
            }
        }
        unsafe extern "system" fn GetState<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> GameInputFeedbackEffectState {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::GetState(this)
            }
        }
        unsafe extern "system" fn SetState<Identity: IGameInputForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: GameInputFeedbackEffectState) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputForceFeedbackEffect_Impl::SetState(this, core::mem::transmute_copy(&state))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetMotorIndex: GetMotorIndex::<Identity, OFFSET>,
            GetGain: GetGain::<Identity, OFFSET>,
            SetGain: SetGain::<Identity, OFFSET>,
            GetParams: GetParams::<Identity, OFFSET>,
            SetParams: SetParams::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameInputForceFeedbackEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameInputForceFeedbackEffect {}
windows_core::imp::define_interface!(IGameInputRawDeviceReport, IGameInputRawDeviceReport_Vtbl, 0x61f08cf1_1ffc_40ca_a2b8_e1ab8bc5b6dc);
windows_core::imp::interface_hierarchy!(IGameInputRawDeviceReport, windows_core::IUnknown);
impl IGameInputRawDeviceReport {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IGameInputDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetReportInfo(&self) -> *mut GameInputRawDeviceReportInfo {
        unsafe { (windows_core::Interface::vtable(self).GetReportInfo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRawDataSize(&self) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetRawDataSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRawData(&self, buffer: &mut [u8]) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetRawData)(windows_core::Interface::as_raw(self), buffer.len().try_into().unwrap(), core::mem::transmute(buffer.as_ptr())) }
    }
    pub unsafe fn SetRawData(&self, buffer: &[u8]) -> bool {
        unsafe { (windows_core::Interface::vtable(self).SetRawData)(windows_core::Interface::as_raw(self), buffer.len().try_into().unwrap(), core::mem::transmute(buffer.as_ptr())) }
    }
    pub unsafe fn GetItemValue(&self, itemindex: u32, value: *mut i64) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetItemValue)(windows_core::Interface::as_raw(self), itemindex, value as _) }
    }
    pub unsafe fn SetItemValue(&self, itemindex: u32, value: i64) -> bool {
        unsafe { (windows_core::Interface::vtable(self).SetItemValue)(windows_core::Interface::as_raw(self), itemindex, value) }
    }
    pub unsafe fn ResetItemValue(&self, itemindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).ResetItemValue)(windows_core::Interface::as_raw(self), itemindex) }
    }
    pub unsafe fn ResetAllItems(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).ResetAllItems)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameInputRawDeviceReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetReportInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut GameInputRawDeviceReportInfo,
    pub GetRawDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
    pub GetRawData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut core::ffi::c_void) -> usize,
    pub SetRawData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const core::ffi::c_void) -> bool,
    pub GetItemValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i64) -> bool,
    pub SetItemValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i64) -> bool,
    pub ResetItemValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub ResetAllItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
}
pub trait IGameInputRawDeviceReport_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, device: windows_core::OutRef<IGameInputDevice>);
    fn GetReportInfo(&self) -> *mut GameInputRawDeviceReportInfo;
    fn GetRawDataSize(&self) -> usize;
    fn GetRawData(&self, buffersize: usize, buffer: *mut core::ffi::c_void) -> usize;
    fn SetRawData(&self, buffersize: usize, buffer: *const core::ffi::c_void) -> bool;
    fn GetItemValue(&self, itemindex: u32, value: *mut i64) -> bool;
    fn SetItemValue(&self, itemindex: u32, value: i64) -> bool;
    fn ResetItemValue(&self, itemindex: u32) -> bool;
    fn ResetAllItems(&self) -> bool;
}
impl IGameInputRawDeviceReport_Vtbl {
    pub const fn new<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::GetDevice(this, core::mem::transmute_copy(&device))
            }
        }
        unsafe extern "system" fn GetReportInfo<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut GameInputRawDeviceReportInfo {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::GetReportInfo(this)
            }
        }
        unsafe extern "system" fn GetRawDataSize<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::GetRawDataSize(this)
            }
        }
        unsafe extern "system" fn GetRawData<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: usize, buffer: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::GetRawData(this, core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&buffer))
            }
        }
        unsafe extern "system" fn SetRawData<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: usize, buffer: *const core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::SetRawData(this, core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&buffer))
            }
        }
        unsafe extern "system" fn GetItemValue<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemindex: u32, value: *mut i64) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::GetItemValue(this, core::mem::transmute_copy(&itemindex), core::mem::transmute_copy(&value))
            }
        }
        unsafe extern "system" fn SetItemValue<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemindex: u32, value: i64) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::SetItemValue(this, core::mem::transmute_copy(&itemindex), core::mem::transmute_copy(&value))
            }
        }
        unsafe extern "system" fn ResetItemValue<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::ResetItemValue(this, core::mem::transmute_copy(&itemindex))
            }
        }
        unsafe extern "system" fn ResetAllItems<Identity: IGameInputRawDeviceReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputRawDeviceReport_Impl::ResetAllItems(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetReportInfo: GetReportInfo::<Identity, OFFSET>,
            GetRawDataSize: GetRawDataSize::<Identity, OFFSET>,
            GetRawData: GetRawData::<Identity, OFFSET>,
            SetRawData: SetRawData::<Identity, OFFSET>,
            GetItemValue: GetItemValue::<Identity, OFFSET>,
            SetItemValue: SetItemValue::<Identity, OFFSET>,
            ResetItemValue: ResetItemValue::<Identity, OFFSET>,
            ResetAllItems: ResetAllItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameInputRawDeviceReport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameInputRawDeviceReport {}
windows_core::imp::define_interface!(IGameInputReading, IGameInputReading_Vtbl, 0x2156947a_e1fa_4de0_a30b_d812931dbd8d);
windows_core::imp::interface_hierarchy!(IGameInputReading, windows_core::IUnknown);
impl IGameInputReading {
    pub unsafe fn GetInputKind(&self) -> GameInputKind {
        unsafe { (windows_core::Interface::vtable(self).GetInputKind)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSequenceNumber(&self, inputkind: GameInputKind) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetSequenceNumber)(windows_core::Interface::as_raw(self), inputkind) }
    }
    pub unsafe fn GetTimestamp(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IGameInputDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetRawReport(&self, report: *mut Option<IGameInputRawDeviceReport>) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetRawReport)(windows_core::Interface::as_raw(self), core::mem::transmute(report)) }
    }
    pub unsafe fn GetControllerAxisCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControllerAxisCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetControllerAxisState(&self, statearray: &mut [f32]) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControllerAxisState)(windows_core::Interface::as_raw(self), statearray.len().try_into().unwrap(), core::mem::transmute(statearray.as_ptr())) }
    }
    pub unsafe fn GetControllerButtonCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControllerButtonCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetControllerButtonState(&self, statearray: &mut [bool]) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControllerButtonState)(windows_core::Interface::as_raw(self), statearray.len().try_into().unwrap(), core::mem::transmute(statearray.as_ptr())) }
    }
    pub unsafe fn GetControllerSwitchCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControllerSwitchCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetControllerSwitchState(&self, statearray: &mut [GameInputSwitchPosition]) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControllerSwitchState)(windows_core::Interface::as_raw(self), statearray.len().try_into().unwrap(), core::mem::transmute(statearray.as_ptr())) }
    }
    pub unsafe fn GetKeyCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetKeyCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetKeyState(&self, statearray: &mut [GameInputKeyState]) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetKeyState)(windows_core::Interface::as_raw(self), statearray.len().try_into().unwrap(), core::mem::transmute(statearray.as_ptr())) }
    }
    pub unsafe fn GetMouseState(&self, state: *mut GameInputMouseState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetMouseState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn GetTouchCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTouchCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTouchState(&self, statearray: &mut [GameInputTouchState]) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTouchState)(windows_core::Interface::as_raw(self), statearray.len().try_into().unwrap(), core::mem::transmute(statearray.as_ptr())) }
    }
    pub unsafe fn GetMotionState(&self, state: *mut GameInputMotionState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetMotionState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn GetArcadeStickState(&self, state: *mut GameInputArcadeStickState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetArcadeStickState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn GetFlightStickState(&self, state: *mut GameInputFlightStickState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetFlightStickState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn GetGamepadState(&self, state: *mut GameInputGamepadState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetGamepadState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn GetRacingWheelState(&self, state: *mut GameInputRacingWheelState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetRacingWheelState)(windows_core::Interface::as_raw(self), state as _) }
    }
    pub unsafe fn GetUiNavigationState(&self, state: *mut GameInputUiNavigationState) -> bool {
        unsafe { (windows_core::Interface::vtable(self).GetUiNavigationState)(windows_core::Interface::as_raw(self), state as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameInputReading_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputKind: unsafe extern "system" fn(*mut core::ffi::c_void) -> GameInputKind,
    pub GetSequenceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, GameInputKind) -> u64,
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetRawReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> bool,
    pub GetControllerAxisCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetControllerAxisState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> u32,
    pub GetControllerButtonCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetControllerButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> u32,
    pub GetControllerSwitchCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetControllerSwitchState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut GameInputSwitchPosition) -> u32,
    pub GetKeyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetKeyState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut GameInputKeyState) -> u32,
    pub GetMouseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputMouseState) -> bool,
    pub GetTouchCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetTouchState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut GameInputTouchState) -> u32,
    pub GetMotionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputMotionState) -> bool,
    pub GetArcadeStickState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputArcadeStickState) -> bool,
    pub GetFlightStickState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputFlightStickState) -> bool,
    pub GetGamepadState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputGamepadState) -> bool,
    pub GetRacingWheelState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputRacingWheelState) -> bool,
    pub GetUiNavigationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameInputUiNavigationState) -> bool,
}
pub trait IGameInputReading_Impl: windows_core::IUnknownImpl {
    fn GetInputKind(&self) -> GameInputKind;
    fn GetSequenceNumber(&self, inputkind: GameInputKind) -> u64;
    fn GetTimestamp(&self) -> u64;
    fn GetDevice(&self, device: windows_core::OutRef<IGameInputDevice>);
    fn GetRawReport(&self, report: windows_core::OutRef<IGameInputRawDeviceReport>) -> bool;
    fn GetControllerAxisCount(&self) -> u32;
    fn GetControllerAxisState(&self, statearraycount: u32, statearray: *mut f32) -> u32;
    fn GetControllerButtonCount(&self) -> u32;
    fn GetControllerButtonState(&self, statearraycount: u32, statearray: *mut bool) -> u32;
    fn GetControllerSwitchCount(&self) -> u32;
    fn GetControllerSwitchState(&self, statearraycount: u32, statearray: *mut GameInputSwitchPosition) -> u32;
    fn GetKeyCount(&self) -> u32;
    fn GetKeyState(&self, statearraycount: u32, statearray: *mut GameInputKeyState) -> u32;
    fn GetMouseState(&self, state: *mut GameInputMouseState) -> bool;
    fn GetTouchCount(&self) -> u32;
    fn GetTouchState(&self, statearraycount: u32, statearray: *mut GameInputTouchState) -> u32;
    fn GetMotionState(&self, state: *mut GameInputMotionState) -> bool;
    fn GetArcadeStickState(&self, state: *mut GameInputArcadeStickState) -> bool;
    fn GetFlightStickState(&self, state: *mut GameInputFlightStickState) -> bool;
    fn GetGamepadState(&self, state: *mut GameInputGamepadState) -> bool;
    fn GetRacingWheelState(&self, state: *mut GameInputRacingWheelState) -> bool;
    fn GetUiNavigationState(&self, state: *mut GameInputUiNavigationState) -> bool;
}
impl IGameInputReading_Vtbl {
    pub const fn new<Identity: IGameInputReading_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputKind<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> GameInputKind {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetInputKind(this)
            }
        }
        unsafe extern "system" fn GetSequenceNumber<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputkind: GameInputKind) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetSequenceNumber(this, core::mem::transmute_copy(&inputkind))
            }
        }
        unsafe extern "system" fn GetTimestamp<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetTimestamp(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetDevice(this, core::mem::transmute_copy(&device))
            }
        }
        unsafe extern "system" fn GetRawReport<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, report: *mut *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetRawReport(this, core::mem::transmute_copy(&report))
            }
        }
        unsafe extern "system" fn GetControllerAxisCount<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetControllerAxisCount(this)
            }
        }
        unsafe extern "system" fn GetControllerAxisState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statearraycount: u32, statearray: *mut f32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetControllerAxisState(this, core::mem::transmute_copy(&statearraycount), core::mem::transmute_copy(&statearray))
            }
        }
        unsafe extern "system" fn GetControllerButtonCount<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetControllerButtonCount(this)
            }
        }
        unsafe extern "system" fn GetControllerButtonState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statearraycount: u32, statearray: *mut bool) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetControllerButtonState(this, core::mem::transmute_copy(&statearraycount), core::mem::transmute_copy(&statearray))
            }
        }
        unsafe extern "system" fn GetControllerSwitchCount<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetControllerSwitchCount(this)
            }
        }
        unsafe extern "system" fn GetControllerSwitchState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statearraycount: u32, statearray: *mut GameInputSwitchPosition) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetControllerSwitchState(this, core::mem::transmute_copy(&statearraycount), core::mem::transmute_copy(&statearray))
            }
        }
        unsafe extern "system" fn GetKeyCount<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetKeyCount(this)
            }
        }
        unsafe extern "system" fn GetKeyState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statearraycount: u32, statearray: *mut GameInputKeyState) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetKeyState(this, core::mem::transmute_copy(&statearraycount), core::mem::transmute_copy(&statearray))
            }
        }
        unsafe extern "system" fn GetMouseState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputMouseState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetMouseState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn GetTouchCount<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetTouchCount(this)
            }
        }
        unsafe extern "system" fn GetTouchState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statearraycount: u32, statearray: *mut GameInputTouchState) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetTouchState(this, core::mem::transmute_copy(&statearraycount), core::mem::transmute_copy(&statearray))
            }
        }
        unsafe extern "system" fn GetMotionState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputMotionState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetMotionState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn GetArcadeStickState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputArcadeStickState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetArcadeStickState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn GetFlightStickState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputFlightStickState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetFlightStickState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn GetGamepadState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputGamepadState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetGamepadState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn GetRacingWheelState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputRacingWheelState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetRacingWheelState(this, core::mem::transmute_copy(&state))
            }
        }
        unsafe extern "system" fn GetUiNavigationState<Identity: IGameInputReading_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut GameInputUiNavigationState) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameInputReading_Impl::GetUiNavigationState(this, core::mem::transmute_copy(&state))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputKind: GetInputKind::<Identity, OFFSET>,
            GetSequenceNumber: GetSequenceNumber::<Identity, OFFSET>,
            GetTimestamp: GetTimestamp::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetRawReport: GetRawReport::<Identity, OFFSET>,
            GetControllerAxisCount: GetControllerAxisCount::<Identity, OFFSET>,
            GetControllerAxisState: GetControllerAxisState::<Identity, OFFSET>,
            GetControllerButtonCount: GetControllerButtonCount::<Identity, OFFSET>,
            GetControllerButtonState: GetControllerButtonState::<Identity, OFFSET>,
            GetControllerSwitchCount: GetControllerSwitchCount::<Identity, OFFSET>,
            GetControllerSwitchState: GetControllerSwitchState::<Identity, OFFSET>,
            GetKeyCount: GetKeyCount::<Identity, OFFSET>,
            GetKeyState: GetKeyState::<Identity, OFFSET>,
            GetMouseState: GetMouseState::<Identity, OFFSET>,
            GetTouchCount: GetTouchCount::<Identity, OFFSET>,
            GetTouchState: GetTouchState::<Identity, OFFSET>,
            GetMotionState: GetMotionState::<Identity, OFFSET>,
            GetArcadeStickState: GetArcadeStickState::<Identity, OFFSET>,
            GetFlightStickState: GetFlightStickState::<Identity, OFFSET>,
            GetGamepadState: GetGamepadState::<Identity, OFFSET>,
            GetRacingWheelState: GetRacingWheelState::<Identity, OFFSET>,
            GetUiNavigationState: GetUiNavigationState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameInputReading as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameInputReading {}
