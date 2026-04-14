/****************************************************************************
*
*  (C) COPYRIGHT 1999-2000, MICROSOFT CORP.
*
*  FILE:        wiamicro.h
*
*  VERSION:     3.0
*
*  DESCRIPTION:
*    Definitions to support WIA scanner and camera microdrivers.
*
*****************************************************************************/

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <SCSISCAN.H>

#define WIAMICRO_API __declspec(dllexport)

#include <pshpack8.h>

/****************************************************************************\
* Scanner microdriver definitions
\****************************************************************************/

//
// Private #defines
//

#define MAX_IO_HANDLES 16
#define MAX_RESERVED    4
#define MAX_ANSI_CHAR 255

//
// Common BUS types
//

#define BUS_TYPE_SCSI         200
#define BUS_TYPE_USB          201
#define BUS_TYPE_PARALLEL     202
#define BUS_TYPE_FIREWIRE     203

//
// command list
//

#define SCAN_FIRST             10
#define SCAN_NEXT              20
#define SCAN_FINISHED          30

#define SCANMODE_FINALSCAN     0
#define SCANMODE_PREVIEWSCAN   1

#define CMD_INITIALIZE        100
#define CMD_UNINITIALIZE      101
#define CMD_SETXRESOLUTION    102
#define CMD_SETYRESOLUTION    103
#define CMD_SETCONTRAST       104
#define CMD_SETINTENSITY      105
#define CMD_SETDATATYPE       106
#define CMD_SETDITHER         107
#define CMD_SETMIRROR         108
#define CMD_SETNEGATIVE       109
#define CMD_SETTONEMAP        110
#define CMD_SETCOLORDITHER    111
#define CMD_SETMATRIX         112
#define CMD_SETSPEED          113
#define CMD_SETFILTER         114
#define CMD_LOAD_ADF          115
#define CMD_UNLOAD_ADF        116
#define CMD_GETADFAVAILABLE   117
#define CMD_GETADFOPEN        118
#define CMD_GETADFREADY       119
#define CMD_GETADFHASPAPER    120
#define CMD_GETADFSTATUS      121
#define CMD_GETADFUNLOADREADY 122
#define CMD_GETTPAAVAILABLE   123
#define CMD_GETTPAOPENED      124
#define CMD_TPAREADY          125
#define CMD_SETLAMP           126
#define CMD_SENDSCSICOMMAND   127
#define CMD_STI_DEVICERESET   128
#define CMD_STI_GETSTATUS     129
#define CMD_STI_DIAGNOSTIC    130
#define CMD_RESETSCANNER      131
#define CMD_GETCAPABILITIES   132
#define CMD_GET_INTERRUPT_EVENT 133
#define CMD_SETGSDNAME        134
#define CMD_SETSCANMODE       135
#define CMD_SETSTIDEVICEHKEY  136
#define CMD_GETSUPPORTEDFILEFORMATS 138
#define CMD_GETSUPPORTEDMEMORYFORMATS 139
#define CMD_SETFORMAT   140

#define SUPPORT_COLOR      0x00000001
#define SUPPORT_BW         0x00000002
#define SUPPORT_GRAYSCALE  0x00000004

//
// Error Codes
//

#define MCRO_ERROR_GENERAL_ERROR     0 // All lVal values are initialized to '0'
#define MCRO_STATUS_OK               1 // General success status return
#define MCRO_ERROR_PAPER_JAM         2 // ADF has a paper Jam
#define MCRO_ERROR_PAPER_PROBLEM     3 // ADF has a paper problem
#define MCRO_ERROR_PAPER_EMPTY       4 // ADF has no paper
#define MCRO_ERROR_OFFLINE           5 // ADF or Device is offline
#define MCRO_ERROR_USER_INTERVENTION 6 // User needs to interact with the physical device

//
// WIA compatible #defines
//

#define WIA_PACKED_PIXEL         0
#define WIA_PLANAR               1

#define WIA_ORDER_RGB            0
#define WIA_ORDER_BGR            1

#define WIA_DATA_THRESHOLD       0
#define WIA_DATA_DITHER          1
#define WIA_DATA_GRAYSCALE       2
#define WIA_DATA_COLOR           3
#define WIA_DATA_COLOR_THRESHOLD 4
#define WIA_DATA_COLOR_DITHER    5

//
// structure definitions
//

typedef struct _RANGEVALUE {
    LONG lMin;                  // minimum value
    LONG lMax;                  // maximum value
    LONG lStep;                 // increment/step value
} RANGEVALUE, *PRANGEVALUE;

typedef struct _SCANWINDOW {
    LONG xPos;                  // X position (left)
    LONG yPos;                  // Y position (top)
    LONG xExtent;               // X extent   (right)
    LONG yExtent;               // Y extent   (bottom)
} SCANWINDOW, *PSCANWINDOW;

typedef struct _SCANINFO {
    // Common Scanner specs
    LONG ADF;                   // (0 - no support,  1 - supported, 2 - supported and It can duplex)
    LONG TPA;                   // (0 - no support,  1 - supported)
    LONG Endorser;              // (0 - no endorser, 1 - supported)
    LONG OpticalXResolution;    // (dpi setting of optics)
    LONG OpticalYResolution;    // (dpi setting of optics)
    LONG BedWidth;              // (bed width in 1000's of an inch)
    LONG BedHeight;             // (bed height in 1000's of an inch)
    RANGEVALUE IntensityRange;  // (Intensity/Brightness ranges)
    RANGEVALUE ContrastRange;   // (Contrast ranges)
    LONG SupportedCompressionType; // (mask of supported compression types, 0 - None)
    LONG SupportedDataTypes;    // (mask of supported types, (ie. SUPPORT_COLOR|SUPPORT_BW...))
    // Current Image Info
    LONG WidthPixels;           // (width of image, using current scanner settings in pixels)
    LONG WidthBytes;            // (width of image, using current scanner settings in bytes)
    LONG Lines;                 // (height of image, using current scanner settings in pixles)
    LONG DataType;              // (current data type set)
    LONG PixelBits;             // (current bit depth setting)
    // Current Scanner settings
    LONG Intensity;             // (current Intensity/Brightness setting)
    LONG Contrast;              // (current contrast setting)
    LONG Xresolution;           // (current X Resolution)
    LONG Yresolution;           // (current Y Resolution
    SCANWINDOW Window;          // (current scanner window settings)
    // Scanner options
    LONG DitherPattern;
    LONG Negative;              // (0 - off,        1 - Negative is on)
    LONG Mirror;                // (0 - off,        1 - Mirror is on)
    LONG AutoBack;              // (0 - off,        1 - AutoBack is on)
    LONG ColorDitherPattern;    // (dither pattern??)
    LONG ToneMap;               // (tone map ??)
    LONG Compression;           // (0 - off,        1 - Compression is on)
    LONG RawDataFormat;         // (0 - Packed data 1 - Planar data)
    LONG RawPixelOrder;         // (0 - RGB,        1 - BGR)
    LONG bNeedDataAlignment;    // (0 - FALSE,      1 - TRUE)
    LONG DelayBetweenRead;      // delay between WIA Scan() calls requesting data (milliseconds)
    LONG MaxBufferSize;         // maximum buffer size in scanner
    HANDLE DeviceIOHandles[MAX_IO_HANDLES]; // Device IO handles needed for device communication
    LONG lReserved[MAX_RESERVED]; // (silly reserved bits)
    VOID *pMicroDriverContext;  // private data for Micro driver's only.
                                // The Micro Driver is responsible for allocating and freeing.
                                // CMD_INITIALIZE - allocate, CMD_UNINITIALIZE - free
}SCANINFO, *PSCANINFO;

typedef struct VAL {
        LONG      lVal;                 // long value
        double    dblVal;               // float/double value
        GUID     *pGuid;                // GUID pointer
        PSCANINFO pScanInfo;            // pointer to the shared ScanInfo struct
        HGLOBAL   handle;               // handle value
        WCHAR   **ppButtonNames;        // pointer to button names array
        HANDLE   *pHandle;              // pointer to a Handle value
        LONG      lReserved;            // lone value
        CHAR      szVal[MAX_ANSI_CHAR]; // ANSI string
}VAL, *PVAL;

//
// Micro driver entry points
//

WIAMICRO_API HRESULT MicroEntry(LONG lCommand, _Inout_ PVAL pValue);
WIAMICRO_API HRESULT Scan(_Inout_ PSCANINFO pScanInfo, LONG lPhase, _Out_writes_bytes_(lLength) PBYTE pBuffer, LONG lLength, _Out_ LONG *plReceived);
WIAMICRO_API HRESULT SetPixelWindow(_Inout_ PSCANINFO pScanInfo, LONG x, LONG y, LONG xExtent, LONG yExtent);

//
// optional debug trace
//

VOID Trace(_In_ LPCTSTR Format, ...);

#include <poppack.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //#if (_WIN32_WINNT >= 0x0501)
