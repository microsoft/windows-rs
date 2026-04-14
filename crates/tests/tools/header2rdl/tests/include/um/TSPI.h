/*++ BUILD Version: 0000    // Increment this if a change has global effects

The  Telephony  API  is jointly copyrighted by Intel and Microsoft.  You are
granted  a royalty free worldwide, unlimited license to make copies, and use
the   API/SPI  for  making  applications/drivers  that  interface  with  the
specification provided that this paragraph and the Intel/Microsoft copyright
statement is maintained as is in the text and source code files.

Copyright (c) Microsoft Corporation. All rights reserved.
Portions copyright 1992, 1993 Intel/Microsoft, all rights reserved.

Module Name:

    tspi.h

Notes:

    Additions to the Telephony Service Provider Interface (TSPI) since
    version 1.0 are noted by version number (i.e. "TSPI v1.4").

--*/

#ifndef TSPI_H
#define TSPI_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <windows.h>

#include "tapi.h"


#if WIN32
#if TAPI_CURRENT_VERSION < 0x00020000
#error Building a 32bit 1.3 or 1.4 service provider is not supported.
#endif
#endif


// tspi.h  is  only  of  use  in  conjunction  with tapi.h.  Very few types are
// defined  in  tspi.h.   Most  types of procedure formal parameters are simply
// passed through from corresponding procedures in tapi.h.  A working knowledge
// of the TAPI interface is required for an understanding of this interface.

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */

#ifndef DECLARE_OPAQUE
#define DECLARE_OPAQUE(name)  struct name##__ { int unused; }; \
                typedef const struct name##__ FAR* name
#endif  // DECLARE_OPAQUE

#ifndef TSPIAPI

#define TSPIAPI PASCAL

#endif

DECLARE_OPAQUE(HDRVCALL);
DECLARE_OPAQUE(HDRVLINE);
DECLARE_OPAQUE(HDRVPHONE);
DECLARE_OPAQUE(HDRVMSPLINE);
DECLARE_OPAQUE(HDRVDIALOGINSTANCE);

typedef HDRVCALL FAR * LPHDRVCALL;
typedef HDRVLINE FAR * LPHDRVLINE;
typedef HDRVPHONE FAR * LPHDRVPHONE;
typedef HDRVDIALOGINSTANCE FAR * LPHDRVDIALOGINSTANCE;
typedef HDRVMSPLINE FAR * LPHDRVMSPLINE;

DECLARE_OPAQUE(HTAPICALL);
DECLARE_OPAQUE(HTAPILINE);
DECLARE_OPAQUE(HTAPIPHONE);

DECLARE_OPAQUE32(HTAPIDIALOGINSTANCE);
DECLARE_OPAQUE32(HTAPIMSPLINE);

typedef HTAPICALL FAR * LPHTAPICALL;
typedef HTAPILINE FAR * LPHTAPILINE;
typedef HTAPIPHONE FAR * LPHTAPIPHONE;
typedef HTAPIDIALOGINSTANCE FAR * LPHTAPIDIALOGINSTANCE;
typedef HTAPIMSPLINE FAR * LPHTAPIMSPLINE;


DECLARE_OPAQUE(HPROVIDER);
typedef HPROVIDER FAR * LPHPROVIDER;

typedef DWORD DRV_REQUESTID;

typedef void (CALLBACK * ASYNC_COMPLETION)(
    DRV_REQUESTID       dwRequestID,
    LONG                lResult
    );

typedef void (CALLBACK * LINEEVENT)(
    HTAPILINE           htLine,
    HTAPICALL           htCall,
    DWORD               dwMsg,
    DWORD_PTR           dwParam1,
    DWORD_PTR           dwParam2,
    DWORD_PTR           dwParam3
    );

typedef void (CALLBACK * PHONEEVENT)(
    HTAPIPHONE          htPhone,
    DWORD               dwMsg,
    DWORD_PTR           dwParam1,
    DWORD_PTR           dwParam2,
    DWORD_PTR           dwParam3
    );

typedef LONG (CALLBACK * TUISPIDLLCALLBACK)(
    DWORD_PTR           dwObjectID,
    DWORD               dwObjectType,
    LPVOID              lpParams,
    DWORD               dwSize
    );

#if TAPI_CURRENT_VERSION >= 0x00020000
typedef struct tuispicreatedialoginstanceparams_tag
{
    DRV_REQUESTID           dwRequestID;

    HDRVDIALOGINSTANCE      hdDlgInst;

    HTAPIDIALOGINSTANCE     htDlgInst;

    LPCWSTR                 lpszUIDLLName;

    LPVOID                  lpParams;

    DWORD                   dwSize;

} TUISPICREATEDIALOGINSTANCEPARAMS, *LPTUISPICREATEDIALOGINSTANCEPARAMS;
#endif

#if (TAPI_CURRENT_VERSION >= 0x00030000)

#define LINEQOSSTRUCT_KEY       ((DWORD)'LQSK')

typedef struct LINEQOSSERVICELEVEL_tag
{
    DWORD           dwMediaMode;

    DWORD           dwQOSServiceLevel;

} LINEQOSSERVICELEVEL, * LPLINEQOSSERVICELEVEL;


typedef struct LINECALLQOSINFO_tag
{
    DWORD           dwKey;

    DWORD           dwTotalSize;

    DWORD           dwQOSRequestType;

    union
    {
        struct
        {
            DWORD               dwNumServiceLevelEntries;

            LINEQOSSERVICELEVEL LineQOSServiceLevel[1];

        } SetQOSServiceLevel;

    };

} LINECALLQOSINFO, * LPLINECALLQOSINFO;



// {831CE2D6-83B5-11d1-BB5C-00C04FB6809F}
EXTERN_C const CLSID TAPIPROTOCOL_PSTN;

// {831CE2D7-83B5-11d1-BB5C-00C04FB6809F}
EXTERN_C const CLSID TAPIPROTOCOL_H323;

// {831CE2D8-83B5-11d1-BB5C-00C04FB6809F}
EXTERN_C const CLSID TAPIPROTOCOL_Multicast;

#endif

#define TSPI_MESSAGE_BASE 500
    // The lowest-numbered TSPI-specific message ID number

#define LINE_NEWCALL                            ((long) TSPI_MESSAGE_BASE + 0)
#define LINE_CALLDEVSPECIFIC                    ((long) TSPI_MESSAGE_BASE + 1)
#define LINE_CALLDEVSPECIFICFEATURE             ((long) TSPI_MESSAGE_BASE + 2)
#if (TAPI_CURRENT_VERSION >= 0x00020000)
#define LINE_CREATEDIALOGINSTANCE               ((long) TSPI_MESSAGE_BASE + 3)
                                                                // TSPI v2.0
#define LINE_SENDDIALOGINSTANCEDATA             ((long) TSPI_MESSAGE_BASE + 4)
                                                                // TSPI v2.0
#endif
#if (TAPI_CURRENT_VERSION >= 0x00030000)
#define LINE_SENDMSPDATA                        ((long) TSPI_MESSAGE_BASE + 5)
                                                                // TSPI v3.0
#define LINE_QOSINFO                            ((long) TSPI_MESSAGE_BASE + 6)
                                                                // TSPI v3.0
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
#define LINETSPIOPTION_NONREENTRANT             0x00000001      // TSPI v2.0
#endif


#if (TAPI_CURRENT_VERSION >= 0x00020000)
#define TUISPIDLL_OBJECT_LINEID                 1L              // TSPI v2.0
#define TUISPIDLL_OBJECT_PHONEID                2L              // TSPI v2.0
#define TUISPIDLL_OBJECT_PROVIDERID             3L              // TSPI v2.0
#define TUISPIDLL_OBJECT_DIALOGINSTANCE         4L              // TSPI v2.0
#endif

#if (TAPI_CURRENT_VERSION >= 0x00030000)
#define PRIVATEOBJECT_NONE                      0x00000001      // TSPI v3.0
#define PRIVATEOBJECT_CALLID                    0x00000002      // TSPI v3.0
#define PRIVATEOBJECT_LINE                      0x00000003      // TSPI v3.0
#define PRIVATEOBJECT_CALL                      0x00000004      // TSPI v3.0
#define PRIVATEOBJECT_PHONE                     0x00000005      // TSPI v3.0
#define PRIVATEOBJECT_ADDRESS                   0x00000006      // TSPI v3.0

#define LINEQOSREQUESTTYPE_SERVICELEVEL         0x00000001      // TSPI v3.0

#define LINEQOSSERVICELEVEL_NEEDED              0x00000001      // TSPI v3.0
#define LINEQOSSERVICELEVEL_IFAVAILABLE         0x00000002      // TSPI v3.0
#define LINEQOSSERVICELEVEL_BESTEFFORT          0x00000003      // TSPI v3.0

#define LINEEQOSINFO_NOQOS                      0x00000001      // TSPI v3.0
#define LINEEQOSINFO_ADMISSIONFAILURE           0x00000002      // TSPI v3.0
#define LINEEQOSINFO_POLICYFAILURE              0x00000003      // TSPI v3.0
#define LINEEQOSINFO_GENERICERROR               0x00000004      // TSPI v3.0
#endif


//
// The following function prototypes pertain
// to a service provider's core module
//



LONG
TSPIAPI
TSPI_lineAccept(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpsUserUserInfo,
    DWORD               dwSize
    );

LONG
TSPIAPI
TSPI_lineAddToConference(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdConfCall,
    HDRVCALL            hdConsultCall
    );

LONG
TSPIAPI
TSPI_lineAnswer(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpsUserUserInfo,
    DWORD               dwSize
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineBlindTransfer(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCWSTR             lpszDestAddress,
    DWORD               dwCountryCode);
#else
LONG
TSPIAPI
TSPI_lineBlindTransfer(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpszDestAddress,
    DWORD               dwCountryCode);
#endif

LONG
TSPIAPI
TSPI_lineClose(
    HDRVLINE            hdLine
    );

LONG
TSPIAPI
TSPI_lineCloseCall(
    HDRVCALL            hdCall
    );

LONG
TSPIAPI
TSPI_lineCompleteCall(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPDWORD             lpdwCompletionID,
    DWORD               dwCompletionMode,
    DWORD               dwMessageID
    );

LONG
TSPIAPI
TSPI_lineCompleteTransfer(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    HDRVCALL            hdConsultCall,
    HTAPICALL           htConfCall,
    LPHDRVCALL          lphdConfCall,
    DWORD               dwTransferMode
    );

LONG
TSPIAPI
TSPI_lineConditionalMediaDetection(
    HDRVLINE            hdLine,
    DWORD               dwMediaModes,
    LPLINECALLPARAMS    const lpCallParams
    );

LONG
TSPIAPI
TSPI_lineDevSpecific(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HDRVCALL            hdCall,
    LPVOID              lpParams,
    DWORD               dwSize
    );

LONG
TSPIAPI
TSPI_lineDevSpecificFeature(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwFeature,
    LPVOID              lpParams,
    DWORD               dwSize
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineDial(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCWSTR             lpszDestAddress,
    DWORD               dwCountryCode
    );
#else
LONG
TSPIAPI
TSPI_lineDial(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpszDestAddress,
    DWORD               dwCountryCode
    );
#endif

LONG
TSPIAPI
TSPI_lineDrop(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpsUserUserInfo,
    DWORD               dwSize
    );

LONG
TSPIAPI
TSPI_lineDropOnClose(                                           // TSPI v1.4
    HDRVCALL            hdCall
    );

LONG
TSPIAPI
TSPI_lineDropNoOwner(                                           // TSPI v1.4
    HDRVCALL            hdCall
    );

LONG
TSPIAPI
TSPI_lineForward(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               bAllAddresses,
    DWORD               dwAddressID,
    LPLINEFORWARDLIST   const lpForwardList,
    DWORD               dwNumRingsNoAnswer,
    HTAPICALL           htConsultCall,
    LPHDRVCALL          lphdConsultCall,
    LPLINECALLPARAMS    const lpCallParams
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineGatherDigits(
    HDRVCALL            hdCall,
    DWORD               dwEndToEndID,
    DWORD               dwDigitModes,
    _Out_writes_opt_(dwNumDigits) LPWSTR lpsDigits,
    DWORD               dwNumDigits,
    LPCWSTR             lpszTerminationDigits,
    DWORD               dwFirstDigitTimeout,
    DWORD               dwInterDigitTimeout
    );
#else
LONG
TSPIAPI
TSPI_lineGatherDigits(
    HDRVCALL            hdCall,
    DWORD               dwEndToEndID,
    DWORD               dwDigitModes,
    _Out_writes_opt_(dwNumDigits) LPSTR lpsDigits,
    DWORD               dwNumDigits,
    LPCSTR              lpszTerminationDigits,
    DWORD               dwFirstDigitTimeout,
    DWORD               dwInterDigitTimeout
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineGenerateDigits(
    HDRVCALL            hdCall,
    DWORD               dwEndToEndID,
    DWORD               dwDigitMode,
    LPCWSTR             lpszDigits,
    DWORD               dwDuration
    );
#else
LONG
TSPIAPI
TSPI_lineGenerateDigits(
    HDRVCALL            hdCall,
    DWORD               dwEndToEndID,
    DWORD               dwDigitMode,
    LPCSTR              lpszDigits,
    DWORD               dwDuration
    );
#endif

LONG
TSPIAPI
TSPI_lineGenerateTone(
    HDRVCALL            hdCall,
    DWORD               dwEndToEndID,
    DWORD               dwToneMode,
    DWORD               dwDuration,
    DWORD               dwNumTones,
    LPLINEGENERATETONE  const lpTones
    );

LONG
TSPIAPI
TSPI_lineGetAddressCaps(
    DWORD               dwDeviceID,
    DWORD               dwAddressID,
    DWORD               dwTSPIVersion,
    DWORD               dwExtVersion,
    LPLINEADDRESSCAPS   lpAddressCaps
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineGetAddressID(
    HDRVLINE            hdLine,
    LPDWORD             lpdwAddressID,
    DWORD               dwAddressMode,
    LPCWSTR             lpsAddress,
    DWORD               dwSize
    );
#else
LONG
TSPIAPI
TSPI_lineGetAddressID(
    HDRVLINE            hdLine,
    LPDWORD             lpdwAddressID,
    DWORD               dwAddressMode,
    LPCSTR              lpsAddress,
    DWORD               dwSize
    );
#endif

LONG
TSPIAPI
TSPI_lineGetAddressStatus(
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    LPLINEADDRESSSTATUS lpAddressStatus
    );

LONG
TSPIAPI
TSPI_lineGetCallAddressID(
    HDRVCALL            hdCall,
    LPDWORD             lpdwAddressID
    );

#if (TAPI_CURRENT_VERSION >= 0x00030000)
LONG
TSPIAPI
TSPI_lineGetCallHubTracking(                                    // TSPI v3.0
    HDRVLINE                    hdLine,
    LPLINECALLHUBTRACKINGINFO   lpTrackingInfo
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00030000)
LONG
TSPIAPI
TSPI_lineGetCallIDs(                                            // TSPI v3.0
    HDRVCALL            hdCall,
    LPDWORD             lpdwAddressID,
    LPDWORD             lpdwCallID,
    LPDWORD             lpdwRelatedCallID
    );
#endif

LONG
TSPIAPI
TSPI_lineGetCallInfo(
    HDRVCALL            hdCall,
    LPLINECALLINFO      lpCallInfo
    );

LONG
TSPIAPI
TSPI_lineGetCallStatus(
    HDRVCALL            hdCall,
    LPLINECALLSTATUS    lpCallStatus
    );

LONG
TSPIAPI
TSPI_lineGetDevCaps(
    DWORD               dwDeviceID,
    DWORD               dwTSPIVersion,
    DWORD               dwExtVersion,
    LPLINEDEVCAPS       lpLineDevCaps
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineGetDevConfig(
    DWORD               dwDeviceID,
    LPVARSTRING         lpDeviceConfig,
    LPCWSTR             lpszDeviceClass
    );
#else
LONG
TSPIAPI
TSPI_lineGetDevConfig(
    DWORD               dwDeviceID,
    LPVARSTRING         lpDeviceConfig,
    LPCSTR              lpszDeviceClass
    );
#endif

LONG
TSPIAPI
TSPI_lineGetExtensionID(
    DWORD               dwDeviceID,
    DWORD               dwTSPIVersion,
    LPLINEEXTENSIONID   lpExtensionID
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineGetIcon(
    DWORD               dwDeviceID,
    LPCWSTR             lpszDeviceClass,
    LPHICON             lphIcon
    );
#else
LONG
TSPIAPI
TSPI_lineGetIcon(
    DWORD               dwDeviceID,
    LPCSTR              lpszDeviceClass,
    LPHICON             lphIcon
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineGetID(
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HDRVCALL            hdCall,
    DWORD               dwSelect,
    LPVARSTRING         lpDeviceID,
    LPCWSTR             lpszDeviceClass,
    HANDLE              hTargetProcess                          // TSPI v2.0
    );
#else
LONG
TSPIAPI
TSPI_lineGetID(
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HDRVCALL            hdCall,
    DWORD               dwSelect,
    LPVARSTRING         lpDeviceID,
    LPCSTR              lpszDeviceClass
    );
#endif


LONG
TSPIAPI
TSPI_lineGetLineDevStatus(
    HDRVLINE            hdLine,
    LPLINEDEVSTATUS     lpLineDevStatus
    );

LONG
TSPIAPI
TSPI_lineGetNumAddressIDs(
    HDRVLINE            hdLine,
    LPDWORD             lpdwNumAddressIDs
    );

LONG
TSPIAPI
TSPI_lineHold(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineMakeCall(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    HTAPICALL           htCall,
    LPHDRVCALL          lphdCall,
    LPCWSTR             lpszDestAddress,
    DWORD               dwCountryCode,
    LPLINECALLPARAMS    const lpCallParams
    );
#else
LONG
TSPIAPI
TSPI_lineMakeCall(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    HTAPICALL           htCall,
    LPHDRVCALL          lphdCall,
    LPCSTR              lpszDestAddress,
    DWORD               dwCountryCode,
    LPLINECALLPARAMS    const lpCallParams
    );
#endif

LONG
TSPIAPI
TSPI_lineMonitorDigits(
    HDRVCALL            hdCall,
    DWORD               dwDigitModes
    );

LONG
TSPIAPI
TSPI_lineMonitorMedia(
    HDRVCALL            hdCall,
    DWORD               dwMediaModes
    );

LONG
TSPIAPI
TSPI_lineMonitorTones(
    HDRVCALL            hdCall,
    DWORD               dwToneListID,
    LPLINEMONITORTONE   const lpToneList,
    DWORD               dwNumEntries
    );

LONG
TSPIAPI
TSPI_lineNegotiateExtVersion(
    DWORD               dwDeviceID,
    DWORD               dwTSPIVersion,
    DWORD               dwLowVersion,
    DWORD               dwHighVersion,
    LPDWORD             lpdwExtVersion
    );

LONG
TSPIAPI
TSPI_lineNegotiateTSPIVersion(
    DWORD               dwDeviceID,
    DWORD               dwLowVersion,
    DWORD               dwHighVersion,
    LPDWORD             lpdwTSPIVersion
    );

LONG
TSPIAPI
TSPI_lineOpen(
    DWORD               dwDeviceID,
    HTAPILINE           htLine,
    LPHDRVLINE          lphdLine,
    DWORD               dwTSPIVersion,
    LINEEVENT           lpfnEventProc
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_linePark(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    DWORD               dwParkMode,
    LPCWSTR             lpszDirAddress,
    LPVARSTRING         lpNonDirAddress
    );
#else
LONG
TSPIAPI
TSPI_linePark(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    DWORD               dwParkMode,
    LPCSTR              lpszDirAddress,
    LPVARSTRING         lpNonDirAddress
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_linePickup(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HTAPICALL           htCall,
    LPHDRVCALL          lphdCall,
    LPCWSTR             lpszDestAddress,
    LPCWSTR             lpszGroupID
    );
#else
LONG
TSPIAPI
TSPI_linePickup(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HTAPICALL           htCall,
    LPHDRVCALL          lphdCall,
    LPCSTR              lpszDestAddress,
    LPCSTR              lpszGroupID
    );
#endif

LONG
TSPIAPI
TSPI_linePrepareAddToConference(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdConfCall,
    HTAPICALL           htConsultCall,
    LPHDRVCALL          lphdConsultCall,
    LPLINECALLPARAMS    const lpCallParams
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineRedirect(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCWSTR             lpszDestAddress,
    DWORD               dwCountryCode
    );
#else
LONG
TSPIAPI
TSPI_lineRedirect(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpszDestAddress,
    DWORD               dwCountryCode
    );
#endif

LONG
TSPIAPI
TSPI_lineReleaseUserUserInfo(                                   // TSPI v1.4
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall
    );

LONG
TSPIAPI
TSPI_lineRemoveFromConference(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall
    );

LONG
TSPIAPI
TSPI_lineSecureCall(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall
    );

LONG
TSPIAPI
TSPI_lineSelectExtVersion(
    HDRVLINE            hdLine,
    DWORD               dwExtVersion
    );

LONG
TSPIAPI
TSPI_lineSendUserUserInfo(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPCSTR              lpsUserUserInfo,
    DWORD               dwSize
    );

LONG
TSPIAPI
TSPI_lineSetAppSpecific(
    HDRVCALL            hdCall,
    DWORD               dwAppSpecific
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineSetCallData(                                           // TSPI v2.0
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPVOID              lpCallData,
    DWORD               dwSize
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00030000)
LONG
TSPIAPI
TSPI_lineSetCallHubTracking(                                    // TSPI v3.0
    HDRVLINE                    hdLine,
    LPLINECALLHUBTRACKINGINFO   lpTrackingInfo
    );
#endif

LONG
TSPIAPI
TSPI_lineSetCallParams(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    DWORD               dwBearerMode,
    DWORD               dwMinRate,
    DWORD               dwMaxRate,
    LPLINEDIALPARAMS    const lpDialParams
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineSetCallQualityOfService(                               // TSPI v2.0
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    LPVOID              lpSendingFlowspec,
    DWORD               dwSendingFlowspecSize,
    LPVOID              lpReceivingFlowspec,
    DWORD               dwReceivingFlowspecSize
    );

LONG
TSPIAPI
TSPI_lineSetCallTreatment(                                      // TSPI v2.0
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    DWORD               dwTreatment
    );
#endif

LONG
TSPIAPI
TSPI_lineSetCurrentLocation(                                    // TSPI v1.4
    DWORD               dwLocation
    );

LONG
TSPIAPI
TSPI_lineSetDefaultMediaDetection(
    HDRVLINE            hdLine,
    DWORD               dwMediaModes
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineSetDevConfig(
    DWORD               dwDeviceID,
    LPVOID              const lpDeviceConfig,
    DWORD               dwSize,
    LPCWSTR             lpszDeviceClass
    );
#else
LONG
TSPIAPI
TSPI_lineSetDevConfig(
    DWORD               dwDeviceID,
    LPVOID              const lpDeviceConfig,
    DWORD               dwSize,
    LPCSTR              lpszDeviceClass
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineSetLineDevStatus(                                      // TSPI v2.0
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwStatusToChange,
    DWORD               fStatus
    );
#endif

LONG
TSPIAPI
TSPI_lineSetMediaControl(
    HDRVLINE                    hdLine,
    DWORD                       dwAddressID,
    HDRVCALL                    hdCall,
    DWORD                       dwSelect,
    LPLINEMEDIACONTROLDIGIT     const lpDigitList,
    DWORD                       dwDigitNumEntries,
    LPLINEMEDIACONTROLMEDIA     const lpMediaList,
    DWORD                       dwMediaNumEntries,
    LPLINEMEDIACONTROLTONE      const lpToneList,
    DWORD                       dwToneNumEntries,
    LPLINEMEDIACONTROLCALLSTATE const lpCallStateList,
    DWORD                       dwCallStateNumEntries
    );

LONG
TSPIAPI
TSPI_lineSetMediaMode(
    HDRVCALL            hdCall,
    DWORD               dwMediaMode
    );

LONG
TSPIAPI
TSPI_lineSetStatusMessages(
    HDRVLINE            hdLine,
    DWORD               dwLineStates,
    DWORD               dwAddressStates
    );

LONG
TSPIAPI
TSPI_lineSetTerminal(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HDRVCALL            hdCall,
    DWORD               dwSelect,
    DWORD               dwTerminalModes,
    DWORD               dwTerminalID,
    DWORD               bEnable
    );

LONG
TSPIAPI
TSPI_lineSetupConference(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    HDRVLINE            hdLine,
    HTAPICALL           htConfCall,
    LPHDRVCALL          lphdConfCall,
    HTAPICALL           htConsultCall,
    LPHDRVCALL          lphdConsultCall,
    DWORD               dwNumParties,
    LPLINECALLPARAMS    const lpCallParams
    );

LONG
TSPIAPI
TSPI_lineSetupTransfer(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall,
    HTAPICALL           htConsultCall,
    LPHDRVCALL          lphdConsultCall,
    LPLINECALLPARAMS    const lpCallParams
    );

LONG
TSPIAPI
TSPI_lineSwapHold(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdActiveCall,
    HDRVCALL            hdHeldCall
    );

LONG
TSPIAPI
TSPI_lineUncompleteCall(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwCompletionID
    );

LONG
TSPIAPI
TSPI_lineUnhold(
    DRV_REQUESTID       dwRequestID,
    HDRVCALL            hdCall
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_lineUnpark(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HTAPICALL           htCall,
    LPHDRVCALL          lphdCall,
    LPCWSTR             lpszDestAddress
    );
#else
LONG
TSPIAPI
TSPI_lineUnpark(
    DRV_REQUESTID       dwRequestID,
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HTAPICALL           htCall,
    LPHDRVCALL          lphdCall,
    LPCSTR              lpszDestAddress
    );
#endif



LONG
TSPIAPI
TSPI_phoneClose(
    HDRVPHONE           hdPhone
    );

LONG
TSPIAPI
TSPI_phoneDevSpecific(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    LPVOID              lpParams,
    DWORD               dwSize
    );

LONG
TSPIAPI
TSPI_phoneGetButtonInfo(
    HDRVPHONE           hdPhone,
    DWORD               dwButtonLampID,
    LPPHONEBUTTONINFO   lpButtonInfo
    );

LONG
TSPIAPI
TSPI_phoneGetData(
    HDRVPHONE           hdPhone,
    DWORD               dwDataID,
    LPVOID              lpData,
    DWORD               dwSize
    );

LONG
TSPIAPI
TSPI_phoneGetDevCaps(
    DWORD               dwDeviceID,
    DWORD               dwTSPIVersion,
    DWORD               dwExtVersion,
    LPPHONECAPS         lpPhoneCaps
    );

LONG
TSPIAPI
TSPI_phoneGetDisplay(
    HDRVPHONE           hdPhone,
    LPVARSTRING         lpDisplay
    );

LONG
TSPIAPI
TSPI_phoneGetExtensionID(
    DWORD               dwDeviceID,
    DWORD               dwTSPIVersion,
    LPPHONEEXTENSIONID  lpExtensionID
    );

LONG
TSPIAPI
TSPI_phoneGetGain(
    HDRVPHONE           hdPhone,
    DWORD               dwHookSwitchDev,
    LPDWORD             lpdwGain
    );

LONG
TSPIAPI
TSPI_phoneGetHookSwitch(
    HDRVPHONE           hdPhone,
    LPDWORD             lpdwHookSwitchDevs
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_phoneGetIcon(
    DWORD               dwDeviceID,
    LPCWSTR             lpszDeviceClass,
    LPHICON             lphIcon
    );
#else
LONG
TSPIAPI
TSPI_phoneGetIcon(
    DWORD               dwDeviceID,
    LPCSTR              lpszDeviceClass,
    LPHICON             lphIcon
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_phoneGetID(
    HDRVPHONE           hdPhone,
    LPVARSTRING         lpDeviceID,
    LPCWSTR             lpszDeviceClass,
    HANDLE              hTargetProcess                          // TSPI v2.0
    );
#else
LONG
TSPIAPI
TSPI_phoneGetID(
    HDRVPHONE           hdPhone,
    LPVARSTRING         lpDeviceID,
    LPCSTR              lpszDeviceClass
    );
#endif

LONG
TSPIAPI
TSPI_phoneGetLamp(
    HDRVPHONE           hdPhone,
    DWORD               dwButtonLampID,
    LPDWORD             lpdwLampMode
    );

LONG
TSPIAPI
TSPI_phoneGetRing(
    HDRVPHONE           hdPhone,
    LPDWORD             lpdwRingMode,
    LPDWORD             lpdwVolume
    );

LONG
TSPIAPI
TSPI_phoneGetStatus(
    HDRVPHONE           hdPhone,
    LPPHONESTATUS       lpPhoneStatus
    );

LONG
TSPIAPI
TSPI_phoneGetVolume(
    HDRVPHONE           hdPhone,
    DWORD               dwHookSwitchDev,
    LPDWORD             lpdwVolume
    );

LONG
TSPIAPI
TSPI_phoneNegotiateExtVersion(
    DWORD               dwDeviceID,
    DWORD               dwTSPIVersion,
    DWORD               dwLowVersion,
    DWORD               dwHighVersion,
    LPDWORD             lpdwExtVersion
    );

LONG
TSPIAPI
TSPI_phoneNegotiateTSPIVersion(
    DWORD               dwDeviceID,
    DWORD               dwLowVersion,
    DWORD               dwHighVersion,
    LPDWORD             lpdwTSPIVersion
    );

LONG
TSPIAPI
TSPI_phoneOpen(
    DWORD               dwDeviceID,
    HTAPIPHONE          htPhone,
    LPHDRVPHONE         lphdPhone,
    DWORD               dwTSPIVersion,
    PHONEEVENT          lpfnEventProc
    );

LONG
TSPIAPI
TSPI_phoneSelectExtVersion(
    HDRVPHONE           hdPhone,
    DWORD               dwExtVersion
    );

LONG
TSPIAPI
TSPI_phoneSetButtonInfo(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwButtonLampID,
    LPPHONEBUTTONINFO   const lpButtonInfo
    );

LONG
TSPIAPI
TSPI_phoneSetData(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwDataID,
    LPVOID              const lpData,
    DWORD               dwSize
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_phoneSetDisplay(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwRow,
    DWORD               dwColumn,
    LPCWSTR             lpsDisplay,
    DWORD               dwSize
    );
#else
LONG
TSPIAPI
TSPI_phoneSetDisplay(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwRow,
    DWORD               dwColumn,
    LPCSTR              lpsDisplay,
    DWORD               dwSize
    );
#endif

LONG
TSPIAPI
TSPI_phoneSetGain(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwHookSwitchDev,
    DWORD               dwGain
    );

LONG
TSPIAPI
TSPI_phoneSetHookSwitch(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwHookSwitchDevs,
    DWORD               dwHookSwitchMode
    );

LONG
TSPIAPI
TSPI_phoneSetLamp(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwButtonLampID,
    DWORD               dwLampMode
    );

LONG
TSPIAPI
TSPI_phoneSetRing(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwRingMode,
    DWORD               dwVolume
    );

LONG
TSPIAPI
TSPI_phoneSetStatusMessages(
    HDRVPHONE           hdPhone,
    DWORD               dwPhoneStates,
    DWORD               dwButtonModes,
    DWORD               dwButtonStates
    );

LONG
TSPIAPI
TSPI_phoneSetVolume(
    DRV_REQUESTID       dwRequestID,
    HDRVPHONE           hdPhone,
    DWORD               dwHookSwitchDev,
    DWORD               dwVolume
    );



LONG
TSPIAPI
TSPI_providerConfig(
    HWND                hwndOwner,
    DWORD               dwPermanentProviderID
    );

LONG
TSPIAPI
TSPI_providerCreateLineDevice(                                  // TSPI v1.4
    DWORD_PTR           dwTempID,
    DWORD               dwDeviceID
    );

LONG
TSPIAPI
TSPI_providerCreatePhoneDevice(                                 // TSPI v1.4
    DWORD_PTR           dwTempID,
    DWORD               dwDeviceID
    );

LONG
TSPIAPI
TSPI_providerEnumDevices(                                       // TSPI v1.4
    DWORD               dwPermanentProviderID,
    LPDWORD             lpdwNumLines,
    LPDWORD             lpdwNumPhones,
    HPROVIDER           hProvider,
    LINEEVENT           lpfnLineCreateProc,
    PHONEEVENT          lpfnPhoneCreateProc
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_providerFreeDialogInstance(                                // TSPI v2.0
    HDRVDIALOGINSTANCE  hdDlgInst
    );

LONG
TSPIAPI
TSPI_providerGenericDialogData(                                 // TSPI v2.0
    DWORD_PTR           dwObjectID,
    DWORD               dwObjectType,
    LPVOID              lpParams,
    DWORD               dwSize
    );
#endif

LONG
TSPIAPI
TSPI_providerInit(
    DWORD               dwTSPIVersion,
    DWORD               dwPermanentProviderID,
    DWORD               dwLineDeviceIDBase,
    DWORD               dwPhoneDeviceIDBase,
    DWORD_PTR           dwNumLines,
    DWORD_PTR           dwNumPhones,
    ASYNC_COMPLETION    lpfnCompletionProc
#if (TAPI_CURRENT_VERSION >= 0x00020000)
    ,
    LPDWORD             lpdwTSPIOptions                         // TSPI v2.0
#endif
    );

LONG
TSPIAPI
TSPI_providerInstall(
    HWND                hwndOwner,
    DWORD               dwPermanentProviderID
    );

LONG
TSPIAPI
TSPI_providerRemove(
    HWND                hwndOwner,
    DWORD               dwPermanentProviderID
    );

LONG
TSPIAPI
TSPI_providerShutdown(
    DWORD               dwTSPIVersion
#if (TAPI_CURRENT_VERSION >= 0x00020000)
    ,
    DWORD               dwPermanentProviderID                   // TSPI v2.0
#endif
    );

#if (TAPI_CURRENT_VERSION >= 0x00020000)
LONG
TSPIAPI
TSPI_providerUIIdentify(                                        // TSPI v2.0
    _Inout_ LPWSTR  lpszUIDLLName
    );
#endif


#if (TAPI_CURRENT_VERSION >= 0x00030000)
LONG
TSPIAPI
TSPI_lineMSPIdentify(
    DWORD               dwDeviceID,
    GUID *              pCLSID
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00030000)
LONG
TSPIAPI
TSPI_lineCreateMSPInstance(
    HDRVLINE            hdLine,
    DWORD               dwAddressID,
    HTAPIMSPLINE        htMSPLine,
    LPHDRVMSPLINE       lphdMSPLine
    );

LONG
TSPIAPI
TSPI_lineCloseMSPInstance(
    HDRVMSPLINE         hdMSPLine
    );

LONG
TSPIAPI
TSPI_lineReceiveMSPData(
    HDRVLINE            hdLine,
    HDRVCALL            hdCall,
    HDRVMSPLINE         hdMSPLine,
    LPVOID              pBuffer,
    DWORD               dwSize
    );
#endif

#if (TAPI_CURRENT_VERSION >= 0x00020000)
//
// The following function prototypes pertain
// to a service provider's UI module
//

LONG
TSPIAPI
TUISPI_lineConfigDialog(                                        // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    DWORD               dwDeviceID,
    HWND                hwndOwner,
    LPCWSTR             lpszDeviceClass
    );

LONG
TSPIAPI
TUISPI_lineConfigDialogEdit(                                    // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    DWORD               dwDeviceID,
    HWND                hwndOwner,
    LPCWSTR             lpszDeviceClass,
    LPVOID              const lpDeviceConfigIn,
    DWORD               dwSize,
    LPVARSTRING         lpDeviceConfigOut
    );

LONG
TSPIAPI
TUISPI_phoneConfigDialog(                                       // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    DWORD               dwDeviceID,
    HWND                hwndOwner,
    LPCWSTR             lpszDeviceClass
    );

LONG
TSPIAPI
TUISPI_providerConfig(                                          // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    HWND                hwndOwner,
    DWORD               dwPermanentProviderID
    );

LONG
TSPIAPI
TUISPI_providerGenericDialog(                                   // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    HTAPIDIALOGINSTANCE htDlgInst,
    LPVOID              lpParams,
    DWORD               dwSize,
    HANDLE              hEvent
    );

LONG
TSPIAPI
TUISPI_providerGenericDialogData(                               // TSPI v2.0
    HTAPIDIALOGINSTANCE htDlgInst,
    LPVOID              lpParams,
    DWORD               dwSize
    );

LONG
TSPIAPI
TUISPI_providerInstall(                                         // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    HWND                hwndOwner,
    DWORD               dwPermanentProviderID
    );

LONG
TSPIAPI
TUISPI_providerRemove(                                          // TSPI v2.0
    TUISPIDLLCALLBACK   lpfnUIDLLCallback,
    HWND                hwndOwner,
    DWORD               dwPermanentProviderID
    );

#else

//
// The following were obsoleted by the above (but are needed to build 1.4 SPs)
//

LONG
TSPIAPI
TSPI_lineConfigDialog(
    DWORD               dwDeviceID,
    HWND                hwndOwner,
    LPCSTR              lpszDeviceClass
    );

LONG
TSPIAPI
TSPI_lineConfigDialogEdit(
    DWORD               dwDeviceID,
    HWND                hwndOwner,
    LPCSTR              lpszDeviceClass,
    LPVOID              const lpDeviceConfigIn,
    DWORD               dwSize,
    LPVARSTRING         lpDeviceConfigOut
    );

LONG
TSPIAPI
TSPI_phoneConfigDialog(
    DWORD               dwDeviceID,
    HWND                hwndOwner,
    LPCSTR              lpszDeviceClass
    );

#endif



#if (TAPI_CURRENT_VERSION < 0x00020000)

//
// The following macros are the ordinal numbers of the exported tspi functions
//

#define TSPI_PROC_BASE                      500

#define TSPI_LINEACCEPT                     (TSPI_PROC_BASE + 0)
#define TSPI_LINEADDTOCONFERENCE            (TSPI_PROC_BASE + 1)
#define TSPI_LINEANSWER                     (TSPI_PROC_BASE + 2)
#define TSPI_LINEBLINDTRANSFER              (TSPI_PROC_BASE + 3)
#define TSPI_LINECLOSE                      (TSPI_PROC_BASE + 4)
#define TSPI_LINECLOSECALL                  (TSPI_PROC_BASE + 5)
#define TSPI_LINECOMPLETECALL               (TSPI_PROC_BASE + 6)
#define TSPI_LINECOMPLETETRANSFER           (TSPI_PROC_BASE + 7)
#define TSPI_LINECONDITIONALMEDIADETECTION  (TSPI_PROC_BASE + 8)
#define TSPI_LINECONFIGDIALOG               (TSPI_PROC_BASE + 9)
#define TSPI_LINEDEVSPECIFIC                (TSPI_PROC_BASE + 10)
#define TSPI_LINEDEVSPECIFICFEATURE         (TSPI_PROC_BASE + 11)
#define TSPI_LINEDIAL                       (TSPI_PROC_BASE + 12)
#define TSPI_LINEDROP                       (TSPI_PROC_BASE + 13)
#define TSPI_LINEFORWARD                    (TSPI_PROC_BASE + 14)
#define TSPI_LINEGATHERDIGITS               (TSPI_PROC_BASE + 15)
#define TSPI_LINEGENERATEDIGITS             (TSPI_PROC_BASE + 16)
#define TSPI_LINEGENERATETONE               (TSPI_PROC_BASE + 17)
#define TSPI_LINEGETADDRESSCAPS             (TSPI_PROC_BASE + 18)
#define TSPI_LINEGETADDRESSID               (TSPI_PROC_BASE + 19)
#define TSPI_LINEGETADDRESSSTATUS           (TSPI_PROC_BASE + 20)
#define TSPI_LINEGETCALLADDRESSID           (TSPI_PROC_BASE + 21)
#define TSPI_LINEGETCALLINFO                (TSPI_PROC_BASE + 22)
#define TSPI_LINEGETCALLSTATUS              (TSPI_PROC_BASE + 23)
#define TSPI_LINEGETDEVCAPS                 (TSPI_PROC_BASE + 24)
#define TSPI_LINEGETDEVCONFIG               (TSPI_PROC_BASE + 25)
#define TSPI_LINEGETEXTENSIONID             (TSPI_PROC_BASE + 26)
#define TSPI_LINEGETICON                    (TSPI_PROC_BASE + 27)
#define TSPI_LINEGETID                      (TSPI_PROC_BASE + 28)
#define TSPI_LINEGETLINEDEVSTATUS           (TSPI_PROC_BASE + 29)
#define TSPI_LINEGETNUMADDRESSIDS           (TSPI_PROC_BASE + 30)
#define TSPI_LINEHOLD                       (TSPI_PROC_BASE + 31)
#define TSPI_LINEMAKECALL                   (TSPI_PROC_BASE + 32)
#define TSPI_LINEMONITORDIGITS              (TSPI_PROC_BASE + 33)
#define TSPI_LINEMONITORMEDIA               (TSPI_PROC_BASE + 34)
#define TSPI_LINEMONITORTONES               (TSPI_PROC_BASE + 35)
#define TSPI_LINENEGOTIATEEXTVERSION        (TSPI_PROC_BASE + 36)
#define TSPI_LINENEGOTIATETSPIVERSION       (TSPI_PROC_BASE + 37)
#define TSPI_LINEOPEN                       (TSPI_PROC_BASE + 38)
#define TSPI_LINEPARK                       (TSPI_PROC_BASE + 39)
#define TSPI_LINEPICKUP                     (TSPI_PROC_BASE + 40)
#define TSPI_LINEPREPAREADDTOCONFERENCE     (TSPI_PROC_BASE + 41)
#define TSPI_LINEREDIRECT                   (TSPI_PROC_BASE + 42)
#define TSPI_LINEREMOVEFROMCONFERENCE       (TSPI_PROC_BASE + 43)
#define TSPI_LINESECURECALL                 (TSPI_PROC_BASE + 44)
#define TSPI_LINESELECTEXTVERSION           (TSPI_PROC_BASE + 45)
#define TSPI_LINESENDUSERUSERINFO           (TSPI_PROC_BASE + 46)
#define TSPI_LINESETAPPSPECIFIC             (TSPI_PROC_BASE + 47)
#define TSPI_LINESETCALLPARAMS              (TSPI_PROC_BASE + 48)
#define TSPI_LINESETDEFAULTMEDIADETECTION   (TSPI_PROC_BASE + 49)
#define TSPI_LINESETDEVCONFIG               (TSPI_PROC_BASE + 50)
#define TSPI_LINESETMEDIACONTROL            (TSPI_PROC_BASE + 51)
#define TSPI_LINESETMEDIAMODE               (TSPI_PROC_BASE + 52)
#define TSPI_LINESETSTATUSMESSAGES          (TSPI_PROC_BASE + 53)
#define TSPI_LINESETTERMINAL                (TSPI_PROC_BASE + 54)
#define TSPI_LINESETUPCONFERENCE            (TSPI_PROC_BASE + 55)
#define TSPI_LINESETUPTRANSFER              (TSPI_PROC_BASE + 56)
#define TSPI_LINESWAPHOLD                   (TSPI_PROC_BASE + 57)
#define TSPI_LINEUNCOMPLETECALL             (TSPI_PROC_BASE + 58)
#define TSPI_LINEUNHOLD                     (TSPI_PROC_BASE + 59)
#define TSPI_LINEUNPARK                     (TSPI_PROC_BASE + 60)
#define TSPI_PHONECLOSE                     (TSPI_PROC_BASE + 61)
#define TSPI_PHONECONFIGDIALOG              (TSPI_PROC_BASE + 62)
#define TSPI_PHONEDEVSPECIFIC               (TSPI_PROC_BASE + 63)
#define TSPI_PHONEGETBUTTONINFO             (TSPI_PROC_BASE + 64)
#define TSPI_PHONEGETDATA                   (TSPI_PROC_BASE + 65)
#define TSPI_PHONEGETDEVCAPS                (TSPI_PROC_BASE + 66)
#define TSPI_PHONEGETDISPLAY                (TSPI_PROC_BASE + 67)
#define TSPI_PHONEGETEXTENSIONID            (TSPI_PROC_BASE + 68)
#define TSPI_PHONEGETGAIN                   (TSPI_PROC_BASE + 69)
#define TSPI_PHONEGETHOOKSWITCH             (TSPI_PROC_BASE + 70)
#define TSPI_PHONEGETICON                   (TSPI_PROC_BASE + 71)
#define TSPI_PHONEGETID                     (TSPI_PROC_BASE + 72)
#define TSPI_PHONEGETLAMP                   (TSPI_PROC_BASE + 73)
#define TSPI_PHONEGETRING                   (TSPI_PROC_BASE + 74)
#define TSPI_PHONEGETSTATUS                 (TSPI_PROC_BASE + 75)
#define TSPI_PHONEGETVOLUME                 (TSPI_PROC_BASE + 76)
#define TSPI_PHONENEGOTIATEEXTVERSION       (TSPI_PROC_BASE + 77)
#define TSPI_PHONENEGOTIATETSPIVERSION      (TSPI_PROC_BASE + 78)
#define TSPI_PHONEOPEN                      (TSPI_PROC_BASE + 79)
#define TSPI_PHONESELECTEXTVERSION          (TSPI_PROC_BASE + 80)
#define TSPI_PHONESETBUTTONINFO             (TSPI_PROC_BASE + 81)
#define TSPI_PHONESETDATA                   (TSPI_PROC_BASE + 82)
#define TSPI_PHONESETDISPLAY                (TSPI_PROC_BASE + 83)
#define TSPI_PHONESETGAIN                   (TSPI_PROC_BASE + 84)
#define TSPI_PHONESETHOOKSWITCH             (TSPI_PROC_BASE + 85)
#define TSPI_PHONESETLAMP                   (TSPI_PROC_BASE + 86)
#define TSPI_PHONESETRING                   (TSPI_PROC_BASE + 87)
#define TSPI_PHONESETSTATUSMESSAGES         (TSPI_PROC_BASE + 88)
#define TSPI_PHONESETVOLUME                 (TSPI_PROC_BASE + 89)
#define TSPI_PROVIDERCONFIG                 (TSPI_PROC_BASE + 90)
#define TSPI_PROVIDERINIT                   (TSPI_PROC_BASE + 91)
#define TSPI_PROVIDERINSTALL                (TSPI_PROC_BASE + 92)
#define TSPI_PROVIDERREMOVE                 (TSPI_PROC_BASE + 93)
#define TSPI_PROVIDERSHUTDOWN               (TSPI_PROC_BASE + 94)

#define TSPI_PROVIDERENUMDEVICES            (TSPI_PROC_BASE + 95)  // TSPI v1.4
#define TSPI_LINEDROPONCLOSE                (TSPI_PROC_BASE + 96)  // TSPI v1.4
#define TSPI_LINEDROPNOOWNER                (TSPI_PROC_BASE + 97)  // TSPI v1.4
#define TSPI_PROVIDERCREATELINEDEVICE       (TSPI_PROC_BASE + 98)  // TSPI v1.4
#define TSPI_PROVIDERCREATEPHONEDEVICE      (TSPI_PROC_BASE + 99)  // TSPI v1.4
#define TSPI_LINESETCURRENTLOCATION         (TSPI_PROC_BASE + 100) // TSPI v1.4
#define TSPI_LINECONFIGDIALOGEDIT           (TSPI_PROC_BASE + 101) // TSPI v1.4
#define TSPI_LINERELEASEUSERUSERINFO        (TSPI_PROC_BASE + 102) // TSPI v1.4

#define TSPI_LINEGETCALLID                  (TSPI_PROC_BASE + 103) // TSPI v3.0
#define TSPI_LINEGETCALLHUBTRACKING         (TSPI_PROC_BASE + 104) // TSPI v3.0
#define TSPI_LINESETCALLHUBTRACKING         (TSPI_PROC_BASE + 105) // TSPI v3.0
#define TSPI_LINERECEIVEMSPDATA             (TSPI_PROC_BASE + 106) // TSPI v3.0
#define TSPI_LINEMSPIDENTIFY                (TSPI_PROC_BASE + 107) // TSPI v3.0
#define TSPI_LINECREATEMSPINSTANCE          (TSPI_PROC_BASE + 108) // TSPI v3.0
#define TSPI_LINECLOSEMSPINSTANCE           (TSPI_PROC_BASE + 109) // TSPI v3.0


#endif


#ifdef __cplusplus
}                       /* End of extern "C" { */
#endif  /* __cplusplus */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // TSPI_H

