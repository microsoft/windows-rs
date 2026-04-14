//+----------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:       cryptuiapi.h
//
//  Contents:   Cryptographic UI API Prototypes and Definitions
//
//-----------------------------------------------------------------------------

#ifndef __CRYPTUIAPI_H__
#define __CRYPTUIAPI_H__

#if defined (_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <wintrust.h>
#include <wincrypt.h>
#include <prsht.h>

#ifdef __cplusplus
extern "C" {
#endif

#include <pshpack8.h>

#define CERT_CREDENTIAL_PROVIDER_ID            -509

//+----------------------------------------------------------------------------
//  Dialog viewer of a certificate, CTL or CRL context.
//
//  dwContextType and associated pvContext's
//      CERT_STORE_CERTIFICATE_CONTEXT  PCCERT_CONTEXT
//      CERT_STORE_CRL_CONTEXT          PCCRL_CONTEXT
//      CERT_STORE_CTL_CONTEXT          PCCTL_CONTEXT
//
//  dwFlags currently isn't used and should be set to 0.
//-----------------------------------------------------------------------------
_Success_(return == TRUE)
BOOL
WINAPI
CryptUIDlgViewContext(
    _In_        DWORD dwContextType,
    _In_        const void *pvContext,
    _In_opt_    HWND hwnd,              // Defaults to the desktop window
    _In_opt_    LPCWSTR pwszTitle,      // Defaults to the context type title
    _In_        DWORD dwFlags,
    _In_        void *pvReserved
    );


//+----------------------------------------------------------------------------
//  Dialog to select a certificate from the specified store.
//
//  Returns the selected certificate context. If no certificate was
//  selected, NULL is returned.
//
//  pwszTitle is either NULL or the title to be used for the dialog.
//  If NULL, the default title is used.  The default title is
//  "Select Certificate".
//
//  pwszDisplayString is either NULL or the text statement in the selection
//  dialog.  If NULL, the default phrase
//  "Select a certificate you wish to use" is used in the dialog.
//
//  dwDontUseColumn can be set to exclude columns from the selection
//  dialog. See the CRYPTDLG_SELECTCERT_*_COLUMN definitions below.
//
//  dwFlags currently isn't used and should be set to 0.
//-----------------------------------------------------------------------------
PCCERT_CONTEXT
WINAPI
CryptUIDlgSelectCertificateFromStore(
    _In_     HCERTSTORE hCertStore,
    _In_opt_ HWND hwnd,              // Defaults to the desktop window
    _In_opt_ LPCWSTR pwszTitle,
    _In_opt_ LPCWSTR pwszDisplayString,
    _In_     DWORD dwDontUseColumn,
    _In_     DWORD dwFlags,
    _In_     void *pvReserved
    );

// flags for dwDontUseColumn
#define CRYPTUI_SELECT_ISSUEDTO_COLUMN                   0x000000001
#define CRYPTUI_SELECT_ISSUEDBY_COLUMN                   0x000000002
#define CRYPTUI_SELECT_INTENDEDUSE_COLUMN                0x000000004
#define CRYPTUI_SELECT_FRIENDLYNAME_COLUMN               0x000000008
#define CRYPTUI_SELECT_LOCATION_COLUMN                   0x000000010
#define CRYPTUI_SELECT_EXPIRATION_COLUMN                 0x000000020

//+----------------------------------------------------------------------------
//
// The select cert dialog can be passed a filter proc to reduce the set of
// certificates displayed.  Return TRUE to display the certificate and FALSE to
// hide it.  If TRUE is returned then optionally the pfInitialSelectedCert
// boolean may be set to TRUE to indicate to the dialog that this cert should
// be the initially selected cert.  Note that the most recent cert that had the
// pfInitialSelectedCert boolean set during the callback will be the initially
// selected cert.
//
//-----------------------------------------------------------------------------
typedef BOOL (WINAPI * PFNCFILTERPROC) (
        PCCERT_CONTEXT  pCertContext,
        BOOL            *pfInitialSelectedCert,
        void            *pvCallbackData
        );

typedef struct {
    HCERTSTORE hStore;
    PCCERT_CHAIN_CONTEXT * prgpChain;
    DWORD cChain;
}CERT_SELECTUI_INPUT, *PCERT_SELECTUI_INPUT;

//+----------------------------------------------------------------------------
//
// CertSelectionGetSerializedBlob
//
// The API to obtain serialized blob from an input struct
//
//
//-----------------------------------------------------------------------------
HRESULT
WINAPI
CertSelectionGetSerializedBlob(
            _In_                            PCERT_SELECTUI_INPUT pcsi,
            _Outptr_result_maybenull_		void ** ppOutBuffer,
            _Out_                           ULONG *pulOutBufferSize);

//+----------------------------------------------------------------------------
// Valid values for dwFlags in CRYPTUI_CERT_MGR_STRUCT struct.
//-----------------------------------------------------------------------------
#define CRYPTUI_CERT_MGR_TAB_MASK                       0x0000000F
#define CRYPTUI_CERT_MGR_PUBLISHER_TAB                  0x00000004
#define CRYPTUI_CERT_MGR_SINGLE_TAB_FLAG                0x00008000

//+----------------------------------------------------------------------------
//
// CRYPTUI_CERT_MGR_STRUCT
//
// dwSize               IN Required: Should be set to 
//                                   sizeof(CRYPTUI_CERT_MGR_STRUCT)
//
// hwndParent           IN Optional: Parent of this dialog.
//
// dwFlags              IN Optional: Personal is the default initially selected
//                                   tab.
//
//                                   CRYPTUI_CERT_MGR_PUBLISHER_TAB may be set
//                                   to select Trusted Publishers as the
//                                   initially selected tab.
//
//                                   CRYPTUI_CERT_MGR_SINGLE_TAB_FLAG may also
//                                   be set to only display the Trusted
//                                   Publishers tab.
//
// pwszTitle            IN Optional: Title of the dialog.
//
// pszInitUsageOID      IN Optional: The enhanced key usage object identifier 
//                                   (OID). Certificates with this OID will 
//                                   initially be shown as a default. User
//                                   can then choose different OIDs. NULL 
//                                   means all certificates will be shown 
//                                   initially.
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_CERT_MGR_STRUCT
{
    DWORD               dwSize;
    HWND                hwndParent;
    DWORD               dwFlags;
    LPCWSTR             pwszTitle;
    LPCSTR              pszInitUsageOID;
} CRYPTUI_CERT_MGR_STRUCT, *PCRYPTUI_CERT_MGR_STRUCT;

typedef const CRYPTUI_CERT_MGR_STRUCT *PCCRYPTUI_CERT_MGR_STRUCT;


//+----------------------------------------------------------------------------
//
// CryptUIDlgCertMgr
//
// The wizard to manage certificates in store.
//
// pCryptUICertMgr      IN  Required: Poitner to CRYPTUI_CERT_MGR_STRUCT 
//                                    structure.
//
//-----------------------------------------------------------------------------
_Success_(return == TRUE)
BOOL
WINAPI
CryptUIDlgCertMgr(
    _In_                  PCCRYPTUI_CERT_MGR_STRUCT pCryptUICertMgr
    );
        
//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO
//
// dwSize               IN Required: Should be set to 
//                                   sizeof(CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO)
//
// pGuidSubject         IN Required: Idenfity the sip functions to load
//
// cbBlob               IN Required: The size of blob, in bytes
//
// pwszDispalyName      IN Optional: The display name of the blob to sign
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO
{
    DWORD               dwSize;			
    GUID                *pGuidSubject;
    DWORD               cbBlob;				
    BYTE                *pbBlob;			
    LPCWSTR             pwszDisplayName;
} CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO, *PCRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO *PCCRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO;

//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO
//
// dwSize               IN Required: Should be set to 
//                                   sizeof(CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO)
//
// cCertStore           IN Required: The acount of certificate store array that
//                                   includes potentical sining certs
//
// rghCertStore         IN Required: The certificate store array that includes 
//                                   potential signing certs
//
// pFilterCallback      IN Optional: The filter call back function for display 
//                                   the certificate
//
// pvCallbackData       IN Optional: The call back data
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO
{
    DWORD               dwSize;	
    DWORD               cCertStore;			
    HCERTSTORE          *rghCertStore;
    PFNCFILTERPROC      pFilterCallback;
    void *              pvCallbackData;
} CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO, *PCRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO *PCCRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO;

//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO
//
// dwSize               IN Required: Should be set to 
//                                   sizeof(CRYPT_WIZ_DIGITAL_SIGN_PVK_FILE_INFO)
//
// pwszPvkFileName      IN Required: The PVK file name
//
// pwszProvName         IN Required: The provider name
//
// dwProvType           IN Required: The provider type
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO
{
    DWORD               dwSize;
    LPWSTR              pwszPvkFileName;
    LPWSTR              pwszProvName;
    DWORD               dwProvType;
} CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO, *PCRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO *PCCRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO;

//+----------------------------------------------------------------------------
// Valid values for dwPvkChoice in CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO 
// struct.
//-----------------------------------------------------------------------------
#define CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE                0x01
#define CRYPTUI_WIZ_DIGITAL_SIGN_PVK_PROV                0x02

//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO
//
// dwSize                   IN Required: Should be set to 
//                                       sizeof(CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO)
//
// pwszSigningCertFileName  IN Required: The file name that contains the 
//                                       signing cert(s)
//
// dwPvkChoice              IN Required: Indicate the private key type. 
//                                       It can be one of the following:
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_PVK_PROV
//
//  pPvkFileInfo            IN Required: If dwPvkChoice == CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE
//
//  pPvkProvInfo            IN Required: If dwPvkContainer == CRYPTUI_WIZ_DIGITAL_SIGN_PVK_PROV
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO
{
    DWORD                                         dwSize;
    LPWSTR                                        pwszSigningCertFileName;
    DWORD                                         dwPvkChoice;		
    union
    {
        PCCRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO  pPvkFileInfo;
        PCRYPT_KEY_PROV_INFO                      pPvkProvInfo;
    };

} CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO, *PCRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO *PCCRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO;

//+----------------------------------------------------------------------------
// Valid values for dwAttrFlags in CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO 
// struct.
//-----------------------------------------------------------------------------
#define CRYPTUI_WIZ_DIGITAL_SIGN_COMMERCIAL              0x0001
#define CRYPTUI_WIZ_DIGITAL_SIGN_INDIVIDUAL              0x0002

//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO
//
// dwSize                       IN Required: Should be set to 
//                                           sizeof(CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO)
//
// dwAttrFlags                  IN Required: Flag to indicate signing options.
//                                           It can be one of the following:
//                                               CRYPTUI_WIZ_DIGITAL_SIGN_COMMERCIAL
//                                               CRYPTUI_WIZ_DIGITAL_SIGN_INDIVIDUAL
//
// pwszDescription              IN Optional: The description of the signing 
//                                           subject.

// pwszMoreInfoLocation         IN Optional: The localtion to get more 
//                                           information about file this 
//                                           information will be shown upon 
//                                           download time.
//
// pszHashAlg                   IN Optional: The hashing algorithm for the 
//                                           signature. NULL means using SHA1 
//                                           hashing algorithm.
//
// pwszSigningCertDisplayString IN Optional: The display string to be 
//                                           displayed on the signing 
//                                           certificate wizard page. The 
//                                           string should prompt user to 
//                                           select a certificate for a 
//                                           particular purpose.
//
// hAddtionalCertStores         IN Optional: The addtional cert store to add to
//                                           the signature.
//
// psAuthenticated              IN Optional: User supplied authenticated 
//                                           attributes added to the signature.
//
// psUnauthenticated	        IN Optional: User supplied unauthenticated 
//                                           attributes added to the signature.
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO
{
    DWORD                   dwSize;			
    DWORD                   dwAttrFlags;
    LPCWSTR                 pwszDescription;
    LPCWSTR                 pwszMoreInfoLocation;		
    LPCSTR                  pszHashAlg;
    LPCWSTR                 pwszSigningCertDisplayString;
    HCERTSTORE              hAdditionalCertStore;
    PCRYPT_ATTRIBUTES       psAuthenticated;	
    PCRYPT_ATTRIBUTES       psUnauthenticated;	
} CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO, *PCRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO *PCCRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO;

//+----------------------------------------------------------------------------
// Valid values for dwSubjectChoice in CRYPTUI_WIZ_DIGITAL_SIGN_INFO struct.
//-----------------------------------------------------------------------------
#define CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_FILE            0x01
#define CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_BLOB            0x02

//+----------------------------------------------------------------------------
// Valid values for dwSigningCertChoice in CRYPTUI_WIZ_DIGITAL_SIGN_INFO 
// struct.
//-----------------------------------------------------------------------------
#define CRYPTUI_WIZ_DIGITAL_SIGN_CERT                    0x01
#define CRYPTUI_WIZ_DIGITAL_SIGN_STORE                   0x02
#define CRYPTUI_WIZ_DIGITAL_SIGN_PVK                     0x03

//+----------------------------------------------------------------------------
// Valid values for dwAddtionalCertChoice in CRYPTUI_WIZ_DIGITAL_SIGN_INFO 
// struct.
//-----------------------------------------------------------------------------
#define CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN               0x00000001
#define CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN_NO_ROOT       0x00000002

//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_INFO
//
// dwSize                   IN Required: Should be set to 
//                                       sizeof(CRYPTUI_WIZ_DIGITAL_SIGN_INFO)
//
// dwSubjectChoice          IN Required: If CRYPTUI_WIZ_NO_UI is set in dwFlags 
//                                       of the CryptUIWizDigitalSign call.
//
//                             Optional: If CRYPTUI_WIZ_NO_UI is not set in 
//                                       dwFlags of the CryptUIWizDigitalSign
//                                       call.
//
//                                       Indicate whether to sign a file or to 
//                                       sign a memory blob. 0 means promting 
//                                       user for the file to sign.
//
//                                       It can be one of the following:
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_FILE
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_BLOB
//
// pwszFileName             IN Required: If dwSubjectChoice == CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_FILE
//
// pSignBlobInfo            IN Required: If dwSubhectChoice == CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_BLOB
//
// dwSigningCertChoice      IN Optional: Indicate the signing certificate.
//                                       0 means using the certificates in
//                                       "My" store".
//
//                                       It can be one of the following choices:
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_CERT
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_STORE
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_PVK
//
//                                       If CRYPTUI_WIZ_NO_UI is set in dwFlags 
//                                       of the CryptUIWizDigitalSign call, 
//                                       dwSigningCertChoice has to be
//                                       CRYPTUI_WIZ_DIGITAL_SIGN_CERT or
//                                       CRYPTUI_WIZ_DIGITAL_SIGN_PVK
//
// pSigningCertContext      IN Required: If dwSigningCertChoice == CRYPTUI_WIZ_DIGITAL_SIGN_CERT
//
// pSigningCertStore        IN Required: If dwSigningCertChoice == CRYPTUI_WIZ_DIGITAL_SIGN_STORE
//
// pSigningCertPvkInfo      IN Required: If dwSigningCertChoise == CRYPTUI_WIZ_DIGITAL_SIGN_PVK
//
// pwszTimestampURL         IN Optional: The timestamp URL address.
//
// dwAdditionalCertChoice   IN Optional: Indicate additional certificates to be
//                                       included in the signature. 0 means no 
//                                       addtional certificates will be added.
//
//                                       The following flags are mutually 
//                                       exclusive.
//                                       Only one of them can be set:
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN
//                                           CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN_NO_ROOT
//
// pSignExtInfo             IN Optional: The extended information for signing.
//
//-----------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_INFO
{
    DWORD                                           dwSize;			
    DWORD                                           dwSubjectChoice;	
    union
    {
        LPCWSTR                                     pwszFileName;	
        PCCRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO        pSignBlobInfo;	
    };
    DWORD                                           dwSigningCertChoice;
    union
    {
        PCCERT_CONTEXT                              pSigningCertContext;
        PCCRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO       pSigningCertStore;
        PCCRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO    pSigningCertPvkInfo;
    };
    LPCWSTR                                         pwszTimestampURL;
    DWORD                                           dwAdditionalCertChoice;
    PCCRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO        pSignExtInfo;
} CRYPTUI_WIZ_DIGITAL_SIGN_INFO, *PCRYPTUI_WIZ_DIGITAL_SIGN_INFO;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_INFO *PCCRYPTUI_WIZ_DIGITAL_SIGN_INFO;

//+----------------------------------------------------------------------------
//
// CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT
//
// dwSize               IN Required: Should be set to 
//                                   sizeof(CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT)
//
// cbBlob               IN Required: The size of pbBlob in bytes.
//
// pbBlob               IN Required: The signed blob.
//
//-----------------------------------------------------------------------------

typedef struct _CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT
{
    DWORD               dwSize;			
    DWORD               cbBlob;				
    BYTE                *pbBlob;			
} CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT, *PCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT;

typedef const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT *PCCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT;

//+----------------------------------------------------------------------------
// Valid values for dwFlags parameter to CryptUIWizDigitalSign.
//-----------------------------------------------------------------------------
#define CRYPTUI_WIZ_NO_UI                               0x0001
#define CRYPTUI_WIZ_DIGITAL_SIGN_EXCLUDE_PAGE_HASHES    0x0002

// The above CRYPTUI_WIZ_DIGITAL_SIGN_EXCLUDE_PAGE_HASHES takes precedence if
// also set.
#define CRYPTUI_WIZ_DIGITAL_SIGN_INCLUDE_PAGE_HASHES    0x0004

//+----------------------------------------------------------------------------
//
// CryptUIWizDigitalSign
//
// The wizard to digitally sign a document or a blob.
//
// If CRYPTUI_WIZ_NO_UI is set in dwFlags, no UI will be shown.  Otherwise,
// user will be prompted for input through a wizard.
//
// dwFlags              IN Required: See dwFlags values above.
//
// hwndParent           IN Optional: The parent window handle.
//
// pwszWizardTitle      IN Optional: The title of the wizard.
//
// pDigitalSignInfo     IN Required: The information about the signing process.
//
// ppSignContext        OUT Optional: The context pointer points to the signed 
//                                    blob.
//
//-----------------------------------------------------------------------------
_Success_(return == TRUE)
BOOL
WINAPI
CryptUIWizDigitalSign(
    _In_                DWORD                               dwFlags,
    _In_opt_            HWND                                hwndParent,
    _In_opt_            LPCWSTR                             pwszWizardTitle,
    _In_                PCCRYPTUI_WIZ_DIGITAL_SIGN_INFO     pDigitalSignInfo,
    _Outptr_opt_        PCCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT  *ppSignContext
    );

_Success_(return == TRUE)
BOOL
WINAPI
CryptUIWizFreeDigitalSignContext(
    _In_                PCCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT  pSignContext
    );
     

/////////////////////////////////////////////////////////////////////////////////////////////////////
//
// dwSize                          size of this struct
// hwndParent                      parent of this dialog                                    (OPTIONAL)
// dwFlags                         flags, may a combination of any of the flags below       (OPTIONAL)
// szTitle                         title for the window                                     (OPTIONAL)
// pCertContext                    the cert context that is to be displayed
// rgszPurposes                    array of purposes that this cert is to be validated for  (OPTIONAL)
// cPurposes                       number of purposes                                       (OPTIONAL)
// pCryptProviderData/hWVTStateData if WinVerifyTrust has already been called for the cert  (OPTIONAL)
//                                 then pass in a pointer to the state struct that was
//                                 acquired through a call to WTHelperProvDataFromStateData(),
//                                 or pass in the hWVTStateData of the WINTRUST_DATA struct
//                                 if WTHelperProvDataFromStateData() was not called.
//                                 if pCryptProviderData/hWVTStateData is used then
//                                 fpCryptProviderDataTrustedUsage, idxSigner, idxCert, and
//                                 fCounterSignature must be set
// fpCryptProviderDataTrustedUsage if WinVerifyTrust was called this is the result of whether (OPTIONAL)
//                                 the cert was trusted
// idxSigner                       the index of the signer to view                          (OPTIONAL)
// idxCert                         the index of the cert that is being viewed within the    (OPTIONAL)
//                                 signer chain.  the cert context of this cert MUST match
//                                 pCertContext
// fCounterSigner                  set to TRUE if a counter signature is being viewed.  if  (OPTIONAL)
//                                 this is TRUE then idxCounterSigner must be valid
// idxCounterSigner                the index of the counter signer to view                  (OPTIONAL)
// cStores                         Count of other stores to search when building and        (OPTIONAL)
//                                 validating chain
// rghStores                       Array of other stores to search when buliding and        (OPTIONAL)
//                                 validating chain
// cPropSheetPages                 number of extra pages to add to the dialog.              (OPTIONAL)
// rgPropSheetPages                extra pages to add to the dialog.                        (OPTIONAL)
//                                 each page in this array will NOT recieve the lParam in
//                                 the PROPSHEET structure as the lParam in the
//                                 WM_INITDIALOG, instead it will receive a pointer to a
//                                 CRYPTUI_INITDIALOG_STRUCT (defined below) which contains
//                                 the lParam in the PROPSSHEET structure AND the
//                                 PCCERT_CONTEXT for which the page is being displayed.
// nStartPage                      this is the index of the initial page that will be
//                                 displayed.  if the upper most bit (0x8000) is set then
//                                 the index is assumed to index rgPropSheetPages
//                                 (after the upper most bit has been stripped off.  eg.
//                                 0x8000 will indicate the first page in rgPropSheetPages),
//                                 if the upper most bit is 0 then nStartPage will be the
//                                 starting index of the default certificate dialog pages.
//
/////////////////////////////////////////////////////////////////////////////////////////////////////

// dwFlags
#define CRYPTUI_HIDE_HIERARCHYPAGE          0x00000001
#define CRYPTUI_HIDE_DETAILPAGE             0x00000002
#define CRYPTUI_DISABLE_EDITPROPERTIES      0x00000004
#define CRYPTUI_ENABLE_EDITPROPERTIES       0x00000008
#define CRYPTUI_DISABLE_ADDTOSTORE          0x00000010
#define CRYPTUI_ENABLE_ADDTOSTORE           0x00000020
#define CRYPTUI_ACCEPT_DECLINE_STYLE        0x00000040
#define CRYPTUI_IGNORE_UNTRUSTED_ROOT       0x00000080
#define CRYPTUI_DONT_OPEN_STORES            0x00000100
#define CRYPTUI_ONLY_OPEN_ROOT_STORE        0x00000200
#define CRYPTUI_WARN_UNTRUSTED_ROOT         0x00000400  // For use with viewing of certificates on remote
                                                        // machines only.  If this flag is used rghStores[0]
                                                        // must be the handle of the root store on the remote machine.
#define CRYPTUI_ENABLE_REVOCATION_CHECKING  0x00000800  // This flag is only valid if pCryptProviderData/hWVTStateData
                                                        // is not passed in.
#define CRYPTUI_WARN_REMOTE_TRUST           0x00001000
#define CRYPTUI_DISABLE_EXPORT              0x00002000  // If this flag is set, then the "Copy to file" button will be
                                                        // disabled on the Detail page.
                                                                
// Revocation flags is only valid if pCryptProviderData/hWVTStateData is not passed in.
#define CRYPTUI_ENABLE_REVOCATION_CHECK_END_CERT           0x00004000
#define CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN              0x00008000
#define CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT CRYPTUI_ENABLE_REVOCATION_CHECKING // Changed the default behavior
        
                                                                                              // to not check root.
#define CRYPTUI_DISABLE_HTMLLINK                           0x00010000   // to disable helplink in viewing certificate
#define CRYPTUI_DISABLE_ISSUERSTATEMENT                    0x00020000   // to disable issuer statement button

#define CRYPTUI_CACHE_ONLY_URL_RETRIEVAL                   0x00040000   // to disable online revocation checking
//
// this struct is passed as the lParam in the WM_INITDIALOG call to each
// property sheet that is in the rgPropSheetPages array of the
// CRYPTUI_VIEWCERTIFICATE_STRUCT structure
//
typedef struct tagCRYPTUI_INITDIALOG_STRUCT {
    LPARAM          lParam;
    PCCERT_CONTEXT  pCertContext;
} CRYPTUI_INITDIALOG_STRUCT, *PCRYPTUI_INITDIALOG_STRUCT;


typedef struct tagCRYPTUI_VIEWCERTIFICATE_STRUCTW {
    DWORD                       dwSize;
    HWND                        hwndParent;                     // OPTIONAL
    DWORD                       dwFlags;                        // OPTIONAL
    LPCWSTR                     szTitle;                        // OPTIONAL
    PCCERT_CONTEXT              pCertContext;
    LPCSTR *                    rgszPurposes;                   // OPTIONAL
    DWORD                       cPurposes;                      // OPTIONAL
    union
    {
        CRYPT_PROVIDER_DATA const * pCryptProviderData;         // OPTIONAL
        HANDLE                      hWVTStateData;              // OPTIONAL
    };
    BOOL                        fpCryptProviderDataTrustedUsage;// OPTIONAL
    DWORD                       idxSigner;                      // OPTIONAL
    DWORD                       idxCert;                        // OPTIONAL
    BOOL                        fCounterSigner;                 // OPTIONAL
    DWORD                       idxCounterSigner;               // OPTIONAL
    DWORD                       cStores;                        // OPTIONAL
    HCERTSTORE *                rghStores;                      // OPTIONAL
    DWORD                       cPropSheetPages;                // OPTIONAL
    LPCPROPSHEETPAGEW           rgPropSheetPages;               // OPTIONAL
    DWORD                       nStartPage;
} CRYPTUI_VIEWCERTIFICATE_STRUCTW, *PCRYPTUI_VIEWCERTIFICATE_STRUCTW;
typedef const CRYPTUI_VIEWCERTIFICATE_STRUCTW *PCCRYPTUI_VIEWCERTIFICATE_STRUCTW;


typedef struct tagCRYPTUI_VIEWCERTIFICATE_STRUCTA {
    DWORD                       dwSize;
    HWND                        hwndParent;                     // OPTIONAL
    DWORD                       dwFlags;                        // OPTIONAL
    LPCSTR                      szTitle;                        // OPTIONAL
    PCCERT_CONTEXT              pCertContext;
    LPCSTR *                    rgszPurposes;                   // OPTIONAL
    DWORD                       cPurposes;                      // OPTIONAL
    union
    {
        CRYPT_PROVIDER_DATA const * pCryptProviderData;         // OPTIONAL
        HANDLE                      hWVTStateData;              // OPTIONAL
    };
    BOOL                        fpCryptProviderDataTrustedUsage;// OPTIONAL
    DWORD                       idxSigner;                      // OPTIONAL
    DWORD                       idxCert;                        // OPTIONAL
    BOOL                        fCounterSigner;                 // OPTIONAL
    DWORD                       idxCounterSigner;               // OPTIONAL
    DWORD                       cStores;                        // OPTIONAL
    HCERTSTORE *                rghStores;                      // OPTIONAL
    DWORD                       cPropSheetPages;                // OPTIONAL
    LPCPROPSHEETPAGEA           rgPropSheetPages;               // OPTIONAL
    DWORD                       nStartPage;
} CRYPTUI_VIEWCERTIFICATE_STRUCTA, *PCRYPTUI_VIEWCERTIFICATE_STRUCTA;
typedef const CRYPTUI_VIEWCERTIFICATE_STRUCTA *PCCRYPTUI_VIEWCERTIFICATE_STRUCTA;

//
// pfPropertiesChanged             this will be set by the dialog proc to inform the caller
//                                 if any properties have been changed on certs in the chain
//                                 while the dialog was open
//
_Success_(return == TRUE)
BOOL
WINAPI
CryptUIDlgViewCertificateW(
        _In_  PCCRYPTUI_VIEWCERTIFICATE_STRUCTW   pCertViewInfo,
        _Out_ BOOL                                *pfPropertiesChanged  // OPTIONAL
        );

_Success_(return == TRUE)
BOOL
WINAPI
CryptUIDlgViewCertificateA(
        _In_  PCCRYPTUI_VIEWCERTIFICATE_STRUCTA   pCertViewInfo,
        _Out_ BOOL                                *pfPropertiesChanged  // OPTIONAL
        );

#ifdef UNICODE
#define CryptUIDlgViewCertificate           CryptUIDlgViewCertificateW
#define PCRYPTUI_VIEWCERTIFICATE_STRUCT     PCRYPTUI_VIEWCERTIFICATE_STRUCTW
#define CRYPTUI_VIEWCERTIFICATE_STRUCT      CRYPTUI_VIEWCERTIFICATE_STRUCTW
#define PCCRYPTUI_VIEWCERTIFICATE_STRUCT    PCCRYPTUI_VIEWCERTIFICATE_STRUCTW
#else
#define CryptUIDlgViewCertificate           CryptUIDlgViewCertificateA
#define PCRYPTUI_VIEWCERTIFICATE_STRUCT     PCRYPTUI_VIEWCERTIFICATE_STRUCTA
#define CRYPTUI_VIEWCERTIFICATE_STRUCT      CRYPTUI_VIEWCERTIFICATE_STRUCTA
#define PCCRYPTUI_VIEWCERTIFICATE_STRUCT    PCCRYPTUI_VIEWCERTIFICATE_STRUCTA
#endif

//-------------------------------------------------------------------------
//
//	Valid values for dwSubjectChoice in CRYPTUI_WIZ_EXPORT_INFO
//-------------------------------------------------------------------------
#define     CRYPTUI_WIZ_EXPORT_CERT_CONTEXT 			        1
#define     CRYPTUI_WIZ_EXPORT_CTL_CONTEXT  			        2
#define     CRYPTUI_WIZ_EXPORT_CRL_CONTEXT  			        3
#define     CRYPTUI_WIZ_EXPORT_CERT_STORE   			        4
#define     CRYPTUI_WIZ_EXPORT_CERT_STORE_CERTIFICATES_ONLY   	5
#define     CRYPTUI_WIZ_EXPORT_FORMAT_CRL                       6
#define     CRYPTUI_WIZ_EXPORT_FORMAT_CTL                       7

//-------------------------------------------------------------------------
//
//	Struct to define the object to be exported and where to export it to
//
//  CRYPTUI_WIZ_EXPORT_SUBJECT_INFO
//
//-------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_EXPORT_INFO
{
	DWORD					dwSize;				//Required: should be set to sizeof(CRYPTUI_WIZ_EXPORT_INFO)
    LPCWSTR                 pwszExportFileName; //Required if the CRYPTUI_WIZ_NO_UI flag is set, Optional otherwise.
                                                //The fully qualified file name to export to, if this is
                                                //non-NULL and the CRYPTUI_WIZ_NO_UI flag is NOT set, then it is
                                                //displayed to the user as the default file name
	DWORD					dwSubjectChoice;	//Required:	indicate the type of the subject:
                                                //          If can one of the following:
                                                //          CRYPTUI_WIZ_EXPORT_CERT_CONTEXT
                                                //          CRYPTUI_WIZ_EXPORT_CTL_CONTEXT
                                                //          CRYPTUI_WIZ_EXPORT_CRL_CONTEXT
                                                //          CRYPTUI_WIZ_EXPORT_CERT_STORE
						                        //	        CRYPTUI_WIZ_EXPORT_CERT_STORE_CERTIFICATES_ONLY
    union
	{
	PCCERT_CONTEXT      pCertContext;
        PCCTL_CONTEXT       pCTLContext;
        PCCRL_CONTEXT       pCRLContext;
        HCERTSTORE          hCertStore;
    };

    DWORD                   cStores;            // Optional: count of extra stores to search for the certs in the
                                                //           trust chain if the chain is being exported with a cert.
                                                //           this is ignored if dwSubjectChoice is anything other
                                                //           than CRYPTUI_WIZ_EXPORT_CERT_CONTEXT
    HCERTSTORE *            rghStores;          // Optional: array of extra stores to search for the certs in the
                                                //           trust chain if the chain is being exported with a cert.
                                                //           this is ignored if dwSubjectChoice is anything other
                                                //           than CRYPTUI_WIZ_EXPORT_CERT_CONTEXT

}CRYPTUI_WIZ_EXPORT_INFO, *PCRYPTUI_WIZ_EXPORT_INFO;

typedef const CRYPTUI_WIZ_EXPORT_INFO *PCCRYPTUI_WIZ_EXPORT_INFO;


//-------------------------------------------------------------------------
//
//	Valid values for dwExportFormat in CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO
//-------------------------------------------------------------------------
#define     CRYPTUI_WIZ_EXPORT_FORMAT_DER                   1
#define     CRYPTUI_WIZ_EXPORT_FORMAT_PFX                   2
#define     CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7                 3
#define     CRYPTUI_WIZ_EXPORT_FORMAT_BASE64                4
#define     CRYPTUI_WIZ_EXPORT_FORMAT_SERIALIZED_CERT_STORE 5   // NOTE: not currently supported!!

//-------------------------------------------------------------------------
//
//	Struct to define the information needed to export a CERT_CONTEXT
//
//  CRYPTUI_WIZ_EXPORT_NOUI_INFO
//
//-------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO
{
	DWORD					dwSize;				//Required: should be set to sizeof(CRYPTUI_WIZ_EXPORT_NOUI_INFO)
	DWORD					dwExportFormat;	    //Required:
                                                //          It can be one of the following:
                                                //          CRYPTUI_WIZ_EXPORT_FORMAT_DER
                                                //          CRYPTUI_WIZ_EXPORT_FORMAT_PFX
                                                //          CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7
                                                //          CRYPTUI_WIZ_EXPORT_FORMAT_SERIALIZED_CERT_STORE

    BOOL                    fExportChain;       //Required
    BOOL                    fExportPrivateKeys; //Required
    LPCWSTR                 pwszPassword;       //Required if the fExportPrivateKeys boolean is TRUE, otherwise,
                                                //it is ignored
    BOOL                    fStrongEncryption;  //Required if dwExportFormat is CRYPTUI_WIZ_EXPORT_FORMAT_PFX
                                                //Note that if this flag is TRUE then the PFX blob produced is
                                                //NOT compatible with IE4.

}CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO, *PCRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO;

typedef const CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO *PCCRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO;

//-----------------------------------------------------------------------
//
// CryptUIWizExport
//
//  The export wizard to export public key related objects to a file
//
//  If dwFlags is set to CRYPTUI_WIZ_NO_UI, no UI will be shown.  Otherwise,
//  User will be prompted for input through a wizard.
//
//  If CRYPTUI_WIZ_NO_UI is set in dwFlags:
//      hwndParent:         Ignored
//      pwszWizardTitle:    Ignored
//      pExportInfo:        IN Required:    The subject to export.
//      pvoid:              IN Required:    Contains information about how to do the export based on what
//                                          is being exported
//
//                                          dwSubjectChoice                     INPUT TYPE
//                                          -------------------------------------------------------------------------
//                                          CRYPTUI_WIZ_EXPORT_CERT_CONTEXT     PCCRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO
//                                          CRYPTUI_WIZ_EXPORT_CTL_CONTEXT      NULL
//                                          CRYPTUI_WIZ_EXPORT_CRL_CONTEXT      NULL
//                                          CRYPTUI_WIZ_EXPORT_CERT_STORE       NULL
//
//  If CRYPTUI_WIZ_NO_UI is not set in dwFlags:
//      hwndPrarent:        IN Optional:    The parent window for the wizard
//      pwszWizardTitle:    IN Optional:    The title of the wizard
//                                          If NULL, the default will be IDS_EXPORT_WIZARD_TITLE
//      pExportInfo:        IN Required:    The subject to export.
//      pvoid:              IN Optional:    Contains information about how to do the export based on what
//                                          is being exported.  See above table for values, if this is non-NULL
//                                          the values are displayed to the user as the default choices.
//------------------------------------------------------------------------
_Success_(return == TRUE)
BOOL
WINAPI
CryptUIWizExport(
     _In_     DWORD                                  dwFlags,
     _In_opt_ HWND                                   hwndParent,
     _In_opt_ LPCWSTR                                pwszWizardTitle,
     _In_     PCCRYPTUI_WIZ_EXPORT_INFO              pExportInfo,
     _In_opt_ void                                   *pvoid
);

//-------------------------------------------------------------------------
//
//	Valid values for dwSubjectChoice in IMPORT_SUBJECT_INFO
//-------------------------------------------------------------------------
#define     CRYPTUI_WIZ_IMPORT_SUBJECT_FILE                 1
#define     CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_CONTEXT         2
#define     CRYPTUI_WIZ_IMPORT_SUBJECT_CTL_CONTEXT          3
#define     CRYPTUI_WIZ_IMPORT_SUBJECT_CRL_CONTEXT          4
#define     CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_STORE           5

//-------------------------------------------------------------------------
//
//	Struct to define the subject CertImportWizard
//
//  CRYPTUI_WIZ_IMPORT_SUBJECT_INFO
//
//-------------------------------------------------------------------------
typedef struct _CRYPTUI_WIZ_IMPORT_SUBJECT_INFO
{
	DWORD					dwSize;				//Required: should be set to sizeof(IMPORT_SUBJECT_INFO)
	DWORD					dwSubjectChoice;	//Required:	indicate the type of the subject:
                                                //          If can one of the following:
                                                //          CRYPTUI_WIZ_IMPORT_SUBJECT_FILE
                                                //          CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_CONTEXT
                                                //          CRYPTUI_WIZ_IMPORT_SUBJECT_CTL_CONTEXT
                                                //          CRYPTUI_WIZ_IMPORT_SUBJECT_CRL_CONTEXT
                                                //          CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_STORE
    union
	{
		LPCWSTR          	pwszFileName;	
        PCCERT_CONTEXT      pCertContext;
        PCCTL_CONTEXT       pCTLContext;
        PCCRL_CONTEXT       pCRLContext;
        HCERTSTORE          hCertStore;
    };

    DWORD                   dwFlags;            //Required if pwszFileName contains a PFX BLOB.
                                                //Ignored otherwise
                                                //This is the same flag for PFXImportCertStore
    LPCWSTR                 pwszPassword;       //Required if pwszFileName contains a PFX BLOB.
                                                //ignored otherwise
}CRYPTUI_WIZ_IMPORT_SRC_INFO, *PCRYPTUI_WIZ_IMPORT_SRC_INFO;

typedef const CRYPTUI_WIZ_IMPORT_SRC_INFO *PCCRYPTUI_WIZ_IMPORT_SRC_INFO;

//-----------------------------------------------------------------------
//
// Valid flags for dwFlags in CryptUIWizImport
//
//-----------------------------------------------------------------------
//if this flag is set in dwFlags, user will not be allowed to change
//the hDesCertStore in the wizard page
#define   CRYPTUI_WIZ_IMPORT_NO_CHANGE_DEST_STORE           0x00010000

//Allow importing certificate
#define   CRYPTUI_WIZ_IMPORT_ALLOW_CERT                     0x00020000

//Allow importing certificate revocation list
#define   CRYPTUI_WIZ_IMPORT_ALLOW_CRL                      0x00040000

//Allow importing certificate trust list
#define   CRYPTUI_WIZ_IMPORT_ALLOW_CTL                      0x00080000

//import contents to local machine (currently only applicable for PFX imports)
#define   CRYPTUI_WIZ_IMPORT_TO_LOCALMACHINE                0x00100000

//import contents to current user (currently only applicable for PFX imports)
#define   CRYPTUI_WIZ_IMPORT_TO_CURRENTUSER                 0x00200000

//if the hDesCertStore is a remote store handle, this flag should be set
#define   CRYPTUI_WIZ_IMPORT_REMOTE_DEST_STORE              0x00400000

//-----------------------------------------------------------------------
//
// CryptUIWizImport
//
//  The import wizard to import public key related files to a certificate
//  store
//
//  dwFlags can be set to any combination of the following flags:
//  CRYPTUI_WIZ_NO_UI                           No UI will be shown.  Otherwise, User will be
//                                              prompted by a wizard.
//  CRYPTUI_WIZ_IMPORT_ALLOW_CERT               Allow importing certificate
//  CRYPTUI_WIZ_IMPORT_ALLOW_CRL                Allow importing CRL(certificate revocation list)
//  CRYPTUI_WIZ_IMPORT_ALLOW_CTL                Allow importing CTL(certificate trust list)
//  CRYPTUI_WIZ_IMPORT_NO_CHANGE_DEST_STORE     user will not be allowed to change
//                                              the hDesCertStore in the wizard page
//  CRYPTUI_WIZ_IMPORT_TO_LOCALMACHINE          the contents should be imported to local machine
//                                              (currently only applicable for PFX imports)
//  CRYPTUI_WIZ_IMPORT_TO_CURRENTUSER           the contents should be imported to current user
//                                              (currently only applicable for PFX imports)
//
//  Please notice that if neither of following three flags is in dwFlags, default to is
//  allow everything.
//  CRYPTUI_WIZ_IMPORT_ALLOW_CERT
//  CRYPTUI_WIZ_IMPORT_ALLOW_CRL
//  CRYPTUI_WIZ_IMPORT_ALLOW_CTL
//
//  Also, note that the CRYPTUI_WIZ_IMPORT_TO_LOCALMACHINE and CRYPTUI_WIZ_IMPORT_TO_CURRENTUSER
//  flags are used force the content of a pfx blob into either local machine or current user.
//  If neither of these flags are used and hDesCertStore is NULL then:
//  1) The private key in the pfx blob will be forced to be imported into current user.
//  2) If CRYPTUI_WIZ_NO_UI is NOT set, the wizard will prompt the user to select a certificate
//     store from the current user stores.
//
//
//
//  If CRYPTUI_WIZ_NO_UI is set in dwFlags:
//      hwndParent:         Ignored
//      pwszWizardTitle:    Ignored
//      pImportSubject:     IN Required:    The subject to import.
//      hDesCertStore:      IN Optional:    The destination certficate store
//
//  If CRYPTUI_WIZ_NO_UI is not set in dwFlags:
//      hwndPrarent:        IN Optional:    The parent window for the wizard
//      pwszWizardTitle:    IN Optional:    The title of the wizard
//                                          If NULL, the default will be IDS_IMPORT_WIZARD_TITLE
//      pImportSubject:     IN Optional:    The file name to import.
//                                          If NULL, the wizard will prompt user to enter the file name
//      hDesCertStore:      IN Optional:    The destination certificate store where the file wil be
//                                          imported to.  The store should be opened with
//                                          flag CERT_STORE_SET_LOCALIZED_NAME_FLAG.  If NULL, the wizard will prompt user to select
//                                          a certificate store.
//------------------------------------------------------------------------
_Success_(return == TRUE)
BOOL
WINAPI
CryptUIWizImport(
     _In_     DWORD                               dwFlags,
     _In_opt_ HWND                                hwndParent,
     _In_opt_ LPCWSTR                             pwszWizardTitle,
     _In_opt_ PCCRYPTUI_WIZ_IMPORT_SRC_INFO       pImportSrc,
     _In_opt_ HCERTSTORE                          hDestCertStore
);

#include <poppack.h>

#ifdef __cplusplus
}       // Balance extern "C" above
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _CRYPTUIAPI_H_
