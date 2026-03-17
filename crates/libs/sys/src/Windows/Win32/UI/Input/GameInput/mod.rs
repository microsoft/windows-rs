windows_link::link!("gameinput.dll" "system" fn GameInputCreate(gameinput : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const FACILITY_GAMEINPUT: u32 = 906u32;
pub const GAMEINPUT_E_DEVICE_DISCONNECTED: windows_sys::core::HRESULT = 0x838A0001_u32 as _;
pub const GAMEINPUT_E_DEVICE_NOT_FOUND: windows_sys::core::HRESULT = 0x838A0002_u32 as _;
pub const GAMEINPUT_E_INSUFFICIENT_FORCE_FEEDBACK_RESOURCES: windows_sys::core::HRESULT = 0x838A0006_u32 as _;
pub const GAMEINPUT_E_READING_NOT_FOUND: windows_sys::core::HRESULT = 0x838A0003_u32 as _;
pub const GAMEINPUT_E_REFERENCE_READING_TOO_OLD: windows_sys::core::HRESULT = 0x838A0004_u32 as _;
pub const GAMEINPUT_E_TIMESTAMP_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x838A0005_u32 as _;
pub const GameInput2WaySwitch: GameInputSwitchKind = 0i32;
pub const GameInput4WaySwitch: GameInputSwitchKind = 1i32;
pub const GameInput8WaySwitch: GameInputSwitchKind = 2i32;
pub const GameInputAbntKeyboard: GameInputKeyboardKind = 3i32;
pub const GameInputAnsiKeyboard: GameInputKeyboardKind = 0i32;
pub const GameInputApplicationItemCollection: GameInputRawDeviceItemCollectionKind = 1i32;
pub const GameInputArcadeStickAction1: GameInputArcadeStickButtons = 64i32;
pub const GameInputArcadeStickAction2: GameInputArcadeStickButtons = 128i32;
pub const GameInputArcadeStickAction3: GameInputArcadeStickButtons = 256i32;
pub const GameInputArcadeStickAction4: GameInputArcadeStickButtons = 512i32;
pub const GameInputArcadeStickAction5: GameInputArcadeStickButtons = 1024i32;
pub const GameInputArcadeStickAction6: GameInputArcadeStickButtons = 2048i32;
pub type GameInputArcadeStickButtons = i32;
pub const GameInputArcadeStickDown: GameInputArcadeStickButtons = 8i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputArcadeStickLeft: GameInputArcadeStickButtons = 16i32;
pub const GameInputArcadeStickMenu: GameInputArcadeStickButtons = 1i32;
pub const GameInputArcadeStickNone: GameInputArcadeStickButtons = 0i32;
pub const GameInputArcadeStickRight: GameInputArcadeStickButtons = 32i32;
pub const GameInputArcadeStickSpecial1: GameInputArcadeStickButtons = 4096i32;
pub const GameInputArcadeStickSpecial2: GameInputArcadeStickButtons = 8192i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputArcadeStickState {
    pub buttons: GameInputArcadeStickButtons,
}
pub const GameInputArcadeStickUp: GameInputArcadeStickButtons = 4i32;
pub const GameInputArcadeStickView: GameInputArcadeStickButtons = 2i32;
pub const GameInputArrayItem: GameInputRawDeviceReportItemFlags = 2i32;
pub const GameInputAsyncEnumeration: GameInputEnumerationKind = 1i32;
pub const GameInputBatteryCharging: GameInputBatteryStatus = 3i32;
pub const GameInputBatteryDischarging: GameInputBatteryStatus = 1i32;
pub const GameInputBatteryIdle: GameInputBatteryStatus = 2i32;
pub const GameInputBatteryNotPresent: GameInputBatteryStatus = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputBatteryState {
    pub chargeRate: f32,
    pub maxChargeRate: f32,
    pub remainingCapacity: f32,
    pub fullChargeCapacity: f32,
    pub status: GameInputBatteryStatus,
}
pub type GameInputBatteryStatus = i32;
pub const GameInputBatteryUnknown: GameInputBatteryStatus = -1i32;
pub const GameInputBlockingEnumeration: GameInputEnumerationKind = 2i32;
pub const GameInputBufferedItem: GameInputRawDeviceReportItemFlags = 256i32;
pub const GameInputConstantItem: GameInputRawDeviceReportItemFlags = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
pub const GameInputDefaultFocusPolicy: GameInputFocusPolicy = 0i32;
pub const GameInputDefaultItem: GameInputRawDeviceReportItemFlags = 0i32;
pub const GameInputDeviceAnyStatus: GameInputDeviceStatus = 16777215i32;
pub const GameInputDeviceAudioCapture: GameInputDeviceStatus = 16i32;
pub const GameInputDeviceAudioRender: GameInputDeviceStatus = 32i32;
pub type GameInputDeviceCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: *mut core::ffi::c_void, timestamp: u64, currentstatus: GameInputDeviceStatus, previousstatus: GameInputDeviceStatus)>;
pub type GameInputDeviceCapabilities = i32;
pub const GameInputDeviceCapabilityAudio: GameInputDeviceCapabilities = 1i32;
pub const GameInputDeviceCapabilityNone: GameInputDeviceCapabilities = 0i32;
pub const GameInputDeviceCapabilityPluginModule: GameInputDeviceCapabilities = 2i32;
pub const GameInputDeviceCapabilityPowerOff: GameInputDeviceCapabilities = 4i32;
pub const GameInputDeviceCapabilitySynchronization: GameInputDeviceCapabilities = 8i32;
pub const GameInputDeviceCapabilityWireless: GameInputDeviceCapabilities = 16i32;
pub const GameInputDeviceConnected: GameInputDeviceStatus = 1i32;
pub type GameInputDeviceFamily = i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const GameInputDeviceInputEnabled: GameInputDeviceStatus = 2i32;
pub const GameInputDeviceNoStatus: GameInputDeviceStatus = 0i32;
pub const GameInputDeviceOutputEnabled: GameInputDeviceStatus = 4i32;
pub const GameInputDeviceRawIoEnabled: GameInputDeviceStatus = 8i32;
pub type GameInputDeviceStatus = i32;
pub const GameInputDeviceSynchronized: GameInputDeviceStatus = 64i32;
pub const GameInputDeviceUserIdle: GameInputDeviceStatus = 1048576i32;
pub const GameInputDeviceWireless: GameInputDeviceStatus = 128i32;
pub const GameInputDisableBackgroundGuideButton: GameInputFocusPolicy = 4i32;
pub const GameInputDisableBackgroundInput: GameInputFocusPolicy = 1i32;
pub const GameInputDisableBackgroundShareButton: GameInputFocusPolicy = 16i32;
pub type GameInputEnumerationKind = i32;
pub const GameInputExclusiveForegroundGuideButton: GameInputFocusPolicy = 8i32;
pub const GameInputExclusiveForegroundInput: GameInputFocusPolicy = 2i32;
pub const GameInputExclusiveForegroundShareButton: GameInputFocusPolicy = 32i32;
pub const GameInputFamilyAggregate: GameInputDeviceFamily = 0i32;
pub const GameInputFamilyHid: GameInputDeviceFamily = 3i32;
pub const GameInputFamilyI8042: GameInputDeviceFamily = 4i32;
pub const GameInputFamilyVirtual: GameInputDeviceFamily = -1i32;
pub const GameInputFamilyXbox360: GameInputDeviceFamily = 2i32;
pub const GameInputFamilyXboxOne: GameInputDeviceFamily = 1i32;
pub type GameInputFeedbackAxes = i32;
pub const GameInputFeedbackAxisAngularX: GameInputFeedbackAxes = 8i32;
pub const GameInputFeedbackAxisAngularY: GameInputFeedbackAxes = 16i32;
pub const GameInputFeedbackAxisAngularZ: GameInputFeedbackAxes = 32i32;
pub const GameInputFeedbackAxisLinearX: GameInputFeedbackAxes = 1i32;
pub const GameInputFeedbackAxisLinearY: GameInputFeedbackAxes = 2i32;
pub const GameInputFeedbackAxisLinearZ: GameInputFeedbackAxes = 4i32;
pub const GameInputFeedbackAxisNone: GameInputFeedbackAxes = 0i32;
pub const GameInputFeedbackAxisNormal: GameInputFeedbackAxes = 64i32;
pub type GameInputFeedbackEffectState = i32;
pub const GameInputFeedbackPaused: GameInputFeedbackEffectState = 2i32;
pub const GameInputFeedbackRunning: GameInputFeedbackEffectState = 1i32;
pub const GameInputFeedbackStopped: GameInputFeedbackEffectState = 0i32;
pub type GameInputFlightStickButtons = i32;
pub const GameInputFlightStickFirePrimary: GameInputFlightStickButtons = 4i32;
pub const GameInputFlightStickFireSecondary: GameInputFlightStickButtons = 8i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputFlightStickInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub firePrimaryButtonLabel: GameInputLabel,
    pub fireSecondaryButtonLabel: GameInputLabel,
    pub hatSwitchKind: GameInputSwitchKind,
}
pub const GameInputFlightStickMenu: GameInputFlightStickButtons = 1i32;
pub const GameInputFlightStickNone: GameInputFlightStickButtons = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputFlightStickState {
    pub buttons: GameInputFlightStickButtons,
    pub hatSwitch: GameInputSwitchPosition,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub throttle: f32,
}
pub const GameInputFlightStickView: GameInputFlightStickButtons = 2i32;
pub type GameInputFocusPolicy = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputForceFeedbackConditionParams {
    pub magnitude: GameInputForceFeedbackMagnitude,
    pub positiveCoefficient: f32,
    pub negativeCoefficient: f32,
    pub maxPositiveMagnitude: f32,
    pub maxNegativeMagnitude: f32,
    pub deadZone: f32,
    pub bias: f32,
}
pub const GameInputForceFeedbackConstant: GameInputForceFeedbackEffectKind = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputForceFeedbackConstantParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub magnitude: GameInputForceFeedbackMagnitude,
}
pub const GameInputForceFeedbackDamper: GameInputForceFeedbackEffectKind = 9i32;
pub type GameInputForceFeedbackEffectKind = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputForceFeedbackFriction: GameInputForceFeedbackEffectKind = 8i32;
pub const GameInputForceFeedbackInertia: GameInputForceFeedbackEffectKind = 10i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct GameInputForceFeedbackPeriodicParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub magnitude: GameInputForceFeedbackMagnitude,
    pub frequency: f32,
    pub phase: f32,
    pub bias: f32,
}
pub const GameInputForceFeedbackRamp: GameInputForceFeedbackEffectKind = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputForceFeedbackRampParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub startMagnitude: GameInputForceFeedbackMagnitude,
    pub endMagnitude: GameInputForceFeedbackMagnitude,
}
pub const GameInputForceFeedbackSawtoothDownWave: GameInputForceFeedbackEffectKind = 6i32;
pub const GameInputForceFeedbackSawtoothUpWave: GameInputForceFeedbackEffectKind = 5i32;
pub const GameInputForceFeedbackSineWave: GameInputForceFeedbackEffectKind = 2i32;
pub const GameInputForceFeedbackSpring: GameInputForceFeedbackEffectKind = 7i32;
pub const GameInputForceFeedbackSquareWave: GameInputForceFeedbackEffectKind = 3i32;
pub const GameInputForceFeedbackTriangleWave: GameInputForceFeedbackEffectKind = 4i32;
pub const GameInputGamepadA: GameInputGamepadButtons = 4i32;
pub const GameInputGamepadB: GameInputGamepadButtons = 8i32;
pub type GameInputGamepadButtons = i32;
pub const GameInputGamepadDPadDown: GameInputGamepadButtons = 128i32;
pub const GameInputGamepadDPadLeft: GameInputGamepadButtons = 256i32;
pub const GameInputGamepadDPadRight: GameInputGamepadButtons = 512i32;
pub const GameInputGamepadDPadUp: GameInputGamepadButtons = 64i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputGamepadLeftShoulder: GameInputGamepadButtons = 1024i32;
pub const GameInputGamepadLeftThumbstick: GameInputGamepadButtons = 4096i32;
pub const GameInputGamepadMenu: GameInputGamepadButtons = 1i32;
pub const GameInputGamepadNone: GameInputGamepadButtons = 0i32;
pub const GameInputGamepadRightShoulder: GameInputGamepadButtons = 2048i32;
pub const GameInputGamepadRightThumbstick: GameInputGamepadButtons = 8192i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputGamepadState {
    pub buttons: GameInputGamepadButtons,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
    pub leftThumbstickX: f32,
    pub leftThumbstickY: f32,
    pub rightThumbstickX: f32,
    pub rightThumbstickY: f32,
}
pub const GameInputGamepadView: GameInputGamepadButtons = 2i32;
pub const GameInputGamepadX: GameInputGamepadButtons = 16i32;
pub const GameInputGamepadY: GameInputGamepadButtons = 32i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct GameInputHapticFeedbackParams {
    pub waveformIndex: u32,
    pub duration: u64,
    pub intensity: f32,
    pub playCount: u32,
    pub repeatDelay: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputHapticWaveformInfo {
    pub usage: GameInputUsage,
    pub isDurationSupported: u8,
    pub isIntensitySupported: u8,
    pub isRepeatSupported: u8,
    pub isRepeatDelaySupported: u8,
    pub defaultDuration: u64,
}
pub const GameInputIsoKeyboard: GameInputKeyboardKind = 1i32;
pub const GameInputJisKeyboard: GameInputKeyboardKind = 4i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputKeyState {
    pub scanCode: u32,
    pub codePoint: u32,
    pub virtualKey: u8,
    pub isDeadKey: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type GameInputKeyboardKind = i32;
pub type GameInputKeyboardLayoutCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: *mut core::ffi::c_void, timestamp: u64, currentlayout: u32, previouslayout: u32)>;
pub type GameInputKind = i32;
pub const GameInputKindArcadeStick: GameInputKind = 65536i32;
pub const GameInputKindController: GameInputKind = 14i32;
pub const GameInputKindControllerAxis: GameInputKind = 2i32;
pub const GameInputKindControllerButton: GameInputKind = 4i32;
pub const GameInputKindControllerSwitch: GameInputKind = 8i32;
pub const GameInputKindFlightStick: GameInputKind = 131072i32;
pub const GameInputKindGamepad: GameInputKind = 262144i32;
pub const GameInputKindKeyboard: GameInputKind = 16i32;
pub const GameInputKindMotion: GameInputKind = 4096i32;
pub const GameInputKindMouse: GameInputKind = 32i32;
pub const GameInputKindRacingWheel: GameInputKind = 524288i32;
pub const GameInputKindRawDeviceReport: GameInputKind = 1i32;
pub const GameInputKindTouch: GameInputKind = 256i32;
pub const GameInputKindUiNavigation: GameInputKind = 16777216i32;
pub const GameInputKindUnknown: GameInputKind = 0i32;
pub const GameInputKsKeyboard: GameInputKeyboardKind = 2i32;
pub type GameInputLabel = i32;
pub const GameInputLabelArrowClockwise: GameInputLabel = 72i32;
pub const GameInputLabelArrowCounterClockwise: GameInputLabel = 73i32;
pub const GameInputLabelArrowDown: GameInputLabel = 65i32;
pub const GameInputLabelArrowDownLLeft: GameInputLabel = 66i32;
pub const GameInputLabelArrowDownRight: GameInputLabel = 64i32;
pub const GameInputLabelArrowLeft: GameInputLabel = 67i32;
pub const GameInputLabelArrowLeftRight: GameInputLabel = 70i32;
pub const GameInputLabelArrowReturn: GameInputLabel = 74i32;
pub const GameInputLabelArrowRight: GameInputLabel = 63i32;
pub const GameInputLabelArrowUp: GameInputLabel = 61i32;
pub const GameInputLabelArrowUpDown: GameInputLabel = 69i32;
pub const GameInputLabelArrowUpDownLeftRight: GameInputLabel = 71i32;
pub const GameInputLabelArrowUpLeft: GameInputLabel = 68i32;
pub const GameInputLabelArrowUpRight: GameInputLabel = 62i32;
pub const GameInputLabelBack: GameInputLabel = 101i32;
pub const GameInputLabelDown: GameInputLabel = 106i32;
pub const GameInputLabelGuide: GameInputLabel = 96i32;
pub const GameInputLabelHome: GameInputLabel = 95i32;
pub const GameInputLabelIconBranding: GameInputLabel = 75i32;
pub const GameInputLabelIconCircle: GameInputLabel = 79i32;
pub const GameInputLabelIconCross: GameInputLabel = 78i32;
pub const GameInputLabelIconDPadDown: GameInputLabel = 84i32;
pub const GameInputLabelIconDPadLeft: GameInputLabel = 85i32;
pub const GameInputLabelIconDPadRight: GameInputLabel = 86i32;
pub const GameInputLabelIconDPadUp: GameInputLabel = 83i32;
pub const GameInputLabelIconDialClockwise: GameInputLabel = 87i32;
pub const GameInputLabelIconDialCounterClockwise: GameInputLabel = 88i32;
pub const GameInputLabelIconHome: GameInputLabel = 76i32;
pub const GameInputLabelIconMenu: GameInputLabel = 77i32;
pub const GameInputLabelIconMinus: GameInputLabel = 93i32;
pub const GameInputLabelIconPlus: GameInputLabel = 92i32;
pub const GameInputLabelIconSliderLeftRight: GameInputLabel = 89i32;
pub const GameInputLabelIconSliderUpDown: GameInputLabel = 90i32;
pub const GameInputLabelIconSquare: GameInputLabel = 80i32;
pub const GameInputLabelIconStar: GameInputLabel = 82i32;
pub const GameInputLabelIconSuspension: GameInputLabel = 94i32;
pub const GameInputLabelIconTriangle: GameInputLabel = 81i32;
pub const GameInputLabelIconWheelUpDown: GameInputLabel = 91i32;
pub const GameInputLabelL1: GameInputLabel = 112i32;
pub const GameInputLabelL2: GameInputLabel = 113i32;
pub const GameInputLabelL3: GameInputLabel = 114i32;
pub const GameInputLabelLB: GameInputLabel = 109i32;
pub const GameInputLabelLSB: GameInputLabel = 111i32;
pub const GameInputLabelLT: GameInputLabel = 110i32;
pub const GameInputLabelLeft: GameInputLabel = 107i32;
pub const GameInputLabelLetterA: GameInputLabel = 25i32;
pub const GameInputLabelLetterB: GameInputLabel = 26i32;
pub const GameInputLabelLetterC: GameInputLabel = 27i32;
pub const GameInputLabelLetterD: GameInputLabel = 28i32;
pub const GameInputLabelLetterE: GameInputLabel = 29i32;
pub const GameInputLabelLetterF: GameInputLabel = 30i32;
pub const GameInputLabelLetterG: GameInputLabel = 31i32;
pub const GameInputLabelLetterH: GameInputLabel = 32i32;
pub const GameInputLabelLetterI: GameInputLabel = 33i32;
pub const GameInputLabelLetterJ: GameInputLabel = 34i32;
pub const GameInputLabelLetterK: GameInputLabel = 35i32;
pub const GameInputLabelLetterL: GameInputLabel = 36i32;
pub const GameInputLabelLetterM: GameInputLabel = 37i32;
pub const GameInputLabelLetterN: GameInputLabel = 38i32;
pub const GameInputLabelLetterO: GameInputLabel = 39i32;
pub const GameInputLabelLetterP: GameInputLabel = 40i32;
pub const GameInputLabelLetterQ: GameInputLabel = 41i32;
pub const GameInputLabelLetterR: GameInputLabel = 42i32;
pub const GameInputLabelLetterS: GameInputLabel = 43i32;
pub const GameInputLabelLetterT: GameInputLabel = 44i32;
pub const GameInputLabelLetterU: GameInputLabel = 45i32;
pub const GameInputLabelLetterV: GameInputLabel = 46i32;
pub const GameInputLabelLetterW: GameInputLabel = 47i32;
pub const GameInputLabelLetterX: GameInputLabel = 48i32;
pub const GameInputLabelLetterY: GameInputLabel = 49i32;
pub const GameInputLabelLetterZ: GameInputLabel = 50i32;
pub const GameInputLabelMenu: GameInputLabel = 99i32;
pub const GameInputLabelMode: GameInputLabel = 97i32;
pub const GameInputLabelNone: GameInputLabel = 0i32;
pub const GameInputLabelNumber0: GameInputLabel = 51i32;
pub const GameInputLabelNumber1: GameInputLabel = 52i32;
pub const GameInputLabelNumber2: GameInputLabel = 53i32;
pub const GameInputLabelNumber3: GameInputLabel = 54i32;
pub const GameInputLabelNumber4: GameInputLabel = 55i32;
pub const GameInputLabelNumber5: GameInputLabel = 56i32;
pub const GameInputLabelNumber6: GameInputLabel = 57i32;
pub const GameInputLabelNumber7: GameInputLabel = 58i32;
pub const GameInputLabelNumber8: GameInputLabel = 59i32;
pub const GameInputLabelNumber9: GameInputLabel = 60i32;
pub const GameInputLabelOptions: GameInputLabel = 103i32;
pub const GameInputLabelP1: GameInputLabel = 121i32;
pub const GameInputLabelP2: GameInputLabel = 122i32;
pub const GameInputLabelP3: GameInputLabel = 123i32;
pub const GameInputLabelP4: GameInputLabel = 124i32;
pub const GameInputLabelR1: GameInputLabel = 118i32;
pub const GameInputLabelR2: GameInputLabel = 119i32;
pub const GameInputLabelR3: GameInputLabel = 120i32;
pub const GameInputLabelRB: GameInputLabel = 115i32;
pub const GameInputLabelRSB: GameInputLabel = 117i32;
pub const GameInputLabelRT: GameInputLabel = 116i32;
pub const GameInputLabelRight: GameInputLabel = 108i32;
pub const GameInputLabelSelect: GameInputLabel = 98i32;
pub const GameInputLabelShare: GameInputLabel = 104i32;
pub const GameInputLabelStart: GameInputLabel = 102i32;
pub const GameInputLabelUnknown: GameInputLabel = -1i32;
pub const GameInputLabelUp: GameInputLabel = 105i32;
pub const GameInputLabelView: GameInputLabel = 100i32;
pub const GameInputLabelXboxA: GameInputLabel = 7i32;
pub const GameInputLabelXboxB: GameInputLabel = 8i32;
pub const GameInputLabelXboxBack: GameInputLabel = 2i32;
pub const GameInputLabelXboxDPadDown: GameInputLabel = 12i32;
pub const GameInputLabelXboxDPadLeft: GameInputLabel = 13i32;
pub const GameInputLabelXboxDPadRight: GameInputLabel = 14i32;
pub const GameInputLabelXboxDPadUp: GameInputLabel = 11i32;
pub const GameInputLabelXboxGuide: GameInputLabel = 1i32;
pub const GameInputLabelXboxLeftShoulder: GameInputLabel = 15i32;
pub const GameInputLabelXboxLeftStickButton: GameInputLabel = 17i32;
pub const GameInputLabelXboxLeftTrigger: GameInputLabel = 16i32;
pub const GameInputLabelXboxMenu: GameInputLabel = 4i32;
pub const GameInputLabelXboxPaddle1: GameInputLabel = 21i32;
pub const GameInputLabelXboxPaddle2: GameInputLabel = 22i32;
pub const GameInputLabelXboxPaddle3: GameInputLabel = 23i32;
pub const GameInputLabelXboxPaddle4: GameInputLabel = 24i32;
pub const GameInputLabelXboxRightShoulder: GameInputLabel = 18i32;
pub const GameInputLabelXboxRightStickButton: GameInputLabel = 20i32;
pub const GameInputLabelXboxRightTrigger: GameInputLabel = 19i32;
pub const GameInputLabelXboxStart: GameInputLabel = 3i32;
pub const GameInputLabelXboxView: GameInputLabel = 5i32;
pub const GameInputLabelXboxX: GameInputLabel = 9i32;
pub const GameInputLabelXboxY: GameInputLabel = 10i32;
pub type GameInputLocation = i32;
pub const GameInputLocationAxis: GameInputLocation = 2i32;
pub const GameInputLocationButton: GameInputLocation = 3i32;
pub const GameInputLocationChassis: GameInputLocation = 0i32;
pub const GameInputLocationDisplay: GameInputLocation = 1i32;
pub const GameInputLocationKey: GameInputLocation = 5i32;
pub const GameInputLocationSwitch: GameInputLocation = 4i32;
pub const GameInputLocationTouchPad: GameInputLocation = 6i32;
pub const GameInputLocationUnknown: GameInputLocation = -1i32;
pub const GameInputLogicalItemCollection: GameInputRawDeviceItemCollectionKind = 2i32;
pub type GameInputMotionAccuracy = i32;
pub const GameInputMotionAccuracyUnknown: GameInputMotionAccuracy = -1i32;
pub const GameInputMotionAccurate: GameInputMotionAccuracy = 3i32;
pub const GameInputMotionApproximate: GameInputMotionAccuracy = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputMotionInfo {
    pub maxAcceleration: f32,
    pub maxAngularVelocity: f32,
    pub maxMagneticFieldStrength: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputMotionUnavailable: GameInputMotionAccuracy = 0i32;
pub const GameInputMotionUnreliable: GameInputMotionAccuracy = 1i32;
pub const GameInputMouseButton4: GameInputMouseButtons = 8i32;
pub const GameInputMouseButton5: GameInputMouseButtons = 16i32;
pub type GameInputMouseButtons = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputMouseInfo {
    pub supportedButtons: GameInputMouseButtons,
    pub sampleRate: u32,
    pub sensorDpi: u32,
    pub hasWheelX: u8,
    pub hasWheelY: u8,
}
pub const GameInputMouseLeftButton: GameInputMouseButtons = 1i32;
pub const GameInputMouseMiddleButton: GameInputMouseButtons = 4i32;
pub const GameInputMouseNone: GameInputMouseButtons = 0i32;
pub const GameInputMouseRightButton: GameInputMouseButtons = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputMouseState {
    pub buttons: GameInputMouseButtons,
    pub positionX: i64,
    pub positionY: i64,
    pub wheelX: i64,
    pub wheelY: i64,
}
pub const GameInputMouseWheelTiltLeft: GameInputMouseButtons = 32i32;
pub const GameInputMouseWheelTiltRight: GameInputMouseButtons = 64i32;
pub const GameInputNamedArrayItemCollection: GameInputRawDeviceItemCollectionKind = 4i32;
pub const GameInputNoEnumeration: GameInputEnumerationKind = 0i32;
pub const GameInputNonlinearItem: GameInputRawDeviceReportItemFlags = 16i32;
pub const GameInputNullableItem: GameInputRawDeviceReportItemFlags = 64i32;
pub const GameInputPhysicalItemCollection: GameInputRawDeviceItemCollectionKind = 0i32;
pub const GameInputPhysicalUnitAcceleration: GameInputRawDevicePhysicalUnitKind = 5i32;
pub const GameInputPhysicalUnitAngle: GameInputRawDevicePhysicalUnitKind = 10i32;
pub const GameInputPhysicalUnitAngularAcceleration: GameInputRawDevicePhysicalUnitKind = 12i32;
pub const GameInputPhysicalUnitAngularMass: GameInputRawDevicePhysicalUnitKind = 13i32;
pub const GameInputPhysicalUnitAngularMomentum: GameInputRawDevicePhysicalUnitKind = 14i32;
pub const GameInputPhysicalUnitAngularTorque: GameInputRawDevicePhysicalUnitKind = 15i32;
pub const GameInputPhysicalUnitAngularVelocity: GameInputRawDevicePhysicalUnitKind = 11i32;
pub const GameInputPhysicalUnitElectricCharge: GameInputRawDevicePhysicalUnitKind = 17i32;
pub const GameInputPhysicalUnitElectricCurrent: GameInputRawDevicePhysicalUnitKind = 16i32;
pub const GameInputPhysicalUnitElectricPotential: GameInputRawDevicePhysicalUnitKind = 18i32;
pub const GameInputPhysicalUnitEnergy: GameInputRawDevicePhysicalUnitKind = 19i32;
pub const GameInputPhysicalUnitForce: GameInputRawDevicePhysicalUnitKind = 8i32;
pub const GameInputPhysicalUnitFrequency: GameInputRawDevicePhysicalUnitKind = 2i32;
pub const GameInputPhysicalUnitIlluminance: GameInputRawDevicePhysicalUnitKind = 24i32;
pub const GameInputPhysicalUnitLength: GameInputRawDevicePhysicalUnitKind = 3i32;
pub const GameInputPhysicalUnitLuminousFlux: GameInputRawDevicePhysicalUnitKind = 23i32;
pub const GameInputPhysicalUnitLuminousIntensity: GameInputRawDevicePhysicalUnitKind = 22i32;
pub const GameInputPhysicalUnitMass: GameInputRawDevicePhysicalUnitKind = 6i32;
pub const GameInputPhysicalUnitMomentum: GameInputRawDevicePhysicalUnitKind = 7i32;
pub const GameInputPhysicalUnitNone: GameInputRawDevicePhysicalUnitKind = 0i32;
pub const GameInputPhysicalUnitPower: GameInputRawDevicePhysicalUnitKind = 20i32;
pub const GameInputPhysicalUnitPressure: GameInputRawDevicePhysicalUnitKind = 9i32;
pub const GameInputPhysicalUnitTemperature: GameInputRawDevicePhysicalUnitKind = 21i32;
pub const GameInputPhysicalUnitTime: GameInputRawDevicePhysicalUnitKind = 1i32;
pub const GameInputPhysicalUnitUnknown: GameInputRawDevicePhysicalUnitKind = -1i32;
pub const GameInputPhysicalUnitVelocity: GameInputRawDevicePhysicalUnitKind = 4i32;
pub type GameInputRacingWheelButtons = i32;
pub const GameInputRacingWheelDpadDown: GameInputRacingWheelButtons = 32i32;
pub const GameInputRacingWheelDpadLeft: GameInputRacingWheelButtons = 64i32;
pub const GameInputRacingWheelDpadRight: GameInputRacingWheelButtons = 128i32;
pub const GameInputRacingWheelDpadUp: GameInputRacingWheelButtons = 16i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputRacingWheelMenu: GameInputRacingWheelButtons = 1i32;
pub const GameInputRacingWheelNextGear: GameInputRacingWheelButtons = 8i32;
pub const GameInputRacingWheelNone: GameInputRacingWheelButtons = 0i32;
pub const GameInputRacingWheelPreviousGear: GameInputRacingWheelButtons = 4i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputRacingWheelState {
    pub buttons: GameInputRacingWheelButtons,
    pub patternShifterGear: i32,
    pub wheel: f32,
    pub throttle: f32,
    pub brake: f32,
    pub clutch: f32,
    pub handbrake: f32,
}
pub const GameInputRacingWheelView: GameInputRacingWheelButtons = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type GameInputRawDeviceItemCollectionKind = i32;
pub type GameInputRawDevicePhysicalUnitKind = i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type GameInputRawDeviceReportItemFlags = i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type GameInputRawDeviceReportKind = i32;
pub const GameInputRawFeatureReport: GameInputRawDeviceReportKind = 2i32;
pub const GameInputRawInputReport: GameInputRawDeviceReportKind = 0i32;
pub const GameInputRawOutputReport: GameInputRawDeviceReportKind = 1i32;
pub type GameInputReadingCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, reading: *mut core::ffi::c_void, hasoverrunoccurred: bool)>;
pub const GameInputRelativeItem: GameInputRawDeviceReportItemFlags = 4i32;
pub const GameInputReportItemCollection: GameInputRawDeviceItemCollectionKind = 3i32;
pub const GameInputRumbleHighFrequency: GameInputRumbleMotors = 2i32;
pub const GameInputRumbleLeftTrigger: GameInputRumbleMotors = 4i32;
pub const GameInputRumbleLowFrequency: GameInputRumbleMotors = 1i32;
pub type GameInputRumbleMotors = i32;
pub const GameInputRumbleNone: GameInputRumbleMotors = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputRumbleParams {
    pub lowFrequency: f32,
    pub highFrequency: f32,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
}
pub const GameInputRumbleRightTrigger: GameInputRumbleMotors = 8i32;
pub const GameInputStableItem: GameInputRawDeviceReportItemFlags = 32i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GameInputString {
    pub sizeInBytes: u32,
    pub codePointCount: u32,
    pub data: windows_sys::core::PCSTR,
}
impl Default for GameInputString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GameInputSwitchCenter: GameInputSwitchPosition = 0i32;
pub const GameInputSwitchDown: GameInputSwitchPosition = 5i32;
pub const GameInputSwitchDownLeft: GameInputSwitchPosition = 6i32;
pub const GameInputSwitchDownRight: GameInputSwitchPosition = 4i32;
pub type GameInputSwitchKind = i32;
pub const GameInputSwitchLeft: GameInputSwitchPosition = 7i32;
pub type GameInputSwitchPosition = i32;
pub const GameInputSwitchRight: GameInputSwitchPosition = 3i32;
pub const GameInputSwitchUp: GameInputSwitchPosition = 1i32;
pub const GameInputSwitchUpLeft: GameInputSwitchPosition = 8i32;
pub const GameInputSwitchUpRight: GameInputSwitchPosition = 2i32;
pub type GameInputSystemButtonCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: *mut core::ffi::c_void, timestamp: u64, currentbuttons: GameInputSystemButtons, previousbuttons: GameInputSystemButtons)>;
pub const GameInputSystemButtonGuide: GameInputSystemButtons = 1i32;
pub const GameInputSystemButtonNone: GameInputSystemButtons = 0i32;
pub const GameInputSystemButtonShare: GameInputSystemButtons = 2i32;
pub type GameInputSystemButtons = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type GameInputTouchShape = i32;
pub const GameInputTouchShape1DIrregular: GameInputTouchShape = 3i32;
pub const GameInputTouchShape1DLinear: GameInputTouchShape = 1i32;
pub const GameInputTouchShape1DRadial: GameInputTouchShape = 2i32;
pub const GameInputTouchShape2DElliptical: GameInputTouchShape = 5i32;
pub const GameInputTouchShape2DIrregular: GameInputTouchShape = 6i32;
pub const GameInputTouchShape2DRectangular: GameInputTouchShape = 4i32;
pub const GameInputTouchShapePoint: GameInputTouchShape = 0i32;
pub const GameInputTouchShapeUnknown: GameInputTouchShape = -1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputUiNavigationAccept: GameInputUiNavigationButtons = 4i32;
pub type GameInputUiNavigationButtons = i32;
pub const GameInputUiNavigationCancel: GameInputUiNavigationButtons = 8i32;
pub const GameInputUiNavigationContext1: GameInputUiNavigationButtons = 256i32;
pub const GameInputUiNavigationContext2: GameInputUiNavigationButtons = 512i32;
pub const GameInputUiNavigationContext3: GameInputUiNavigationButtons = 1024i32;
pub const GameInputUiNavigationContext4: GameInputUiNavigationButtons = 2048i32;
pub const GameInputUiNavigationDown: GameInputUiNavigationButtons = 32i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const GameInputUiNavigationLeft: GameInputUiNavigationButtons = 64i32;
pub const GameInputUiNavigationMenu: GameInputUiNavigationButtons = 1i32;
pub const GameInputUiNavigationNone: GameInputUiNavigationButtons = 0i32;
pub const GameInputUiNavigationPageDown: GameInputUiNavigationButtons = 8192i32;
pub const GameInputUiNavigationPageLeft: GameInputUiNavigationButtons = 16384i32;
pub const GameInputUiNavigationPageRight: GameInputUiNavigationButtons = 32768i32;
pub const GameInputUiNavigationPageUp: GameInputUiNavigationButtons = 4096i32;
pub const GameInputUiNavigationRight: GameInputUiNavigationButtons = 128i32;
pub const GameInputUiNavigationScrollDown: GameInputUiNavigationButtons = 131072i32;
pub const GameInputUiNavigationScrollLeft: GameInputUiNavigationButtons = 262144i32;
pub const GameInputUiNavigationScrollRight: GameInputUiNavigationButtons = 524288i32;
pub const GameInputUiNavigationScrollUp: GameInputUiNavigationButtons = 65536i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputUiNavigationState {
    pub buttons: GameInputUiNavigationButtons,
}
pub const GameInputUiNavigationUp: GameInputUiNavigationButtons = 16i32;
pub const GameInputUiNavigationView: GameInputUiNavigationButtons = 2i32;
pub const GameInputUnknownItemCollection: GameInputRawDeviceItemCollectionKind = -1i32;
pub const GameInputUnknownKeyboard: GameInputKeyboardKind = -1i32;
pub const GameInputUnknownSwitchKind: GameInputSwitchKind = -1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputUsage {
    pub page: u16,
    pub id: u16,
}
pub const GameInputUsageModifierItemCollection: GameInputRawDeviceItemCollectionKind = 6i32;
pub const GameInputUsageSwitchItemCollection: GameInputRawDeviceItemCollectionKind = 5i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GameInputVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub revision: u16,
}
pub const GameInputVolatileItem: GameInputRawDeviceReportItemFlags = 128i32;
pub const GameInputWraparoundItem: GameInputRawDeviceReportItemFlags = 8i32;
