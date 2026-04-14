//
//  Microsoft Windows Media Technologies
//  Copyright (C) Microsoft Corporation, 1999 - 2001. All rights reserved.
//

#ifndef _CSECURECHANNELSERVER_H_2AD99357_6FD2_11d3_8497_00C04F79DBC0
#define _CSECURECHANNELSERVER_H_2AD99357_6FD2_11d3_8497_00C04F79DBC0

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "wtypes.h"
#include "sac.h"

class CSecureChannelServer
{
public:
     CSecureChannelServer();
     ~CSecureChannelServer();
     HRESULT SetCertificate(DWORD dwFlags,
                              BYTE *pbAppCert, 
                         DWORD dwCertLen, 
                         BYTE *pbAppPVK,
                         DWORD dwPVKLen);
     HRESULT SACAuth(DWORD dwProtocolID,
                    DWORD dwPass,
                    BYTE *pbDataIn,
                    DWORD dwDataInLen,
                    BYTE **ppbDataOut,
                    DWORD *pdwDataOutLen);
     HRESULT SACGetProtocols(DWORD **ppdwProtocols,
                              DWORD *pdwProtocolCount);
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
     BYTE m_abCallerChallenge[4];
     BYTE m_abIssuedChallenge[4];
     BOOL m_TableInit;
     unsigned long m_OldDesTable[32];
     DWORD m_dwCertFlags;
     MACINFO aMacInfo[20];
     BYTE m_abMacKey[64];
     BOOL m_fMacKeyInit;
     CRITICAL_SECTION m_CS;      
     HRESULT Protocol1(DWORD dwPass,
                    BYTE *pbDataIn,
                    DWORD dwDataInLen,
                    BYTE **ppbDataOut,
                    DWORD *pdwDataOutLen);

     HRESULT Protocol2(DWORD dwPass,
                    BYTE *pbDataIn,
                    DWORD dwDataInLen,
                    BYTE **ppbDataOut,
                    DWORD *pdwDataOutLen);

     BYTE m_DesTable[400];
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _CSECURECHANNELSERVER_H_2AD99357-6FD2-11d3-8497-00C04F79DBC0
