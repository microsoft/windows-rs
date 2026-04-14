/*++

Copyright (c) 2004  Microsoft Corporation

Module Name:

    aux_ulib.h

Abstract:

    User mode shim to access system functionality that is not properly exposed
    to applications in currently shipping operating systems.

--*/

#ifndef _AUX_SHLD_LIB_H
#define _AUX_SHLD_LIB_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif

/*++

Routine Description:

    This routine must be successfully called by an application before any
    other routine in the library may be called. It serves to initialize any global
    state that may be required by other routines in the file.

    It is safe to call this routine in a multi-threaded environment.
    
Arguments:

    None.

Return Value:

    Boolean status. Error code available via GetLastError ().

--*/

BOOL
WINAPI
AuxUlibInitialize (
    VOID
    );

/*++

Routine Description:

    This routine is used to set the current file system cache working set size. It 
    requires that the caller has enabled the SE_INCREASE_QUOTA_PRIVILEGE 
    in the currently active token prior to invoking this routine.

    This API is supported on Windows 2000 and later.
    
Arguments:

    MinimumFileCacheSize - The minimum file cache size. Use (SIZE_T)-1 if 
        the file cache is being flushed.

    MaximumFileCacheSize - The maximum file cache size. Use (SIZE_T)-1 
        if the file cache is being flushed.

    Flags - Flags relevant to the file cache size adjustment. Currently this must 
        be zero.

Return Value:

    Boolean status. Error code available via GetLastError (). If the routine is
        invoked prior to invoking the initialization routine, the returned error code
        will be ERROR_INVALID_FUNCTION.

--*/

BOOL 
WINAPI
AuxUlibSetSystemFileCacheSize (
    _In_ SIZE_T MinimumFileCacheSize,
    _In_ SIZE_T MaximumFileCacheSize,
    _In_ DWORD Flags
   );

/*++

Routine Description:

    This routine is used to determine whether or not the caller is executing
    code while holding a system synchronization primitive. Such a situation
    can arise when the OS temporarily calls into user-specified code as part 
    of the DLL load procedure.

    A caller can benefit from this information by avoiding operations that 
    could potentially lead to deadlocks, e.g., acquiring a process private lock.

    For example, consider the following case:

        Thread A runs the THREAD_ATTACH routine for DLL X. This routine
            is invoked with OS DLL synchronization held. Suppose further that
            as part of this routine Thread A acquires some lock in DLL X (Lx).

        Thread B runs some code in DLL X that, while holding Lx, calls the OS
            library loader to, e.g. GetModuleHandle. As this routine acquires
            OS DLL synchronization, Thread B will deadlock with Thread A.

        This is an inherent limitation in the design of the OS loader as it
        performs such callouts as THREAD_ATTACH while holding loader
        synchronization. It can be partially ameliorated if Thread A detects
        that it is running with DLL synchronization held and only try-acquires
        other locks (such as Lx) that it may wish to take
    
Arguments:

    SynchronizationHeld - Boolean value which indicates whether or not
        synchronization is held.

Return Value:

    Boolean status. Error code available via GetLastError (). If the routine is
        invoked prior to invoking the initialization routine, the returned error code
        will be ERROR_INVALID_FUNCTION.

--*/

BOOL
WINAPI
AuxUlibIsDLLSynchronizationHeld (
    _Out_ PBOOL SynchronizationHeld
    );

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _AUX_SHLD_LIB_H

