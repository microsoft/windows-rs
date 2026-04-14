//+-------------------------------------------------------------------------
//
//  IEProcess.h -- This module defines the IE APIs that only exist in the IExplore process.
// 
//  Copyright (c) Microsoft Corp. All rights reserved.
//
//--------------------------------------------------------------------------

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (_WIN32_IE >= _WIN32_IE_IE80)
#define IEPROCESS_MODULE_NAME               L"IERtUtil.dll"
#define IEGetProcessModule_PROC_NAME        "IEGetProcessModule"
#endif

/// <summary>
/// Get the module handle that provides per-process export tables.
/// Callers should use GetModuleHandle(IEPROCESS_MODULE_NAME)
/// and GetProcAddress(IEGetProcessModule_PROC_NAME) to bind to
/// this method.  
/// </summary>
/// <returns>
/// Non-NULL module handle if this process was initialized with 
///     IE process exports. Use handle when calling GetProcAddress() 
///     for any of the export table accessor (eg IETabWindowExports).  
///     The handle is invariant for the lifetime of the process.
/// NULL if the process has no process exports.
/// </returns>
typedef HMODULE (*IEGetProcessModule_t)();

// This struct encapsulate the tabdata that will not be changed during the lifetime of the tab thread
// so that this data can be copied and store in the TLS of the worker threads associated with the original tab thread
typedef struct _TLS_IMMUTABLE_TABDATA
{
    LONG  lTabId;               // The tab ID
    DWORD dwTabThreadId;        // The thread ID of the main UI thread
} TLS_IMMUTABLE_TABDATA;

/// <summary>
/// Table of tab window related functions exported by the IE process.
/// The table is invariant for the lifetime of the process.
/// </summary>
struct IE80TabWindowExports
{
    /// <summary>
    /// Tab windows are not always visible and IE avoids showing modal dialogs
    /// or showing new windows for a tab if the tab is hidden.  
    //  IE supports queuing of such actions so that when the tab is visible again, 
    /// this call will return. In addition to do the show/hide/enable logic correctly, 
    /// a different parent may need to be substituted.
    /// </summary>
    /// <param name="allowUnknownThread">Threads in a tab process are associated with 
    ///     a tab window where possible.  If this flag is false and the thread is not
    ///     associated with a tab window, the call will fail.  Callers should default 
    ///     to false.  Pass true to work even if the thread is unnassociated.  </param>
    /// <param name="proposedParentWindow">Proposed parent window to be used in 
    ///     CreateWindow(), MessageBox(), ShellExecute(), etc.</param>
    /// <param name="pActualParentWindow">Window that should be used instead
    ///     of the proposed parent.  Where possible this will be the proposed parent.</param>
    /// <returns>S_OK if the caller should continue, a failure code otherwise.</returns>
    /// <remarks>
    /// Method can block in it's own modal loop waiting for the parent window to be available.
    /// </remarks>
    HRESULT (WINAPI *WaitForTabWindow)(
        _In_    bool    allowUnknownThread,  
        _In_    HWND    proposedParentWindow,
        _Out_   HWND*   pActualParentWindow);  //  actual parent to use in following calls that require a parent.

    /// <summary>
    /// Helper for implementations that have their own dialog manager 
    /// or other similar modal loop.  Callers need to notify IE8 that 
    /// they will be running their loop by calling acquiring this lock.
    /// 
    /// In addition, the isolation boundaries need to be kept for 
    /// security reasons.  A new parent with the correct security
    /// mode and input forwarding will be provided if necessary.
    /// 
    /// When running in IE8, modal dialogs are treated specially.
    /// The security differences between windows in the hierarchy
    /// can affect whether or not a dialog will show up correctly.
    /// </summary>
    /// <param name="proposedParentWindow">Proposed parent window to be used in 
    ///     CreateWindow() and similar functions.</param>
    /// <param name="pActualParentWindow">Window that should be used instead
    ///     of the proposed parent.  Can be the same as the proposed parent.</param>
    /// <param name="phModalDialogLock">Handle to control the lifetime
    ///     of the lock.  Used when calling ReleaseModalDialogLockAndParent.
    ///     Can be NULL even in the success case.</param>
    /// <returns>S_OK if the caller should continue, a failure code otherwise.</returns>
    /// <remarks>
    /// This function uses COM and will CoInitialize() if necessary.
    /// It can also block in it's own modal loop making x-process COM calls.
    /// </remarks>
    HRESULT (WINAPI *AcquireModalDialogLockAndParent)(
        _In_    HWND    proposedParentWindow,
        _Out_   HWND*   pActualParentWindow,
        _Out_   HANDLE* phModalDialogLock);

    /// <summary>
    /// Free and cleanup any resources initialized in AcquireModalDialogLockAndParent()
    /// </summary>
    void (WINAPI *ReleaseModalDialogLockAndParent)(
        _In_opt_ HANDLE hModalDialogLock);

    /// <summary>
    /// Get the TLS data about the IE tab
    /// </summary>
    TLS_IMMUTABLE_TABDATA* (WINAPI *TLSGetImmutableTabData)();

    /// <summary>
    /// Set the TLS data about the IE tab
    /// </summary>
    HRESULT (WINAPI *TLSSetImmutableTabData)(TLS_IMMUTABLE_TABDATA* tlsITD);

    /// <summary>
    /// Free the TLS data about the IE tab
    /// </summary>
    void (WINAPI *TLSFreeImmutableTabData)();
};

#if (_WIN32_IE >= _WIN32_IE_IE80)
#define IETabWindowExports                  IE80TabWindowExports
#define IEGetTabWindowExports_PROC_NAME     "IEGetTabWindowExports"
#endif


/// <summary>
/// Get the table of tab window related exports for this process.
/// Callers should use the result of IEGetProcessModule() and 
/// GetProcAddress(IEGetTabWindowExports_PROC_NAME) to bind to
/// this method.  
/// </summary>
/// <returns>
/// Non-NULL table if this process exposes the tab window exports.
///     The table is invariant for the lifetime of the process.
/// NULL if the process has no process exports.
/// </returns>
typedef const struct IETabWindowExports* (*IEGetTabWindowExports_t)();

#if defined(__cplusplus) && !defined(NO_IEPROCESS_CLASS)
/// <summary>
/// Static helper class that encapsulates the kernel calls for
/// binding to the IEProcess export tables.
/// </summary>
class IEProcess
{
public:
    /// <summary>
    /// Gets the module handle that hosts the IE per-process exports.  
    /// Use in other IEProcess method calls.  This module handle
    /// doesn't need to be freed since it lasts the entire process.
    /// </summary>
    /// <returns>Non-NULL if any IE process exports have been initialized.</returns>
    static HMODULE 
    GetProcessModule()
    {
        HMODULE moduleHandle = ::GetModuleHandleW(IEPROCESS_MODULE_NAME);
        if (moduleHandle)
        {
            IEGetProcessModule_t IEGetProcessModule = (IEGetProcessModule_t)::GetProcAddress(moduleHandle, IEGetProcessModule_PROC_NAME);
            if (IEGetProcessModule)
            {
                return IEGetProcessModule();
            }
        }
        return NULL;
    }

    /// <summary>
    /// Gets the export table for Exports that implement IE process specific .  
    /// behavior relating to being hosted in a tab window.
    /// </summary>
    /// <returns>Non-NULL if the tab window exports have been initialized.</returns>
    static const IETabWindowExports* 
    GetTabWindowExports(HMODULE ieProcessModule)
    {
        if (ieProcessModule)
        {
            IEGetTabWindowExports_t IEGetTabWindowExports = (IEGetTabWindowExports_t)::GetProcAddress(ieProcessModule, IEGetTabWindowExports_PROC_NAME);
            if (IEGetTabWindowExports)
            {
                return IEGetTabWindowExports();
            }
        }
        return NULL;
    }      
};
#endif // NO_IEPROCESS_CLASS



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


