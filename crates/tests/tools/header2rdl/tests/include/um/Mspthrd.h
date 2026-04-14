/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    mspthrd.h

Abstract:

    Definitions for MSP thread management classes.

--*/

#ifndef __MSPTHRD_H
#define __MSPTHRD_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Commands that the worker worker thread can handle.
//

typedef enum
{
    WORK_ITEM,          // process a work item
    STOP,               // kill the worker thread.

} COMMAND;

typedef struct
{
    COMMAND                cmd;
    LPTHREAD_START_ROUTINE pfn;
    PVOID                  pContext;
    HANDLE                 hEvent;

} COMMAND_NODE;

typedef struct
{
    LIST_ENTRY  link;
    COMMAND_NODE node;

} COMMAND_QUEUE_ITEM;

typedef struct _NOTIF_LIST
{
    CMSPAddress      *addr;
    _NOTIF_LIST      *next;
} NOTIF_LIST, *PNOTIF_LIST;

class CMSPThread
{
public:
    CMSPThread()
    {
        InitializeListHead(&m_CommandQueue);

        m_hCommandEvent = NULL;
        m_hThread       = NULL;

        m_NotifList     = NULL;

        m_iStartCount = 0;
    }

    ~CMSPThread() { };

    HRESULT Start();
    HRESULT Stop();

    // Shutdown is used to clean up the thread unconditionally. This can be
    // used as an alternative to matched Start() / Stop() calls.

    HRESULT Shutdown();

    HRESULT ThreadProc();

    static LRESULT CALLBACK NotifWndProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

    HRESULT RegisterPnpNotification(CMSPAddress *pCMSPAddress);
    HRESULT UnregisterPnpNotification(CMSPAddress *pCMSPAddress);

    HRESULT QueueWorkItem(
        LPTHREAD_START_ROUTINE Function,
        PVOID Context,
        BOOL  fSynchronous
        );

private:
    BOOL SignalThreadProc() { return SetEvent(m_hCommandEvent); }

private:

    CMSPCritSection         m_CountLock;     // Protects start count
    CMSPCritSection         m_QueueLock;     // Protects command queue
    int                     m_iStartCount;   // number of times we've been
                                             // started minus number of times
                                             // we've been stopped. If == 0
                                             // then we actually stop thread.
    LIST_ENTRY              m_CommandQueue;  // Queue of commands for thread
                                             // to process.
    HANDLE                  m_hCommandEvent; // Signaled to tell us to do
                                             // something.

    HANDLE                  m_hThread;       // The thread handle. We need to
                                             // save it so that we can wait
                                             // for it when stopping the
                                             // thread.

    HDEVNOTIFY              m_hDevNotifyVideo;  // Handles of device notification registration
    HDEVNOTIFY              m_hDevNotifyAudio;  // for video and audio devices.

    HWND                    m_hWndNotif;     // Window handle for notification window

    PNOTIF_LIST             m_NotifList;     // List of notification functions to call
                                             // on a PNP event
    CMSPCritSection         m_NotifLock;     // Notification list critical section
};

extern CMSPThread g_Thread;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __MSPTHRD_H

// eof
