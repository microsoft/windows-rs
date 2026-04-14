

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __credentialprovider_h__
#define __credentialprovider_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __ICredentialProviderCredential_FWD_DEFINED__
#define __ICredentialProviderCredential_FWD_DEFINED__
typedef interface ICredentialProviderCredential ICredentialProviderCredential;

#endif 	/* __ICredentialProviderCredential_FWD_DEFINED__ */


#ifndef __IQueryContinueWithStatus_FWD_DEFINED__
#define __IQueryContinueWithStatus_FWD_DEFINED__
typedef interface IQueryContinueWithStatus IQueryContinueWithStatus;

#endif 	/* __IQueryContinueWithStatus_FWD_DEFINED__ */


#ifndef __IConnectableCredentialProviderCredential_FWD_DEFINED__
#define __IConnectableCredentialProviderCredential_FWD_DEFINED__
typedef interface IConnectableCredentialProviderCredential IConnectableCredentialProviderCredential;

#endif 	/* __IConnectableCredentialProviderCredential_FWD_DEFINED__ */


#ifndef __ICredentialProviderCredentialEvents_FWD_DEFINED__
#define __ICredentialProviderCredentialEvents_FWD_DEFINED__
typedef interface ICredentialProviderCredentialEvents ICredentialProviderCredentialEvents;

#endif 	/* __ICredentialProviderCredentialEvents_FWD_DEFINED__ */


#ifndef __ICredentialProvider_FWD_DEFINED__
#define __ICredentialProvider_FWD_DEFINED__
typedef interface ICredentialProvider ICredentialProvider;

#endif 	/* __ICredentialProvider_FWD_DEFINED__ */


#ifndef __ICredentialProviderEvents_FWD_DEFINED__
#define __ICredentialProviderEvents_FWD_DEFINED__
typedef interface ICredentialProviderEvents ICredentialProviderEvents;

#endif 	/* __ICredentialProviderEvents_FWD_DEFINED__ */


#ifndef __ICredentialProviderFilter_FWD_DEFINED__
#define __ICredentialProviderFilter_FWD_DEFINED__
typedef interface ICredentialProviderFilter ICredentialProviderFilter;

#endif 	/* __ICredentialProviderFilter_FWD_DEFINED__ */


#ifndef __ICredentialProviderCredential2_FWD_DEFINED__
#define __ICredentialProviderCredential2_FWD_DEFINED__
typedef interface ICredentialProviderCredential2 ICredentialProviderCredential2;

#endif 	/* __ICredentialProviderCredential2_FWD_DEFINED__ */


#ifndef __ICredentialProviderCredentialWithFieldOptions_FWD_DEFINED__
#define __ICredentialProviderCredentialWithFieldOptions_FWD_DEFINED__
typedef interface ICredentialProviderCredentialWithFieldOptions ICredentialProviderCredentialWithFieldOptions;

#endif 	/* __ICredentialProviderCredentialWithFieldOptions_FWD_DEFINED__ */


#ifndef __ICredentialProviderCredentialEvents2_FWD_DEFINED__
#define __ICredentialProviderCredentialEvents2_FWD_DEFINED__
typedef interface ICredentialProviderCredentialEvents2 ICredentialProviderCredentialEvents2;

#endif 	/* __ICredentialProviderCredentialEvents2_FWD_DEFINED__ */


#ifndef __ICredentialProviderUser_FWD_DEFINED__
#define __ICredentialProviderUser_FWD_DEFINED__
typedef interface ICredentialProviderUser ICredentialProviderUser;

#endif 	/* __ICredentialProviderUser_FWD_DEFINED__ */


#ifndef __ICredentialProviderUserArray_FWD_DEFINED__
#define __ICredentialProviderUserArray_FWD_DEFINED__
typedef interface ICredentialProviderUserArray ICredentialProviderUserArray;

#endif 	/* __ICredentialProviderUserArray_FWD_DEFINED__ */


#ifndef __ICredentialProviderSetUserArray_FWD_DEFINED__
#define __ICredentialProviderSetUserArray_FWD_DEFINED__
typedef interface ICredentialProviderSetUserArray ICredentialProviderSetUserArray;

#endif 	/* __ICredentialProviderSetUserArray_FWD_DEFINED__ */


#ifndef __PasswordCredentialProvider_FWD_DEFINED__
#define __PasswordCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class PasswordCredentialProvider PasswordCredentialProvider;
#else
typedef struct PasswordCredentialProvider PasswordCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __PasswordCredentialProvider_FWD_DEFINED__ */


#ifndef __V1PasswordCredentialProvider_FWD_DEFINED__
#define __V1PasswordCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class V1PasswordCredentialProvider V1PasswordCredentialProvider;
#else
typedef struct V1PasswordCredentialProvider V1PasswordCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __V1PasswordCredentialProvider_FWD_DEFINED__ */


#ifndef __PINLogonCredentialProvider_FWD_DEFINED__
#define __PINLogonCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class PINLogonCredentialProvider PINLogonCredentialProvider;
#else
typedef struct PINLogonCredentialProvider PINLogonCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __PINLogonCredentialProvider_FWD_DEFINED__ */


#ifndef __NPCredentialProvider_FWD_DEFINED__
#define __NPCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class NPCredentialProvider NPCredentialProvider;
#else
typedef struct NPCredentialProvider NPCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __NPCredentialProvider_FWD_DEFINED__ */


#ifndef __SmartcardCredentialProvider_FWD_DEFINED__
#define __SmartcardCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class SmartcardCredentialProvider SmartcardCredentialProvider;
#else
typedef struct SmartcardCredentialProvider SmartcardCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __SmartcardCredentialProvider_FWD_DEFINED__ */


#ifndef __V1SmartcardCredentialProvider_FWD_DEFINED__
#define __V1SmartcardCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class V1SmartcardCredentialProvider V1SmartcardCredentialProvider;
#else
typedef struct V1SmartcardCredentialProvider V1SmartcardCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __V1SmartcardCredentialProvider_FWD_DEFINED__ */


#ifndef __SmartcardPinProvider_FWD_DEFINED__
#define __SmartcardPinProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class SmartcardPinProvider SmartcardPinProvider;
#else
typedef struct SmartcardPinProvider SmartcardPinProvider;
#endif /* __cplusplus */

#endif 	/* __SmartcardPinProvider_FWD_DEFINED__ */


#ifndef __SmartcardReaderSelectionProvider_FWD_DEFINED__
#define __SmartcardReaderSelectionProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class SmartcardReaderSelectionProvider SmartcardReaderSelectionProvider;
#else
typedef struct SmartcardReaderSelectionProvider SmartcardReaderSelectionProvider;
#endif /* __cplusplus */

#endif 	/* __SmartcardReaderSelectionProvider_FWD_DEFINED__ */


#ifndef __SmartcardWinRTProvider_FWD_DEFINED__
#define __SmartcardWinRTProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class SmartcardWinRTProvider SmartcardWinRTProvider;
#else
typedef struct SmartcardWinRTProvider SmartcardWinRTProvider;
#endif /* __cplusplus */

#endif 	/* __SmartcardWinRTProvider_FWD_DEFINED__ */


#ifndef __GenericCredentialProvider_FWD_DEFINED__
#define __GenericCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class GenericCredentialProvider GenericCredentialProvider;
#else
typedef struct GenericCredentialProvider GenericCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __GenericCredentialProvider_FWD_DEFINED__ */


#ifndef __RASProvider_FWD_DEFINED__
#define __RASProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class RASProvider RASProvider;
#else
typedef struct RASProvider RASProvider;
#endif /* __cplusplus */

#endif 	/* __RASProvider_FWD_DEFINED__ */


#ifndef __OnexCredentialProvider_FWD_DEFINED__
#define __OnexCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class OnexCredentialProvider OnexCredentialProvider;
#else
typedef struct OnexCredentialProvider OnexCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __OnexCredentialProvider_FWD_DEFINED__ */


#ifndef __OnexPlapSmartcardCredentialProvider_FWD_DEFINED__
#define __OnexPlapSmartcardCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class OnexPlapSmartcardCredentialProvider OnexPlapSmartcardCredentialProvider;
#else
typedef struct OnexPlapSmartcardCredentialProvider OnexPlapSmartcardCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __OnexPlapSmartcardCredentialProvider_FWD_DEFINED__ */


#ifndef __VaultProvider_FWD_DEFINED__
#define __VaultProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class VaultProvider VaultProvider;
#else
typedef struct VaultProvider VaultProvider;
#endif /* __cplusplus */

#endif 	/* __VaultProvider_FWD_DEFINED__ */


#ifndef __WinBioCredentialProvider_FWD_DEFINED__
#define __WinBioCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class WinBioCredentialProvider WinBioCredentialProvider;
#else
typedef struct WinBioCredentialProvider WinBioCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __WinBioCredentialProvider_FWD_DEFINED__ */


#ifndef __V1WinBioCredentialProvider_FWD_DEFINED__
#define __V1WinBioCredentialProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class V1WinBioCredentialProvider V1WinBioCredentialProvider;
#else
typedef struct V1WinBioCredentialProvider V1WinBioCredentialProvider;
#endif /* __cplusplus */

#endif 	/* __V1WinBioCredentialProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "shobjidl_core.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_credentialprovider_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [v1_enum] */ 
enum _CREDENTIAL_PROVIDER_USAGE_SCENARIO
    {
        CPUS_INVALID	= 0,
        CPUS_LOGON	= ( CPUS_INVALID + 1 ) ,
        CPUS_UNLOCK_WORKSTATION	= ( CPUS_LOGON + 1 ) ,
        CPUS_CHANGE_PASSWORD	= ( CPUS_UNLOCK_WORKSTATION + 1 ) ,
        CPUS_CREDUI	= ( CPUS_CHANGE_PASSWORD + 1 ) ,
        CPUS_PLAP	= ( CPUS_CREDUI + 1 ) 
    } 	CREDENTIAL_PROVIDER_USAGE_SCENARIO;

typedef /* [v1_enum] */ 
enum _CREDENTIAL_PROVIDER_FIELD_TYPE
    {
        CPFT_INVALID	= 0,
        CPFT_LARGE_TEXT	= ( CPFT_INVALID + 1 ) ,
        CPFT_SMALL_TEXT	= ( CPFT_LARGE_TEXT + 1 ) ,
        CPFT_COMMAND_LINK	= ( CPFT_SMALL_TEXT + 1 ) ,
        CPFT_EDIT_TEXT	= ( CPFT_COMMAND_LINK + 1 ) ,
        CPFT_PASSWORD_TEXT	= ( CPFT_EDIT_TEXT + 1 ) ,
        CPFT_TILE_IMAGE	= ( CPFT_PASSWORD_TEXT + 1 ) ,
        CPFT_CHECKBOX	= ( CPFT_TILE_IMAGE + 1 ) ,
        CPFT_COMBOBOX	= ( CPFT_CHECKBOX + 1 ) ,
        CPFT_SUBMIT_BUTTON	= ( CPFT_COMBOBOX + 1 ) 
    } 	CREDENTIAL_PROVIDER_FIELD_TYPE;

typedef /* [v1_enum] */ 
enum _CREDENTIAL_PROVIDER_FIELD_STATE
    {
        CPFS_HIDDEN	= 0,
        CPFS_DISPLAY_IN_SELECTED_TILE	= ( CPFS_HIDDEN + 1 ) ,
        CPFS_DISPLAY_IN_DESELECTED_TILE	= ( CPFS_DISPLAY_IN_SELECTED_TILE + 1 ) ,
        CPFS_DISPLAY_IN_BOTH	= ( CPFS_DISPLAY_IN_DESELECTED_TILE + 1 ) 
    } 	CREDENTIAL_PROVIDER_FIELD_STATE;

typedef /* [v1_enum] */ 
enum _CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE
    {
        CPFIS_NONE	= 0,
        CPFIS_READONLY	= ( CPFIS_NONE + 1 ) ,
        CPFIS_DISABLED	= ( CPFIS_READONLY + 1 ) ,
        CPFIS_FOCUSED	= ( CPFIS_DISABLED + 1 ) 
    } 	CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE;

typedef struct _CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR
    {
    DWORD dwFieldID;
    CREDENTIAL_PROVIDER_FIELD_TYPE cpft;
    LPWSTR pszLabel;
    GUID guidFieldType;
    } 	CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR;

typedef /* [v1_enum] */ 
enum _CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE
    {
        CPGSR_NO_CREDENTIAL_NOT_FINISHED	= 0,
        CPGSR_NO_CREDENTIAL_FINISHED	= ( CPGSR_NO_CREDENTIAL_NOT_FINISHED + 1 ) ,
        CPGSR_RETURN_CREDENTIAL_FINISHED	= ( CPGSR_NO_CREDENTIAL_FINISHED + 1 ) ,
        CPGSR_RETURN_NO_CREDENTIAL_FINISHED	= ( CPGSR_RETURN_CREDENTIAL_FINISHED + 1 ) 
    } 	CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE;

typedef /* [v1_enum] */ 
enum _CREDENTIAL_PROVIDER_STATUS_ICON
    {
        CPSI_NONE	= 0,
        CPSI_ERROR	= ( CPSI_NONE + 1 ) ,
        CPSI_WARNING	= ( CPSI_ERROR + 1 ) ,
        CPSI_SUCCESS	= ( CPSI_WARNING + 1 ) 
    } 	CREDENTIAL_PROVIDER_STATUS_ICON;

typedef struct _CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION
    {
    ULONG ulAuthenticationPackage;
    GUID clsidCredentialProvider;
    ULONG cbSerialization;
    /* [size_is] */ byte *rgbSerialization;
    } 	CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef /* [v1_enum] */ 
enum CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS
    {
        CPAO_NONE	= 0,
        CPAO_EMPTY_LOCAL	= 0x1,
        CPAO_EMPTY_CONNECTED	= 0x2
    } 	CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS)
typedef /* [v1_enum] */ 
enum CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS
    {
        CPCFO_NONE	= 0,
        CPCFO_ENABLE_PASSWORD_REVEAL	= 0x1,
        CPCFO_IS_EMAIL_ADDRESS	= 0x2,
        CPCFO_ENABLE_TOUCH_KEYBOARD_AUTO_INVOKE	= 0x4,
        CPCFO_NUMBERS_ONLY	= 0x8,
        CPCFO_SHOW_ENGLISH_KEYBOARD	= 0x10
    } 	CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#ifdef __midl
typedef LONG NTSTATUS;

#else // __midl
#ifndef NTSTATUS
typedef _Return_type_success_(return >= 0) LONG NTSTATUS;
#endif // NTSTATUS
#endif // __midl
#define CREDENTIAL_PROVIDER_NO_DEFAULT       ((DWORD)-1)



extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0000_v0_0_s_ifspec;

#ifndef __ICredentialProviderCredential_INTERFACE_DEFINED__
#define __ICredentialProviderCredential_INTERFACE_DEFINED__

/* interface ICredentialProviderCredential */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_ICredentialProviderCredential;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63913a93-40c1-481a-818d-4072ff8c70cc")
    ICredentialProviderCredential : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [annotation][in] */ 
            _In_  ICredentialProviderCredentialEvents *pcpce) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnAdvise( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSelected( 
            /* [annotation][out] */ 
            _Out_  BOOL *pbAutoLogon) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDeselected( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldState( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_STATE *pcpfs,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE *pcpfis) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringValue( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppsz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBitmapValue( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  HBITMAP *phbmp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCheckboxValue( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  BOOL *pbChecked,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszLabel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubmitButtonValue( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwAdjacentTo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComboBoxValueCount( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pcItems,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSelectedItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComboBoxValueAt( 
            /* [in] */ DWORD dwFieldID,
            DWORD dwItem,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStringValue( 
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCheckboxValue( 
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetComboBoxSelectedValue( 
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommandLinkClicked( 
            /* [in] */ DWORD dwFieldID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSerialization( 
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE *pcpgsr,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcs,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportResult( 
            /* [in] */ NTSTATUS ntsStatus,
            /* [in] */ NTSTATUS ntsSubstatus,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderCredentialVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderCredential * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            ICredentialProviderCredential * This,
            /* [annotation][in] */ 
            _In_  ICredentialProviderCredentialEvents *pcpce);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            ICredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetSelected)
        HRESULT ( STDMETHODCALLTYPE *SetSelected )( 
            ICredentialProviderCredential * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbAutoLogon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetDeselected)
        HRESULT ( STDMETHODCALLTYPE *SetDeselected )( 
            ICredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetFieldState)
        HRESULT ( STDMETHODCALLTYPE *GetFieldState )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_STATE *pcpfs,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE *pcpfis);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetStringValue)
        HRESULT ( STDMETHODCALLTYPE *GetStringValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppsz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetBitmapValue)
        HRESULT ( STDMETHODCALLTYPE *GetBitmapValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  HBITMAP *phbmp);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetCheckboxValue)
        HRESULT ( STDMETHODCALLTYPE *GetCheckboxValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  BOOL *pbChecked,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszLabel);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetSubmitButtonValue)
        HRESULT ( STDMETHODCALLTYPE *GetSubmitButtonValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwAdjacentTo);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetComboBoxValueCount)
        HRESULT ( STDMETHODCALLTYPE *GetComboBoxValueCount )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pcItems,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetComboBoxValueAt)
        HRESULT ( STDMETHODCALLTYPE *GetComboBoxValueAt )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            DWORD dwItem,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetStringValue)
        HRESULT ( STDMETHODCALLTYPE *SetStringValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR psz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetCheckboxValue)
        HRESULT ( STDMETHODCALLTYPE *SetCheckboxValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetComboBoxSelectedValue)
        HRESULT ( STDMETHODCALLTYPE *SetComboBoxSelectedValue )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, CommandLinkClicked)
        HRESULT ( STDMETHODCALLTYPE *CommandLinkClicked )( 
            ICredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetSerialization)
        HRESULT ( STDMETHODCALLTYPE *GetSerialization )( 
            ICredentialProviderCredential * This,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE *pcpgsr,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcs,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, ReportResult)
        HRESULT ( STDMETHODCALLTYPE *ReportResult )( 
            ICredentialProviderCredential * This,
            /* [in] */ NTSTATUS ntsStatus,
            /* [in] */ NTSTATUS ntsSubstatus,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon);
        
        END_INTERFACE
    } ICredentialProviderCredentialVtbl;

    interface ICredentialProviderCredential
    {
        CONST_VTBL struct ICredentialProviderCredentialVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderCredential_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderCredential_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderCredential_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderCredential_Advise(This,pcpce)	\
    ( (This)->lpVtbl -> Advise(This,pcpce) ) 

#define ICredentialProviderCredential_UnAdvise(This)	\
    ( (This)->lpVtbl -> UnAdvise(This) ) 

#define ICredentialProviderCredential_SetSelected(This,pbAutoLogon)	\
    ( (This)->lpVtbl -> SetSelected(This,pbAutoLogon) ) 

#define ICredentialProviderCredential_SetDeselected(This)	\
    ( (This)->lpVtbl -> SetDeselected(This) ) 

#define ICredentialProviderCredential_GetFieldState(This,dwFieldID,pcpfs,pcpfis)	\
    ( (This)->lpVtbl -> GetFieldState(This,dwFieldID,pcpfs,pcpfis) ) 

#define ICredentialProviderCredential_GetStringValue(This,dwFieldID,ppsz)	\
    ( (This)->lpVtbl -> GetStringValue(This,dwFieldID,ppsz) ) 

#define ICredentialProviderCredential_GetBitmapValue(This,dwFieldID,phbmp)	\
    ( (This)->lpVtbl -> GetBitmapValue(This,dwFieldID,phbmp) ) 

#define ICredentialProviderCredential_GetCheckboxValue(This,dwFieldID,pbChecked,ppszLabel)	\
    ( (This)->lpVtbl -> GetCheckboxValue(This,dwFieldID,pbChecked,ppszLabel) ) 

#define ICredentialProviderCredential_GetSubmitButtonValue(This,dwFieldID,pdwAdjacentTo)	\
    ( (This)->lpVtbl -> GetSubmitButtonValue(This,dwFieldID,pdwAdjacentTo) ) 

#define ICredentialProviderCredential_GetComboBoxValueCount(This,dwFieldID,pcItems,pdwSelectedItem)	\
    ( (This)->lpVtbl -> GetComboBoxValueCount(This,dwFieldID,pcItems,pdwSelectedItem) ) 

#define ICredentialProviderCredential_GetComboBoxValueAt(This,dwFieldID,dwItem,ppszItem)	\
    ( (This)->lpVtbl -> GetComboBoxValueAt(This,dwFieldID,dwItem,ppszItem) ) 

#define ICredentialProviderCredential_SetStringValue(This,dwFieldID,psz)	\
    ( (This)->lpVtbl -> SetStringValue(This,dwFieldID,psz) ) 

#define ICredentialProviderCredential_SetCheckboxValue(This,dwFieldID,bChecked)	\
    ( (This)->lpVtbl -> SetCheckboxValue(This,dwFieldID,bChecked) ) 

#define ICredentialProviderCredential_SetComboBoxSelectedValue(This,dwFieldID,dwSelectedItem)	\
    ( (This)->lpVtbl -> SetComboBoxSelectedValue(This,dwFieldID,dwSelectedItem) ) 

#define ICredentialProviderCredential_CommandLinkClicked(This,dwFieldID)	\
    ( (This)->lpVtbl -> CommandLinkClicked(This,dwFieldID) ) 

#define ICredentialProviderCredential_GetSerialization(This,pcpgsr,pcpcs,ppszOptionalStatusText,pcpsiOptionalStatusIcon)	\
    ( (This)->lpVtbl -> GetSerialization(This,pcpgsr,pcpcs,ppszOptionalStatusText,pcpsiOptionalStatusIcon) ) 

#define ICredentialProviderCredential_ReportResult(This,ntsStatus,ntsSubstatus,ppszOptionalStatusText,pcpsiOptionalStatusIcon)	\
    ( (This)->lpVtbl -> ReportResult(This,ntsStatus,ntsSubstatus,ppszOptionalStatusText,pcpsiOptionalStatusIcon) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderCredential_INTERFACE_DEFINED__ */


#ifndef __IQueryContinueWithStatus_INTERFACE_DEFINED__
#define __IQueryContinueWithStatus_INTERFACE_DEFINED__

/* interface IQueryContinueWithStatus */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_IQueryContinueWithStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9090be5b-502b-41fb-bccc-0049a6c7254b")
    IQueryContinueWithStatus : public IQueryContinue
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetStatusMessage( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR psz) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryContinueWithStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IQueryContinueWithStatus * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IQueryContinueWithStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IQueryContinueWithStatus * This);
        
        DECLSPEC_XFGVIRT(IQueryContinue, QueryContinue)
        HRESULT ( STDMETHODCALLTYPE *QueryContinue )( 
            IQueryContinueWithStatus * This);
        
        DECLSPEC_XFGVIRT(IQueryContinueWithStatus, SetStatusMessage)
        HRESULT ( STDMETHODCALLTYPE *SetStatusMessage )( 
            IQueryContinueWithStatus * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR psz);
        
        END_INTERFACE
    } IQueryContinueWithStatusVtbl;

    interface IQueryContinueWithStatus
    {
        CONST_VTBL struct IQueryContinueWithStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryContinueWithStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryContinueWithStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryContinueWithStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryContinueWithStatus_QueryContinue(This)	\
    ( (This)->lpVtbl -> QueryContinue(This) ) 


#define IQueryContinueWithStatus_SetStatusMessage(This,psz)	\
    ( (This)->lpVtbl -> SetStatusMessage(This,psz) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryContinueWithStatus_INTERFACE_DEFINED__ */


#ifndef __IConnectableCredentialProviderCredential_INTERFACE_DEFINED__
#define __IConnectableCredentialProviderCredential_INTERFACE_DEFINED__

/* interface IConnectableCredentialProviderCredential */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_IConnectableCredentialProviderCredential;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9387928b-ac75-4bf9-8ab2-2b93c4a55290")
    IConnectableCredentialProviderCredential : public ICredentialProviderCredential
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [annotation][in] */ 
            _In_  IQueryContinueWithStatus *pqcws) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConnectableCredentialProviderCredentialVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IConnectableCredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IConnectableCredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            IConnectableCredentialProviderCredential * This,
            /* [annotation][in] */ 
            _In_  ICredentialProviderCredentialEvents *pcpce);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            IConnectableCredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetSelected)
        HRESULT ( STDMETHODCALLTYPE *SetSelected )( 
            IConnectableCredentialProviderCredential * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbAutoLogon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetDeselected)
        HRESULT ( STDMETHODCALLTYPE *SetDeselected )( 
            IConnectableCredentialProviderCredential * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetFieldState)
        HRESULT ( STDMETHODCALLTYPE *GetFieldState )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_STATE *pcpfs,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE *pcpfis);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetStringValue)
        HRESULT ( STDMETHODCALLTYPE *GetStringValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppsz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetBitmapValue)
        HRESULT ( STDMETHODCALLTYPE *GetBitmapValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  HBITMAP *phbmp);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetCheckboxValue)
        HRESULT ( STDMETHODCALLTYPE *GetCheckboxValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  BOOL *pbChecked,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszLabel);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetSubmitButtonValue)
        HRESULT ( STDMETHODCALLTYPE *GetSubmitButtonValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwAdjacentTo);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetComboBoxValueCount)
        HRESULT ( STDMETHODCALLTYPE *GetComboBoxValueCount )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pcItems,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetComboBoxValueAt)
        HRESULT ( STDMETHODCALLTYPE *GetComboBoxValueAt )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            DWORD dwItem,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetStringValue)
        HRESULT ( STDMETHODCALLTYPE *SetStringValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR psz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetCheckboxValue)
        HRESULT ( STDMETHODCALLTYPE *SetCheckboxValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetComboBoxSelectedValue)
        HRESULT ( STDMETHODCALLTYPE *SetComboBoxSelectedValue )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, CommandLinkClicked)
        HRESULT ( STDMETHODCALLTYPE *CommandLinkClicked )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ DWORD dwFieldID);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetSerialization)
        HRESULT ( STDMETHODCALLTYPE *GetSerialization )( 
            IConnectableCredentialProviderCredential * This,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE *pcpgsr,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcs,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, ReportResult)
        HRESULT ( STDMETHODCALLTYPE *ReportResult )( 
            IConnectableCredentialProviderCredential * This,
            /* [in] */ NTSTATUS ntsStatus,
            /* [in] */ NTSTATUS ntsSubstatus,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon);
        
        DECLSPEC_XFGVIRT(IConnectableCredentialProviderCredential, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IConnectableCredentialProviderCredential * This,
            /* [annotation][in] */ 
            _In_  IQueryContinueWithStatus *pqcws);
        
        DECLSPEC_XFGVIRT(IConnectableCredentialProviderCredential, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IConnectableCredentialProviderCredential * This);
        
        END_INTERFACE
    } IConnectableCredentialProviderCredentialVtbl;

    interface IConnectableCredentialProviderCredential
    {
        CONST_VTBL struct IConnectableCredentialProviderCredentialVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConnectableCredentialProviderCredential_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConnectableCredentialProviderCredential_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConnectableCredentialProviderCredential_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConnectableCredentialProviderCredential_Advise(This,pcpce)	\
    ( (This)->lpVtbl -> Advise(This,pcpce) ) 

#define IConnectableCredentialProviderCredential_UnAdvise(This)	\
    ( (This)->lpVtbl -> UnAdvise(This) ) 

#define IConnectableCredentialProviderCredential_SetSelected(This,pbAutoLogon)	\
    ( (This)->lpVtbl -> SetSelected(This,pbAutoLogon) ) 

#define IConnectableCredentialProviderCredential_SetDeselected(This)	\
    ( (This)->lpVtbl -> SetDeselected(This) ) 

#define IConnectableCredentialProviderCredential_GetFieldState(This,dwFieldID,pcpfs,pcpfis)	\
    ( (This)->lpVtbl -> GetFieldState(This,dwFieldID,pcpfs,pcpfis) ) 

#define IConnectableCredentialProviderCredential_GetStringValue(This,dwFieldID,ppsz)	\
    ( (This)->lpVtbl -> GetStringValue(This,dwFieldID,ppsz) ) 

#define IConnectableCredentialProviderCredential_GetBitmapValue(This,dwFieldID,phbmp)	\
    ( (This)->lpVtbl -> GetBitmapValue(This,dwFieldID,phbmp) ) 

#define IConnectableCredentialProviderCredential_GetCheckboxValue(This,dwFieldID,pbChecked,ppszLabel)	\
    ( (This)->lpVtbl -> GetCheckboxValue(This,dwFieldID,pbChecked,ppszLabel) ) 

#define IConnectableCredentialProviderCredential_GetSubmitButtonValue(This,dwFieldID,pdwAdjacentTo)	\
    ( (This)->lpVtbl -> GetSubmitButtonValue(This,dwFieldID,pdwAdjacentTo) ) 

#define IConnectableCredentialProviderCredential_GetComboBoxValueCount(This,dwFieldID,pcItems,pdwSelectedItem)	\
    ( (This)->lpVtbl -> GetComboBoxValueCount(This,dwFieldID,pcItems,pdwSelectedItem) ) 

#define IConnectableCredentialProviderCredential_GetComboBoxValueAt(This,dwFieldID,dwItem,ppszItem)	\
    ( (This)->lpVtbl -> GetComboBoxValueAt(This,dwFieldID,dwItem,ppszItem) ) 

#define IConnectableCredentialProviderCredential_SetStringValue(This,dwFieldID,psz)	\
    ( (This)->lpVtbl -> SetStringValue(This,dwFieldID,psz) ) 

#define IConnectableCredentialProviderCredential_SetCheckboxValue(This,dwFieldID,bChecked)	\
    ( (This)->lpVtbl -> SetCheckboxValue(This,dwFieldID,bChecked) ) 

#define IConnectableCredentialProviderCredential_SetComboBoxSelectedValue(This,dwFieldID,dwSelectedItem)	\
    ( (This)->lpVtbl -> SetComboBoxSelectedValue(This,dwFieldID,dwSelectedItem) ) 

#define IConnectableCredentialProviderCredential_CommandLinkClicked(This,dwFieldID)	\
    ( (This)->lpVtbl -> CommandLinkClicked(This,dwFieldID) ) 

#define IConnectableCredentialProviderCredential_GetSerialization(This,pcpgsr,pcpcs,ppszOptionalStatusText,pcpsiOptionalStatusIcon)	\
    ( (This)->lpVtbl -> GetSerialization(This,pcpgsr,pcpcs,ppszOptionalStatusText,pcpsiOptionalStatusIcon) ) 

#define IConnectableCredentialProviderCredential_ReportResult(This,ntsStatus,ntsSubstatus,ppszOptionalStatusText,pcpsiOptionalStatusIcon)	\
    ( (This)->lpVtbl -> ReportResult(This,ntsStatus,ntsSubstatus,ppszOptionalStatusText,pcpsiOptionalStatusIcon) ) 


#define IConnectableCredentialProviderCredential_Connect(This,pqcws)	\
    ( (This)->lpVtbl -> Connect(This,pqcws) ) 

#define IConnectableCredentialProviderCredential_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConnectableCredentialProviderCredential_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderCredentialEvents_INTERFACE_DEFINED__
#define __ICredentialProviderCredentialEvents_INTERFACE_DEFINED__

/* interface ICredentialProviderCredentialEvents */
/* [uuid][ref][object] */ 


EXTERN_C const IID IID_ICredentialProviderCredentialEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa6fa76b-66b7-4b11-95f1-86171118e816")
    ICredentialProviderCredentialEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFieldState( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ CREDENTIAL_PROVIDER_FIELD_STATE cpfs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldInteractiveState( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE cpfis) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldString( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR psz) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldCheckbox( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked,
            /* [in] */ __RPC__in LPCWSTR pszLabel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldBitmap( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ __RPC__in HBITMAP hbmp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldComboBoxSelectedItem( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteFieldComboBoxItem( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AppendFieldComboBoxItem( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldSubmitButton( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwAdjacentTo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCreatingWindow( 
            /* [out] */ __RPC__deref_out_opt HWND *phwndOwner) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderCredentialEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICredentialProviderCredentialEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICredentialProviderCredentialEvents * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldState)
        HRESULT ( STDMETHODCALLTYPE *SetFieldState )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ CREDENTIAL_PROVIDER_FIELD_STATE cpfs);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldInteractiveState)
        HRESULT ( STDMETHODCALLTYPE *SetFieldInteractiveState )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE cpfis);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldString)
        HRESULT ( STDMETHODCALLTYPE *SetFieldString )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR psz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldCheckbox)
        HRESULT ( STDMETHODCALLTYPE *SetFieldCheckbox )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked,
            /* [in] */ __RPC__in LPCWSTR pszLabel);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldBitmap)
        HRESULT ( STDMETHODCALLTYPE *SetFieldBitmap )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ __RPC__in HBITMAP hbmp);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldComboBoxSelectedItem)
        HRESULT ( STDMETHODCALLTYPE *SetFieldComboBoxSelectedItem )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, DeleteFieldComboBoxItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteFieldComboBoxItem )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, AppendFieldComboBoxItem)
        HRESULT ( STDMETHODCALLTYPE *AppendFieldComboBoxItem )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldSubmitButton)
        HRESULT ( STDMETHODCALLTYPE *SetFieldSubmitButton )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwAdjacentTo);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, OnCreatingWindow)
        HRESULT ( STDMETHODCALLTYPE *OnCreatingWindow )( 
            __RPC__in ICredentialProviderCredentialEvents * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwndOwner);
        
        END_INTERFACE
    } ICredentialProviderCredentialEventsVtbl;

    interface ICredentialProviderCredentialEvents
    {
        CONST_VTBL struct ICredentialProviderCredentialEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderCredentialEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderCredentialEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderCredentialEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderCredentialEvents_SetFieldState(This,pcpc,dwFieldID,cpfs)	\
    ( (This)->lpVtbl -> SetFieldState(This,pcpc,dwFieldID,cpfs) ) 

#define ICredentialProviderCredentialEvents_SetFieldInteractiveState(This,pcpc,dwFieldID,cpfis)	\
    ( (This)->lpVtbl -> SetFieldInteractiveState(This,pcpc,dwFieldID,cpfis) ) 

#define ICredentialProviderCredentialEvents_SetFieldString(This,pcpc,dwFieldID,psz)	\
    ( (This)->lpVtbl -> SetFieldString(This,pcpc,dwFieldID,psz) ) 

#define ICredentialProviderCredentialEvents_SetFieldCheckbox(This,pcpc,dwFieldID,bChecked,pszLabel)	\
    ( (This)->lpVtbl -> SetFieldCheckbox(This,pcpc,dwFieldID,bChecked,pszLabel) ) 

#define ICredentialProviderCredentialEvents_SetFieldBitmap(This,pcpc,dwFieldID,hbmp)	\
    ( (This)->lpVtbl -> SetFieldBitmap(This,pcpc,dwFieldID,hbmp) ) 

#define ICredentialProviderCredentialEvents_SetFieldComboBoxSelectedItem(This,pcpc,dwFieldID,dwSelectedItem)	\
    ( (This)->lpVtbl -> SetFieldComboBoxSelectedItem(This,pcpc,dwFieldID,dwSelectedItem) ) 

#define ICredentialProviderCredentialEvents_DeleteFieldComboBoxItem(This,pcpc,dwFieldID,dwItem)	\
    ( (This)->lpVtbl -> DeleteFieldComboBoxItem(This,pcpc,dwFieldID,dwItem) ) 

#define ICredentialProviderCredentialEvents_AppendFieldComboBoxItem(This,pcpc,dwFieldID,pszItem)	\
    ( (This)->lpVtbl -> AppendFieldComboBoxItem(This,pcpc,dwFieldID,pszItem) ) 

#define ICredentialProviderCredentialEvents_SetFieldSubmitButton(This,pcpc,dwFieldID,dwAdjacentTo)	\
    ( (This)->lpVtbl -> SetFieldSubmitButton(This,pcpc,dwFieldID,dwAdjacentTo) ) 

#define ICredentialProviderCredentialEvents_OnCreatingWindow(This,phwndOwner)	\
    ( (This)->lpVtbl -> OnCreatingWindow(This,phwndOwner) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderCredentialEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_credentialprovider_0000_0004 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0004_v0_0_s_ifspec;

#ifndef __ICredentialProvider_INTERFACE_DEFINED__
#define __ICredentialProvider_INTERFACE_DEFINED__

/* interface ICredentialProvider */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_ICredentialProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d27c3481-5a1c-45b2-8aaa-c20ebbe8229e")
    ICredentialProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetUsageScenario( 
            /* [in] */ CREDENTIAL_PROVIDER_USAGE_SCENARIO cpus,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSerialization( 
            /* [annotation][in] */ 
            _In_  const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [annotation][in] */ 
            _In_  ICredentialProviderEvents *pcpe,
            /* [annotation][in] */ 
            _In_  UINT_PTR upAdviseContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnAdvise( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldDescriptorCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldDescriptorAt( 
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR **ppcpfd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCredentialCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDefault,
            /* [annotation][out] */ 
            _Out_  BOOL *pbAutoLogonWithDefault) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCredentialAt( 
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  ICredentialProviderCredential **ppcpc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProvider * This);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, SetUsageScenario)
        HRESULT ( STDMETHODCALLTYPE *SetUsageScenario )( 
            ICredentialProvider * This,
            /* [in] */ CREDENTIAL_PROVIDER_USAGE_SCENARIO cpus,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, SetSerialization)
        HRESULT ( STDMETHODCALLTYPE *SetSerialization )( 
            ICredentialProvider * This,
            /* [annotation][in] */ 
            _In_  const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcs);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            ICredentialProvider * This,
            /* [annotation][in] */ 
            _In_  ICredentialProviderEvents *pcpe,
            /* [annotation][in] */ 
            _In_  UINT_PTR upAdviseContext);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            ICredentialProvider * This);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, GetFieldDescriptorCount)
        HRESULT ( STDMETHODCALLTYPE *GetFieldDescriptorCount )( 
            ICredentialProvider * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, GetFieldDescriptorAt)
        HRESULT ( STDMETHODCALLTYPE *GetFieldDescriptorAt )( 
            ICredentialProvider * This,
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR **ppcpfd);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, GetCredentialCount)
        HRESULT ( STDMETHODCALLTYPE *GetCredentialCount )( 
            ICredentialProvider * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDefault,
            /* [annotation][out] */ 
            _Out_  BOOL *pbAutoLogonWithDefault);
        
        DECLSPEC_XFGVIRT(ICredentialProvider, GetCredentialAt)
        HRESULT ( STDMETHODCALLTYPE *GetCredentialAt )( 
            ICredentialProvider * This,
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  ICredentialProviderCredential **ppcpc);
        
        END_INTERFACE
    } ICredentialProviderVtbl;

    interface ICredentialProvider
    {
        CONST_VTBL struct ICredentialProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProvider_SetUsageScenario(This,cpus,dwFlags)	\
    ( (This)->lpVtbl -> SetUsageScenario(This,cpus,dwFlags) ) 

#define ICredentialProvider_SetSerialization(This,pcpcs)	\
    ( (This)->lpVtbl -> SetSerialization(This,pcpcs) ) 

#define ICredentialProvider_Advise(This,pcpe,upAdviseContext)	\
    ( (This)->lpVtbl -> Advise(This,pcpe,upAdviseContext) ) 

#define ICredentialProvider_UnAdvise(This)	\
    ( (This)->lpVtbl -> UnAdvise(This) ) 

#define ICredentialProvider_GetFieldDescriptorCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetFieldDescriptorCount(This,pdwCount) ) 

#define ICredentialProvider_GetFieldDescriptorAt(This,dwIndex,ppcpfd)	\
    ( (This)->lpVtbl -> GetFieldDescriptorAt(This,dwIndex,ppcpfd) ) 

#define ICredentialProvider_GetCredentialCount(This,pdwCount,pdwDefault,pbAutoLogonWithDefault)	\
    ( (This)->lpVtbl -> GetCredentialCount(This,pdwCount,pdwDefault,pbAutoLogonWithDefault) ) 

#define ICredentialProvider_GetCredentialAt(This,dwIndex,ppcpc)	\
    ( (This)->lpVtbl -> GetCredentialAt(This,dwIndex,ppcpc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProvider_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderEvents_INTERFACE_DEFINED__
#define __ICredentialProviderEvents_INTERFACE_DEFINED__

/* interface ICredentialProviderEvents */
/* [uuid][ref][object] */ 


EXTERN_C const IID IID_ICredentialProviderEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34201e5a-a787-41a3-a5a4-bd6dcf2a854e")
    ICredentialProviderEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CredentialsChanged( 
            /* [in] */ UINT_PTR upAdviseContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICredentialProviderEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICredentialProviderEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICredentialProviderEvents * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderEvents, CredentialsChanged)
        HRESULT ( STDMETHODCALLTYPE *CredentialsChanged )( 
            __RPC__in ICredentialProviderEvents * This,
            /* [in] */ UINT_PTR upAdviseContext);
        
        END_INTERFACE
    } ICredentialProviderEventsVtbl;

    interface ICredentialProviderEvents
    {
        CONST_VTBL struct ICredentialProviderEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderEvents_CredentialsChanged(This,upAdviseContext)	\
    ( (This)->lpVtbl -> CredentialsChanged(This,upAdviseContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderEvents_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderFilter_INTERFACE_DEFINED__
#define __ICredentialProviderFilter_INTERFACE_DEFINED__

/* interface ICredentialProviderFilter */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_ICredentialProviderFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a5da53f9-d475-4080-a120-910c4a739880")
    ICredentialProviderFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Filter( 
            /* [in] */ CREDENTIAL_PROVIDER_USAGE_SCENARIO cpus,
            /* [in] */ DWORD dwFlags,
            /* [annotation][size_is][in] */ 
            _In_reads_(cProviders)  GUID *rgclsidProviders,
            /* [annotation][size_is][out][in] */ 
            _Inout_updates_(cProviders)  BOOL *rgbAllow,
            /* [in] */ DWORD cProviders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateRemoteCredential( 
            /* [annotation][in] */ 
            _In_  const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcsIn,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcsOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderFilter * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderFilter, Filter)
        HRESULT ( STDMETHODCALLTYPE *Filter )( 
            ICredentialProviderFilter * This,
            /* [in] */ CREDENTIAL_PROVIDER_USAGE_SCENARIO cpus,
            /* [in] */ DWORD dwFlags,
            /* [annotation][size_is][in] */ 
            _In_reads_(cProviders)  GUID *rgclsidProviders,
            /* [annotation][size_is][out][in] */ 
            _Inout_updates_(cProviders)  BOOL *rgbAllow,
            /* [in] */ DWORD cProviders);
        
        DECLSPEC_XFGVIRT(ICredentialProviderFilter, UpdateRemoteCredential)
        HRESULT ( STDMETHODCALLTYPE *UpdateRemoteCredential )( 
            ICredentialProviderFilter * This,
            /* [annotation][in] */ 
            _In_  const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcsIn,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcsOut);
        
        END_INTERFACE
    } ICredentialProviderFilterVtbl;

    interface ICredentialProviderFilter
    {
        CONST_VTBL struct ICredentialProviderFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderFilter_Filter(This,cpus,dwFlags,rgclsidProviders,rgbAllow,cProviders)	\
    ( (This)->lpVtbl -> Filter(This,cpus,dwFlags,rgclsidProviders,rgbAllow,cProviders) ) 

#define ICredentialProviderFilter_UpdateRemoteCredential(This,pcpcsIn,pcpcsOut)	\
    ( (This)->lpVtbl -> UpdateRemoteCredential(This,pcpcsIn,pcpcsOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_credentialprovider_0000_0007 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0007_v0_0_s_ifspec;

#ifndef __ICredentialProviderCredential2_INTERFACE_DEFINED__
#define __ICredentialProviderCredential2_INTERFACE_DEFINED__

/* interface ICredentialProviderCredential2 */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_ICredentialProviderCredential2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fd672c54-40ea-4d6e-9b49-cfb1a7507bd7")
    ICredentialProviderCredential2 : public ICredentialProviderCredential
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUserSid( 
            /* [annotation][string][out] */ 
            _Outptr_result_maybenull_  LPWSTR *sid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderCredential2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderCredential2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderCredential2 * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            ICredentialProviderCredential2 * This,
            /* [annotation][in] */ 
            _In_  ICredentialProviderCredentialEvents *pcpce);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            ICredentialProviderCredential2 * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetSelected)
        HRESULT ( STDMETHODCALLTYPE *SetSelected )( 
            ICredentialProviderCredential2 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbAutoLogon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetDeselected)
        HRESULT ( STDMETHODCALLTYPE *SetDeselected )( 
            ICredentialProviderCredential2 * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetFieldState)
        HRESULT ( STDMETHODCALLTYPE *GetFieldState )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_STATE *pcpfs,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE *pcpfis);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetStringValue)
        HRESULT ( STDMETHODCALLTYPE *GetStringValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppsz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetBitmapValue)
        HRESULT ( STDMETHODCALLTYPE *GetBitmapValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Outptr_result_nullonfailure_  HBITMAP *phbmp);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetCheckboxValue)
        HRESULT ( STDMETHODCALLTYPE *GetCheckboxValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  BOOL *pbChecked,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszLabel);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetSubmitButtonValue)
        HRESULT ( STDMETHODCALLTYPE *GetSubmitButtonValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwAdjacentTo);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetComboBoxValueCount)
        HRESULT ( STDMETHODCALLTYPE *GetComboBoxValueCount )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][out] */ 
            _Out_  DWORD *pcItems,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetComboBoxValueAt)
        HRESULT ( STDMETHODCALLTYPE *GetComboBoxValueAt )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            DWORD dwItem,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *ppszItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetStringValue)
        HRESULT ( STDMETHODCALLTYPE *SetStringValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR psz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetCheckboxValue)
        HRESULT ( STDMETHODCALLTYPE *SetCheckboxValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, SetComboBoxSelectedValue)
        HRESULT ( STDMETHODCALLTYPE *SetComboBoxSelectedValue )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, CommandLinkClicked)
        HRESULT ( STDMETHODCALLTYPE *CommandLinkClicked )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ DWORD dwFieldID);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, GetSerialization)
        HRESULT ( STDMETHODCALLTYPE *GetSerialization )( 
            ICredentialProviderCredential2 * This,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE *pcpgsr,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION *pcpcs,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential, ReportResult)
        HRESULT ( STDMETHODCALLTYPE *ReportResult )( 
            ICredentialProviderCredential2 * This,
            /* [in] */ NTSTATUS ntsStatus,
            /* [in] */ NTSTATUS ntsSubstatus,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *ppszOptionalStatusText,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_STATUS_ICON *pcpsiOptionalStatusIcon);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredential2, GetUserSid)
        HRESULT ( STDMETHODCALLTYPE *GetUserSid )( 
            ICredentialProviderCredential2 * This,
            /* [annotation][string][out] */ 
            _Outptr_result_maybenull_  LPWSTR *sid);
        
        END_INTERFACE
    } ICredentialProviderCredential2Vtbl;

    interface ICredentialProviderCredential2
    {
        CONST_VTBL struct ICredentialProviderCredential2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderCredential2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderCredential2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderCredential2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderCredential2_Advise(This,pcpce)	\
    ( (This)->lpVtbl -> Advise(This,pcpce) ) 

#define ICredentialProviderCredential2_UnAdvise(This)	\
    ( (This)->lpVtbl -> UnAdvise(This) ) 

#define ICredentialProviderCredential2_SetSelected(This,pbAutoLogon)	\
    ( (This)->lpVtbl -> SetSelected(This,pbAutoLogon) ) 

#define ICredentialProviderCredential2_SetDeselected(This)	\
    ( (This)->lpVtbl -> SetDeselected(This) ) 

#define ICredentialProviderCredential2_GetFieldState(This,dwFieldID,pcpfs,pcpfis)	\
    ( (This)->lpVtbl -> GetFieldState(This,dwFieldID,pcpfs,pcpfis) ) 

#define ICredentialProviderCredential2_GetStringValue(This,dwFieldID,ppsz)	\
    ( (This)->lpVtbl -> GetStringValue(This,dwFieldID,ppsz) ) 

#define ICredentialProviderCredential2_GetBitmapValue(This,dwFieldID,phbmp)	\
    ( (This)->lpVtbl -> GetBitmapValue(This,dwFieldID,phbmp) ) 

#define ICredentialProviderCredential2_GetCheckboxValue(This,dwFieldID,pbChecked,ppszLabel)	\
    ( (This)->lpVtbl -> GetCheckboxValue(This,dwFieldID,pbChecked,ppszLabel) ) 

#define ICredentialProviderCredential2_GetSubmitButtonValue(This,dwFieldID,pdwAdjacentTo)	\
    ( (This)->lpVtbl -> GetSubmitButtonValue(This,dwFieldID,pdwAdjacentTo) ) 

#define ICredentialProviderCredential2_GetComboBoxValueCount(This,dwFieldID,pcItems,pdwSelectedItem)	\
    ( (This)->lpVtbl -> GetComboBoxValueCount(This,dwFieldID,pcItems,pdwSelectedItem) ) 

#define ICredentialProviderCredential2_GetComboBoxValueAt(This,dwFieldID,dwItem,ppszItem)	\
    ( (This)->lpVtbl -> GetComboBoxValueAt(This,dwFieldID,dwItem,ppszItem) ) 

#define ICredentialProviderCredential2_SetStringValue(This,dwFieldID,psz)	\
    ( (This)->lpVtbl -> SetStringValue(This,dwFieldID,psz) ) 

#define ICredentialProviderCredential2_SetCheckboxValue(This,dwFieldID,bChecked)	\
    ( (This)->lpVtbl -> SetCheckboxValue(This,dwFieldID,bChecked) ) 

#define ICredentialProviderCredential2_SetComboBoxSelectedValue(This,dwFieldID,dwSelectedItem)	\
    ( (This)->lpVtbl -> SetComboBoxSelectedValue(This,dwFieldID,dwSelectedItem) ) 

#define ICredentialProviderCredential2_CommandLinkClicked(This,dwFieldID)	\
    ( (This)->lpVtbl -> CommandLinkClicked(This,dwFieldID) ) 

#define ICredentialProviderCredential2_GetSerialization(This,pcpgsr,pcpcs,ppszOptionalStatusText,pcpsiOptionalStatusIcon)	\
    ( (This)->lpVtbl -> GetSerialization(This,pcpgsr,pcpcs,ppszOptionalStatusText,pcpsiOptionalStatusIcon) ) 

#define ICredentialProviderCredential2_ReportResult(This,ntsStatus,ntsSubstatus,ppszOptionalStatusText,pcpsiOptionalStatusIcon)	\
    ( (This)->lpVtbl -> ReportResult(This,ntsStatus,ntsSubstatus,ppszOptionalStatusText,pcpsiOptionalStatusIcon) ) 


#define ICredentialProviderCredential2_GetUserSid(This,sid)	\
    ( (This)->lpVtbl -> GetUserSid(This,sid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderCredential2_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderCredentialWithFieldOptions_INTERFACE_DEFINED__
#define __ICredentialProviderCredentialWithFieldOptions_INTERFACE_DEFINED__

/* interface ICredentialProviderCredentialWithFieldOptions */
/* [uuid][ref][object][local] */ 


EXTERN_C const IID IID_ICredentialProviderCredentialWithFieldOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DBC6FB30-C843-49E3-A645-573E6F39446A")
    ICredentialProviderCredentialWithFieldOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFieldOptions( 
            /* [in] */ DWORD fieldID,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS *options) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderCredentialWithFieldOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderCredentialWithFieldOptions * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderCredentialWithFieldOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderCredentialWithFieldOptions * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialWithFieldOptions, GetFieldOptions)
        HRESULT ( STDMETHODCALLTYPE *GetFieldOptions )( 
            ICredentialProviderCredentialWithFieldOptions * This,
            /* [in] */ DWORD fieldID,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS *options);
        
        END_INTERFACE
    } ICredentialProviderCredentialWithFieldOptionsVtbl;

    interface ICredentialProviderCredentialWithFieldOptions
    {
        CONST_VTBL struct ICredentialProviderCredentialWithFieldOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderCredentialWithFieldOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderCredentialWithFieldOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderCredentialWithFieldOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderCredentialWithFieldOptions_GetFieldOptions(This,fieldID,options)	\
    ( (This)->lpVtbl -> GetFieldOptions(This,fieldID,options) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderCredentialWithFieldOptions_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderCredentialEvents2_INTERFACE_DEFINED__
#define __ICredentialProviderCredentialEvents2_INTERFACE_DEFINED__

/* interface ICredentialProviderCredentialEvents2 */
/* [uuid][ref][object] */ 


EXTERN_C const IID IID_ICredentialProviderCredentialEvents2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B53C00B6-9922-4B78-B1F4-DDFE774DC39B")
    ICredentialProviderCredentialEvents2 : public ICredentialProviderCredentialEvents
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginFieldUpdates( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndFieldUpdates( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFieldOptions( 
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *credential,
            /* [in] */ DWORD fieldID,
            /* [in] */ CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS options) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderCredentialEvents2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldState)
        HRESULT ( STDMETHODCALLTYPE *SetFieldState )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ CREDENTIAL_PROVIDER_FIELD_STATE cpfs);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldInteractiveState)
        HRESULT ( STDMETHODCALLTYPE *SetFieldInteractiveState )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE cpfis);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldString)
        HRESULT ( STDMETHODCALLTYPE *SetFieldString )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR psz);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldCheckbox)
        HRESULT ( STDMETHODCALLTYPE *SetFieldCheckbox )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ BOOL bChecked,
            /* [in] */ __RPC__in LPCWSTR pszLabel);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldBitmap)
        HRESULT ( STDMETHODCALLTYPE *SetFieldBitmap )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ __RPC__in HBITMAP hbmp);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldComboBoxSelectedItem)
        HRESULT ( STDMETHODCALLTYPE *SetFieldComboBoxSelectedItem )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwSelectedItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, DeleteFieldComboBoxItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteFieldComboBoxItem )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, AppendFieldComboBoxItem)
        HRESULT ( STDMETHODCALLTYPE *AppendFieldComboBoxItem )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [string][in] */ __RPC__in_string LPCWSTR pszItem);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, SetFieldSubmitButton)
        HRESULT ( STDMETHODCALLTYPE *SetFieldSubmitButton )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *pcpc,
            /* [in] */ DWORD dwFieldID,
            /* [in] */ DWORD dwAdjacentTo);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents, OnCreatingWindow)
        HRESULT ( STDMETHODCALLTYPE *OnCreatingWindow )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwndOwner);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents2, BeginFieldUpdates)
        HRESULT ( STDMETHODCALLTYPE *BeginFieldUpdates )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents2, EndFieldUpdates)
        HRESULT ( STDMETHODCALLTYPE *EndFieldUpdates )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderCredentialEvents2, SetFieldOptions)
        HRESULT ( STDMETHODCALLTYPE *SetFieldOptions )( 
            __RPC__in ICredentialProviderCredentialEvents2 * This,
            /* [in] */ __RPC__in_opt ICredentialProviderCredential *credential,
            /* [in] */ DWORD fieldID,
            /* [in] */ CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS options);
        
        END_INTERFACE
    } ICredentialProviderCredentialEvents2Vtbl;

    interface ICredentialProviderCredentialEvents2
    {
        CONST_VTBL struct ICredentialProviderCredentialEvents2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderCredentialEvents2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderCredentialEvents2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderCredentialEvents2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderCredentialEvents2_SetFieldState(This,pcpc,dwFieldID,cpfs)	\
    ( (This)->lpVtbl -> SetFieldState(This,pcpc,dwFieldID,cpfs) ) 

#define ICredentialProviderCredentialEvents2_SetFieldInteractiveState(This,pcpc,dwFieldID,cpfis)	\
    ( (This)->lpVtbl -> SetFieldInteractiveState(This,pcpc,dwFieldID,cpfis) ) 

#define ICredentialProviderCredentialEvents2_SetFieldString(This,pcpc,dwFieldID,psz)	\
    ( (This)->lpVtbl -> SetFieldString(This,pcpc,dwFieldID,psz) ) 

#define ICredentialProviderCredentialEvents2_SetFieldCheckbox(This,pcpc,dwFieldID,bChecked,pszLabel)	\
    ( (This)->lpVtbl -> SetFieldCheckbox(This,pcpc,dwFieldID,bChecked,pszLabel) ) 

#define ICredentialProviderCredentialEvents2_SetFieldBitmap(This,pcpc,dwFieldID,hbmp)	\
    ( (This)->lpVtbl -> SetFieldBitmap(This,pcpc,dwFieldID,hbmp) ) 

#define ICredentialProviderCredentialEvents2_SetFieldComboBoxSelectedItem(This,pcpc,dwFieldID,dwSelectedItem)	\
    ( (This)->lpVtbl -> SetFieldComboBoxSelectedItem(This,pcpc,dwFieldID,dwSelectedItem) ) 

#define ICredentialProviderCredentialEvents2_DeleteFieldComboBoxItem(This,pcpc,dwFieldID,dwItem)	\
    ( (This)->lpVtbl -> DeleteFieldComboBoxItem(This,pcpc,dwFieldID,dwItem) ) 

#define ICredentialProviderCredentialEvents2_AppendFieldComboBoxItem(This,pcpc,dwFieldID,pszItem)	\
    ( (This)->lpVtbl -> AppendFieldComboBoxItem(This,pcpc,dwFieldID,pszItem) ) 

#define ICredentialProviderCredentialEvents2_SetFieldSubmitButton(This,pcpc,dwFieldID,dwAdjacentTo)	\
    ( (This)->lpVtbl -> SetFieldSubmitButton(This,pcpc,dwFieldID,dwAdjacentTo) ) 

#define ICredentialProviderCredentialEvents2_OnCreatingWindow(This,phwndOwner)	\
    ( (This)->lpVtbl -> OnCreatingWindow(This,phwndOwner) ) 


#define ICredentialProviderCredentialEvents2_BeginFieldUpdates(This)	\
    ( (This)->lpVtbl -> BeginFieldUpdates(This) ) 

#define ICredentialProviderCredentialEvents2_EndFieldUpdates(This)	\
    ( (This)->lpVtbl -> EndFieldUpdates(This) ) 

#define ICredentialProviderCredentialEvents2_SetFieldOptions(This,credential,fieldID,options)	\
    ( (This)->lpVtbl -> SetFieldOptions(This,credential,fieldID,options) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderCredentialEvents2_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderUser_INTERFACE_DEFINED__
#define __ICredentialProviderUser_INTERFACE_DEFINED__

/* interface ICredentialProviderUser */
/* [ref][object][uuid][local] */ 


EXTERN_C const IID IID_ICredentialProviderUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13793285-3ea6-40fd-b420-15f47da41fbb")
    ICredentialProviderUser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSid( 
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *sid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProviderID( 
            /* [annotation][out] */ 
            _Out_  GUID *providerID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringValue( 
            /* [annotation][in] */ 
            _In_  REFPROPERTYKEY key,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *stringValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [annotation][in] */ 
            _In_  REFPROPERTYKEY key,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderUser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderUser * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUser, GetSid)
        HRESULT ( STDMETHODCALLTYPE *GetSid )( 
            ICredentialProviderUser * This,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *sid);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUser, GetProviderID)
        HRESULT ( STDMETHODCALLTYPE *GetProviderID )( 
            ICredentialProviderUser * This,
            /* [annotation][out] */ 
            _Out_  GUID *providerID);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUser, GetStringValue)
        HRESULT ( STDMETHODCALLTYPE *GetStringValue )( 
            ICredentialProviderUser * This,
            /* [annotation][in] */ 
            _In_  REFPROPERTYKEY key,
            /* [annotation][string][out] */ 
            _Outptr_result_nullonfailure_  LPWSTR *stringValue);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUser, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            ICredentialProviderUser * This,
            /* [annotation][in] */ 
            _In_  REFPROPERTYKEY key,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *value);
        
        END_INTERFACE
    } ICredentialProviderUserVtbl;

    interface ICredentialProviderUser
    {
        CONST_VTBL struct ICredentialProviderUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderUser_GetSid(This,sid)	\
    ( (This)->lpVtbl -> GetSid(This,sid) ) 

#define ICredentialProviderUser_GetProviderID(This,providerID)	\
    ( (This)->lpVtbl -> GetProviderID(This,providerID) ) 

#define ICredentialProviderUser_GetStringValue(This,key,stringValue)	\
    ( (This)->lpVtbl -> GetStringValue(This,key,stringValue) ) 

#define ICredentialProviderUser_GetValue(This,key,value)	\
    ( (This)->lpVtbl -> GetValue(This,key,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderUser_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_credentialprovider_0000_0011 */
/* [local] */ 

DEFINE_GUID(Identity_LocalUserProvider, 0xA198529B, 0x730F, 0x4089, 0xB6, 0x46, 0xA1, 0x25, 0x57, 0xF5, 0x66, 0x5E);


extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0011_v0_0_s_ifspec;

#ifndef __ICredentialProviderUserArray_INTERFACE_DEFINED__
#define __ICredentialProviderUserArray_INTERFACE_DEFINED__

/* interface ICredentialProviderUserArray */
/* [ref][object][uuid][local] */ 


EXTERN_C const IID IID_ICredentialProviderUserArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90C119AE-0F18-4520-A1F1-114366A40FE8")
    ICredentialProviderUserArray : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetProviderFilter( 
            /* [in] */ REFGUID guidProviderToFilterTo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAccountOptions( 
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS *credentialProviderAccountOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *userCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ DWORD userIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  ICredentialProviderUser **user) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderUserArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderUserArray * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderUserArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderUserArray * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUserArray, SetProviderFilter)
        HRESULT ( STDMETHODCALLTYPE *SetProviderFilter )( 
            ICredentialProviderUserArray * This,
            /* [in] */ REFGUID guidProviderToFilterTo);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUserArray, GetAccountOptions)
        HRESULT ( STDMETHODCALLTYPE *GetAccountOptions )( 
            ICredentialProviderUserArray * This,
            /* [annotation][out] */ 
            _Out_  CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS *credentialProviderAccountOptions);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUserArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            ICredentialProviderUserArray * This,
            /* [annotation][out] */ 
            _Out_  DWORD *userCount);
        
        DECLSPEC_XFGVIRT(ICredentialProviderUserArray, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            ICredentialProviderUserArray * This,
            /* [in] */ DWORD userIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  ICredentialProviderUser **user);
        
        END_INTERFACE
    } ICredentialProviderUserArrayVtbl;

    interface ICredentialProviderUserArray
    {
        CONST_VTBL struct ICredentialProviderUserArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderUserArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderUserArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderUserArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderUserArray_SetProviderFilter(This,guidProviderToFilterTo)	\
    ( (This)->lpVtbl -> SetProviderFilter(This,guidProviderToFilterTo) ) 

#define ICredentialProviderUserArray_GetAccountOptions(This,credentialProviderAccountOptions)	\
    ( (This)->lpVtbl -> GetAccountOptions(This,credentialProviderAccountOptions) ) 

#define ICredentialProviderUserArray_GetCount(This,userCount)	\
    ( (This)->lpVtbl -> GetCount(This,userCount) ) 

#define ICredentialProviderUserArray_GetAt(This,userIndex,user)	\
    ( (This)->lpVtbl -> GetAt(This,userIndex,user) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderUserArray_INTERFACE_DEFINED__ */


#ifndef __ICredentialProviderSetUserArray_INTERFACE_DEFINED__
#define __ICredentialProviderSetUserArray_INTERFACE_DEFINED__

/* interface ICredentialProviderSetUserArray */
/* [ref][object][uuid][local] */ 


EXTERN_C const IID IID_ICredentialProviderSetUserArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("095c1484-1c0c-4388-9c6d-500e61bf84bd")
    ICredentialProviderSetUserArray : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetUserArray( 
            /* [annotation][in] */ 
            _In_  ICredentialProviderUserArray *users) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICredentialProviderSetUserArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICredentialProviderSetUserArray * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICredentialProviderSetUserArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICredentialProviderSetUserArray * This);
        
        DECLSPEC_XFGVIRT(ICredentialProviderSetUserArray, SetUserArray)
        HRESULT ( STDMETHODCALLTYPE *SetUserArray )( 
            ICredentialProviderSetUserArray * This,
            /* [annotation][in] */ 
            _In_  ICredentialProviderUserArray *users);
        
        END_INTERFACE
    } ICredentialProviderSetUserArrayVtbl;

    interface ICredentialProviderSetUserArray
    {
        CONST_VTBL struct ICredentialProviderSetUserArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICredentialProviderSetUserArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICredentialProviderSetUserArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICredentialProviderSetUserArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICredentialProviderSetUserArray_SetUserArray(This,users)	\
    ( (This)->lpVtbl -> SetUserArray(This,users) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICredentialProviderSetUserArray_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_credentialprovider_0000_0013 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0013_v0_0_s_ifspec;


#ifndef __CredentialProviders_LIBRARY_DEFINED__
#define __CredentialProviders_LIBRARY_DEFINED__

/* library CredentialProviders */
/* [uuid] */ 


EXTERN_C const IID LIBID_CredentialProviders;

EXTERN_C const CLSID CLSID_PasswordCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("60b78e88-ead8-445c-9cfd-0b87f74ea6cd")
PasswordCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_V1PasswordCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("6f45dc1e-5384-457a-bc13-2cd81b0d28ed")
V1PasswordCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_PINLogonCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("cb82ea12-9f71-446d-89e1-8d0924e1256e")
PINLogonCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_NPCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("3dd6bec0-8193-4ffe-ae25-e08e39ea4063")
NPCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_SmartcardCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("8FD7E19C-3BF7-489B-A72C-846AB3678C96")
SmartcardCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_V1SmartcardCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("8bf9a910-a8ff-457f-999f-a5ca10b4a885")
V1SmartcardCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_SmartcardPinProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("94596c7e-3744-41ce-893e-bbf09122f76a")
SmartcardPinProvider;
#endif

EXTERN_C const CLSID CLSID_SmartcardReaderSelectionProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("1b283861-754f-4022-ad47-a5eaaa618894")
SmartcardReaderSelectionProvider;
#endif

EXTERN_C const CLSID CLSID_SmartcardWinRTProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("1ee7337f-85ac-45e2-a23c-37c753209769")
SmartcardWinRTProvider;
#endif

EXTERN_C const CLSID CLSID_GenericCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("25CBB996-92ED-457e-B28C-4774084BD562")
GenericCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_RASProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("5537E283-B1E7-4EF8-9C6E-7AB0AFE5056D")
RASProvider;
#endif

EXTERN_C const CLSID CLSID_OnexCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("07AA0886-CC8D-4e19-A410-1C75AF686E62")
OnexCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_OnexPlapSmartcardCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("33c86cd6-705f-4ba1-9adb-67070b837775")
OnexPlapSmartcardCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_VaultProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("503739d0-4c5e-4cfd-b3ba-d881334f0df2")
VaultProvider;
#endif

EXTERN_C const CLSID CLSID_WinBioCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("BEC09223-B018-416D-A0AC-523971B639F5")
WinBioCredentialProvider;
#endif

EXTERN_C const CLSID CLSID_V1WinBioCredentialProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("AC3AC249-E820-4343-A65B-377AC634DC09")
V1WinBioCredentialProvider;
#endif
#endif /* __CredentialProviders_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_credentialprovider_0000_0014 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_credentialprovider_0000_0014_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


