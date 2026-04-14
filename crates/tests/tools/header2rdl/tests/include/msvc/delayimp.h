//
// DelayImp.h
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  Define structures and prototypes necessary for delay loading of imports
//
#pragma once

#define _DELAY_IMP_VER  2

#if defined(__cplusplus)
#define ExternC extern "C"
#else
#define ExternC extern
#endif

typedef IMAGE_THUNK_DATA *          PImgThunkData;
typedef const IMAGE_THUNK_DATA *    PCImgThunkData;
typedef DWORD                       RVA;

typedef struct ImgDelayDescr {
    DWORD           grAttrs;        // attributes
    RVA             rvaDLLName;     // RVA to dll name
    RVA             rvaHmod;        // RVA of module handle
    RVA             rvaIAT;         // RVA of the IAT
    RVA             rvaINT;         // RVA of the INT
    RVA             rvaBoundIAT;    // RVA of the optional bound IAT
    RVA             rvaUnloadIAT;   // RVA of optional copy of original IAT
    DWORD           dwTimeStamp;    // 0 if not bound,
                                    // O.W. date/time stamp of DLL bound to (Old BIND)
    } ImgDelayDescr, * PImgDelayDescr;

typedef const ImgDelayDescr *   PCImgDelayDescr;

enum DLAttr {                   // Delay Load Attributes
    dlattrRva = 0x1,                // RVAs are used instead of pointers
                                    // Having this set indicates a VC7.0
                                    // and above delay load descriptor.
    };

//
// Delay load import hook notifications
//
enum {
    dliStartProcessing,             // used to bypass or note helper only
    dliNoteStartProcessing = dliStartProcessing,

    dliNotePreLoadLibrary,          // called just before LoadLibrary, can
                                    //  override w/ new HMODULE return val
    dliNotePreGetProcAddress,       // called just before GetProcAddress, can
                                    //  override w/ new FARPROC return value
    dliFailLoadLib,                 // failed to load library, fix it by
                                    //  returning a valid HMODULE
    dliFailGetProc,                 // failed to get proc address, fix it by
                                    //  returning a valid FARPROC
    dliNoteEndProcessing,           // called after all processing is done, no
                                    //  bypass possible at this point except
                                    //  by longjmp()/throw()/RaiseException.
    };

typedef struct DelayLoadProc {
    BOOL                fImportByName;
    union {
        LPCSTR          szProcName;
        DWORD           dwOrdinal;
        };
    } DelayLoadProc;

typedef struct DelayLoadInfo {
    DWORD               cb;         // size of structure
    PCImgDelayDescr     pidd;       // raw form of data (everything is there)
    FARPROC *           ppfn;       // points to address of function to load
    LPCSTR              szDll;      // name of dll
    DelayLoadProc       dlp;        // name or ordinal of procedure
    HMODULE             hmodCur;    // the hInstance of the library we have loaded
    FARPROC             pfnCur;     // the actual function that will be called
    DWORD               dwLastError;// error received (if an error notification)
    } DelayLoadInfo, * PDelayLoadInfo;

typedef FARPROC (WINAPI *PfnDliHook)(
    unsigned        dliNotify,
    PDelayLoadInfo  pdli
    );

//
// Unload support
//

// routine definition; takes a pointer to a name to unload
//
ExternC
BOOL WINAPI
__FUnloadDelayLoadedDLL2(LPCSTR szDll);

//
// Snap load support
//
ExternC
HRESULT WINAPI
__HrLoadAllImportsForDll(LPCSTR szDll);


//
// Exception information
//
//#define FACILITY_VISUALCPP  ((LONG)0x6d)  now defined in winerror.h
#define VcppException(sev,err)  ((sev) | (FACILITY_VISUALCPP<<16) | err)

//
// Hook pointers
//

// The "notify hook" gets called for every call to the
// delay load helper.  This allows a user to hook every call and
// skip the delay load helper entirely.
//
// dliNotify == {
//  dliStartProcessing |
//  dliNotePreLoadLibrary  |
//  dliNotePreGetProc |
//  dliNoteEndProcessing}
//  on this call.
//
// Prior to Visual Studio 2015 Update 3, these hooks were non-const.  They were
// made const to improve security (global, writable function pointers are bad).
// If for backwards compatibility you require the hooks to be writable, define
// the macro DELAYIMP_INSECURE_WRITABLE_HOOKS prior to including this header and
// provide your own non-const definition of the hooks.
ExternC
#ifndef DELAYIMP_INSECURE_WRITABLE_HOOKS
const
#endif
PfnDliHook   __pfnDliNotifyHook2;

// This is the failure hook, dliNotify = {dliFailLoadLib|dliFailGetProc}
ExternC
#ifndef DELAYIMP_INSECURE_WRITABLE_HOOKS
const
#endif
PfnDliHook   __pfnDliFailureHook2;

// This is a global variable that can be redefined or overwritten by apps to
// change memory protection of the whole dload section instead of a smaller
// range that only includes the pages containing the delayload IAT entry to
// be updated.
ExternC
#ifndef DELAYIMP_WRITABLE_DLOAD_SECTION_PROTECTION_POLICY
const
#endif
BOOL         __bChangeProtectionOfWholeDloadSection;
