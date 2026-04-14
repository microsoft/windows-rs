/*********************************************************************
*
* WTSDEFS.H
*
*   Windows Terminal Server Interface Header File
*
*   Copyright (c) 1997-2001 Microsoft Corporation
*
**********************************************************************/

#ifndef _INC_WTSDEFS
#define _INC_WTSDEFS

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif


/*****************************************************************
 *  DEFINES
 ****************************************************************/
 
#define WTS_DOMAIN_LENGTH            255
#define WTS_USERNAME_LENGTH          255
#define WTS_PASSWORD_LENGTH          255
#define WTS_DIRECTORY_LENGTH         256
#define WTS_INITIALPROGRAM_LENGTH    256
#define WTS_PROTOCOL_NAME_LENGTH     8
#define WTS_DRIVER_NAME_LENGTH       8
#define WTS_DEVICE_NAME_LENGTH       19
#define WTS_IMEFILENAME_LENGTH       32
#define WTS_CLIENTNAME_LENGTH        20
#define WTS_CLIENTADDRESS_LENGTH     30
#define WTS_CLIENT_PRODUCT_ID_LENGTH 32
#define WTS_MAX_PROTOCOL_CACHE       4
#define WTS_MAX_CACHE_RESERVED       20
#define WTS_MAX_RESERVED             100
#define WTS_MAX_COUNTERS             100
#define WTS_MAX_DISPLAY_IOCTL_DATA   256

//
// Fields used in the PerformanceFlags property in WTS_CONNECTION_CONFIG
// Setting a flag disables that feature
//
#define WTS_PERF_DISABLE_NOTHING                              0x00000000
#define WTS_PERF_DISABLE_WALLPAPER                            0x00000001
#define WTS_PERF_DISABLE_FULLWINDOWDRAG                       0x00000002
#define WTS_PERF_DISABLE_MENUANIMATIONS                       0x00000004
#define WTS_PERF_DISABLE_THEMING                              0x00000008
#define WTS_PERF_ENABLE_ENHANCED_GRAPHICS                     0x00000010
#define WTS_PERF_DISABLE_CURSOR_SHADOW                        0x00000020
#define WTS_PERF_DISABLE_CURSORSETTINGS                       0x00000040
#define WTS_PERF_ENABLE_FONT_SMOOTHING                        0x00000080
#define WTS_PERF_ENABLE_DESKTOP_COMPOSITION                   0x00000100


/*****************************************************************
 *  STRUCTURES
 ****************************************************************/

typedef BYTE *PBYTE;

// IP address
//
typedef struct _WTS_SOCKADDR {
#ifdef __midl
    USHORT sin_family;
    union switch (unsigned short sin_family) u 
    {
        case 2:         // AF_INET
        struct{
            USHORT sin_port;
            ULONG  in_addr;
            UCHAR  sin_zero[8];
        } ipv4;
        case 23:        // AF_INET6
        struct {
            USHORT sin6_port;
            ULONG  sin6_flowinfo;
            USHORT sin6_addr[8];
            ULONG  sin6_scope_id;
        } ipv6;
    };
#else
    USHORT sin_family;
    union {
        struct{
            USHORT sin_port;
            ULONG  in_addr;
            UCHAR  sin_zero[8];
        } ipv4;
        struct {
            USHORT sin6_port;
            ULONG  sin6_flowinfo;
            USHORT sin6_addr[8];
            ULONG  sin6_scope_id;
        } ipv6;               
    } u;
#endif
} _WTS_SOCKADDR, WTS_SOCKADDR, *PWTS_SOCKADDR;

typedef struct _WTS_SMALL_RECT {  
    SHORT Left;  
    SHORT Top;  
    SHORT Right;  
    SHORT Bottom;
} _WTS_SMALL_RECT, WTS_SMALL_RECT, *PWTS_SMALL_RECT;

// =================================================================
//  Service State Change
// =================================================================
typedef enum {
    WTS_SERVICE_NONE,                   // No service (start/stop) state change
    WTS_SERVICE_START,                  // RCM Service starting
    WTS_SERVICE_STOP                    // RCM Service stopping
} WTS_RCM_SERVICE_STATE;

typedef enum {
    WTS_DRAIN_STATE_NONE,               // No drain state change
    WTS_DRAIN_IN_DRAIN,                 // RCM In drain.
    WTS_DRAIN_NOT_IN_DRAIN              // RCM Out of drain.
} WTS_RCM_DRAIN_STATE;

typedef struct _WTS_SERVICE_STATE {
    WTS_RCM_SERVICE_STATE   RcmServiceState;
    WTS_RCM_DRAIN_STATE     RcmDrainState;
} _WTS_SERVICE_STATE, WTS_SERVICE_STATE, *PWTS_SERVICE_STATE;

typedef struct _WTS_SESSION_ID {
    GUID    SessionUniqueGuid;
    ULONG   SessionId;
} _WTS_SESSION_ID, WTS_SESSION_ID, *PWTS_SESSION_ID;

typedef struct _WTS_USER_CREDENTIAL {
    WCHAR   UserName[ WTS_USERNAME_LENGTH + 1 ];
    WCHAR   Password[ WTS_PASSWORD_LENGTH + 1 ];
    WCHAR   Domain[ WTS_DOMAIN_LENGTH + 1 ];
} _WTS_USER_CREDENTIAL, WTS_USER_CREDENTIAL, *PWTS_USER_CREDENTIAL;

/* -------------------------------------------------------------------
 * Structures used to query connectoin specific data from the protocol
 * ------------------------------------------------------------------*/
typedef struct _WTS_SYSTEMTIME {
    USHORT wYear;
    USHORT wMonth;
    USHORT wDayOfWeek;
    USHORT wDay;
    USHORT wHour;
    USHORT wMinute;
    USHORT wSecond;
    USHORT wMilliseconds;
} _WTS_SYSTEMTIME, WTS_SYSTEMTIME, *PWTS_SYSTEMTIME;

typedef struct _WTS_TIME_ZONE_INFORMATION {
    LONG Bias;
    WCHAR StandardName[ 32 ];
    WTS_SYSTEMTIME StandardDate;
    LONG StandardBias;
    WCHAR DaylightName[ 32 ];
    WTS_SYSTEMTIME DaylightDate;
    LONG DaylightBias;
} _WTS_TIME_ZONE_INFORMATION, WTS_TIME_ZONE_INFORMATION, *PWTS_TIME_ZONE_INFORMATION;

typedef struct _WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    LONG Bias;
    WCHAR StandardName[ 32 ];
    WTS_SYSTEMTIME StandardDate;
    LONG StandardBias;
    WCHAR DaylightName[ 32 ];
    WTS_SYSTEMTIME DaylightDate;
    LONG DaylightBias;
    WCHAR TimeZoneKeyName[ 128 ];
    USHORT DynamicDaylightTimeDisabled;
} WRDS_DYNAMIC_TIME_ZONE_INFORMATION, *PWRDS_DYNAMIC_TIME_ZONE_INFORMATION;

typedef struct _WTS_CLIENT_DATA {
  
    /* --------------------------------------------------------
     * Properties that will be used by RCM for the session. 
     * These MIGHT be exposed through WTSAPI
     * -------------------------------------------------------*/
     
    // Disable Ctrl-Atl-Del
    BOOLEAN fDisableCtrlAltDel;    
    
    // Detect Double Click
    BOOLEAN fDoubleClickDetect;    
    
    // Enable Windows key
    // Will be sent to win32k
    BOOLEAN fEnableWindowsKey;
         
    // Hide the title bar
    // Will be sent to win32k
    BOOLEAN fHideTitleBar; 
   
    // This will overwrite the fInheritAutoLogon flag 
    // from listener registry
    BOOL fInheritAutoLogon;
    
    // Prompt for password even if autologon set
    // Will be used if fInheritAutoLogon is set
    // and policy not set on the machine
    BOOLEAN fPromptForPassword;
        
    // Client used saved credentials
    BOOLEAN fUsingSavedCreds;
    
    // User credentials
    // Will be used if fInheritAutoLogon is set
    WCHAR Domain[WTS_DOMAIN_LENGTH + 1];   // WTS_DOMAIN_LENGTH + 1
    WCHAR UserName[ WTS_USERNAME_LENGTH + 1 ];
    WCHAR Password[ WTS_PASSWORD_LENGTH + 1 ];

    // Smartcard PIN is used to log in 
    // Will be used if fInheritAutoLogon is set
    BOOLEAN fPasswordIsScPin;
    
    // This will overwrite the fInheritInitialProgram flag 
    // from listener registry
    BOOL fInheritInitialProgram;
    
    // Working directory
    // Will be used if fInheritInitialProgram is set
    WCHAR WorkDirectory[ WTS_DIRECTORY_LENGTH + 1 ];
    
    // Intial Program to start in TS session
    // Will be used if fInheritInitialProgram is set
    WCHAR InitialProgram[ WTS_INITIALPROGRAM_LENGTH + 1 ];
    
    // Maximize the shell
    // Will be used if fInheritInitialProgram is set
    BOOLEAN fMaximizeShell;        
    
    // Security level of encryption
    BYTE EncryptionLevel;
        
    // List of features to disable for perf
    // Will be used to disable specific features
    ULONG PerformanceFlags;

    // Name of the protocol
    // Will be sent to win32k
    WCHAR ProtocolName[ WTS_PROTOCOL_NAME_LENGTH + 1 ];

    // Protocol type
    USHORT ProtocolType;
    
    // This will overwrite the fInheritColorDepth flag 
    // from listener registry
    BOOL fInheritColorDepth;
    
    // Display properties
    // Will be sent to win32k
    USHORT HRes;
    USHORT VRes;
    USHORT ColorDepth;
    WCHAR  DisplayDriverName[ WTS_DRIVER_NAME_LENGTH + 1 ];
    WCHAR  DisplayDeviceName[ WTS_DEVICE_NAME_LENGTH + 1 ];
        
    // Enable Mouse input
    // Will be sent to win32k
    BOOLEAN fMouse;

    // Keyboard information
    // Will be sent to win32k
    ULONG KeyboardLayout;
    ULONG KeyboardType;
    ULONG KeyboardSubType;
    ULONG KeyboardFunctionKey;
    WCHAR imeFileName[ WTS_IMEFILENAME_LENGTH + 1 ];

    // Client's active input locale HKL
    ULONG ActiveInputLocale; 
    
    // Client selected no audio
    BOOLEAN fNoAudioPlayback;
    
    // Client selected leave audio at remote computer
    BOOLEAN fRemoteConsoleAudio;  

    // Name of the audio driver
    // Will be sent to win32k
    WCHAR AudioDriverName[ WTS_DRIVER_NAME_LENGTH + 1 ];

    // Client time zone information
    // Will be sent to win32k
    WTS_TIME_ZONE_INFORMATION ClientTimeZone;
   
    /* ----------------------------------------------------
     * Properties that are not used by RCM but exposed 
     * through WTS API
     * ---------------------------------------------------*/ 
     
    // Client machine name
    WCHAR ClientName[ WTS_CLIENTNAME_LENGTH + 1 ];
    
    // Client computer's unique serial number
    ULONG SerialNumber; 
        
    // Client IP address
    ULONG ClientAddressFamily;
    WCHAR ClientAddress[ WTS_CLIENTADDRESS_LENGTH + 1 ];
    WTS_SOCKADDR ClientSockAddress;
    
    // Client Directory
    WCHAR ClientDirectory[ WTS_DIRECTORY_LENGTH + 1 ];
    
    // Client build number
    ULONG ClientBuildNumber;
    
    // Client software product id
    USHORT ClientProductId;     
    
    // number of output buffers on host
    USHORT OutBufCountHost;
      
    // number of output buffers on client
    USHORT OutBufCountClient;
    
    // length of output buffers in bytes
    USHORT OutBufLength;
    
    // Client Session ID
    ULONG ClientSessionId;
    
    // Client Product ID
    WCHAR ClientDigProductId[ WTS_CLIENT_PRODUCT_ID_LENGTH + 1 ];
    
    // ******* REDIRECTIONS ********//
    // These will be prepopulated with data from policies
    // Changing them will led to policies being overwritten 
    
    // Disable Printer Mapping
    BOOLEAN fDisableCpm;
    
    // Disable Drive Mapping
    BOOLEAN fDisableCdm;
    
    // Disable COM port Mapping
    BOOLEAN fDisableCcm;
    
    // Disable LPT printer redirection
    BOOLEAN fDisableLPT;
    
    // Disable Clipboard redirection
    BOOLEAN fDisableClip;

    // Disable PNP redirection
    BOOLEAN fDisablePNP;
} _WTS_CLIENT_DATA, WTS_CLIENT_DATA, *PWTS_CLIENT_DATA;

typedef struct _WTS_USER_DATA {
  
    /* --------------------------------------------------------
     * Should be filled in with user specific properties.
     * Queried after user has logged in and User GP merged.
     * This will be pre-populated with client data. Hence if user 
     * property doesn't exist for any field, do not change it.
     * -------------------------------------------------------*/
     
    // Corresponding to WorkDirectory in WTS_CLIENT_DATA
    WCHAR WorkDirectory[ WTS_DIRECTORY_LENGTH + 1 ];
    
    // Corresponding to InitialProgram in WTS_CLIENT_DATA
    WCHAR InitialProgram[ WTS_INITIALPROGRAM_LENGTH + 1 ];
    
    // Corresponding to ClientTimeZone in WTS_CLIENT_DATA
    WTS_TIME_ZONE_INFORMATION UserTimeZone;

} WTS_USER_DATA, *PWTS_USER_DATA;

/* -------------------------------------------------------------------
 * Structures used to pass down policy data to the protocol
 * ------------------------------------------------------------------*/
typedef struct _WTS_POLICY_DATA {
    // Disable encryption
    BOOLEAN fDisableEncryption;

    // Disable auto-reconnect
    BOOLEAN fDisableAutoReconnect;

    // Color Depth
    ULONG ColorDepth;

    // Minimum allowed Encryption Level
    BYTE MinEncryptionLevel;
    
    // ******* REDIRECTIONS ********//
    // Disable Printer Mapping
    BOOLEAN fDisableCpm;
    
    // Disable Drive Mapping
    BOOLEAN fDisableCdm;
    
    // Disable COM port Mapping
    BOOLEAN fDisableCcm;
    
    // Disable LPT printer redirection
    BOOLEAN fDisableLPT;
    
    // Disable Clipboard redirection
    BOOLEAN fDisableClip;

    // Disable PNP redirection
    BOOLEAN fDisablePNPRedir;
    
} WTS_POLICY_DATA, *PWTS_POLICY_DATA;

/* -------------------------------------------------------------------
 * Structures used to query performance data from the protocol using
 * GetProtocolStatus call 
 * -------------------------------------------------------------------*/

// Protocol Cache stats
//
typedef struct _WTS_PROTOCOL_CACHE {
    ULONG CacheReads;
    ULONG CacheHits;
} _WTS_PROTOCOL_CACHE, WTS_PROTOCOL_CACHE, *PWTS_PROTOCOL_CACHE;

// Union of Protocol cache, TShare Cache and Reserved stats
//
#ifdef __midl
typedef [switch_type(DWORD)] union _WTS_CACHE_STATS_UN {
    [case(1)]
        WTS_PROTOCOL_CACHE ProtocolCache[ WTS_MAX_PROTOCOL_CACHE ];  
    [case(2)]
        ULONG TShareCacheStats;
    [case(3)]
        ULONG Reserved[ WTS_MAX_CACHE_RESERVED ];    // Protocol specific Reserved data
} _WTS_CACHE_STATS_UN, WTS_CACHE_STATS_UN, *PWTS_CACHE_STATS_UN;

// Cache Statistics
//
typedef struct _WTS_CACHE_STATS {
    DWORD Specific;
    [switch_is(Specific)] WTS_CACHE_STATS_UN Data;
    USHORT ProtocolType;        // Protocol Type
    USHORT Length;              // Length of Data
} _WTS_CACHE_STATS, WTS_CACHE_STATS, *PWTS_CACHE_STATS;
#else
typedef union _WTS_CACHE_STATS_UN {
    WTS_PROTOCOL_CACHE ProtocolCache[ WTS_MAX_PROTOCOL_CACHE ];  
    ULONG TShareCacheStats;
    ULONG Reserved[ WTS_MAX_CACHE_RESERVED ];    // Protocol specific Reserved data
} _WTS_CACHE_STATS_UN, WTS_CACHE_STATS_UN, *PWTS_CACHE_STATS_UN;

// Cache Statistics
//
typedef struct _WTS_CACHE_STATS {
    DWORD Specific;
    WTS_CACHE_STATS_UN Data;
    USHORT ProtocolType;        // Protocol Type
    USHORT Length;              // Length of Data
} _WTS_CACHE_STATS, WTS_CACHE_STATS, *PWTS_CACHE_STATS;
#endif

// Protocol Counters Data
//
typedef struct _WTS_PROTOCOL_COUNTERS {
    ULONG WdBytes;              // wd common
    ULONG WdFrames;             // wd common
    ULONG WaitForOutBuf;        // wd common
    ULONG Frames;               // td common
    ULONG Bytes;                // td common
    ULONG CompressedBytes;      // pdcomp
    ULONG CompressFlushes;      // pdcomp
    ULONG Errors;               // pdreli
    ULONG Timeouts;             // pdreli
    ULONG AsyncFramingError;    // pdasync
    ULONG AsyncOverrunError;    // pdasync
    ULONG AsyncOverflowError;   // pdasync
    ULONG AsyncParityError;     // pdasync
    ULONG TdErrors;             // td common
    USHORT ProtocolType;        // Protocol Type
    USHORT Length;              // Length of Reserved
    USHORT Specific;            // Specifies type of Reserved
    ULONG Reserved[ WTS_MAX_RESERVED ];        
} _WTS_PROTOCOL_COUNTERS, WTS_PROTOCOL_COUNTERS, * PWTS_PROTOCOL_COUNTERS;

// Protocol Status
//
typedef struct _WTS_PROTOCOL_STATUS {
    WTS_PROTOCOL_COUNTERS Output;
    WTS_PROTOCOL_COUNTERS Input;
    WTS_CACHE_STATS Cache;
    ULONG AsyncSignal;                  // MS_CTS_ON, MS_DSR_ON, etc...
    ULONG AsyncSignalMask;              // EV_CTS, EV_DSR, etc...
    LARGE_INTEGER  Counters[ WTS_MAX_COUNTERS ];      
} _WTS_PROTOCOL_STATUS, WTS_PROTOCOL_STATUS, * PWTS_PROTOCOL_STATUS;

// Display IOCTL data
typedef struct _WTS_DISPLAY_IOCTL {
    BYTE  pDisplayIOCtlData[ WTS_MAX_DISPLAY_IOCTL_DATA ];
    ULONG cbDisplayIOCtlData;
} _WTS_DISPLAY_IOCTL, WTS_DISPLAY_IOCTL, * PWTS_DISPLAY_IOCTL;

/* -------------------------------------------------------------------
 * LogonUI Error Redirector Response
 * -------------------------------------------------------------------*/
typedef enum _WTS_LOGON_ERROR_REDIRECTOR_RESPONSE
{
    // Used only as a safe initialization value.
    WTS_LOGON_ERR_INVALID = 0,

    // The call was not handled by the redirector.   
    // The call should be handled normally by LogonUI.
    WTS_LOGON_ERR_NOT_HANDLED,

    // The call was handled by the redirector.
    // LogonUI should paint itself normally.
    WTS_LOGON_ERR_HANDLED_SHOW,

    // The call was handled by the redirector and should not be passed to
    // the next redirector.  LogonUI should not paint the normal error screen.
    // LogonUI will then attempt to gather credentials again.
    WTS_LOGON_ERR_HANDLED_DONT_SHOW,

    // The call was handled by the redirector and should not be passed to
    // the next redirector.  LogonUI should not show itself and should
    // NOT attempt to gather credentials again.
    WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER,

} WTS_LOGON_ERROR_REDIRECTOR_RESPONSE;


/* -------------------------------------------------------------------
 * QueryProperty GUIDs
 * These are the mandatory properties that the protocol must implement.
 * For any other QueryProperty that the protocol doesn't understand,
 * it should return E_NOTIMPL
 * -------------------------------------------------------------------*/

#define WTS_VALUE_TYPE_ULONG        1
#define WTS_VALUE_TYPE_STRING       2
#define WTS_VALUE_TYPE_BINARY       3
#define WTS_VALUE_TYPE_GUID         4

// Basic structure used to query properties.
// This can be used as both input and output parameter
typedef struct __WTS_PROPERTY_VALUE
{
#ifdef __midl
    union switch ( unsigned short Type ) u {
        case WTS_VALUE_TYPE_ULONG:
            ULONG   ulVal;
        case WTS_VALUE_TYPE_STRING:
            struct {
                [range(0, 0x8000)]  ULONG size;
                [string, size_is(size)] WCHAR* pstrVal;
            } strVal;
        case WTS_VALUE_TYPE_BINARY:
            struct {
                [range(0, 0x8000)]  ULONG size;
                [size_is(size)]     char* pbVal;
            } bVal;                
        case WTS_VALUE_TYPE_GUID:
            GUID   guidVal;
        };

#else
    unsigned short Type;
    union {
        ULONG ulVal;
        struct {
            ULONG size;
            WCHAR* pstrVal;
        } strVal;
        struct {
            ULONG size;
            char* pbVal;
        } bVal; 
        GUID guidVal;               
    } u;
#endif
} __WTS_PROPERTY_VALUE, WTS_PROPERTY_VALUE, *PWTS_PROPERTY_VALUE;

#ifndef __midl

//
// **************** WTS_QUERY_ALLOWED_INITIAL_APP *********************
//
// This is used to query if protocol wants to allow an initial app to run.
// If protocol wants initial app to run, it must implement this property 
// query and return success.
// 
// Input to this query - It passes 3 WTS_PROPERTY_VALUE structures
//          pPropertyEntriesIn[0].Type = WTS_VALUE_TYPE_STRING;
//          pPropertyEntriesIn[0].u.strVal.pstrVal = NAME of the inital app, ex. notepad.exe
//          pPropertyEntriesIn[0].u.strVal.size = Length of the name string
//      
//          pPropertyEntriesIn[1].Type = WTS_VALUE_TYPE_STRING;
//          pPropertyEntriesIn[1].u.strVal.pstrVal = PARAMETERS for the inital app
//          pPropertyEntriesIn[1].u.strVal.size = Length of the parameter string
//      
//          pPropertyEntriesIn[2].Type = WTS_VALUE_TYPE_ULONG;
//          pPropertyEntriesIn[2].u.ulVal = Reserved
//      
// Output to this query - It passes 3 WTS_PROPERTY_VALUE structures. Protocol needs to set it to:
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_STRING;
//          pPropertyEntriesOut[0].u.strVal.pstrVal = Full command line to execute. Protocol should 
//                                                    append current working dir to the app name and put here
//          pPropertyEntriesOut[0].u.strVal.size = Length of command line
//
//          pPropertyEntriesOut[1].Type = WTS_VALUE_TYPE_STRING;
//          pPropertyEntriesOut[1].u.strVal.pstrVal = PARAMETERS for the inital app
//          pPropertyEntriesOut[1].u.strVal.size = Length of the parameter string
//
//      If protocol want to maintain the initial app and parameters passed in input parameter, 
//      it should just copy them to the corresponding fields (two above) in output paramters
//      
//          pPropertyEntriesOut[2].Type = WTS_VALUE_TYPE_ULONG;
//          pPropertyEntriesOut[2].u.ulVal = 0 indicates don't allow initial app to run, allow otherwise
//
EXTERN_C const __declspec(selectany) GUID WTS_QUERY_ALLOWED_INITIAL_APP = /*C77D1B30-5BE1-4c6b-A0E1-BD6D2E5C9FCC*/
{ 0xc77d1b30, 0x5be1, 0x4c6b, { 0xa0, 0xe1, 0xbd, 0x6d, 0x2e, 0x5c, 0x9f, 0xcc} };

// **************** WTS_QUERY_LOGON_SCREEN_SIZE *********************
//
// This is used by LogonUI to query the size of the logon screen.
// 
// Input to this query - NONE
//      
// Output to this query - It passes 1 WTS_PROPERTY_VALUE structure. Protocol needs to set it to:
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_ULONG;
//          pPropertyEntriesOut[0].u.strVal.pstrVal = Size of the LogonUI screen.
//
// If protocol doesn't want to keep the default size of the LogonUI screen, it must fail this call with error E_NOTIMPL.
//
EXTERN_C const __declspec(selectany) GUID WTS_QUERY_LOGON_SCREEN_SIZE = /* 8b8e0fe7-0804-4a0e-b279-8660b1df0049 */
{ 0x8b8e0fe7, 0x0804, 0x4a0e, {0xb2, 0x79, 0x86, 0x60, 0xb1, 0xdf, 0x00, 0x49} };

// **************** WTS_QUERY_AUDIOENUM_DLL **************************
//
// This is used by to query Remote audio enumerator DLL name.
// 
// Input to this query - NONE
//      
// Output to this query - 
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_STRING;
//          pPropertyEntriesOut[0].u.strVal.pstrVal =  Remote audio enumerator DLL name
//          pPropertyEntriesOut[0].u.strVal.size = Length of DLL name
//
//
EXTERN_C const __declspec(selectany) GUID WTS_QUERY_AUDIOENUM_DLL = /* 9bf4fa97-c883-4c2a-80ab-5a39c9af00db */
{ 0x9bf4fa97, 0xc883, 0x4c2a, {0x80, 0xab, 0x5a, 0x39, 0xc9, 0xaf, 0x00, 0xdb} };

// ************************************************************************

// **************** WTS_QUERY_MF_FORMAT_SUPPORT **************************
//
// This is used by Remote Desktop Media Foundation Plugin to query the sink 
// objects to be used for specified media formats.
// 
// Input to this query is a WTS_PROPERTY_VALUE structures containing
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_BINARY;
//          pPropertyEntriesOut[0].u.bVal.pbVal = TSMF_SUPPORT_DATA_IN structure
//          pPropertyEntriesOut[0].u.bVal.size = Size of TSMF_SUPPORT_DATA_IN structure. 
//               This will vary based on the size of the variable data in the structure.
//      
// Output to this query is a WTS_PROPERTY_VALUE structures containing
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_BINARY;
//          pPropertyEntriesOut[0].u.bVal.pbVal = TSMF_SUPPORT_DATA_OUT structure
//          pPropertyEntriesOut[0].u.bVal.size = Size of TSMF_SUPPORT_DATA_OUT structure.
//                This will vary based on the size of the variable data in the structure.
//
//  STRUCTURE DEFINITIONS
//
//          typedef struct tagTSMF_SUPPORT_DATA_IN {
//              GUID        guidMfSession;
//              //number of nodes in input data.
//              UINT32      numEntries;   
//              //
//              // variable length data. The data is of type TSMF_SUPPORT_NODEDATA_IN
//              // for each node corresponding to numEntries
//              //
//          } TSMF_SUPPORT_DATA_IN, *PTSMF_SUPPORT_DATA_IN;
//           
//          typedef struct tagTSMF_SUPPORT_NODEDATA_IN {
//              UINT32      byteCount;
//              INT64       nodeId;
//              // Number of media type entries for this node
//              UINT32      numMediaTypes;    
//              //
//              // variable length media format data. It should be read as a set of TS_AM_MEDIA_TYPE structs.
//              // The format itself is of type FORMAT_WaveFormatEx for audio and FORMAT_MFVideoFormat
//              // for video
//              //
//          } TSMF_SUPPORT_NODEDATA_IN, *PTSMF_SUPPORT_NODEDATA_IN;
//           
//           
//          // 
//          // Structures used to format data output from RCM. This is the format expected from protocol extension
//          //
//          typedef struct tagTSMF_SUPPORT_DATA_OUT {
//              GUID        guidMfSession; // must match the MF session guid in input data
//              UINT32      numEntries; // must match the number of entries in input data
//           
//              //
//              // variable length data. The data is of type TSMF_SUPPORT_NODEDATA_OUT
//              // for each entry corresponding to numEntries
//              //
//          } TSMF_SUPPORT_DATA_OUT, *PTSMF_SUPPORT_DATA_OUT;
//           
//          typedef struct tagTSMF_SUPPORT_NODEDATA_OUT {
//              INT64       nodeId;
//              //
//              // Status of 0 means not supported, 1 means supported.  
//              // Other values reserved for future use
//              // If supported, the sink given by clsidNewSink will be used
//              //
//              HRESULT     hrSupportStatus; 
//              CLSID       clsidNewSink;
//              // 0 based index of the media type for which support is being expressed.
//              UINT32      supportedMediaTypeIndex;
//          } TSMF_SUPPORT_NODEDATA_OUT, *PTSMF_SUPPORT_NODEDATA_OUT;
//           
//
//          // The following struct is used to express the media type. The fields in this struct have the same meaning as AM_MEDIA_TYPE
//          // This struct is used to ensure consistency between 32-bit and 64-bit callers
//          //
//          typedef struct tagTS_AM_MEDIA_TYPE {
//              GUID        majortype;
//              GUID        subtype;
//              UINT32    bFixedSizeSamples;
//              UINT32    bTemporalCompression;
//              UINT32    lSampleSize;
//              GUID        formattype;
//              UINT32    cbFormat;
//              // Variable size data appened at the end
//          } TS_AM_MEDIA_TYPE, *PTS_AM_MEDIA_TYPE;
//          
//
EXTERN_C const __declspec(selectany) GUID WTS_QUERY_MF_FORMAT_SUPPORT = /*41869ad0-6332-4dc8-95d5-db749e2f1d94*/
{ 0x41869ad0, 0x6332, 0x4dc8, {0x95, 0xd5, 0xdb, 0x74, 0x9e, 0x2f, 0x1d, 0x94} };


// ************************************************************************

// **************** WRDS_SERVICE_ID_GRAPHICS_GUID *************************
//
// This is used to retrieve an IWRdsGraphics interface pointer from
// IWRdsServiceProvider
//

EXTERN_C const __declspec(selectany) GUID WRDS_SERVICE_ID_GRAPHICS_GUID = /* d2993f4d-2cf-4280-8c48-1624b44f8706 */
	{0xd2993f4d, 0x2cf, 0x4280, { 0x8c, 0x48, 0x16, 0x24, 0xb4, 0x4f, 0x87, 0x6 } };

// ************************************************************************

// ************** PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION ******************
// 
// This is used to retrieve the dynamic time zone information from
// a connection. During the connection process, the Remote Connection
// Manager will query the connection to see if it supports this property.
// When present, this property enables sessions with Time Zone 
// Redirection enabled to have their applications retrieve the complete 
// DYNAMIC_TIME_ZONE_INFORMATION when calling GetDynamicTimeZoneInformation().
//
// 
// There are no inputs to this query.
//      
// Output to this query is a WTS_PROPERTY_VALUE structures containing
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_BINARY;
//          pPropertyEntriesOut[0].u.bVal.pbVal = WRDS_DYNAMIC_TIME_ZONE_INFORMATION structure
//          pPropertyEntriesOut[0].u.bVal.size = Size of WRDS_DYNAMIC_TIME_ZONE_INFORMATION structure.
//
// See above for definition of the WRDS_DYNAMIC_TIME_ZONE_INFORMATION struct
// 
EXTERN_C const __declspec(selectany) GUID PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION = /* cdfd28e-d0b9-4c1f-a5eb-6d1f6c6535b9 */
{ 0xcdfd28e, 0xd0b9, 0x4c1f, { 0xa5, 0xeb, 0x6d, 0x1f, 0x6c, 0x65, 0x35, 0xb9 } };


// ************************************************************************


// ************** PROPERTY_GET_FAST_RECONNECT ******************
// 
// This is used to get what type of fast reconnect (if any) should be used based on the stack
// 
// There are no inputs to this query.
//
// Output to this query - It passes 1 WTS_PROPERTY_VALUE structure. Protocol needs to set it to:
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_ULONG;
//          pPropertyEntriesOut[0].u.ulVal = (0 = Don't do fast reconnect, 1 = Basic Fast Reconnect, 2 = Enhanced Fast reconnect)
//
EXTERN_C const _declspec(selectany) GUID PROPERTY_TYPE_GET_FAST_RECONNECT = /* 6212d757-0043-4862-99c3-9f3059ac2a3b*/
{ 0x6212d757, 0x0043, 0x4862,{ 0x99, 0xc3, 0x9f, 0x30, 0x59, 0xac, 0x2a, 0x3b } };


// ************************************************************************

// ************** PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID ******************
// 
// This is used to get the User SID that should be used to filter for sessions to fast reconnect to.
// This is used only in the case of Enhanced Fast Reconnect
// 
// There are no inputs to this query.
//
// Output to this query - It passes 1 WTS_PROPERTY_VALUE structure. Protocol needs to set it to:
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_STRING;
//          pPropertyEntriesOut[0].u.strVal.pstrVal = User SID to be used as filter
//          pPropertyEntriesOut[0].u.strVal.size = Length of User SID
//
EXTERN_C const _declspec(selectany) GUID PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID = /* 197c427a-0135-4b6d-9c5e-e6579a0ab625*/
{ 0x197c427a, 0x0135, 0x4b6d,{ 0x9c, 0x5e, 0xe6, 0x57, 0x9a, 0x0a, 0xb6, 0x25 } };


// ************************************************************************

// ************** PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL ******************
// 
// This is used to get whether Universal Apps should be enabled or not for Custom shells based on the stack
// 
// There are no inputs to this query.
//
// Output to this query - It passes 1 WTS_PROPERTY_VALUE structure. Protocol needs to set it to:
//          pPropertyEntriesOut[0].Type = WTS_VALUE_TYPE_ULONG;
//          pPropertyEntriesOut[0].u.ulVal = (0 = Don't enable Universal Apps for Custom Shell, 1 = Enable Universal Apps for Custom Shell)
//
EXTERN_C const _declspec(selectany) GUID PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL = /* ed2c3fda-338d-4d3f-81a3-e767310d908e*/
{ 0xed2c3fda, 0x338d, 0x4d3f,{ 0x81, 0xa3, 0xe7, 0x67, 0x31, 0x0d, 0x90, 0x8e } };

#endif

#ifndef __midl

/* -------------------------------------------------------------------
 * IWrdsProtocolConnectionSettings GUIDs
 * 
 * There are the GUIDS that can be passed in to Get/Set Settings under IWrdsProtocolConnectionSettings
 *
 * -------------------------------------------------------------------*/

EXTERN_C const _declspec(selectany) GUID CONNECTION_PROPERTY_IDLE_TIME_WARNING = /* 693f7ff5-0c4e-4d17-b8e0-1f70325e5d58*/
{ 0x693F7FF5, 0x0C4E, 0x4D17,{ 0xB8, 0xE0, 0x1F, 0x70, 0x32, 0x5E, 0x5D, 0x58 } };

EXTERN_C const _declspec(selectany) GUID CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED = /* 4B150580-FEA4-4D3C-9DE4-7433A66618F7 */
{ 0x4B150580, 0xFEA4, 0x4D3C,{ 0x9D, 0xE4, 0x74, 0x33, 0xA6, 0x66, 0x18, 0xF7 } };



#endif

/* -------------------------------------------------------------------
 * Licensing structures
 * 
 * For more informtion on the licensing structures and sequence
 * please refer to [MS-RDPBCGR] - 
 * http://msdn.microsoft.com/en-us/library/cc241880(PROT.10).aspx
 *
 * -------------------------------------------------------------------*/

///////////////////////////////////////////////////////////////////////////////
//
// Key exchange algorithms
//

#define WTS_KEY_EXCHANGE_ALG_RSA    1
#define WTS_KEY_EXCHANGE_ALG_DH     2

#define WTS_LICENSE_PROTOCOL_VERSION            0x00010000
#define WTS_LICENSE_PREAMBLE_VERSION            0x03
#define WTS_LICENSE_CURRENT_PROTOCOL_VERSION    WTS_LICENSE_PROTOCOL_VERSION | WTS_LICENSE_PREAMBLE_VERSION

//-----------------------------------------------------------------------------
//
// Types of certificate used by the server to authenticate itself to the clients
//
// CERT_TYPE_PROPRIETORY
//      Proprietory format certificate
//
// CERT_TYPE_X509
//      X509 format certificate
//
//-----------------------------------------------------------------------------
typedef enum
{
    WTS_CERT_TYPE_INVALID       = 0,
    WTS_CERT_TYPE_PROPRIETORY   = 1,
    WTS_CERT_TYPE_X509          = 2
} WTS_CERT_TYPE;


//-----------------------------------------------------------------------------
//
// WTS_LICENSE_CAPABILITIES
//
// Data structure used in RequestLicensingCapabilities.
//
// KeyExchangeAlg       - The key exchange algorithm: RSA or Diffie Helman (see defines above)
// ProtocolVer          - The supported licensing protocol. 
//                        Always set it to WTS_LICENSE_CURRENT_PROTOCOL_VERSION.
// fAuthenticateServer  - Whether the client is going to authenticate the server
// CertType             - Indicate the type of certificate that has already been transmitted
//                        to the client.
// cbClientName         - Size of the client name in bytes
// rgbClientName        - Name of the client
//
//-----------------------------------------------------------------------------

#define WTS_LICENSE_CLIENT_NAME_BYTE_LENGTH (WTS_CLIENTNAME_LENGTH + 1) * sizeof( WCHAR )

typedef struct _WTS_LICENSE_CAPABILITIES
{
    DWORD           KeyExchangeAlg;
    DWORD           ProtocolVer;    
    BOOL            fAuthenticateServer;
    WTS_CERT_TYPE   CertType;
    DWORD           cbClientName;
    BYTE            rgbClientName[WTS_LICENSE_CLIENT_NAME_BYTE_LENGTH];

} _WTS_LICENSE_CAPABILITIES, WTS_LICENSE_CAPABILITIES, *PWTS_LICENSE_CAPABILITIES;

//
//  Defining all above WTSxxx as WRDSxxx for use in IWRdsXXX
//
#define WRDS_DOMAIN_LENGTH            WTS_DOMAIN_LENGTH
#define WRDS_USERNAME_LENGTH          WTS_USERNAME_LENGTH
#define WRDS_PASSWORD_LENGTH          WTS_PASSWORD_LENGTH
#define WRDS_DIRECTORY_LENGTH         WTS_DIRECTORY_LENGTH
#define WRDS_INITIALPROGRAM_LENGTH    WTS_INITIALPROGRAM_LENGTH
#define WRDS_PROTOCOL_NAME_LENGTH     WTS_PROTOCOL_NAME_LENGTH
#define WRDS_DRIVER_NAME_LENGTH       WTS_DRIVER_NAME_LENGTH
#define WRDS_DEVICE_NAME_LENGTH       WTS_DEVICE_NAME_LENGTH
#define WRDS_IMEFILENAME_LENGTH       WTS_IMEFILENAME_LENGTH
#define WRDS_CLIENTNAME_LENGTH        WTS_CLIENTNAME_LENGTH
#define WRDS_CLIENTADDRESS_LENGTH     WTS_CLIENTADDRESS_LENGTH
#define WRDS_CLIENT_PRODUCT_ID_LENGTH WTS_CLIENT_PRODUCT_ID_LENGTH
#define WRDS_MAX_PROTOCOL_CACHE       WTS_MAX_PROTOCOL_CACHE
#define WRDS_MAX_CACHE_RESERVED       WTS_MAX_CACHE_RESERVED
#define WRDS_MAX_RESERVED             WTS_MAX_RESERVED
#define WRDS_MAX_COUNTERS             WTS_MAX_COUNTERS
#define WRDS_MAX_DISPLAY_IOCTL_DATA   WTS_MAX_DISPLAY_IOCTL_DATA

#define WRDS_PERF_DISABLE_NOTHING                              WTS_PERF_DISABLE_NOTHING
#define WRDS_PERF_DISABLE_WALLPAPER                            WTS_PERF_DISABLE_WALLPAPER
#define WRDS_PERF_DISABLE_FULLWINDOWDRAG                       WTS_PERF_DISABLE_FULLWINDOWDRAG
#define WRDS_PERF_DISABLE_MENUANIMATIONS                       WTS_PERF_DISABLE_MENUANIMATIONS
#define WRDS_PERF_DISABLE_THEMING                              WTS_PERF_DISABLE_THEMING
#define WRDS_PERF_ENABLE_ENHANCED_GRAPHICS                     WTS_PERF_ENABLE_ENHANCED_GRAPHICS
#define WRDS_PERF_DISABLE_CURSOR_SHADOW                        WTS_PERF_DISABLE_CURSOR_SHADOW
#define WRDS_PERF_DISABLE_CURSORSETTINGS                       WTS_PERF_DISABLE_CURSORSETTINGS
#define WRDS_PERF_ENABLE_FONT_SMOOTHING                        WTS_PERF_ENABLE_FONT_SMOOTHING
#define WRDS_PERF_ENABLE_DESKTOP_COMPOSITION                   WTS_PERF_ENABLE_DESKTOP_COMPOSITION

typedef struct _WTS_SMALL_RECT _WRDS_SMALL_RECT;
typedef WTS_SMALL_RECT WRDS_SMALL_RECT;
typedef PWTS_SMALL_RECT PWRDS_SMALL_RECT;

#define WRDS_SERVICE_NONE WTS_SERVICE_NONE
#define WRDS_SERVICE_START WTS_SERVICE_START
#define WRDS_SERVICE_STOP WTS_SERVICE_STOP                   
typedef WTS_RCM_SERVICE_STATE WRDS_RCM_SERVICE_STATE;

#define WRDS_DRAIN_STATE_NONE WTS_DRAIN_STATE_NONE
#define WRDS_DRAIN_IN_DRAIN WTS_DRAIN_IN_DRAIN
#define WRDS_DRAIN_NOT_IN_DRAIN WTS_DRAIN_NOT_IN_DRAIN
typedef WTS_RCM_DRAIN_STATE WRDS_RCM_DRAIN_STATE;

typedef struct _WTS_SERVICE_STATE _WRDS_SERVICE_STATE;
typedef WTS_SERVICE_STATE WRDS_SERVICE_STATE;
typedef PWTS_SERVICE_STATE PWRDS_SERVICE_STATE;

typedef struct _WTS_SESSION_ID _WRDS_SESSION_ID;
typedef WTS_SESSION_ID WRDS_SESSION_ID;
typedef PWTS_SESSION_ID PWRDS_SESSION_ID;

typedef struct _WTS_USER_CREDENTIAL _WRDS_USER_CREDENTIAL;
typedef WTS_USER_CREDENTIAL WRDS_USER_CREDENTIAL;
typedef PWTS_USER_CREDENTIAL PWRDS_USER_CREDENTIAL;

typedef struct _WTS_SYSTEMTIME _WRDS_SYSTEMTIME;
typedef WTS_SYSTEMTIME WRDS_SYSTEMTIME;
typedef PWTS_SYSTEMTIME PWRDS_SYSTEMTIME;

typedef struct _WTS_TIME_ZONE_INFORMATION _WRDS_TIME_ZONE_INFORMATION;
typedef WTS_TIME_ZONE_INFORMATION WRDS_TIME_ZONE_INFORMATION;
typedef PWTS_TIME_ZONE_INFORMATION PWRDS_TIME_ZONE_INFORMATION;

typedef struct _WTS_SOCKADDR _WRDS_SOCKADDR;
typedef WTS_SOCKADDR WRDS_SOCKADDR;
typedef PWTS_SOCKADDR PWRDS_SOCKADDR;

typedef struct _WTS_CLIENT_DATA _WRDS_CLIENT_DATA;
typedef WTS_CLIENT_DATA WRDS_CLIENT_DATA;
typedef PWTS_CLIENT_DATA PWRDS_CLIENT_DATA;

typedef struct _WTS_PROTOCOL_CACHE _WRDS_PROTOCOL_CACHE;
typedef WTS_PROTOCOL_CACHE WRDS_PROTOCOL_CACHE;
typedef PWTS_PROTOCOL_CACHE PWRDS_PROTOCOL_CACHE;

typedef union _WTS_CACHE_STATS_UN _WRDS_CACHE_STATS_UN;
typedef WTS_CACHE_STATS_UN WRDS_CACHE_STATS_UN;
typedef PWTS_CACHE_STATS_UN PWRDS_CACHE_STATS_UN;

typedef struct _WTS_CACHE_STATS _WRDS_CACHE_STATS;
typedef WTS_CACHE_STATS WRDS_CACHE_STATS;
typedef PWTS_CACHE_STATS PWRDS_CACHE_STATS;

typedef struct _WTS_PROTOCOL_COUNTERS _WRDS_PROTOCOL_COUNTERS;
typedef WTS_PROTOCOL_COUNTERS WRDS_PROTOCOL_COUNTERS;
typedef PWTS_PROTOCOL_COUNTERS PWRDS_PROTOCOL_COUNTERS;

typedef struct _WTS_PROTOCOL_STATUS _WRDS_PROTOCOL_STATUS;
typedef WTS_PROTOCOL_STATUS WRDS_PROTOCOL_STATUS;
typedef PWTS_PROTOCOL_STATUS PWRDS_PROTOCOL_STATUS;

typedef struct _WTS_DISPLAY_IOCTL _WRDS_DISPLAY_IOCTL;
typedef WTS_DISPLAY_IOCTL WRDS_DISPLAY_IOCTL;
typedef PWTS_DISPLAY_IOCTL PWRDS_DISPLAY_IOCTL;

#define WRDS_LOGON_ERR_INVALID WTS_LOGON_ERR_INVALID
#define WRDS_LOGON_ERR_NOT_HANDLED WTS_LOGON_ERR_NOT_HANDLED
#define WRDS_LOGON_ERR_HANDLED_SHOW WTS_LOGON_ERR_HANDLED_SHOW
#define WRDS_LOGON_ERR_HANDLED_DONT_SHOW WTS_LOGON_ERR_HANDLED_DONT_SHOW
#define WRDS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER

typedef WTS_LOGON_ERROR_REDIRECTOR_RESPONSE WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE;

#define WRDS_QUERY_ALLOWED_INITIAL_APP WTS_QUERY_ALLOWED_INITIAL_APP
#define WRDS_QUERY_LOGON_SCREEN_SIZE WTS_QUERY_LOGON_SCREEN_SIZE
#define WRDS_QUERY_AUDIOENUM_DLL WTS_QUERY_AUDIOENUM_DLL
#define WRDS_QUERY_MF_FORMAT_SUPPORT WTS_QUERY_MF_FORMAT_SUPPORT

#define WRDS_VALUE_TYPE_ULONG       WTS_VALUE_TYPE_ULONG
#define WRDS_VALUE_TYPE_STRING      WTS_VALUE_TYPE_STRING
#define WRDS_VALUE_TYPE_BINARY      WTS_VALUE_TYPE_BINARY
#define WRDS_VALUE_TYPE_GUID        WTS_VALUE_TYPE_GUID

typedef struct __WTS_PROPERTY_VALUE __WRDS_PROPERTY_VALUE;
typedef WTS_PROPERTY_VALUE WRDS_PROPERTY_VALUE;
typedef PWTS_PROPERTY_VALUE PWRDS_PROPERTY_VALUE;


#define WRDS_KEY_EXCHANGE_ALG_RSA    WTS_KEY_EXCHANGE_ALG_RSA
#define WRDS_KEY_EXCHANGE_ALG_DH     WTS_KEY_EXCHANGE_ALG_DH

#define WRDS_LICENSE_PROTOCOL_VERSION            WTS_LICENSE_PROTOCOL_VERSION
#define WRDS_LICENSE_PREAMBLE_VERSION            WTS_LICENSE_PREAMBLE_VERSION
#define WRDS_LICENSE_CURRENT_PROTOCOL_VERSION    WTS_LICENSE_CURRENT_PROTOCOL_VERSION

#define WRDS_CERT_TYPE_INVALID WTS_CERT_TYPE_INVALID
#define WRDS_CERT_TYPE_PROPRIETORY WTS_CERT_TYPE_PROPRIETORY
#define WRDS_CERT_TYPE_X509 WTS_CERT_TYPE_X509

typedef WTS_CERT_TYPE WRDS_CERT_TYPE;

#define WRDS_LICENSE_CLIENT_NAME_BYTE_LENGTH WTS_LICENSE_CLIENT_NAME_BYTE_LENGTH

typedef struct _WTS_LICENSE_CAPABILITIES _WRDS_LICENSE_CAPABILITIES;
typedef WTS_LICENSE_CAPABILITIES WRDS_LICENSE_CAPABILITIES;
typedef PWTS_LICENSE_CAPABILITIES PWRDS_LICENSE_CAPABILITIES;

typedef enum _WRDS_CONNECTION_SETTING_LEVEL {
    WRDS_CONNECTION_SETTING_LEVEL_INVALID,
    WRDS_CONNECTION_SETTING_LEVEL_1,
} WRDS_CONNECTION_SETTING_LEVEL, *PWRDS_CONNECTION_SETTING_LEVEL;

#define WRDS_CONNECTION_SETTING_CURRENT_LEVEL WRDS_CONNECTION_SETTING_LEVEL_1

typedef enum _WRDS_LISTENER_SETTING_LEVEL {
    WRDS_LISTENER_SETTING_LEVEL_INVALID,
    WRDS_LISTENER_SETTING_LEVEL_1,
} WRDS_LISTENER_SETTING_LEVEL, *PWRDS_LISTENER_SETTING_LEVEL;

#define WRDS_LISTENER_SETTING_CURRENT_LEVEL WRDS_LISTENER_SETTING_LEVEL_1

typedef enum _WRDS_SETTING_TYPE {
    WRDS_SETTING_TYPE_INVALID,
    WRDS_SETTING_TYPE_MACHINE,                  // Machine GP
    WRDS_SETTING_TYPE_USER,                     // User GP
    WRDS_SETTING_TYPE_SAM,                      // User SAM
} WRDS_SETTING_TYPE, *PWRDS_SETTING_TYPE;

typedef enum _WRDS_SETTING_STATUS {
    WRDS_SETTING_STATUS_NOTAPPLICABLE = -1,
    WRDS_SETTING_STATUS_DISABLED,
    WRDS_SETTING_STATUS_ENABLED,
    WRDS_SETTING_STATUS_NOTCONFIGURED,          // not configured for policy
} WRDS_SETTING_STATUS, *PWRDS_SETTING_STATUS;

typedef enum _WRDS_SETTING_LEVEL {
    WRDS_SETTING_LEVEL_INVALID,
    WRDS_SETTING_LEVEL_1,
} WRDS_SETTING_LEVEL, *PWRDS_SETTING_LEVEL;

#define WRDS_SETTING_CURRENT_LEVEL WRDS_SETTING_LEVEL_1

typedef struct _WRDS_LISTENER_SETTINGS_1 {

    ULONG                       MaxProtocolListenerConnectionCount; // ULONG_MAX means maximum allowed

#ifdef __midl
    [range(0, 16384)] 
    ULONG                       SecurityDescriptorSize;
    [size_is(SecurityDescriptorSize)] 
    PBYTE                       pSecurityDescriptor;
#else
    ULONG                       SecurityDescriptorSize;
    PBYTE                       pSecurityDescriptor;
#endif

} WRDS_LISTENER_SETTINGS_1, *PWRDS_LISTENER_SETTINGS_1;

#ifdef __midl

typedef [switch_type(WRDS_LISTENER_SETTING_LEVEL)] union _WRDS_LISTENER_SETTING {
    [case(WRDS_LISTENER_SETTING_LEVEL_1)]
    WRDS_LISTENER_SETTINGS_1                    WRdsListenerSettings1;
} WRDS_LISTENER_SETTING, *PWRDS_LISTENERN_SETTING;

typedef struct _WRDS_LISTENER_SETTINGS {
    WRDS_LISTENER_SETTING_LEVEL                 WRdsListenerSettingLevel;
    [switch_is(WRdsListenerSettingLevel)] 
    WRDS_LISTENER_SETTING                       WRdsListenerSetting;
} WRDS_LISTENER_SETTINGS, *PWRDS_LISTENER_SETTINGS;

#else

typedef union _WRDS_LISTENER_SETTING {
    WRDS_LISTENER_SETTINGS_1                    WRdsListenerSettings1;
} WRDS_LISTENER_SETTING, *PWRDS_LISTENERN_SETTING;

typedef struct _WRDS_LISTENER_SETTINGS {
    WRDS_LISTENER_SETTING_LEVEL                 WRdsListenerSettingLevel;
    WRDS_LISTENER_SETTING                       WRdsListenerSetting;
} WRDS_LISTENER_SETTINGS, *PWRDS_LISTENER_SETTINGS;

#endif

typedef struct _WRDS_CONNECTION_SETTINGS_1 {

    BOOLEAN                     fInheritInitialProgram;
    BOOLEAN                     fInheritColorDepth;
    BOOLEAN                     fHideTitleBar;
    BOOLEAN                     fInheritAutoLogon;
    BOOLEAN                     fMaximizeShell;
    BOOLEAN                     fDisablePNP;
    BOOLEAN                     fPasswordIsScPin;
    BOOLEAN                     fPromptForPassword;
    BOOLEAN                     fDisableCpm;
    BOOLEAN                     fDisableCdm;
    BOOLEAN                     fDisableCcm;
    BOOLEAN                     fDisableLPT;
    BOOLEAN                     fDisableClip;
    BOOLEAN                     fResetBroken;
    BOOLEAN                     fDisableEncryption;
    BOOLEAN                     fDisableAutoReconnect;
    BOOLEAN                     fDisableCtrlAltDel;
    BOOLEAN                     fDoubleClickDetect;
    BOOLEAN                     fEnableWindowsKey;
    BOOLEAN                     fUsingSavedCreds;
    BOOLEAN                     fMouse;
    BOOLEAN                     fNoAudioPlayback;
    BOOLEAN                     fRemoteConsoleAudio;

    BYTE                        EncryptionLevel;
    USHORT                      ColorDepth;
    USHORT                      ProtocolType;
    USHORT                      HRes;
    USHORT                      VRes;
    USHORT                      ClientProductId;
    USHORT                      OutBufCountHost;
    USHORT                      OutBufCountClient;
    USHORT                      OutBufLength;
    
    ULONG                       KeyboardLayout;
    ULONG                       MaxConnectionTime;
    ULONG                       MaxDisconnectionTime;
    ULONG                       MaxIdleTime;
    ULONG                       PerformanceFlags;
    ULONG                       KeyboardType;
    ULONG                       KeyboardSubType;
    ULONG                       KeyboardFunctionKey;
    ULONG                       ActiveInputLocale;
    ULONG                       SerialNumber; 
    ULONG                       ClientAddressFamily;
    ULONG                       ClientBuildNumber;
    ULONG                       ClientSessionId;

    WCHAR                       WorkDirectory[ WTS_DIRECTORY_LENGTH + 1 ];
    WCHAR                       InitialProgram[ WTS_INITIALPROGRAM_LENGTH + 1 ];
    WCHAR                       UserName[ WTS_USERNAME_LENGTH + 1 ];
    WCHAR                       Domain[WTS_DOMAIN_LENGTH + 1];
    WCHAR                       Password[ WTS_PASSWORD_LENGTH + 1 ];
    WCHAR                       ProtocolName[ WTS_PROTOCOL_NAME_LENGTH + 1 ];
    WCHAR                       DisplayDriverName[ WTS_DRIVER_NAME_LENGTH + 1 ];
    WCHAR                       DisplayDeviceName[ WTS_DEVICE_NAME_LENGTH + 1 ];
    WCHAR                       imeFileName[ WTS_IMEFILENAME_LENGTH + 1 ];
    WCHAR                       AudioDriverName[ WTS_DRIVER_NAME_LENGTH + 1 ];
    WCHAR                       ClientName[ WTS_CLIENTNAME_LENGTH + 1 ];
    WCHAR                       ClientAddress[ WTS_CLIENTADDRESS_LENGTH + 1 ];
    WCHAR                       ClientDirectory[ WTS_DIRECTORY_LENGTH + 1 ];
    WCHAR                       ClientDigProductId[ WTS_CLIENT_PRODUCT_ID_LENGTH + 1 ];

    WRDS_SOCKADDR               ClientSockAddress;
    WRDS_TIME_ZONE_INFORMATION  ClientTimeZone;

    WRDS_LISTENER_SETTINGS      WRdsListenerSettings;

    GUID                        EventLogActivityId;

#ifdef __midl
    [range(0, 16384)] 
    ULONG                       ContextSize;
    [size_is(ContextSize)] 
    PBYTE                       ContextData;
#else
    ULONG                       ContextSize;
    PBYTE                       ContextData;
#endif

} WRDS_CONNECTION_SETTINGS_1, *PWRDS_CONNECTION_SETTINGS_1;

//
// This is a subset of WRDS_CONNECTION_SETTING_1.
// It will only contain the fields which have a policy. 
//
typedef struct _WRDS_SETTINGS_1 {

    WRDS_SETTING_STATUS     WRdsDisableClipStatus;
    ULONG                   WRdsDisableClipValue;

    WRDS_SETTING_STATUS     WRdsDisableLPTStatus;
    ULONG                   WRdsDisableLPTValue;

    WRDS_SETTING_STATUS     WRdsDisableCcmStatus;
    ULONG                   WRdsDisableCcmValue;

    WRDS_SETTING_STATUS     WRdsDisableCdmStatus;
    ULONG                   WRdsDisableCdmValue;

    WRDS_SETTING_STATUS     WRdsDisableCpmStatus;
    ULONG                   WRdsDisableCpmValue;

    WRDS_SETTING_STATUS     WRdsDisablePnpStatus;
    ULONG                   WRdsDisablePnpValue;

    WRDS_SETTING_STATUS     WRdsEncryptionLevelStatus;
    ULONG                   WRdsEncryptionValue;

    WRDS_SETTING_STATUS     WRdsColorDepthStatus;
    ULONG                   WRdsColorDepthValue;

    WRDS_SETTING_STATUS     WRdsDisableAutoReconnecetStatus;
    ULONG                   WRdsDisableAutoReconnecetValue;

    WRDS_SETTING_STATUS     WRdsDisableEncryptionStatus;
    ULONG                   WRdsDisableEncryptionValue;

    WRDS_SETTING_STATUS     WRdsResetBrokenStatus;
    ULONG                   WRdsResetBrokenValue;

    WRDS_SETTING_STATUS     WRdsMaxIdleTimeStatus;
    ULONG                   WRdsMaxIdleTimeValue;

    WRDS_SETTING_STATUS     WRdsMaxDisconnectTimeStatus;
    ULONG                   WRdsMaxDisconnectTimeValue;

    WRDS_SETTING_STATUS     WRdsMaxConnectTimeStatus;
    ULONG                   WRdsMaxConnectTimeValue;

    WRDS_SETTING_STATUS     WRdsKeepAliveStatus;
    BOOLEAN                 WRdsKeepAliveStartValue;
    ULONG                   WRdsKeepAliveIntervalValue;

} WRDS_SETTINGS_1, *PWRDS_SETTINGS_1;

#ifdef __midl

typedef [switch_type(WRDS_CONNECTION_SETTING_LEVEL)] union _WRDS_CONNECTION_SETTING {
    [case(WRDS_CONNECTION_SETTING_LEVEL_1)]
    WRDS_CONNECTION_SETTINGS_1                  WRdsConnectionSettings1;
} WRDS_CONNECTION_SETTING, *PWRDS_CONNECTION_SETTING;

typedef struct _WRDS_CONNECTION_SETTINGS {
    WRDS_CONNECTION_SETTING_LEVEL               WRdsConnectionSettingLevel;
    [switch_is(WRdsConnectionSettingLevel)] 
    WRDS_CONNECTION_SETTING                     WRdsConnectionSetting;
} WRDS_CONNECTION_SETTINGS, *PWRDS_CONNECTION_SETTINGS;

typedef [switch_type(WRDS_SETTING_LEVEL)] union _WRDS_SETTING {
    [case (WRDS_SETTING_LEVEL_1)]
    WRDS_SETTINGS_1                             WRdsSettings1;
} WRDS_SETTING, *PWRDS_SETTING;

typedef struct _WRDS_SETTINGS {
    WRDS_SETTING_TYPE                           WRdsSettingType;
    WRDS_SETTING_LEVEL                          WRdsSettingLevel;
    [switch_is(WRdsSettingLevel)] 
    WRDS_SETTING                                WRdsSetting;
} WRDS_SETTINGS, *PWRDS_SETTINGS;

#else

typedef union _WRDS_CONNECTION_SETTING {
    WRDS_CONNECTION_SETTINGS_1                  WRdsConnectionSettings1;
} WRDS_CONNECTION_SETTING, *PWRDS_CONNECTION_SETTING;

typedef struct _WRDS_CONNECTION_SETTINGS {
    WRDS_CONNECTION_SETTING_LEVEL               WRdsConnectionSettingLevel;
    WRDS_CONNECTION_SETTING                     WRdsConnectionSetting;
} WRDS_CONNECTION_SETTINGS, *PWRDS_CONNECTION_SETTINGS;

typedef union _WRDS_SETTING {
    WRDS_SETTINGS_1                             WRdsSettings1;
} WRDS_SETTING, *PWRDS_SETTING;

typedef struct _WRDS_SETTINGS {
    WRDS_SETTING_TYPE                           WRdsSettingType;
    WRDS_SETTING_LEVEL                          WRdsSettingLevel;
    WRDS_SETTING                                WRdsSetting;
} WRDS_SETTINGS, *PWRDS_SETTINGS;

#endif

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#ifdef __midl
typedef struct _WTS_SERIALIZED_USER_CREDENTIAL {
    ULONG SerializationLength;
    [size_is(SerializationLength)] BYTE* Serialization;
} WTS_SERIALIZED_USER_CREDENTIAL, * PWTS_SERIALIZED_USER_CREDENTIAL, WRDS_SERIALIZED_USER_CREDENTIAL, * PWRDS_SERIALIZED_USER_CREDENTIAL;
#else
typedef struct _WTS_SERIALIZED_USER_CREDENTIAL WRDS_SERIALIZED_USER_CREDENTIAL;
#endif
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* !_INC_WTSDEFS */
