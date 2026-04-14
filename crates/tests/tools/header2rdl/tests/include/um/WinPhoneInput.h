////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
////////////////////////////////////////////////////////////////////////////////


#pragma once
#ifndef _WINPHONEINPUT_H_
#define _WINPHONEINPUT_H_

#include <InputEventFlags.h>

//
// Define a device interface unique GUID for touch 
// Note: this is used in \src\uxplat\Splash\Dev\Libraries\InputReader\Library\InputStateManager.cpp 
//                  and in \src\uxplat\Input\Dev\Drivers\TchHID\TchHID.c
//

DEFINE_GUID( GUID_WP_DEVINTERFACE_TCH, 0x2c05ce1a, 0x75e8, 0x4ea7, 0xa7, 0xa, 0xe3, 0x23, 0xb0, 0x72, 0x25, 0x8c);

//
// The maximum number of the supported capacitive buttons.
// Back/Start/Search are supported for now.
//

#define MAX_CAPACITIVE_BUTTONS   3


//
// InputSessionFlags are intended to be generic Input Session Flags to indicate
// actions for a full Touch Session.
//
// Session state ordering during normal operation:
// - Contact touch: (Begin) --> zero or more (None) --> (End)
// - Hover touch: (Hover|Begin) --> zero or more (Hover) --> (Hover|End)
// Hover touch sessions do not overlap with contact touch sessions.  Input reader
// will synthesize begin/end events to ensure the separation.
//

enum InputSessionFlag
{
    InputSessionFlag_None               = 0x0000,
    
    InputSessionFlag_Begin              = 0x0001,
    InputSessionFlag_End                = 0x0004,

    InputSessionFlag_Hover              = 0x0100,

    InputSessionFlag_EdgeGesture        = 0x0200,
    InputSessionFlag_CapacitiveButton   = 0x0400,

    InputSessionFlag_Mouse              = 0x0800,
    
    InputSessionFlag_Cancel             = 0x8000
};


//
// ModifierKeyStateFlags are intended to capture modifiers key states that exist
// before/while other key events occur.
//

enum ModifierKeyStateFlag
{
    ModifierKeyStateFlag_None           = 0x0000,

    ModifierKeyStateFlag_Shift          = 0x0001,
    ModifierKeyStateFlag_ShiftLeft      = 0x0003,
    ModifierKeyStateFlag_ShiftRight     = 0x0005,
    ModifierKeyStateFlag_ShiftMask      = 0x0007,

    ModifierKeyStateFlag_Control        = 0x0008,
    ModifierKeyStateFlag_ControlLeft    = 0x0018,
    ModifierKeyStateFlag_ControlRight   = 0x0028,
    ModifierKeyStateFlag_ControlMask    = 0x0038,

    ModifierKeyStateFlag_Alt            = 0x0040,
    ModifierKeyStateFlag_AltLeft        = 0x00C0,
    ModifierKeyStateFlag_AltRight       = 0x0140,
    ModifierKeyStateFlag_AltMask        = 0x01C0,

    ModifierKeyStateFlag_Function       = 0x0200,
    ModifierKeyStateFlag_FunctionLeft   = 0x0600,
    ModifierKeyStateFlag_FunctionRight  = 0x0A00,
    ModifierKeyStateFlag_FunctionMask   = 0x0E00,

    ModifierKeyStateFlag_LockShift      = 0x1000,
    ModifierKeyStateFlag_LockNum        = 0x2000,
    ModifierKeyStateFlag_LockFunction   = 0x4000,
    ModifierKeyStateFlag_LockMask       = 0xF000,

    ModifierKeyStateFlag_RefreshModifier  = 0x10000     // Only SIP send this flag to refresh the latest Modifier status on SIP
};


//
// MouseInfo represents information about a Single Mouse Event.  At the moment
// only the left mouse button is represented, but dwFlags is large enough to 
// represent other button actions in the HIWORD.  For now assume all
// InputEventFlag_* data represents the left button only.
//

#ifdef __cplusplus 
struct MouseInfo
{
    DWORD   dwFlags;        // See InputEventFlag_* flags
    DWORD   dwTimeStamp;    // Driver timestamp
    HANDLE  hSource;        // Source of the Mouse data
    SHORT   xScreen;        // Screen Space X-Position
    SHORT   yScreen;        // Screen Space Y-Position
    SHORT   xWindow;        // Client Space X-Position
    SHORT   yWindow;        // Client Space Y-Position
};
#else // __cplusplus 
// not yet needed for C
#endif // __cplusplus 


//
// TouchContact represents information about a Single Touch Contact that should only
// exists as part of a Touch Session update (See TouchInfo).
//
// Any changes here require a corresponding change in:
// * XNA's C# declaration
//   (Windows Phone Runtimes depot)\XNA\Runtime\Framework\Touch\NativeMethods.cs
// * XDE's C# declaration
//   (Windows Phone Tools depot)\XDE\Products\Common\IXdeInputPipe.cs
// * CoreUI message marshalling Cn declaration
//   (Windows Phone UxPlat depot)\CoreUI\Dev\CoreUIComponents\RemoteInterfaces\Input\IRemoteInputInjection.cs
// * Limited version of WinPhoneInput.h sent out to OEMs while hover is still confidential.
//   (Windows Phone UxPlat depot)\Splash\Dev\Inc\Limited\WinPhoneInput.h
// * MinUser Cn declaration
//   (Windows MinCore depot)\CoreUI\Dev\MinUser\Input\InputInterop.cs
// * Manipulation converged tests declaration
//   (Windows ShellTest depot)\Personality\Touch\Manipulation\ConvergedTests\DataCollector\Inc\Phone.h
//

#ifdef __cplusplus 
struct TouchContact
{
    WORD    wContactID;     // The ID of this contact
    WORD    wFlags;         // See InputEventFlag_* flags
    SHORT   xScreen;        // Screen Space X-Position
    SHORT   yScreen;        // Screen Space Y-Position
    SHORT   xWindow;        // Client Space X-Position
    SHORT   yWindow;        // Client Space Y-Position
    SHORT   xArea;          // Screen space X-dimension (width) of contact area
    SHORT   yArea;          // Screen space Y-dimension (height) of contact area
    SHORT   zDistance;      // Unitless Z-distance
    SHORT   reserved;       // Reserved
    SHORT   xTilt;          // Angle in degrees along X-axis. 0=straight up, Positive angle = tilt right.
    SHORT   yTilt;          // Angle in degrees along Y-axis. 0=straight up, Positive angle = tilt up.
    ULONG64 inputSink;      // For CoreInput, window handle for inputSink.
    FLOAT   inputTransform[6];// For CoreInput, 2D affine transform for inputSink
};
#else // __cplusplus 
typedef struct _TouchContact
{
    UINT16  ContactID;      // The ID of this contact
    UINT16  Flags;          // See InputEventFlag_* flags
    INT16   ScreenX;        // Screen Space X-Position
    INT16   ScreenY;        // Screen Space Y-Position
    INT16   WindowX;        // Client Space X-Position
    INT16   WindowY;        // Client Space Y-Position
    INT16   AreaX;          // Screen space X-dimension (width) of contact area
    INT16   AreaY;          // Screen space Y-dimension (height) of contact area
    INT16   DistanceZ;      // Unitless Z-distance
    INT16   Reserved;       // Reserved
    INT16   TiltX;          // Angle in degrees along X-axis. 0=straight up, Positive angle = tilt right.
    INT16   TiltY;          // Angle in degrees along Y-axis. 0=straight up, Positive angle = tilt up.
    ULONG64 InputSink;      // For CoreInput, window handle for inputSink.
    float   InputTransform[6];// For CoreInput, 2D affine transform for inputSink
} TouchContact;
#endif // __cplusplus 


//
// TouchInfo represents information about an in-the-moment view of Touch Events within
// a Touch Session.
//
// Any changes here require a corresponding change in:
// * XNA's C# declaration
//   (Windows Phone Runtimes depot)\XNA\Runtime\Framework\Touch\NativeMethods.cs
// * XDE's C# declaration
//   (Windows Phone Tools depot)\XDE\Product\Common\IXdeInputPipe.cs
// * CoreUI message marshalling Cn declaration
//   (Windows Phone UxPlat depot)\CoreUI\Dev\CoreUIComponents\RemoteInterfaces\Input\IRemoteInputInjection.cs
// * Limited version of WinPhoneInput.h sent out to OEMs while hover is still confidential.
//   (Windows Phone UxPlat depot)\Splash\Dev\Inc\Limited\WinPhoneInput.h
// * MinUser Cn declaration
//   (Windows MinCore depot)\CoreUI\Dev\MinUser\Input\InputInterop.cs
// * Manipulation converged tests declaration
//   (Windows ShellTest depot)\Personality\Touch\Manipulation\ConvergedTests\DataCollector\Inc\Phone.h
//

#ifdef __cplusplus
static const DWORD c_cTouchContactsMaximum = 10;

struct TouchInfo
{
    WORD            cbSize;             // Size, in bytes, of this structure (includes n contacts)
    WORD            wFlags;             // See InputSessionFlag_* flags
    DWORD           dwTimeStamp;        // Driver timestamp
    ULONG64         hSource;            // HANDLE to the Source of the Touch data
    DWORD           dwSessionID;        // Touch session ID
    BYTE            reserved;           // Reserved for byte alignment
    BYTE            cInRangeContacts;   // The count of contacts that are marked InRange
    BYTE            cInContactContacts; // The count of contacts that are marked Down or Move
    BYTE            cContacts;          // Count of all TouchContact data points
    TouchContact    rgContacts[1];      // Collection of contacts
};
#else // __cplusplus 
#define c_cTouchContactsMaximum  10

typedef struct _TouchInfo
{
    UINT16          Size;                                   // Size, in bytes, of this structure (includes n contacts)
    UINT16          Flags;                                  // See InputSessionFlag_* flags
    UINT32          TimeStamp;                              // Driver timestamp
    ULONG64         Source;                                 // HANDLE to the Source of the Touch data
    UINT32          SessionID;                              // Gesture session ID
    UINT8           Reserved;                               // Reserved for byte alignment 
    UINT8           InRangeContactCount;                    // The count of contacts that are marked InRange
    UINT8           InContactContactCount;                  // The count of contacts that are marked Down or Move
    UINT8           ContactCount;                           // Count of TouchContact data points
    TouchContact    ContactArray[c_cTouchContactsMaximum];  // Collection of contacts
} TouchInfo;
#endif // __cplusplus 


//
// Min TouchInfo size is the predefined size (with only 1 TouchContact).
// Max TouchInfo size can include CETOUCHINPUT_MAX_SIMULTANEOUS (16) total
// points, but TouchInfo is defined with one of these already (so -1).
//

#ifdef __cplusplus
static const DWORD c_cbTouchInfoMinimum = (sizeof(TouchInfo));
static const DWORD c_cbTouchInfoMaximum = 
        (c_cbTouchInfoMinimum + (sizeof(TouchContact) * (c_cTouchContactsMaximum - 1)));
#else // __cplusplus
#define c_cbTouchInfoMinimum (sizeof(TouchInfo) - (sizeof(TouchContact) * (c_cTouchContactsMaximum - 1)))
#define c_cbTouchInfoMaximum (sizeof(TouchInfo))
#endif // __cplusplus 


//
// TouchInfoBuffer is a structure large enough to hold a TouchInfo structure
// including the maximum allowable contacts.
//

#ifdef __cplusplus
struct TouchInfoBuffer
{
    //
    // WARNING: This structure is intended to allow overflow for TouchInfo
    // including c_cTouchContactsMaximum total contacts (one contiguous block)
    //
    
    TouchInfo       ti;
    TouchContact    rgAdditionalContacts[c_cTouchContactsMaximum - 1];
};
#else // __cplusplus
// Currently TouchInfo acts like TouchInfoBuffer does in C++
#endif // __cplusplus 


#ifdef __cplusplus

//
// Min KeyEventInfo size is the predefined size (with only 1 Character).
// Max KeyEventInfo size can include (16) total characters, but KeyEventInfo is
// defined with one character already (so -1).
//

static const DWORD c_cKeyEventCharacterMaximum = 16;

//
// Maximum allowed number of characters (including terminate null character) for
// key namae text
//

static const DWORD c_cKeyNameTextMaximum = 32;


//
// InputDeviceType represents an input device type or set of input device types.
//

enum InputDeviceType
{
    InputDeviceType_Mouse           = 0x0001,
    InputDeviceType_Touch           = 0x0002,
    InputDeviceType_Keyboard        = 0x0004,
    
    InputDeviceType_AllDevicesMask  = 
            InputDeviceType_Mouse       |
            InputDeviceType_Touch       | 
            InputDeviceType_Keyboard
};

#else // __cplusplus 
// not yet needed for C
#endif // __cplusplus 


//
// TOUCH_DEVICE_CAPABILITIES
//

#define TOUCHCAP_EMPTY                  0x00000000
#define TOUCHCAP_HOVER                  0x00000001
#define TOUCHCAP_GEOMETRY               0x00000002
#define TOUCHCAP_ANGLE                  0x00000004


//
// Touch device usage info
// This struct must keep the same as the struct Input::PointerDeviceProperty that is defined
// in \mincore\CoreUI\Dev\MinUser\Input\InputInterop.cs on windows side, so that the device
// usage information can be propagated correctly between MinUser GetPointerDeviceProperties,
// MinUserHost, and TchHID.
//

typedef struct _USAGE_INFO
{
    LONG                    LogMin;
    LONG                    LogMax;
    LONG                    PhyMin;
    LONG                    PhyMax;
    ULONG                   Units;
    ULONG                   UnitsExp;
    USHORT                  UsagePageId;
    USHORT                  UsageId;
} USAGE_INFO, *PUSAGE_INFO;


typedef struct _TOUCH_MAX_INPUTS
{
    UCHAR MaxContactCount;    
    UCHAR MaxHoverCount;
} TOUCH_MAX_INPUTS, *PTOUCH_MAX_INPUTS;



//
// Device IO codes for retrieving the supported device usages from TchHID. There is no 
// FILE_DEVICE_* defined for touch, and FILE_DEVICE_UNKNOWN is used based on a recommendation
// from Windows.
//

#define IOCTL_TCHHID_GET_DEVICE_USAGE_COUNT CTL_CODE(FILE_DEVICE_UNKNOWN, 0x8001, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_TCHHID_GET_DEVICE_USAGES CTL_CODE(FILE_DEVICE_UNKNOWN, 0x8002, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_TCHHID_GET_DEVICE_MAX_INPUTS CTL_CODE(FILE_DEVICE_UNKNOWN, 0x8003, METHOD_BUFFERED, FILE_ANY_ACCESS)


//
// Device name and the symbolic link for the virtual driver device that user-mode applications
// can retrieve touch device usages from.
//

#define TOUCH_USAGE_DEVICE_NAME L"\\Device\\TouchUsages0"
#define TOUCH_USAGE_DEVICE_LINK L"\\DosDevices\\TouchUsages0"
#define TOUCH_USAGE_FILE_PATH L"\\\\.\\TouchUsages0"


//
// The height of the 0D capacitive buttons in number of pixels. Any arbitrary number
// can be used because the buttons are outside fo the regular touch panel on 0D devices.
// It works as long as Chrome and InputStateManager agree on the same button height.
//

#define CAPACITIVE_BUTTON_HEIGHT 100


//
// These constants define the default maximum distance from the display surface edge in physical 
// pixels of the first event of a gesture for the gesture to be recognized as a system edge gesture.  
// Gestures that begin farther away than this distance are never recognized as edge gestures.  
// Additionally, edge gestures are only recognized when they begin over a visual that has registered 
// to receive edge gestures.
//
// Values for the constants assume a 480x800 display size.  At runtime the values are scaled up or
// down based on the actual screen size.
//
// The default value may be overwritten by an oem in the registry based on specific hardware 
// characteristcs.  More sensitive hardware could use a lower value, which reduces the latency
// added into any gesture that is a candidate for edge gestures and reduces the likelihood
// of a false positive in the case that user did not intend an edge gesture.  The registry keys 
// examined for overrides are dword values at
//
// HKEY_LOCAL_MACHINE\System\Touch\EdgeGestureThresholds
//      Top
//      Bottom
//      Left
//      Right
//
// Note: these are used in \src\tools\Product\DH\Test\libs\HealthUtils\Native\HealthUtil.cpp
// and \src\uxplat\Splash\Dev\Render\Engine\Components\Desktop\Mobile\EdgeGestureRedirector.cpp
//

#define DEFAULT_VERTICAL_EDGE_GESTURE_THRESHOLD 40
#define DEFAULT_HORIZONTAL_EDGE_GESTURE_THRESHOLD 40


//------------------------------------------------------------------------------
//
// CalculateTouchInfoSize()
//
// Calculate enough space (in bytes) to hold a TouchInfo structure plus any additional
// needed for contact data beyond the mandatory one TouchContact.
//
//------------------------------------------------------------------------------

#ifdef __cplusplus
inline DWORD
CalculateTouchInfoSize(
    DWORD cContacts)    // the number of contacts that the TouchInfo structure will require
#else // __cplusplus 
__inline ULONG
CalculateTouchInfoSize(
    ULONG cContacts)    // the number of contacts that the TouchInfo structure will require
#endif // __cplusplus 
{
    if ((cContacts > c_cTouchContactsMaximum) || (cContacts <= 0))
    {
        // This should never be a request
        return 0;
    }
    else if (cContacts == 1)
    {
        return c_cbTouchInfoMinimum;
    }


    //
    // This is a more complex case where we need enough room for TouchInfo,
    // plus additional space for any more than the first pre-allocated
    // TouchContact.
    //

    return (c_cbTouchInfoMinimum + (sizeof(TouchContact) * (cContacts - 1)));
}

#endif
