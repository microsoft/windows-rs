/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rasshost.h

Abstract:

    This header defines the interface between third party security
    DLLs and the RAS server.
    
--*/

#ifndef _RASSHOST_
#define _RASSHOST_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <mprapi.h>

typedef HANDLE  HPORT;

typedef struct _SECURITY_MESSAGE
{
    DWORD dwMsgId;

    HPORT hPort;

    DWORD dwError;                  // Should be non-zero only if error
                                    // occurred during the security dialog.
                                    // Should contain errors from winerror.h
                                    // or raserror.h

    CHAR  UserName[UNLEN+1];        // Should always contain username if
                                    // dwMsgId is SUCCESS/FAILURE

    CHAR  Domain[DNLEN+1];          // Should always contain domain if
                                    // dwMsgId is SUCCESS/FAILURE

} SECURITY_MESSAGE, *PSECURITY_MESSAGE;


// Values for dwMsgId in SECURITY_MESSAGE structure

#define SECURITYMSG_SUCCESS     1
#define SECURITYMSG_FAILURE     2
#define SECURITYMSG_ERROR       3

// Used by RasSecurityGetInfo call

typedef struct _RAS_SECURITY_INFO
{

    DWORD LastError;                    // SUCCESS = receive completed
                                        // PENDING = receive pending
                                        // else completed with error

    DWORD BytesReceived;                // only valid if LastError == SUCCESS

    CHAR  DeviceName[MAX_DEVICE_NAME+1];


}RAS_SECURITY_INFO,*PRAS_SECURITY_INFO;

typedef DWORD (WINAPI *RASSECURITYPROC)();

//
// Called by third party DLL to notify the supervisor of termination of
// the security dialog
//

VOID WINAPI
RasSecurityDialogComplete(
    IN SECURITY_MESSAGE * pSecMsg       // Pointer to the above info. structure
);

//
// Called by supervisor into the security DLL to notify it to begin the
// security dialog for a client.
//
// Should return errors from winerror.h or raserror.h
//

DWORD WINAPI
RasSecurityDialogBegin(
    IN HPORT  hPort,        // RAS handle to port
    IN PBYTE  pSendBuf,     // Pointer to the buffer used in
                            // RasSecurityDialogSend
    IN DWORD  SendBufSize,  // Size of above bufer in bytes
    IN PBYTE  pRecvBuf,     // Pointer to the buffer used in
                            // RasSecurityDialogReceive
    IN DWORD  RecvBufSize,  // Size of above buffer
    IN VOID  (WINAPI *RasSecurityDialogComplete)( SECURITY_MESSAGE* )
                            // Pointer to function RasSecurityDialogComplete.
                            // Guaranteed to be the same on every call.
);

//
// Called by supervisor into the security DLL to notify it to stop the
// security dialog for a client. If this call returns an error, then it is not
// neccesary for the dll to call RasSecurityDialogComplete. Otherwise the DLL
// must call RasSecurityDialogComplete.
//
// Should return errors from winerror.h or raserror.h
//

DWORD WINAPI
RasSecurityDialogEnd(
    IN HPORT    hPort           // RAS handle to port.
);

//
// The following entrypoints should be loaded by calling GetProcAddress from
// RasMan.lib
//
// Called to send data to remote host
// Will return errors from winerror.h or raserror.h
//

DWORD WINAPI
RasSecurityDialogSend(
    IN HPORT    hPort,          // RAS handle to port.
    IN PBYTE    pBuffer,        // Pointer to buffer containing data to send
    IN WORD     BufferLength    // Length of above buffer.
);

//
// Called to receive data from remote host
// Will return errors from winerror.h or raserror.h
//

DWORD WINAPI
RasSecurityDialogReceive(
    IN HPORT    hPort,          // RAS handle to port.
    IN PBYTE    pBuffer,        // Pointer to buffer to receive data
    IN PWORD    pBufferLength,  // length of data received in bytes.
    IN DWORD    Timeout,        // in seconds
    IN HANDLE   hEvent          // Event to set when receive completes or
                                // timeouts
);

//
// Called to get Information about port.
// Will return errors from winerror.h or raserror.h
//

DWORD WINAPI
RasSecurityDialogGetInfo(
    IN HPORT                hPort,      // RAS handle to port.
    IN RAS_SECURITY_INFO*   pBuffer     // Pointer to get info structure.
);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
