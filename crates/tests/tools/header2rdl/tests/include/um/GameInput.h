// Copyright (c) Microsoft Corporation.  All rights reserved.

#if (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef __gameinput_h__
#define __gameinput_h__

#include <stdint.h>
#include <unknwn.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
extern "C" {
#endif

typedef enum GameInputKind
{
    GameInputKindUnknown          = 0x00000000,
    GameInputKindRawDeviceReport  = 0x00000001,
    GameInputKindControllerAxis   = 0x00000002,
    GameInputKindControllerButton = 0x00000004,
    GameInputKindControllerSwitch = 0x00000008,
    GameInputKindController       = 0x0000000E,
    GameInputKindKeyboard         = 0x00000010,
    GameInputKindMouse            = 0x00000020,
    GameInputKindTouch            = 0x00000100,
    GameInputKindMotion           = 0x00001000,
    GameInputKindArcadeStick      = 0x00010000,
    GameInputKindFlightStick      = 0x00020000,
    GameInputKindGamepad          = 0x00040000,
    GameInputKindRacingWheel      = 0x00080000,
    GameInputKindUiNavigation     = 0x01000000
} GameInputKind;

DEFINE_ENUM_FLAG_OPERATORS(GameInputKind);

typedef enum GameInputEnumerationKind
{
    GameInputNoEnumeration       = 0,
    GameInputAsyncEnumeration    = 1,
    GameInputBlockingEnumeration = 2
} GameInputEnumerationKind;

typedef enum GameInputFocusPolicy
{
    GameInputDefaultFocusPolicy             = 0x00000000,
    GameInputDisableBackgroundInput         = 0x00000001,
    GameInputExclusiveForegroundInput       = 0x00000002,
    GameInputDisableBackgroundGuideButton   = 0x00000004,
    GameInputExclusiveForegroundGuideButton = 0x00000008,
    GameInputDisableBackgroundShareButton   = 0x00000010,
    GameInputExclusiveForegroundShareButton = 0x00000020
} GameInputFocusPolicy;

DEFINE_ENUM_FLAG_OPERATORS(GameInputFocusPolicy);

typedef enum GameInputSwitchKind
{
    GameInputUnknownSwitchKind = -1,
    GameInput2WaySwitch        =  0,
    GameInput4WaySwitch        =  1,
    GameInput8WaySwitch        =  2
} GameInputSwitchKind;

typedef enum GameInputSwitchPosition
{
    GameInputSwitchCenter    = 0,
    GameInputSwitchUp        = 1,
    GameInputSwitchUpRight   = 2,
    GameInputSwitchRight     = 3,
    GameInputSwitchDownRight = 4,
    GameInputSwitchDown      = 5,
    GameInputSwitchDownLeft  = 6,
    GameInputSwitchLeft      = 7,
    GameInputSwitchUpLeft    = 8
} GameInputSwitchPosition;

typedef enum GameInputKeyboardKind
{
    GameInputUnknownKeyboard = -1,
    GameInputAnsiKeyboard    =  0,
    GameInputIsoKeyboard     =  1,
    GameInputKsKeyboard      =  2,
    GameInputAbntKeyboard    =  3,
    GameInputJisKeyboard     =  4
} GameInputKeyboardKind;

typedef enum GameInputMouseButtons
{
    GameInputMouseNone           = 0x00000000,
    GameInputMouseLeftButton     = 0x00000001,
    GameInputMouseRightButton    = 0x00000002,
    GameInputMouseMiddleButton   = 0x00000004,
    GameInputMouseButton4        = 0x00000008,
    GameInputMouseButton5        = 0x00000010,
    GameInputMouseWheelTiltLeft  = 0x00000020,
    GameInputMouseWheelTiltRight = 0x00000040
} GameInputMouseButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputMouseButtons)

typedef enum GameInputTouchShape
{
    GameInputTouchShapeUnknown       = -1,
    GameInputTouchShapePoint         =  0,
    GameInputTouchShape1DLinear      =  1,
    GameInputTouchShape1DRadial      =  2,
    GameInputTouchShape1DIrregular   =  3,
    GameInputTouchShape2DRectangular =  4,
    GameInputTouchShape2DElliptical  =  5,
    GameInputTouchShape2DIrregular   =  6
} GameInputTouchShape;

typedef enum GameInputMotionAccuracy
{
    GameInputMotionAccuracyUnknown = -1,
    GameInputMotionUnavailable     =  0,
    GameInputMotionUnreliable      =  1,
    GameInputMotionApproximate     =  2,
    GameInputMotionAccurate        =  3
} GameInputMotionAccuracy;

typedef enum GameInputArcadeStickButtons
{
    GameInputArcadeStickNone     = 0x00000000,
    GameInputArcadeStickMenu     = 0x00000001,
    GameInputArcadeStickView     = 0x00000002,
    GameInputArcadeStickUp       = 0x00000004,
    GameInputArcadeStickDown     = 0x00000008,
    GameInputArcadeStickLeft     = 0x00000010,
    GameInputArcadeStickRight    = 0x00000020,
    GameInputArcadeStickAction1  = 0x00000040,
    GameInputArcadeStickAction2  = 0x00000080,
    GameInputArcadeStickAction3  = 0x00000100,
    GameInputArcadeStickAction4  = 0x00000200,
    GameInputArcadeStickAction5  = 0x00000400,
    GameInputArcadeStickAction6  = 0x00000800,
    GameInputArcadeStickSpecial1 = 0x00001000,
    GameInputArcadeStickSpecial2 = 0x00002000
} GameInputArcadeStickButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputArcadeStickButtons)

typedef enum GameInputFlightStickButtons
{
    GameInputFlightStickNone          = 0x00000000,
    GameInputFlightStickMenu          = 0x00000001,
    GameInputFlightStickView          = 0x00000002,
    GameInputFlightStickFirePrimary   = 0x00000004,
    GameInputFlightStickFireSecondary = 0x00000008
} GameInputFlightStickButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputFlightStickButtons)

typedef enum GameInputGamepadButtons
{
    GameInputGamepadNone            = 0x00000000,
    GameInputGamepadMenu            = 0x00000001,
    GameInputGamepadView            = 0x00000002,
    GameInputGamepadA               = 0x00000004,
    GameInputGamepadB               = 0x00000008,
    GameInputGamepadX               = 0x00000010,
    GameInputGamepadY               = 0x00000020,
    GameInputGamepadDPadUp          = 0x00000040,
    GameInputGamepadDPadDown        = 0x00000080,
    GameInputGamepadDPadLeft        = 0x00000100,
    GameInputGamepadDPadRight       = 0x00000200,
    GameInputGamepadLeftShoulder    = 0x00000400,
    GameInputGamepadRightShoulder   = 0x00000800,
    GameInputGamepadLeftThumbstick  = 0x00001000,
    GameInputGamepadRightThumbstick = 0x00002000
} GameInputGamepadButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputGamepadButtons)

typedef enum GameInputRacingWheelButtons
{
    GameInputRacingWheelNone         = 0x00000000,
    GameInputRacingWheelMenu         = 0x00000001,
    GameInputRacingWheelView         = 0x00000002,
    GameInputRacingWheelPreviousGear = 0x00000004,
    GameInputRacingWheelNextGear     = 0x00000008,
    GameInputRacingWheelDpadUp       = 0x00000010,
    GameInputRacingWheelDpadDown     = 0x00000020,
    GameInputRacingWheelDpadLeft     = 0x00000040,
    GameInputRacingWheelDpadRight    = 0x00000080
} GameInputRacingWheelButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputRacingWheelButtons)

typedef enum GameInputUiNavigationButtons
{
    GameInputUiNavigationNone        = 0x00000000,
    GameInputUiNavigationMenu        = 0x00000001,
    GameInputUiNavigationView        = 0x00000002,
    GameInputUiNavigationAccept      = 0x00000004,
    GameInputUiNavigationCancel      = 0x00000008,
    GameInputUiNavigationUp          = 0x00000010,
    GameInputUiNavigationDown        = 0x00000020,
    GameInputUiNavigationLeft        = 0x00000040,
    GameInputUiNavigationRight       = 0x00000080,
    GameInputUiNavigationContext1    = 0x00000100,
    GameInputUiNavigationContext2    = 0x00000200,
    GameInputUiNavigationContext3    = 0x00000400,
    GameInputUiNavigationContext4    = 0x00000800,
    GameInputUiNavigationPageUp      = 0x00001000,
    GameInputUiNavigationPageDown    = 0x00002000,
    GameInputUiNavigationPageLeft    = 0x00004000,
    GameInputUiNavigationPageRight   = 0x00008000,
    GameInputUiNavigationScrollUp    = 0x00010000,
    GameInputUiNavigationScrollDown  = 0x00020000,
    GameInputUiNavigationScrollLeft  = 0x00040000,
    GameInputUiNavigationScrollRight = 0x00080000
} GameInputUiNavigationButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputUiNavigationButtons)

typedef enum GameInputSystemButtons
{
    GameInputSystemButtonNone  = 0x00000000,
    GameInputSystemButtonGuide = 0x00000001,
    GameInputSystemButtonShare = 0x00000002
} GameInputSystemButtons;

DEFINE_ENUM_FLAG_OPERATORS(GameInputSystemButtons)

typedef enum GameInputDeviceStatus
{
    GameInputDeviceNoStatus      = 0x00000000,
    GameInputDeviceConnected     = 0x00000001,
    GameInputDeviceInputEnabled  = 0x00000002,
    GameInputDeviceOutputEnabled = 0x00000004,
    GameInputDeviceRawIoEnabled  = 0x00000008,
    GameInputDeviceAudioCapture  = 0x00000010,
    GameInputDeviceAudioRender   = 0x00000020,
    GameInputDeviceSynchronized  = 0x00000040,
    GameInputDeviceWireless      = 0x00000080,
    GameInputDeviceUserIdle      = 0x00100000,
    GameInputDeviceAnyStatus     = 0x00FFFFFF
} GameInputDeviceStatus;

DEFINE_ENUM_FLAG_OPERATORS(GameInputDeviceStatus);

typedef enum GameInputBatteryStatus
{
    GameInputBatteryUnknown     = -1,
    GameInputBatteryNotPresent  =  0,
    GameInputBatteryDischarging =  1,
    GameInputBatteryIdle        =  2,
    GameInputBatteryCharging    =  3
} GameInputBatteryStatus;

typedef enum GameInputDeviceFamily
{
    GameInputFamilyVirtual   = -1,
    GameInputFamilyAggregate =  0,
    GameInputFamilyXboxOne   =  1,
    GameInputFamilyXbox360   =  2,
    GameInputFamilyHid       =  3,
    GameInputFamilyI8042     =  4
} GameInputDeviceFamily;

typedef enum GameInputDeviceCapabilities
{
    GameInputDeviceCapabilityNone            = 0x00000000,
    GameInputDeviceCapabilityAudio           = 0x00000001,
    GameInputDeviceCapabilityPluginModule    = 0x00000002,
    GameInputDeviceCapabilityPowerOff        = 0x00000004,
    GameInputDeviceCapabilitySynchronization = 0x00000008,
    GameInputDeviceCapabilityWireless        = 0x00000010
} GameInputDeviceCapabilities;

DEFINE_ENUM_FLAG_OPERATORS(GameInputDeviceCapabilities)

typedef enum GameInputRawDeviceReportKind
{
    GameInputRawInputReport   = 0,
    GameInputRawOutputReport  = 1,
    GameInputRawFeatureReport = 2
} GameInputRawDeviceReportKind;

typedef enum GameInputRawDeviceReportItemFlags
{
    GameInputDefaultItem    = 0x00000000,
    GameInputConstantItem   = 0x00000001,
    GameInputArrayItem      = 0x00000002,
    GameInputRelativeItem   = 0x00000004,
    GameInputWraparoundItem = 0x00000008,
    GameInputNonlinearItem  = 0x00000010,
    GameInputStableItem     = 0x00000020,
    GameInputNullableItem   = 0x00000040,
    GameInputVolatileItem   = 0x00000080,
    GameInputBufferedItem   = 0x00000100
} GameInputRawDeviceReportItemFlags;

DEFINE_ENUM_FLAG_OPERATORS(GameInputRawDeviceReportItemFlags)

typedef enum GameInputRawDeviceItemCollectionKind
{
    GameInputUnknownItemCollection       = -1,
    GameInputPhysicalItemCollection      =  0,
    GameInputApplicationItemCollection   =  1,
    GameInputLogicalItemCollection       =  2,
    GameInputReportItemCollection        =  3,
    GameInputNamedArrayItemCollection    =  4,
    GameInputUsageSwitchItemCollection   =  5,
    GameInputUsageModifierItemCollection =  6
} GameInputRawDeviceItemCollectionKind;

typedef enum GameInputRawDevicePhysicalUnitKind
{
    GameInputPhysicalUnitUnknown             = -1,
    GameInputPhysicalUnitNone                =  0,
    GameInputPhysicalUnitTime                =  1,
    GameInputPhysicalUnitFrequency           =  2,
    GameInputPhysicalUnitLength              =  3,
    GameInputPhysicalUnitVelocity            =  4,
    GameInputPhysicalUnitAcceleration        =  5,
    GameInputPhysicalUnitMass                =  6,
    GameInputPhysicalUnitMomentum            =  7,
    GameInputPhysicalUnitForce               =  8,
    GameInputPhysicalUnitPressure            =  9,
    GameInputPhysicalUnitAngle               = 10,
    GameInputPhysicalUnitAngularVelocity     = 11,
    GameInputPhysicalUnitAngularAcceleration = 12,
    GameInputPhysicalUnitAngularMass         = 13,
    GameInputPhysicalUnitAngularMomentum     = 14,
    GameInputPhysicalUnitAngularTorque       = 15,
    GameInputPhysicalUnitElectricCurrent     = 16,
    GameInputPhysicalUnitElectricCharge      = 17,
    GameInputPhysicalUnitElectricPotential   = 18,
    GameInputPhysicalUnitEnergy              = 19,
    GameInputPhysicalUnitPower               = 20,
    GameInputPhysicalUnitTemperature         = 21,
    GameInputPhysicalUnitLuminousIntensity   = 22,
    GameInputPhysicalUnitLuminousFlux        = 23,
    GameInputPhysicalUnitIlluminance         = 24
} GameInputRawDevicePhysicalUnitKind;

typedef enum GameInputLabel
{
    GameInputLabelUnknown                  =  -1,
    GameInputLabelNone                     =   0,
    GameInputLabelXboxGuide                =   1,
    GameInputLabelXboxBack                 =   2,
    GameInputLabelXboxStart                =   3,
    GameInputLabelXboxMenu                 =   4,
    GameInputLabelXboxView                 =   5,
    GameInputLabelXboxA                    =   7,
    GameInputLabelXboxB                    =   8,
    GameInputLabelXboxX                    =   9,
    GameInputLabelXboxY                    =  10,
    GameInputLabelXboxDPadUp               =  11,
    GameInputLabelXboxDPadDown             =  12,
    GameInputLabelXboxDPadLeft             =  13,
    GameInputLabelXboxDPadRight            =  14,
    GameInputLabelXboxLeftShoulder         =  15,
    GameInputLabelXboxLeftTrigger          =  16,
    GameInputLabelXboxLeftStickButton      =  17,
    GameInputLabelXboxRightShoulder        =  18,
    GameInputLabelXboxRightTrigger         =  19,
    GameInputLabelXboxRightStickButton     =  20,
    GameInputLabelXboxPaddle1              =  21,
    GameInputLabelXboxPaddle2              =  22,
    GameInputLabelXboxPaddle3              =  23,
    GameInputLabelXboxPaddle4              =  24,
    GameInputLabelLetterA                  =  25,
    GameInputLabelLetterB                  =  26,
    GameInputLabelLetterC                  =  27,
    GameInputLabelLetterD                  =  28,
    GameInputLabelLetterE                  =  29,
    GameInputLabelLetterF                  =  30,
    GameInputLabelLetterG                  =  31,
    GameInputLabelLetterH                  =  32,
    GameInputLabelLetterI                  =  33,
    GameInputLabelLetterJ                  =  34,
    GameInputLabelLetterK                  =  35,
    GameInputLabelLetterL                  =  36,
    GameInputLabelLetterM                  =  37,
    GameInputLabelLetterN                  =  38,
    GameInputLabelLetterO                  =  39,
    GameInputLabelLetterP                  =  40,
    GameInputLabelLetterQ                  =  41,
    GameInputLabelLetterR                  =  42,
    GameInputLabelLetterS                  =  43,
    GameInputLabelLetterT                  =  44,
    GameInputLabelLetterU                  =  45,
    GameInputLabelLetterV                  =  46,
    GameInputLabelLetterW                  =  47,
    GameInputLabelLetterX                  =  48,
    GameInputLabelLetterY                  =  49,
    GameInputLabelLetterZ                  =  50,
    GameInputLabelNumber0                  =  51,
    GameInputLabelNumber1                  =  52,
    GameInputLabelNumber2                  =  53,
    GameInputLabelNumber3                  =  54,
    GameInputLabelNumber4                  =  55,
    GameInputLabelNumber5                  =  56,
    GameInputLabelNumber6                  =  57,
    GameInputLabelNumber7                  =  58,
    GameInputLabelNumber8                  =  59,
    GameInputLabelNumber9                  =  60,
    GameInputLabelArrowUp                  =  61,
    GameInputLabelArrowUpRight             =  62,
    GameInputLabelArrowRight               =  63,
    GameInputLabelArrowDownRight           =  64,
    GameInputLabelArrowDown                =  65,
    GameInputLabelArrowDownLLeft           =  66,
    GameInputLabelArrowLeft                =  67,
    GameInputLabelArrowUpLeft              =  68,
    GameInputLabelArrowUpDown              =  69,
    GameInputLabelArrowLeftRight           =  70,
    GameInputLabelArrowUpDownLeftRight     =  71,
    GameInputLabelArrowClockwise           =  72,
    GameInputLabelArrowCounterClockwise    =  73,
    GameInputLabelArrowReturn              =  74,
    GameInputLabelIconBranding             =  75,
    GameInputLabelIconHome                 =  76,
    GameInputLabelIconMenu                 =  77,
    GameInputLabelIconCross                =  78,
    GameInputLabelIconCircle               =  79,
    GameInputLabelIconSquare               =  80,
    GameInputLabelIconTriangle             =  81,
    GameInputLabelIconStar                 =  82,
    GameInputLabelIconDPadUp               =  83,
    GameInputLabelIconDPadDown             =  84,
    GameInputLabelIconDPadLeft             =  85,
    GameInputLabelIconDPadRight            =  86,
    GameInputLabelIconDialClockwise        =  87,
    GameInputLabelIconDialCounterClockwise =  88,
    GameInputLabelIconSliderLeftRight      =  89,
    GameInputLabelIconSliderUpDown         =  90,
    GameInputLabelIconWheelUpDown          =  91,
    GameInputLabelIconPlus                 =  92,
    GameInputLabelIconMinus                =  93,
    GameInputLabelIconSuspension           =  94,
    GameInputLabelHome                     =  95,
    GameInputLabelGuide                    =  96,
    GameInputLabelMode                     =  97,
    GameInputLabelSelect                   =  98,
    GameInputLabelMenu                     =  99,
    GameInputLabelView                     = 100,
    GameInputLabelBack                     = 101,
    GameInputLabelStart                    = 102,
    GameInputLabelOptions                  = 103,
    GameInputLabelShare                    = 104,
    GameInputLabelUp                       = 105,
    GameInputLabelDown                     = 106,
    GameInputLabelLeft                     = 107,
    GameInputLabelRight                    = 108,
    GameInputLabelLB                       = 109,
    GameInputLabelLT                       = 110,
    GameInputLabelLSB                      = 111,
    GameInputLabelL1                       = 112,
    GameInputLabelL2                       = 113,
    GameInputLabelL3                       = 114,
    GameInputLabelRB                       = 115,
    GameInputLabelRT                       = 116,
    GameInputLabelRSB                      = 117,
    GameInputLabelR1                       = 118,
    GameInputLabelR2                       = 119,
    GameInputLabelR3                       = 120,
    GameInputLabelP1                       = 121,
    GameInputLabelP2                       = 122,
    GameInputLabelP3                       = 123,
    GameInputLabelP4                       = 124
} GameInputLabel;

typedef enum GameInputLocation
{
    GameInputLocationUnknown  = -1,
    GameInputLocationChassis  =  0,
    GameInputLocationDisplay  =  1,
    GameInputLocationAxis     =  2,
    GameInputLocationButton   =  3,
    GameInputLocationSwitch   =  4,
    GameInputLocationKey      =  5,
    GameInputLocationTouchPad =  6
} GameInputLocation;

typedef enum GameInputFeedbackAxes
{
    GameInputFeedbackAxisNone     = 0x00000000,
    GameInputFeedbackAxisLinearX  = 0x00000001,
    GameInputFeedbackAxisLinearY  = 0x00000002,
    GameInputFeedbackAxisLinearZ  = 0x00000004,
    GameInputFeedbackAxisAngularX = 0x00000008,
    GameInputFeedbackAxisAngularY = 0x00000010,
    GameInputFeedbackAxisAngularZ = 0x00000020,
    GameInputFeedbackAxisNormal   = 0x00000040
} GameInputFeedbackAxes;

DEFINE_ENUM_FLAG_OPERATORS(GameInputFeedbackAxes)

typedef enum GameInputFeedbackEffectState
{
    GameInputFeedbackStopped = 0,
    GameInputFeedbackRunning = 1,
    GameInputFeedbackPaused  = 2
} GameInputFeedbackEffectState;

typedef enum GameInputForceFeedbackEffectKind
{
    GameInputForceFeedbackConstant         = 0,
    GameInputForceFeedbackRamp             = 1,
    GameInputForceFeedbackSineWave         = 2,
    GameInputForceFeedbackSquareWave       = 3,
    GameInputForceFeedbackTriangleWave     = 4,
    GameInputForceFeedbackSawtoothUpWave   = 5,
    GameInputForceFeedbackSawtoothDownWave = 6,
    GameInputForceFeedbackSpring           = 7,
    GameInputForceFeedbackFriction         = 8,
    GameInputForceFeedbackDamper           = 9,
    GameInputForceFeedbackInertia          = 10
} GameInputForceFeedbackEffectKind;

typedef enum GameInputRumbleMotors
{
    GameInputRumbleNone          = 0x00000000,
    GameInputRumbleLowFrequency  = 0x00000001,
    GameInputRumbleHighFrequency = 0x00000002,
    GameInputRumbleLeftTrigger   = 0x00000004,
    GameInputRumbleRightTrigger  = 0x00000008
} GameInputRumbleMotors;

DEFINE_ENUM_FLAG_OPERATORS(GameInputRumbleMotors)

typedef interface IGameInput IGameInput;
typedef interface IGameInputReading IGameInputReading;
typedef interface IGameInputDevice IGameInputDevice;
typedef interface IGameInputDispatcher IGameInputDispatcher;
typedef interface IGameInputForceFeedbackEffect IGameInputForceFeedbackEffect;
typedef interface IGameInputRawDeviceReport IGameInputRawDeviceReport;

typedef uint64_t GameInputCallbackToken;

#define GAMEINPUT_CURRENT_CALLBACK_TOKEN_VALUE 0xFFFFFFFFFFFFFFFFULL
#define GAMEINPUT_INVALID_CALLBACK_TOKEN_VALUE 0x0000000000000000ULL

typedef void (CALLBACK * GameInputReadingCallback)(
    _In_ GameInputCallbackToken callbackToken,
    _In_ void * context,
    _In_ IGameInputReading * reading,
    _In_ bool hasOverrunOccurred);

typedef void (CALLBACK * GameInputDeviceCallback)(
    _In_ GameInputCallbackToken callbackToken,
    _In_ void * context,
    _In_ IGameInputDevice * device,
    _In_ uint64_t timestamp,
    _In_ GameInputDeviceStatus currentStatus,
    _In_ GameInputDeviceStatus previousStatus);

typedef void (CALLBACK * GameInputSystemButtonCallback)(
    _In_ GameInputCallbackToken callbackToken,
    _In_ void * context,
    _In_ IGameInputDevice * device,
    _In_ uint64_t timestamp,
    _In_ GameInputSystemButtons currentButtons,
    _In_ GameInputSystemButtons previousButtons);

typedef void (CALLBACK * GameInputKeyboardLayoutCallback)(
    _In_ GameInputCallbackToken callbackToken,
    _In_ void * context,
    _In_ IGameInputDevice * device,
    _In_ uint64_t timestamp,
    _In_ uint32_t currentLayout,
    _In_ uint32_t previousLayout);

typedef struct GameInputKeyState
{
    uint32_t scanCode;
    uint32_t codePoint;
    uint8_t virtualKey;
    bool isDeadKey;
} GameInputKeyState;

typedef struct GameInputMouseState
{
    GameInputMouseButtons buttons;
    int64_t positionX;
    int64_t positionY;
    int64_t wheelX;
    int64_t wheelY;
} GameInputMouseState;

typedef struct GameInputTouchState
{
    uint64_t touchId;
    uint32_t sensorIndex;
    float positionX;
    float positionY;
    float pressure;
    float proximity;
    float contactRectTop;
    float contactRectLeft;
    float contactRectRight;
    float contactRectBottom;
} GameInputTouchState;

typedef struct GameInputMotionState
{
    float accelerationX;
    float accelerationY;
    float accelerationZ;
    float angularVelocityX;
    float angularVelocityY;
    float angularVelocityZ;
    float magneticFieldX;
    float magneticFieldY;
    float magneticFieldZ;
    float orientationW;
    float orientationX;
    float orientationY;
    float orientationZ;
    GameInputMotionAccuracy accelerometerAccuracy;
    GameInputMotionAccuracy gyroscopeAccuracy;
    GameInputMotionAccuracy magnetometerAccuracy;
    GameInputMotionAccuracy orientationAccuracy;
} GameInputMotionState;

typedef struct GameInputArcadeStickState
{
    GameInputArcadeStickButtons buttons;
} GameInputArcadeStickState;

typedef struct GameInputFlightStickState
{
    GameInputFlightStickButtons buttons;
    GameInputSwitchPosition hatSwitch;
    float roll;
    float pitch;
    float yaw;
    float throttle;
} GameInputFlightStickState;

typedef struct GameInputGamepadState
{
    GameInputGamepadButtons buttons;
    float leftTrigger;
    float rightTrigger;
    float leftThumbstickX;
    float leftThumbstickY;
    float rightThumbstickX;
    float rightThumbstickY;
} GameInputGamepadState;

typedef struct GameInputRacingWheelState
{
    GameInputRacingWheelButtons buttons;
    int32_t patternShifterGear;
    float wheel;
    float throttle;
    float brake;
    float clutch;
    float handbrake;
} GameInputRacingWheelState;

typedef struct GameInputUiNavigationState
{
    GameInputUiNavigationButtons buttons;
} GameInputUiNavigationState;

typedef struct GameInputBatteryState
{
    float chargeRate;
    float maxChargeRate;
    float remainingCapacity;
    float fullChargeCapacity;
    GameInputBatteryStatus status;
} GameInputBatteryState;

typedef struct GameInputString
{
    uint32_t sizeInBytes;
    uint32_t codePointCount;
    _Field_z_ char const * data;
} GameInputString;

typedef struct GameInputUsage
{
    uint16_t page;
    uint16_t id;
} GameInputUsage;

typedef struct GameInputVersion
{
    uint16_t major;
    uint16_t minor;
    uint16_t build;
    uint16_t revision;
} GameInputVersion;

typedef struct GameInputRawDeviceItemCollectionInfo
{
    GameInputRawDeviceItemCollectionKind kind;
    uint32_t childCount;
    uint32_t siblingCount;
    uint32_t usageCount;
    _Field_size_full_(usageCount) GameInputUsage const * usages;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * parent;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * firstSibling;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * previousSibling;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * nextSibling;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * lastSibling;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * firstChild;
    _Field_size_full_opt_(1) struct GameInputRawDeviceItemCollectionInfo const * lastChild;
} GameInputRawDeviceItemCollectionInfo;

typedef struct GameInputRawDeviceReportItemInfo
{
    uint32_t bitOffset;
    uint32_t bitSize;
    int64_t logicalMin;
    int64_t logicalMax;
    double physicalMin;
    double physicalMax;
    GameInputRawDevicePhysicalUnitKind physicalUnits;
    uint32_t rawPhysicalUnits;
    int32_t rawPhysicalUnitsExponent;
    GameInputRawDeviceReportItemFlags flags;
    uint32_t usageCount;
    _Field_size_full_(usageCount) GameInputUsage const * usages;
    _Field_size_full_(1) GameInputRawDeviceItemCollectionInfo const * collection;
    _Field_size_full_opt_(1) GameInputString const * itemString;
} GameInputRawDeviceReportItemInfo;

typedef struct GameInputRawDeviceReportInfo
{
    GameInputRawDeviceReportKind kind;
    uint32_t id;
    uint32_t size;
    uint32_t itemCount;
    _Field_size_full_opt_(itemCount) GameInputRawDeviceReportItemInfo const * items;
} GameInputRawDeviceReportInfo;

typedef struct GameInputControllerAxisInfo
{
    GameInputKind mappedInputKinds;
    GameInputLabel label;
    bool isContinuous;
    bool isNonlinear;
    bool isQuantized;
    bool hasRestValue;
    float restValue;
    uint64_t resolution;
    uint16_t legacyDInputIndex;
    uint16_t legacyHidIndex;
    uint32_t rawReportIndex;
    _Field_size_full_(1) GameInputRawDeviceReportInfo const * inputReport;
    _Field_size_full_(1) GameInputRawDeviceReportItemInfo const * inputReportItem;
} GameInputControllerAxisInfo;

typedef struct GameInputControllerButtonInfo
{
    GameInputKind mappedInputKinds;
    GameInputLabel label;
    uint16_t legacyDInputIndex;
    uint16_t legacyHidIndex;
    uint32_t rawReportIndex;
    _Field_size_full_(1) GameInputRawDeviceReportInfo const * inputReport;
    _Field_size_full_(1) GameInputRawDeviceReportItemInfo const * inputReportItem;
} GameInputControllerButtonInfo;

typedef struct GameInputControllerSwitchInfo
{
    GameInputKind mappedInputKinds;
    GameInputLabel label;
    GameInputLabel positionLabels[9];
    GameInputSwitchKind kind;
    uint16_t legacyDInputIndex;
    uint16_t legacyHidIndex;
    uint32_t rawReportIndex;
    _Field_size_full_(1) GameInputRawDeviceReportInfo const * inputReport;
    _Field_size_full_(1) GameInputRawDeviceReportItemInfo const * inputReportItem;
} GameInputControllerSwitchInfo;

typedef struct GameInputKeyboardInfo
{
    GameInputKeyboardKind kind;
    uint32_t layout;
    uint32_t keyCount;
    uint32_t functionKeyCount;
    uint32_t maxSimultaneousKeys;
    uint32_t platformType;
    uint32_t platformSubtype;
    _Field_size_full_opt_(1) GameInputString const * nativeLanguage;
} GameInputKeyboardInfo;

typedef struct GameInputMouseInfo
{
    GameInputMouseButtons supportedButtons;
    uint32_t sampleRate;
    uint32_t sensorDpi;
    bool hasWheelX;
    bool hasWheelY;
} GameInputMouseInfo;

typedef struct GameInputTouchSensorInfo
{
    GameInputKind mappedInputKinds;
    GameInputLabel label;
    GameInputLocation location;
    uint32_t locationId;
    uint64_t resolutionX;
    uint64_t resolutionY;
    GameInputTouchShape shape;
    float aspectRatio;
    float orientation;
    float physicalWidth;
    float physicalHeight;
    float maxPressure;
    float maxProximity;
    uint32_t maxTouchPoints;
} GameInputTouchSensorInfo;

typedef struct GameInputMotionInfo
{
    float maxAcceleration;
    float maxAngularVelocity;
    float maxMagneticFieldStrength;
} GameInputMotionInfo;

typedef struct GameInputArcadeStickInfo
{
    GameInputLabel menuButtonLabel;
    GameInputLabel viewButtonLabel;
    GameInputLabel stickUpLabel;
    GameInputLabel stickDownLabel;
    GameInputLabel stickLeftLabel;
    GameInputLabel stickRightLabel;
    GameInputLabel actionButton1Label;
    GameInputLabel actionButton2Label;
    GameInputLabel actionButton3Label;
    GameInputLabel actionButton4Label;
    GameInputLabel actionButton5Label;
    GameInputLabel actionButton6Label;
    GameInputLabel specialButton1Label;
    GameInputLabel specialButton2Label;
} GameInputArcadeStickInfo;

typedef struct GameInputFlightStickInfo
{
    GameInputLabel menuButtonLabel;
    GameInputLabel viewButtonLabel;
    GameInputLabel firePrimaryButtonLabel;
    GameInputLabel fireSecondaryButtonLabel;
    GameInputSwitchKind hatSwitchKind;
} GameInputFlightStickInfo;

typedef struct GameInputGamepadInfo
{
    GameInputLabel menuButtonLabel;
    GameInputLabel viewButtonLabel;
    GameInputLabel aButtonLabel;
    GameInputLabel bButtonLabel;
    GameInputLabel xButtonLabel;
    GameInputLabel yButtonLabel;
    GameInputLabel dpadUpLabel;
    GameInputLabel dpadDownLabel;
    GameInputLabel dpadLeftLabel;
    GameInputLabel dpadRightLabel;
    GameInputLabel leftShoulderButtonLabel;
    GameInputLabel rightShoulderButtonLabel;
    GameInputLabel leftThumbstickButtonLabel;
    GameInputLabel rightThumbstickButtonLabel;
} GameInputGamepadInfo;

typedef struct GameInputRacingWheelInfo
{
    GameInputLabel menuButtonLabel;
    GameInputLabel viewButtonLabel;
    GameInputLabel previousGearButtonLabel;
    GameInputLabel nextGearButtonLabel;
    GameInputLabel dpadUpLabel;
    GameInputLabel dpadDownLabel;
    GameInputLabel dpadLeftLabel;
    GameInputLabel dpadRightLabel;
    bool hasClutch;
    bool hasHandbrake;
    bool hasPatternShifter;
    int32_t minPatternShifterGear;
    int32_t maxPatternShifterGear;
    float maxWheelAngle;
} GameInputRacingWheelInfo;

typedef struct GameInputUiNavigationInfo
{
    GameInputLabel menuButtonLabel;
    GameInputLabel viewButtonLabel;
    GameInputLabel acceptButtonLabel;
    GameInputLabel cancelButtonLabel;
    GameInputLabel upButtonLabel;
    GameInputLabel downButtonLabel;
    GameInputLabel leftButtonLabel;
    GameInputLabel rightButtonLabel;
    GameInputLabel contextButton1Label;
    GameInputLabel contextButton2Label;
    GameInputLabel contextButton3Label;
    GameInputLabel contextButton4Label;
    GameInputLabel pageUpButtonLabel;
    GameInputLabel pageDownButtonLabel;
    GameInputLabel pageLeftButtonLabel;
    GameInputLabel pageRightButtonLabel;
    GameInputLabel scrollUpButtonLabel;
    GameInputLabel scrollDownButtonLabel;
    GameInputLabel scrollLeftButtonLabel;
    GameInputLabel scrollRightButtonLabel;
    GameInputLabel guideButtonLabel;
} GameInputUiNavigationInfo;

typedef struct GameInputForceFeedbackMotorInfo
{
    GameInputFeedbackAxes supportedAxes;
    GameInputLocation location;
    uint32_t locationId;
    uint32_t maxSimultaneousEffects;
    bool isConstantEffectSupported;
    bool isRampEffectSupported;
    bool isSineWaveEffectSupported;
    bool isSquareWaveEffectSupported;
    bool isTriangleWaveEffectSupported;
    bool isSawtoothUpWaveEffectSupported;
    bool isSawtoothDownWaveEffectSupported;
    bool isSpringEffectSupported;
    bool isFrictionEffectSupported;
    bool isDamperEffectSupported;
    bool isInertiaEffectSupported;
} GameInputForceFeedbackMotorInfo;

typedef struct GameInputHapticWaveformInfo
{
    GameInputUsage usage;
    bool isDurationSupported;
    bool isIntensitySupported;
    bool isRepeatSupported;
    bool isRepeatDelaySupported;
    uint64_t defaultDuration;
} GameInputHapticWaveformInfo;

typedef struct GameInputHapticFeedbackMotorInfo
{
    GameInputRumbleMotors mappedRumbleMotors;
    GameInputLocation location;
    uint32_t locationId;
    uint32_t waveformCount;
    _Field_size_full_(waveformCount) GameInputHapticWaveformInfo const * waveformInfo;
} GameInputHapticFeedbackMotorInfo;

typedef struct GameInputDeviceInfo
{
    uint32_t infoSize;
    uint16_t vendorId;
    uint16_t productId;
    uint16_t revisionNumber;
    uint8_t interfaceNumber;
    uint8_t collectionNumber;
    GameInputUsage usage;
    GameInputVersion hardwareVersion;
    GameInputVersion firmwareVersion;
    APP_LOCAL_DEVICE_ID deviceId;
    APP_LOCAL_DEVICE_ID deviceRootId;
    GameInputDeviceFamily deviceFamily;
    GameInputDeviceCapabilities capabilities;
    GameInputKind supportedInput;
    GameInputRumbleMotors supportedRumbleMotors;
    uint32_t inputReportCount;
    uint32_t outputReportCount;
    uint32_t featureReportCount;
    uint32_t controllerAxisCount;
    uint32_t controllerButtonCount;
    uint32_t controllerSwitchCount;
    uint32_t touchPointCount;
    uint32_t touchSensorCount;
    uint32_t forceFeedbackMotorCount;
    uint32_t hapticFeedbackMotorCount;
    uint32_t deviceStringCount;
    uint32_t deviceDescriptorSize;
    _Field_size_full_opt_(inputReportCount) GameInputRawDeviceReportInfo const * inputReportInfo;
    _Field_size_full_opt_(outputReportCount) GameInputRawDeviceReportInfo const * outputReportInfo;
    _Field_size_full_opt_(featureReportCount) GameInputRawDeviceReportInfo const * featureReportInfo;
    _Field_size_full_opt_(controllerAxisCount) GameInputControllerAxisInfo const * controllerAxisInfo;
    _Field_size_full_opt_(controllerButtonCount) GameInputControllerButtonInfo const * controllerButtonInfo;
    _Field_size_full_opt_(controllerSwitchCount) GameInputControllerSwitchInfo const * controllerSwitchInfo;
    _Field_size_full_opt_(1) GameInputKeyboardInfo const * keyboardInfo;
    _Field_size_full_opt_(1) GameInputMouseInfo const * mouseInfo;
    _Field_size_full_opt_(touchSensorCount) GameInputTouchSensorInfo const * touchSensorInfo;
    _Field_size_full_opt_(1) GameInputMotionInfo const * motionInfo;
    _Field_size_full_opt_(1) GameInputArcadeStickInfo const * arcadeStickInfo;
    _Field_size_full_opt_(1) GameInputFlightStickInfo const * flightStickInfo;
    _Field_size_full_opt_(1) GameInputGamepadInfo const * gamepadInfo;
    _Field_size_full_opt_(1) GameInputRacingWheelInfo const * racingWheelInfo;
    _Field_size_full_opt_(1) GameInputUiNavigationInfo const * uiNavigationInfo;
    _Field_size_full_opt_(forceFeedbackMotorCount) GameInputForceFeedbackMotorInfo const * forceFeedbackMotorInfo;
    _Field_size_full_opt_(hapticFeedbackMotorCount) GameInputHapticFeedbackMotorInfo const * hapticFeedbackMotorInfo;
    _Field_size_full_opt_(1) GameInputString const * displayName;
    _Field_size_full_opt_(deviceStringCount) GameInputString const * deviceStrings;
    _Field_size_bytes_full_opt_(deviceDescriptorSize) void const * deviceDescriptorData;
    GameInputSystemButtons supportedSystemButtons;
} GameInputDeviceInfo;

typedef struct GameInputForceFeedbackEnvelope
{
    uint64_t attackDuration;
    uint64_t sustainDuration;
    uint64_t releaseDuration;
    float attackGain;
    float sustainGain;
    float releaseGain;
    uint32_t playCount;
    uint64_t repeatDelay;
} GameInputForceFeedbackEnvelope;

typedef struct GameInputForceFeedbackMagnitude
{
    float linearX;
    float linearY;
    float linearZ;
    float angularX;
    float angularY;
    float angularZ;
    float normal;
} GameInputForceFeedbackMagnitude;

typedef struct GameInputForceFeedbackConditionParams
{
    GameInputForceFeedbackMagnitude magnitude;
    float positiveCoefficient;
    float negativeCoefficient;
    float maxPositiveMagnitude;
    float maxNegativeMagnitude;
    float deadZone;
    float bias;
} GameInputForceFeedbackConditionParams;

typedef struct GameInputForceFeedbackConstantParams
{
    GameInputForceFeedbackEnvelope envelope;
    GameInputForceFeedbackMagnitude magnitude;
} GameInputForceFeedbackConstantParams;

typedef struct GameInputForceFeedbackPeriodicParams
{
    GameInputForceFeedbackEnvelope envelope;
    GameInputForceFeedbackMagnitude magnitude;
    float frequency;
    float phase;
    float bias;
} GameInputForceFeedbackPeriodicParams;

typedef struct GameInputForceFeedbackRampParams
{
    GameInputForceFeedbackEnvelope envelope;
    GameInputForceFeedbackMagnitude startMagnitude;
    GameInputForceFeedbackMagnitude endMagnitude;
} GameInputForceFeedbackRampParams;

typedef struct GameInputForceFeedbackParams
{
    GameInputForceFeedbackEffectKind kind;
    union
    {
        GameInputForceFeedbackConstantParams constant;
        GameInputForceFeedbackRampParams ramp;
        GameInputForceFeedbackPeriodicParams sineWave;
        GameInputForceFeedbackPeriodicParams squareWave;
        GameInputForceFeedbackPeriodicParams triangleWave;
        GameInputForceFeedbackPeriodicParams sawtoothUpWave;
        GameInputForceFeedbackPeriodicParams sawtoothDownWave;
        GameInputForceFeedbackConditionParams spring;
        GameInputForceFeedbackConditionParams friction;
        GameInputForceFeedbackConditionParams damper;
        GameInputForceFeedbackConditionParams inertia;
    } data;
} GameInputForceFeedbackParams;

typedef struct GameInputHapticFeedbackParams
{
    uint32_t waveformIndex;
    uint64_t duration;
    float intensity;
    uint32_t playCount;
    uint64_t repeatDelay;
} GameInputHapticFeedbackParams;

typedef struct GameInputRumbleParams
{
    float lowFrequency;
    float highFrequency;
    float leftTrigger;
    float rightTrigger;
} GameInputRumbleParams;

#undef INTERFACE
#define INTERFACE IGameInput
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "11BE2A7E-4254-445A-9C09-FFC40F006918")
{
    _Must_inspect_result_ IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(uint64_t, GetCurrentTimestamp)(THIS) PURE;

    _Must_inspect_result_ IFACEMETHOD(GetCurrentReading)(THIS_
        _In_ GameInputKind inputKind,
        _In_opt_ IGameInputDevice * device,
        _COM_Outptr_ IGameInputReading ** reading) PURE;

    _Must_inspect_result_ IFACEMETHOD(GetNextReading)(THIS_
        _In_ IGameInputReading * referenceReading,
        _In_ GameInputKind inputKind,
        _In_opt_ IGameInputDevice * device,
        _COM_Outptr_ IGameInputReading ** reading) PURE;

    _Must_inspect_result_ IFACEMETHOD(GetPreviousReading)(THIS_
        _In_ IGameInputReading * referenceReading,
        _In_ GameInputKind inputKind,
        _In_opt_ IGameInputDevice * device,
        _COM_Outptr_ IGameInputReading ** reading) PURE;

    _Must_inspect_result_ IFACEMETHOD(GetTemporalReading)(THIS_
        _In_ uint64_t timestamp,
        _In_ IGameInputDevice * device,
        _COM_Outptr_ IGameInputReading ** reading) PURE;

    IFACEMETHOD(RegisterReadingCallback)(THIS_
        _In_opt_ IGameInputDevice * device,
        _In_ GameInputKind inputKind,
        _In_ float analogThreshold,
        _In_opt_ void * context,
        _In_ GameInputReadingCallback callbackFunc,
        _Out_opt_ _Result_zeroonfailure_ GameInputCallbackToken * callbackToken) PURE;

    IFACEMETHOD(RegisterDeviceCallback)(THIS_
        _In_opt_ IGameInputDevice * device,
        _In_ GameInputKind inputKind,
        _In_ GameInputDeviceStatus statusFilter,
        _In_ GameInputEnumerationKind enumerationKind,
        _In_opt_ void * context,
        _In_ GameInputDeviceCallback callbackFunc,
        _Out_opt_ _Result_zeroonfailure_ GameInputCallbackToken * callbackToken) PURE;

    IFACEMETHOD(RegisterSystemButtonCallback)(THIS_
        _In_opt_ IGameInputDevice * device,
        _In_ GameInputSystemButtons buttonFilter,
        _In_opt_ void * context,
        _In_ GameInputSystemButtonCallback callbackFunc,
        _Out_opt_ _Result_zeroonfailure_ GameInputCallbackToken * callbackToken) PURE;

    IFACEMETHOD(RegisterKeyboardLayoutCallback)(THIS_
        _In_opt_ IGameInputDevice * device,
        _In_opt_ void * context,
        _In_ GameInputKeyboardLayoutCallback callbackFunc,
        _Out_opt_ _Result_zeroonfailure_ GameInputCallbackToken * callbackToken) PURE;

    IFACEMETHOD_(void, StopCallback)(THIS_
        _In_ GameInputCallbackToken callbackToken) PURE;

    IFACEMETHOD_(bool, UnregisterCallback)(THIS_
        _In_ GameInputCallbackToken callbackToken,
        _In_ uint64_t timeoutInMicroseconds) PURE;

    _Must_inspect_result_ IFACEMETHOD(CreateDispatcher)(THIS_
        _COM_Outptr_ IGameInputDispatcher ** dispatcher) PURE;

    _Must_inspect_result_ IFACEMETHOD(CreateAggregateDevice)(THIS_
        _In_ GameInputKind inputKind,
        _COM_Outptr_ IGameInputDevice ** device) PURE;

    _Must_inspect_result_ IFACEMETHOD(FindDeviceFromId)(THIS_
        _In_ APP_LOCAL_DEVICE_ID const * value,
        _COM_Outptr_ IGameInputDevice ** device) PURE;

    _Must_inspect_result_ IFACEMETHOD(FindDeviceFromObject)(THIS_
        _In_ IUnknown * value,
        _COM_Outptr_ IGameInputDevice ** device) PURE;

    _Must_inspect_result_ IFACEMETHOD(FindDeviceFromPlatformHandle)(THIS_
        _In_ HANDLE value,
        _COM_Outptr_ IGameInputDevice ** device) PURE;

    _Must_inspect_result_ IFACEMETHOD(FindDeviceFromPlatformString)(THIS_
        _In_ LPCWSTR value,
        _COM_Outptr_ IGameInputDevice ** device) PURE;

    IFACEMETHOD(EnableOemDeviceSupport)(THIS_
        _In_ uint16_t vendorId,
        _In_ uint16_t productId,
        _In_ uint8_t interfaceNumber,
        _In_ uint8_t collectionNumber) PURE;

    IFACEMETHOD_(void, SetFocusPolicy)(THIS_
        _In_ GameInputFocusPolicy policy) PURE;
};

#undef INTERFACE
#define INTERFACE IGameInputReading
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "2156947A-E1FA-4DE0-A30B-D812931DBD8D")
{
    _Must_inspect_result_ IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(GameInputKind, GetInputKind)(THIS) PURE;

    IFACEMETHOD_(uint64_t, GetSequenceNumber)(THIS_
        _In_ GameInputKind inputKind) PURE;

    IFACEMETHOD_(uint64_t, GetTimestamp)(THIS) PURE;

    IFACEMETHOD_(void, GetDevice)(THIS_
        _Outptr_ IGameInputDevice ** device) PURE;

    IFACEMETHOD_(bool, GetRawReport)(THIS_
        _Outptr_result_maybenull_ IGameInputRawDeviceReport ** report) PURE;

    IFACEMETHOD_(uint32_t, GetControllerAxisCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetControllerAxisState)(THIS_
        _In_ uint32_t stateArrayCount,
        _Out_writes_(stateArrayCount) float * stateArray) PURE;

    IFACEMETHOD_(uint32_t, GetControllerButtonCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetControllerButtonState)(THIS_
        _In_ uint32_t stateArrayCount,
        _Out_writes_(stateArrayCount) bool * stateArray) PURE;

    IFACEMETHOD_(uint32_t, GetControllerSwitchCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetControllerSwitchState)(THIS_
        _In_ uint32_t stateArrayCount,
        _Out_writes_(stateArrayCount) GameInputSwitchPosition * stateArray) PURE;

    IFACEMETHOD_(uint32_t, GetKeyCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetKeyState)(THIS_
        _In_ uint32_t stateArrayCount,
        _Out_writes_(stateArrayCount) GameInputKeyState * stateArray) PURE;

    IFACEMETHOD_(bool, GetMouseState)(THIS_
        _Out_ GameInputMouseState * state) PURE;

    IFACEMETHOD_(uint32_t, GetTouchCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetTouchState)(THIS_
        _In_ uint32_t stateArrayCount,
        _Out_writes_(stateArrayCount) GameInputTouchState * stateArray) PURE;

    IFACEMETHOD_(bool, GetMotionState)(THIS_
        _Out_ GameInputMotionState * state) PURE;

    IFACEMETHOD_(bool, GetArcadeStickState)(THIS_
        _Out_ GameInputArcadeStickState * state) PURE;

    IFACEMETHOD_(bool, GetFlightStickState)(THIS_
        _Out_ GameInputFlightStickState * state) PURE;

    IFACEMETHOD_(bool, GetGamepadState)(THIS_
        _Out_ GameInputGamepadState * state) PURE;

    IFACEMETHOD_(bool, GetRacingWheelState)(THIS_
        _Out_ GameInputRacingWheelState * state) PURE;

    IFACEMETHOD_(bool, GetUiNavigationState)(THIS_
        _Out_ GameInputUiNavigationState * state) PURE;
};

#undef INTERFACE
#define INTERFACE IGameInputDevice
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "31DD86FB-4C1B-408A-868F-439B3CD47125")
{
    _Must_inspect_result_ IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(_Ret_notnull_ GameInputDeviceInfo const *, GetDeviceInfo)(THIS) PURE;

    IFACEMETHOD_(GameInputDeviceStatus, GetDeviceStatus)(THIS) PURE;

    IFACEMETHOD_(void, GetBatteryState)(THIS_
        _Out_ GameInputBatteryState * state) PURE;

    _Must_inspect_result_ IFACEMETHOD(CreateForceFeedbackEffect)(THIS_
        _In_ uint32_t motorIndex,
        _In_ GameInputForceFeedbackParams const * params,
        _COM_Outptr_ IGameInputForceFeedbackEffect ** effect) PURE;

    IFACEMETHOD_(bool, IsForceFeedbackMotorPoweredOn)(THIS_
        _In_ uint32_t motorIndex) PURE;

    IFACEMETHOD_(void, SetForceFeedbackMotorGain)(THIS_
        _In_ uint32_t motorIndex,
        _In_ float masterGain) PURE;

    IFACEMETHOD(SetHapticMotorState)(THIS_
        _In_ uint32_t motorIndex,
        _In_opt_ GameInputHapticFeedbackParams const * params) PURE;

    IFACEMETHOD_(void, SetRumbleState)(THIS_
        _In_opt_ GameInputRumbleParams const * params) PURE;

    IFACEMETHOD_(void, SetInputSynchronizationState)(THIS_
        _In_ bool enabled) PURE;

    IFACEMETHOD_(void, SendInputSynchronizationHint)(THIS) PURE;

    IFACEMETHOD_(void, PowerOff)(THIS) PURE;

    _Must_inspect_result_ IFACEMETHOD(CreateRawDeviceReport)(THIS_
        _In_ uint32_t reportId,
        _In_ GameInputRawDeviceReportKind reportKind,
        _COM_Outptr_ IGameInputRawDeviceReport ** report) PURE;

    _Must_inspect_result_ IFACEMETHOD(GetRawDeviceFeature)(THIS_
        _In_ uint32_t reportId,
        _COM_Outptr_ IGameInputRawDeviceReport ** report) PURE;

    IFACEMETHOD(SetRawDeviceFeature)(THIS_
        _In_ IGameInputRawDeviceReport * report) PURE;

    IFACEMETHOD(SendRawDeviceOutput)(THIS_
        _In_ IGameInputRawDeviceReport * report) PURE;

    IFACEMETHOD(SendRawDeviceOutputWithResponse)(THIS_
        _In_ IGameInputRawDeviceReport * requestReport,
        _COM_Outptr_ IGameInputRawDeviceReport ** responseReport) PURE;

    IFACEMETHOD(ExecuteRawDeviceIoControl)(THIS_
        _In_ uint32_t controlCode,
        _In_ size_t inputBufferSize,
        _In_reads_bytes_opt_(inputBufferSize) void const * inputBuffer,
        _In_ size_t outputBufferSize,
        _Out_writes_bytes_all_opt_(outputBufferSize) void * outputBuffer,
        _Out_opt_ _Result_zeroonfailure_ size_t * outputSize) PURE;

    IFACEMETHOD_(bool, AcquireExclusiveRawDeviceAccess)(THIS_
        _In_ uint64_t timeoutInMicroseconds) PURE;

    IFACEMETHOD_(void, ReleaseExclusiveRawDeviceAccess)(THIS) PURE;
};

#undef INTERFACE
#define INTERFACE IGameInputDispatcher
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "415EED2E-98CB-42C2-8F28-B94601074E31")
{
    _Must_inspect_result_ IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(bool, Dispatch)(THIS_
        _In_ uint64_t quotaInMicroseconds) PURE;

    IFACEMETHOD(OpenWaitHandle)(THIS_
        _Outptr_result_nullonfailure_ HANDLE * waitHandle) PURE;
};

#undef INTERFACE
#define INTERFACE IGameInputForceFeedbackEffect
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "51BDA05E-F742-45D9-B085-9444AE48381D")
{
    _Must_inspect_result_ IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(void, GetDevice)(THIS_
        _Outptr_ IGameInputDevice ** device) PURE;

    IFACEMETHOD_(uint32_t, GetMotorIndex)(THIS) PURE;

    IFACEMETHOD_(float, GetGain)(THIS) PURE;

    IFACEMETHOD_(void, SetGain)(THIS_
        _In_ float gain) PURE;

    IFACEMETHOD_(void, GetParams)(THIS_
        _Out_ GameInputForceFeedbackParams * params) PURE;

    IFACEMETHOD_(bool, SetParams)(THIS_
        _In_ GameInputForceFeedbackParams const * params) PURE;

    IFACEMETHOD_(GameInputFeedbackEffectState, GetState)(THIS) PURE;

    IFACEMETHOD_(void, SetState)(THIS_
        _In_ GameInputFeedbackEffectState state) PURE;
};

#undef INTERFACE
#define INTERFACE IGameInputRawDeviceReport
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "61F08CF1-1FFC-40CA-A2B8-E1AB8BC5B6DC")
{
    _Must_inspect_result_ IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(void, GetDevice)(THIS_
        _Outptr_ IGameInputDevice ** device) PURE;

    IFACEMETHOD_(_Ret_notnull_ GameInputRawDeviceReportInfo const *, GetReportInfo)(THIS) PURE;

    IFACEMETHOD_(size_t, GetRawDataSize)(THIS) PURE;

    IFACEMETHOD_(size_t, GetRawData)(THIS_
        _In_ size_t bufferSize,
        _Out_writes_(bufferSize) void * buffer) PURE;

    IFACEMETHOD_(bool, SetRawData)(THIS_
        _In_ size_t bufferSize,
        _In_reads_(bufferSize) void const * buffer) PURE;

    IFACEMETHOD_(bool, GetItemValue)(THIS_
        _In_ uint32_t itemIndex,
        _Out_ int64_t * value) PURE;

    IFACEMETHOD_(bool, SetItemValue)(THIS_
        _In_ uint32_t itemIndex,
        _In_ int64_t value) PURE;

    IFACEMETHOD_(bool, ResetItemValue)(THIS_
        _In_ uint32_t itemIndex) PURE;

    IFACEMETHOD_(bool, ResetAllItems)(THIS) PURE;
};

#undef INTERFACE

_Must_inspect_result_ STDAPI GameInputCreate(
    _COM_Outptr_ IGameInput ** gameInput);

#ifdef __cplusplus
}   // extern "C"
#endif

#if (!defined(__cplusplus) || defined(CINTERFACE)) && defined(COBJMACROS)

#define IGameInput_QueryInterface(This, riid, ppvObj) ((This)->lpVtbl->QueryInterface(This, riid, ppvObj))
#define IGameInput_AddRef(This) ((This)->lpVtbl->AddRef(This))
#define IGameInput_Release(This) ((This)->lpVtbl->Release(This))
#define IGameInput_GetCurrentTimestamp(This) ((This)->lpVtbl->GetCurrentTimestamp(This))
#define IGameInput_GetCurrentReading(This, inputKind, device, reading) ((This)->lpVtbl->GetCurrentReading(This, inputKind, device, reading))
#define IGameInput_GetNextReading(This, referenceReading, inputKind, device, reading) ((This)->lpVtbl->GetNextReading(This, referenceReading, inputKind, device, reading))
#define IGameInput_GetPreviousReading(This, referenceReading, inputKind, device, reading) ((This)->lpVtbl->GetPreviousReading(This, referenceReading, inputKind, device, reading))
#define IGameInput_GetTemporalReading(This, timestamp, device, reading) ((This)->lpVtbl->GetTemporalReading(This, timestamp, device, reading))
#define IGameInput_RegisterReadingCallback(This, device, inputKind, analogThreshold, context, callbackFunc, callbackToken) ((This)->lpVtbl->RegisterReadingCallback(This, device, inputKind, analogThreshold, context, callbackFunc, callbackToken))
#define IGameInput_RegisterDeviceCallback(This, device, inputKind, statusFilter, enumerationKind, context, callbackFunc, callbackToken) ((This)->lpVtbl->RegisterDeviceCallback(This, device, inputKind, statusFilter, enumerationKind, context, callbackFunc, callbackToken))
#define IGameInput_RegisterSystemButtonCallback(This, device, buttonFilter, focusPolicy, context, callbackFunc, callbackToken) ((This)->lpVtbl->RegisterSystemButtonCallback(This, device, buttonFilter, focusPolicy, context, callbackFunc, callbackToken))
#define IGameInput_RegisterKeyboardLayoutCallback(This, device, context, callbackFunc, callbackToken) ((This)->lpVtbl->RegisterKeyboardLayoutCallback(This, device, context, callbackFunc, callbackToken))
#define IGameInput_StopCallback(This, callbackToken) ((This)->lpVtbl->StopCallback(This, callbackToken))
#define IGameInput_UnregisterCallback(This, callbackToken, timeoutInMicroseconds) ((This)->lpVtbl->UnregisterCallback(This, callbackToken, timeoutInMicroseconds))
#define IGameInput_CreateDispatcher(This, dispatcher) ((This)->lpVtbl->CreateDispatcher(This, dispatcher))
#define IGameInput_CreateAggregateDevice(This, inputKind, device) ((This)->lpVtbl->CreateAggregateDevice(This, inputKind, device))
#define IGameInput_FindDeviceFromId(This, value, device) ((This)->lpVtbl->FindDeviceFromId(This, value, device))
#define IGameInput_FindDeviceFromObject(This, value, device) ((This)->lpVtbl->FindDeviceFromObject(This, value, device))
#define IGameInput_FindDeviceFromPlatformHandle(This, value, device) ((This)->lpVtbl->FindDeviceFromPlatformHandle(This, value, device))
#define IGameInput_FindDeviceFromPlatformString(This, value, device) ((This)->lpVtbl->FindDeviceFromPlatformString(This, value, device))
#define IGameInput_EnableOemDeviceSupport(This, vendorId, productId, interfaceNumber, collectionNumber) ((This)->lpVtbl->EnableOemDeviceSupport(This, vendorId, productId, interfaceNumber, collectionNumber))
#define IGameInput_SetFocusPolicy(This, policy) ((This)->lpVtbl->SetFocusPolicy(This, policy))

#define IGameInputReading_QueryInterface(This, riid, ppvObj) ((This)->lpVtbl->QueryInterface(This, riid, ppvObj))
#define IGameInputReading_AddRef(This) ((This)->lpVtbl->AddRef(This))
#define IGameInputReading_Release(This) ((This)->lpVtbl->Release(This))
#define IGameInputReading_GetInputKind(This) ((This)->lpVtbl->GetInputKind(This))
#define IGameInputReading_GetSequenceNumber(This, inputKind) ((This)->lpVtbl->GetSequenceNumber(This, inputKind))
#define IGameInputReading_GetTimestamp(This) ((This)->lpVtbl->GetTimestamp(This))
#define IGameInputReading_GetDevice(This, device) ((This)->lpVtbl->GetDevice(This, device))
#define IGameInputReading_GetRawReport(This, report) ((This)->lpVtbl->GetRawReport(This, report))
#define IGameInputReading_GetControllerAxisCount(This) ((This)->lpVtbl->GetControllerAxisCount(This))
#define IGameInputReading_GetControllerAxisState(This, stateArrayCount, stateArray) ((This)->lpVtbl->GetControllerAxisState(This, stateArrayCount, stateArray))
#define IGameInputReading_GetControllerButtonCount(This) ((This)->lpVtbl->GetControllerButtonCount(This))
#define IGameInputReading_GetControllerButtonState(This, stateArrayCount, stateArray) ((This)->lpVtbl->GetControllerButtonState(This, stateArrayCount, stateArray))
#define IGameInputReading_GetControllerSwitchCount(This) ((This)->lpVtbl->GetControllerSwitchCount(This))
#define IGameInputReading_GetControllerSwitchState(This, stateArrayCount, stateArray) ((This)->lpVtbl->GetControllerSwitchState(This, stateArrayCount, stateArray))
#define IGameInputReading_GetKeyCount(This) ((This)->lpVtbl->GetKeyCount(This))
#define IGameInputReading_GetKeyState(This, stateArrayCount, stateArray) ((This)->lpVtbl->GetKeyState(This, stateArrayCount, stateArray))
#define IGameInputReading_GetMouseState(This, state) ((This)->lpVtbl->GetMouseState(This, state))
#define IGameInputReading_GetTouchCount(This) ((This)->lpVtbl->GetTouchCount(This))
#define IGameInputReading_GetTouchState(This, stateArrayCount, stateArray) ((This)->lpVtbl->GetTouchState(This, stateArrayCount, stateArray))
#define IGameInputReading_GetMotionState(This, state) ((This)->lpVtbl->GetMotionState(This, state))
#define IGameInputReading_GetArcadeStickState(This, state) ((This)->lpVtbl->GetArcadeStickState(This, state))
#define IGameInputReading_GetFlightStickState(This, state) ((This)->lpVtbl->GetFlightStickState(This, state))
#define IGameInputReading_GetGamepadState(This, state) ((This)->lpVtbl->GetGamepadState(This, state))
#define IGameInputReading_GetRacingWheelState(This, state) ((This)->lpVtbl->GetRacingWheelState(This, state))
#define IGameInputReading_GetUiNavigationState(This, state) ((This)->lpVtbl->GetUiNavigationState(This, state))

#define IGameInputDevice_QueryInterface(This, riid, ppvObj) ((This)->lpVtbl->QueryInterface(This, riid, ppvObj))
#define IGameInputDevice_AddRef(This) ((This)->lpVtbl->AddRef(This))
#define IGameInputDevice_Release(This) ((This)->lpVtbl->Release(This))
#define IGameInputDevice_GetDeviceInfo(This) ((This)->lpVtbl->GetDeviceInfo(This))
#define IGameInputDevice_GetDeviceStatus(This) ((This)->lpVtbl->GetDeviceStatus(This))
#define IGameInputDevice_GetBatteryState(This, state) ((This)->lpVtbl->GetBatteryState(This, state))
#define IGameInputDevice_CreateForceFeedbackEffect(This, motorIndex, params, effect) ((This)->lpVtbl->CreateForceFeedbackEffect(This, motorIndex, params, effect))
#define IGameInputDevice_IsForceFeedbackMotorPoweredOn(This, motorIndex) ((This)->lpVtbl->IsForceFeedbackMotorPoweredOn(This, motorIndex))
#define IGameInputDevice_SetForceFeedbackMotorGain(This, motorIndex, masterGain) ((This)->lpVtbl->SetForceFeedbackMotorGain(This, motorIndex, masterGain))
#define IGameInputDevice_SetHapticMotorState(This, motorIndex, params) ((This)->lpVtbl->SetHapticMotorState(This, motorIndex, params))
#define IGameInputDevice_SetRumbleState(This, params) ((This)->lpVtbl->SetRumbleState(This, params))
#define IGameInputDevice_SetInputSynchronizationState(This, enabled) ((This)->lpVtbl->SetInputSynchronizationState(This, enabled))
#define IGameInputDevice_SendInputSynchronizationHint(This) ((This)->lpVtbl->SendInputSynchronizationHint(This))
#define IGameInputDevice_PowerOff(This) ((This)->lpVtbl->PowerOff(This))
#define IGameInputDevice_CreateRawDeviceReport(This, reportId, reportKind, report) ((This)->lpVtbl->CreateRawDeviceReport(This, reportId, reportKind, report))
#define IGameInputDevice_GetRawDeviceFeature(This, reportId, report) ((This)->lpVtbl->GetRawDeviceFeature(This, reportId, report))
#define IGameInputDevice_SetRawDeviceFeature(This, report) ((This)->lpVtbl->SetRawDeviceFeature(This, report))
#define IGameInputDevice_SendRawDeviceOutput(This, report) ((This)->lpVtbl->SendRawDeviceOutput(This, report))
#define IGameInputDevice_SendRawDeviceOutputWithResponse(This, requestReport, responseReport) ((This)->lpVtbl->SendRawDeviceOutputWithResponse(This, requestReport, responseReport))
#define IGameInputDevice_ExecuteRawDeviceIoControl(This, controlCode, inputBufferSize, inputBuffer, outputBufferSize, outputBuffer, outputSize) ((This)->lpVtbl->ExecuteRawDeviceIoControl(This, controlCode, inputBufferSize, inputBuffer, outputBufferSize, outputBuffer, outputSize))
#define IGameInputDevice_AcquireExclusiveRawDeviceAccess(This, timeoutInMicroseconds) ((This)->lpVtbl->AcquireExclusiveRawDeviceAccess(This, timeoutInMicroseconds))
#define IGameInputDevice_ReleaseExclusiveRawDeviceAccess(This) ((This)->lpVtbl->ReleaseExclusiveRawDeviceAccess(This))

#define IGameInputDispatcher_QueryInterface(This, riid, ppvObj) ((This)->lpVtbl->QueryInterface(This, riid, ppvObj)
#define IGameInputDispatcher_AddRef(This) ((This)->lpVtbl->AddRef(This)
#define IGameInputDispatcher_Release(This) ((This)->lpVtbl->Release(This)
#define IGameInputDispatcher_Dispatch(This, quotaInMicroseconds) ((This)->lpVtbl->Dispatch(This, quotaInMicroseconds)
#define IGameInputDispatcher_OpenWaitHandle(This, waitHandle) ((This)->lpVtbl->OpenWaitHandle(This, waitHandle)

#define IGameInputForceFeedbackEffect_QueryInterface(This, riid, ppvObj) ((This)->lpVtbl->QueryInterface(This, riid, ppvObj)
#define IGameInputForceFeedbackEffect_AddRef(This) ((This)->lpVtbl->AddRef(This)
#define IGameInputForceFeedbackEffect_Release(This) ((This)->lpVtbl->Release(This)
#define IGameInputForceFeedbackEffect_GetDevice(This, device) ((This)->lpVtbl->GetDevice(This, device)
#define IGameInputForceFeedbackEffect_GetMotorIndex(This) ((This)->lpVtbl->GetMotorIndex(This)
#define IGameInputForceFeedbackEffect_GetGain(This) ((This)->lpVtbl->GetGain(This)
#define IGameInputForceFeedbackEffect_SetGain(This, gain) ((This)->lpVtbl->SetGain(This, gain)
#define IGameInputForceFeedbackEffect_GetParams(This, params) ((This)->lpVtbl->GetParams(This, params)
#define IGameInputForceFeedbackEffect_SetParams(This, params) ((This)->lpVtbl->SetParams(This, params)
#define IGameInputForceFeedbackEffect_GetState(This) ((This)->lpVtbl->GetState(This)
#define IGameInputForceFeedbackEffect_SetState(This, state) ((This)->lpVtbl->SetState(This, state)

#define IGameInputRawDeviceReport_QueryInterface(This, riid, ppvObj) ((This)->lpVtbl->QueryInterface(This, riid, ppvObj)
#define IGameInputRawDeviceReport_AddRef(This) ((This)->lpVtbl->AddRef(This)
#define IGameInputRawDeviceReport_Release(This) ((This)->lpVtbl->Release(This)
#define IGameInputRawDeviceReport_GetDevice(This, device) ((This)->lpVtbl->GetDevice(This, device)
#define IGameInputRawDeviceReport_GetReportInfo(This) ((This)->lpVtbl->GetReportInfo(This)
#define IGameInputRawDeviceReport_GetRawDataSize(This) ((This)->lpVtbl->GetRawDataSize(This)
#define IGameInputRawDeviceReport_GetRawData(This, bufferSize, buffer) ((This)->lpVtbl->GetRawData(This, bufferSize, buffer)
#define IGameInputRawDeviceReport_SetRawData(This, bufferSize, buffer) ((This)->lpVtbl->SetRawData(This, bufferSize, buffer)
#define IGameInputRawDeviceReport_GetItemValue(This, itemIndex, value) ((This)->lpVtbl->GetItemValue(This, itemIndex, value)
#define IGameInputRawDeviceReport_SetItemValue(This, itemIndex, value) ((This)->lpVtbl->SetItemValue(This, itemIndex, value)
#define IGameInputRawDeviceReport_ResetItemValue(This, itemIndex) ((This)->lpVtbl->ResetItemValue()
#define IGameInputRawDeviceReport_ResetAllItems(This) ((This)->lpVtbl->ResetAllItems(This)

#endif   // #if (!defined(__cplusplus) || defined(CINTERFACE)) && defined(COBJMACROS)

#endif   // #if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#define FACILITY_GAMEINPUT 906

//
// MessageId: GAMEINPUT_E_DEVICE_DISCONNECTED
//
// MessageText:
//
// The device is not currently connected to the system.
//
#define GAMEINPUT_E_DEVICE_DISCONNECTED _HRESULT_TYPEDEF_(0x838A0001L)

//
// MessageId: GAMEINPUT_E_DEVICE_NOT_FOUND
//
// MessageText:
//
// The requested device could not be found.
//
#define GAMEINPUT_E_DEVICE_NOT_FOUND _HRESULT_TYPEDEF_(0x838A0002L)

//
// MessageId: GAMEINPUT_E_READING_NOT_FOUND
//
// MessageText:
//
// The requested reading could not be found.
//
#define GAMEINPUT_E_READING_NOT_FOUND _HRESULT_TYPEDEF_(0x838A0003L)

//
// MessageId: GAMEINPUT_E_REFERENCE_READING_TOO_OLD
//
// MessageText:
//
// The reference reading no longer exists in the reading history.
//
#define GAMEINPUT_E_REFERENCE_READING_TOO_OLD _HRESULT_TYPEDEF_(0x838A0004L)

//
// MessageId: GAMEINPUT_E_TIMESTAMP_OUT_OF_RANGE
//
// MessageText:
//
// The target timestamp for the temporal reading is too far in the past or future.
//
#define GAMEINPUT_E_TIMESTAMP_OUT_OF_RANGE _HRESULT_TYPEDEF_(0x838A0005L)

//
// MessageId: GAMEINPUT_E_INSUFFICIENT_FORCE_FEEDBACK_RESOURCES
//
// MessageText:
//
// The device does not have enough resources remaining to create the requested force feedback effect.
//
#define GAMEINPUT_E_INSUFFICIENT_FORCE_FEEDBACK_RESOURCES _HRESULT_TYPEDEF_(0x838A0006L)

#endif   // #ifndef __gameinput_h__
