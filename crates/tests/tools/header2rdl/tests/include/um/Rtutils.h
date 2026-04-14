/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rtutils.h

Abstract:
     Public declarations for the Router process  utility functions.

--*/

#ifndef __ROUTING_RTUTILS_H__
#define __ROUTING_RTUTILS_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// TRACING FUNCTION PROTOTYPES                                              //
//                                                                          //
// See DOCUMENT for more information                                        //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Definitions for flags and constants                                      //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define TRACE_USE_FILE      0x00000001
#define TRACE_USE_CONSOLE   0x00000002
#define TRACE_NO_SYNCH      0x00000004 //this flag is deprecated

#define TRACE_NO_STDINFO    0x00000001
#define TRACE_USE_MASK      0x00000002
#define TRACE_USE_MSEC      0x00000004
#define TRACE_USE_DATE      0x00000008

#define INVALID_TRACEID     0xFFFFFFFF


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// ANSI entry-points                                                        //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

DWORD
APIENTRY
TraceRegisterExA(
    IN  LPCSTR      lpszCallerName,
    IN  DWORD       dwFlags
    );

DWORD
APIENTRY
TraceDeregisterA(
    IN  DWORD       dwTraceID
    );

DWORD
APIENTRY
TraceDeregisterExA(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags
    );

DWORD
APIENTRY
TraceGetConsoleA(
    IN  DWORD       dwTraceID,
    OUT LPHANDLE    lphConsole
    );

DWORD
__cdecl
TracePrintfA(
    IN  DWORD       dwTraceID,
    IN  LPCSTR      lpszFormat,
    IN  ...         OPTIONAL
    );

DWORD
__cdecl
TracePrintfExA(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPCSTR      lpszFormat,
    IN  ...         OPTIONAL
    );

DWORD
APIENTRY
TraceVprintfExA(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPCSTR      lpszFormat,
    IN  va_list     arglist
    );

DWORD
APIENTRY
TracePutsExA(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPCSTR      lpszString
    );

DWORD
APIENTRY
TraceDumpExA(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPBYTE      lpbBytes,
    IN  DWORD       dwByteCount,
    IN  DWORD       dwGroupSize,
    IN  BOOL        bAddressPrefix,
    IN  LPCSTR      lpszPrefix
    );


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// ANSI entry-points macros                                                 //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define TraceRegisterA(a)               TraceRegisterExA(a,0)
#define TraceVprintfA(a,b,c)            TraceVprintfExA(a,0,b,c)
#define TracePutsA(a,b)                 TracePutsExA(a,0,b)
#define TraceDumpA(a,b,c,d,e,f)         TraceDumpExA(a,0,b,c,d,e,f)



//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Unicode entry-points                                                     //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

DWORD
APIENTRY
TraceRegisterExW(
    IN  LPCWSTR     lpszCallerName,
    IN  DWORD       dwFlags
    );

DWORD
APIENTRY
TraceDeregisterW(
    IN  DWORD       dwTraceID
    );

DWORD
APIENTRY
TraceDeregisterExW(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags
    );

DWORD
APIENTRY
TraceGetConsoleW(
    IN  DWORD       dwTraceID,
    OUT LPHANDLE    lphConsole
    );

DWORD
__cdecl
TracePrintfW(
    IN  DWORD       dwTraceID,
    IN  LPCWSTR     lpszFormat,
    IN  ...         OPTIONAL
    );

DWORD
__cdecl
TracePrintfExW(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPCWSTR     lpszFormat,
    IN  ...         OPTIONAL
    );

DWORD
APIENTRY
TraceVprintfExW(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPCWSTR     lpszFormat,
    IN  va_list     arglist
    );

DWORD
APIENTRY
TracePutsExW(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPCWSTR     lpszString
    );

DWORD
APIENTRY
TraceDumpExW(
    IN  DWORD       dwTraceID,
    IN  DWORD       dwFlags,
    IN  LPBYTE      lpbBytes,
    IN  DWORD       dwByteCount,
    IN  DWORD       dwGroupSize,
    IN  BOOL        bAddressPrefix,
    IN  LPCWSTR     lpszPrefix
    );


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Unicode entry-points macros                                              //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define TraceRegisterW(a)               TraceRegisterExW(a,0)
#define TraceVprintfW(a,b,c)            TraceVprintfExW(a,0,b,c)
#define TracePutsW(a,b)                 TracePutsExW(a,0,b)
#define TraceDumpW(a,b,c,d,e,f)         TraceDumpExW(a,0,b,c,d,e,f)



//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Code-page dependent entry-point macros                                   //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#ifdef UNICODE
#define TraceRegister           TraceRegisterW
#define TraceDeregister         TraceDeregisterW
#define TraceDeregisterEx       TraceDeregisterExW
#define TraceGetConsole         TraceGetConsoleW
#define TracePrintf             TracePrintfW
#define TraceVprintf            TraceVprintfW
#define TracePuts               TracePutsW
#define TraceDump               TraceDumpW
#define TraceRegisterEx         TraceRegisterExW
#define TracePrintfEx           TracePrintfExW
#define TraceVprintfEx          TraceVprintfExW
#define TracePutsEx             TracePutsExW
#define TraceDumpEx             TraceDumpExW
#else
#define TraceRegister           TraceRegisterA
#define TraceDeregister         TraceDeregisterA
#define TraceDeregisterEx       TraceDeregisterExA
#define TraceGetConsole         TraceGetConsoleA
#define TracePrintf             TracePrintfA
#define TraceVprintf            TraceVprintfA
#define TracePuts               TracePutsA
#define TraceDump               TraceDumpA
#define TraceRegisterEx         TraceRegisterExA
#define TracePrintfEx           TracePrintfExA
#define TraceVprintfEx          TraceVprintfExA
#define TracePutsEx             TracePutsExA
#define TraceDumpEx             TraceDumpExA
#endif



//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// EVENT LOGGING FUNCTION PROTOTYPES                                        //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// ANSI prototypes                                                          //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


VOID
APIENTRY
LogErrorA(
    IN DWORD    dwMessageId,
    IN DWORD    cNumberOfSubStrings,
    _In_reads_(cNumberOfSubStrings) IN LPSTR   *plpwsSubStrings,
    IN DWORD    dwErrorCode
);

VOID
APIENTRY
LogEventA(
    IN DWORD   wEventType,
    IN DWORD   dwMessageId,
    IN DWORD   cNumberOfSubStrings,
    _In_reads_(cNumberOfSubStrings) IN LPSTR  *plpwsSubStrings
);


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Unicode prototypes                                                       //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

VOID
LogErrorW(
    IN DWORD    dwMessageId,
    IN DWORD    cNumberOfSubStrings,
    _In_reads_(cNumberOfSubStrings) IN LPWSTR  *plpwsSubStrings,
    IN DWORD    dwErrorCode
);

VOID
LogEventW(
    IN DWORD   wEventType,
    IN DWORD   dwMessageId,
    IN DWORD   cNumberOfSubStrings,
    _In_reads_(cNumberOfSubStrings) IN LPWSTR *plpwsSubStrings
);


#ifdef UNICODE
#define LogError                LogErrorW
#define LogEvent                LogEventW
#else
#define LogError                LogErrorA
#define LogEvent                LogEventA
#endif


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The following functions allow the caller to specify the event source.    //
//                                                                          //
// Call RouterLogRegister with the strings which would be passed to         //
// RegisterEventSource; this returns a handle which can be passed           //
// to the functions RouterLogEvent and RouterLogEventData.                  //
//                                                                          //
// Call RouterLogDeregister to close the handle.                            //
//                                                                          //
// Macros are provided for the different kinds of event log entrys:         //
//  RouterLogError          logs an error (EVENTLOG_ERROR_TYPE)             //
//  RouterLogWarning        logs a warning (EVENTLOG_WARNING_TYPE)          //
//  RouterLogInformation    logs information (EVENTLOG_INFORMATION_TYPE)    //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// ANSI prototypes                                                          //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

HANDLE
RouterLogRegisterA(
    LPCSTR lpszSource
    );

VOID
RouterLogDeregisterA(
    HANDLE hLogHandle
    );

VOID
RouterLogEventA(
    IN HANDLE hLogHandle,
    IN DWORD dwEventType,
    IN DWORD dwMessageId,
    IN DWORD dwSubStringCount,
    IN _In_reads_opt_(dwSubStringCount) LPSTR *plpszSubStringArray,
    IN DWORD dwErrorCode
    );

VOID
RouterLogEventDataA(
    IN HANDLE hLogHandle,
    IN DWORD dwEventType,
    IN DWORD dwMessageId,
    IN DWORD dwSubStringCount,
    _In_reads_opt_(dwSubStringCount) IN LPSTR *plpszSubStringArray,
    IN DWORD dwDataBytes,
    IN LPBYTE lpDataBytes
    );

VOID
RouterLogEventStringA(
    IN HANDLE hLogHandle,
    IN DWORD dwEventType,
    IN DWORD dwMessageId,
    IN DWORD dwSubStringCount,
    _In_reads_(dwSubStringCount) IN LPSTR *plpszSubStringArray,
    IN DWORD dwErrorCode,
    IN DWORD dwErrorIndex
    );

VOID
__cdecl
RouterLogEventExA(
    IN HANDLE   hLogHandle,
    IN DWORD    dwEventType,
    IN DWORD    dwErrorCode,
    IN DWORD    dwMessageId,
    IN LPCSTR   ptszFormat,
    ...
    );

VOID
RouterLogEventValistExA(
    IN HANDLE   hLogHandle,
    IN DWORD    dwEventType,
    IN DWORD    dwErrorCode,
    IN DWORD    dwMessageId,
    IN LPCSTR   ptszFormat,
    IN va_list  arglist
    );

DWORD
RouterGetErrorStringA(
    IN  DWORD   dwErrorCode,
    _Out_ OUT LPSTR * lplpszErrorString
    );

#define RouterLogErrorA(h,msg,count,array,err) \
        RouterLogEventA(h,EVENTLOG_ERROR_TYPE,msg,count,array,err)
#define RouterLogWarningA(h,msg,count,array,err) \
        RouterLogEventA(h,EVENTLOG_WARNING_TYPE,msg,count,array,err)
#define RouterLogInformationA(h,msg,count,array,err) \
        RouterLogEventA(h,EVENTLOG_INFORMATION_TYPE,msg,count,array,err)

#define RouterLogErrorDataA(h,msg,count,array,c,buf) \
        RouterLogEventDataA(h,EVENTLOG_ERROR_TYPE,msg,count,array,c,buf)
#define RouterLogWarningDataA(h,msg,count,array,c,buf) \
        RouterLogEventDataA(h,EVENTLOG_WARNING_TYPE,msg,count,array,c,buf)
#define RouterLogInformationDataA(h,msg,count,array,c,buf) \
        RouterLogEventDataA(h,EVENTLOG_INFORMATION_TYPE,msg,count,array,c,buf)

#define RouterLogErrorStringA(h,msg,count,array,err,index) \
        RouterLogEventStringA(h,EVENTLOG_ERROR_TYPE,msg,count,array, err,index)
#define RouterLogWarningStringA(h,msg,count,array,err,index) \
        RouterLogEventStringA(h,EVENTLOG_WARNING_TYPE,msg,count,array,err,index)
#define RouterLogInformationStringA(h,msg,count,array, err,index) \
        RouterLogEventStringA(h,EVENTLOG_INFORMATION_TYPE,msg,count,array,err,\
                              index)


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Unicode prototypes                                                       //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

HANDLE
RouterLogRegisterW(
    LPCWSTR lpszSource
    );

VOID
RouterLogDeregisterW(
    HANDLE hLogHandle
    );

VOID
RouterLogEventW(
    IN HANDLE hLogHandle,
    IN DWORD dwEventType,
    IN DWORD dwMessageId,
    IN DWORD dwSubStringCount,
    IN _In_reads_opt_(dwSubStringCount) LPWSTR *plpszSubStringArray,
    IN DWORD dwErrorCode
    );

VOID
RouterLogEventDataW(
    IN HANDLE hLogHandle,
    IN DWORD dwEventType,
    IN DWORD dwMessageId,
    IN DWORD dwSubStringCount,
    _In_reads_opt_(dwSubStringCount) IN LPWSTR *plpszSubStringArray,
    IN DWORD dwDataBytes,
    IN LPBYTE lpDataBytes
    );

VOID
RouterLogEventStringW(
    IN HANDLE hLogHandle,
    IN DWORD dwEventType,
    IN DWORD dwMessageId,
    IN DWORD dwSubStringCount,
    _In_reads_(dwSubStringCount) IN LPWSTR *plpszSubStringArray,
    IN DWORD dwErrorCode,
    IN DWORD dwErrorIndex
    );

VOID
__cdecl
RouterLogEventExW(
    IN HANDLE   hLogHandle,
    IN DWORD    dwEventType,
    IN DWORD    dwErrorCode,
    IN DWORD    dwMessageId,
    IN LPCWSTR  ptszFormat,
    ...
    );

VOID
RouterLogEventValistExW(
    IN HANDLE   hLogHandle,
    IN DWORD    dwEventType,
    IN DWORD    dwErrorCode,
    IN DWORD    dwMessageId,
    IN LPCWSTR  ptszFormat,
    IN va_list  arglist
    );

DWORD
RouterGetErrorStringW(
    IN  DWORD    dwErrorCode,
    _Out_ OUT LPWSTR * lplpwszErrorString
    );


#define RouterLogErrorW(h,msg,count,array,err) \
        RouterLogEventW(h,EVENTLOG_ERROR_TYPE,msg,count,array,err)
#define RouterLogWarningW(h,msg,count,array,err) \
        RouterLogEventW(h,EVENTLOG_WARNING_TYPE,msg,count,array,err)
#define RouterLogInformationW(h,msg,count,array,err) \
        RouterLogEventW(h,EVENTLOG_INFORMATION_TYPE,msg,count,array,err)

#define RouterLogErrorDataW(h,msg,count,array,c,buf) \
        RouterLogEventDataW(h,EVENTLOG_ERROR_TYPE,msg,count,array,c,buf)
#define RouterLogWarningDataW(h,msg,count,array,c,buf) \
        RouterLogEventDataW(h,EVENTLOG_WARNING_TYPE,msg,count,array,c,buf)
#define RouterLogInformationDataW(h,msg,count,array,c,buf) \
        RouterLogEventDataW(h,EVENTLOG_INFORMATION_TYPE,msg,count,array,c,buf)

#define RouterLogErrorStringW(h,msg,count,array,err,index) \
        RouterLogEventStringW(h,EVENTLOG_ERROR_TYPE,msg,count,array,err,index)
#define RouterLogWarningStringW(h,msg,count,array,err,index) \
        RouterLogEventStringW(h,EVENTLOG_WARNING_TYPE,msg,count,array,err,index)
#define RouterLogInformationStringW(h,msg,count,array,err,index) \
        RouterLogEventStringW(h,EVENTLOG_INFORMATION_TYPE,msg,count,array,err,\
                              index)


#ifdef UNICODE
#define RouterLogRegister           RouterLogRegisterW
#define RouterLogDeregister         RouterLogDeregisterW
#define RouterLogEvent              RouterLogEventW
#define RouterLogError              RouterLogErrorW
#define RouterLogWarning            RouterLogWarningW
#define RouterLogInformation        RouterLogInformationW
#define RouterLogEventData          RouterLogEventDataW
#define RouterLogErrorData          RouterLogErrorDataW
#define RouterLogWarningData        RouterLogWarningDataW
#define RouterLogInformationData    RouterLogInformationDataW
#define RouterLogEventString        RouterLogEventStringW
#define RouterLogEventEx            RouterLogEventExW
#define RouterLogEventValistEx      RouterLogEventValistExW
#define RouterLogErrorString        RouterLogErrorStringW
#define RouterLogWarningString      RouterLogWarningStringW
#define RouterLogInformationString  RouterLogInformationStringW
#define RouterGetErrorString        RouterGetErrorStringW
#
#else
#define RouterLogRegister           RouterLogRegisterA
#define RouterLogDeregister         RouterLogDeregisterA
#define RouterLogEvent              RouterLogEventA
#define RouterLogError              RouterLogErrorA
#define RouterLogWarning            RouterLogWarningA
#define RouterLogInformation        RouterLogInformationA
#define RouterLogEventData          RouterLogEventDataA
#define RouterLogErrorData          RouterLogErrorDataA
#define RouterLogWarningData        RouterLogWarningDataA
#define RouterLogInformationData    RouterLogInformationDataA
#define RouterLogEventString        RouterLogEventStringA
#define RouterLogEventEx            RouterLogEventExA
#define RouterLogEventValistEx      RouterLogEventValistExA
#define RouterLogErrorString        RouterLogErrorStringA
#define RouterLogWarningString      RouterLogWarningStringA
#define RouterLogInformationString  RouterLogInformationStringA
#define RouterGetErrorString        RouterGetErrorStringA
#endif


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// WORKER THREAD POOL FUNCTIONS                                             //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// definition of worker function passed in QueueWorkItem API                //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

typedef VOID (APIENTRY * WORKERFUNCTION)(PVOID);


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// ROUTER ASSERT DECLARATION                                                //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

_Analysis_noreturn_
VOID
RouterAssert(
    _In_ IN PSTR pszFailedAssertion,
    _In_ IN PSTR pszFileName,
    _In_ IN DWORD dwLineNumber,
    _In_ IN PSTR pszMessage OPTIONAL
    );


#if DBG
#define RTASSERT(exp) \
        if (!(exp)) \
            RouterAssert(#exp, __FILE__, __LINE__, NULL)
#define RTASSERTMSG(msg, exp) \
        if (!(exp)) \
            RouterAssert(#exp, __FILE__, __LINE__, msg)
#else
#define RTASSERT(exp)
#define RTASSERTMSG(msg, exp)
#endif

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// REGISTRY CONFIGURATION FUNCTIONS                                         //
//                                                                          //
// The following definitions are used to read configuration information     //
// about installed protocols.                                               //
//                                                                          //
// Call 'MprSetupProtocolEnum' to enumerate the routing-protocols           //
// for transport 'dwTransportId'. This fills an array with entries          //
// of type 'MPR_PROTOCOL_0'.                                                //
//                                                                          //
// The array loaded can be destroyed by calling 'MprSetupProtocolFree'.     //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define RTUTILS_MAX_PROTOCOL_NAME_LEN	                40
#define RTUTILS_MAX_PROTOCOL_DLL_LEN                    48

//
// the below two names should not be used
//

#ifndef MAX_PROTOCOL_NAME_LEN
#define MAX_PROTOCOL_NAME_LEN RTUTILS_MAX_PROTOCOL_NAME_LEN
#else
#undef MAX_PROTOCOL_NAME_LEN
#endif
#define MAX_PROTOCOL_DLL_LEN  RTUTILS_MAX_PROTOCOL_DLL_LEN



typedef struct _MPR_PROTOCOL_0 {

    DWORD       dwProtocolId;                           // e.g. IP_RIP
    WCHAR       wszProtocol[RTUTILS_MAX_PROTOCOL_NAME_LEN+1];   // e.g. "IPRIP"
    WCHAR       wszDLLName[RTUTILS_MAX_PROTOCOL_DLL_LEN+1];     // e.g. "iprip2.dll"

} MPR_PROTOCOL_0;


DWORD APIENTRY
MprSetupProtocolEnum(
    IN      DWORD                   dwTransportId,
    OUT     LPBYTE*                 lplpBuffer,         // MPR_PROTOCOL_0
    OUT     LPDWORD                 lpdwEntriesRead
    );


DWORD APIENTRY
MprSetupProtocolFree(
    IN      LPVOID                  lpBuffer
    );


//////////////////////////////////////////////////////////////////////////////
// Extensions to Rtutils to improve worker thread utilization.				//
// 																			//
//////////////////////////////////////////////////////////////////////////////

#define ROUTING_RESERVED
#define OPT1_1
#define OPT1_2
#define OPT2_1
#define OPT2_2
#define OPT3_1
#define OPT3_2


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // ___ROUTING_RTUTILS_H__


