/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    pchannel.h

Abstract:

    Windows Terminal Server Virtual Channel Client API

Revision History:

--*/

#ifndef H_CCHANNEL
#define H_CCHANNEL

/****************************************************************************/
/* Include Virtual Channel Protocol header                                  */
/****************************************************************************/
#include <pchannel.h>

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_XP)

#ifdef _WIN32 
#define VCAPITYPE _stdcall
#define VCEXPORT
#else // _WIN32
#define VCAPITYPE CALLBACK
#define VCEXPORT  __export
#endif // _WIN32

/****************************************************************************/
/* Name: CHANNEL_INIT_EVENT_FN                                              */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is passed to MSTSC on VirtualChannelInit.  It is called by */
/* MSTSC to tell the application about interesting events.                  */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* none                                                                     */
/*                                                                          */
/* Params:                                                                  */
/*                                                                          */
/* - pInitHandle - a handle uniquely identifying this connection            */
/* - event - the event that has occurred - see CHANNEL_EVENT_XXX below      */
/* - pData - data associated with the event - see CHANNEL_EVENT_XXX below   */
/* - dataLength - length of the data.                                       */
/*                                                                          */
/****************************************************************************/
typedef VOID VCAPITYPE CHANNEL_INIT_EVENT_FN(LPVOID pInitHandle,
                                             UINT   event,
                                             LPVOID pData,
                                             UINT   dataLength);

typedef CHANNEL_INIT_EVENT_FN FAR * PCHANNEL_INIT_EVENT_FN;

/****************************************************************************/
/* Events passed to VirtualChannelInitEvent                                 */
/****************************************************************************/
/* Client initialized (no data)                                             */
#define CHANNEL_EVENT_INITIALIZED       0

/* Connection established (data = name of Server)                           */
#define CHANNEL_EVENT_CONNECTED         1

/* Connection established with old Server, so no channel support            */
#define CHANNEL_EVENT_V1_CONNECTED      2

/* Connection ended (no data)                                               */
#define CHANNEL_EVENT_DISCONNECTED      3

/* Client terminated (no data)                                              */
#define CHANNEL_EVENT_TERMINATED        4

/****************************************************************************/
/* Name: CHANNEL_OPEN_EVENT_FN                                              */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is passed to MSTSC on VirtualChannelOpen.  It is called by */
/* MSTSC when data is available on the channel.                             */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* none                                                                     */
/*                                                                          */
/* Params:                                                                  */
/*                                                                          */
/* - openHandle - a handle uniquely identifying this channel                */
/* - event - event that has occurred - see CHANNEL_EVENT_XXX below          */
/* - pData - data received                                                  */
/* - dataLength - length of the data                                        */
/* - totalLength - total length of data written by the Server               */
/* - dataFlags - flags, zero, one or more of:                               */
/*   - 0x01 - beginning of data from a single write operation at the Server */
/*   - 0x02 - end of data from a single write operation at the Server.      */
/*                                                                          */
/****************************************************************************/
typedef VOID VCAPITYPE CHANNEL_OPEN_EVENT_FN(DWORD  openHandle,
                                             UINT   event,
                                             LPVOID pData,
                                             UINT32 dataLength,
                                             UINT32 totalLength,
                                             UINT32 dataFlags);

typedef CHANNEL_OPEN_EVENT_FN FAR * PCHANNEL_OPEN_EVENT_FN;


/****************************************************************************/
/* Events passed to VirtualChannelOpenEvent                                 */
/****************************************************************************/
/* Data received from Server (data = incoming data)                         */
#define CHANNEL_EVENT_DATA_RECEIVED     10

/* VirtualChannelWrite completed (pData - pUserData passed on
   VirtualChannelWrite)                                                     */
#define CHANNEL_EVENT_WRITE_COMPLETE    11

/* VirtualChannelWrite cancelled (pData - pUserData passed on
   VirtualChannelWrite)                                                     */
#define CHANNEL_EVENT_WRITE_CANCELLED   12


/****************************************************************************/
/* Return codes from VirtualChannelXxx functions                            */
/****************************************************************************/
#define CHANNEL_RC_OK                             0
#define CHANNEL_RC_ALREADY_INITIALIZED            1
#define CHANNEL_RC_NOT_INITIALIZED                2
#define CHANNEL_RC_ALREADY_CONNECTED              3
#define CHANNEL_RC_NOT_CONNECTED                  4
#define CHANNEL_RC_TOO_MANY_CHANNELS              5
#define CHANNEL_RC_BAD_CHANNEL                    6
#define CHANNEL_RC_BAD_CHANNEL_HANDLE             7
#define CHANNEL_RC_NO_BUFFER                      8
#define CHANNEL_RC_BAD_INIT_HANDLE                9
#define CHANNEL_RC_NOT_OPEN                      10
#define CHANNEL_RC_BAD_PROC                      11
#define CHANNEL_RC_NO_MEMORY                     12
#define CHANNEL_RC_UNKNOWN_CHANNEL_NAME          13
#define CHANNEL_RC_ALREADY_OPEN                  14
#define CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY    15
#define CHANNEL_RC_NULL_DATA                     16
#define CHANNEL_RC_ZERO_LENGTH                   17
#define CHANNEL_RC_INVALID_INSTANCE              18
#define CHANNEL_RC_UNSUPPORTED_VERSION           19
#define CHANNEL_RC_INITIALIZATION_ERROR          20

/****************************************************************************/
/* Levels of Virtual Channel Support                                        */
/****************************************************************************/
#define VIRTUAL_CHANNEL_VERSION_WIN2000         1

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */
/****************************************************************************/
/* Name: VirtualChannelInit                                                 */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is called by the application to register the virtual       */
/* channels it wants to have access to.  Note that this does not open the   */
/* channels, merely reserves the names for use by this application.  This   */
/* function must be called before the Client connects to the Server, hence  */
/* it is recommended that it is called from the DLL's initialization        */
/* procedure.                                                               */
/*                                                                          */
/*                                                                          */
/* On_return, the channels requested have been registered.  However, other  */
/* MSTSC initialization may not yet have completed.  The application        */
/* receives a VirtualChannelInitEvent callback with the "Client             */
/* initialized" event when all MSTSC initialization is complete.            */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* CHANNEL_RC_OK                                                            */
/* CHANNEL_RC_ALREADY_INITIALIZED                                           */
/* CHANNEL_RC_ALREADY_CONNECTED                                             */
/* CHANNEL_RC_TOO_MANY_CHANNELS                                             */
/* CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY                                    */
/*                                                                          */
/* Parameters                                                               */
/*                                                                          */
/* - ppInitHandle (returned) - handle to pass to subsequent                 */
/*                             VirtualChannelXxx calls                      */
/* - pChannel - list of names registered by this application                */
/* - channelCount - number of channels registered.                          */
/* - versionRequested - level of virtual channel support requested (one of  */
/*                      the VIRTUAL_CHANNEL_LEVEL_XXX parameters)           */
/* - pChannelInitEventProc - address of VirtualChannelInitEvent procedure   */
/*                                                                          */
/****************************************************************************/
typedef UINT VCAPITYPE VIRTUALCHANNELINIT(
                LPVOID FAR *           ppInitHandle,
                PCHANNEL_DEF           pChannel,
                INT                    channelCount,
                ULONG                  versionRequested,
                PCHANNEL_INIT_EVENT_FN pChannelInitEventProc);

typedef VIRTUALCHANNELINIT FAR * PVIRTUALCHANNELINIT;


/****************************************************************************/
/* Name: VirtualChannelOpen                                                 */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is called by the application to open a channel.  It cannot */
/* be called until a connection is established with a Server.               */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* CHANNEL_RC_OK                                                            */
/* CHANNEL_RC_NOT_INITIALIZED                                               */
/* CHANNEL_RC_NOT_CONNECTED                                                 */
/* CHANNEL_RC_BAD_CHANNEL_NAME                                              */
/* CHANNEL_RC_BAD_INIT_HANDLE                                               */
/*                                                                          */
/* Params:                                                                  */
/*                                                                          */
/* - pInitHandle - handle from VirtualChannelInit                           */
/*                                                                          */
/* - pOpenHandle (returned) - handle to pass to subsequent                  */
/*                            VirtualChannelXxx calls                       */
/* - pChannelName - name of channel to open                                 */
/* - pChannelOpenEventProc - address of VirtualChannelOpenEvent procedure   */
/*                                                                          */
/****************************************************************************/
typedef UINT VCAPITYPE VIRTUALCHANNELOPEN(
                                LPVOID                 pInitHandle,
                                LPDWORD                pOpenHandle,
                                PCHAR                  pChannelName,
                                PCHANNEL_OPEN_EVENT_FN pChannelOpenEventProc);

typedef VIRTUALCHANNELOPEN FAR * PVIRTUALCHANNELOPEN;


/****************************************************************************/
/* Name: VirtualChannelClose                                                */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is called to close a previously opened channel.            */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* CHANNEL_RC_OK                                                            */
/* CHANNEL_RC_BAD_CHANNEL_HANDLE                                            */
/*                                                                          */
/* Params:                                                                  */
/*                                                                          */
/* - openHandle - handle returned on VirtualChannelOpen                     */
/*                                                                          */
/****************************************************************************/
typedef UINT VCAPITYPE VIRTUALCHANNELCLOSE(DWORD openHandle);

typedef VIRTUALCHANNELCLOSE FAR * PVIRTUALCHANNELCLOSE;


/****************************************************************************/
/* Name: VirtualChannelWrite                                                */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is used to send data to the partner app on the Server.     */
/*                                                                          */
/* VirtualChannelWrite copies the data to one or more network buffers as    */
/* necessary.  VirtualChannelWrite ensures that data is sent to the Server  */
/* on the right context.  It sends all data on MS TC's Sender thread.       */
/*                                                                          */
/* VirtualChannelWrite is asynchronous - the VirtualChannelOpenEvent        */
/* procedure is called when the write completes.  Until that callback is    */
/* made, the caller must not free or reuse the buffer passed on             */
/* VirtualChannelWrite.  The caller passes a piece of data (pUserData) to   */
/* VirtualChannelWrite, which is returned on the VirtualChannelOpenEvent    */
/* callback.  The caller can use this data to identify the write which has  */
/* completed.                                                               */
/*                                                                          */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* CHANNEL_RC_OK                                                            */
/* CHANNEL_RC_NOT_INITIALIZED                                               */
/* CHANNEL_RC_NOT_CONNECTED                                                 */
/* CHANNEL_RC_BAD_CHANNEL_HANDLE                                            */
/*                                                                          */
/* Params:                                                                  */
/* - openHandle - handle from VirtualChannelOpen                            */
/* - pData - data to write                                                  */
/* - datalength - length of data to write                                   */
/* - pUserData - user supplied data, returned on VirtualChannelOpenEvent    */
/*               when the write completes                                   */
/*                                                                          */
/****************************************************************************/
typedef UINT VCAPITYPE VIRTUALCHANNELWRITE(DWORD  openHandle,
                                           LPVOID pData,
                                           ULONG  dataLength,
                                           LPVOID pUserData);

typedef VIRTUALCHANNELWRITE FAR * PVIRTUALCHANNELWRITE;


/****************************************************************************/
/* Structure: CHANNEL_ENTRY_POINTS                                          */
/*                                                                          */
/* Description: Virtual Channel entry points passed to VirtualChannelEntry  */
/****************************************************************************/
typedef struct tagCHANNEL_ENTRY_POINTS
{
    DWORD cbSize;
    DWORD protocolVersion;
    PVIRTUALCHANNELINIT  pVirtualChannelInit;
    PVIRTUALCHANNELOPEN  pVirtualChannelOpen;
    PVIRTUALCHANNELCLOSE pVirtualChannelClose;
    PVIRTUALCHANNELWRITE pVirtualChannelWrite;
} CHANNEL_ENTRY_POINTS, FAR * PCHANNEL_ENTRY_POINTS;


/****************************************************************************/
/* Name: VirtualChannelEntry                                                */
/*                                                                          */
/* Purpose:                                                                 */
/*                                                                          */
/* This function is provided by addin DLLS.  It is called by MSTSC at       */
/* initialization to tell the addin DLL the addresses of the                */
/* VirtualChannelXxx functions.                                             */
/*                                                                          */
/* Returns:                                                                 */
/*                                                                          */
/* TRUE - everything OK                                                     */
/* FALSE - error, unload the DLL                                            */
/*                                                                          */
/* Parameters:                                                              */
/*                                                                          */
/* - pVirtualChannelInit - pointers to VirtualChannelXxx functions          */
/* - pVirtualChannelOpen                                                    */
/* - pVirtualChannelClose                                                   */
/* - pVirtualChannelWrite                                                   */
/*                                                                          */
/****************************************************************************/
typedef BOOL VCAPITYPE VIRTUALCHANNELENTRY(
                                          PCHANNEL_ENTRY_POINTS pEntryPoints);

typedef VIRTUALCHANNELENTRY FAR * PVIRTUALCHANNELENTRY;


#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif /* (NTDDI_VERSION >= NTDDI_XP) */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* H_CCHANNEL */
