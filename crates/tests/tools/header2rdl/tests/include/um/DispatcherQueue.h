
//----------------------------------------------------------------------------
//----------------------------------------------------------------------------
//
// DispatcherQueue.h
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
// Contains DispatcherQueue declarations for Native C++ consumers.
//
//----------------------------------------------------------------------------
//----------------------------------------------------------------------------

#pragma once


#include <Windows.System.h>
typedef ABI::Windows::System::IDispatcherQueue * PDISPATCHERQUEUE;
typedef ABI::Windows::System::IDispatcherQueueController * PDISPATCHERQUEUECONTROLLER;


#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

enum DISPATCHERQUEUE_THREAD_APARTMENTTYPE
{
    DQTAT_COM_NONE = 0,
    DQTAT_COM_ASTA = 1,
    DQTAT_COM_STA = 2
};


enum DISPATCHERQUEUE_THREAD_TYPE
{
    DQTYPE_THREAD_DEDICATED = 1,
    DQTYPE_THREAD_CURRENT = 2,
};


struct DispatcherQueueOptions
{
    DWORD                                   dwSize;          // Size of the struct
    DISPATCHERQUEUE_THREAD_TYPE             threadType;      // Thread affinity on which DispatcherQueueController is created.
    DISPATCHERQUEUE_THREAD_APARTMENTTYPE    apartmentType;   // Initialize COM apartment on the new thread as ASTA or STA. Only relevant if threadType is DQTYPE_THREAD_DEDICATED
};


//----------------------------------------------------------------------------
//
// CreateDispatcherQueueController
//
//     This is Win32 API to create a DispatcherQueueController instance. 
//     Passed in options will be required to configure the type of 
//     instance being created.
//
//     If the options.threadType = DQTYPE_THREAD_DEDICATED then,
//     DispatcherQueueController will be created on the dedicated thread. As part
//     of the API we will also create the thread and create this object
//     on it. It will also run the DispatcherQueue event loop on the new thread.
//
//     PS: API will be blocked until it spawns a new thread and creates the
//     DispatcherQueueController instance. Also specified COM apartment will
//     be initialized on that thread.
//
//     Event loop will be run on the background thread asynchronously to dispatch
//     the pending task items on the new thread.
//
//
//     if the options.threadType = DQTYPE_THREAD_CURRENT then,
//     DispatcherQueueController instance is created on the current thread owned
//     by the caller or some other module. API will throw if there already 
//     exists a DispatcherQueueController on this thread.
//
//     It is the responsibility of the caller to run a NTUSER Message pump loop
//     so as to dispatch DispatcherQueue's tasks onto this thread.
//
//     Associated DispatcherQueue instance  can be accessed as its property.
//
// Parameters:
//     options - struct to configure the DispatcherQueueController initialization.
//
//     dispatcherQueueController - An OUT parameter returning instance of
//     DispatcherQueueController.
//
// Returns:
//     HRESULT - S_OK on success, otherwise failure result.
//
//----------------------------------------------------------------------------

extern "C" HRESULT  __declspec(dllexport) WINAPI
CreateDispatcherQueueController(
    _In_ DispatcherQueueOptions options,
    _Deref_out_ PDISPATCHERQUEUECONTROLLER * dispatcherQueueController);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

