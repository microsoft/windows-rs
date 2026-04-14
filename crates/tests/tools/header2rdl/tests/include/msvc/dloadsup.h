/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    dloadsup.h

Abstract:

    This module implements downlevel support for read only delay load import
    tables. (From nt source: minkernel\dload\helper\dloadsup.h)


    For the non-explicitly-unloading delay load helper library :

    DLOAD_UNLOAD is not defined or is defined to 0.

    If the host OS supports delay load natively in the loader, then
    the native loader support is invoked.  Otherwise, the built-in support for
    resolving delay load imports is invoked.


    For the explicitly-unloading delay load helper library (the "VS" version) :

    DLOAD_UNLOAD is defined to 1.

    If the image load config directory marks the image as having a protected
    delay load section, then on each delay load resolution request, the module
    delay load protection is changed for the duration of the request (and then
    changed back afterwards).  Local support for delay loads is used so as to
    retain compatibility with programs customizing the behavior of the delay
    load notification hook (and the ability to perform an explicit module
    unload).

--*/

#pragma once

#ifndef _DLOAD_DLOADSUP_H
#define _DLOAD_DLOADSUP_H

#pragma warning(push)
#pragma warning(disable:4714) // forceinline function 'YieldProcessor' not inlined
#pragma warning(disable:28112) // Disable Prefast warning about variables being accessed through both interlocked and non-interlocked functions

#define DLOAD_INLINE __inline

#define FAST_FAIL_DLOAD_PROTECTION_FAILURE    25
#define IMAGE_GUARD_PROTECT_DELAYLOAD_IAT             0x00001000

#if DBG

#define DLOAD_ASSERT(_exp) \
    ((!(_exp)) ? \
        (__annotation(L"Debug", L"AssertFail", L## #_exp), \
         DbgRaiseAssertionFailure(), FALSE) : \
        TRUE)

#else

#define DLOAD_ASSERT(_exp) ((void) 0)

#endif

#if DLOAD_UNLOAD

//++
//
// ULONG
// DLOAD_BYTE_OFFSET (
//     _In_ PVOID Va,
//     _In_ SIZE_T PageSize
//     )
//
// Routine Description:
//
//     The DLOAD_BYTE_OFFSET macro takes a virtual address and returns the byte offset
//     of that address within the page.
//
// Arguments:
//
//     Va - Virtual address.
//
//     PageSize - System page size in bytes.
//
// Return Value:
//
//     Returns the byte offset portion of the virtual address.
//
//--

#define DLOAD_BYTE_OFFSET(Va, PageSize) ((ULONG)((LONG_PTR)(Va) & (PageSize - 1)))

//++
//
// ULONG
// ADDRESS_AND_SIZE_TO_SPAN_PAGES (
//     _In_ PVOID Va,
//     _In_ ULONG Size,
//     _In_ SIZE_T PageSize
//     )
//
// Routine Description:
//
//     The ADDRESS_AND_SIZE_TO_SPAN_PAGES macro takes a virtual address and
//     size and returns the number of pages spanned by the size.
//
// Arguments:
//
//     Va - Virtual address.
//
//     Size - Size in bytes.
//
//     PageSize - System page size in bytes.
//
// Return Value:
//
//     Returns the number of pages spanned by the size.
//
//--

#define DLOAD_ADDRESS_AND_SIZE_TO_SPAN_PAGES(Va,Size,PageSize) \
    ((ULONG)((((ULONG_PTR)(Size)) / PageSize) + ((DLOAD_BYTE_OFFSET (Va, PageSize) + DLOAD_BYTE_OFFSET (Size, PageSize) + PageSize - 1) / PageSize)))

#define SRWLOCK_UNINITIALIZED ((HMODULE)0x0)
#define SRWLOCK_UNSUPPORTED ((HMODULE)0x1)

typedef ULONG_PTR SRWLOCK_TYPE;

typedef
VOID
(NTAPI *
AcquireSRWLockExclusiveProc) (
    _Inout_ _Acquires_exclusive_lock_(*SRWLock) SRWLOCK_TYPE *SRWLock
    );

typedef
VOID
(NTAPI *
ReleaseSRWLockExclusiveProc) (
    _Inout_ _Releases_exclusive_lock_(*SRWLock) SRWLOCK_TYPE *SRWLock
    );

HMODULE DloadKernel32 = SRWLOCK_UNINITIALIZED;
AcquireSRWLockExclusiveProc DloadAcquireSRWLockExclusive;
ReleaseSRWLockExclusiveProc DloadReleaseSRWLockExclusive;
SRWLOCK_TYPE DloadSrwLock = 0x0;
ULONG DloadSectionLockCount;
DWORD DloadSectionOldProtection;
ULONG DloadSectionCommitPermanent;

extern "C" IMAGE_LOAD_CONFIG_DIRECTORY _load_config_used;

extern "C" const IMAGE_DOS_HEADER __ImageBase;

extern "C" const BOOL __bChangeProtectionOfWholeDloadSection;

#else

#define DLOAD_UNSUPPORTED ((HMODULE)0x1)

typedef
PVOID
(NTAPI *
ResolveDelayLoadedAPIProc) (
    _In_ PVOID ParentModuleBase,
    _In_ PCIMAGE_DELAYLOAD_DESCRIPTOR DelayloadDescriptor,
    _In_opt_ PDELAYLOAD_FAILURE_DLL_CALLBACK FailureDllHook,
    _In_opt_ PDELAYLOAD_FAILURE_SYSTEM_ROUTINE FailureSystemHook,
    _Out_ PIMAGE_THUNK_DATA ThunkAddress,
    _Reserved_ ULONG Flags
    );

typedef
NTSTATUS
(NTAPI *
ResolveDelayLoadsFromDllProc) (
    _In_ PVOID ParentBase,
    _In_ LPCSTR TargetDllName,
    _Reserved_ ULONG Flags
    );

HMODULE DloadKernel32;
ResolveDelayLoadedAPIProc DloadResolveDelayLoadedAPI;
ResolveDelayLoadsFromDllProc DloadResolveDelayLoadsFromDll;

#endif

//
// The following dload support APIs are used for the explicit-unloading version
// of the delay load helper.
//

#if DLOAD_UNLOAD

DLOAD_INLINE
BOOLEAN
DloadGetSRWLockFunctionPointers (
    VOID
    )

/*++

Routine Description:

    This function obtains pointers to SRWLock Acquire and Release
    functions.

Arguments:

    None.

Return Value:

    TRUE is returned as the function value if the host OS supports SRW locks
    and the function pointers have been initialized.

--*/

{

    FARPROC FunctionPointer;
    HMODULE Kernel32;
    HMODULE OldValue;

    Kernel32 = (HMODULE)ReadPointerAcquire((PVOID *) &DloadKernel32);
    if (Kernel32 == SRWLOCK_UNSUPPORTED) {
        return FALSE;
    }
    if (Kernel32 != NULL) {
        return TRUE;
    }

    Kernel32 = GetModuleHandleW(L"KERNEL32.DLL");
    if (Kernel32 == NULL) {
        Kernel32 = SRWLOCK_UNSUPPORTED;
        goto Done;
    }

    FunctionPointer = GetProcAddress(Kernel32, "AcquireSRWLockExclusive");
    if (FunctionPointer == NULL) {
        Kernel32 = SRWLOCK_UNSUPPORTED;
        goto Done;
    }

    DloadAcquireSRWLockExclusive = (AcquireSRWLockExclusiveProc)FunctionPointer;
    FunctionPointer = GetProcAddress(Kernel32, "ReleaseSRWLockExclusive");
    if (FunctionPointer == NULL) {
        Kernel32 = SRWLOCK_UNSUPPORTED;
        goto Done;
    }

    DloadReleaseSRWLockExclusive = (ReleaseSRWLockExclusiveProc)FunctionPointer;

Done:
    OldValue = (HMODULE)InterlockedCompareExchangePointer((PVOID *)&DloadKernel32,
                                                 (PVOID)Kernel32,
                                                 SRWLOCK_UNINITIALIZED);

    if (((OldValue == SRWLOCK_UNINITIALIZED) &&
         (Kernel32 == SRWLOCK_UNSUPPORTED)) ||
        (OldValue == SRWLOCK_UNSUPPORTED)) {

        return FALSE;
    }

    return TRUE;
}

_Acquires_lock_(DloadSrwLock)
DLOAD_INLINE
VOID
DloadLock (
    VOID
    )

/*++

Routine Description:

    This function obtains the delay load unload lock.

Arguments:

    None.

Return Value:

    None.

--*/

{
	if (DloadGetSRWLockFunctionPointers() != FALSE) {
        DloadAcquireSRWLockExclusive(&DloadSrwLock);
        return;
    }

    for ( ; ; ) {
        while (ReadPointerAcquire((PVOID *)&DloadSrwLock) != 0) {
            YieldProcessor();
        }

        if (InterlockedCompareExchangePointer((PVOID *)&DloadSrwLock, (PVOID)1, 0) == 0) {
            break;
        }
    }

    return;
}

_Releases_lock_(DloadSrwLock)
DLOAD_INLINE
VOID
DloadUnlock (
    VOID
    )

/*++

Routine Description:

    This function releases the delay load unload lock.

Arguments:

    None.

Return Value:

    None.

--*/

{
    if (DloadGetSRWLockFunctionPointers() != FALSE) {
         DloadReleaseSRWLockExclusive(&DloadSrwLock);

    } else {
#pragma warning(suppress:6387) // Passing 0 is valid, but WritePointerRelease isn't annotated properly
        WritePointerRelease((PVOID *)&DloadSrwLock, 0);
    }

    return;
}

_Success_(return != nullptr)
DLOAD_INLINE
PVOID
DloadObtainSection (
    _Out_ PULONG SectionSize,
    _Out_ PULONG SectionCharacteristics
    )

/*++

Routine Description:

    This function locates the delay load import table section for the current
    module.

Arguments:

    SectionSize - Receives the size, in bytes, of the delay load import
                  section.

    SectionCharacteristics - Receives the section characteristics.

Return Value:

    A pointer to the delay load section base is returned, else NULL if the
    image does not require processing.

--*/

{

    PIMAGE_DATA_DIRECTORY DataDir;
    PCIMAGE_DELAYLOAD_DESCRIPTOR DloadDesc;
    ULONG Entries;
    PUCHAR ImageBase;
    ULONG Index;
    PIMAGE_NT_HEADERS NtHeaders;
    ULONG Rva;
    PIMAGE_SECTION_HEADER SectionHeader;

    ImageBase = (PUCHAR)&__ImageBase;
    NtHeaders = (PIMAGE_NT_HEADERS)(ImageBase + __ImageBase.e_lfanew);
    Entries = NtHeaders->OptionalHeader.NumberOfRvaAndSizes;

    if (IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT >= Entries) {
        return NULL;
    }

    DataDir = &NtHeaders->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT];
    Rva = DataDir->VirtualAddress;
    if (Rva == 0) {
        return NULL;
    }

    DloadDesc = (PCIMAGE_DELAYLOAD_DESCRIPTOR)(ImageBase + Rva);
    SectionHeader = IMAGE_FIRST_SECTION(NtHeaders);
    Rva = DloadDesc->ImportAddressTableRVA;
    for (Index = 0;
         Index < NtHeaders->FileHeader.NumberOfSections;
         Index += 1, SectionHeader += 1) {

        if ((Rva >= SectionHeader->VirtualAddress) &&
            (Rva < SectionHeader->VirtualAddress + SectionHeader->Misc.VirtualSize)) {

            *SectionSize = SectionHeader->Misc.VirtualSize;
            *SectionCharacteristics = SectionHeader->Characteristics;
            return ImageBase + SectionHeader->VirtualAddress;
        }
    }

    return NULL;
}

DECLSPEC_NOINLINE
DLOAD_INLINE
VOID
DloadMakePermanentImageCommit (
    _Inout_updates_ (Length) PVOID BaseAddress,
    _In_ SIZE_T Length
    )

/*++

Routine Description:

    This routine takes a set of image pages that are protected as writable
    (i.e. which have commit precharged) and performs a dummy write to each
    contained page so that the commit currently charged cannot be released by
    memory management if any pages were not yet dirtied when they are
    protected to PAGE_READONLY.

    This avoids a low resources failure path during future reprotections where
    the region spanned will be repeatedly toggled between PAGE_READWRITE and
    PAGE_READONLY.

    N.B.  This routine is called with delay load unload lock held and returns
          with the delay load lock held (i.e., there can be no competing call
          to change the section protection).

Arguments:

    BaseAddress - Supplies the base of the region of pages.

    Length - Supplies the length, in bytes, of the region.

Return Value:

    None.

--*/

{

    MEMORY_BASIC_INFORMATION MemoryInfo;
    PUCHAR Page;
    SIZE_T PageCount;
    SIZE_T PageSize;
    SYSTEM_INFO SystemInfo;

    //
    // Determine if the delay load section is already read only.  If so, i.e.
    // the loader was not enlightened to support protected delay load
    // internally, then ensure that commit precharged for the writable section
    // is made permanent before the section is made read only.  Otherwise,
    // future calls to VirtualProtect from PAGE_READONLY to PAGE_READWRITE
    // could require memory management to obtain additional commit charges -
    // which could fail in low resources conditions.  In order to maintain
    // existing contractural behaviors with delay loads it is necessary to not
    // introduce additional low resources failure paths with protected delay
    // load support.
    //
    // N.B.  This call to VirtualQuery cannot fail without an invalid argument
    //       which should not be possible as all arguments are sourced from
    //       the image header which is constant data.
    //

    if (VirtualQuery(BaseAddress, &MemoryInfo, sizeof(MemoryInfo)) == 0) {
        DLOAD_ASSERT(FALSE);
        __fastfail(FAST_FAIL_DLOAD_PROTECTION_FAILURE);
    }

    //
    // If the page is not writable then the loader has already assumed
    // responsibility for charging commit.  In this case, nothing must be done
    // to ensure forward progress.
    //

    if ((MemoryInfo.Protect & (PAGE_READWRITE | PAGE_EXECUTE_READWRITE)) == 0) {
        return;
    }

    GetSystemInfo(&SystemInfo);

    PageSize = SystemInfo.dwPageSize;

    //
    // The loader is not guaranteed to have charged commit.  Access every page
    // in the delay import section to make the commit already precharged
    // "permanent", such that it cannot be revoked by memory management once
    // the pages are transitioned from PAGE_READWRITE to PAGE_READONLY.  A
    // dummy write will dirty each page in succession so that the copy on write
    // split happens now (which precludes memory management returning the
    // commit charges when the pages are returned to PAGE_READONLY protection).
    //

    PageCount = DLOAD_ADDRESS_AND_SIZE_TO_SPAN_PAGES(BaseAddress,
                                                     Length,
                                                     PageSize);
    Page = (PUCHAR)((ULONG_PTR)BaseAddress & ~(PageSize - 1));

    while (PageCount != 0) {
        InterlockedOr((PLONG)Page, 0);
        Page += PageSize;
        PageCount -= 1;
    }

    return;
}

DLOAD_INLINE
BOOLEAN
IsBufferInBounds (
    _In_ PVOID OuterPointer,
    _In_ SIZE_T OuterSize,
    _In_ PVOID InnerPointer,
    _In_ SIZE_T InnerSize
    )
/*++

Routine Description:

    This function checks if an inner buffer is in an outer buffer's
    bound.

Arguments:

    OuterPointer - supplies the beginning address of the outer buffer.

    OuterSize - supplies the size of the outer buffer in bytes.

    InnerPointer - supplies the beginning address of the inner buffer.

    InnerSize - supplies the size of the inner buffer in bytes.

Return Value:

    TRUE if the inner buffer is within the outer buffer's bound, FALSE
    otherwise including the case of overflow or empty ranges.

--*/
{
    ULONG_PTR OuterStart = (ULONG_PTR)OuterPointer;
    ULONG_PTR InnerStart = (ULONG_PTR)InnerPointer;
    ULONG_PTR OuterEnd = OuterStart + OuterSize;
    ULONG_PTR InnerEnd = InnerStart + InnerSize;

    //
    // Check for overflow or empty ranges
    //

    if ((OuterEnd <= OuterStart) || (InnerEnd <= InnerStart)) {
        return FALSE;
    }

    //
    // Check if Inner fits inside Outer
    //

    if ((InnerStart >= OuterStart) && (InnerEnd <= OuterEnd)) {
        return TRUE;
    }

    return FALSE;
}

DLOAD_INLINE
VOID
DloadProtectSection (
    _In_opt_ PVOID RangeStart,
    _In_ SIZE_T RangeSize,
    _In_ ULONG Protection,
    _Out_ PULONG OldProtection
    )

/*++

Routine Description:

    This function changes the protection of the delay load section for this
    module.

    N.B.  This function is called with the dload protection lock held and
          returns with the dload protection lock held.

Arguments:

    RangeStart - supplies a pointer to the beginning address of the range. If
                 it is nullptr, then the entire delayload section is specified.

    RangeSize - supplies the size of the range, in bytes.  It must be 0 if
                RangeStart is nullptr.

    Protection - Supplies the new section protection.

    OldProtection - Receives the old section protection.

Return Value:

    None.  This function may fast-fail if a bad image is produced in which the
           dload section is not marked as writable, or something is wrong with
           the OS, or for whatever reason an invalid range is given.  Otherwise,
           it should always succeed.

--*/

{

    ULONG Characteristics;
    PVOID Section;
    ULONG Size;

    Section = DloadObtainSection(&Size, &Characteristics);

    if (Section == NULL) {
        *OldProtection = PAGE_READWRITE;
        return;
    }

    //
    // Ensure that commit obtained by memory management for the read/write
    // state of the delay load import section cannot be released by a
    // VirtualProtect to a read only protection, before the first transition to
    // read only.  This is required in order to ensure that future calls to
    // VirtualProtect are guaranteed to support forward progress under low
    // resources.
    //

    if (DloadSectionCommitPermanent == 0) {
        DloadSectionCommitPermanent = 1;

        if ((Characteristics & IMAGE_SCN_MEM_WRITE) == 0) {

            //
            // This delay load helper module does not support merging the delay
            // load section to a read only section because memory management
            // would not guarantee that there is commit available - and thus a
            // low memory failure path where the delay load failure hook could
            // not be safely invoked (the delay load section would still be
            // read only) might be encountered.
            //
            // It is a build time configuration problem to produce such a
            // binary so abort here and now so that the problem can be
            // identified & fixed.
            //

            __fastfail(FAST_FAIL_DLOAD_PROTECTION_FAILURE);
        }

        DloadMakePermanentImageCommit(Section, Size);
    }

    //
    // If the range is invalid, fast fail.
    //

    if ((RangeStart != nullptr && RangeSize == 0) ||
        (RangeStart == nullptr && RangeSize != 0)) {
        __fastfail(FAST_FAIL_DLOAD_PROTECTION_FAILURE);
    }

    if (RangeStart == nullptr) {
        RangeStart = Section;
        RangeSize = Size;
    } else if (IsBufferInBounds(Section, Size, RangeStart, RangeSize) == FALSE) {
        __fastfail(FAST_FAIL_DLOAD_PROTECTION_FAILURE);
    }

    //
    // Protect the delay load import section.
    //
    // N.B.  This call cannot fail unless an argument is invalid and all
    //       arguments come from the image header.
    //

    if (VirtualProtect(RangeStart, RangeSize, Protection, OldProtection) == FALSE) {
        DLOAD_ASSERT(FALSE);
        __fastfail(FAST_FAIL_DLOAD_PROTECTION_FAILURE);
    }

    return;
}

DLOAD_INLINE
VOID
DloadAcquireSectionWriteAccess (
    VOID
    )

/*++

Routine Description:

    This function obtains write access to the specified range of the delay
    load section.  Until a matched call to DloadReleaseSectionWriteAccess is
    made the section is still considered writable.

Arguments:

    This function obtains write access to the delay load section.  Until a
    matched call to DloadReleaseSectionAccess is made the section is still
    considered writable.

Arguments:

    None.

--*/

{

    //
    // If protected delay load is not in use, there is nothing to do.
    //

    if ((_load_config_used.GuardFlags & IMAGE_GUARD_PROTECT_DELAYLOAD_IAT) == 0) {
        return;
    }

    if (__bChangeProtectionOfWholeDloadSection == FALSE) {
        return;
    }

    //
    // Acquire the Dload protection lock for this module and change protection.
    //

    DloadLock();

    DloadSectionLockCount += 1;
    if (DloadSectionLockCount == 1) {
        DloadProtectSection(nullptr,
                            0,
                            PAGE_READWRITE,
                            &DloadSectionOldProtection);
    }

    DloadUnlock();

    return;
}

DLOAD_INLINE
VOID
DloadReleaseSectionWriteAccess (
    VOID
    )

/*++

Routine Description:

    This function relinquishes write access to the specified range of the
    delay load section.

Arguments:

    This function relinquishes write access to the delay load section.

Arguments:

    None.

--*/

{

    ULONG OldProtect;

    //
    // If protected delay load is not in use, there is nothing to do.
    //

    if ((_load_config_used.GuardFlags & IMAGE_GUARD_PROTECT_DELAYLOAD_IAT) == 0) {
       return;
    }

    if (__bChangeProtectionOfWholeDloadSection == FALSE) {
        return;
    }

    //
    // Acquire the Dload protection lock for this module and change protection.
    //

    DloadLock();

    DloadSectionLockCount -= 1;
    if (DloadSectionLockCount == 0) {
        DloadProtectSection(nullptr,
                            0,
                            DloadSectionOldProtection,
                            &OldProtect);
    }

    DloadUnlock();

    return;
}

#else

//
// The following dload support APIs are used for the non-explicit-unloading
// version of the delay load helper.
//

DLOAD_INLINE
BOOLEAN
DloadResolve (
    VOID
    )

/*++

Routine Description:

    This function resolves support for native loader-based delay load handling.

Arguments:

    None.

Return Value:

    TRUE is returned as the function value if the host OS supports delay load
    in the loader (in which case all linkage to the host OS native support has
    been initialized).

--*/

{

    HMODULE Kernel32;

    Kernel32 = (HMODULE)ReadPointerAcquire((PVOID *) &DloadKernel32);
    if (Kernel32 == DLOAD_UNSUPPORTED) {
        return FALSE;
    }
    if (Kernel32 != NULL) {
        return TRUE;
    }

    Kernel32 = GetModuleHandleW(L"api-ms-win-core-delayload-l1-1-1.dll");
    if (Kernel32 == NULL) {
        Kernel32 = GetModuleHandleW(L"KERNEL32.DLL");
        if (Kernel32 == NULL) {
            Kernel32 = DLOAD_UNSUPPORTED;
            goto Done;
        }
    }

    DloadResolveDelayLoadedAPI =
        (ResolveDelayLoadedAPIProc)GetProcAddress(Kernel32,
                                                  "ResolveDelayLoadedAPI");

    if (DloadResolveDelayLoadedAPI == NULL) {
        Kernel32 = DLOAD_UNSUPPORTED;
        goto Done;
    }

    DloadResolveDelayLoadsFromDll =
        (ResolveDelayLoadsFromDllProc)GetProcAddress(Kernel32,
                                                     "ResolveDelayLoadsFromDll");

    if (DloadResolveDelayLoadsFromDll == NULL) {
        Kernel32 = DLOAD_UNSUPPORTED;
        goto Done;
    }

Done:
    WritePointerRelease((PVOID *)&DloadKernel32, Kernel32);
    return (Kernel32 != DLOAD_UNSUPPORTED);
}

DLOAD_INLINE
PVOID
WINAPI
Dload__delayLoadHelper2 (
    _In_ PCIMAGE_DELAYLOAD_DESCRIPTOR DelayloadDescriptor,
    _Out_ PIMAGE_THUNK_DATA ThunkAddress,
    _Out_ PBOOLEAN NativeHandled
    )

/*++

Routine Description:

    This function is a thin wrapper around the loader functionality in ntdll to
        resolve a particular delayload thunk from a delayload descriptor.

Arguments:

    DelayloadDescriptor - Supplies a pointer to a structure that describes the
        module to be loaded in order to satisfy the delayed load.

    ThunkAddress - Supplies a pointer to the thunk to be filled in with the
        appropriate target function. This thunk pointer is used to find
        the specific name table entry of the function to be imported.

    NativeHandled - Receives TRUE if the implementation was handled by the
                    native loader, else FALSE if it was not handled by the
                    native loader and the caller must satisfy the request.

Return Value:

    Address of the import, or the failure stub for it.

--*/

{

    PVOID Symbol;

    if (DloadResolve() == FALSE) {
        *NativeHandled = FALSE;
        return NULL;
    }

    *NativeHandled = TRUE;

    Symbol = DloadResolveDelayLoadedAPI((PVOID)&__ImageBase,
                                        DelayloadDescriptor,
                                        (PDELAYLOAD_FAILURE_DLL_CALLBACK)__pfnDliFailureHook2,
                                        DelayLoadFailureHook,
                                        ThunkAddress,
                                        0);

    return Symbol;
}

DLOAD_INLINE
HRESULT
WINAPI
Dload__HrLoadAllImportsForDll (
    _In_ LPCSTR DllName,
    _Out_ PBOOLEAN NativeHandled
    )

/*++

Routine Description:

    This function is a thin wrapper around the loader functionality in ntdll to
         resolve all delayload thunks in the current binary from a target
         module.

Arguments:

    DllName - Supplies the case-insensitive name of the delayloaded target
        module whose imports are to be resolved.

    NativeHandled - Receives TRUE if the implementation was handled by the
                    native loader, else FALSE if it was not handled by the
                    native loader and the caller must satisfy the request.

Return Value:

    HRESULT

--*/

{

    NTSTATUS Status;

    if (DloadResolve() == FALSE) {
        *NativeHandled = FALSE;
        return HRESULT_FROM_WIN32(ERROR_MOD_NOT_FOUND);
    }

    *NativeHandled = TRUE;

    Status = DloadResolveDelayLoadsFromDll((PVOID)&__ImageBase,
                                           DllName,
                                           0);

    if (Status == STATUS_DLL_NOT_FOUND) {
        return HRESULT_FROM_WIN32(ERROR_MOD_NOT_FOUND);

    } else {
        return S_OK;
    }
}

#endif

#pragma warning(pop)

#endif
