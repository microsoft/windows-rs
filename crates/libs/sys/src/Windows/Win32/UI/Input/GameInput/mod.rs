windows_link::link!("gameinput.dll" "system" fn GameInputCreate(gameinput : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const FACILITY_GAMEINPUT: u32 = 906;
pub const GAMEINPUT_E_DEVICE_DISCONNECTED: windows_sys::core::HRESULT = 0x838A0001_u32 as _;
pub const GAMEINPUT_E_DEVICE_NOT_FOUND: windows_sys::core::HRESULT = 0x838A0002_u32 as _;
pub const GAMEINPUT_E_INSUFFICIENT_FORCE_FEEDBACK_RESOURCES: windows_sys::core::HRESULT = 0x838A0006_u32 as _;
pub const GAMEINPUT_E_READING_NOT_FOUND: windows_sys::core::HRESULT = 0x838A0003_u32 as _;
pub const GAMEINPUT_E_REFERENCE_READING_TOO_OLD: windows_sys::core::HRESULT = 0x838A0004_u32 as _;
pub const GAMEINPUT_E_TIMESTAMP_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x838A0005_u32 as _;
pub const GameInput2WaySwitch: GameInputSwitchKind = 0;
pub const GameInput4WaySwitch: GameInputSwitchKind = 1;
pub const GameInput8WaySwitch: GameInputSwitchKind = 2;
pub const GameInputAbntKeyboard: GameInputKeyboardKind = 3;
pub const GameInputAnsiKeyboard: GameInputKeyboardKind = 0;
pub const GameInputApplicationItemCollection: GameInputRawDeviceItemCollectionKind = 1;
pub const GameInputArcadeStickAction1: GameInputArcadeStickButtons = 64;
pub const GameInputArcadeStickAction2: GameInputArcadeStickButtons = 128;
pub const GameInputArcadeStickAction3: GameInputArcadeStickButtons = 256;
pub const GameInputArcadeStickAction4: GameInputArcadeStickButtons = 512;
pub const GameInputArcadeStickAction5: GameInputArcadeStickButtons = 1024;
pub const GameInputArcadeStickAction6: GameInputArcadeStickButtons = 2048;
pub type GameInputArcadeStickButtons = i32;
pub const GameInputArcadeStickDown: GameInputArcadeStickButtons = 8;
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
pub const GameInputArcadeStickLeft: GameInputArcadeStickButtons = 16;
pub const GameInputArcadeStickMenu: GameInputArcadeStickButtons = 1;
pub const GameInputArcadeStickNone: GameInputArcadeStickButtons = 0;
pub const GameInputArcadeStickRight: GameInputArcadeStickButtons = 32;
pub const GameInputArcadeStickSpecial1: GameInputArcadeStickButtons = 4096;
pub const GameInputArcadeStickSpecial2: GameInputArcadeStickButtons = 8192;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputArcadeStickState {
    pub buttons: GameInputArcadeStickButtons,
}
pub const GameInputArcadeStickUp: GameInputArcadeStickButtons = 4;
pub const GameInputArcadeStickView: GameInputArcadeStickButtons = 2;
pub const GameInputArrayItem: GameInputRawDeviceReportItemFlags = 2;
pub const GameInputAsyncEnumeration: GameInputEnumerationKind = 1;
pub const GameInputBatteryCharging: GameInputBatteryStatus = 3;
pub const GameInputBatteryDischarging: GameInputBatteryStatus = 1;
pub const GameInputBatteryIdle: GameInputBatteryStatus = 2;
pub const GameInputBatteryNotPresent: GameInputBatteryStatus = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputBatteryState {
    pub chargeRate: f32,
    pub maxChargeRate: f32,
    pub remainingCapacity: f32,
    pub fullChargeCapacity: f32,
    pub status: GameInputBatteryStatus,
}
pub type GameInputBatteryStatus = i32;
pub const GameInputBatteryUnknown: GameInputBatteryStatus = -1;
pub const GameInputBlockingEnumeration: GameInputEnumerationKind = 2;
pub const GameInputBufferedItem: GameInputRawDeviceReportItemFlags = 256;
pub const GameInputConstantItem: GameInputRawDeviceReportItemFlags = 1;
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
pub const GameInputDefaultFocusPolicy: GameInputFocusPolicy = 0;
pub const GameInputDefaultItem: GameInputRawDeviceReportItemFlags = 0;
pub const GameInputDeviceAnyStatus: GameInputDeviceStatus = 16777215;
pub const GameInputDeviceAudioCapture: GameInputDeviceStatus = 16;
pub const GameInputDeviceAudioRender: GameInputDeviceStatus = 32;
pub type GameInputDeviceCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: *mut core::ffi::c_void, timestamp: u64, currentstatus: GameInputDeviceStatus, previousstatus: GameInputDeviceStatus)>;
pub type GameInputDeviceCapabilities = i32;
pub const GameInputDeviceCapabilityAudio: GameInputDeviceCapabilities = 1;
pub const GameInputDeviceCapabilityNone: GameInputDeviceCapabilities = 0;
pub const GameInputDeviceCapabilityPluginModule: GameInputDeviceCapabilities = 2;
pub const GameInputDeviceCapabilityPowerOff: GameInputDeviceCapabilities = 4;
pub const GameInputDeviceCapabilitySynchronization: GameInputDeviceCapabilities = 8;
pub const GameInputDeviceCapabilityWireless: GameInputDeviceCapabilities = 16;
pub const GameInputDeviceConnected: GameInputDeviceStatus = 1;
pub type GameInputDeviceFamily = i32;
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
pub const GameInputDeviceInputEnabled: GameInputDeviceStatus = 2;
pub const GameInputDeviceNoStatus: GameInputDeviceStatus = 0;
pub const GameInputDeviceOutputEnabled: GameInputDeviceStatus = 4;
pub const GameInputDeviceRawIoEnabled: GameInputDeviceStatus = 8;
pub type GameInputDeviceStatus = i32;
pub const GameInputDeviceSynchronized: GameInputDeviceStatus = 64;
pub const GameInputDeviceUserIdle: GameInputDeviceStatus = 1048576;
pub const GameInputDeviceWireless: GameInputDeviceStatus = 128;
pub const GameInputDisableBackgroundGuideButton: GameInputFocusPolicy = 4;
pub const GameInputDisableBackgroundInput: GameInputFocusPolicy = 1;
pub const GameInputDisableBackgroundShareButton: GameInputFocusPolicy = 16;
pub type GameInputEnumerationKind = i32;
pub const GameInputExclusiveForegroundGuideButton: GameInputFocusPolicy = 8;
pub const GameInputExclusiveForegroundInput: GameInputFocusPolicy = 2;
pub const GameInputExclusiveForegroundShareButton: GameInputFocusPolicy = 32;
pub const GameInputFamilyAggregate: GameInputDeviceFamily = 0;
pub const GameInputFamilyHid: GameInputDeviceFamily = 3;
pub const GameInputFamilyI8042: GameInputDeviceFamily = 4;
pub const GameInputFamilyVirtual: GameInputDeviceFamily = -1;
pub const GameInputFamilyXbox360: GameInputDeviceFamily = 2;
pub const GameInputFamilyXboxOne: GameInputDeviceFamily = 1;
pub type GameInputFeedbackAxes = i32;
pub const GameInputFeedbackAxisAngularX: GameInputFeedbackAxes = 8;
pub const GameInputFeedbackAxisAngularY: GameInputFeedbackAxes = 16;
pub const GameInputFeedbackAxisAngularZ: GameInputFeedbackAxes = 32;
pub const GameInputFeedbackAxisLinearX: GameInputFeedbackAxes = 1;
pub const GameInputFeedbackAxisLinearY: GameInputFeedbackAxes = 2;
pub const GameInputFeedbackAxisLinearZ: GameInputFeedbackAxes = 4;
pub const GameInputFeedbackAxisNone: GameInputFeedbackAxes = 0;
pub const GameInputFeedbackAxisNormal: GameInputFeedbackAxes = 64;
pub type GameInputFeedbackEffectState = i32;
pub const GameInputFeedbackPaused: GameInputFeedbackEffectState = 2;
pub const GameInputFeedbackRunning: GameInputFeedbackEffectState = 1;
pub const GameInputFeedbackStopped: GameInputFeedbackEffectState = 0;
pub type GameInputFlightStickButtons = i32;
pub const GameInputFlightStickFirePrimary: GameInputFlightStickButtons = 4;
pub const GameInputFlightStickFireSecondary: GameInputFlightStickButtons = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputFlightStickInfo {
    pub menuButtonLabel: GameInputLabel,
    pub viewButtonLabel: GameInputLabel,
    pub firePrimaryButtonLabel: GameInputLabel,
    pub fireSecondaryButtonLabel: GameInputLabel,
    pub hatSwitchKind: GameInputSwitchKind,
}
pub const GameInputFlightStickMenu: GameInputFlightStickButtons = 1;
pub const GameInputFlightStickNone: GameInputFlightStickButtons = 0;
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
pub const GameInputFlightStickView: GameInputFlightStickButtons = 2;
pub type GameInputFocusPolicy = i32;
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
pub const GameInputForceFeedbackConstant: GameInputForceFeedbackEffectKind = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackConstantParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub magnitude: GameInputForceFeedbackMagnitude,
}
pub const GameInputForceFeedbackDamper: GameInputForceFeedbackEffectKind = 9;
pub type GameInputForceFeedbackEffectKind = i32;
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
pub const GameInputForceFeedbackFriction: GameInputForceFeedbackEffectKind = 8;
pub const GameInputForceFeedbackInertia: GameInputForceFeedbackEffectKind = 10;
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
pub const GameInputForceFeedbackRamp: GameInputForceFeedbackEffectKind = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputForceFeedbackRampParams {
    pub envelope: GameInputForceFeedbackEnvelope,
    pub startMagnitude: GameInputForceFeedbackMagnitude,
    pub endMagnitude: GameInputForceFeedbackMagnitude,
}
pub const GameInputForceFeedbackSawtoothDownWave: GameInputForceFeedbackEffectKind = 6;
pub const GameInputForceFeedbackSawtoothUpWave: GameInputForceFeedbackEffectKind = 5;
pub const GameInputForceFeedbackSineWave: GameInputForceFeedbackEffectKind = 2;
pub const GameInputForceFeedbackSpring: GameInputForceFeedbackEffectKind = 7;
pub const GameInputForceFeedbackSquareWave: GameInputForceFeedbackEffectKind = 3;
pub const GameInputForceFeedbackTriangleWave: GameInputForceFeedbackEffectKind = 4;
pub const GameInputGamepadA: GameInputGamepadButtons = 4;
pub const GameInputGamepadB: GameInputGamepadButtons = 8;
pub type GameInputGamepadButtons = i32;
pub const GameInputGamepadDPadDown: GameInputGamepadButtons = 128;
pub const GameInputGamepadDPadLeft: GameInputGamepadButtons = 256;
pub const GameInputGamepadDPadRight: GameInputGamepadButtons = 512;
pub const GameInputGamepadDPadUp: GameInputGamepadButtons = 64;
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
pub const GameInputGamepadLeftShoulder: GameInputGamepadButtons = 1024;
pub const GameInputGamepadLeftThumbstick: GameInputGamepadButtons = 4096;
pub const GameInputGamepadMenu: GameInputGamepadButtons = 1;
pub const GameInputGamepadNone: GameInputGamepadButtons = 0;
pub const GameInputGamepadRightShoulder: GameInputGamepadButtons = 2048;
pub const GameInputGamepadRightThumbstick: GameInputGamepadButtons = 8192;
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
pub const GameInputGamepadView: GameInputGamepadButtons = 2;
pub const GameInputGamepadX: GameInputGamepadButtons = 16;
pub const GameInputGamepadY: GameInputGamepadButtons = 32;
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
pub const GameInputIsoKeyboard: GameInputKeyboardKind = 1;
pub const GameInputJisKeyboard: GameInputKeyboardKind = 4;
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
pub type GameInputKeyboardKind = i32;
pub type GameInputKeyboardLayoutCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: *mut core::ffi::c_void, timestamp: u64, currentlayout: u32, previouslayout: u32)>;
pub type GameInputKind = i32;
pub const GameInputKindArcadeStick: GameInputKind = 65536;
pub const GameInputKindController: GameInputKind = 14;
pub const GameInputKindControllerAxis: GameInputKind = 2;
pub const GameInputKindControllerButton: GameInputKind = 4;
pub const GameInputKindControllerSwitch: GameInputKind = 8;
pub const GameInputKindFlightStick: GameInputKind = 131072;
pub const GameInputKindGamepad: GameInputKind = 262144;
pub const GameInputKindKeyboard: GameInputKind = 16;
pub const GameInputKindMotion: GameInputKind = 4096;
pub const GameInputKindMouse: GameInputKind = 32;
pub const GameInputKindRacingWheel: GameInputKind = 524288;
pub const GameInputKindRawDeviceReport: GameInputKind = 1;
pub const GameInputKindTouch: GameInputKind = 256;
pub const GameInputKindUiNavigation: GameInputKind = 16777216;
pub const GameInputKindUnknown: GameInputKind = 0;
pub const GameInputKsKeyboard: GameInputKeyboardKind = 2;
pub type GameInputLabel = i32;
pub const GameInputLabelArrowClockwise: GameInputLabel = 72;
pub const GameInputLabelArrowCounterClockwise: GameInputLabel = 73;
pub const GameInputLabelArrowDown: GameInputLabel = 65;
pub const GameInputLabelArrowDownLLeft: GameInputLabel = 66;
pub const GameInputLabelArrowDownRight: GameInputLabel = 64;
pub const GameInputLabelArrowLeft: GameInputLabel = 67;
pub const GameInputLabelArrowLeftRight: GameInputLabel = 70;
pub const GameInputLabelArrowReturn: GameInputLabel = 74;
pub const GameInputLabelArrowRight: GameInputLabel = 63;
pub const GameInputLabelArrowUp: GameInputLabel = 61;
pub const GameInputLabelArrowUpDown: GameInputLabel = 69;
pub const GameInputLabelArrowUpDownLeftRight: GameInputLabel = 71;
pub const GameInputLabelArrowUpLeft: GameInputLabel = 68;
pub const GameInputLabelArrowUpRight: GameInputLabel = 62;
pub const GameInputLabelBack: GameInputLabel = 101;
pub const GameInputLabelDown: GameInputLabel = 106;
pub const GameInputLabelGuide: GameInputLabel = 96;
pub const GameInputLabelHome: GameInputLabel = 95;
pub const GameInputLabelIconBranding: GameInputLabel = 75;
pub const GameInputLabelIconCircle: GameInputLabel = 79;
pub const GameInputLabelIconCross: GameInputLabel = 78;
pub const GameInputLabelIconDPadDown: GameInputLabel = 84;
pub const GameInputLabelIconDPadLeft: GameInputLabel = 85;
pub const GameInputLabelIconDPadRight: GameInputLabel = 86;
pub const GameInputLabelIconDPadUp: GameInputLabel = 83;
pub const GameInputLabelIconDialClockwise: GameInputLabel = 87;
pub const GameInputLabelIconDialCounterClockwise: GameInputLabel = 88;
pub const GameInputLabelIconHome: GameInputLabel = 76;
pub const GameInputLabelIconMenu: GameInputLabel = 77;
pub const GameInputLabelIconMinus: GameInputLabel = 93;
pub const GameInputLabelIconPlus: GameInputLabel = 92;
pub const GameInputLabelIconSliderLeftRight: GameInputLabel = 89;
pub const GameInputLabelIconSliderUpDown: GameInputLabel = 90;
pub const GameInputLabelIconSquare: GameInputLabel = 80;
pub const GameInputLabelIconStar: GameInputLabel = 82;
pub const GameInputLabelIconSuspension: GameInputLabel = 94;
pub const GameInputLabelIconTriangle: GameInputLabel = 81;
pub const GameInputLabelIconWheelUpDown: GameInputLabel = 91;
pub const GameInputLabelL1: GameInputLabel = 112;
pub const GameInputLabelL2: GameInputLabel = 113;
pub const GameInputLabelL3: GameInputLabel = 114;
pub const GameInputLabelLB: GameInputLabel = 109;
pub const GameInputLabelLSB: GameInputLabel = 111;
pub const GameInputLabelLT: GameInputLabel = 110;
pub const GameInputLabelLeft: GameInputLabel = 107;
pub const GameInputLabelLetterA: GameInputLabel = 25;
pub const GameInputLabelLetterB: GameInputLabel = 26;
pub const GameInputLabelLetterC: GameInputLabel = 27;
pub const GameInputLabelLetterD: GameInputLabel = 28;
pub const GameInputLabelLetterE: GameInputLabel = 29;
pub const GameInputLabelLetterF: GameInputLabel = 30;
pub const GameInputLabelLetterG: GameInputLabel = 31;
pub const GameInputLabelLetterH: GameInputLabel = 32;
pub const GameInputLabelLetterI: GameInputLabel = 33;
pub const GameInputLabelLetterJ: GameInputLabel = 34;
pub const GameInputLabelLetterK: GameInputLabel = 35;
pub const GameInputLabelLetterL: GameInputLabel = 36;
pub const GameInputLabelLetterM: GameInputLabel = 37;
pub const GameInputLabelLetterN: GameInputLabel = 38;
pub const GameInputLabelLetterO: GameInputLabel = 39;
pub const GameInputLabelLetterP: GameInputLabel = 40;
pub const GameInputLabelLetterQ: GameInputLabel = 41;
pub const GameInputLabelLetterR: GameInputLabel = 42;
pub const GameInputLabelLetterS: GameInputLabel = 43;
pub const GameInputLabelLetterT: GameInputLabel = 44;
pub const GameInputLabelLetterU: GameInputLabel = 45;
pub const GameInputLabelLetterV: GameInputLabel = 46;
pub const GameInputLabelLetterW: GameInputLabel = 47;
pub const GameInputLabelLetterX: GameInputLabel = 48;
pub const GameInputLabelLetterY: GameInputLabel = 49;
pub const GameInputLabelLetterZ: GameInputLabel = 50;
pub const GameInputLabelMenu: GameInputLabel = 99;
pub const GameInputLabelMode: GameInputLabel = 97;
pub const GameInputLabelNone: GameInputLabel = 0;
pub const GameInputLabelNumber0: GameInputLabel = 51;
pub const GameInputLabelNumber1: GameInputLabel = 52;
pub const GameInputLabelNumber2: GameInputLabel = 53;
pub const GameInputLabelNumber3: GameInputLabel = 54;
pub const GameInputLabelNumber4: GameInputLabel = 55;
pub const GameInputLabelNumber5: GameInputLabel = 56;
pub const GameInputLabelNumber6: GameInputLabel = 57;
pub const GameInputLabelNumber7: GameInputLabel = 58;
pub const GameInputLabelNumber8: GameInputLabel = 59;
pub const GameInputLabelNumber9: GameInputLabel = 60;
pub const GameInputLabelOptions: GameInputLabel = 103;
pub const GameInputLabelP1: GameInputLabel = 121;
pub const GameInputLabelP2: GameInputLabel = 122;
pub const GameInputLabelP3: GameInputLabel = 123;
pub const GameInputLabelP4: GameInputLabel = 124;
pub const GameInputLabelR1: GameInputLabel = 118;
pub const GameInputLabelR2: GameInputLabel = 119;
pub const GameInputLabelR3: GameInputLabel = 120;
pub const GameInputLabelRB: GameInputLabel = 115;
pub const GameInputLabelRSB: GameInputLabel = 117;
pub const GameInputLabelRT: GameInputLabel = 116;
pub const GameInputLabelRight: GameInputLabel = 108;
pub const GameInputLabelSelect: GameInputLabel = 98;
pub const GameInputLabelShare: GameInputLabel = 104;
pub const GameInputLabelStart: GameInputLabel = 102;
pub const GameInputLabelUnknown: GameInputLabel = -1;
pub const GameInputLabelUp: GameInputLabel = 105;
pub const GameInputLabelView: GameInputLabel = 100;
pub const GameInputLabelXboxA: GameInputLabel = 7;
pub const GameInputLabelXboxB: GameInputLabel = 8;
pub const GameInputLabelXboxBack: GameInputLabel = 2;
pub const GameInputLabelXboxDPadDown: GameInputLabel = 12;
pub const GameInputLabelXboxDPadLeft: GameInputLabel = 13;
pub const GameInputLabelXboxDPadRight: GameInputLabel = 14;
pub const GameInputLabelXboxDPadUp: GameInputLabel = 11;
pub const GameInputLabelXboxGuide: GameInputLabel = 1;
pub const GameInputLabelXboxLeftShoulder: GameInputLabel = 15;
pub const GameInputLabelXboxLeftStickButton: GameInputLabel = 17;
pub const GameInputLabelXboxLeftTrigger: GameInputLabel = 16;
pub const GameInputLabelXboxMenu: GameInputLabel = 4;
pub const GameInputLabelXboxPaddle1: GameInputLabel = 21;
pub const GameInputLabelXboxPaddle2: GameInputLabel = 22;
pub const GameInputLabelXboxPaddle3: GameInputLabel = 23;
pub const GameInputLabelXboxPaddle4: GameInputLabel = 24;
pub const GameInputLabelXboxRightShoulder: GameInputLabel = 18;
pub const GameInputLabelXboxRightStickButton: GameInputLabel = 20;
pub const GameInputLabelXboxRightTrigger: GameInputLabel = 19;
pub const GameInputLabelXboxStart: GameInputLabel = 3;
pub const GameInputLabelXboxView: GameInputLabel = 5;
pub const GameInputLabelXboxX: GameInputLabel = 9;
pub const GameInputLabelXboxY: GameInputLabel = 10;
pub type GameInputLocation = i32;
pub const GameInputLocationAxis: GameInputLocation = 2;
pub const GameInputLocationButton: GameInputLocation = 3;
pub const GameInputLocationChassis: GameInputLocation = 0;
pub const GameInputLocationDisplay: GameInputLocation = 1;
pub const GameInputLocationKey: GameInputLocation = 5;
pub const GameInputLocationSwitch: GameInputLocation = 4;
pub const GameInputLocationTouchPad: GameInputLocation = 6;
pub const GameInputLocationUnknown: GameInputLocation = -1;
pub const GameInputLogicalItemCollection: GameInputRawDeviceItemCollectionKind = 2;
pub type GameInputMotionAccuracy = i32;
pub const GameInputMotionAccuracyUnknown: GameInputMotionAccuracy = -1;
pub const GameInputMotionAccurate: GameInputMotionAccuracy = 3;
pub const GameInputMotionApproximate: GameInputMotionAccuracy = 2;
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
pub const GameInputMotionUnavailable: GameInputMotionAccuracy = 0;
pub const GameInputMotionUnreliable: GameInputMotionAccuracy = 1;
pub const GameInputMouseButton4: GameInputMouseButtons = 8;
pub const GameInputMouseButton5: GameInputMouseButtons = 16;
pub type GameInputMouseButtons = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputMouseInfo {
    pub supportedButtons: GameInputMouseButtons,
    pub sampleRate: u32,
    pub sensorDpi: u32,
    pub hasWheelX: u8,
    pub hasWheelY: u8,
}
pub const GameInputMouseLeftButton: GameInputMouseButtons = 1;
pub const GameInputMouseMiddleButton: GameInputMouseButtons = 4;
pub const GameInputMouseNone: GameInputMouseButtons = 0;
pub const GameInputMouseRightButton: GameInputMouseButtons = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputMouseState {
    pub buttons: GameInputMouseButtons,
    pub positionX: i64,
    pub positionY: i64,
    pub wheelX: i64,
    pub wheelY: i64,
}
pub const GameInputMouseWheelTiltLeft: GameInputMouseButtons = 32;
pub const GameInputMouseWheelTiltRight: GameInputMouseButtons = 64;
pub const GameInputNamedArrayItemCollection: GameInputRawDeviceItemCollectionKind = 4;
pub const GameInputNoEnumeration: GameInputEnumerationKind = 0;
pub const GameInputNonlinearItem: GameInputRawDeviceReportItemFlags = 16;
pub const GameInputNullableItem: GameInputRawDeviceReportItemFlags = 64;
pub const GameInputPhysicalItemCollection: GameInputRawDeviceItemCollectionKind = 0;
pub const GameInputPhysicalUnitAcceleration: GameInputRawDevicePhysicalUnitKind = 5;
pub const GameInputPhysicalUnitAngle: GameInputRawDevicePhysicalUnitKind = 10;
pub const GameInputPhysicalUnitAngularAcceleration: GameInputRawDevicePhysicalUnitKind = 12;
pub const GameInputPhysicalUnitAngularMass: GameInputRawDevicePhysicalUnitKind = 13;
pub const GameInputPhysicalUnitAngularMomentum: GameInputRawDevicePhysicalUnitKind = 14;
pub const GameInputPhysicalUnitAngularTorque: GameInputRawDevicePhysicalUnitKind = 15;
pub const GameInputPhysicalUnitAngularVelocity: GameInputRawDevicePhysicalUnitKind = 11;
pub const GameInputPhysicalUnitElectricCharge: GameInputRawDevicePhysicalUnitKind = 17;
pub const GameInputPhysicalUnitElectricCurrent: GameInputRawDevicePhysicalUnitKind = 16;
pub const GameInputPhysicalUnitElectricPotential: GameInputRawDevicePhysicalUnitKind = 18;
pub const GameInputPhysicalUnitEnergy: GameInputRawDevicePhysicalUnitKind = 19;
pub const GameInputPhysicalUnitForce: GameInputRawDevicePhysicalUnitKind = 8;
pub const GameInputPhysicalUnitFrequency: GameInputRawDevicePhysicalUnitKind = 2;
pub const GameInputPhysicalUnitIlluminance: GameInputRawDevicePhysicalUnitKind = 24;
pub const GameInputPhysicalUnitLength: GameInputRawDevicePhysicalUnitKind = 3;
pub const GameInputPhysicalUnitLuminousFlux: GameInputRawDevicePhysicalUnitKind = 23;
pub const GameInputPhysicalUnitLuminousIntensity: GameInputRawDevicePhysicalUnitKind = 22;
pub const GameInputPhysicalUnitMass: GameInputRawDevicePhysicalUnitKind = 6;
pub const GameInputPhysicalUnitMomentum: GameInputRawDevicePhysicalUnitKind = 7;
pub const GameInputPhysicalUnitNone: GameInputRawDevicePhysicalUnitKind = 0;
pub const GameInputPhysicalUnitPower: GameInputRawDevicePhysicalUnitKind = 20;
pub const GameInputPhysicalUnitPressure: GameInputRawDevicePhysicalUnitKind = 9;
pub const GameInputPhysicalUnitTemperature: GameInputRawDevicePhysicalUnitKind = 21;
pub const GameInputPhysicalUnitTime: GameInputRawDevicePhysicalUnitKind = 1;
pub const GameInputPhysicalUnitUnknown: GameInputRawDevicePhysicalUnitKind = -1;
pub const GameInputPhysicalUnitVelocity: GameInputRawDevicePhysicalUnitKind = 4;
pub type GameInputRacingWheelButtons = i32;
pub const GameInputRacingWheelDpadDown: GameInputRacingWheelButtons = 32;
pub const GameInputRacingWheelDpadLeft: GameInputRacingWheelButtons = 64;
pub const GameInputRacingWheelDpadRight: GameInputRacingWheelButtons = 128;
pub const GameInputRacingWheelDpadUp: GameInputRacingWheelButtons = 16;
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
pub const GameInputRacingWheelMenu: GameInputRacingWheelButtons = 1;
pub const GameInputRacingWheelNextGear: GameInputRacingWheelButtons = 8;
pub const GameInputRacingWheelNone: GameInputRacingWheelButtons = 0;
pub const GameInputRacingWheelPreviousGear: GameInputRacingWheelButtons = 4;
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
pub const GameInputRacingWheelView: GameInputRacingWheelButtons = 2;
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
pub type GameInputRawDeviceItemCollectionKind = i32;
pub type GameInputRawDevicePhysicalUnitKind = i32;
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
pub type GameInputRawDeviceReportItemFlags = i32;
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
pub type GameInputRawDeviceReportKind = i32;
pub const GameInputRawFeatureReport: GameInputRawDeviceReportKind = 2;
pub const GameInputRawInputReport: GameInputRawDeviceReportKind = 0;
pub const GameInputRawOutputReport: GameInputRawDeviceReportKind = 1;
pub type GameInputReadingCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, reading: *mut core::ffi::c_void, hasoverrunoccurred: bool)>;
pub const GameInputRelativeItem: GameInputRawDeviceReportItemFlags = 4;
pub const GameInputReportItemCollection: GameInputRawDeviceItemCollectionKind = 3;
pub const GameInputRumbleHighFrequency: GameInputRumbleMotors = 2;
pub const GameInputRumbleLeftTrigger: GameInputRumbleMotors = 4;
pub const GameInputRumbleLowFrequency: GameInputRumbleMotors = 1;
pub type GameInputRumbleMotors = i32;
pub const GameInputRumbleNone: GameInputRumbleMotors = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputRumbleParams {
    pub lowFrequency: f32,
    pub highFrequency: f32,
    pub leftTrigger: f32,
    pub rightTrigger: f32,
}
pub const GameInputRumbleRightTrigger: GameInputRumbleMotors = 8;
pub const GameInputStableItem: GameInputRawDeviceReportItemFlags = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const GameInputSwitchCenter: GameInputSwitchPosition = 0;
pub const GameInputSwitchDown: GameInputSwitchPosition = 5;
pub const GameInputSwitchDownLeft: GameInputSwitchPosition = 6;
pub const GameInputSwitchDownRight: GameInputSwitchPosition = 4;
pub type GameInputSwitchKind = i32;
pub const GameInputSwitchLeft: GameInputSwitchPosition = 7;
pub type GameInputSwitchPosition = i32;
pub const GameInputSwitchRight: GameInputSwitchPosition = 3;
pub const GameInputSwitchUp: GameInputSwitchPosition = 1;
pub const GameInputSwitchUpLeft: GameInputSwitchPosition = 8;
pub const GameInputSwitchUpRight: GameInputSwitchPosition = 2;
pub type GameInputSystemButtonCallback = Option<unsafe extern "system" fn(callbacktoken: u64, context: *const core::ffi::c_void, device: *mut core::ffi::c_void, timestamp: u64, currentbuttons: GameInputSystemButtons, previousbuttons: GameInputSystemButtons)>;
pub const GameInputSystemButtonGuide: GameInputSystemButtons = 1;
pub const GameInputSystemButtonNone: GameInputSystemButtons = 0;
pub const GameInputSystemButtonShare: GameInputSystemButtons = 2;
pub type GameInputSystemButtons = i32;
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
pub type GameInputTouchShape = i32;
pub const GameInputTouchShape1DIrregular: GameInputTouchShape = 3;
pub const GameInputTouchShape1DLinear: GameInputTouchShape = 1;
pub const GameInputTouchShape1DRadial: GameInputTouchShape = 2;
pub const GameInputTouchShape2DElliptical: GameInputTouchShape = 5;
pub const GameInputTouchShape2DIrregular: GameInputTouchShape = 6;
pub const GameInputTouchShape2DRectangular: GameInputTouchShape = 4;
pub const GameInputTouchShapePoint: GameInputTouchShape = 0;
pub const GameInputTouchShapeUnknown: GameInputTouchShape = -1;
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
pub const GameInputUiNavigationAccept: GameInputUiNavigationButtons = 4;
pub type GameInputUiNavigationButtons = i32;
pub const GameInputUiNavigationCancel: GameInputUiNavigationButtons = 8;
pub const GameInputUiNavigationContext1: GameInputUiNavigationButtons = 256;
pub const GameInputUiNavigationContext2: GameInputUiNavigationButtons = 512;
pub const GameInputUiNavigationContext3: GameInputUiNavigationButtons = 1024;
pub const GameInputUiNavigationContext4: GameInputUiNavigationButtons = 2048;
pub const GameInputUiNavigationDown: GameInputUiNavigationButtons = 32;
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
pub const GameInputUiNavigationLeft: GameInputUiNavigationButtons = 64;
pub const GameInputUiNavigationMenu: GameInputUiNavigationButtons = 1;
pub const GameInputUiNavigationNone: GameInputUiNavigationButtons = 0;
pub const GameInputUiNavigationPageDown: GameInputUiNavigationButtons = 8192;
pub const GameInputUiNavigationPageLeft: GameInputUiNavigationButtons = 16384;
pub const GameInputUiNavigationPageRight: GameInputUiNavigationButtons = 32768;
pub const GameInputUiNavigationPageUp: GameInputUiNavigationButtons = 4096;
pub const GameInputUiNavigationRight: GameInputUiNavigationButtons = 128;
pub const GameInputUiNavigationScrollDown: GameInputUiNavigationButtons = 131072;
pub const GameInputUiNavigationScrollLeft: GameInputUiNavigationButtons = 262144;
pub const GameInputUiNavigationScrollRight: GameInputUiNavigationButtons = 524288;
pub const GameInputUiNavigationScrollUp: GameInputUiNavigationButtons = 65536;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputUiNavigationState {
    pub buttons: GameInputUiNavigationButtons,
}
pub const GameInputUiNavigationUp: GameInputUiNavigationButtons = 16;
pub const GameInputUiNavigationView: GameInputUiNavigationButtons = 2;
pub const GameInputUnknownItemCollection: GameInputRawDeviceItemCollectionKind = -1;
pub const GameInputUnknownKeyboard: GameInputKeyboardKind = -1;
pub const GameInputUnknownSwitchKind: GameInputSwitchKind = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputUsage {
    pub page: u16,
    pub id: u16,
}
pub const GameInputUsageModifierItemCollection: GameInputRawDeviceItemCollectionKind = 6;
pub const GameInputUsageSwitchItemCollection: GameInputRawDeviceItemCollectionKind = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameInputVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub revision: u16,
}
pub const GameInputVolatileItem: GameInputRawDeviceReportItemFlags = 128;
pub const GameInputWraparoundItem: GameInputRawDeviceReportItemFlags = 8;
