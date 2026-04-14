// Copyright Microsoft Corporation.  All rights reserved.

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <dpx1.h>

//
//  DpxNewJob and DpxRestoreJob require a TargetPath which is the local file
//  system directory under which extracted files should be created.  If the
//  directory does not exist, DpxNewJob or DpxRestoreJob will fail.  Files
//  created under TargetPath directory may include relative subdirectory names
//  or even stream names.  Files and subdirectories will be created with
//  inherited ACL from TargetPath and owner from the thread calling
//  IDpxJob::ProvideRequestedData.  During the course of extraction, additional
//  temporary files might be created in the TargetPath directory but will be
//  deleted when the job completes.  If the job is cancelled or destroyed
//  before completing, these temporary files may not be automatically deleted.
//  To move partially completed extraction job to different TargetPath, caller
//  may Suspend and SaveJobState, then tree-copy entire existing TargetPath
//  contents to new location, create a new IDpxJob instance, then DpxRestoreJob
//  using the new TargetPath location.
//
//  DpxNewJobEx and DpxRestoreJobEx are similar to DpxNewJob and DpxRestoreJob,
//  with the only difference being that the user provides a GUID which is used
//  to create the temporary content below the TargetPath as described above.
//  This enables more than one Dpx job to be simultaneouly created and managed
//  with identical TargetPath without interfering with one another.
//

EXTERN_C HRESULT WINAPI DpxNewJob( _In_ LPCWSTR TargetPath, _Outptr_ IDpxJob ** ppJob );

EXTERN_C HRESULT WINAPI DpxNewJobEx( _In_ LPCWSTR TargetPath, _In_opt_ GUID* ClientGuid, _Outptr_ IDpxJob ** ppJob );

EXTERN_C HRESULT WINAPI DpxRestoreJob( _In_ LPCWSTR TargetPath, _Outptr_ IDpxJob ** ppJob );

EXTERN_C HRESULT WINAPI DpxRestoreJobEx( _In_ LPCWSTR TargetPath, _In_opt_ GUID* ClientGuid, _Outptr_ IDpxJob ** ppJob );

EXTERN_C VOID    WINAPI DpxFreeMemory( _In_ void* Allocation );

EXTERN_C HRESULT WINAPI DpxDeleteJob( _In_ LPCWSTR TargetPath );

EXTERN_C HRESULT WINAPI DpxDeleteJobEx( _In_ LPCWSTR TargetPath, _In_opt_ GUID* ClientGuid );

EXTERN_C HRESULT WINAPI DpxCheckJobExists( _In_ LPCWSTR TargetPath, _Out_ BOOL* pbExists );

EXTERN_C HRESULT WINAPI DpxCheckJobExistsEx( _In_ LPCWSTR TargetPath, _In_opt_ GUID* ClientGuid, _Out_ BOOL* pbExists );

#ifdef __cplusplus
enum class DPX_JOB_DISPOSITION
{
    Invalid = 0,
    New = 1,
    Restored = 2,
};
#else
typedef enum
{
    DPX_JOB_DISPOSITION_INVALID = 0,
    DPX_JOB_DISPOSITION_NEW = 1,
    DPX_JOB_DISPOSITION_RESTORED = 2,
} DPX_JOB_DISPOSITION;
#endif

EXTERN_C HRESULT WINAPI DpxRestoreOrNewJob( _In_ LPCWSTR TargetPath, _Outptr_ IDpxJob ** ppJob, _Out_ DPX_JOB_DISPOSITION* pDisposition );

EXTERN_C HRESULT WINAPI DpxRestoreOrNewJobEx( _In_ LPCWSTR TargetPath, _In_opt_ GUID* ClientGuid, _Outptr_ IDpxJob ** ppJob , _Out_ DPX_JOB_DISPOSITION* pDisposition );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

