/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    VmSavedStateDump.h

Abstract:

    This module contains the interface definition for the VmSavedState Dump Provider APIs.

--*/

#ifndef _VMSAVEDSTATEDUMP_H_
#define _VMSAVEDSTATEDUMP_H_

#include "VmSavedStateDumpDefs.h"

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif


/// Locates the saved state file(s) for a given VM and/or snapshot.
/// This function uses WMI and the V1 or V2 virtualization namespace.
/// This is expected to fail if at least one of the following conditions is met:
/// * The system does not have Hyper-V installed.
/// * The system does not support doing WMI queries.
///   * A good indicator is to see if your system has `wmic` CLI available.
/// * The system doesn't have the virtualization WMI namespace installed.
///   * Run in powershell `gwmi -namespace root -class __NAMESPACE | select __RELPATH` to get a list of namespaces.
///     `__NAMESPACE.Name="virtualization"` should be listed.
///
/// Additional notes regarding the lookup behavior of this API:
///  - If the given VM has a VMRS file, parameters binPath and vsvPath will be a single null terminator character.
///  - If the given VM has BIN and VSV files, parameter vmrsPath will be a single null terminator character.
///  - If no saved state files are found, all three returned string parameters will be single null terminator characters.
///
/// \param  vmName        Supplies the VM name for which the saved state file will be located.
/// \param  snapshotName  Supplies an optional snapshot name to locate its saved state file
///                       on relation to the given VM name.
/// \retval binPath       Returns a pointer to a NULL-terminated string containing the full path
///                       name to the BIN file. The caller must call LocalFree on the returned
///                       pointer in order to release the memory occupied by the string.
/// \retval vsvPath       Returns a pointer to a NULL-terminated string containing the full path
///                       name to the VSV file. The caller must call LocalFree on the returned
///                       pointer in order to release the memory occupied by the string.
/// \retval vmrsPath      Returns a pointer to a NULL-terminated string containing the full path
///                       name to the VMRS file. The caller must call LocalFree on the returned
///                       pointer in order to release the memory occupied by the string.
///
/// \return S_OK           The full path(s) to the saved state file were returned successfully.
///         E_OUTOFMEMORY  There was insufficient memory to return the full path(s).
///         HRESULT        Other HRESULT failure codes might be returned.
///
HRESULT
WINAPI
LocateSavedStateFiles(
    LPCWSTR vmName,
    _In_opt_ LPCWSTR snapshotName,
    _Out_ LPWSTR* binPath,
    _Out_ LPWSTR* vsvPath,
    _Out_ LPWSTR* vmrsPath
    );


/// Loads the given saved state file and creates an instance of VmSavedStateDump.
/// This instance can be referenced on the other methods with the returned UINT64 Id.
///
/// \param  vmrsFile                Supplies the path to the VMRS file to load.
/// \retval vmSavedStateDumpHandle  Returns a Handle to the dump provider instance created.
///
/// \return HRESULT.
///
HRESULT
WINAPI
LoadSavedStateFile(
    LPCWSTR vmrsFile,
    _Out_ VM_SAVED_STATE_DUMP_HANDLE* vmSavedStateDumpHandle
    );


/// Opens the given saved state file in read-write exclusive mode so that it applies any pending
/// replay logs to the contents. This method doesn't loads the saved state file into the library
/// and can't be used to get content data; function LoadSavedStateFile must be used instead.
///
/// \param vmrsFile  Supplies the path to the VMRS file whose any pending replay log will be applied.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ApplyPendingSavedStateFileReplayLog(
    LPCWSTR vmrsFile
    );


/// Loads the given saved state files and creates an instance of VmSavedStateDump.
/// This instance can be referenced on the other methods with the returned UINT64 Id.
///
/// \param  binFile                 Supplies the path to the BIN file to load.
/// \param  vsvFile                 Supplies the path to the VSV file to load.
/// \retval vmSavedStateDumpHandle  Returns the ID for the dump provider instance created.
///
/// \return HRESULT.
///
HRESULT
WINAPI
LoadSavedStateFiles(
    LPCWSTR binFile,
    LPCWSTR vsvFile,
    _Out_ VM_SAVED_STATE_DUMP_HANDLE* vmSavedStateDumpHandle
    );


/// Releases the given VmSavedStateDump provider that matches the supplied ID.
/// Releasing the provider releases the locks to the saved state files.
/// This means that it won't be available for use on other methods.
///
/// \param vmSavedStateDumpHandle  Supplies the ID of the dump provider instance to release.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ReleaseSavedStateFiles(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle
    );


//-----------------------------------------------------------------------------------------//
//------ The following methods provide mechanisms to query processor related content ------//
//------       and getting memory from already loaded saved state file(s).           ------//
//-----------------------------------------------------------------------------------------//

/// Queries for the virtual trust levels enabled on the guest's partition at the time the saved state file was generated.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \retval virtualTrustLevels      Returns a bitmap representing enabled virtual trust levels, where lower bit 0
///                                 corresponds to virtual trust level 0, and so on.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetGuestEnabledVirtualTrustLevels(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _Out_ UINT32* virtualTrustLevels
    );


/// Returns the Guest OS information as interpreted when parsing the VM saved partition state blob.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  virtualTrustLevel       Supplies the guest partition VTL to query the OS Id from.
/// \retval guestOsInfo             Returns the Guest OS information.
///
/// \returns HRESULT.
///
HRESULT
WINAPI
GetGuestOsInfo(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT8 virtualTrustLevel,
    _Out_ GUEST_OS_INFO* guestOsInfo
    );


/// Queries for the Virtual Processor count for a given VmSavedStateDump.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \retval vpCount                 Returns the Virtual Processor count.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetVpCount(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _Out_ UINT32* vpCount
    );


/// Queries for the current architecture/ISA the virtual processor was running at the time the
/// saved state file was generated.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to query.
/// \retval architecture            Returns the architecture of the supplied vp.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetArchitecture(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _Out_ VIRTUAL_PROCESSOR_ARCH* architecture
    );


/// Forces the current architecture/ISA of a given virtual processor.
/// This is useful to force architecture specific virtual to physical address translation techniques.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to force the architecure on.
/// \retval architecture            Supplies the architecture to force on the vp.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ForceArchitecture(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    VIRTUAL_PROCESSOR_ARCH architecture
    );


/// Queries for the current Virtual Trust Level the virtual processor was running at the time the
/// saved state file was generated.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to query.
/// \retval virtualTrustLevel       Returns the Virtual Trust Level of the supplied vp.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetActiveVirtualTrustLevel(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _Out_ UINT8* virtualTrustLevel
    );


/// Queries for the virtual trust levels enabled on the virtual processor was running at the time the
/// saved state file was generated.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to query.
/// \retval virtualTrustLevels      Returns a bitmap representing enabled virtual trust levels, where lower bit 0
///                                 corresponds to virtual trust level 0, and so on.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetEnabledVirtualTrustLevels(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    UINT32* virtualTrustLevels
    );


/// Forces the current Virtual Trust Level of a given virtual processor.
/// This is useful to force register state to and virtual address translation
/// to come from a different VTL. Some registers are shared across VTLs, so it is
/// possible that some registers will contain invalid values.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to force the virtual trust level on.
/// \retval virtualTrustLevel       Supplies the virtual trust level to force on the vp.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ForceActiveVirtualTrustLevel(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    UINT8 virtualTrustLevel
    );


/// Queries whether the Virtual Trust Level active on the specific virtual processor is
/// enabled.
/// It should only be possible for this to return false if the VTL was forced on the processor.
/// If the VTL is not enabled, the register state is nonexistent and therefore invalid.
///
/// \param  vmSavedStateDumpHandle          Supplies a handle to a dump provider instance.
/// \param  vpId                            Supplies the VP to query.
/// \retval activeVirtualTrustLevelEnabled  Returns whether the active VTL is enabled.
///
/// \return HRESULT.
///
HRESULT
WINAPI
IsActiveVirtualTrustLevelEnabled(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _Out_ BOOL* activeVirtualTrustLevelEnabled
    );


/// Queries whether nested virtualization is enabled on at least one of the virtual processors.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \retval enabled                 Returns whether nested virtualization is enabled.
///
/// \return HRESULT.
///
HRESULT
WINAPI
IsNestedVirtualizationEnabled(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _Out_ BOOL* enabled
    );


/// Queries whether the virtual processor is currently in nested guest mode.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to query.
/// \retval enabled                 Returns whether the VP is currently in nested guest mode.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetNestedVirtualizationMode(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _Out_ BOOL* enabled
    );


/// Force enables nested "host mode" on a VP (if the VP is currently in nested guest mode).
/// It forces all registers to be read from nested host mode, which would be loaded on the
/// next nested VM-Exit. This can be useful to force virtual address translation to be
/// performed as in nested host mode.
///
/// Only some registers can be queried in host mode. Available registers vary based on
/// processor vendor.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to query.
/// \param  hostMode                Supplies if nested host mode should be force enabled.
/// \retval oldMode                 Optionally returns the old mode.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ForceNestedHostMode(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    BOOL hostMode,
    _Out_opt_ BOOL* oldMode
    );


/// Queries if a given VP is in kernel space.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP to query.
/// \retval inKernelSpace           Returns if the VP is in kernel space.
///
/// \return HRESULT.
///
HRESULT
WINAPI
InKernelSpace(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _Out_ BOOL* inKernelSpace
    );


/// Queries for a specific register value for a given VP in a VmSavedStateDump.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Virtual Processor Id.
/// \param  registerId              Supplies the register ID to query.
/// \retval registerValue           Returns the register value.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetRegisterValue(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    DWORD registerId,
    _Out_ VIRTUAL_PROCESSOR_REGISTER* registerValue
    );


/// Queries for the current Paging Mode in use by the virtual processor at the time the
/// saved state file was generated.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Virtual Processor Id.
/// \retval pagingMode              Returns the paging mode.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetPagingMode(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _Out_ PAGING_MODE* pagingMode
    );


/// Forces the Paging Mode for a given virtual processor.
/// This is useful to force paging mode specific virtual to physical address translation techniques.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Virtual Processor Id.
/// \retval pagingMode              Supplies the paging mode to force on the virtual processor.
///
/// \returns HRESULT.
///
HRESULT
WINAPI
ForcePagingMode(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    PAGING_MODE pagingMode
    );


/// Reads from the saved state file the given guest physical address range and then
/// it is written into the supplied buffer.
/// If BytesRead returns something lower than bufferSize, then the end of memory has been reached.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  physicalAddress         Supplies the physical address to read.
/// \retval buffer                  Returns the read memory range on the given address.
/// \param  bufferSize              Supplies the requested byte count to read.
/// \retval bytesRead               Optionally returns the bytes actually read.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ReadGuestPhysicalAddress(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    GUEST_PHYSICAL_ADDRESS physicalAddress,
    _Out_writes_bytes_(bufferSize) LPVOID buffer,
    UINT32 bufferSize,
    _Out_opt_ UINT32* bytesRead
    );


/// Translates a virtual address to a physical address using information found in the
/// guest's memory and processor's state.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP from where the virtual address is read.
/// \param  virtualAddress          Supplies the virtual address to translate.
/// \retval physicalAddress         Returns the physical address assigned to the supplied virtual address.
/// \retval unmappedRegionSize      In case the given VA is not mapped, optionally returns the size of the
///                                 region that is not mapped.
///
/// \return HRESULT on any error.
///         VM_SAVED_STATE_DUMP_E_VA_NOT_MAPPED - The given virtual address is not mapped to a physical address.
///
HRESULT
WINAPI
GuestVirtualAddressToPhysicalAddress(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    const GUEST_VIRTUAL_ADDRESS virtualAddress,
    _Out_ GUEST_PHYSICAL_ADDRESS* physicalAddress,
    _Out_opt_ GUEST_VIRTUAL_ADDRESS*  unmappedRegionSize
    );

//------------------------------------------------------------------------//
//------ Guest physical memory layout and raw saved memory methods. ------//
//------------------------------------------------------------------------//


/// Returns the layout of the physical memory of the guest. This information contains the chunks of memory
/// with consecutive pages and from where each one starts. If the supplied count is less than the amount
/// of chunks for this guest, then this function returns the expected chunk count.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \retval memoryChunkPageSize     Returns the size of a page in the memory chunk layout.
/// \retval memoryChunks            Supplies a buffer of memory chunk structures that are filled up with the
///                                 requested information if the buffer size is the same or bigger than the
///                                 memory chunks count for this guest.
/// \retval memoryChunkCount        Supplies the size of the memoryChunks buffer. If this count is lower than
///                                 what the guest really has, then it returns the expected count. If it was
///                                 higher than what the guest has, then it returns the exact count.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetGuestPhysicalMemoryChunks(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _Out_ UINT64* memoryChunkPageSize,
    _Out_ GPA_MEMORY_CHUNK* memoryChunks,
    _Inout_ UINT64* memoryChunkCount
    );


/// Translates the given guest physical address to a raw saved memory offset. This is specially useful
/// if callers need to read a memory range directly from all of the guest's saved memory starting
/// in the saved memory address equivalent to the supplied guest physical address.
/// Translation from raw saved memory offset to physical address is not supported.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  physicalAddress         Supplies the guest physical address to translate.
/// \retval rawSavedMemoryOffset    Returns the raw saved memory offset for a given physical address.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GuestPhysicalAddressToRawSavedMemoryOffset(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    GUEST_PHYSICAL_ADDRESS physicalAddress,
    _Out_ UINT64* rawSavedMemoryOffset
    );


/// Reads raw memory from the saved state file. This function reads raw memory from the saved state file
/// as if it were a flat memory layout, regardless of the guest memory layout.
/// If BytesRead returns something lower than bufferSize, then the end of memory has been reached.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  rawSavedMemoryOffset    Byte offset on the raw saved memory from where to start reading.
/// \retval buffer                  Returns the raw memory read on the current raw memory offset.
/// \param  bufferSize              Supplies the requested byte count to read.
/// \retval bytesRead               Optionally returns the bytes actually read.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ReadGuestRawSavedMemory(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT64 rawSavedMemoryOffset,
    _Out_writes_bytes_(bufferSize) LPVOID buffer,
    UINT32 bufferSize,
    _Out_opt_ UINT32* bytesRead
    );


/// Returns the size in bytes of the saved memory for a given VM saved state file.
///
/// \param  vmSavedStateDumpHandle   Supplies a handle to a dump provider instance.
/// \retval guestRawSavedMemorySize  Returns the size of the saved memory of a given guest in bytes.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetGuestRawSavedMemorySize(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _Out_ UINT64* guestRawSavedMemorySize
    );


/// Sets the memory block cache limit for a saved state file. By default this is 0.
/// A VmSavedStateDump provider instance reads from the saved state file in a memory block basis.
/// Setting a memory block cache limit will make the instance cache in-memory blocks read from the
/// file, which is useful for performance.
/// Setting the limit back to 0 discards all cached memory blocks.
///
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param memoryBlockCacheLimit   Supplies the memory block cache limit to set.
///
/// \return HRESULT.
///
HRESULT
WINAPI
SetMemoryBlockCacheLimit(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT64 memoryBlockCacheLimit
    );


/// Returns the memory block cache limit for a saved state file.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \retval memoryBlockCacheLimit   Returns the memory block cache limit.
///
/// \return HRESULT.
///
HRESULT
GetMemoryBlockCacheLimit(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _Out_ UINT64* memoryBlockCacheLimit
    );


/// Applies a memory fix to the supplied virtual address using the provided buffer. The fix buffer is saved
/// internally, and every time a consumer of this class reads virtual/physical addresses all applied fix buffers
/// are returned in the output buffer.
/// Note that these memory fixes are not directly applied to the saved state file(s) of the Virtual Machine.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param vpId                    Supplies the VP from where the virtual address is read.
/// \param virtualAddress          Supplies a virtual address where the fix needs to be applied.
/// \param fixBuffer               Supplies the buffer that will replpace the content in the supplied virtual address.
/// \param fixBufferSize           Supplies the size of the fix buffer.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ApplyGuestMemoryFix(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    GUEST_VIRTUAL_ADDRESS virtualAddress,
    _In_reads_bytes_(FixBufferSize) LPVOID fixBuffer,
    UINT32 fixBufferSize
    );


/// Loads a guest symbols provider for a given dump provider. This simplifies
/// reading global variables and resolving global variable addresses.
/// Uses dbghelp.dll and symsrv.dll implicitly.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param userSymbols             Optionally supplies path(s) to user symbols.
/// \param force                   Determines if the guest symbol provider will be force loaded and replace any loaded one.
///
/// \return HRESULT.
///
HRESULT
WINAPI
LoadSavedStateSymbolProvider(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    _In_opt_  LPCWSTR userSymbols,
    BOOL force
    );


/// Releases a guest symbols provider for a given dump provider.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ReleaseSavedStateSymbolProvider(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle
    );


/// Returns the HANDLE used by the guest symbols provider for a given dump provider.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
///
/// \return HANDLE.
HANDLE
WINAPI
GetSavedStateSymbolProviderHandle(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle
    );


/// Sets the guest symbol provider debug info callback for a given dump provider.
/// It's not necessary to load a symbol provider to call this function; it'll be assigned
/// after it's loaded. If already loaded, then the callback is changed immediatly.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param Callback                Supplies the debug info callback to set.
///
HRESULT
SetSavedStateSymbolProviderDebugInfoCallback(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK Callback
    );


/// Loads a module to the symbols provider assigned to a given dump provider.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param imageName               Supplies the image name of the symbols to load.
/// \param moduleName              Supplies the module from the image to load.
/// \param baseAddress             Supplies the base address of the image.
/// \param sizeOfBase              Supplies the size in bytes of the image.
///
/// \return HRESULT.
///
HRESULT
WINAPI
LoadSavedStateModuleSymbols(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    LPCSTR imageName,
    LPCSTR moduleName,
    GUEST_VIRTUAL_ADDRESS baseAddress,
    DWORD sizeOfBase
    );


/// Loads a module to the symbols provider assigned to a given dump provider.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param imageName               Supplies the image name of the symbols to load.
/// \param imageTimestamp          Supplies the image timestamp to load.
/// \param moduleName              Supplies the module from the image to load.
/// \param baseAddress             Supplies the base address of the image.
/// \param sizeOfBase              Supplies the size in bytes of the image.
///
/// \return HRESULT.
///
HRESULT
WINAPI
LoadSavedStateModuleSymbolsEx(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    LPCSTR imageName,
    UINT32 imageTimestamp,
    LPCSTR moduleName,
    GUEST_VIRTUAL_ADDRESS baseAddress,
    DWORD sizeOfBase
    );


/// Resolves the virtual address of a given global variable.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the virtual processor where the virtual address is related to.
/// \param  globalName              Supplies the global variable name.
/// \retval virtualAddress          Returns the resolved global variable virtual address.
/// \retval size                    Optionally returns the size of the resolved global variable.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ResolveSavedStateGlobalVariableAddress(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    LPCSTR globalName,
    GUEST_VIRTUAL_ADDRESS* virtualAddress,
    _Out_opt_ ULONG* size
    );


/// Returns the HANDLE used by the guest symbols provider for a given dump provider.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
///
/// \return HANDLE.
///
HANDLE
WINAPI
GetSavedStateSymbolProviderHandle(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle
    );


/// Resolves and reads a global variable.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Vp from where the variable is read.
/// \param  globalName              Supplies the name of the global variable to read.
/// \retval buffer                  Supplies a buffer where the guest memory is written.
/// \param  bufferSize              Supplies the byte count to read.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ReadSavedStateGlobalVariable(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    LPCSTR globalName,
    _Out_writes_bytes_(BufferSize) LPVOID buffer,
    ULONG bufferSize
    );


/// Returns the size in bytes of a given type.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Vp from where the variable is read.
/// \param  typeName                Supplies the name of the type.
/// \retval size                    Returns size in bytes of the given Type name. 0 if type was not found.
///
/// \return HRESULT.
///
HRESULT
WINAPI
GetSavedStateSymbolTypeSize(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    LPCSTR typeName,
    ULONG* size
    );


/// Search and return the field linked to the supplied type name.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Vp from where the variable is read.
/// \param  typeName                Supplies the name of the type.
/// \param  fieldName               Supplies the type's field name to find.
/// \retval offset                  Returns the field's offset in the type.
/// \retval found                   Returns if the field was found or not.
///
/// \return HRESULT.
///
HRESULT
WINAPI
FindSavedStateSymbolFieldInType(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    LPCSTR typeName,
    LPCWSTR fieldName,
    UINT32* offset,
    BOOL* found
    );


/// Constructs and returns a type fieldinfo map.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the Vp from where the variable is read.
/// \param  typeName                Supplies the name of the type to query for its field info.
/// \retval typeFieldInfoMap        JSON document of a map containing all the supplied's type field information. Empty map if nothing was found.
///                                 Release string with ::LocalFree.
///
/// \return HRESULT.
///
/// \remarks The returned JSON document in parameter typeFieldInfoMap has the following schema:
/// {
///   "type": "object",
///   "additionalProperties": {
///     "type": "object",
///     "properties": {
///       "FieldOffset": {
///         "type": "integer",
///         "format": "uint32"
///       },
///       "FieldSize": {
///         "type": "integer",
///         "format": "uint64"
///       }
///     }
///   }
/// }
///
HRESULT
WINAPI
GetSavedStateSymbolFieldInfo(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    LPCSTR typeName,
    LPWSTR* typeFieldInfoMap
    );


/// Scans the supplied virtual address range in a given virtual processor.
/// The scan is done on reverse order, where the StartAddress must be higher than
/// the EndAddress. The scan logic will skip page entries when not present.
///
/// \param vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param vpId                    Supplies the virtual processor whose virtual address space is scanned.
/// \param startAddress            Supplies the virtual address where the scan starts from. Must be higher than EndAddress.
/// \param endAddress              Supplies the virtual address where the scan ends. Must be lower than EndAddress.
/// \param callbackContext         Supplies a context that will be included on the callback function.
/// \param foundImageCallback      Supplies the callback for when a valid DOS image is found.
/// \param standaloneAddress       Supplies a list of standalone virtual addresses to check for DOS image validity
///                                before starting the memory scan. If the callback returns true, then this function returns.
/// \param standaloneAddressCount  Supplies the number of standalone addresses to check before starting the scan.
///
/// \return HRESULT.
///
HRESULT
WINAPI
ScanMemoryForDosImages(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    GUEST_VIRTUAL_ADDRESS startAddress,
    GUEST_VIRTUAL_ADDRESS endAddress,
    PVOID callbackContext,
    FOUND_IMAGE_CALLBACK foundImageCallback,
    _In_reads_(StandaloneAddressCount) GUEST_VIRTUAL_ADDRESS* standaloneAddress,
    DWORD standaloneAddressCount
    );


/// Attempts to resolve the call stack for a particular VP.
///
/// \param  vmSavedStateDumpHandle  Supplies a handle to a dump provider instance.
/// \param  vpId                    Supplies the VP whose call stack will be attempted to be unwinded.
/// \param  imageInfo               Supplies a list of DOS Image infos, used to load module symbols.
/// \param  imageInfoCount          Supplies the image info list count.
/// \param  frameCount              Supplies the frame count limit during stack unwinding.
/// \retval callStack               Returns a pointer to a NULL-terminated string containing
///                                 the call stack as printable text. The caller must call LocalFree
///                                 on the returned pointer to release the memory occupied by it.
///
/// \return HRESULT.
///
HRESULT
WINAPI
CallStackUnwind(
    VM_SAVED_STATE_DUMP_HANDLE vmSavedStateDumpHandle,
    UINT32 vpId,
    _In_reads_(ImageInfoCount) PMODULE_INFO imageInfo,
    DWORD imageInfoCount,
    UINT32 frameCount,
    LPWSTR* callStack
    );

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _VMSAVEDSTATEDUMP_H_
