//
//    Copyright (C) Microsoft.  All rights reserved.
//
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

NTSTATUS 
NTAPI
PstGetTrustAnchors(
    _In_                           PUNICODE_STRING                             pTargetName,
    _In_                           DWORD                                       cCriteria,
    _In_reads_opt_(cCriteria)      PCCERT_SELECT_CRITERIA                      rgpCriteria,
    _Outptr_                       PSecPkgContext_IssuerListInfoEx*            ppTrustedIssuers
    );

NTSTATUS 
NTAPI
PstGetTrustAnchorsEx(
    _In_                           PUNICODE_STRING                             pTargetName,
    _In_                           DWORD                                       cCriteria,
    _In_reads_opt_(cCriteria)      PCCERT_SELECT_CRITERIA                      rgpCriteria,
    _In_opt_                       PCCERT_CONTEXT                              pCertContext, 
    _Outptr_                       PSecPkgContext_IssuerListInfoEx*            ppTrustedIssuers
    );

NTSTATUS 
NTAPI
PstGetCertificateChain(
    _In_                                    PCCERT_CONTEXT                      pCert,
    _In_                                    PSecPkgContext_IssuerListInfoEx     pTrustedIssuers,
    _Out_                                   PCCERT_CHAIN_CONTEXT*               ppCertChainContext
    );
    
NTSTATUS 
NTAPI
PstGetCertificates(
    _In_                                    PUNICODE_STRING                     pTargetName,
    _In_                                    DWORD                               cCriteria,
    _In_reads_opt_(cCriteria)               PCCERT_SELECT_CRITERIA              rgpCriteria,
    _In_                                    BOOL                                bIsClient,
    _Out_                                   PDWORD                              pdwCertChainContextCount,
    _Outptr_result_buffer_(*pdwCertChainContextCount) PCCERT_CHAIN_CONTEXT**    ppCertChainContexts
    );
    
NTSTATUS
NTAPI
PstAcquirePrivateKey(
    _In_                                    PCCERT_CONTEXT                      pCert
    );


NTSTATUS 
NTAPI
PstValidate(
    _In_opt_                                PUNICODE_STRING                     pTargetName,
    _In_                                    BOOL                                bIsClient,
    _In_opt_                                CERT_USAGE_MATCH*                   pRequestedIssuancePolicy,
    _In_opt_                                HCERTSTORE*                         phAdditionalCertStore,
    _In_                                    PCCERT_CONTEXT                      pCert,
    _Out_opt_                               GUID*                               pProvGUID
   );

NTSTATUS 
NTAPI
PstMapCertificate(
    _In_                                    PCCERT_CONTEXT                      pCert,
    _Out_                                   LSA_TOKEN_INFORMATION_TYPE*         pTokenInformationType,
    _Outptr_                             PVOID*                              ppTokenInformation
  );

NTSTATUS 
NTAPI
PstGetUserNameForCertificate(
    _In_                                    PCCERT_CONTEXT                      pCertContext,
    _Out_                                   PUNICODE_STRING                     UserName
    );

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

