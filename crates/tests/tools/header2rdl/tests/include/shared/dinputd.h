/****************************************************************************
 *
 *  Copyright (C) 1995-2000 Microsoft Corporation.  All Rights Reserved.
 *
 *  File:       dinputd.h
 *  Content:    DirectInput include file for device driver implementors
 *
 ****************************************************************************/
#ifndef __DINPUTD_INCLUDED__
#define __DINPUTD_INCLUDED__

#include <winapifamily.h>

#ifndef DIRECTINPUT_VERSION
#define DIRECTINPUT_VERSION         0x0800
#pragma message(__FILE__ ": DIRECTINPUT_VERSION undefined. Defaulting to version 0x0800")
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/****************************************************************************
 *
 *      Interfaces
 *
 ****************************************************************************/

#ifndef DIJ_RINGZERO

DEFINE_GUID(IID_IDirectInputEffectDriver,   0x02538130,0x898F,0x11D0,0x9A,0xD0,0x00,0xA0,0xC9,0xA0,0x6E,0x35);
DEFINE_GUID(IID_IDirectInputJoyConfig,      0x1DE12AB1,0xC9F5,0x11CF,0xBF,0xC7,0x44,0x45,0x53,0x54,0x00,0x00);
DEFINE_GUID(IID_IDirectInputPIDDriver,      0xEEC6993A,0xB3FD,0x11D2,0xA9,0x16,0x00,0xC0,0x4F,0xB9,0x86,0x38);

DEFINE_GUID(IID_IDirectInputJoyConfig8,     0xeb0d7dfa,0x1990,0x4f27,0xb4,0xd6,0xed,0xf2,0xee,0xc4,0xa4,0x4c);

#endif /* DIJ_RINGZERO */


/****************************************************************************
 *
 *      IDirectInputEffectDriver
 *
 ****************************************************************************/

typedef struct DIOBJECTATTRIBUTES {
    DWORD   dwFlags;
    WORD    wUsagePage;
    WORD    wUsage;
} DIOBJECTATTRIBUTES, *LPDIOBJECTATTRIBUTES;
typedef const DIOBJECTATTRIBUTES *LPCDIOBJECTATTRIBUTES;

typedef struct DIFFOBJECTATTRIBUTES {
    DWORD   dwFFMaxForce;
    DWORD   dwFFForceResolution;
} DIFFOBJECTATTRIBUTES, *LPDIFFOBJECTATTRIBUTES;
typedef const DIFFOBJECTATTRIBUTES *LPCDIFFOBJECTATTRIBUTES;

typedef struct DIOBJECTCALIBRATION {
    LONG    lMin;
    LONG    lCenter;
    LONG    lMax;
} DIOBJECTCALIBRATION, *LPDIOBJECTCALIBRATION;
typedef const DIOBJECTCALIBRATION *LPCDIOBJECTCALIBRATION;

typedef struct DIPOVCALIBRATION {
    LONG    lMin[5];
    LONG    lMax[5];
} DIPOVCALIBRATION, *LPDIPOVCALIBRATION;
typedef const DIPOVCALIBRATION *LPCDIPOVCALIBRATION;

typedef struct DIEFFECTATTRIBUTES {
    DWORD   dwEffectId;
    DWORD   dwEffType;
    DWORD   dwStaticParams;
    DWORD   dwDynamicParams;
    DWORD   dwCoords;
} DIEFFECTATTRIBUTES, *LPDIEFFECTATTRIBUTES;
typedef const DIEFFECTATTRIBUTES *LPCDIEFFECTATTRIBUTES;

typedef struct DIFFDEVICEATTRIBUTES {
    DWORD   dwFlags;
    DWORD   dwFFSamplePeriod;
    DWORD   dwFFMinTimeResolution;
} DIFFDEVICEATTRIBUTES, *LPDIFFDEVICEATTRIBUTES;
typedef const DIFFDEVICEATTRIBUTES *LPCDIFFDEVICEATTRIBUTES;

typedef struct DIDRIVERVERSIONS {
    DWORD   dwSize;
    DWORD   dwFirmwareRevision;
    DWORD   dwHardwareRevision;
    DWORD   dwFFDriverVersion;
} DIDRIVERVERSIONS, *LPDIDRIVERVERSIONS;
typedef const DIDRIVERVERSIONS *LPCDIDRIVERVERSIONS;

typedef struct DIDEVICESTATE {
    DWORD   dwSize;
    DWORD   dwState;
    DWORD   dwLoad;
} DIDEVICESTATE, *LPDIDEVICESTATE;

#define DEV_STS_EFFECT_RUNNING  DIEGES_PLAYING

#ifndef DIJ_RINGZERO

typedef struct DIHIDFFINITINFO {
    DWORD   dwSize;
    LPWSTR  pwszDeviceInterface;
    GUID    GuidInstance;
} DIHIDFFINITINFO, *LPDIHIDFFINITINFO;

#undef INTERFACE
#define INTERFACE IDirectInputEffectDriver

DECLARE_INTERFACE_(IDirectInputEffectDriver, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    /*** IDirectInputEffectDriver methods ***/
    STDMETHOD(DeviceID)(THIS_ DWORD,DWORD,DWORD,DWORD,LPVOID) PURE;
    STDMETHOD(GetVersions)(THIS_ LPDIDRIVERVERSIONS) PURE;
    STDMETHOD(Escape)(THIS_ DWORD,DWORD,LPDIEFFESCAPE) PURE;
    STDMETHOD(SetGain)(THIS_ DWORD,DWORD) PURE;
    STDMETHOD(SendForceFeedbackCommand)(THIS_ DWORD,DWORD) PURE;
    STDMETHOD(GetForceFeedbackState)(THIS_ DWORD,LPDIDEVICESTATE) PURE;
    STDMETHOD(DownloadEffect)(THIS_ DWORD,DWORD,LPDWORD,LPCDIEFFECT,DWORD) PURE;
    STDMETHOD(DestroyEffect)(THIS_ DWORD,DWORD) PURE;
    STDMETHOD(StartEffect)(THIS_ DWORD,DWORD,DWORD,DWORD) PURE;
    STDMETHOD(StopEffect)(THIS_ DWORD,DWORD) PURE;
    STDMETHOD(GetEffectStatus)(THIS_ DWORD,DWORD,LPDWORD) PURE;
};

typedef struct IDirectInputEffectDriver *LPDIRECTINPUTEFFECTDRIVER;

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IDirectInputEffectDriver_QueryInterface(p,a,b) (p)->lpVtbl->QueryInterface(p,a,b)
#define IDirectInputEffectDriver_AddRef(p) (p)->lpVtbl->AddRef(p)
#define IDirectInputEffectDriver_Release(p) (p)->lpVtbl->Release(p)
#define IDirectInputEffectDriver_DeviceID(p,a,b,c,d,e) (p)->lpVtbl->DeviceID(p,a,b,c,d,e)
#define IDirectInputEffectDriver_GetVersions(p,a) (p)->lpVtbl->GetVersions(p,a)
#define IDirectInputEffectDriver_Escape(p,a,b,c) (p)->lpVtbl->Escape(p,a,b,c)
#define IDirectInputEffectDriver_SetGain(p,a,b) (p)->lpVtbl->SetGain(p,a,b)
#define IDirectInputEffectDriver_SendForceFeedbackCommand(p,a,b) (p)->lpVtbl->SendForceFeedbackCommand(p,a,b)
#define IDirectInputEffectDriver_GetForceFeedbackState(p,a,b) (p)->lpVtbl->GetForceFeedbackState(p,a,b)
#define IDirectInputEffectDriver_DownloadEffect(p,a,b,c,d,e) (p)->lpVtbl->DownloadEffect(p,a,b,c,d,e)
#define IDirectInputEffectDriver_DestroyEffect(p,a,b) (p)->lpVtbl->DestroyEffect(p,a,b)
#define IDirectInputEffectDriver_StartEffect(p,a,b,c,d) (p)->lpVtbl->StartEffect(p,a,b,c,d)
#define IDirectInputEffectDriver_StopEffect(p,a,b) (p)->lpVtbl->StopEffect(p,a,b)
#define IDirectInputEffectDriver_GetEffectStatus(p,a,b,c) (p)->lpVtbl->GetEffectStatus(p,a,b,c)
#else
#define IDirectInputEffectDriver_QueryInterface(p,a,b) (p)->QueryInterface(a,b)
#define IDirectInputEffectDriver_AddRef(p) (p)->AddRef()
#define IDirectInputEffectDriver_Release(p) (p)->Release()
#define IDirectInputEffectDriver_DeviceID(p,a,b,c,d,e) (p)->DeviceID(a,b,c,d,e)
#define IDirectInputEffectDriver_GetVersions(p,a) (p)->GetVersions(a)
#define IDirectInputEffectDriver_Escape(p,a,b,c) (p)->Escape(a,b,c)
#define IDirectInputEffectDriver_SetGain(p,a,b) (p)->SetGain(a,b)
#define IDirectInputEffectDriver_SendForceFeedbackCommand(p,a,b) (p)->SendForceFeedbackCommand(a,b)
#define IDirectInputEffectDriver_GetForceFeedbackState(p,a,b) (p)->GetForceFeedbackState(a,b)
#define IDirectInputEffectDriver_DownloadEffect(p,a,b,c,d,e) (p)->DownloadEffect(a,b,c,d,e)
#define IDirectInputEffectDriver_DestroyEffect(p,a,b) (p)->DestroyEffect(a,b)
#define IDirectInputEffectDriver_StartEffect(p,a,b,c,d) (p)->StartEffect(a,b,c,d)
#define IDirectInputEffectDriver_StopEffect(p,a,b) (p)->StopEffect(a,b)
#define IDirectInputEffectDriver_GetEffectStatus(p,a,b,c) (p)->GetEffectStatus(a,b,c)
#endif


#endif /* DIJ_RINGZERO */


/****************************************************************************
 *
 *      IDirectInputJoyConfig
 *
 ****************************************************************************/

/****************************************************************************
 *
 *      Definitions copied from the DDK
 *
 ****************************************************************************/

#ifndef JOY_HW_NONE

/* pre-defined joystick types */
#define JOY_HW_NONE                     0
#define JOY_HW_CUSTOM                   1
#define JOY_HW_2A_2B_GENERIC            2
#define JOY_HW_2A_4B_GENERIC            3
#define JOY_HW_2B_GAMEPAD               4
#define JOY_HW_2B_FLIGHTYOKE            5
#define JOY_HW_2B_FLIGHTYOKETHROTTLE    6
#define JOY_HW_3A_2B_GENERIC            7
#define JOY_HW_3A_4B_GENERIC            8
#define JOY_HW_4B_GAMEPAD               9
#define JOY_HW_4B_FLIGHTYOKE            10
#define JOY_HW_4B_FLIGHTYOKETHROTTLE    11
#define JOY_HW_TWO_2A_2B_WITH_Y         12
#define JOY_HW_LASTENTRY                13


/* calibration flags */
#define JOY_ISCAL_XY            0x00000001l     /* XY are calibrated */
#define JOY_ISCAL_Z             0x00000002l     /* Z is calibrated */
#define JOY_ISCAL_R             0x00000004l     /* R is calibrated */
#define JOY_ISCAL_U             0x00000008l     /* U is calibrated */
#define JOY_ISCAL_V             0x00000010l     /* V is calibrated */
#define JOY_ISCAL_POV           0x00000020l     /* POV is calibrated */

/* point of view constants */
#define JOY_POV_NUMDIRS          4
#define JOY_POVVAL_FORWARD       0
#define JOY_POVVAL_BACKWARD      1
#define JOY_POVVAL_LEFT          2
#define JOY_POVVAL_RIGHT         3

/* Specific settings for joystick hardware */
#define JOY_HWS_HASZ            0x00000001l     /* has Z info? */
#define JOY_HWS_HASPOV          0x00000002l     /* point of view hat present */
#define JOY_HWS_POVISBUTTONCOMBOS 0x00000004l   /* pov done through combo of buttons */
#define JOY_HWS_POVISPOLL       0x00000008l     /* pov done through polling */
#define JOY_HWS_ISYOKE          0x00000010l     /* joystick is a flight yoke */
#define JOY_HWS_ISGAMEPAD       0x00000020l     /* joystick is a game pad */
#define JOY_HWS_ISCARCTRL       0x00000040l     /* joystick is a car controller */
/* X defaults to J1 X axis */
#define JOY_HWS_XISJ1Y          0x00000080l     /* X is on J1 Y axis */
#define JOY_HWS_XISJ2X          0x00000100l     /* X is on J2 X axis */
#define JOY_HWS_XISJ2Y          0x00000200l     /* X is on J2 Y axis */
/* Y defaults to J1 Y axis */
#define JOY_HWS_YISJ1X          0x00000400l     /* Y is on J1 X axis */
#define JOY_HWS_YISJ2X          0x00000800l     /* Y is on J2 X axis */
#define JOY_HWS_YISJ2Y          0x00001000l     /* Y is on J2 Y axis */
/* Z defaults to J2 Y axis */
#define JOY_HWS_ZISJ1X          0x00002000l     /* Z is on J1 X axis */
#define JOY_HWS_ZISJ1Y          0x00004000l     /* Z is on J1 Y axis */
#define JOY_HWS_ZISJ2X          0x00008000l     /* Z is on J2 X axis */
/* POV defaults to J2 Y axis, if it is not button based */
#define JOY_HWS_POVISJ1X        0x00010000l     /* pov done through J1 X axis */
#define JOY_HWS_POVISJ1Y        0x00020000l     /* pov done through J1 Y axis */
#define JOY_HWS_POVISJ2X        0x00040000l     /* pov done through J2 X axis */
/* R defaults to J2 X axis */
#define JOY_HWS_HASR            0x00080000l     /* has R (4th axis) info */
#define JOY_HWS_RISJ1X          0x00100000l     /* R done through J1 X axis */
#define JOY_HWS_RISJ1Y          0x00200000l     /* R done through J1 Y axis */
#define JOY_HWS_RISJ2Y          0x00400000l     /* R done through J2 X axis */
/* U & V for future hardware */
#define JOY_HWS_HASU            0x00800000l     /* has U (5th axis) info */
#define JOY_HWS_HASV            0x01000000l     /* has V (6th axis) info */

/* Usage settings */
#define JOY_US_HASRUDDER        0x00000001l     /* joystick configured with rudder */
#define JOY_US_PRESENT          0x00000002l     /* is joystick actually present? */
#define JOY_US_ISOEM            0x00000004l     /* joystick is an OEM defined type */

/* reserved for future use -> as link to next possible dword */
#define JOY_US_RESERVED         0x80000000l     /* reserved */


/* Settings for TypeInfo Flags1 */
#define JOYTYPE_ZEROGAMEENUMOEMDATA     0x00000001l /* Zero GameEnum's OEM data field */
#define JOYTYPE_NOAUTODETECTGAMEPORT    0x00000002l /* Device does not support Autodetect gameport*/
#define JOYTYPE_NOHIDDIRECT             0x00000004l /* Do not use HID directly for this device */
#define JOYTYPE_ANALOGCOMPAT            0x00000008l /* Expose the analog compatible ID */
#define JOYTYPE_DEFAULTPROPSHEET        0x80000000l /* CPL overrides custom property sheet */

/* Settings for TypeInfo Flags2 */
#define JOYTYPE_DEVICEHIDE              0x00010000l /* Hide unclassified devices */
#define JOYTYPE_MOUSEHIDE               0x00020000l /* Hide mice */
#define JOYTYPE_KEYBHIDE                0x00040000l /* Hide keyboards */
#define JOYTYPE_GAMEHIDE                0x00080000l /* Hide game controllers */
#define JOYTYPE_HIDEACTIVE              0x00100000l /* Hide flags are active */
#define JOYTYPE_INFOMASK                0x00E00000l /* Mask for type specific info */
#define JOYTYPE_INFODEFAULT             0x00000000l /* Use default axis mappings */
#define JOYTYPE_INFOYYPEDALS            0x00200000l /* Use Y as a combined pedals axis */
#define JOYTYPE_INFOZYPEDALS            0x00400000l /* Use Z for accelerate, Y for brake */
#define JOYTYPE_INFOYRPEDALS            0x00600000l /* Use Y for accelerate, R for brake */
#define JOYTYPE_INFOZRPEDALS            0x00800000l /* Use Z for accelerate, R for brake */
#define JOYTYPE_INFOZISSLIDER           0x00200000l /* Use Z as a slider */
#define JOYTYPE_INFOZISZ                0x00400000l /* Use Z as Z axis */
#define JOYTYPE_ENABLEINPUTREPORT       0x01000000l /* Enable initial input reports */

/* struct for storing x,y, z, and rudder values */
typedef struct joypos_tag {
    DWORD       dwX;
    DWORD       dwY;
    DWORD       dwZ;
    DWORD       dwR;
    DWORD       dwU;
    DWORD       dwV;
} JOYPOS, FAR *LPJOYPOS;

/* struct for storing ranges */
typedef struct joyrange_tag {
    JOYPOS      jpMin;
    JOYPOS      jpMax;
    JOYPOS      jpCenter;
} JOYRANGE,FAR *LPJOYRANGE;

/*
 *  dwTimeout - value at which to timeout joystick polling
 *  jrvRanges - range of values app wants returned for axes
 *  jpDeadZone - area around center to be considered
 *               as "dead". specified as a percentage
 *               (0-100). Only X & Y handled by system driver
 */
typedef struct joyreguservalues_tag {
    DWORD       dwTimeOut;
    JOYRANGE    jrvRanges;
    JOYPOS      jpDeadZone;
} JOYREGUSERVALUES, FAR *LPJOYREGUSERVALUES;

typedef struct joyreghwsettings_tag {
    DWORD       dwFlags;
    DWORD       dwNumButtons;
} JOYREGHWSETTINGS, FAR *LPJOYHWSETTINGS;

/* range of values returned by the hardware (filled in by calibration) */
/*
 *  jrvHardware - values returned by hardware
 *  dwPOVValues - POV values returned by hardware
 *  dwCalFlags  - what has been calibrated
 */
typedef struct joyreghwvalues_tag {
    JOYRANGE    jrvHardware;
    DWORD       dwPOVValues[JOY_POV_NUMDIRS];
    DWORD       dwCalFlags;
} JOYREGHWVALUES, FAR *LPJOYREGHWVALUES;

/* hardware configuration */
/*
 *  hws             - hardware settings
 *  dwUsageSettings - usage settings
 *  hwv             - values returned by hardware
 *  dwType          - type of joystick
 *  dwReserved      - reserved for OEM drivers
 */
typedef struct joyreghwconfig_tag {
    JOYREGHWSETTINGS    hws;
    DWORD               dwUsageSettings;
    JOYREGHWVALUES      hwv;
    DWORD               dwType;
    DWORD               dwReserved;
} JOYREGHWCONFIG, FAR *LPJOYREGHWCONFIG;

/* joystick calibration info structure */
typedef struct joycalibrate_tag {
    UINT    wXbase;
    UINT    wXdelta;
    UINT    wYbase;
    UINT    wYdelta;
    UINT    wZbase;
    UINT    wZdelta;
} JOYCALIBRATE;
typedef JOYCALIBRATE FAR *LPJOYCALIBRATE;

#endif

#ifndef DIJ_RINGZERO

#define MAX_JOYSTRING 256
typedef BOOL (FAR PASCAL * LPDIJOYTYPECALLBACK)(LPCWSTR, LPVOID);

#ifndef MAX_JOYSTICKOEMVXDNAME
#define MAX_JOYSTICKOEMVXDNAME 260
#endif

#define DITC_REGHWSETTINGS          0x00000001
#define DITC_CLSIDCONFIG            0x00000002
#define DITC_DISPLAYNAME            0x00000004
#define DITC_CALLOUT                0x00000008
#define DITC_HARDWAREID             0x00000010
#define DITC_FLAGS1                 0x00000020
#define DITC_FLAGS2                 0x00000040
#define DITC_MAPFILE                0x00000080



/* This structure is defined for DirectX 5.0 compatibility */

typedef struct DIJOYTYPEINFO_DX5 {
    DWORD dwSize;
    JOYREGHWSETTINGS hws;
    CLSID clsidConfig;
    WCHAR wszDisplayName[MAX_JOYSTRING];
    WCHAR wszCallout[MAX_JOYSTICKOEMVXDNAME];
} DIJOYTYPEINFO_DX5, *LPDIJOYTYPEINFO_DX5;
typedef const DIJOYTYPEINFO_DX5 *LPCDIJOYTYPEINFO_DX5;

/* This structure is defined for DirectX 6.1 compatibility */
typedef struct DIJOYTYPEINFO_DX6 {
    DWORD dwSize;
    JOYREGHWSETTINGS hws;
    CLSID clsidConfig;
    WCHAR wszDisplayName[MAX_JOYSTRING];
    WCHAR wszCallout[MAX_JOYSTICKOEMVXDNAME];
    WCHAR wszHardwareId[MAX_JOYSTRING];
    DWORD dwFlags1;
} DIJOYTYPEINFO_DX6, *LPDIJOYTYPEINFO_DX6;
typedef const DIJOYTYPEINFO_DX6 *LPCDIJOYTYPEINFO_DX6;

typedef struct DIJOYTYPEINFO {
    DWORD dwSize;
    JOYREGHWSETTINGS hws;
    CLSID clsidConfig;
    WCHAR wszDisplayName[MAX_JOYSTRING];
    WCHAR wszCallout[MAX_JOYSTICKOEMVXDNAME];
#if(DIRECTINPUT_VERSION >= 0x05b2)
    WCHAR wszHardwareId[MAX_JOYSTRING];
    DWORD dwFlags1;
#if(DIRECTINPUT_VERSION >= 0x0800)
    DWORD dwFlags2;
    WCHAR wszMapFile[MAX_JOYSTRING];
#endif /* DIRECTINPUT_VERSION >= 0x0800 */
#endif /* DIRECTINPUT_VERSION >= 0x05b2 */
} DIJOYTYPEINFO, *LPDIJOYTYPEINFO;
typedef const DIJOYTYPEINFO *LPCDIJOYTYPEINFO;
#define DIJC_GUIDINSTANCE           0x00000001
#define DIJC_REGHWCONFIGTYPE        0x00000002
#define DIJC_GAIN                   0x00000004
#define DIJC_CALLOUT                0x00000008
#define DIJC_WDMGAMEPORT            0x00000010

/* This structure is defined for DirectX 5.0 compatibility */

typedef struct DIJOYCONFIG_DX5 {
    DWORD dwSize;
    GUID guidInstance;
    JOYREGHWCONFIG hwc;
    DWORD dwGain;
    WCHAR wszType[MAX_JOYSTRING];
    WCHAR wszCallout[MAX_JOYSTRING];
} DIJOYCONFIG_DX5, *LPDIJOYCONFIG_DX5;
typedef const DIJOYCONFIG_DX5 *LPCDIJOYCONFIG_DX5;

typedef struct DIJOYCONFIG {
    DWORD dwSize;
    GUID guidInstance;
    JOYREGHWCONFIG hwc;
    DWORD dwGain;
    WCHAR wszType[MAX_JOYSTRING];
    WCHAR wszCallout[MAX_JOYSTRING];
#if(DIRECTINPUT_VERSION >= 0x05b2)
    GUID  guidGameport;
#endif /* DIRECTINPUT_VERSION >= 0x05b2 */
    } DIJOYCONFIG, *LPDIJOYCONFIG;
typedef const DIJOYCONFIG *LPCDIJOYCONFIG;


#define DIJU_USERVALUES             0x00000001
#define DIJU_GLOBALDRIVER           0x00000002
#define DIJU_GAMEPORTEMULATOR       0x00000004

typedef struct DIJOYUSERVALUES {
    DWORD dwSize;
    JOYREGUSERVALUES ruv;
    WCHAR wszGlobalDriver[MAX_JOYSTRING];
    WCHAR wszGameportEmulator[MAX_JOYSTRING];
} DIJOYUSERVALUES, *LPDIJOYUSERVALUES;
typedef const DIJOYUSERVALUES *LPCDIJOYUSERVALUES;

DEFINE_GUID(GUID_KeyboardClass, 0x4D36E96B,0xE325,0x11CE,0xBF,0xC1,0x08,0x00,0x2B,0xE1,0x03,0x18);
DEFINE_GUID(GUID_MediaClass,    0x4D36E96C,0xE325,0x11CE,0xBF,0xC1,0x08,0x00,0x2B,0xE1,0x03,0x18);
DEFINE_GUID(GUID_MouseClass,    0x4D36E96F,0xE325,0x11CE,0xBF,0xC1,0x08,0x00,0x2B,0xE1,0x03,0x18);
DEFINE_GUID(GUID_HIDClass,      0x745A17A0,0x74D3,0x11D0,0xB6,0xFE,0x00,0xA0,0xC9,0x0F,0x57,0xDA);

#undef INTERFACE
#define INTERFACE IDirectInputJoyConfig

DECLARE_INTERFACE_(IDirectInputJoyConfig, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    /*** IDirectInputJoyConfig methods ***/
    STDMETHOD(Acquire)(THIS) PURE;
    STDMETHOD(Unacquire)(THIS) PURE;
    STDMETHOD(SetCooperativeLevel)(THIS_ HWND,DWORD) PURE;
    STDMETHOD(SendNotify)(THIS) PURE;
    STDMETHOD(EnumTypes)(THIS_ LPDIJOYTYPECALLBACK,LPVOID) PURE;
    STDMETHOD(GetTypeInfo)(THIS_ LPCWSTR,LPDIJOYTYPEINFO,DWORD) PURE;
    STDMETHOD(SetTypeInfo)(THIS_ LPCWSTR,LPCDIJOYTYPEINFO,DWORD) PURE;
    STDMETHOD(DeleteType)(THIS_ LPCWSTR) PURE;
    STDMETHOD(GetConfig)(THIS_ UINT,LPDIJOYCONFIG,DWORD) PURE;
    STDMETHOD(SetConfig)(THIS_ UINT,LPCDIJOYCONFIG,DWORD) PURE;
    STDMETHOD(DeleteConfig)(THIS_ UINT) PURE;
    STDMETHOD(GetUserValues)(THIS_ LPDIJOYUSERVALUES,DWORD) PURE;
    STDMETHOD(SetUserValues)(THIS_ LPCDIJOYUSERVALUES,DWORD) PURE;
    STDMETHOD(AddNewHardware)(THIS_ HWND,REFGUID) PURE;
    STDMETHOD(OpenTypeKey)(THIS_ LPCWSTR,DWORD,PHKEY) PURE;
    STDMETHOD(OpenConfigKey)(THIS_ UINT,DWORD,PHKEY) PURE;
};

typedef struct IDirectInputJoyConfig *LPDIRECTINPUTJOYCONFIG;

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IDirectInputJoyConfig_QueryInterface(p,a,b) (p)->lpVtbl->QueryInterface(p,a,b)
#define IDirectInputJoyConfig_AddRef(p) (p)->lpVtbl->AddRef(p)
#define IDirectInputJoyConfig_Release(p) (p)->lpVtbl->Release(p)
#define IDirectInputJoyConfig_Acquire(p) (p)->lpVtbl->Acquire(p)
#define IDirectInputJoyConfig_Unacquire(p) (p)->lpVtbl->Unacquire(p)
#define IDirectInputJoyConfig_SetCooperativeLevel(p,a,b) (p)->lpVtbl->SetCooperativeLevel(p,a,b)
#define IDirectInputJoyConfig_SendNotify(p) (p)->lpVtbl->SendNotify(p)
#define IDirectInputJoyConfig_EnumTypes(p,a,b) (p)->lpVtbl->EnumTypes(p,a,b)
#define IDirectInputJoyConfig_GetTypeInfo(p,a,b,c) (p)->lpVtbl->GetTypeInfo(p,a,b,c)
#define IDirectInputJoyConfig_SetTypeInfo(p,a,b,c) (p)->lpVtbl->SetTypeInfo(p,a,b,c)
#define IDirectInputJoyConfig_DeleteType(p,a) (p)->lpVtbl->DeleteType(p,a)
#define IDirectInputJoyConfig_GetConfig(p,a,b,c) (p)->lpVtbl->GetConfig(p,a,b,c)
#define IDirectInputJoyConfig_SetConfig(p,a,b,c) (p)->lpVtbl->SetConfig(p,a,b,c)
#define IDirectInputJoyConfig_DeleteConfig(p,a) (p)->lpVtbl->DeleteConfig(p,a)
#define IDirectInputJoyConfig_GetUserValues(p,a,b) (p)->lpVtbl->GetUserValues(p,a,b)
#define IDirectInputJoyConfig_SetUserValues(p,a,b) (p)->lpVtbl->SetUserValues(p,a,b)
#define IDirectInputJoyConfig_AddNewHardware(p,a,b) (p)->lpVtbl->AddNewHardware(p,a,b)
#define IDirectInputJoyConfig_OpenTypeKey(p,a,b,c) (p)->lpVtbl->OpenTypeKey(p,a,b,c)
#define IDirectInputJoyConfig_OpenConfigKey(p,a,b,c) (p)->lpVtbl->OpenConfigKey(p,a,b,c)
#else
#define IDirectInputJoyConfig_QueryInterface(p,a,b) (p)->QueryInterface(a,b)
#define IDirectInputJoyConfig_AddRef(p) (p)->AddRef()
#define IDirectInputJoyConfig_Release(p) (p)->Release()
#define IDirectInputJoyConfig_Acquire(p) (p)->Acquire()
#define IDirectInputJoyConfig_Unacquire(p) (p)->Unacquire()
#define IDirectInputJoyConfig_SetCooperativeLevel(p,a,b) (p)->SetCooperativeLevel(a,b)
#define IDirectInputJoyConfig_SendNotify(p) (p)->SendNotify()
#define IDirectInputJoyConfig_EnumTypes(p,a,b) (p)->EnumTypes(a,b)
#define IDirectInputJoyConfig_GetTypeInfo(p,a,b,c) (p)->GetTypeInfo(a,b,c)
#define IDirectInputJoyConfig_SetTypeInfo(p,a,b,c) (p)->SetTypeInfo(a,b,c)
#define IDirectInputJoyConfig_DeleteType(p,a) (p)->DeleteType(a)
#define IDirectInputJoyConfig_GetConfig(p,a,b,c) (p)->GetConfig(a,b,c)
#define IDirectInputJoyConfig_SetConfig(p,a,b,c) (p)->SetConfig(a,b,c)
#define IDirectInputJoyConfig_DeleteConfig(p,a) (p)->DeleteConfig(a)
#define IDirectInputJoyConfig_GetUserValues(p,a,b) (p)->GetUserValues(a,b)
#define IDirectInputJoyConfig_SetUserValues(p,a,b) (p)->SetUserValues(a,b)
#define IDirectInputJoyConfig_AddNewHardware(p,a,b) (p)->AddNewHardware(a,b)
#define IDirectInputJoyConfig_OpenTypeKey(p,a,b,c) (p)->OpenTypeKey(a,b,c)
#define IDirectInputJoyConfig_OpenConfigKey(p,a,b,c) (p)->OpenConfigKey(a,b,c)
#endif

#endif /* DIJ_RINGZERO */

#if(DIRECTINPUT_VERSION >= 0x0800)

#ifndef DIJ_RINGZERO

#undef INTERFACE
#define INTERFACE IDirectInputJoyConfig8

DECLARE_INTERFACE_(IDirectInputJoyConfig8, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    /*** IDirectInputJoyConfig8 methods ***/
    STDMETHOD(Acquire)(THIS) PURE;
    STDMETHOD(Unacquire)(THIS) PURE;
    STDMETHOD(SetCooperativeLevel)(THIS_ HWND,DWORD) PURE;
    STDMETHOD(SendNotify)(THIS) PURE;
    STDMETHOD(EnumTypes)(THIS_ LPDIJOYTYPECALLBACK,LPVOID) PURE;
    STDMETHOD(GetTypeInfo)(THIS_ LPCWSTR,LPDIJOYTYPEINFO,DWORD) PURE;
    STDMETHOD(SetTypeInfo)(THIS_ LPCWSTR,LPCDIJOYTYPEINFO,DWORD,LPWSTR) PURE;
    STDMETHOD(DeleteType)(THIS_ LPCWSTR) PURE;
    STDMETHOD(GetConfig)(THIS_ UINT,LPDIJOYCONFIG,DWORD) PURE;
    STDMETHOD(SetConfig)(THIS_ UINT,LPCDIJOYCONFIG,DWORD) PURE;
    STDMETHOD(DeleteConfig)(THIS_ UINT) PURE;
    STDMETHOD(GetUserValues)(THIS_ LPDIJOYUSERVALUES,DWORD) PURE;
    STDMETHOD(SetUserValues)(THIS_ LPCDIJOYUSERVALUES,DWORD) PURE;
    STDMETHOD(AddNewHardware)(THIS_ HWND,REFGUID) PURE;
    STDMETHOD(OpenTypeKey)(THIS_ LPCWSTR,DWORD,PHKEY) PURE;
    STDMETHOD(OpenAppStatusKey)(THIS_ PHKEY) PURE;
};

typedef struct IDirectInputJoyConfig8 *LPDIRECTINPUTJOYCONFIG8;

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IDirectInputJoyConfig8_QueryInterface(p,a,b) (p)->lpVtbl->QueryInterface(p,a,b)
#define IDirectInputJoyConfig8_AddRef(p) (p)->lpVtbl->AddRef(p)
#define IDirectInputJoyConfig8_Release(p) (p)->lpVtbl->Release(p)
#define IDirectInputJoyConfig8_Acquire(p) (p)->lpVtbl->Acquire(p)
#define IDirectInputJoyConfig8_Unacquire(p) (p)->lpVtbl->Unacquire(p)
#define IDirectInputJoyConfig8_SetCooperativeLevel(p,a,b) (p)->lpVtbl->SetCooperativeLevel(p,a,b)
#define IDirectInputJoyConfig8_SendNotify(p) (p)->lpVtbl->SendNotify(p)
#define IDirectInputJoyConfig8_EnumTypes(p,a,b) (p)->lpVtbl->EnumTypes(p,a,b)
#define IDirectInputJoyConfig8_GetTypeInfo(p,a,b,c) (p)->lpVtbl->GetTypeInfo(p,a,b,c)
#define IDirectInputJoyConfig8_SetTypeInfo(p,a,b,c,d) (p)->lpVtbl->SetTypeInfo(p,a,b,c,d)
#define IDirectInputJoyConfig8_DeleteType(p,a) (p)->lpVtbl->DeleteType(p,a)
#define IDirectInputJoyConfig8_GetConfig(p,a,b,c) (p)->lpVtbl->GetConfig(p,a,b,c)
#define IDirectInputJoyConfig8_SetConfig(p,a,b,c) (p)->lpVtbl->SetConfig(p,a,b,c)
#define IDirectInputJoyConfig8_DeleteConfig(p,a) (p)->lpVtbl->DeleteConfig(p,a)
#define IDirectInputJoyConfig8_GetUserValues(p,a,b) (p)->lpVtbl->GetUserValues(p,a,b)
#define IDirectInputJoyConfig8_SetUserValues(p,a,b) (p)->lpVtbl->SetUserValues(p,a,b)
#define IDirectInputJoyConfig8_AddNewHardware(p,a,b) (p)->lpVtbl->AddNewHardware(p,a,b)
#define IDirectInputJoyConfig8_OpenTypeKey(p,a,b,c) (p)->lpVtbl->OpenTypeKey(p,a,b,c)
#define IDirectInputJoyConfig8_OpenAppStatusKey(p,a) (p)->lpVtbl->OpenAppStatusKey(p,a)
#else
#define IDirectInputJoyConfig8_QueryInterface(p,a,b) (p)->QueryInterface(a,b)
#define IDirectInputJoyConfig8_AddRef(p) (p)->AddRef()
#define IDirectInputJoyConfig8_Release(p) (p)->Release()
#define IDirectInputJoyConfig8_Acquire(p) (p)->Acquire()
#define IDirectInputJoyConfig8_Unacquire(p) (p)->Unacquire()
#define IDirectInputJoyConfig8_SetCooperativeLevel(p,a,b) (p)->SetCooperativeLevel(a,b)
#define IDirectInputJoyConfig8_SendNotify(p) (p)->SendNotify()
#define IDirectInputJoyConfig8_EnumTypes(p,a,b) (p)->EnumTypes(a,b)
#define IDirectInputJoyConfig8_GetTypeInfo(p,a,b,c) (p)->GetTypeInfo(a,b,c)
#define IDirectInputJoyConfig8_SetTypeInfo(p,a,b,c,d) (p)->SetTypeInfo(a,b,c,d)
#define IDirectInputJoyConfig8_DeleteType(p,a) (p)->DeleteType(a)
#define IDirectInputJoyConfig8_GetConfig(p,a,b,c) (p)->GetConfig(a,b,c)
#define IDirectInputJoyConfig8_SetConfig(p,a,b,c) (p)->SetConfig(a,b,c)
#define IDirectInputJoyConfig8_DeleteConfig(p,a) (p)->DeleteConfig(a)
#define IDirectInputJoyConfig8_GetUserValues(p,a,b) (p)->GetUserValues(a,b)
#define IDirectInputJoyConfig8_SetUserValues(p,a,b) (p)->SetUserValues(a,b)
#define IDirectInputJoyConfig8_AddNewHardware(p,a,b) (p)->AddNewHardware(a,b)
#define IDirectInputJoyConfig8_OpenTypeKey(p,a,b,c) (p)->OpenTypeKey(a,b,c)
#define IDirectInputJoyConfig8_OpenAppStatusKey(p,a) (p)->OpenAppStatusKey(a)
#endif

#endif /* DIJ_RINGZERO */

/****************************************************************************
 *
 *  Notification Messages
 *
 ****************************************************************************/

/* RegisterWindowMessage with this to get DirectInput notification messages */
#define DIRECTINPUT_NOTIFICATION_MSGSTRINGA  "DIRECTINPUT_NOTIFICATION_MSGSTRING"
#define DIRECTINPUT_NOTIFICATION_MSGSTRINGW  L"DIRECTINPUT_NOTIFICATION_MSGSTRING"

#ifdef UNICODE
#define DIRECTINPUT_NOTIFICATION_MSGSTRING  DIRECTINPUT_NOTIFICATION_MSGSTRINGW
#else
#define DIRECTINPUT_NOTIFICATION_MSGSTRING  DIRECTINPUT_NOTIFICATION_MSGSTRINGA
#endif

#define DIMSGWP_NEWAPPSTART         0x00000001
#define DIMSGWP_DX8APPSTART         0x00000002
#define DIMSGWP_DX8MAPPERAPPSTART   0x00000003

#endif /* DIRECTINPUT_VERSION >= 0x0800 */

#define DIAPPIDFLAG_NOTIME         0x00000001
#define DIAPPIDFLAG_NOSIZE         0x00000002

#define DIRECTINPUT_REGSTR_VAL_APPIDFLAGA   "AppIdFlag"
#define DIRECTINPUT_REGSTR_KEY_LASTAPPA     "MostRecentApplication"
#define DIRECTINPUT_REGSTR_KEY_LASTMAPAPPA  "MostRecentMapperApplication"
#define DIRECTINPUT_REGSTR_VAL_VERSIONA     "Version"
#define DIRECTINPUT_REGSTR_VAL_NAMEA        "Name"
#define DIRECTINPUT_REGSTR_VAL_IDA          "Id"
#define DIRECTINPUT_REGSTR_VAL_MAPPERA      "UsesMapper"
#define DIRECTINPUT_REGSTR_VAL_LASTSTARTA   "MostRecentStart"

#define DIRECTINPUT_REGSTR_VAL_APPIDFLAGW   L"AppIdFlag"
#define DIRECTINPUT_REGSTR_KEY_LASTAPPW     L"MostRecentApplication"
#define DIRECTINPUT_REGSTR_KEY_LASTMAPAPPW  L"MostRecentMapperApplication"
#define DIRECTINPUT_REGSTR_VAL_VERSIONW     L"Version"
#define DIRECTINPUT_REGSTR_VAL_NAMEW        L"Name"
#define DIRECTINPUT_REGSTR_VAL_IDW          L"Id"
#define DIRECTINPUT_REGSTR_VAL_MAPPERW      L"UsesMapper"
#define DIRECTINPUT_REGSTR_VAL_LASTSTARTW   L"MostRecentStart"

#ifdef UNICODE
#define DIRECTINPUT_REGSTR_VAL_APPIDFLAG    DIRECTINPUT_REGSTR_VAL_APPIDFLAGW
#define DIRECTINPUT_REGSTR_KEY_LASTAPP      DIRECTINPUT_REGSTR_KEY_LASTAPPW
#define DIRECTINPUT_REGSTR_KEY_LASTMAPAPP   DIRECTINPUT_REGSTR_KEY_LASTMAPAPPW
#define DIRECTINPUT_REGSTR_VAL_VERSION      DIRECTINPUT_REGSTR_VAL_VERSIONW
#define DIRECTINPUT_REGSTR_VAL_NAME         DIRECTINPUT_REGSTR_VAL_NAMEW
#define DIRECTINPUT_REGSTR_VAL_ID           DIRECTINPUT_REGSTR_VAL_IDW
#define DIRECTINPUT_REGSTR_VAL_MAPPER       DIRECTINPUT_REGSTR_VAL_MAPPERW
#define DIRECTINPUT_REGSTR_VAL_LASTSTART    DIRECTINPUT_REGSTR_VAL_LASTSTARTW
#else
#define DIRECTINPUT_REGSTR_VAL_APPIDFLAG    DIRECTINPUT_REGSTR_VAL_APPIDFLAGA
#define DIRECTINPUT_REGSTR_KEY_LASTAPP      DIRECTINPUT_REGSTR_KEY_LASTAPPA
#define DIRECTINPUT_REGSTR_KEY_LASTMAPAPP   DIRECTINPUT_REGSTR_KEY_LASTMAPAPPA
#define DIRECTINPUT_REGSTR_VAL_VERSION      DIRECTINPUT_REGSTR_VAL_VERSIONA
#define DIRECTINPUT_REGSTR_VAL_NAME         DIRECTINPUT_REGSTR_VAL_NAMEA
#define DIRECTINPUT_REGSTR_VAL_ID           DIRECTINPUT_REGSTR_VAL_IDA
#define DIRECTINPUT_REGSTR_VAL_MAPPER       DIRECTINPUT_REGSTR_VAL_MAPPERA
#define DIRECTINPUT_REGSTR_VAL_LASTSTART    DIRECTINPUT_REGSTR_VAL_LASTSTARTA
#endif


/****************************************************************************
 *
 *  Return Codes
 *
 ****************************************************************************/

#define DIERR_NOMOREITEMS               \
    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIN32, ERROR_NO_MORE_ITEMS)

/*
 *  Device driver-specific codes.
 */

#define DIERR_DRIVERFIRST               0x80040300L
#define DIERR_DRIVERLAST                0x800403FFL

/*
 *  Unless the specific driver has been precisely identified, no meaning 
 *  should be attributed to these values other than that the driver 
 *  originated the error.  However, to illustrate the types of error that 
 *  may be causing the failure, the PID force feedback driver distributed 
 *  with DirectX 7 could return the following errors:
 *
 *  DIERR_DRIVERFIRST + 1   
 *      The requested usage was not found.
 *  DIERR_DRIVERFIRST + 2   
 *      The parameter block couldn't be	downloaded to the device.
 *  DIERR_DRIVERFIRST + 3   
 *      PID initialization failed.
 *  DIERR_DRIVERFIRST + 4   
 *      The provided values couldn't be scaled.
 */


/*
 *  Device installer errors.
 */

/*
 *  Registry entry or DLL for class installer invalid
 *  or class installer not found.
 */
#define DIERR_INVALIDCLASSINSTALLER     0x80040400L

/*
 *  The user cancelled the install operation.
 */
#define DIERR_CANCELLED                 0x80040401L

/*
 *  The INF file for the selected device could not be
 *  found or is invalid or is damaged.
 */
#define DIERR_BADINF                    0x80040402L

/****************************************************************************
 *
 *  Map files
 *
 ****************************************************************************/

/*
 *  Delete particular data from default map file.
 */
#define DIDIFT_DELETE                   0x01000000

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
};
#endif

#endif  /* __DINPUTD_INCLUDED__ */
