/*++

Copyright (c) Microsoft Corporation

Module Name:

    veinterop_kcm.h

Abstract:

    VTL1 User Bound Key API. Provides APIs for secure user bound key operations
--*/

#ifndef _VEINTEROP_KCM_
#define _VEINTEROP_KCM_

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// Auth Context Handle
DECLARE_HANDLE(USER_BOUND_KEY_AUTH_CONTEXT_HANDLE);

// Session Handle
DECLARE_HANDLE(USER_BOUND_KEY_SESSION_HANDLE);

typedef enum _USER_BOUND_KEY_AUTH_CONTEXT_PROPERTY_NAME {
    UserBoundKeyAuthContextPropertyCacheConfig = 0, 
} USER_BOUND_KEY_AUTH_CONTEXT_PROPERTY_NAME;

// Structure for auth context property validation
typedef struct _USER_BOUND_KEY_AUTH_CONTEXT_PROPERTY
{
    USER_BOUND_KEY_AUTH_CONTEXT_PROPERTY_NAME name;
    UINT32 size;
    _Field_size_bytes_(size) void* value;
} USER_BOUND_KEY_AUTH_CONTEXT_PROPERTY, *PUSER_BOUND_KEY_AUTH_CONTEXT_PROPERTY;

/**
 * Initializes a user bound key session and generates an attestation report.
 *
 * @param challenge Input challenge data to be included in attestation
 * @param challengeSize Size of challenge data in bytes
 * @param report Output pointer to receive encrypted attestation report
 * @param reportSize Output size of encrypted attestation report in bytes
 * @param sessionHandle Output session handle for subsequent operations
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
InitializeUserBoundKeySession(
    _In_reads_bytes_(challengeSize) const void* challenge,
    _In_ UINT32 challengeSize,
    _Outptr_result_buffer_(*reportSize) void** report,
    _Out_ UINT32* reportSize,
    _Out_ USER_BOUND_KEY_SESSION_HANDLE* sessionHandle
);

/**
 * Creates an encrypted KCM request for authorization context retrieval.
 *
 * @param sessionHandle Session handle from InitializeUserBoundKeySession
 * @param keyName Name of the user bound key
 * @param nonce Output nonce value for the request
 * @param encryptedRequest Output pointer to encrypted request data
 * @param encryptedRequestSize Output size of encrypted request in bytes
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
CreateUserBoundKeyRequestForRetrieveAuthorizationContext(
   _Inout_ USER_BOUND_KEY_SESSION_HANDLE sessionHandle,
    _In_ PCWSTR keyName,
    _Out_ UINT64* nonce,
    _Outptr_result_buffer_(*encryptedRequestSize) void** encryptedRequest,
    _Out_ UINT32* encryptedRequestSize
);

/**
 * Decrypts and retrieves the auth context for user bound key operations.
 *
 * @param sessionHandle Session handle from InitializeUserBoundKeySession
 * @param authContextBlob Encrypted auth context blob from KCM
 * @param authContextBlobSize Size of auth context blob in bytes
 * @param nonce Nonce value from the authorization request
 * @param authContextHandle Output handle to the decrypted auth context
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
GetUserBoundKeyAuthContext(
    _In_ USER_BOUND_KEY_SESSION_HANDLE sessionHandle,
    _In_reads_bytes_(authContextBlobSize) const void* authContextBlob,
    _In_ UINT32 authContextBlobSize,
    _In_ UINT64 nonce,
    _Out_ USER_BOUND_KEY_AUTH_CONTEXT_HANDLE* authContextHandle
);

/**
 * Validates the user bound key auth context against specified properties.
 *
 * @param keyName Name of the user bound key to validate
 * @param authContextHandle Handle to the auth context from GetUserBoundKeyAuthContext
 * @param count Number of properties to validate
 * @param values Array of property validation structures
 *
 * @return S_OK on success, E_INVALIDARG for validation failures, or E_ACCESSDENIED for authorization failures
 */
STDAPI
ValidateUserBoundKeyAuthContext(
    _In_ PCWSTR keyName,
    _In_ USER_BOUND_KEY_AUTH_CONTEXT_HANDLE authContextHandle,
    _In_ UINT32 count,
    _In_reads_(count) const USER_BOUND_KEY_AUTH_CONTEXT_PROPERTY* values
);

/**
 * Encrypts and protects a user key using the validated auth context.
 *
 * @param authContext Handle to the validated auth context
 * @param userKey The user key data to protect
 * @param userKeySize Size of the user key data in bytes
 * @param boundKey Output pointer to receive the protected key data
 * @param boundKeySize Input/output size of the bound key buffer
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
ProtectUserBoundKey(
    _In_ USER_BOUND_KEY_AUTH_CONTEXT_HANDLE authContext,
    _In_reads_bytes_(userKeySize) const void* userKey,
    _In_ UINT32 userKeySize,
    _Outptr_result_buffer_(*boundKeySize) void** boundKey,
    _Inout_ UINT32* boundKeySize
);

/**
 * Closes and releases an auth context handle.
 *
 * @param handle The auth context handle to close
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
CloseUserBoundKeyAuthContext(
    _In_ USER_BOUND_KEY_AUTH_CONTEXT_HANDLE handle);
	
/**
 * Closes a user bound key session and releases associated resources.
 *
 * @param sessionHandle The session handle to close
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
CloseUserBoundKeySession(
    _In_ USER_BOUND_KEY_SESSION_HANDLE sessionHandle
);

/**
 * Creates an encrypted KCM request for shared secret derivation.
 *
 * @param sessionHandle Session handle for the operation
 * @param keyName Name of the user bound key
 * @param publicKeyBytes Ephemeral public key data
 * @param publicKeyBytesSize Size of the public key data in bytes
 * @param nonce Output nonce value for the request
 * @param encryptedRequest Output pointer to encrypted request data
 * @param encryptedRequestSize Output size of encrypted request in bytes
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
CreateUserBoundKeyRequestForDeriveSharedSecret(
    _Inout_ USER_BOUND_KEY_SESSION_HANDLE sessionHandle,
    _In_ PCWSTR keyName,
    _In_reads_bytes_(publicKeyBytesSize) const void* publicKeyBytes,
    _In_ UINT32 publicKeyBytesSize,
    _Out_ UINT64* nonce,
    _Outptr_result_buffer_(*encryptedRequestSize) void** encryptedRequest,
    _Out_ UINT32* encryptedRequestSize
);

/**
 * Decrypts and recovers a user key from protected storage.
 *
 * @param sessionHandle Session handle for the operation
 * @param authContext Auth context handle for the key
 * @param sessionEncryptedDerivedSecret Encrypted shared secret from KCM
 * @param sessionEncryptedDerivedSecretSize Size of encrypted secret in bytes
 * @param encryptedUserBoundKey Encrypted user key data
 * @param encryptedUserBoundKeySize Size of encrypted key data in bytes
 * @param nonce Nonce value from the derivation request
 * @param userKey Output pointer to receive the decrypted user key
 * @param userKeySize Input/output size of the user key buffer
 *
 * @return S_OK on success, or an error HRESULT on failure
 */
STDAPI
UnprotectUserBoundKey(
    _In_ USER_BOUND_KEY_SESSION_HANDLE sessionHandle,
    _In_ USER_BOUND_KEY_AUTH_CONTEXT_HANDLE authContext,
    _In_reads_bytes_(sessionEncryptedDerivedSecretSize) const void* sessionEncryptedDerivedSecret,
    _In_ UINT32 sessionEncryptedDerivedSecretSize,
    _In_reads_bytes_(encryptedUserBoundKeySize) const void* encryptedUserBoundKey,
    _In_ UINT32 encryptedUserBoundKeySize,
    _In_ UINT64 nonce,
    _Outptr_result_buffer_(*userKeySize) void** userKey,
    _Inout_ UINT32* userKeySize
);

#ifdef __cplusplus
}
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif 
#endif // _VEINTEROP_KCM_
