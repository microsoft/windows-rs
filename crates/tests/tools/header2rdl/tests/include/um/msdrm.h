#pragma once

#ifndef __MSDRM_H_
#define __MSDRM_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//-----------------------------------------------------------------------------
//
// 
// File: msdrm.h
//
// Copyright (C) 2001-2004 Microsoft Corporation.  All Rights Reserved.
//
//-----------------------------------------------------------------------------

#include "msdrmdefs.h"

// environment & handle control

DRMEXPORT HRESULT UDAPICALL DRMSetGlobalOptions(
                    IN DRMGLOBALOPTIONS eGlobalOptions,
                    IN LPVOID pvdata,
                    IN DWORD  dwlen);

DRMEXPORT HRESULT UDAPICALL DRMGetClientVersion(
                    OUT DRM_CLIENT_VERSION_INFO   *pDRMClientVersionInfo);

DRMEXPORT HRESULT UDAPICALL DRMInitEnvironment (
                    IN DRMSECURITYPROVIDERTYPE eSecurityProviderType,
                    IN DRMSPECTYPE eSpecification,
                    _In_opt_ PWSTR wszSecurityProvider,
                    _In_opt_ PWSTR wszManifestCredentials,
                    _In_     PWSTR wszMachineCredentials,
                    OUT DRMENVHANDLE* phEnv,
                    OUT DRMHANDLE* phDefaultLibrary);

DRMEXPORT HRESULT UDAPICALL DRMLoadLibrary (
                    IN DRMENVHANDLE hEnv,
                    IN DRMSPECTYPE eSpecification,
                    _In_     PWSTR wszLibraryProvider,
                    _In_opt_ PWSTR wszCredentials,
                    OUT DRMHANDLE* phLibrary);

DRMEXPORT HRESULT UDAPICALL DRMCreateEnablingPrincipal (
                    IN DRMENVHANDLE hEnv,
                    IN DRMHANDLE hLibrary,
                    _In_ PWSTR wszObject,
                    IN DRMID* pidPrincipal,
                    _In_ PWSTR wszCredentials,
                    OUT DRMHANDLE* phEnablingPrincipal);

DRMEXPORT HRESULT UDAPICALL DRMCloseHandle(
                    IN DRMHANDLE handle);

DRMEXPORT HRESULT UDAPICALL DRMCloseEnvironmentHandle(
                    IN DRMENVHANDLE hEnv);

DRMEXPORT HRESULT UDAPICALL DRMDuplicateHandle(
                    IN DRMHANDLE hToCopy,
                    OUT DRMHANDLE* phCopy);

DRMEXPORT HRESULT UDAPICALL DRMDuplicateEnvironmentHandle(
                    IN DRMENVHANDLE hToCopy,
                    OUT DRMENVHANDLE* phCopy);
 
DRMEXPORT HRESULT UDAPICALL DRMRegisterRevocationList (
                    IN DRMENVHANDLE hEnv,
                    _In_opt_ PWSTR wszRevocationList);

DRMEXPORT HRESULT UDAPICALL DRMCheckSecurity(
                    IN DRMENVHANDLE hEnv,
                    IN UINT cLevel);

DRMEXPORT HRESULT UDAPICALL DRMRegisterContent(
                    IN BOOL fRegister);

// cryptographic functions 

DRMEXPORT HRESULT UDAPICALL DRMEncrypt (
                    IN DRMHANDLE hCryptoProvider,
                    IN UINT iPosition,
                    IN UINT cNumInBytes,
                    IN BYTE* pbInData,
                    IN OUT UINT* pcNumOutBytes,
                    OUT BYTE* pbOutData);

DRMEXPORT HRESULT UDAPICALL DRMDecrypt (
                    IN DRMHANDLE hCryptoProvider,
                    IN UINT iPosition,
                    IN UINT cNumInBytes,
                    IN BYTE* pbInData,
                    IN OUT UINT* pcNumOutBytes,
                    OUT BYTE* pbOutData);

// license binding and enabling bits

DRMEXPORT HRESULT UDAPICALL DRMCreateBoundLicense (
                    IN DRMENVHANDLE hEnv,
                    IN DRMBOUNDLICENSEPARAMS* pParams,
                    _In_ PWSTR wszLicenseChain,
                    OUT DRMHANDLE* phBoundLicense,
                    OUT DRMHANDLE* phErrorLog);

DRMEXPORT HRESULT UDAPICALL DRMCreateEnablingBitsDecryptor (
                    IN DRMHANDLE hBoundLicense,
                    _In_opt_ PWSTR wszRight,
                    IN DRMHANDLE hAuxLib,
                    _In_opt_ PWSTR wszAuxPlug,
                    OUT DRMHANDLE* phDecryptor);

DRMEXPORT HRESULT UDAPICALL DRMCreateEnablingBitsEncryptor (
                    IN DRMHANDLE hBoundLicense,
                    _In_opt_ PWSTR wszRight,
                    IN DRMHANDLE hAuxLib,
                    _In_opt_ PWSTR wszAuxPlug,
                    OUT DRMHANDLE* phEncryptor);

// inter-environment security attestation

DRMEXPORT HRESULT UDAPICALL DRMAttest (
                    IN DRMHANDLE hEnablingPrincipal,
                    _In_ PWSTR wszData,
                    IN DRMATTESTTYPE eType,
                    _Inout_ UINT* pcAttestedBlob,
                    _Out_writes_(*pcAttestedBlob) PWSTR wszAttestedBlob);


// miscellaneous calls and helper functions

DRMEXPORT HRESULT UDAPICALL DRMGetTime (
                    IN DRMENVHANDLE hEnv,
                    IN DRMTIMETYPE eTimerIdType,
                    OUT SYSTEMTIME* poTimeObject);

DRMEXPORT HRESULT UDAPICALL DRMGetInfo(
                    _In_ DRMHANDLE handle,
                    _In_ PWSTR wszAttribute,
                    _In_ DRMENCODINGTYPE* peEncoding,
                    _Inout_ UINT* pcBuffer,
                    OUT BYTE* pbBuffer);

DRMEXPORT HRESULT UDAPICALL DRMGetEnvironmentInfo(
                    IN DRMENVHANDLE handle,
                    _In_opt_ PWSTR wszAttribute,
                    OUT DRMENCODINGTYPE* peEncoding,
                    IN OUT UINT* pcBuffer,
                    OUT BYTE* pbBuffer);

DRMEXPORT HRESULT UDAPICALL DRMGetProcAddress(
                    IN DRMHANDLE hLibrary,
                    _In_ PWSTR wszProcName,
                    OUT FARPROC* ppfnProcAddress);

// support for querying bound licenses

DRMEXPORT HRESULT UDAPICALL DRMGetBoundLicenseObjectCount(
                    IN DRMHANDLE hQueryRoot,
                    _In_ PWSTR wszSubObjectType,
                    OUT UINT* pcSubObjects);

DRMEXPORT HRESULT UDAPICALL DRMGetBoundLicenseObject(
                    IN DRMHANDLE hQueryRoot,
                    _In_ PWSTR wszSubObjectType,
                    IN UINT iWhich,
                    OUT DRMHANDLE* phSubObject);

DRMEXPORT HRESULT UDAPICALL DRMGetBoundLicenseAttributeCount(
                    IN DRMHANDLE hQueryRoot,
                    _In_ PWSTR wszAttribute,
                    OUT UINT* pcAttributes);

DRMEXPORT HRESULT UDAPICALL DRMGetBoundLicenseAttribute(
                    IN DRMHANDLE hQueryRoot,
                    _In_ PWSTR wszAttribute,
                    IN UINT iWhich,
                    OUT DRMENCODINGTYPE* peEncoding,
                    IN OUT UINT* pcBuffer,
                    OUT BYTE* pbBuffer);



DRMEXPORT HRESULT UDAPICALL DRMCreateClientSession(
                        IN  DRMCALLBACK  pfnCallback,            // Callback 
                        IN  UINT         uCallbackVersion,        // Version of the callback
                        _In_     PWSTR        wszGroupIDProviderType, // one of WINDOWSAUTH/PASSPORT
                        _In_opt_ PWSTR        wszGroupID,             // Group ID
                        OUT DRMHSESSION* phClient );


DRMEXPORT HRESULT UDAPICALL DRMIsActivated(
                        IN  DRMHSESSION       hClient,      // Client session
                        IN  UINT              uFlags,       // One of DRM_ACTIVATE_MACHINE/GROUPIDENTITY
                        IN  DRM_ACTSERV_INFO* pActServInfo);// Optional activation server info


DRMEXPORT HRESULT UDAPICALL DRMActivate(
                        IN DRMHSESSION    hClient,            // Client session
                        IN UINT           uFlags,             // One/both: DRM_ACTIVATE_MACHINE/GROUPIDENTITY
                        IN UINT           uLangID,            // Language ID
                        IN DRM_ACTSERV_INFO* pActServInfo,  // Optional activation server info
                        IN VOID*          pvContext,          // Context used for callback
                        IN HWND           hParentWnd);        // Parent window Handle


DRMEXPORT HRESULT UDAPICALL DRMGetServiceLocation(
                        IN    DRMHSESSION    hClient,            // Client session
                        IN    UINT           uServiceType,       //One of DRM_SERVICE_TYPE
                        IN    UINT           uServiceLocation,   //One of DRM_SERVICE_LOCATION
                        _In_opt_    PWSTR          wszIssuanceLicense, //Optional
                        IN OUT UINT*         puServiceURLLength,
                        _Out_writes_opt_(*puServiceURLLength)   PWSTR          wszServiceURL);



//    LicenseStorage
DRMEXPORT HRESULT UDAPICALL DRMCreateLicenseStorageSession(
                        IN  DRMENVHANDLE hEnv,               // Environment Handle ( o/p of DRMInitEnvironment)
                        IN  DRMHANDLE    hDefaultLibrary,    // Default Library Handle (o/p of DRMInitEnvironment)
                        IN  DRMHSESSION  hClient,            // Client session
                        IN  UINT         uFlags,             // Reserved
                        _In_  PWSTR        wszIssuanceLicense, // IssuanceLicense
                        OUT DRMHSESSION* phLicenseStorage);

DRMEXPORT HRESULT UDAPICALL DRMAddLicense(
                        IN DRMHSESSION  hLicenseStorage,    // LicenseStorage session
                        IN UINT         uFlags,             // One of DRM_ADD_LICENSE_FLAGS
                        _In_ PWSTR        wszLicense);        // License to add to the inmemory license store

DRMEXPORT HRESULT UDAPICALL DRMAcquireAdvisories(
                        IN    DRMHSESSION   hLicenseStorage,
                        _In_    PWSTR         wszLicense,
                        _In_opt_    PWSTR         wszURL,//Optional
                        IN    VOID*         pvContext);



DRMEXPORT HRESULT UDAPICALL DRMEnumerateLicense(
                        IN    DRMHSESSION  hSession,                   // Client/License Storage session
                        IN    UINT         uFlags,                     // One of DRM_ENUMERATE_LICENSE_FLAGS
                        IN    UINT         uIndex,                     // Cert index
                        IN OUT BOOL*       pfSharedFlag,               // Shared Flag
                        IN OUT UINT*       puCertificateDataLen,       // String Length of wszCertificateData buffer 
                        _Out_writes_opt_(*puCertificateDataLen)   PWSTR        wszCertificateData );       // CertChain 

DRMEXPORT HRESULT UDAPICALL DRMAcquireLicense(
                        IN    DRMHSESSION  hSession,           //Client/License Storage session
                        IN    UINT         uFlags,             //DRM_ACQUIRE_LICENSE_FLAGS
                        _In_opt_    PWSTR        wszGroupIdentityCredential,// Optional
                        _In_opt_    PWSTR        wszRequestedRights, //RESERVED, must be NULL.
                        _In_opt_    PWSTR        wszCustomData,      //Custom data that goes with the request
                        _In_opt_    PWSTR        wszURL,             //
                        IN    VOID*        pvContext );

DRMEXPORT HRESULT UDAPICALL DRMDeleteLicense( 
                        IN DRMHSESSION  hSession,           //Client/License Storage session
                        _In_ PWSTR        wszLicenseId);


DRMEXPORT HRESULT UDAPICALL DRMCloseSession(
                        IN DRMHSESSION hSession);

DRMEXPORT HRESULT UDAPICALL DRMDuplicateSession(
                        IN  DRMHSESSION hSessionIn, 
                        OUT DRMHSESSION *phSessionOut);

DRMEXPORT HRESULT UDAPICALL DRMGetSecurityProvider(
                        IN    UINT      uFlags,
                        IN OUT UINT*    puTypeLen,
                        _Out_writes_opt_(*puTypeLen)   PWSTR     wszType,
                        IN OUT UINT*    puPathLen,
                        _Out_writes_opt_(*puPathLen)   PWSTR     wszPath);

DRMEXPORT HRESULT UDAPICALL DRMEncode(
                        _In_ PWSTR    wszAlgID,
                        IN UINT     uDataLen,
                        IN BYTE*    pbDecodedData,
                        IN OUT UINT* puEncodedStringLen,
                        _Out_writes_opt_(*puEncodedStringLen) PWSTR   wszEncodedString);

DRMEXPORT HRESULT UDAPICALL DRMDecode(
                        _In_ PWSTR    wszAlgID,
                        _In_ PWSTR    wszEncodedString,
                        IN OUT UINT* puDecodedDataLen,
                        OUT BYTE*   pbDecodedData);

DRMEXPORT HRESULT UDAPICALL DRMConstructCertificateChain(
                        IN      UINT cCertificates,
                        _In_reads_(cCertificates)      PWSTR* rgwszCertificates,
                        IN OUT   UINT* pcChain,
                        _Out_writes_opt_(*pcChain)     PWSTR wszChain);


// support for unbound license querying

DRMEXPORT HRESULT UDAPICALL DRMParseUnboundLicense(
                        _In_      PWSTR wszCertificate,
                        OUT     DRMQUERYHANDLE* phQueryRoot);

DRMEXPORT HRESULT UDAPICALL DRMCloseQueryHandle(
                        IN      DRMQUERYHANDLE hQuery);

DRMEXPORT HRESULT UDAPICALL DRMGetUnboundLicenseObjectCount(
                        IN      DRMQUERYHANDLE hQueryRoot,
                        _In_      PWSTR wszSubObjectType,
                        OUT     UINT* pcSubObjects);

DRMEXPORT HRESULT UDAPICALL DRMGetUnboundLicenseObject(
                        IN      DRMQUERYHANDLE hQueryRoot,
                        _In_      PWSTR wszSubObjectType,
                        IN      UINT iIndex,
                        OUT     DRMQUERYHANDLE* phSubQuery);

DRMEXPORT HRESULT UDAPICALL DRMGetUnboundLicenseAttributeCount(
                        IN      DRMQUERYHANDLE hQueryRoot,
                        _In_      PWSTR wszAttributeType,
                        OUT     UINT* pcAttributes);

DRMEXPORT HRESULT UDAPICALL DRMGetUnboundLicenseAttribute(
                        IN      DRMQUERYHANDLE hQueryRoot,
                        _In_      PWSTR wszAttributeType,
                        IN      UINT iWhich,
                        OUT     DRMENCODINGTYPE* peEncoding,
                        IN OUT   UINT* pcBuffer,
                        OUT     BYTE* pbBuffer);

DRMEXPORT HRESULT UDAPICALL DRMGetCertificateChainCount(
                        _In_      PWSTR wszChain,
                        OUT     UINT* pcCertCount);

DRMEXPORT HRESULT UDAPICALL DRMDeconstructCertificateChain(
                        _In_      PWSTR wszChain,
                        IN      UINT iWhich,
                        IN OUT   UINT* pcCert,
                        _Out_writes_opt_(*pcCert)     PWSTR wszCert);

DRMEXPORT HRESULT UDAPICALL DRMVerify (
                    _In_opt_ PWSTR wszData,
                    UINT* pcAttestedData,
                    _Out_writes_opt_(*pcAttestedData) PWSTR wszAttestedData,
                    DRMATTESTTYPE* peType,
                    UINT* pcPrincipal,
                    _Out_writes_opt_(*pcPrincipal) PWSTR wszPrincipal,
                    UINT* pcManifest,
                    _Out_writes_opt_(*pcManifest) PWSTR wszManifest);




DRMEXPORT HRESULT UDAPICALL DRMCreateUser(
                                _In_opt_  PWSTR           wszUserName,
                                _In_opt_  PWSTR           wszUserId,
                                _In_opt_  PWSTR           wszUserIdType,
                                OUT DRMPUBHANDLE*   phUser);


DRMEXPORT HRESULT UDAPICALL DRMCreateRight(
                                _In_ PWSTR           wszRightName,
                                IN  SYSTEMTIME*     pstFrom,
                                IN  SYSTEMTIME*     pstUntil,
                                IN  UINT            cExtendedInfo,
                                _In_reads_opt_(cExtendedInfo) PWSTR*          pwszExtendedInfoName,
                                _In_reads_opt_(cExtendedInfo) PWSTR*          pwszExtendedInfoValue,
                                OUT DRMPUBHANDLE    *phRight);

DRMEXPORT HRESULT UDAPICALL DRMCreateIssuanceLicense(
                                IN  SYSTEMTIME*     pstTimeFrom,
                                IN  SYSTEMTIME*     pstTimeUntil,
                                _In_opt_  PWSTR           wszReferralInfoName,
                                _In_opt_  PWSTR           wszReferralInfoURL,
                                IN  DRMPUBHANDLE    hOwner,//Use created using DRMCreateUser.

                                _In_opt_  PWSTR           wszIssuanceLicense,//Issuance License Template or Signed Issuance License
                                IN  DRMHANDLE       hBoundLicense,
                                OUT DRMPUBHANDLE*   phIssuanceLicense);

    
DRMEXPORT HRESULT UDAPICALL DRMAddRightWithUser(
                                IN  DRMPUBHANDLE     hIssuanceLicense,
                                IN  DRMPUBHANDLE     hRight,
                                IN  DRMPUBHANDLE     hUser);

DRMEXPORT HRESULT UDAPICALL DRMClearAllRights(
                                IN  DRMPUBHANDLE     hIssuanceLicense);


DRMEXPORT HRESULT UDAPICALL DRMSetMetaData(
                                IN  DRMPUBHANDLE    hIssuanceLicense,
                                _In_  PWSTR           wszContentId,
                                _In_  PWSTR           wszContentIdType,
                                _In_opt_  PWSTR           wszSKUId,
                                _In_opt_  PWSTR           wszSKUIdType,
                                _In_opt_  PWSTR           wszContentType,
                                _In_opt_  PWSTR           wszContentName);

DRMEXPORT HRESULT UDAPICALL DRMSetUsagePolicy(
                                IN  DRMPUBHANDLE    hIssuanceLicense,
                                IN  DRM_USAGEPOLICY_TYPE eUsagePolicyType,
                                IN  BOOL            fDelete,
                                IN  BOOL            fExclusion,

                                _In_opt_  PWSTR           wszName,//Set usage policy by Name
                                _In_opt_  PWSTR           wszMinVersion,
                                _In_opt_  PWSTR           wszMaxVersion,

                                _In_opt_  PWSTR           wszPublicKey,//Set usage policy by public key

                                _In_opt_  PWSTR           wszDigestAlgorithm,//Set usage policy by digest
                                IN  BYTE*           pbDigest,
                                IN  UINT            cbDigest);

DRMEXPORT HRESULT UDAPICALL DRMSetRevocationPoint(
                                IN  DRMPUBHANDLE    hIssuanceLicense,
                                IN  BOOL            fDelete,
                                _In_  PWSTR           wszId,
                                _In_  PWSTR           wszIdType,
                                _In_  PWSTR           wszURL,
                                IN  SYSTEMTIME*     pstFrequency,
                                _In_opt_  PWSTR           wszName,//Optional
                                _In_opt_  PWSTR           wszPublicKey);

DRMEXPORT HRESULT UDAPICALL DRMSetApplicationSpecificData(
                                IN  DRMPUBHANDLE    hIssuanceLicense,
                                IN  BOOL            fDelete,
                                _In_opt_  PWSTR           wszName,
                                _In_opt_  PWSTR           wszValue);

DRMEXPORT HRESULT UDAPICALL DRMSetNameAndDescription(
                                IN   DRMPUBHANDLE    hIssuanceLicense,
                                IN   BOOL            fDelete,
                                IN   UINT            lcid,
                                _In_opt_   PWSTR           wszName,
                                _In_opt_   PWSTR           wszDescription);

DRMEXPORT HRESULT UDAPICALL DRMSetIntervalTime(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN      UINT            cDays);


DRMEXPORT HRESULT UDAPICALL DRMGetIssuanceLicenseTemplate(
                                IN      DRMPUBHANDLE hIssuanceLicense,
                                IN OUT   UINT*       puIssuanceLicenseTemplateLength,
                                _Out_writes_opt_(*puIssuanceLicenseTemplateLength)     PWSTR        wszIssuanceLicenseTemplate);


DRMEXPORT HRESULT UDAPICALL DRMGetSignedIssuanceLicense(
                                IN      DRMENVHANDLE hEnv,//Optional.Mandatory for DRM_SIGN_OFFLINE
                                IN      DRMPUBHANDLE hIssuanceLicense,
                                IN      UINT         uFlags,//DRM_SIGN_ONLINE/DRM_SIGN_OFFLINE/DRM_SIGN_CANCEL
                                
                                IN      BYTE*        pbSymKey,
                                IN      UINT         cbSymKey,
                                _In_opt_      PWSTR        wszSymKeyType,
                                
                                _In_opt_      PWSTR        wszClientLicensorCertificate,//Should be NULL for DRM_SIGN_ONLINE , not NULL otherwise
                                IN      DRMCALLBACK  pfnCallback,
                                _In_opt_      PWSTR        wszURL,//Mandatory if uFlags is DRM_SIGN_ONLINE

                                IN      VOID*        pvContext);//Optional

DRMEXPORT HRESULT UDAPICALL DRMGetSignedIssuanceLicenseEx(
                                DRMENVHANDLE                hEnv,
                                DRMPUBHANDLE                hIssuanceLicense,
                                UINT                        uFlags,
    _In_reads_bytes_opt_(cbSymKey)   BYTE*                       pbSymKey,
                                UINT                        cbSymKey,
            _In_opt_            PWSTR                       wszSymKeyType,                                
            _In_opt_            PVOID                       pvReserved,                                
            _In_                DRMHANDLE                   hEnablingPrincipal,
            _In_                DRMHANDLE                   hBoundLicenseCLC,
            _In_                DRMCALLBACK                 pfnCallback,
            _In_                PVOID                       pvContext);

DRMEXPORT HRESULT UDAPICALL DRMClosePubHandle(
                                IN      DRMPUBHANDLE hPub);

DRMEXPORT HRESULT UDAPICALL DRMDuplicatePubHandle(
                                IN      DRMPUBHANDLE  hPubIn,
                                OUT     DRMPUBHANDLE* phPubOut);


DRMEXPORT HRESULT UDAPICALL DRMGetUserInfo(  
                                 IN     DRMPUBHANDLE    hUser,
                                 IN OUT UINT*           puUserNameLength,
                                 _Out_writes_opt_(*puUserNameLength) PWSTR           wszUserName,
                                 IN OUT UINT*           puUserIdLength,
                                 _Out_writes_opt_(*puUserIdLength) PWSTR           wszUserId,
                                 IN OUT UINT*           puUserIdTypeLength,
                                 _Out_writes_opt_(*puUserIdTypeLength) PWSTR           wszUserIdType);

DRMEXPORT HRESULT UDAPICALL DRMGetRightInfo(  
                                 IN    DRMPUBHANDLE     hRight,
                                 IN OUT UINT*           puRightNameLength,
                                 _Out_writes_opt_(*puRightNameLength)   PWSTR            wszRightName,
                                 OUT   SYSTEMTIME*      pstFrom,
                                 OUT   SYSTEMTIME*      pstUntil);

DRMEXPORT HRESULT UDAPICALL DRMGetRightExtendedInfo(  
                                 IN     DRMPUBHANDLE    hRight,
                                 IN     UINT            uIndex,
                                 IN OUT  UINT*          puExtendedInfoNameLength,
                                 _Out_writes_opt_(*puExtendedInfoNameLength)    PWSTR           wszExtendedInfoName,
                                 IN OUT  UINT*          puExtendedInfoValueLength,
                                 _Out_writes_opt_(*puExtendedInfoValueLength)    PWSTR           wszExtendedInfoValue);

DRMEXPORT HRESULT UDAPICALL DRMGetUsers(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN      UINT            uIndex,
                                OUT     DRMPUBHANDLE*   phUser);

DRMEXPORT HRESULT UDAPICALL DRMGetUserRights(  
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN      DRMPUBHANDLE    hUser,
                                IN      UINT            uIndex,
                                OUT     DRMPUBHANDLE*   phRight);

DRMEXPORT HRESULT UDAPICALL DRMGetMetaData(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN OUT  UINT*           puContentIdLength,
                                _Out_writes_opt_(*puContentIdLength)      PWSTR           wszContentId,
                                IN OUT  UINT*           puContentIdTypeLength,
                                _Out_writes_opt_(*puContentIdTypeLength)  PWSTR           wszContentIdType,
                                IN OUT  UINT*           puSKUIdLength,
                                _Out_writes_opt_(*puSKUIdLength)          PWSTR           wszSKUId,
                                IN OUT  UINT*           puSKUIdTypeLength,
                                _Out_writes_opt_(*puSKUIdTypeLength)      PWSTR           wszSKUIdType,
                                IN OUT  UINT*           puContentTypeLength,
                                _Out_writes_opt_(*puContentTypeLength)    PWSTR           wszContentType,
                                IN OUT  UINT*           puContentNameLength,
                                _Out_writes_opt_(*puContentNameLength)    PWSTR           wszContentName);

DRMEXPORT HRESULT UDAPICALL DRMGetApplicationSpecificData(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN      UINT            uIndex,
                                IN OUT   UINT*          puNameLength,
                                _Out_writes_opt_(*puNameLength)     PWSTR   wszName,
                                IN OUT   UINT*          puValueLength,
                                _Out_writes_opt_(*puValueLength)    PWSTR   wszValue);



DRMEXPORT HRESULT UDAPICALL DRMGetIssuanceLicenseInfo(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                OUT     SYSTEMTIME*     pstTimeFrom,
                                OUT     SYSTEMTIME*     pstTimeUntil,
                                IN      UINT            uFlags,
                                IN OUT  UINT*           puDistributionPointNameLength,
                                _Out_writes_opt_(*puDistributionPointNameLength)    PWSTR   wszDistributionPointName,
                                IN OUT  UINT*           puDistributionPointURLLength,
                                _Out_writes_opt_(*puDistributionPointURLLength)     PWSTR   wszDistributionPointURL,
                                OUT     DRMPUBHANDLE*   phOwner,
                                OUT     BOOL*           pfOfficial);


DRMEXPORT HRESULT UDAPICALL DRMGetRevocationPoint(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN OUT  UINT*           puIdLength,
                                _Out_writes_opt_(*puIdLength)      PWSTR      wszId,
                                IN OUT  UINT*           puIdTypeLength,
                                _Out_writes_opt_(*puIdTypeLength)  PWSTR      wszIdType,
                                IN OUT  UINT*           puURLLength,
                                _Out_writes_opt_(*puURLLength)     PWSTR      wszRL,
                                OUT     SYSTEMTIME*     pstFrequency,
                                IN OUT  UINT*           puNameLength,
                                _Out_writes_opt_(*puNameLength)    PWSTR      wszName,
                                IN OUT  UINT*           puPublicKeyLength,
                                _Out_writes_opt_(*puPublicKeyLength) PWSTR    wszPublicKey);

DRMEXPORT HRESULT UDAPICALL DRMGetUsagePolicy(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN      UINT            uIndex,
                                OUT     DRM_USAGEPOLICY_TYPE* peUsagePolicyType,
                                OUT     BOOL*           pfExclusion,
                                IN OUT  UINT*           puNameLength,
                                _Out_writes_opt_(*puNameLength)       PWSTR           wszName,
                                IN OUT  UINT*           puMinVersionLength,
                                _Out_writes_opt_(*puMinVersionLength) PWSTR           wszMinVersion,
                                IN OUT  UINT*           puMaxVersionLength,
                                _Out_writes_opt_(*puMaxVersionLength) PWSTR           wszMaxVersion,
                                IN OUT  UINT*           puPublicKeyLength,
                                _Out_writes_opt_(*puPublicKeyLength)  PWSTR           wszPublicKey,
                                IN OUT  UINT*           puDigestAlgorithmLength,
                                _Out_writes_opt_(*puDigestAlgorithmLength)  PWSTR     wszDigestAlgorithm,
                                IN OUT  UINT*           pcbDigest,
                                OUT     BYTE*           pbDigest);

DRMEXPORT HRESULT UDAPICALL DRMGetNameAndDescription(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN      UINT            uIndex,
                                OUT     UINT*           pulcid,
                                IN OUT   UINT*          puNameLength,
                                _Out_writes_opt_(*puNameLength)     PWSTR           wszName,
                                IN OUT   UINT*          puDescriptionLength,
                                _Out_writes_opt_(*puDescriptionLength)     PWSTR           wszDescription);

DRMEXPORT HRESULT UDAPICALL DRMGetOwnerLicense(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                IN OUT  UINT*           puOwnerLicenseLength,
                                _Out_writes_opt_(*puOwnerLicenseLength)     PWSTR           wszOwnerLicense);

DRMEXPORT HRESULT UDAPICALL DRMGetIntervalTime(
                                IN      DRMPUBHANDLE    hIssuanceLicense,
                                OUT     UINT*           pcDays);


DRMEXPORT HRESULT UDAPICALL DRMRepair();

DRMEXPORT HRESULT UDAPICALL DRMRegisterProtectedWindow(
								IN		DRMENVHANDLE hEnv,
								IN		HWND hwnd);

DRMEXPORT HRESULT UDAPICALL DRMIsWindowProtected(
								IN		HWND	hwnd,
								OUT		BOOL*	pfProtected);

DRMEXPORT HRESULT UDAPICALL DRMAcquireIssuanceLicenseTemplate(
								IN		DRMHSESSION	hClient,
								IN		UINT uFlags,
								IN		VOID* pvReserved,
								IN		UINT cTemplates,
								_In_reads_opt_(cTemplates) PWSTR* pwszTemplateIds,
								_In_ PWSTR wszUrl,
								IN		VOID* pvContext);



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // #ifndef __MSDRM_H_
