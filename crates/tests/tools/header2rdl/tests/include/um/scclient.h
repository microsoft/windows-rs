//
//  Microsoft Windows Media Technologies
//  Copyright (C) Microsoft Corporation, 1999 - 2001. All rights reserved.
//

#ifndef _CSECURECHANNELCLIENT_H_2AD99356_6FD2_11d3_8497_00C04F79DBC0
#define _CSECURECHANNELCLIENT_H_2AD99356_6FD2_11d3_8497_00C04F79DBC0

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "wtypes.h"
#include "sac.h"
#include "mswmdm.h"

class CSecureChannelClient
{
public:
       CSecureChannelClient();
       ~CSecureChannelClient();
       HRESULT SetCertificate(DWORD dwFlags,
		                      BYTE *pbAppCert, 
                              DWORD dwCertLen, 
                              BYTE *pbAppPVK,
                              DWORD dwPVKLen);
       void SetInterface(IComponentAuthenticate *pComponentAuth);
       HRESULT Authenticate(DWORD dwProtocolID);
       HRESULT EncryptParam(BYTE *pbData,
                            DWORD dwDataLen);
       HRESULT DecryptParam(BYTE *pbData,
                           DWORD dwDataLen);
       HRESULT MACInit(HMAC *phMAC);
       HRESULT MACUpdate(HMAC hMAC,
                         BYTE *pbData,
                         DWORD dwDataLen);
       HRESULT MACFinal(HMAC hMAC,
                        BYTE abData[SAC_MAC_LEN]);
       HRESULT GetAppSec(DWORD *pdwLocalAppSec, DWORD *pdwRemoteAppSec);
       HRESULT SetSessionKey(BYTE *pbSPSessionKey);
       HRESULT GetSessionKey(BYTE *pbSPSessionKey);
       HRESULT GetRemoteAppCert(
            _Out_writes_to_opt_(*pdwCertLen, *pdwCertLen) BYTE *pbAppCert,
            _Out_                                           DWORD* pdwCertLen);
       BOOL fIsAuthenticated();
private:
       BOOL m_fAuthenticated;
       BYTE *m_pbAppCert;
       DWORD m_dwCertLen;
       BYTE *m_pbRemoteCert;
       DWORD m_dwRemoteCertLen;
       BYTE *m_pbAppPVK;
       DWORD m_dwPVKLen; 
       BYTE *m_pbSessionKey;
       DWORD m_dwSessionKeyLen;
       BOOL m_TableInit;
       unsigned long m_OldDesTable[32];
       IComponentAuthenticate *m_pAuth;
       DWORD m_dwCertFlags;
       MACINFO aMacInfo[20];
       BYTE m_abMacKey[64];
       BOOL m_fMacKeyInit;
       CRITICAL_SECTION m_CS;
       HRESULT Protocol1();
       HRESULT Protocol2();
       BYTE m_DesTable[400];
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _CSECURECHANNELCLIENT_H_2AD99356-6FD2-11d3-8497-00C04F79DBC0
