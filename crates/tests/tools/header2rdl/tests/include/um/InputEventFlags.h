#pragma once
//
// InputEventFlags are intended to be generic Input Flags to indicate actions for
// Touch, Mouse or Keyboard events.
//

enum InputEventFlag
{
    InputEventFlag_None             = 0x0000,

    InputEventFlag_Down             = 0x0001,
    InputEventFlag_Move             = 0x0002,
    InputEventFlag_Hold             = 0x0002,
    InputEventFlag_Up               = 0x0004,

    InputEventFlag_InRange          = 0x0008,
    InputEventFlag_FromISM          = 0x0008,

    InputEventFlag_DownAndUp        = (InputEventFlag_Down|InputEventFlag_Up),  // 0x0005

    InputEventFlag_FromHWKeyboard   = 0x0010,
    InputEventFlag_FromSyntheticHW  = 0x0020,
    InputEventFlag_SkipHotKey       = 0x0040,
    InputEventFlag_FromOverrider    = 0x0080,

    InputEventFlag_SuppressAcceleratorKey = 0x0100,

    InputEventFlag_CharacterFromSoftwareKeyboard = 0x0200,

    InputEventFlag_ScreenReaderEnabled = 0x0400,

    InputEventFlag_AcknowledgementRequired = 0x0800,

    InputEventFlag_Empty            = 0x1000,
    InputEventFlag_Invalid          = 0x2000,

    InputEventFlag_TestInjection    = 0x4000,
    InputEventFlag_TestSync         = 0x8000,
};

enum InputEventFlag2
{
    InputEventFlag2_None                    = 0x00000000,
    InputEventFlag2_InputServiceInjection   = 0x00000001,

    InputEventFlag2_RawCustomText           = 0x00000002,
    InputEventFlag2_TelemetrySamplePicked   = 0x00000004,
    InputEventFlag2_TelemetryVirtualKey     = 0x00000008,
};
