/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    winenclaveapi.h

Abstract:

    This module includes definitions of APIs that are exposed to software
    enclaves.

--*/

#ifndef _WINENCLAVEAPI_
#define _WINENCLAVEAPI_

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Attestation APIs.
//

STDAPI
EnclaveGetAttestationReport(
    _In_opt_ const UINT8 EnclaveData[ENCLAVE_REPORT_DATA_LENGTH],
    _Out_writes_bytes_to_opt_(BufferSize, *OutputSize) PVOID Report,
    _In_ UINT32 BufferSize,
    _Out_ UINT32 *OutputSize
    );

STDAPI
EnclaveVerifyAttestationReport(
    _In_ UINT32 EnclaveType,
    _In_reads_bytes_(ReportSize) const VOID *Report,
    _In_ UINT32 ReportSize
    );

//
// Sealing/unsealing APIs.
//

STDAPI
EnclaveSealData(
    _In_reads_bytes_(DataToEncryptSize) const VOID *DataToEncrypt,
    _In_ UINT32 DataToEncryptSize,
    _In_ ENCLAVE_SEALING_IDENTITY_POLICY IdentityPolicy,
    _In_ UINT32 RuntimePolicy,
    _Out_writes_bytes_to_opt_(BufferSize, *ProtectedBlobSize) PVOID ProtectedBlob,
    _In_ UINT32 BufferSize,
    _Out_ UINT32 *ProtectedBlobSize
    );

STDAPI
EnclaveUnsealData(
    _In_reads_bytes_(ProtectedBlobSize) const VOID *ProtectedBlob,
    _In_ UINT32 ProtectedBlobSize,
    _Out_writes_bytes_to_opt_(BufferSize, *DecryptedDataSize) PVOID DecryptedData,
    _In_ UINT32 BufferSize,
    _Out_ UINT32 *DecryptedDataSize,
    _Out_opt_ ENCLAVE_IDENTITY *SealingIdentity,
    _Out_opt_ UINT32 *UnsealingFlags
    );

STDAPI
EnclaveEncryptDataForTrustlet(
    _In_reads_bytes_(DataToEncryptSize) const VOID *DataToEncrypt,
    _In_ UINT32 DataToEncryptSize,
    _In_ PTRUSTLET_BINDING_DATA TrustletBindingData,
    _Out_writes_bytes_to_opt_(BufferSize, *EncryptedDataSize) PVOID EncryptedData,
    _In_ UINT32 BufferSize,
    _Out_ UINT32 *EncryptedDataSize
    );

//
// Enclave properties.
//

STDAPI
EnclaveGetEnclaveInformation(
    _In_ UINT32 InformationSize,
    _Out_writes_bytes_(InformationSize) ENCLAVE_INFORMATION *EnclaveInformation
    );

BOOLEAN
EnclaveUsesAttestedKeys(
    VOID
    );

STDAPI
EnclaveRestrictContainingProcessAccess(
    _In_ BOOL RestrictAccess,
    _Out_opt_ PBOOL PreviouslyRestricted
    );

//
// Routines to copy data in and out of an enclave.
//

STDAPI
EnclaveCopyIntoEnclave(
    _Out_writes_bytes_(NumberOfBytes) VOID *EnclaveAddress,
    _In_reads_bytes_(NumberOfBytes) const VOID *UnsecureAddress,
    _In_ SIZE_T NumberOfBytes
    );

STDAPI
EnclaveCopyOutOfEnclave(
    _Out_writes_bytes_(NumberOfBytes) VOID *UnsecureAddress,
    _In_reads_bytes_(NumberOfBytes) const VOID *EnclaveAddress,
    _In_ SIZE_T NumberOfBytes
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif /* _WINENCLAVEAPI_ */
