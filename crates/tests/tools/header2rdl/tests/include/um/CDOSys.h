

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

#ifndef __cdo_h__
#define __cdo_h__
#include "cdosysstr.h"
#if defined __cplusplus && !defined CDO_NO_NAMESPACE
namespace CDO {
#else
#undef IDataSource
#endif

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

#ifndef __IDataSource_FWD_DEFINED__
#define __IDataSource_FWD_DEFINED__
typedef interface IDataSource IDataSource;

#endif 	/* __IDataSource_FWD_DEFINED__ */


#ifndef __IMessage_FWD_DEFINED__
#define __IMessage_FWD_DEFINED__
typedef interface IMessage IMessage;

#endif 	/* __IMessage_FWD_DEFINED__ */


#ifndef __IBodyPart_FWD_DEFINED__
#define __IBodyPart_FWD_DEFINED__
typedef interface IBodyPart IBodyPart;

#endif 	/* __IBodyPart_FWD_DEFINED__ */


#ifndef __IConfiguration_FWD_DEFINED__
#define __IConfiguration_FWD_DEFINED__
typedef interface IConfiguration IConfiguration;

#endif 	/* __IConfiguration_FWD_DEFINED__ */


#ifndef __IMessages_FWD_DEFINED__
#define __IMessages_FWD_DEFINED__
typedef interface IMessages IMessages;

#endif 	/* __IMessages_FWD_DEFINED__ */


#ifndef __IDropDirectory_FWD_DEFINED__
#define __IDropDirectory_FWD_DEFINED__
typedef interface IDropDirectory IDropDirectory;

#endif 	/* __IDropDirectory_FWD_DEFINED__ */


#ifndef __IBodyParts_FWD_DEFINED__
#define __IBodyParts_FWD_DEFINED__
typedef interface IBodyParts IBodyParts;

#endif 	/* __IBodyParts_FWD_DEFINED__ */


#ifndef __ISMTPScriptConnector_FWD_DEFINED__
#define __ISMTPScriptConnector_FWD_DEFINED__
typedef interface ISMTPScriptConnector ISMTPScriptConnector;

#endif 	/* __ISMTPScriptConnector_FWD_DEFINED__ */


#ifndef __INNTPEarlyScriptConnector_FWD_DEFINED__
#define __INNTPEarlyScriptConnector_FWD_DEFINED__
typedef interface INNTPEarlyScriptConnector INNTPEarlyScriptConnector;

#endif 	/* __INNTPEarlyScriptConnector_FWD_DEFINED__ */


#ifndef __INNTPPostScriptConnector_FWD_DEFINED__
#define __INNTPPostScriptConnector_FWD_DEFINED__
typedef interface INNTPPostScriptConnector INNTPPostScriptConnector;

#endif 	/* __INNTPPostScriptConnector_FWD_DEFINED__ */


#ifndef __INNTPFinalScriptConnector_FWD_DEFINED__
#define __INNTPFinalScriptConnector_FWD_DEFINED__
typedef interface INNTPFinalScriptConnector INNTPFinalScriptConnector;

#endif 	/* __INNTPFinalScriptConnector_FWD_DEFINED__ */


#ifndef __ISMTPOnArrival_FWD_DEFINED__
#define __ISMTPOnArrival_FWD_DEFINED__
typedef interface ISMTPOnArrival ISMTPOnArrival;

#endif 	/* __ISMTPOnArrival_FWD_DEFINED__ */


#ifndef __INNTPOnPostEarly_FWD_DEFINED__
#define __INNTPOnPostEarly_FWD_DEFINED__
typedef interface INNTPOnPostEarly INNTPOnPostEarly;

#endif 	/* __INNTPOnPostEarly_FWD_DEFINED__ */


#ifndef __INNTPOnPost_FWD_DEFINED__
#define __INNTPOnPost_FWD_DEFINED__
typedef interface INNTPOnPost INNTPOnPost;

#endif 	/* __INNTPOnPost_FWD_DEFINED__ */


#ifndef __INNTPOnPostFinal_FWD_DEFINED__
#define __INNTPOnPostFinal_FWD_DEFINED__
typedef interface INNTPOnPostFinal INNTPOnPostFinal;

#endif 	/* __INNTPOnPostFinal_FWD_DEFINED__ */


#ifndef __IProxyObject_FWD_DEFINED__
#define __IProxyObject_FWD_DEFINED__
typedef interface IProxyObject IProxyObject;

#endif 	/* __IProxyObject_FWD_DEFINED__ */


#ifndef __IGetInterface_FWD_DEFINED__
#define __IGetInterface_FWD_DEFINED__
typedef interface IGetInterface IGetInterface;

#endif 	/* __IGetInterface_FWD_DEFINED__ */


#ifndef __IBodyParts_FWD_DEFINED__
#define __IBodyParts_FWD_DEFINED__
typedef interface IBodyParts IBodyParts;

#endif 	/* __IBodyParts_FWD_DEFINED__ */


#ifndef __IMessages_FWD_DEFINED__
#define __IMessages_FWD_DEFINED__
typedef interface IMessages IMessages;

#endif 	/* __IMessages_FWD_DEFINED__ */


#ifndef __Message_FWD_DEFINED__
#define __Message_FWD_DEFINED__

#ifdef __cplusplus
typedef class Message Message;
#else
typedef struct Message Message;
#endif /* __cplusplus */

#endif 	/* __Message_FWD_DEFINED__ */


#ifndef __Configuration_FWD_DEFINED__
#define __Configuration_FWD_DEFINED__

#ifdef __cplusplus
typedef class Configuration Configuration;
#else
typedef struct Configuration Configuration;
#endif /* __cplusplus */

#endif 	/* __Configuration_FWD_DEFINED__ */


#ifndef __DropDirectory_FWD_DEFINED__
#define __DropDirectory_FWD_DEFINED__

#ifdef __cplusplus
typedef class DropDirectory DropDirectory;
#else
typedef struct DropDirectory DropDirectory;
#endif /* __cplusplus */

#endif 	/* __DropDirectory_FWD_DEFINED__ */


#ifndef __SMTPConnector_FWD_DEFINED__
#define __SMTPConnector_FWD_DEFINED__

#ifdef __cplusplus
typedef class SMTPConnector SMTPConnector;
#else
typedef struct SMTPConnector SMTPConnector;
#endif /* __cplusplus */

#endif 	/* __SMTPConnector_FWD_DEFINED__ */


#ifndef __NNTPEarlyConnector_FWD_DEFINED__
#define __NNTPEarlyConnector_FWD_DEFINED__

#ifdef __cplusplus
typedef class NNTPEarlyConnector NNTPEarlyConnector;
#else
typedef struct NNTPEarlyConnector NNTPEarlyConnector;
#endif /* __cplusplus */

#endif 	/* __NNTPEarlyConnector_FWD_DEFINED__ */


#ifndef __NNTPPostConnector_FWD_DEFINED__
#define __NNTPPostConnector_FWD_DEFINED__

#ifdef __cplusplus
typedef class NNTPPostConnector NNTPPostConnector;
#else
typedef struct NNTPPostConnector NNTPPostConnector;
#endif /* __cplusplus */

#endif 	/* __NNTPPostConnector_FWD_DEFINED__ */


#ifndef __NNTPFinalConnector_FWD_DEFINED__
#define __NNTPFinalConnector_FWD_DEFINED__

#ifdef __cplusplus
typedef class NNTPFinalConnector NNTPFinalConnector;
#else
typedef struct NNTPFinalConnector NNTPFinalConnector;
#endif /* __cplusplus */

#endif 	/* __NNTPFinalConnector_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#ifndef __cplusplus // X5-101346
typedef interface ADOError ADOError;
typedef interface ADOErrors ADOErrors;
typedef interface _ADOCommand _ADOCommand;
typedef interface _ADOConnection _ADOConnection;
typedef interface _ADORecord _ADORecord;
typedef interface IRecADOFields IRecADOFields;
typedef interface _ADOStream _ADOStream;
typedef interface _ADORecordset _ADORecordset;
typedef interface ADOField ADOField;
typedef interface _ADOField _ADOField;
typedef interface ADOFields ADOFields;
typedef interface _ADOParameter _ADOParameter;
typedef interface ADOParameters ADOParameters;
typedef interface ADOProperty ADOProperty;
typedef interface ADOProperties ADOProperties;
#endif // __cplusplus
#include "adoint_backcompat.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_cdo_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






typedef /* [helpstring] */ 
enum CdoConfigSource
    {
        cdoDefaults	= -1,
        cdoIIS	= 1,
        cdoOutlookExpress	= 2
    } 	CdoConfigSource;

typedef /* [helpstring] */ 
enum CdoDSNOptions
    {
        cdoDSNDefault	= 0,
        cdoDSNNever	= 1,
        cdoDSNFailure	= 2,
        cdoDSNSuccess	= 4,
        cdoDSNDelay	= 8,
        cdoDSNSuccessFailOrDelay	= 14
    } 	CdoDSNOptions;

typedef /* [helpstring] */ 
enum CdoEventStatus
    {
        cdoRunNextSink	= 0,
        cdoSkipRemainingSinks	= 1
    } 	CdoEventStatus;

typedef /* [helpstring] */ 
enum CdoEventType
    {
        cdoSMTPOnArrival	= 1,
        cdoNNTPOnPostEarly	= 2,
        cdoNNTPOnPost	= 3,
        cdoNNTPOnPostFinal	= 4
    } 	CdoEventType;

typedef /* [helpstring] */ 
enum cdoImportanceValues
    {
        cdoLow	= 0,
        cdoNormal	= 1,
        cdoHigh	= 2
    } 	cdoImportanceValues;

typedef /* [helpstring] */ 
enum CdoMessageStat
    {
        cdoStatSuccess	= 0,
        cdoStatAbortDelivery	= 2,
        cdoStatBadMail	= 3
    } 	CdoMessageStat;

typedef /* [helpstring] */ 
enum CdoMHTMLFlags
    {
        cdoSuppressNone	= 0,
        cdoSuppressImages	= 1,
        cdoSuppressBGSounds	= 2,
        cdoSuppressFrames	= 4,
        cdoSuppressObjects	= 8,
        cdoSuppressStyleSheets	= 16,
        cdoSuppressAll	= 31
    } 	CdoMHTMLFlags;

typedef /* [helpstring] */ 
enum CdoNNTPProcessingField
    {
        cdoPostMessage	= 1,
        cdoProcessControl	= 2,
        cdoProcessModerator	= 4
    } 	CdoNNTPProcessingField;

typedef /* [helpstring] */ 
enum CdoPostUsing
    {
        cdoPostUsingPickup	= 1,
        cdoPostUsingPort	= 2
    } 	CdoPostUsing;

typedef 
enum cdoPriorityValues
    {
        cdoPriorityNonUrgent	= -1,
        cdoPriorityNormal	= 0,
        cdoPriorityUrgent	= 1
    } 	cdoPriorityValues;

typedef /* [helpstring] */ 
enum CdoProtocolsAuthentication
    {
        cdoAnonymous	= 0,
        cdoBasic	= 1,
        cdoNTLM	= 2
    } 	CdoProtocolsAuthentication;

typedef /* [helpstring] */ 
enum CdoReferenceType
    {
        cdoRefTypeId	= 0,
        cdoRefTypeLocation	= 1
    } 	CdoReferenceType;

typedef /* [helpstring] */ 
enum CdoSendUsing
    {
        cdoSendUsingPickup	= 1,
        cdoSendUsingPort	= 2
    } 	CdoSendUsing;

typedef /* [helpstring] */ 
enum cdoSensitivityValues
    {
        cdoSensitivityNone	= 0,
        cdoPersonal	= 1,
        cdoPrivate	= 2,
        cdoCompanyConfidential	= 3
    } 	cdoSensitivityValues;

typedef /* [helpstring] */ 
enum CdoTimeZoneId
    {
        cdoUTC	= 0,
        cdoGMT	= 1,
        cdoSarajevo	= 2,
        cdoParis	= 3,
        cdoBerlin	= 4,
        cdoEasternEurope	= 5,
        cdoPrague	= 6,
        cdoAthens	= 7,
        cdoBrasilia	= 8,
        cdoAtlanticCanada	= 9,
        cdoEastern	= 10,
        cdoCentral	= 11,
        cdoMountain	= 12,
        cdoPacific	= 13,
        cdoAlaska	= 14,
        cdoHawaii	= 15,
        cdoMidwayIsland	= 16,
        cdoWellington	= 17,
        cdoBrisbane	= 18,
        cdoAdelaide	= 19,
        cdoTokyo	= 20,
        cdoSingapore	= 21,
        cdoBangkok	= 22,
        cdoBombay	= 23,
        cdoAbuDhabi	= 24,
        cdoTehran	= 25,
        cdoBaghdad	= 26,
        cdoIsrael	= 27,
        cdoNewfoundland	= 28,
        cdoAzores	= 29,
        cdoMidAtlantic	= 30,
        cdoMonrovia	= 31,
        cdoBuenosAires	= 32,
        cdoCaracas	= 33,
        cdoIndiana	= 34,
        cdoBogota	= 35,
        cdoSaskatchewan	= 36,
        cdoMexicoCity	= 37,
        cdoArizona	= 38,
        cdoEniwetok	= 39,
        cdoFiji	= 40,
        cdoMagadan	= 41,
        cdoHobart	= 42,
        cdoGuam	= 43,
        cdoDarwin	= 44,
        cdoBeijing	= 45,
        cdoAlmaty	= 46,
        cdoIslamabad	= 47,
        cdoKabul	= 48,
        cdoCairo	= 49,
        cdoHarare	= 50,
        cdoMoscow	= 51,
        cdoFloating	= 52,
        cdoCapeVerde	= 53,
        cdoCaucasus	= 54,
        cdoCentralAmerica	= 55,
        cdoEastAfrica	= 56,
        cdoMelbourne	= 57,
        cdoEkaterinburg	= 58,
        cdoHelsinki	= 59,
        cdoGreenland	= 60,
        cdoRangoon	= 61,
        cdoNepal	= 62,
        cdoIrkutsk	= 63,
        cdoKrasnoyarsk	= 64,
        cdoSantiago	= 65,
        cdoSriLanka	= 66,
        cdoTonga	= 67,
        cdoVladivostok	= 68,
        cdoWestCentralAfrica	= 69,
        cdoYakutsk	= 70,
        cdoDhaka	= 71,
        cdoSeoul	= 72,
        cdoPerth	= 73,
        cdoArab	= 74,
        cdoTaipei	= 75,
        cdoSydney2000	= 76,
        cdoChihuahua	= 77,
        cdoCanberraCommonwealthGames2006	= 78,
        cdoAdelaideCommonwealthGames2006	= 79,
        cdoHobartCommonwealthGames2006	= 80,
        cdoTijuana	= 81,
        cdoInvalidTimeZone	= 82
    } 	CdoTimeZoneId;



extern RPC_IF_HANDLE __MIDL_itf_cdo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cdo_0000_0000_v0_0_s_ifspec;

#ifndef __IDataSource_INTERFACE_DEFINED__
#define __IDataSource_INTERFACE_DEFINED__

/* interface IDataSource */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IDataSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000029-8B95-11D1-82DB-00C04FB1625D")
    IDataSource : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SourceClass( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *varSourceClass) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **varSource) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_IsDirty( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pIsDirty) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_IsDirty( 
            /* [in] */ VARIANT_BOOL varIsDirty) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SourceURL( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *varSourceURL) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__deref_out_opt _Connection **varActiveConnection) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveToObject( 
            /* [in] */ __RPC__in_opt IUnknown *Source,
            /* [in] */ __RPC__in BSTR InterfaceName) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenObject( 
            /* [in] */ __RPC__in_opt IUnknown *Source,
            /* [in] */ __RPC__in BSTR InterfaceName) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveTo( 
            /* [in] */ __RPC__in BSTR SourceURL,
            /* [defaultvalue][in] */ __RPC__in_opt IDispatch *ActiveConnection,
            /* [optional][in] */ ConnectModeEnum Mode,
            /* [optional][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [optional][in] */ RecordOpenOptionsEnum Options,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ __RPC__in BSTR SourceURL,
            /* [defaultvalue][in] */ __RPC__in_opt IDispatch *ActiveConnection,
            /* [optional][in] */ ConnectModeEnum Mode,
            /* [defaultvalue][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [optional][in] */ RecordOpenOptionsEnum Options,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveToContainer( 
            /* [in] */ __RPC__in BSTR ContainerURL,
            /* [defaultvalue][in] */ __RPC__in_opt IDispatch *ActiveConnection,
            /* [optional][in] */ ConnectModeEnum Mode,
            /* [optional][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [optional][in] */ RecordOpenOptionsEnum Options,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataSource * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDataSource * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDataSource * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDataSource * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDataSource, get_SourceClass)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SourceClass )( 
            __RPC__in IDataSource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *varSourceClass);
        
        DECLSPEC_XFGVIRT(IDataSource, get_Source)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in IDataSource * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **varSource);
        
        DECLSPEC_XFGVIRT(IDataSource, get_IsDirty)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_IsDirty )( 
            __RPC__in IDataSource * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pIsDirty);
        
        DECLSPEC_XFGVIRT(IDataSource, put_IsDirty)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_IsDirty )( 
            __RPC__in IDataSource * This,
            /* [in] */ VARIANT_BOOL varIsDirty);
        
        DECLSPEC_XFGVIRT(IDataSource, get_SourceURL)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SourceURL )( 
            __RPC__in IDataSource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *varSourceURL);
        
        DECLSPEC_XFGVIRT(IDataSource, get_ActiveConnection)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in IDataSource * This,
            /* [retval][out] */ __RPC__deref_out_opt _Connection **varActiveConnection);
        
        DECLSPEC_XFGVIRT(IDataSource, SaveToObject)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveToObject )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in_opt IUnknown *Source,
            /* [in] */ __RPC__in BSTR InterfaceName);
        
        DECLSPEC_XFGVIRT(IDataSource, OpenObject)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OpenObject )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in_opt IUnknown *Source,
            /* [in] */ __RPC__in BSTR InterfaceName);
        
        DECLSPEC_XFGVIRT(IDataSource, SaveTo)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveTo )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in BSTR SourceURL,
            /* [defaultvalue][in] */ __RPC__in_opt IDispatch *ActiveConnection,
            /* [optional][in] */ ConnectModeEnum Mode,
            /* [optional][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [optional][in] */ RecordOpenOptionsEnum Options,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password);
        
        DECLSPEC_XFGVIRT(IDataSource, Open)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in BSTR SourceURL,
            /* [defaultvalue][in] */ __RPC__in_opt IDispatch *ActiveConnection,
            /* [optional][in] */ ConnectModeEnum Mode,
            /* [defaultvalue][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [optional][in] */ RecordOpenOptionsEnum Options,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password);
        
        DECLSPEC_XFGVIRT(IDataSource, Save)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IDataSource * This);
        
        DECLSPEC_XFGVIRT(IDataSource, SaveToContainer)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveToContainer )( 
            __RPC__in IDataSource * This,
            /* [in] */ __RPC__in BSTR ContainerURL,
            /* [defaultvalue][in] */ __RPC__in_opt IDispatch *ActiveConnection,
            /* [optional][in] */ ConnectModeEnum Mode,
            /* [optional][in] */ RecordCreateOptionsEnum CreateOptions,
            /* [optional][in] */ RecordOpenOptionsEnum Options,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password);
        
        END_INTERFACE
    } IDataSourceVtbl;

    interface IDataSource
    {
        CONST_VTBL struct IDataSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataSource_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDataSource_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDataSource_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDataSource_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDataSource_get_SourceClass(This,varSourceClass)	\
    ( (This)->lpVtbl -> get_SourceClass(This,varSourceClass) ) 

#define IDataSource_get_Source(This,varSource)	\
    ( (This)->lpVtbl -> get_Source(This,varSource) ) 

#define IDataSource_get_IsDirty(This,pIsDirty)	\
    ( (This)->lpVtbl -> get_IsDirty(This,pIsDirty) ) 

#define IDataSource_put_IsDirty(This,varIsDirty)	\
    ( (This)->lpVtbl -> put_IsDirty(This,varIsDirty) ) 

#define IDataSource_get_SourceURL(This,varSourceURL)	\
    ( (This)->lpVtbl -> get_SourceURL(This,varSourceURL) ) 

#define IDataSource_get_ActiveConnection(This,varActiveConnection)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,varActiveConnection) ) 

#define IDataSource_SaveToObject(This,Source,InterfaceName)	\
    ( (This)->lpVtbl -> SaveToObject(This,Source,InterfaceName) ) 

#define IDataSource_OpenObject(This,Source,InterfaceName)	\
    ( (This)->lpVtbl -> OpenObject(This,Source,InterfaceName) ) 

#define IDataSource_SaveTo(This,SourceURL,ActiveConnection,Mode,CreateOptions,Options,UserName,Password)	\
    ( (This)->lpVtbl -> SaveTo(This,SourceURL,ActiveConnection,Mode,CreateOptions,Options,UserName,Password) ) 

#define IDataSource_Open(This,SourceURL,ActiveConnection,Mode,CreateOptions,Options,UserName,Password)	\
    ( (This)->lpVtbl -> Open(This,SourceURL,ActiveConnection,Mode,CreateOptions,Options,UserName,Password) ) 

#define IDataSource_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IDataSource_SaveToContainer(This,ContainerURL,ActiveConnection,Mode,CreateOptions,Options,UserName,Password)	\
    ( (This)->lpVtbl -> SaveToContainer(This,ContainerURL,ActiveConnection,Mode,CreateOptions,Options,UserName,Password) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataSource_INTERFACE_DEFINED__ */


#ifndef __IMessage_INTERFACE_DEFINED__
#define __IMessage_INTERFACE_DEFINED__

/* interface IMessage */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IMessage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000020-8B95-11D1-82DB-00C04FB1625D")
    IMessage : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BCC( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pBCC) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_BCC( 
            /* [in] */ __RPC__in BSTR varBCC) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CC( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCC) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_CC( 
            /* [in] */ __RPC__in BSTR varCC) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FollowUpTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pFollowUpTo) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_FollowUpTo( 
            /* [in] */ __RPC__in BSTR varFollowUpTo) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_From( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pFrom) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_From( 
            /* [in] */ __RPC__in BSTR varFrom) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Keywords( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pKeywords) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Keywords( 
            /* [in] */ __RPC__in BSTR varKeywords) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MimeFormatted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pMimeFormatted) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MimeFormatted( 
            /* [in] */ VARIANT_BOOL varMimeFormatted) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Newsgroups( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pNewsgroups) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Newsgroups( 
            /* [in] */ __RPC__in BSTR varNewsgroups) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Organization( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pOrganization) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Organization( 
            /* [in] */ __RPC__in BSTR varOrganization) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ReceivedTime( 
            /* [retval][out] */ __RPC__out DATE *varReceivedTime) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ReplyTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pReplyTo) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ReplyTo( 
            /* [in] */ __RPC__in BSTR varReplyTo) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DSNOptions( 
            /* [retval][out] */ __RPC__out CdoDSNOptions *pDSNOptions) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DSNOptions( 
            /* [in] */ CdoDSNOptions varDSNOptions) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SentOn( 
            /* [retval][out] */ __RPC__out DATE *varSentOn) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Subject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSubject) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Subject( 
            /* [in] */ __RPC__in BSTR varSubject) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_To( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTo) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_To( 
            /* [in] */ __RPC__in BSTR varTo) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TextBody( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTextBody) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_TextBody( 
            /* [in] */ __RPC__in BSTR varTextBody) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HTMLBody( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pHTMLBody) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_HTMLBody( 
            /* [in] */ __RPC__in BSTR varHTMLBody) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Attachments( 
            /* [retval][out] */ __RPC__deref_out_opt IBodyParts **varAttachments) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Sender( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSender) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Sender( 
            /* [in] */ __RPC__in BSTR varSender) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Configuration( 
            /* [retval][out] */ __RPC__deref_out_opt IConfiguration **pConfiguration) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Configuration( 
            /* [in] */ __RPC__in_opt IConfiguration *varConfiguration) = 0;
        
        virtual /* [helpcontext][helpstring][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Configuration( 
            /* [in] */ __RPC__in_opt IConfiguration *varConfiguration) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AutoGenerateTextBody( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pAutoGenerateTextBody) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AutoGenerateTextBody( 
            /* [in] */ VARIANT_BOOL varAutoGenerateTextBody) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_EnvelopeFields( 
            /* [retval][out] */ __RPC__deref_out_opt Fields **varEnvelopeFields) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TextBodyPart( 
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varTextBodyPart) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HTMLBodyPart( 
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varHTMLBodyPart) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BodyPart( 
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varBodyPart) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DataSource( 
            /* [retval][out] */ __RPC__deref_out_opt IDataSource **varDataSource) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Fields( 
            /* [retval][out] */ __RPC__deref_out_opt Fields **varFields) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MDNRequested( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pMDNRequested) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MDNRequested( 
            /* [in] */ VARIANT_BOOL varMDNRequested) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddRelatedBodyPart( 
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Reference,
            /* [in] */ CdoReferenceType ReferenceType,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppBody) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddAttachment( 
            /* [in] */ __RPC__in BSTR URL,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppBody) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateMHTMLBody( 
            /* [in] */ __RPC__in BSTR URL,
            /* [defaultvalue][in] */ CdoMHTMLFlags Flags,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Forward( 
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Post( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE PostReply( 
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Reply( 
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ReplyAll( 
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Send( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInterface( 
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMessageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMessage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMessage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMessage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMessage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMessage * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMessage, get_BCC)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BCC )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pBCC);
        
        DECLSPEC_XFGVIRT(IMessage, put_BCC)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BCC )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varBCC);
        
        DECLSPEC_XFGVIRT(IMessage, get_CC)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CC )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCC);
        
        DECLSPEC_XFGVIRT(IMessage, put_CC)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CC )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varCC);
        
        DECLSPEC_XFGVIRT(IMessage, get_FollowUpTo)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FollowUpTo )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pFollowUpTo);
        
        DECLSPEC_XFGVIRT(IMessage, put_FollowUpTo)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_FollowUpTo )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varFollowUpTo);
        
        DECLSPEC_XFGVIRT(IMessage, get_From)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_From )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pFrom);
        
        DECLSPEC_XFGVIRT(IMessage, put_From)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_From )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varFrom);
        
        DECLSPEC_XFGVIRT(IMessage, get_Keywords)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Keywords )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pKeywords);
        
        DECLSPEC_XFGVIRT(IMessage, put_Keywords)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Keywords )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varKeywords);
        
        DECLSPEC_XFGVIRT(IMessage, get_MimeFormatted)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MimeFormatted )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pMimeFormatted);
        
        DECLSPEC_XFGVIRT(IMessage, put_MimeFormatted)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MimeFormatted )( 
            __RPC__in IMessage * This,
            /* [in] */ VARIANT_BOOL varMimeFormatted);
        
        DECLSPEC_XFGVIRT(IMessage, get_Newsgroups)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Newsgroups )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pNewsgroups);
        
        DECLSPEC_XFGVIRT(IMessage, put_Newsgroups)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Newsgroups )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varNewsgroups);
        
        DECLSPEC_XFGVIRT(IMessage, get_Organization)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Organization )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pOrganization);
        
        DECLSPEC_XFGVIRT(IMessage, put_Organization)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Organization )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varOrganization);
        
        DECLSPEC_XFGVIRT(IMessage, get_ReceivedTime)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReceivedTime )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__out DATE *varReceivedTime);
        
        DECLSPEC_XFGVIRT(IMessage, get_ReplyTo)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReplyTo )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pReplyTo);
        
        DECLSPEC_XFGVIRT(IMessage, put_ReplyTo)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ReplyTo )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varReplyTo);
        
        DECLSPEC_XFGVIRT(IMessage, get_DSNOptions)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DSNOptions )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__out CdoDSNOptions *pDSNOptions);
        
        DECLSPEC_XFGVIRT(IMessage, put_DSNOptions)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DSNOptions )( 
            __RPC__in IMessage * This,
            /* [in] */ CdoDSNOptions varDSNOptions);
        
        DECLSPEC_XFGVIRT(IMessage, get_SentOn)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SentOn )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__out DATE *varSentOn);
        
        DECLSPEC_XFGVIRT(IMessage, get_Subject)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSubject);
        
        DECLSPEC_XFGVIRT(IMessage, put_Subject)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Subject )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varSubject);
        
        DECLSPEC_XFGVIRT(IMessage, get_To)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_To )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTo);
        
        DECLSPEC_XFGVIRT(IMessage, put_To)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_To )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varTo);
        
        DECLSPEC_XFGVIRT(IMessage, get_TextBody)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TextBody )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTextBody);
        
        DECLSPEC_XFGVIRT(IMessage, put_TextBody)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TextBody )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varTextBody);
        
        DECLSPEC_XFGVIRT(IMessage, get_HTMLBody)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HTMLBody )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pHTMLBody);
        
        DECLSPEC_XFGVIRT(IMessage, put_HTMLBody)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_HTMLBody )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varHTMLBody);
        
        DECLSPEC_XFGVIRT(IMessage, get_Attachments)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Attachments )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IBodyParts **varAttachments);
        
        DECLSPEC_XFGVIRT(IMessage, get_Sender)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pSender);
        
        DECLSPEC_XFGVIRT(IMessage, put_Sender)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Sender )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR varSender);
        
        DECLSPEC_XFGVIRT(IMessage, get_Configuration)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IConfiguration **pConfiguration);
        
        DECLSPEC_XFGVIRT(IMessage, put_Configuration)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Configuration )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in_opt IConfiguration *varConfiguration);
        
        DECLSPEC_XFGVIRT(IMessage, putref_Configuration)
        /* [helpcontext][helpstring][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Configuration )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in_opt IConfiguration *varConfiguration);
        
        DECLSPEC_XFGVIRT(IMessage, get_AutoGenerateTextBody)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AutoGenerateTextBody )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pAutoGenerateTextBody);
        
        DECLSPEC_XFGVIRT(IMessage, put_AutoGenerateTextBody)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AutoGenerateTextBody )( 
            __RPC__in IMessage * This,
            /* [in] */ VARIANT_BOOL varAutoGenerateTextBody);
        
        DECLSPEC_XFGVIRT(IMessage, get_EnvelopeFields)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EnvelopeFields )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt Fields **varEnvelopeFields);
        
        DECLSPEC_XFGVIRT(IMessage, get_TextBodyPart)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TextBodyPart )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varTextBodyPart);
        
        DECLSPEC_XFGVIRT(IMessage, get_HTMLBodyPart)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HTMLBodyPart )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varHTMLBodyPart);
        
        DECLSPEC_XFGVIRT(IMessage, get_BodyPart)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BodyPart )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varBodyPart);
        
        DECLSPEC_XFGVIRT(IMessage, get_DataSource)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSource )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataSource **varDataSource);
        
        DECLSPEC_XFGVIRT(IMessage, get_Fields)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt Fields **varFields);
        
        DECLSPEC_XFGVIRT(IMessage, get_MDNRequested)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MDNRequested )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pMDNRequested);
        
        DECLSPEC_XFGVIRT(IMessage, put_MDNRequested)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MDNRequested )( 
            __RPC__in IMessage * This,
            /* [in] */ VARIANT_BOOL varMDNRequested);
        
        DECLSPEC_XFGVIRT(IMessage, AddRelatedBodyPart)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddRelatedBodyPart )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Reference,
            /* [in] */ CdoReferenceType ReferenceType,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppBody);
        
        DECLSPEC_XFGVIRT(IMessage, AddAttachment)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddAttachment )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppBody);
        
        DECLSPEC_XFGVIRT(IMessage, CreateMHTMLBody)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateMHTMLBody )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [defaultvalue][in] */ CdoMHTMLFlags Flags,
            /* [optional][in] */ __RPC__in BSTR UserName,
            /* [optional][in] */ __RPC__in BSTR Password);
        
        DECLSPEC_XFGVIRT(IMessage, Forward)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Forward )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg);
        
        DECLSPEC_XFGVIRT(IMessage, Post)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Post )( 
            __RPC__in IMessage * This);
        
        DECLSPEC_XFGVIRT(IMessage, PostReply)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PostReply )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg);
        
        DECLSPEC_XFGVIRT(IMessage, Reply)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reply )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg);
        
        DECLSPEC_XFGVIRT(IMessage, ReplyAll)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReplyAll )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMsg);
        
        DECLSPEC_XFGVIRT(IMessage, Send)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IMessage * This);
        
        DECLSPEC_XFGVIRT(IMessage, GetStream)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream);
        
        DECLSPEC_XFGVIRT(IMessage, GetInterface)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInterface )( 
            __RPC__in IMessage * This,
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown);
        
        END_INTERFACE
    } IMessageVtbl;

    interface IMessage
    {
        CONST_VTBL struct IMessageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMessage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMessage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMessage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMessage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMessage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMessage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMessage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMessage_get_BCC(This,pBCC)	\
    ( (This)->lpVtbl -> get_BCC(This,pBCC) ) 

#define IMessage_put_BCC(This,varBCC)	\
    ( (This)->lpVtbl -> put_BCC(This,varBCC) ) 

#define IMessage_get_CC(This,pCC)	\
    ( (This)->lpVtbl -> get_CC(This,pCC) ) 

#define IMessage_put_CC(This,varCC)	\
    ( (This)->lpVtbl -> put_CC(This,varCC) ) 

#define IMessage_get_FollowUpTo(This,pFollowUpTo)	\
    ( (This)->lpVtbl -> get_FollowUpTo(This,pFollowUpTo) ) 

#define IMessage_put_FollowUpTo(This,varFollowUpTo)	\
    ( (This)->lpVtbl -> put_FollowUpTo(This,varFollowUpTo) ) 

#define IMessage_get_From(This,pFrom)	\
    ( (This)->lpVtbl -> get_From(This,pFrom) ) 

#define IMessage_put_From(This,varFrom)	\
    ( (This)->lpVtbl -> put_From(This,varFrom) ) 

#define IMessage_get_Keywords(This,pKeywords)	\
    ( (This)->lpVtbl -> get_Keywords(This,pKeywords) ) 

#define IMessage_put_Keywords(This,varKeywords)	\
    ( (This)->lpVtbl -> put_Keywords(This,varKeywords) ) 

#define IMessage_get_MimeFormatted(This,pMimeFormatted)	\
    ( (This)->lpVtbl -> get_MimeFormatted(This,pMimeFormatted) ) 

#define IMessage_put_MimeFormatted(This,varMimeFormatted)	\
    ( (This)->lpVtbl -> put_MimeFormatted(This,varMimeFormatted) ) 

#define IMessage_get_Newsgroups(This,pNewsgroups)	\
    ( (This)->lpVtbl -> get_Newsgroups(This,pNewsgroups) ) 

#define IMessage_put_Newsgroups(This,varNewsgroups)	\
    ( (This)->lpVtbl -> put_Newsgroups(This,varNewsgroups) ) 

#define IMessage_get_Organization(This,pOrganization)	\
    ( (This)->lpVtbl -> get_Organization(This,pOrganization) ) 

#define IMessage_put_Organization(This,varOrganization)	\
    ( (This)->lpVtbl -> put_Organization(This,varOrganization) ) 

#define IMessage_get_ReceivedTime(This,varReceivedTime)	\
    ( (This)->lpVtbl -> get_ReceivedTime(This,varReceivedTime) ) 

#define IMessage_get_ReplyTo(This,pReplyTo)	\
    ( (This)->lpVtbl -> get_ReplyTo(This,pReplyTo) ) 

#define IMessage_put_ReplyTo(This,varReplyTo)	\
    ( (This)->lpVtbl -> put_ReplyTo(This,varReplyTo) ) 

#define IMessage_get_DSNOptions(This,pDSNOptions)	\
    ( (This)->lpVtbl -> get_DSNOptions(This,pDSNOptions) ) 

#define IMessage_put_DSNOptions(This,varDSNOptions)	\
    ( (This)->lpVtbl -> put_DSNOptions(This,varDSNOptions) ) 

#define IMessage_get_SentOn(This,varSentOn)	\
    ( (This)->lpVtbl -> get_SentOn(This,varSentOn) ) 

#define IMessage_get_Subject(This,pSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pSubject) ) 

#define IMessage_put_Subject(This,varSubject)	\
    ( (This)->lpVtbl -> put_Subject(This,varSubject) ) 

#define IMessage_get_To(This,pTo)	\
    ( (This)->lpVtbl -> get_To(This,pTo) ) 

#define IMessage_put_To(This,varTo)	\
    ( (This)->lpVtbl -> put_To(This,varTo) ) 

#define IMessage_get_TextBody(This,pTextBody)	\
    ( (This)->lpVtbl -> get_TextBody(This,pTextBody) ) 

#define IMessage_put_TextBody(This,varTextBody)	\
    ( (This)->lpVtbl -> put_TextBody(This,varTextBody) ) 

#define IMessage_get_HTMLBody(This,pHTMLBody)	\
    ( (This)->lpVtbl -> get_HTMLBody(This,pHTMLBody) ) 

#define IMessage_put_HTMLBody(This,varHTMLBody)	\
    ( (This)->lpVtbl -> put_HTMLBody(This,varHTMLBody) ) 

#define IMessage_get_Attachments(This,varAttachments)	\
    ( (This)->lpVtbl -> get_Attachments(This,varAttachments) ) 

#define IMessage_get_Sender(This,pSender)	\
    ( (This)->lpVtbl -> get_Sender(This,pSender) ) 

#define IMessage_put_Sender(This,varSender)	\
    ( (This)->lpVtbl -> put_Sender(This,varSender) ) 

#define IMessage_get_Configuration(This,pConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,pConfiguration) ) 

#define IMessage_put_Configuration(This,varConfiguration)	\
    ( (This)->lpVtbl -> put_Configuration(This,varConfiguration) ) 

#define IMessage_putref_Configuration(This,varConfiguration)	\
    ( (This)->lpVtbl -> putref_Configuration(This,varConfiguration) ) 

#define IMessage_get_AutoGenerateTextBody(This,pAutoGenerateTextBody)	\
    ( (This)->lpVtbl -> get_AutoGenerateTextBody(This,pAutoGenerateTextBody) ) 

#define IMessage_put_AutoGenerateTextBody(This,varAutoGenerateTextBody)	\
    ( (This)->lpVtbl -> put_AutoGenerateTextBody(This,varAutoGenerateTextBody) ) 

#define IMessage_get_EnvelopeFields(This,varEnvelopeFields)	\
    ( (This)->lpVtbl -> get_EnvelopeFields(This,varEnvelopeFields) ) 

#define IMessage_get_TextBodyPart(This,varTextBodyPart)	\
    ( (This)->lpVtbl -> get_TextBodyPart(This,varTextBodyPart) ) 

#define IMessage_get_HTMLBodyPart(This,varHTMLBodyPart)	\
    ( (This)->lpVtbl -> get_HTMLBodyPart(This,varHTMLBodyPart) ) 

#define IMessage_get_BodyPart(This,varBodyPart)	\
    ( (This)->lpVtbl -> get_BodyPart(This,varBodyPart) ) 

#define IMessage_get_DataSource(This,varDataSource)	\
    ( (This)->lpVtbl -> get_DataSource(This,varDataSource) ) 

#define IMessage_get_Fields(This,varFields)	\
    ( (This)->lpVtbl -> get_Fields(This,varFields) ) 

#define IMessage_get_MDNRequested(This,pMDNRequested)	\
    ( (This)->lpVtbl -> get_MDNRequested(This,pMDNRequested) ) 

#define IMessage_put_MDNRequested(This,varMDNRequested)	\
    ( (This)->lpVtbl -> put_MDNRequested(This,varMDNRequested) ) 

#define IMessage_AddRelatedBodyPart(This,URL,Reference,ReferenceType,UserName,Password,ppBody)	\
    ( (This)->lpVtbl -> AddRelatedBodyPart(This,URL,Reference,ReferenceType,UserName,Password,ppBody) ) 

#define IMessage_AddAttachment(This,URL,UserName,Password,ppBody)	\
    ( (This)->lpVtbl -> AddAttachment(This,URL,UserName,Password,ppBody) ) 

#define IMessage_CreateMHTMLBody(This,URL,Flags,UserName,Password)	\
    ( (This)->lpVtbl -> CreateMHTMLBody(This,URL,Flags,UserName,Password) ) 

#define IMessage_Forward(This,ppMsg)	\
    ( (This)->lpVtbl -> Forward(This,ppMsg) ) 

#define IMessage_Post(This)	\
    ( (This)->lpVtbl -> Post(This) ) 

#define IMessage_PostReply(This,ppMsg)	\
    ( (This)->lpVtbl -> PostReply(This,ppMsg) ) 

#define IMessage_Reply(This,ppMsg)	\
    ( (This)->lpVtbl -> Reply(This,ppMsg) ) 

#define IMessage_ReplyAll(This,ppMsg)	\
    ( (This)->lpVtbl -> ReplyAll(This,ppMsg) ) 

#define IMessage_Send(This)	\
    ( (This)->lpVtbl -> Send(This) ) 

#define IMessage_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IMessage_GetInterface(This,Interface,ppUnknown)	\
    ( (This)->lpVtbl -> GetInterface(This,Interface,ppUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMessage_INTERFACE_DEFINED__ */


#ifndef __IBodyPart_INTERFACE_DEFINED__
#define __IBodyPart_INTERFACE_DEFINED__

/* interface IBodyPart */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IBodyPart;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000021-8B95-11D1-82DB-00C04FB1625D")
    IBodyPart : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BodyParts( 
            /* [retval][out] */ __RPC__deref_out_opt IBodyParts **varBodyParts) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ContentTransferEncoding( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentTransferEncoding) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ContentTransferEncoding( 
            /* [in] */ __RPC__in BSTR varContentTransferEncoding) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ContentMediaType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentMediaType) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ContentMediaType( 
            /* [in] */ __RPC__in BSTR varContentMediaType) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Fields( 
            /* [retval][out] */ __RPC__deref_out_opt Fields **varFields) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Charset( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCharset) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Charset( 
            /* [in] */ __RPC__in BSTR varCharset) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *varFileName) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DataSource( 
            /* [retval][out] */ __RPC__deref_out_opt IDataSource **varDataSource) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ContentClass( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentClass) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ContentClass( 
            /* [in] */ __RPC__in BSTR varContentClass) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ContentClassName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentClassName) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ContentClassName( 
            /* [in] */ __RPC__in BSTR varContentClassName) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varParent) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddBodyPart( 
            /* [defaultvalue][in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppPart) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveToFile( 
            /* [in] */ __RPC__in BSTR FileName) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEncodedContentStream( 
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDecodedContentStream( 
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFieldParameter( 
            /* [in] */ __RPC__in BSTR FieldName,
            /* [in] */ __RPC__in BSTR Parameter,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInterface( 
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBodyPartVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBodyPart * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBodyPart * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBodyPart * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBodyPart * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBodyPart * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_BodyParts)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BodyParts )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IBodyParts **varBodyParts);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_ContentTransferEncoding)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ContentTransferEncoding )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentTransferEncoding);
        
        DECLSPEC_XFGVIRT(IBodyPart, put_ContentTransferEncoding)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ContentTransferEncoding )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR varContentTransferEncoding);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_ContentMediaType)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ContentMediaType )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentMediaType);
        
        DECLSPEC_XFGVIRT(IBodyPart, put_ContentMediaType)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ContentMediaType )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR varContentMediaType);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_Fields)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt Fields **varFields);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_Charset)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Charset )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCharset);
        
        DECLSPEC_XFGVIRT(IBodyPart, put_Charset)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Charset )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR varCharset);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_FileName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *varFileName);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_DataSource)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DataSource )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IDataSource **varDataSource);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_ContentClass)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ContentClass )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentClass);
        
        DECLSPEC_XFGVIRT(IBodyPart, put_ContentClass)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ContentClass )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR varContentClass);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_ContentClassName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ContentClassName )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pContentClassName);
        
        DECLSPEC_XFGVIRT(IBodyPart, put_ContentClassName)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ContentClassName )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR varContentClassName);
        
        DECLSPEC_XFGVIRT(IBodyPart, get_Parent)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **varParent);
        
        DECLSPEC_XFGVIRT(IBodyPart, AddBodyPart)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddBodyPart )( 
            __RPC__in IBodyPart * This,
            /* [defaultvalue][in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppPart);
        
        DECLSPEC_XFGVIRT(IBodyPart, SaveToFile)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveToFile )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR FileName);
        
        DECLSPEC_XFGVIRT(IBodyPart, GetEncodedContentStream)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEncodedContentStream )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream);
        
        DECLSPEC_XFGVIRT(IBodyPart, GetDecodedContentStream)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDecodedContentStream )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream);
        
        DECLSPEC_XFGVIRT(IBodyPart, GetStream)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IBodyPart * This,
            /* [retval][out] */ __RPC__deref_out_opt _Stream **ppStream);
        
        DECLSPEC_XFGVIRT(IBodyPart, GetFieldParameter)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFieldParameter )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR FieldName,
            /* [in] */ __RPC__in BSTR Parameter,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IBodyPart, GetInterface)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInterface )( 
            __RPC__in IBodyPart * This,
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown);
        
        END_INTERFACE
    } IBodyPartVtbl;

    interface IBodyPart
    {
        CONST_VTBL struct IBodyPartVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBodyPart_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBodyPart_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBodyPart_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBodyPart_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBodyPart_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBodyPart_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBodyPart_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBodyPart_get_BodyParts(This,varBodyParts)	\
    ( (This)->lpVtbl -> get_BodyParts(This,varBodyParts) ) 

#define IBodyPart_get_ContentTransferEncoding(This,pContentTransferEncoding)	\
    ( (This)->lpVtbl -> get_ContentTransferEncoding(This,pContentTransferEncoding) ) 

#define IBodyPart_put_ContentTransferEncoding(This,varContentTransferEncoding)	\
    ( (This)->lpVtbl -> put_ContentTransferEncoding(This,varContentTransferEncoding) ) 

#define IBodyPart_get_ContentMediaType(This,pContentMediaType)	\
    ( (This)->lpVtbl -> get_ContentMediaType(This,pContentMediaType) ) 

#define IBodyPart_put_ContentMediaType(This,varContentMediaType)	\
    ( (This)->lpVtbl -> put_ContentMediaType(This,varContentMediaType) ) 

#define IBodyPart_get_Fields(This,varFields)	\
    ( (This)->lpVtbl -> get_Fields(This,varFields) ) 

#define IBodyPart_get_Charset(This,pCharset)	\
    ( (This)->lpVtbl -> get_Charset(This,pCharset) ) 

#define IBodyPart_put_Charset(This,varCharset)	\
    ( (This)->lpVtbl -> put_Charset(This,varCharset) ) 

#define IBodyPart_get_FileName(This,varFileName)	\
    ( (This)->lpVtbl -> get_FileName(This,varFileName) ) 

#define IBodyPart_get_DataSource(This,varDataSource)	\
    ( (This)->lpVtbl -> get_DataSource(This,varDataSource) ) 

#define IBodyPart_get_ContentClass(This,pContentClass)	\
    ( (This)->lpVtbl -> get_ContentClass(This,pContentClass) ) 

#define IBodyPart_put_ContentClass(This,varContentClass)	\
    ( (This)->lpVtbl -> put_ContentClass(This,varContentClass) ) 

#define IBodyPart_get_ContentClassName(This,pContentClassName)	\
    ( (This)->lpVtbl -> get_ContentClassName(This,pContentClassName) ) 

#define IBodyPart_put_ContentClassName(This,varContentClassName)	\
    ( (This)->lpVtbl -> put_ContentClassName(This,varContentClassName) ) 

#define IBodyPart_get_Parent(This,varParent)	\
    ( (This)->lpVtbl -> get_Parent(This,varParent) ) 

#define IBodyPart_AddBodyPart(This,Index,ppPart)	\
    ( (This)->lpVtbl -> AddBodyPart(This,Index,ppPart) ) 

#define IBodyPart_SaveToFile(This,FileName)	\
    ( (This)->lpVtbl -> SaveToFile(This,FileName) ) 

#define IBodyPart_GetEncodedContentStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetEncodedContentStream(This,ppStream) ) 

#define IBodyPart_GetDecodedContentStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetDecodedContentStream(This,ppStream) ) 

#define IBodyPart_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IBodyPart_GetFieldParameter(This,FieldName,Parameter,pbstrValue)	\
    ( (This)->lpVtbl -> GetFieldParameter(This,FieldName,Parameter,pbstrValue) ) 

#define IBodyPart_GetInterface(This,Interface,ppUnknown)	\
    ( (This)->lpVtbl -> GetInterface(This,Interface,ppUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBodyPart_INTERFACE_DEFINED__ */


#ifndef __IConfiguration_INTERFACE_DEFINED__
#define __IConfiguration_INTERFACE_DEFINED__

/* interface IConfiguration */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000022-8B95-11D1-82DB-00C04FB1625D")
    IConfiguration : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Fields( 
            /* [retval][out] */ __RPC__deref_out_opt Fields **varFields) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ CdoConfigSource LoadFrom,
            /* [optional][in] */ __RPC__in BSTR URL) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInterface( 
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConfiguration * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IConfiguration * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IConfiguration * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IConfiguration * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IConfiguration, get_Fields)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Fields )( 
            __RPC__in IConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt Fields **varFields);
        
        DECLSPEC_XFGVIRT(IConfiguration, Load)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IConfiguration * This,
            /* [in] */ CdoConfigSource LoadFrom,
            /* [optional][in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IConfiguration, GetInterface)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInterface )( 
            __RPC__in IConfiguration * This,
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown);
        
        END_INTERFACE
    } IConfigurationVtbl;

    interface IConfiguration
    {
        CONST_VTBL struct IConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConfiguration_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IConfiguration_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IConfiguration_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IConfiguration_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IConfiguration_get_Fields(This,varFields)	\
    ( (This)->lpVtbl -> get_Fields(This,varFields) ) 

#define IConfiguration_Load(This,LoadFrom,URL)	\
    ( (This)->lpVtbl -> Load(This,LoadFrom,URL) ) 

#define IConfiguration_GetInterface(This,Interface,ppUnknown)	\
    ( (This)->lpVtbl -> GetInterface(This,Interface,ppUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConfiguration_INTERFACE_DEFINED__ */


#ifndef __IMessages_INTERFACE_DEFINED__
#define __IMessages_INTERFACE_DEFINED__

/* interface IMessages */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IMessages;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000025-8B95-11D1-82DB-00C04FB1625D")
    IMessages : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long Index,
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMessage) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *varCount) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ long Index) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAll( void) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Filename( 
            VARIANT var,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Filename) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMessagesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMessages * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMessages * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMessages * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMessages * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMessages * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMessages * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMessages * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMessages, get_Item)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IMessages * This,
            long Index,
            /* [retval][out] */ __RPC__deref_out_opt IMessage **ppMessage);
        
        DECLSPEC_XFGVIRT(IMessages, get_Count)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IMessages * This,
            /* [retval][out] */ __RPC__out long *varCount);
        
        DECLSPEC_XFGVIRT(IMessages, Delete)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMessages * This,
            /* [in] */ long Index);
        
        DECLSPEC_XFGVIRT(IMessages, DeleteAll)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAll )( 
            __RPC__in IMessages * This);
        
        DECLSPEC_XFGVIRT(IMessages, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IMessages * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(IMessages, get_Filename)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Filename )( 
            __RPC__in IMessages * This,
            VARIANT var,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Filename);
        
        END_INTERFACE
    } IMessagesVtbl;

    interface IMessages
    {
        CONST_VTBL struct IMessagesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMessages_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMessages_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMessages_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMessages_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMessages_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMessages_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMessages_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMessages_get_Item(This,Index,ppMessage)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppMessage) ) 

#define IMessages_get_Count(This,varCount)	\
    ( (This)->lpVtbl -> get_Count(This,varCount) ) 

#define IMessages_Delete(This,Index)	\
    ( (This)->lpVtbl -> Delete(This,Index) ) 

#define IMessages_DeleteAll(This)	\
    ( (This)->lpVtbl -> DeleteAll(This) ) 

#define IMessages_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define IMessages_get_Filename(This,var,Filename)	\
    ( (This)->lpVtbl -> get_Filename(This,var,Filename) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMessages_INTERFACE_DEFINED__ */


#ifndef __IDropDirectory_INTERFACE_DEFINED__
#define __IDropDirectory_INTERFACE_DEFINED__

/* interface IDropDirectory */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IDropDirectory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000024-8B95-11D1-82DB-00C04FB1625D")
    IDropDirectory : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessages( 
            /* [optional][in] */ __RPC__in BSTR DirName,
            /* [retval][out] */ __RPC__deref_out_opt IMessages **Msgs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDropDirectoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDropDirectory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDropDirectory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDropDirectory * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDropDirectory * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDropDirectory * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDropDirectory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDropDirectory * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDropDirectory, GetMessages)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessages )( 
            __RPC__in IDropDirectory * This,
            /* [optional][in] */ __RPC__in BSTR DirName,
            /* [retval][out] */ __RPC__deref_out_opt IMessages **Msgs);
        
        END_INTERFACE
    } IDropDirectoryVtbl;

    interface IDropDirectory
    {
        CONST_VTBL struct IDropDirectoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDropDirectory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDropDirectory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDropDirectory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDropDirectory_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDropDirectory_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDropDirectory_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDropDirectory_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDropDirectory_GetMessages(This,DirName,Msgs)	\
    ( (This)->lpVtbl -> GetMessages(This,DirName,Msgs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDropDirectory_INTERFACE_DEFINED__ */


#ifndef __IBodyParts_INTERFACE_DEFINED__
#define __IBodyParts_INTERFACE_DEFINED__

/* interface IBodyParts */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IBodyParts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000023-8B95-11D1-82DB-00C04FB1625D")
    IBodyParts : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *varCount) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppBody) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT varBP) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteAll( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [defaultvalue][in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppPart) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBodyPartsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBodyParts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBodyParts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBodyParts * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBodyParts * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBodyParts * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBodyParts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBodyParts * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IBodyParts, get_Count)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IBodyParts * This,
            /* [retval][out] */ __RPC__out long *varCount);
        
        DECLSPEC_XFGVIRT(IBodyParts, get_Item)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IBodyParts * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppBody);
        
        DECLSPEC_XFGVIRT(IBodyParts, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IBodyParts * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(IBodyParts, Delete)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IBodyParts * This,
            /* [in] */ VARIANT varBP);
        
        DECLSPEC_XFGVIRT(IBodyParts, DeleteAll)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteAll )( 
            __RPC__in IBodyParts * This);
        
        DECLSPEC_XFGVIRT(IBodyParts, Add)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IBodyParts * This,
            /* [defaultvalue][in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IBodyPart **ppPart);
        
        END_INTERFACE
    } IBodyPartsVtbl;

    interface IBodyParts
    {
        CONST_VTBL struct IBodyPartsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBodyParts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBodyParts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBodyParts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBodyParts_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBodyParts_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBodyParts_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBodyParts_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBodyParts_get_Count(This,varCount)	\
    ( (This)->lpVtbl -> get_Count(This,varCount) ) 

#define IBodyParts_get_Item(This,Index,ppBody)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppBody) ) 

#define IBodyParts_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define IBodyParts_Delete(This,varBP)	\
    ( (This)->lpVtbl -> Delete(This,varBP) ) 

#define IBodyParts_DeleteAll(This)	\
    ( (This)->lpVtbl -> DeleteAll(This) ) 

#define IBodyParts_Add(This,Index,ppPart)	\
    ( (This)->lpVtbl -> Add(This,Index,ppPart) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBodyParts_INTERFACE_DEFINED__ */


#ifndef __ISMTPScriptConnector_INTERFACE_DEFINED__
#define __ISMTPScriptConnector_INTERFACE_DEFINED__

/* interface ISMTPScriptConnector */
/* [hidden][unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_ISMTPScriptConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000030-8B95-11D1-82DB-00C04FB1625D")
    ISMTPScriptConnector : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISMTPScriptConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISMTPScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISMTPScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISMTPScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISMTPScriptConnector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISMTPScriptConnector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISMTPScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISMTPScriptConnector * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } ISMTPScriptConnectorVtbl;

    interface ISMTPScriptConnector
    {
        CONST_VTBL struct ISMTPScriptConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISMTPScriptConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISMTPScriptConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISMTPScriptConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISMTPScriptConnector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISMTPScriptConnector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISMTPScriptConnector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISMTPScriptConnector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISMTPScriptConnector_INTERFACE_DEFINED__ */


#ifndef __INNTPEarlyScriptConnector_INTERFACE_DEFINED__
#define __INNTPEarlyScriptConnector_INTERFACE_DEFINED__

/* interface INNTPEarlyScriptConnector */
/* [hidden][unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_INNTPEarlyScriptConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000034-8B95-11D1-82DB-00C04FB1625D")
    INNTPEarlyScriptConnector : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct INNTPEarlyScriptConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INNTPEarlyScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INNTPEarlyScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INNTPEarlyScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INNTPEarlyScriptConnector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INNTPEarlyScriptConnector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INNTPEarlyScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INNTPEarlyScriptConnector * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } INNTPEarlyScriptConnectorVtbl;

    interface INNTPEarlyScriptConnector
    {
        CONST_VTBL struct INNTPEarlyScriptConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INNTPEarlyScriptConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INNTPEarlyScriptConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INNTPEarlyScriptConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INNTPEarlyScriptConnector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INNTPEarlyScriptConnector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INNTPEarlyScriptConnector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INNTPEarlyScriptConnector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INNTPEarlyScriptConnector_INTERFACE_DEFINED__ */


#ifndef __INNTPPostScriptConnector_INTERFACE_DEFINED__
#define __INNTPPostScriptConnector_INTERFACE_DEFINED__

/* interface INNTPPostScriptConnector */
/* [hidden][unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_INNTPPostScriptConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000031-8B95-11D1-82DB-00C04FB1625D")
    INNTPPostScriptConnector : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct INNTPPostScriptConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INNTPPostScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INNTPPostScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INNTPPostScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INNTPPostScriptConnector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INNTPPostScriptConnector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INNTPPostScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INNTPPostScriptConnector * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } INNTPPostScriptConnectorVtbl;

    interface INNTPPostScriptConnector
    {
        CONST_VTBL struct INNTPPostScriptConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INNTPPostScriptConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INNTPPostScriptConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INNTPPostScriptConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INNTPPostScriptConnector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INNTPPostScriptConnector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INNTPPostScriptConnector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INNTPPostScriptConnector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INNTPPostScriptConnector_INTERFACE_DEFINED__ */


#ifndef __INNTPFinalScriptConnector_INTERFACE_DEFINED__
#define __INNTPFinalScriptConnector_INTERFACE_DEFINED__

/* interface INNTPFinalScriptConnector */
/* [hidden][unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_INNTPFinalScriptConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000032-8B95-11D1-82DB-00C04FB1625D")
    INNTPFinalScriptConnector : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct INNTPFinalScriptConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INNTPFinalScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INNTPFinalScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INNTPFinalScriptConnector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INNTPFinalScriptConnector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INNTPFinalScriptConnector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INNTPFinalScriptConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INNTPFinalScriptConnector * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } INNTPFinalScriptConnectorVtbl;

    interface INNTPFinalScriptConnector
    {
        CONST_VTBL struct INNTPFinalScriptConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INNTPFinalScriptConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INNTPFinalScriptConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INNTPFinalScriptConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INNTPFinalScriptConnector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INNTPFinalScriptConnector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INNTPFinalScriptConnector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INNTPFinalScriptConnector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INNTPFinalScriptConnector_INTERFACE_DEFINED__ */


#ifndef __ISMTPOnArrival_INTERFACE_DEFINED__
#define __ISMTPOnArrival_INTERFACE_DEFINED__

/* interface ISMTPOnArrival */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_ISMTPOnArrival;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000026-8B95-11D1-82DB-00C04FB1625D")
    ISMTPOnArrival : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE OnArrival( 
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISMTPOnArrivalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISMTPOnArrival * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISMTPOnArrival * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISMTPOnArrival * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISMTPOnArrival * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISMTPOnArrival * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISMTPOnArrival * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISMTPOnArrival * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISMTPOnArrival, OnArrival)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnArrival )( 
            __RPC__in ISMTPOnArrival * This,
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus);
        
        END_INTERFACE
    } ISMTPOnArrivalVtbl;

    interface ISMTPOnArrival
    {
        CONST_VTBL struct ISMTPOnArrivalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISMTPOnArrival_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISMTPOnArrival_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISMTPOnArrival_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISMTPOnArrival_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISMTPOnArrival_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISMTPOnArrival_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISMTPOnArrival_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISMTPOnArrival_OnArrival(This,Msg,EventStatus)	\
    ( (This)->lpVtbl -> OnArrival(This,Msg,EventStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISMTPOnArrival_INTERFACE_DEFINED__ */


#ifndef __INNTPOnPostEarly_INTERFACE_DEFINED__
#define __INNTPOnPostEarly_INTERFACE_DEFINED__

/* interface INNTPOnPostEarly */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_INNTPOnPostEarly;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000033-8B95-11D1-82DB-00C04FB1625D")
    INNTPOnPostEarly : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPostEarly( 
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INNTPOnPostEarlyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INNTPOnPostEarly * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INNTPOnPostEarly * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INNTPOnPostEarly * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INNTPOnPostEarly * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INNTPOnPostEarly * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INNTPOnPostEarly * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INNTPOnPostEarly * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(INNTPOnPostEarly, OnPostEarly)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPostEarly )( 
            __RPC__in INNTPOnPostEarly * This,
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus);
        
        END_INTERFACE
    } INNTPOnPostEarlyVtbl;

    interface INNTPOnPostEarly
    {
        CONST_VTBL struct INNTPOnPostEarlyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INNTPOnPostEarly_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INNTPOnPostEarly_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INNTPOnPostEarly_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INNTPOnPostEarly_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INNTPOnPostEarly_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INNTPOnPostEarly_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INNTPOnPostEarly_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define INNTPOnPostEarly_OnPostEarly(This,Msg,EventStatus)	\
    ( (This)->lpVtbl -> OnPostEarly(This,Msg,EventStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INNTPOnPostEarly_INTERFACE_DEFINED__ */


#ifndef __INNTPOnPost_INTERFACE_DEFINED__
#define __INNTPOnPost_INTERFACE_DEFINED__

/* interface INNTPOnPost */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_INNTPOnPost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000027-8B95-11D1-82DB-00C04FB1625D")
    INNTPOnPost : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPost( 
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INNTPOnPostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INNTPOnPost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INNTPOnPost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INNTPOnPost * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INNTPOnPost * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INNTPOnPost * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INNTPOnPost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INNTPOnPost * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(INNTPOnPost, OnPost)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPost )( 
            __RPC__in INNTPOnPost * This,
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus);
        
        END_INTERFACE
    } INNTPOnPostVtbl;

    interface INNTPOnPost
    {
        CONST_VTBL struct INNTPOnPostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INNTPOnPost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INNTPOnPost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INNTPOnPost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INNTPOnPost_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INNTPOnPost_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INNTPOnPost_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INNTPOnPost_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define INNTPOnPost_OnPost(This,Msg,EventStatus)	\
    ( (This)->lpVtbl -> OnPost(This,Msg,EventStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INNTPOnPost_INTERFACE_DEFINED__ */


#ifndef __INNTPOnPostFinal_INTERFACE_DEFINED__
#define __INNTPOnPostFinal_INTERFACE_DEFINED__

/* interface INNTPOnPostFinal */
/* [unique][helpcontext][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_INNTPOnPostFinal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000028-8B95-11D1-82DB-00C04FB1625D")
    INNTPOnPostFinal : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPostFinal( 
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INNTPOnPostFinalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INNTPOnPostFinal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INNTPOnPostFinal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INNTPOnPostFinal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INNTPOnPostFinal * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INNTPOnPostFinal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INNTPOnPostFinal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INNTPOnPostFinal * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(INNTPOnPostFinal, OnPostFinal)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPostFinal )( 
            __RPC__in INNTPOnPostFinal * This,
            /* [in] */ __RPC__in_opt IMessage *Msg,
            /* [out][in] */ __RPC__inout CdoEventStatus *EventStatus);
        
        END_INTERFACE
    } INNTPOnPostFinalVtbl;

    interface INNTPOnPostFinal
    {
        CONST_VTBL struct INNTPOnPostFinalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INNTPOnPostFinal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INNTPOnPostFinal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INNTPOnPostFinal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INNTPOnPostFinal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INNTPOnPostFinal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INNTPOnPostFinal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INNTPOnPostFinal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define INNTPOnPostFinal_OnPostFinal(This,Msg,EventStatus)	\
    ( (This)->lpVtbl -> OnPostFinal(This,Msg,EventStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INNTPOnPostFinal_INTERFACE_DEFINED__ */


#ifndef __IProxyObject_INTERFACE_DEFINED__
#define __IProxyObject_INTERFACE_DEFINED__

/* interface IProxyObject */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProxyObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD000083-8B95-11D1-82DB-00C04FB1625D")
    IProxyObject : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Object( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppParent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProxyObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProxyObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProxyObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProxyObject * This);
        
        DECLSPEC_XFGVIRT(IProxyObject, get_Object)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Object )( 
            __RPC__in IProxyObject * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppParent);
        
        END_INTERFACE
    } IProxyObjectVtbl;

    interface IProxyObject
    {
        CONST_VTBL struct IProxyObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProxyObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProxyObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProxyObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProxyObject_get_Object(This,ppParent)	\
    ( (This)->lpVtbl -> get_Object(This,ppParent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProxyObject_INTERFACE_DEFINED__ */


#ifndef __IGetInterface_INTERFACE_DEFINED__
#define __IGetInterface_INTERFACE_DEFINED__

/* interface IGetInterface */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IGetInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD0ff000-8B95-11D1-82DB-00C04FB1625D")
    IGetInterface : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInterface( 
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInterfaceInner( 
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetInterfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetInterface * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetInterface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetInterface * This);
        
        DECLSPEC_XFGVIRT(IGetInterface, GetInterface)
        HRESULT ( STDMETHODCALLTYPE *GetInterface )( 
            __RPC__in IGetInterface * This,
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown);
        
        DECLSPEC_XFGVIRT(IGetInterface, GetInterfaceInner)
        HRESULT ( STDMETHODCALLTYPE *GetInterfaceInner )( 
            __RPC__in IGetInterface * This,
            /* [in] */ __RPC__in BSTR Interface,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppUnknown);
        
        END_INTERFACE
    } IGetInterfaceVtbl;

    interface IGetInterface
    {
        CONST_VTBL struct IGetInterfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetInterface_GetInterface(This,Interface,ppUnknown)	\
    ( (This)->lpVtbl -> GetInterface(This,Interface,ppUnknown) ) 

#define IGetInterface_GetInterfaceInner(This,Interface,ppUnknown)	\
    ( (This)->lpVtbl -> GetInterfaceInner(This,Interface,ppUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetInterface_INTERFACE_DEFINED__ */



#ifndef __CDO_LIBRARY_DEFINED__
#define __CDO_LIBRARY_DEFINED__

/* library CDO */
/* [helpstring][helpfile][version][uuid] */ 


















EXTERN_C const IID LIBID_CDO;




/* module CdoCalendar */
/* [dllname] */ 






/* module CdoCharset */
/* [dllname] */ 


























/* module CdoConfiguration */
/* [dllname] */ 






































/* module CdoContentTypeValues */
/* [dllname] */ 
















/* module CdoDAV */
/* [dllname] */ 







/* module CdoEncodingType */
/* [dllname] */ 












/* module CdoExchange */
/* [dllname] */ 






/* module CdoHTTPMail */
/* [dllname] */ 
























/* module CdoInterfaces */
/* [dllname] */ 











/* module CdoMailHeader */
/* [dllname] */ 









































/* module CdoNamespace */
/* [dllname] */ 











/* module CdoNNTPEnvelope */
/* [dllname] */ 







/* module CdoOffice */
/* [dllname] */ 






/* module CdoSMTPEnvelope */
/* [dllname] */ 









#ifndef __CdoErrors_MODULE_DEFINED__
#define __CdoErrors_MODULE_DEFINED__


/* module CdoErrors */
/* [dllname] */ 

const LONG CDO_E_UNCAUGHT_EXCEPTION	=	0x80040201L;

const LONG CDO_E_NOT_OPENED	=	0x80040202L;

const LONG CDO_E_UNSUPPORTED_DATASOURCE	=	0x80040203L;

const LONG CDO_E_INVALID_PROPERTYNAME	=	0x80040204L;

const LONG CDO_E_PROP_UNSUPPORTED	=	0x80040205L;

const LONG CDO_E_INACTIVE	=	0x80040206L;

const LONG CDO_E_NO_SUPPORT_FOR_OBJECTS	=	0x80040207L;

const LONG CDO_E_NOT_AVAILABLE	=	0x80040208L;

const LONG CDO_E_NO_DEFAULT_DROP_DIR	=	0x80040209L;

const LONG CDO_E_SMTP_SERVER_REQUIRED	=	0x8004020aL;

const LONG CDO_E_NNTP_SERVER_REQUIRED	=	0x8004020bL;

const LONG CDO_E_RECIPIENT_MISSING	=	0x8004020cL;

const LONG CDO_E_FROM_MISSING	=	0x8004020dL;

const LONG CDO_E_SENDER_REJECTED	=	0x8004020eL;

const LONG CDO_E_RECIPIENTS_REJECTED	=	0x8004020fL;

const LONG CDO_E_NNTP_POST_FAILED	=	0x80040210L;

const LONG CDO_E_SMTP_SEND_FAILED	=	0x80040211L;

const LONG CDO_E_CONNECTION_DROPPED	=	0x80040212L;

const LONG CDO_E_FAILED_TO_CONNECT	=	0x80040213L;

const LONG CDO_E_INVALID_POST	=	0x80040214L;

const LONG CDO_E_AUTHENTICATION_FAILURE	=	0x80040215L;

const LONG CDO_E_INVALID_CONTENT_TYPE	=	0x80040216L;

const LONG CDO_E_LOGON_FAILURE	=	0x80040217L;

const LONG CDO_E_HTTP_NOT_FOUND	=	0x80040218L;

const LONG CDO_E_HTTP_FORBIDDEN	=	0x80040219L;

const LONG CDO_E_HTTP_FAILED	=	0x8004021aL;

const LONG CDO_E_MULTIPART_NO_DATA	=	0x8004021bL;

const LONG CDO_E_INVALID_ENCODING_FOR_MULTIPART	=	0x8004021cL;

const LONG CDO_E_UNSAFE_OPERATION	=	0x8004021dL;

const LONG CDO_E_PROP_NOT_FOUND	=	0x8004021eL;

const LONG CDO_E_INVALID_SEND_OPTION	=	0x80040220L;

const LONG CDO_E_INVALID_POST_OPTION	=	0x80040221L;

const LONG CDO_E_NO_PICKUP_DIR	=	0x80040222L;

const LONG CDO_E_NOT_ALL_DELETED	=	0x80040223L;

const LONG CDO_E_NO_METHOD	=	0x80040224L;

const LONG CDO_E_PROP_READONLY	=	0x80040227L;

const LONG CDO_E_PROP_CANNOT_DELETE	=	0x80040228L;

const LONG CDO_E_BAD_DATA	=	0x80040229L;

const LONG CDO_E_PROP_NONHEADER	=	0x8004022aL;

const LONG CDO_E_INVALID_CHARSET	=	0x8004022bL;

const LONG CDO_E_ADOSTREAM_NOT_BOUND	=	0x8004022cL;

const LONG CDO_E_CONTENTPROPXML_NOT_FOUND	=	0x8004022dL;

const LONG CDO_E_CONTENTPROPXML_WRONG_CHARSET	=	0x8004022eL;

const LONG CDO_E_CONTENTPROPXML_PARSE_FAILED	=	0x8004022fL;

const LONG CDO_E_CONTENTPROPXML_CONVERT_FAILED	=	0x80040230L;

const LONG CDO_E_NO_DIRECTORIES_SPECIFIED	=	0x80040231L;

const LONG CDO_E_DIRECTORIES_UNREACHABLE	=	0x80040232L;

const LONG CDO_E_BAD_SENDER	=	0x80040233L;

const LONG CDO_E_SELF_BINDING	=	0x80040234L;

const LONG CDO_E_BAD_ATTENDEE_DATA	=	0x80040235L;

const LONG CDO_E_ROLE_NOMORE_AVAILABLE	=	0x80040236L;

const LONG CDO_E_BAD_TASKTYPE_ONASSIGN	=	0x80040237L;

const LONG CDO_E_NOT_ASSIGNEDTO_USER	=	0x80040238L;

const LONG CDO_E_OUTOFDATE	=	0x80040239L;

const LONG CDO_E_ARGUMENT1	=	0x80044000L;

const LONG CDO_E_ARGUMENT2	=	0x80044001L;

const LONG CDO_E_ARGUMENT3	=	0x80044002L;

const LONG CDO_E_ARGUMENT4	=	0x80044003L;

const LONG CDO_E_ARGUMENT5	=	0x80044004L;

const LONG CDO_E_NOT_FOUND	=	0x800cce05L;

const LONG CDO_E_INVALID_ENCODING_TYPE	=	0x800cce1dL;

#endif /* __CdoErrors_MODULE_DEFINED__ */

EXTERN_C const CLSID CLSID_Message;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000001-8B95-11D1-82DB-00C04FB1625D")
Message;
#endif

EXTERN_C const CLSID CLSID_Configuration;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000002-8B95-11D1-82DB-00C04FB1625D")
Configuration;
#endif

EXTERN_C const CLSID CLSID_DropDirectory;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000004-8B95-11D1-82DB-00C04FB1625D")
DropDirectory;
#endif

EXTERN_C const CLSID CLSID_SMTPConnector;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000008-8B95-11D1-82DB-00C04FB1625D")
SMTPConnector;
#endif

EXTERN_C const CLSID CLSID_NNTPEarlyConnector;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000011-8B95-11D1-82DB-00C04FB1625D")
NNTPEarlyConnector;
#endif

EXTERN_C const CLSID CLSID_NNTPPostConnector;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000009-8B95-11D1-82DB-00C04FB1625D")
NNTPPostConnector;
#endif

EXTERN_C const CLSID CLSID_NNTPFinalConnector;

#ifdef __cplusplus

class DECLSPEC_UUID("CD000010-8B95-11D1-82DB-00C04FB1625D")
NNTPFinalConnector;
#endif
#endif /* __CDO_LIBRARY_DEFINED__ */
#if defined __cplusplus && !defined CDO_NO_NAMESPACE
} // namespace CDO
#endif

/* interface __MIDL_itf_cdo_0000_0018 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_cdo_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cdo_0000_0018_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


