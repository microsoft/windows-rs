

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

#ifndef __dvbsiparser_h__
#define __dvbsiparser_h__

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

#ifndef __IDvbSiParser_FWD_DEFINED__
#define __IDvbSiParser_FWD_DEFINED__
typedef interface IDvbSiParser IDvbSiParser;

#endif 	/* __IDvbSiParser_FWD_DEFINED__ */


#ifndef __IDvbSiParser2_FWD_DEFINED__
#define __IDvbSiParser2_FWD_DEFINED__
typedef interface IDvbSiParser2 IDvbSiParser2;

#endif 	/* __IDvbSiParser2_FWD_DEFINED__ */


#ifndef __IIsdbSiParser2_FWD_DEFINED__
#define __IIsdbSiParser2_FWD_DEFINED__
typedef interface IIsdbSiParser2 IIsdbSiParser2;

#endif 	/* __IIsdbSiParser2_FWD_DEFINED__ */


#ifndef __IDVB_NIT_FWD_DEFINED__
#define __IDVB_NIT_FWD_DEFINED__
typedef interface IDVB_NIT IDVB_NIT;

#endif 	/* __IDVB_NIT_FWD_DEFINED__ */


#ifndef __IDVB_SDT_FWD_DEFINED__
#define __IDVB_SDT_FWD_DEFINED__
typedef interface IDVB_SDT IDVB_SDT;

#endif 	/* __IDVB_SDT_FWD_DEFINED__ */


#ifndef __IISDB_SDT_FWD_DEFINED__
#define __IISDB_SDT_FWD_DEFINED__
typedef interface IISDB_SDT IISDB_SDT;

#endif 	/* __IISDB_SDT_FWD_DEFINED__ */


#ifndef __IDVB_EIT_FWD_DEFINED__
#define __IDVB_EIT_FWD_DEFINED__
typedef interface IDVB_EIT IDVB_EIT;

#endif 	/* __IDVB_EIT_FWD_DEFINED__ */


#ifndef __IDVB_EIT2_FWD_DEFINED__
#define __IDVB_EIT2_FWD_DEFINED__
typedef interface IDVB_EIT2 IDVB_EIT2;

#endif 	/* __IDVB_EIT2_FWD_DEFINED__ */


#ifndef __IDVB_BAT_FWD_DEFINED__
#define __IDVB_BAT_FWD_DEFINED__
typedef interface IDVB_BAT IDVB_BAT;

#endif 	/* __IDVB_BAT_FWD_DEFINED__ */


#ifndef __IDVB_RST_FWD_DEFINED__
#define __IDVB_RST_FWD_DEFINED__
typedef interface IDVB_RST IDVB_RST;

#endif 	/* __IDVB_RST_FWD_DEFINED__ */


#ifndef __IDVB_ST_FWD_DEFINED__
#define __IDVB_ST_FWD_DEFINED__
typedef interface IDVB_ST IDVB_ST;

#endif 	/* __IDVB_ST_FWD_DEFINED__ */


#ifndef __IDVB_TDT_FWD_DEFINED__
#define __IDVB_TDT_FWD_DEFINED__
typedef interface IDVB_TDT IDVB_TDT;

#endif 	/* __IDVB_TDT_FWD_DEFINED__ */


#ifndef __IDVB_TOT_FWD_DEFINED__
#define __IDVB_TOT_FWD_DEFINED__
typedef interface IDVB_TOT IDVB_TOT;

#endif 	/* __IDVB_TOT_FWD_DEFINED__ */


#ifndef __IDVB_DIT_FWD_DEFINED__
#define __IDVB_DIT_FWD_DEFINED__
typedef interface IDVB_DIT IDVB_DIT;

#endif 	/* __IDVB_DIT_FWD_DEFINED__ */


#ifndef __IDVB_SIT_FWD_DEFINED__
#define __IDVB_SIT_FWD_DEFINED__
typedef interface IDVB_SIT IDVB_SIT;

#endif 	/* __IDVB_SIT_FWD_DEFINED__ */


#ifndef __IISDB_BIT_FWD_DEFINED__
#define __IISDB_BIT_FWD_DEFINED__
typedef interface IISDB_BIT IISDB_BIT;

#endif 	/* __IISDB_BIT_FWD_DEFINED__ */


#ifndef __IISDB_NBIT_FWD_DEFINED__
#define __IISDB_NBIT_FWD_DEFINED__
typedef interface IISDB_NBIT IISDB_NBIT;

#endif 	/* __IISDB_NBIT_FWD_DEFINED__ */


#ifndef __IISDB_LDT_FWD_DEFINED__
#define __IISDB_LDT_FWD_DEFINED__
typedef interface IISDB_LDT IISDB_LDT;

#endif 	/* __IISDB_LDT_FWD_DEFINED__ */


#ifndef __IISDB_SDTT_FWD_DEFINED__
#define __IISDB_SDTT_FWD_DEFINED__
typedef interface IISDB_SDTT IISDB_SDTT;

#endif 	/* __IISDB_SDTT_FWD_DEFINED__ */


#ifndef __IISDB_CDT_FWD_DEFINED__
#define __IISDB_CDT_FWD_DEFINED__
typedef interface IISDB_CDT IISDB_CDT;

#endif 	/* __IISDB_CDT_FWD_DEFINED__ */


#ifndef __IISDB_EMM_FWD_DEFINED__
#define __IISDB_EMM_FWD_DEFINED__
typedef interface IISDB_EMM IISDB_EMM;

#endif 	/* __IISDB_EMM_FWD_DEFINED__ */


#ifndef __IDvbServiceAttributeDescriptor_FWD_DEFINED__
#define __IDvbServiceAttributeDescriptor_FWD_DEFINED__
typedef interface IDvbServiceAttributeDescriptor IDvbServiceAttributeDescriptor;

#endif 	/* __IDvbServiceAttributeDescriptor_FWD_DEFINED__ */


#ifndef __IDvbContentIdentifierDescriptor_FWD_DEFINED__
#define __IDvbContentIdentifierDescriptor_FWD_DEFINED__
typedef interface IDvbContentIdentifierDescriptor IDvbContentIdentifierDescriptor;

#endif 	/* __IDvbContentIdentifierDescriptor_FWD_DEFINED__ */


#ifndef __IDvbDefaultAuthorityDescriptor_FWD_DEFINED__
#define __IDvbDefaultAuthorityDescriptor_FWD_DEFINED__
typedef interface IDvbDefaultAuthorityDescriptor IDvbDefaultAuthorityDescriptor;

#endif 	/* __IDvbDefaultAuthorityDescriptor_FWD_DEFINED__ */


#ifndef __IDvbSatelliteDeliverySystemDescriptor_FWD_DEFINED__
#define __IDvbSatelliteDeliverySystemDescriptor_FWD_DEFINED__
typedef interface IDvbSatelliteDeliverySystemDescriptor IDvbSatelliteDeliverySystemDescriptor;

#endif 	/* __IDvbSatelliteDeliverySystemDescriptor_FWD_DEFINED__ */


#ifndef __IDvbCableDeliverySystemDescriptor_FWD_DEFINED__
#define __IDvbCableDeliverySystemDescriptor_FWD_DEFINED__
typedef interface IDvbCableDeliverySystemDescriptor IDvbCableDeliverySystemDescriptor;

#endif 	/* __IDvbCableDeliverySystemDescriptor_FWD_DEFINED__ */


#ifndef __IDvbTerrestrialDeliverySystemDescriptor_FWD_DEFINED__
#define __IDvbTerrestrialDeliverySystemDescriptor_FWD_DEFINED__
typedef interface IDvbTerrestrialDeliverySystemDescriptor IDvbTerrestrialDeliverySystemDescriptor;

#endif 	/* __IDvbTerrestrialDeliverySystemDescriptor_FWD_DEFINED__ */


#ifndef __IDvbTerrestrial2DeliverySystemDescriptor_FWD_DEFINED__
#define __IDvbTerrestrial2DeliverySystemDescriptor_FWD_DEFINED__
typedef interface IDvbTerrestrial2DeliverySystemDescriptor IDvbTerrestrial2DeliverySystemDescriptor;

#endif 	/* __IDvbTerrestrial2DeliverySystemDescriptor_FWD_DEFINED__ */


#ifndef __IDvbFrequencyListDescriptor_FWD_DEFINED__
#define __IDvbFrequencyListDescriptor_FWD_DEFINED__
typedef interface IDvbFrequencyListDescriptor IDvbFrequencyListDescriptor;

#endif 	/* __IDvbFrequencyListDescriptor_FWD_DEFINED__ */


#ifndef __IDvbPrivateDataSpecifierDescriptor_FWD_DEFINED__
#define __IDvbPrivateDataSpecifierDescriptor_FWD_DEFINED__
typedef interface IDvbPrivateDataSpecifierDescriptor IDvbPrivateDataSpecifierDescriptor;

#endif 	/* __IDvbPrivateDataSpecifierDescriptor_FWD_DEFINED__ */


#ifndef __IDvbLogicalChannelDescriptor_FWD_DEFINED__
#define __IDvbLogicalChannelDescriptor_FWD_DEFINED__
typedef interface IDvbLogicalChannelDescriptor IDvbLogicalChannelDescriptor;

#endif 	/* __IDvbLogicalChannelDescriptor_FWD_DEFINED__ */


#ifndef __IDvbLogicalChannelDescriptor2_FWD_DEFINED__
#define __IDvbLogicalChannelDescriptor2_FWD_DEFINED__
typedef interface IDvbLogicalChannelDescriptor2 IDvbLogicalChannelDescriptor2;

#endif 	/* __IDvbLogicalChannelDescriptor2_FWD_DEFINED__ */


#ifndef __IDvbLogicalChannel2Descriptor_FWD_DEFINED__
#define __IDvbLogicalChannel2Descriptor_FWD_DEFINED__
typedef interface IDvbLogicalChannel2Descriptor IDvbLogicalChannel2Descriptor;

#endif 	/* __IDvbLogicalChannel2Descriptor_FWD_DEFINED__ */


#ifndef __IDvbHDSimulcastLogicalChannelDescriptor_FWD_DEFINED__
#define __IDvbHDSimulcastLogicalChannelDescriptor_FWD_DEFINED__
typedef interface IDvbHDSimulcastLogicalChannelDescriptor IDvbHDSimulcastLogicalChannelDescriptor;

#endif 	/* __IDvbHDSimulcastLogicalChannelDescriptor_FWD_DEFINED__ */


#ifndef __IDvbDataBroadcastIDDescriptor_FWD_DEFINED__
#define __IDvbDataBroadcastIDDescriptor_FWD_DEFINED__
typedef interface IDvbDataBroadcastIDDescriptor IDvbDataBroadcastIDDescriptor;

#endif 	/* __IDvbDataBroadcastIDDescriptor_FWD_DEFINED__ */


#ifndef __IDvbDataBroadcastDescriptor_FWD_DEFINED__
#define __IDvbDataBroadcastDescriptor_FWD_DEFINED__
typedef interface IDvbDataBroadcastDescriptor IDvbDataBroadcastDescriptor;

#endif 	/* __IDvbDataBroadcastDescriptor_FWD_DEFINED__ */


#ifndef __IDvbLinkageDescriptor_FWD_DEFINED__
#define __IDvbLinkageDescriptor_FWD_DEFINED__
typedef interface IDvbLinkageDescriptor IDvbLinkageDescriptor;

#endif 	/* __IDvbLinkageDescriptor_FWD_DEFINED__ */


#ifndef __IDvbTeletextDescriptor_FWD_DEFINED__
#define __IDvbTeletextDescriptor_FWD_DEFINED__
typedef interface IDvbTeletextDescriptor IDvbTeletextDescriptor;

#endif 	/* __IDvbTeletextDescriptor_FWD_DEFINED__ */


#ifndef __IDvbSubtitlingDescriptor_FWD_DEFINED__
#define __IDvbSubtitlingDescriptor_FWD_DEFINED__
typedef interface IDvbSubtitlingDescriptor IDvbSubtitlingDescriptor;

#endif 	/* __IDvbSubtitlingDescriptor_FWD_DEFINED__ */


#ifndef __IDvbServiceDescriptor_FWD_DEFINED__
#define __IDvbServiceDescriptor_FWD_DEFINED__
typedef interface IDvbServiceDescriptor IDvbServiceDescriptor;

#endif 	/* __IDvbServiceDescriptor_FWD_DEFINED__ */


#ifndef __IDvbServiceDescriptor2_FWD_DEFINED__
#define __IDvbServiceDescriptor2_FWD_DEFINED__
typedef interface IDvbServiceDescriptor2 IDvbServiceDescriptor2;

#endif 	/* __IDvbServiceDescriptor2_FWD_DEFINED__ */


#ifndef __IDvbServiceListDescriptor_FWD_DEFINED__
#define __IDvbServiceListDescriptor_FWD_DEFINED__
typedef interface IDvbServiceListDescriptor IDvbServiceListDescriptor;

#endif 	/* __IDvbServiceListDescriptor_FWD_DEFINED__ */


#ifndef __IDvbMultilingualServiceNameDescriptor_FWD_DEFINED__
#define __IDvbMultilingualServiceNameDescriptor_FWD_DEFINED__
typedef interface IDvbMultilingualServiceNameDescriptor IDvbMultilingualServiceNameDescriptor;

#endif 	/* __IDvbMultilingualServiceNameDescriptor_FWD_DEFINED__ */


#ifndef __IDvbNetworkNameDescriptor_FWD_DEFINED__
#define __IDvbNetworkNameDescriptor_FWD_DEFINED__
typedef interface IDvbNetworkNameDescriptor IDvbNetworkNameDescriptor;

#endif 	/* __IDvbNetworkNameDescriptor_FWD_DEFINED__ */


#ifndef __IDvbShortEventDescriptor_FWD_DEFINED__
#define __IDvbShortEventDescriptor_FWD_DEFINED__
typedef interface IDvbShortEventDescriptor IDvbShortEventDescriptor;

#endif 	/* __IDvbShortEventDescriptor_FWD_DEFINED__ */


#ifndef __IDvbExtendedEventDescriptor_FWD_DEFINED__
#define __IDvbExtendedEventDescriptor_FWD_DEFINED__
typedef interface IDvbExtendedEventDescriptor IDvbExtendedEventDescriptor;

#endif 	/* __IDvbExtendedEventDescriptor_FWD_DEFINED__ */


#ifndef __IDvbComponentDescriptor_FWD_DEFINED__
#define __IDvbComponentDescriptor_FWD_DEFINED__
typedef interface IDvbComponentDescriptor IDvbComponentDescriptor;

#endif 	/* __IDvbComponentDescriptor_FWD_DEFINED__ */


#ifndef __IDvbContentDescriptor_FWD_DEFINED__
#define __IDvbContentDescriptor_FWD_DEFINED__
typedef interface IDvbContentDescriptor IDvbContentDescriptor;

#endif 	/* __IDvbContentDescriptor_FWD_DEFINED__ */


#ifndef __IDvbParentalRatingDescriptor_FWD_DEFINED__
#define __IDvbParentalRatingDescriptor_FWD_DEFINED__
typedef interface IDvbParentalRatingDescriptor IDvbParentalRatingDescriptor;

#endif 	/* __IDvbParentalRatingDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbTerrestrialDeliverySystemDescriptor_FWD_DEFINED__
#define __IIsdbTerrestrialDeliverySystemDescriptor_FWD_DEFINED__
typedef interface IIsdbTerrestrialDeliverySystemDescriptor IIsdbTerrestrialDeliverySystemDescriptor;

#endif 	/* __IIsdbTerrestrialDeliverySystemDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbTSInformationDescriptor_FWD_DEFINED__
#define __IIsdbTSInformationDescriptor_FWD_DEFINED__
typedef interface IIsdbTSInformationDescriptor IIsdbTSInformationDescriptor;

#endif 	/* __IIsdbTSInformationDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbDigitalCopyControlDescriptor_FWD_DEFINED__
#define __IIsdbDigitalCopyControlDescriptor_FWD_DEFINED__
typedef interface IIsdbDigitalCopyControlDescriptor IIsdbDigitalCopyControlDescriptor;

#endif 	/* __IIsdbDigitalCopyControlDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbAudioComponentDescriptor_FWD_DEFINED__
#define __IIsdbAudioComponentDescriptor_FWD_DEFINED__
typedef interface IIsdbAudioComponentDescriptor IIsdbAudioComponentDescriptor;

#endif 	/* __IIsdbAudioComponentDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbDataContentDescriptor_FWD_DEFINED__
#define __IIsdbDataContentDescriptor_FWD_DEFINED__
typedef interface IIsdbDataContentDescriptor IIsdbDataContentDescriptor;

#endif 	/* __IIsdbDataContentDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbCAContractInformationDescriptor_FWD_DEFINED__
#define __IIsdbCAContractInformationDescriptor_FWD_DEFINED__
typedef interface IIsdbCAContractInformationDescriptor IIsdbCAContractInformationDescriptor;

#endif 	/* __IIsdbCAContractInformationDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbEventGroupDescriptor_FWD_DEFINED__
#define __IIsdbEventGroupDescriptor_FWD_DEFINED__
typedef interface IIsdbEventGroupDescriptor IIsdbEventGroupDescriptor;

#endif 	/* __IIsdbEventGroupDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbComponentGroupDescriptor_FWD_DEFINED__
#define __IIsdbComponentGroupDescriptor_FWD_DEFINED__
typedef interface IIsdbComponentGroupDescriptor IIsdbComponentGroupDescriptor;

#endif 	/* __IIsdbComponentGroupDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbSeriesDescriptor_FWD_DEFINED__
#define __IIsdbSeriesDescriptor_FWD_DEFINED__
typedef interface IIsdbSeriesDescriptor IIsdbSeriesDescriptor;

#endif 	/* __IIsdbSeriesDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbDownloadContentDescriptor_FWD_DEFINED__
#define __IIsdbDownloadContentDescriptor_FWD_DEFINED__
typedef interface IIsdbDownloadContentDescriptor IIsdbDownloadContentDescriptor;

#endif 	/* __IIsdbDownloadContentDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbLogoTransmissionDescriptor_FWD_DEFINED__
#define __IIsdbLogoTransmissionDescriptor_FWD_DEFINED__
typedef interface IIsdbLogoTransmissionDescriptor IIsdbLogoTransmissionDescriptor;

#endif 	/* __IIsdbLogoTransmissionDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbSIParameterDescriptor_FWD_DEFINED__
#define __IIsdbSIParameterDescriptor_FWD_DEFINED__
typedef interface IIsdbSIParameterDescriptor IIsdbSIParameterDescriptor;

#endif 	/* __IIsdbSIParameterDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbEmergencyInformationDescriptor_FWD_DEFINED__
#define __IIsdbEmergencyInformationDescriptor_FWD_DEFINED__
typedef interface IIsdbEmergencyInformationDescriptor IIsdbEmergencyInformationDescriptor;

#endif 	/* __IIsdbEmergencyInformationDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbCADescriptor_FWD_DEFINED__
#define __IIsdbCADescriptor_FWD_DEFINED__
typedef interface IIsdbCADescriptor IIsdbCADescriptor;

#endif 	/* __IIsdbCADescriptor_FWD_DEFINED__ */


#ifndef __IIsdbCAServiceDescriptor_FWD_DEFINED__
#define __IIsdbCAServiceDescriptor_FWD_DEFINED__
typedef interface IIsdbCAServiceDescriptor IIsdbCAServiceDescriptor;

#endif 	/* __IIsdbCAServiceDescriptor_FWD_DEFINED__ */


#ifndef __IIsdbHierarchicalTransmissionDescriptor_FWD_DEFINED__
#define __IIsdbHierarchicalTransmissionDescriptor_FWD_DEFINED__
typedef interface IIsdbHierarchicalTransmissionDescriptor IIsdbHierarchicalTransmissionDescriptor;

#endif 	/* __IIsdbHierarchicalTransmissionDescriptor_FWD_DEFINED__ */


#ifndef __IPBDASiParser_FWD_DEFINED__
#define __IPBDASiParser_FWD_DEFINED__
typedef interface IPBDASiParser IPBDASiParser;

#endif 	/* __IPBDASiParser_FWD_DEFINED__ */


#ifndef __IPBDA_EIT_FWD_DEFINED__
#define __IPBDA_EIT_FWD_DEFINED__
typedef interface IPBDA_EIT IPBDA_EIT;

#endif 	/* __IPBDA_EIT_FWD_DEFINED__ */


#ifndef __IPBDA_Services_FWD_DEFINED__
#define __IPBDA_Services_FWD_DEFINED__
typedef interface IPBDA_Services IPBDA_Services;

#endif 	/* __IPBDA_Services_FWD_DEFINED__ */


#ifndef __IPBDAEntitlementDescriptor_FWD_DEFINED__
#define __IPBDAEntitlementDescriptor_FWD_DEFINED__
typedef interface IPBDAEntitlementDescriptor IPBDAEntitlementDescriptor;

#endif 	/* __IPBDAEntitlementDescriptor_FWD_DEFINED__ */


#ifndef __IPBDAAttributesDescriptor_FWD_DEFINED__
#define __IPBDAAttributesDescriptor_FWD_DEFINED__
typedef interface IPBDAAttributesDescriptor IPBDAAttributesDescriptor;

#endif 	/* __IPBDAAttributesDescriptor_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "mpeg2structs.h"
#include "mpeg2data.h"
#include "mpeg2psiparser.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_dvbsiparser_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_dvbsiparser_0000_0000_0001
    {
        STRCONV_MODE_DVB	= 0,
        STRCONV_MODE_DVB_EMPHASIS	= ( STRCONV_MODE_DVB + 1 ) ,
        STRCONV_MODE_DVB_WITHOUT_EMPHASIS	= ( STRCONV_MODE_DVB_EMPHASIS + 1 ) ,
        STRCONV_MODE_ISDB	= ( STRCONV_MODE_DVB_WITHOUT_EMPHASIS + 1 ) 
    } 	DVB_STRCONV_MODE;

























extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0000_v0_0_s_ifspec;

#ifndef __IDvbSiParser_INTERFACE_DEFINED__
#define __IDvbSiParser_INTERFACE_DEFINED__

/* interface IDvbSiParser */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbSiParser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B758A7BD-14DC-449d-B828-35909ACB3B1E")
    IDvbSiParser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IUnknown *punkMpeg2Data) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPAT( 
            /* [out] */ IPAT **ppPAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAT( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ ICAT **ppCAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPMT( 
            /* [in] */ PID pid,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwProgramNumber,
            /* [out] */ IPMT **ppPMT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTSDT( 
            /* [out] */ ITSDT **ppTSDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNIT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwNetworkId,
            /* [out] */ IDVB_NIT **ppNIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSDT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTransportStreamId,
            /* [out] */ IDVB_SDT **ppSDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEIT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [out] */ IDVB_EIT **ppEIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBAT( 
            /* [annotation][in] */ 
            _In_opt_  WORD *pwBouquetId,
            /* [out] */ IDVB_BAT **ppBAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRST( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_RST **ppRST) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetST( 
            /* [in] */ PID pid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_ST **ppST) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTDT( 
            /* [out] */ IDVB_TDT **ppTDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTOT( 
            /* [out] */ IDVB_TOT **ppTOT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDIT( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_DIT **ppDIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSIT( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_SIT **ppSIT) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbSiParserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbSiParser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbSiParser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbSiParser * This);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDvbSiParser * This,
            /* [in] */ IUnknown *punkMpeg2Data);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetPAT)
        HRESULT ( STDMETHODCALLTYPE *GetPAT )( 
            IDvbSiParser * This,
            /* [out] */ IPAT **ppPAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetCAT)
        HRESULT ( STDMETHODCALLTYPE *GetCAT )( 
            IDvbSiParser * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ ICAT **ppCAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetPMT)
        HRESULT ( STDMETHODCALLTYPE *GetPMT )( 
            IDvbSiParser * This,
            /* [in] */ PID pid,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwProgramNumber,
            /* [out] */ IPMT **ppPMT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTSDT)
        HRESULT ( STDMETHODCALLTYPE *GetTSDT )( 
            IDvbSiParser * This,
            /* [out] */ ITSDT **ppTSDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetNIT)
        HRESULT ( STDMETHODCALLTYPE *GetNIT )( 
            IDvbSiParser * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwNetworkId,
            /* [out] */ IDVB_NIT **ppNIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetSDT)
        HRESULT ( STDMETHODCALLTYPE *GetSDT )( 
            IDvbSiParser * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTransportStreamId,
            /* [out] */ IDVB_SDT **ppSDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetEIT)
        HRESULT ( STDMETHODCALLTYPE *GetEIT )( 
            IDvbSiParser * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [out] */ IDVB_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetBAT)
        HRESULT ( STDMETHODCALLTYPE *GetBAT )( 
            IDvbSiParser * This,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwBouquetId,
            /* [out] */ IDVB_BAT **ppBAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetRST)
        HRESULT ( STDMETHODCALLTYPE *GetRST )( 
            IDvbSiParser * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_RST **ppRST);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetST)
        HRESULT ( STDMETHODCALLTYPE *GetST )( 
            IDvbSiParser * This,
            /* [in] */ PID pid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_ST **ppST);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTDT)
        HRESULT ( STDMETHODCALLTYPE *GetTDT )( 
            IDvbSiParser * This,
            /* [out] */ IDVB_TDT **ppTDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTOT)
        HRESULT ( STDMETHODCALLTYPE *GetTOT )( 
            IDvbSiParser * This,
            /* [out] */ IDVB_TOT **ppTOT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetDIT)
        HRESULT ( STDMETHODCALLTYPE *GetDIT )( 
            IDvbSiParser * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_DIT **ppDIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetSIT)
        HRESULT ( STDMETHODCALLTYPE *GetSIT )( 
            IDvbSiParser * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_SIT **ppSIT);
        
        END_INTERFACE
    } IDvbSiParserVtbl;

    interface IDvbSiParser
    {
        CONST_VTBL struct IDvbSiParserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbSiParser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbSiParser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbSiParser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbSiParser_Initialize(This,punkMpeg2Data)	\
    ( (This)->lpVtbl -> Initialize(This,punkMpeg2Data) ) 

#define IDvbSiParser_GetPAT(This,ppPAT)	\
    ( (This)->lpVtbl -> GetPAT(This,ppPAT) ) 

#define IDvbSiParser_GetCAT(This,dwTimeout,ppCAT)	\
    ( (This)->lpVtbl -> GetCAT(This,dwTimeout,ppCAT) ) 

#define IDvbSiParser_GetPMT(This,pid,pwProgramNumber,ppPMT)	\
    ( (This)->lpVtbl -> GetPMT(This,pid,pwProgramNumber,ppPMT) ) 

#define IDvbSiParser_GetTSDT(This,ppTSDT)	\
    ( (This)->lpVtbl -> GetTSDT(This,ppTSDT) ) 

#define IDvbSiParser_GetNIT(This,tableId,pwNetworkId,ppNIT)	\
    ( (This)->lpVtbl -> GetNIT(This,tableId,pwNetworkId,ppNIT) ) 

#define IDvbSiParser_GetSDT(This,tableId,pwTransportStreamId,ppSDT)	\
    ( (This)->lpVtbl -> GetSDT(This,tableId,pwTransportStreamId,ppSDT) ) 

#define IDvbSiParser_GetEIT(This,tableId,pwServiceId,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT(This,tableId,pwServiceId,ppEIT) ) 

#define IDvbSiParser_GetBAT(This,pwBouquetId,ppBAT)	\
    ( (This)->lpVtbl -> GetBAT(This,pwBouquetId,ppBAT) ) 

#define IDvbSiParser_GetRST(This,dwTimeout,ppRST)	\
    ( (This)->lpVtbl -> GetRST(This,dwTimeout,ppRST) ) 

#define IDvbSiParser_GetST(This,pid,dwTimeout,ppST)	\
    ( (This)->lpVtbl -> GetST(This,pid,dwTimeout,ppST) ) 

#define IDvbSiParser_GetTDT(This,ppTDT)	\
    ( (This)->lpVtbl -> GetTDT(This,ppTDT) ) 

#define IDvbSiParser_GetTOT(This,ppTOT)	\
    ( (This)->lpVtbl -> GetTOT(This,ppTOT) ) 

#define IDvbSiParser_GetDIT(This,dwTimeout,ppDIT)	\
    ( (This)->lpVtbl -> GetDIT(This,dwTimeout,ppDIT) ) 

#define IDvbSiParser_GetSIT(This,dwTimeout,ppSIT)	\
    ( (This)->lpVtbl -> GetSIT(This,dwTimeout,ppSIT) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbSiParser_INTERFACE_DEFINED__ */


#ifndef __IDvbSiParser2_INTERFACE_DEFINED__
#define __IDvbSiParser2_INTERFACE_DEFINED__

/* interface IDvbSiParser2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbSiParser2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0AC5525F-F816-42F4-93BA-4C0F32F46E54")
    IDvbSiParser2 : public IDvbSiParser
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEIT2( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [annotation][in] */ 
            _In_opt_  BYTE *pbSegment,
            /* [out] */ IDVB_EIT2 **ppEIT) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbSiParser2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbSiParser2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbSiParser2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbSiParser2 * This);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDvbSiParser2 * This,
            /* [in] */ IUnknown *punkMpeg2Data);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetPAT)
        HRESULT ( STDMETHODCALLTYPE *GetPAT )( 
            IDvbSiParser2 * This,
            /* [out] */ IPAT **ppPAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetCAT)
        HRESULT ( STDMETHODCALLTYPE *GetCAT )( 
            IDvbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ ICAT **ppCAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetPMT)
        HRESULT ( STDMETHODCALLTYPE *GetPMT )( 
            IDvbSiParser2 * This,
            /* [in] */ PID pid,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwProgramNumber,
            /* [out] */ IPMT **ppPMT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTSDT)
        HRESULT ( STDMETHODCALLTYPE *GetTSDT )( 
            IDvbSiParser2 * This,
            /* [out] */ ITSDT **ppTSDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetNIT)
        HRESULT ( STDMETHODCALLTYPE *GetNIT )( 
            IDvbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwNetworkId,
            /* [out] */ IDVB_NIT **ppNIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetSDT)
        HRESULT ( STDMETHODCALLTYPE *GetSDT )( 
            IDvbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTransportStreamId,
            /* [out] */ IDVB_SDT **ppSDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetEIT)
        HRESULT ( STDMETHODCALLTYPE *GetEIT )( 
            IDvbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [out] */ IDVB_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetBAT)
        HRESULT ( STDMETHODCALLTYPE *GetBAT )( 
            IDvbSiParser2 * This,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwBouquetId,
            /* [out] */ IDVB_BAT **ppBAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetRST)
        HRESULT ( STDMETHODCALLTYPE *GetRST )( 
            IDvbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_RST **ppRST);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetST)
        HRESULT ( STDMETHODCALLTYPE *GetST )( 
            IDvbSiParser2 * This,
            /* [in] */ PID pid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_ST **ppST);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTDT)
        HRESULT ( STDMETHODCALLTYPE *GetTDT )( 
            IDvbSiParser2 * This,
            /* [out] */ IDVB_TDT **ppTDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTOT)
        HRESULT ( STDMETHODCALLTYPE *GetTOT )( 
            IDvbSiParser2 * This,
            /* [out] */ IDVB_TOT **ppTOT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetDIT)
        HRESULT ( STDMETHODCALLTYPE *GetDIT )( 
            IDvbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_DIT **ppDIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetSIT)
        HRESULT ( STDMETHODCALLTYPE *GetSIT )( 
            IDvbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_SIT **ppSIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser2, GetEIT2)
        HRESULT ( STDMETHODCALLTYPE *GetEIT2 )( 
            IDvbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [annotation][in] */ 
            _In_opt_  BYTE *pbSegment,
            /* [out] */ IDVB_EIT2 **ppEIT);
        
        END_INTERFACE
    } IDvbSiParser2Vtbl;

    interface IDvbSiParser2
    {
        CONST_VTBL struct IDvbSiParser2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbSiParser2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbSiParser2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbSiParser2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbSiParser2_Initialize(This,punkMpeg2Data)	\
    ( (This)->lpVtbl -> Initialize(This,punkMpeg2Data) ) 

#define IDvbSiParser2_GetPAT(This,ppPAT)	\
    ( (This)->lpVtbl -> GetPAT(This,ppPAT) ) 

#define IDvbSiParser2_GetCAT(This,dwTimeout,ppCAT)	\
    ( (This)->lpVtbl -> GetCAT(This,dwTimeout,ppCAT) ) 

#define IDvbSiParser2_GetPMT(This,pid,pwProgramNumber,ppPMT)	\
    ( (This)->lpVtbl -> GetPMT(This,pid,pwProgramNumber,ppPMT) ) 

#define IDvbSiParser2_GetTSDT(This,ppTSDT)	\
    ( (This)->lpVtbl -> GetTSDT(This,ppTSDT) ) 

#define IDvbSiParser2_GetNIT(This,tableId,pwNetworkId,ppNIT)	\
    ( (This)->lpVtbl -> GetNIT(This,tableId,pwNetworkId,ppNIT) ) 

#define IDvbSiParser2_GetSDT(This,tableId,pwTransportStreamId,ppSDT)	\
    ( (This)->lpVtbl -> GetSDT(This,tableId,pwTransportStreamId,ppSDT) ) 

#define IDvbSiParser2_GetEIT(This,tableId,pwServiceId,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT(This,tableId,pwServiceId,ppEIT) ) 

#define IDvbSiParser2_GetBAT(This,pwBouquetId,ppBAT)	\
    ( (This)->lpVtbl -> GetBAT(This,pwBouquetId,ppBAT) ) 

#define IDvbSiParser2_GetRST(This,dwTimeout,ppRST)	\
    ( (This)->lpVtbl -> GetRST(This,dwTimeout,ppRST) ) 

#define IDvbSiParser2_GetST(This,pid,dwTimeout,ppST)	\
    ( (This)->lpVtbl -> GetST(This,pid,dwTimeout,ppST) ) 

#define IDvbSiParser2_GetTDT(This,ppTDT)	\
    ( (This)->lpVtbl -> GetTDT(This,ppTDT) ) 

#define IDvbSiParser2_GetTOT(This,ppTOT)	\
    ( (This)->lpVtbl -> GetTOT(This,ppTOT) ) 

#define IDvbSiParser2_GetDIT(This,dwTimeout,ppDIT)	\
    ( (This)->lpVtbl -> GetDIT(This,dwTimeout,ppDIT) ) 

#define IDvbSiParser2_GetSIT(This,dwTimeout,ppSIT)	\
    ( (This)->lpVtbl -> GetSIT(This,dwTimeout,ppSIT) ) 


#define IDvbSiParser2_GetEIT2(This,tableId,pwServiceId,pbSegment,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT2(This,tableId,pwServiceId,pbSegment,ppEIT) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbSiParser2_INTERFACE_DEFINED__ */


#ifndef __IIsdbSiParser2_INTERFACE_DEFINED__
#define __IIsdbSiParser2_INTERFACE_DEFINED__

/* interface IIsdbSiParser2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbSiParser2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("900E4BB7-18CD-453F-98BE-3BE6AA211772")
    IIsdbSiParser2 : public IDvbSiParser2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSDT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTransportStreamId,
            /* [out] */ IISDB_SDT **ppSDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBIT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwOriginalNetworkId,
            /* [out] */ IISDB_BIT **ppBIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNBIT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwOriginalNetworkId,
            /* [out] */ IISDB_NBIT **ppNBIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLDT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwOriginalServiceId,
            /* [out] */ IISDB_LDT **ppLDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSDTT( 
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTableIdExt,
            /* [out] */ IISDB_SDTT **ppSDTT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCDT( 
            /* [in] */ TID tableId,
            /* [in] */ BYTE bSectionNumber,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwDownloadDataId,
            /* [out] */ IISDB_CDT **ppCDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEMM( 
            /* [in] */ PID pid,
            /* [in] */ WORD wTableIdExt,
            /* [out] */ IISDB_EMM **ppEMM) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbSiParser2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbSiParser2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbSiParser2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbSiParser2 * This);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IIsdbSiParser2 * This,
            /* [in] */ IUnknown *punkMpeg2Data);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetPAT)
        HRESULT ( STDMETHODCALLTYPE *GetPAT )( 
            IIsdbSiParser2 * This,
            /* [out] */ IPAT **ppPAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetCAT)
        HRESULT ( STDMETHODCALLTYPE *GetCAT )( 
            IIsdbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ ICAT **ppCAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetPMT)
        HRESULT ( STDMETHODCALLTYPE *GetPMT )( 
            IIsdbSiParser2 * This,
            /* [in] */ PID pid,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwProgramNumber,
            /* [out] */ IPMT **ppPMT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTSDT)
        HRESULT ( STDMETHODCALLTYPE *GetTSDT )( 
            IIsdbSiParser2 * This,
            /* [out] */ ITSDT **ppTSDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetNIT)
        HRESULT ( STDMETHODCALLTYPE *GetNIT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwNetworkId,
            /* [out] */ IDVB_NIT **ppNIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetSDT)
        HRESULT ( STDMETHODCALLTYPE *GetSDT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTransportStreamId,
            /* [out] */ IDVB_SDT **ppSDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetEIT)
        HRESULT ( STDMETHODCALLTYPE *GetEIT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [out] */ IDVB_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetBAT)
        HRESULT ( STDMETHODCALLTYPE *GetBAT )( 
            IIsdbSiParser2 * This,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwBouquetId,
            /* [out] */ IDVB_BAT **ppBAT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetRST)
        HRESULT ( STDMETHODCALLTYPE *GetRST )( 
            IIsdbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_RST **ppRST);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetST)
        HRESULT ( STDMETHODCALLTYPE *GetST )( 
            IIsdbSiParser2 * This,
            /* [in] */ PID pid,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_ST **ppST);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTDT)
        HRESULT ( STDMETHODCALLTYPE *GetTDT )( 
            IIsdbSiParser2 * This,
            /* [out] */ IDVB_TDT **ppTDT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetTOT)
        HRESULT ( STDMETHODCALLTYPE *GetTOT )( 
            IIsdbSiParser2 * This,
            /* [out] */ IDVB_TOT **ppTOT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetDIT)
        HRESULT ( STDMETHODCALLTYPE *GetDIT )( 
            IIsdbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_DIT **ppDIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser, GetSIT)
        HRESULT ( STDMETHODCALLTYPE *GetSIT )( 
            IIsdbSiParser2 * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_SIT **ppSIT);
        
        DECLSPEC_XFGVIRT(IDvbSiParser2, GetEIT2)
        HRESULT ( STDMETHODCALLTYPE *GetEIT2 )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwServiceId,
            /* [annotation][in] */ 
            _In_opt_  BYTE *pbSegment,
            /* [out] */ IDVB_EIT2 **ppEIT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetSDT)
        HRESULT ( STDMETHODCALLTYPE *GetSDT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTransportStreamId,
            /* [out] */ IISDB_SDT **ppSDT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetBIT)
        HRESULT ( STDMETHODCALLTYPE *GetBIT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwOriginalNetworkId,
            /* [out] */ IISDB_BIT **ppBIT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetNBIT)
        HRESULT ( STDMETHODCALLTYPE *GetNBIT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwOriginalNetworkId,
            /* [out] */ IISDB_NBIT **ppNBIT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetLDT)
        HRESULT ( STDMETHODCALLTYPE *GetLDT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwOriginalServiceId,
            /* [out] */ IISDB_LDT **ppLDT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetSDTT)
        HRESULT ( STDMETHODCALLTYPE *GetSDTT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwTableIdExt,
            /* [out] */ IISDB_SDTT **ppSDTT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetCDT)
        HRESULT ( STDMETHODCALLTYPE *GetCDT )( 
            IIsdbSiParser2 * This,
            /* [in] */ TID tableId,
            /* [in] */ BYTE bSectionNumber,
            /* [annotation][in] */ 
            _In_opt_  WORD *pwDownloadDataId,
            /* [out] */ IISDB_CDT **ppCDT);
        
        DECLSPEC_XFGVIRT(IIsdbSiParser2, GetEMM)
        HRESULT ( STDMETHODCALLTYPE *GetEMM )( 
            IIsdbSiParser2 * This,
            /* [in] */ PID pid,
            /* [in] */ WORD wTableIdExt,
            /* [out] */ IISDB_EMM **ppEMM);
        
        END_INTERFACE
    } IIsdbSiParser2Vtbl;

    interface IIsdbSiParser2
    {
        CONST_VTBL struct IIsdbSiParser2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbSiParser2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbSiParser2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbSiParser2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbSiParser2_Initialize(This,punkMpeg2Data)	\
    ( (This)->lpVtbl -> Initialize(This,punkMpeg2Data) ) 

#define IIsdbSiParser2_GetPAT(This,ppPAT)	\
    ( (This)->lpVtbl -> GetPAT(This,ppPAT) ) 

#define IIsdbSiParser2_GetCAT(This,dwTimeout,ppCAT)	\
    ( (This)->lpVtbl -> GetCAT(This,dwTimeout,ppCAT) ) 

#define IIsdbSiParser2_GetPMT(This,pid,pwProgramNumber,ppPMT)	\
    ( (This)->lpVtbl -> GetPMT(This,pid,pwProgramNumber,ppPMT) ) 

#define IIsdbSiParser2_GetTSDT(This,ppTSDT)	\
    ( (This)->lpVtbl -> GetTSDT(This,ppTSDT) ) 

#define IIsdbSiParser2_GetNIT(This,tableId,pwNetworkId,ppNIT)	\
    ( (This)->lpVtbl -> GetNIT(This,tableId,pwNetworkId,ppNIT) ) 

#define IIsdbSiParser2_GetSDT(This,tableId,pwTransportStreamId,ppSDT)	\
    ( (This)->lpVtbl -> GetSDT(This,tableId,pwTransportStreamId,ppSDT) ) 

#define IIsdbSiParser2_GetEIT(This,tableId,pwServiceId,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT(This,tableId,pwServiceId,ppEIT) ) 

#define IIsdbSiParser2_GetBAT(This,pwBouquetId,ppBAT)	\
    ( (This)->lpVtbl -> GetBAT(This,pwBouquetId,ppBAT) ) 

#define IIsdbSiParser2_GetRST(This,dwTimeout,ppRST)	\
    ( (This)->lpVtbl -> GetRST(This,dwTimeout,ppRST) ) 

#define IIsdbSiParser2_GetST(This,pid,dwTimeout,ppST)	\
    ( (This)->lpVtbl -> GetST(This,pid,dwTimeout,ppST) ) 

#define IIsdbSiParser2_GetTDT(This,ppTDT)	\
    ( (This)->lpVtbl -> GetTDT(This,ppTDT) ) 

#define IIsdbSiParser2_GetTOT(This,ppTOT)	\
    ( (This)->lpVtbl -> GetTOT(This,ppTOT) ) 

#define IIsdbSiParser2_GetDIT(This,dwTimeout,ppDIT)	\
    ( (This)->lpVtbl -> GetDIT(This,dwTimeout,ppDIT) ) 

#define IIsdbSiParser2_GetSIT(This,dwTimeout,ppSIT)	\
    ( (This)->lpVtbl -> GetSIT(This,dwTimeout,ppSIT) ) 


#define IIsdbSiParser2_GetEIT2(This,tableId,pwServiceId,pbSegment,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT2(This,tableId,pwServiceId,pbSegment,ppEIT) ) 


#define IIsdbSiParser2_GetSDT(This,tableId,pwTransportStreamId,ppSDT)	\
    ( (This)->lpVtbl -> GetSDT(This,tableId,pwTransportStreamId,ppSDT) ) 

#define IIsdbSiParser2_GetBIT(This,tableId,pwOriginalNetworkId,ppBIT)	\
    ( (This)->lpVtbl -> GetBIT(This,tableId,pwOriginalNetworkId,ppBIT) ) 

#define IIsdbSiParser2_GetNBIT(This,tableId,pwOriginalNetworkId,ppNBIT)	\
    ( (This)->lpVtbl -> GetNBIT(This,tableId,pwOriginalNetworkId,ppNBIT) ) 

#define IIsdbSiParser2_GetLDT(This,tableId,pwOriginalServiceId,ppLDT)	\
    ( (This)->lpVtbl -> GetLDT(This,tableId,pwOriginalServiceId,ppLDT) ) 

#define IIsdbSiParser2_GetSDTT(This,tableId,pwTableIdExt,ppSDTT)	\
    ( (This)->lpVtbl -> GetSDTT(This,tableId,pwTableIdExt,ppSDTT) ) 

#define IIsdbSiParser2_GetCDT(This,tableId,bSectionNumber,pwDownloadDataId,ppCDT)	\
    ( (This)->lpVtbl -> GetCDT(This,tableId,bSectionNumber,pwDownloadDataId,ppCDT) ) 

#define IIsdbSiParser2_GetEMM(This,pid,wTableIdExt,ppEMM)	\
    ( (This)->lpVtbl -> GetEMM(This,pid,wTableIdExt,ppEMM) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbSiParser2_INTERFACE_DEFINED__ */


#ifndef __IDVB_NIT_INTERFACE_DEFINED__
#define __IDVB_NIT_INTERFACE_DEFINED__

/* interface IDVB_NIT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_NIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C64935F4-29E4-4e22-911A-63F7F55CB097")
    IDVB_NIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTransportStreamId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordOriginalNetworkId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ IDVB_NIT **ppNIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_NITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_NIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_NIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_NIT * This);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_NIT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IDVB_NIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetNetworkId )( 
            IDVB_NIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IDVB_NIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IDVB_NIT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IDVB_NIT * This,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_NIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetRecordTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTransportStreamId )( 
            IDVB_NIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetRecordOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordOriginalNetworkId )( 
            IDVB_NIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IDVB_NIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IDVB_NIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IDVB_NIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IDVB_NIT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IDVB_NIT * This,
            /* [out] */ IDVB_NIT **ppNIT);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IDVB_NIT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IDVB_NIT * This);
        
        DECLSPEC_XFGVIRT(IDVB_NIT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IDVB_NIT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IDVB_NITVtbl;

    interface IDVB_NIT
    {
        CONST_VTBL struct IDVB_NITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_NIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_NIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_NIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_NIT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IDVB_NIT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IDVB_NIT_GetNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetNetworkId(This,pwVal) ) 

#define IDVB_NIT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IDVB_NIT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IDVB_NIT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_NIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_NIT_GetRecordTransportStreamId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordTransportStreamId(This,dwRecordIndex,pwVal) ) 

#define IDVB_NIT_GetRecordOriginalNetworkId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordOriginalNetworkId(This,dwRecordIndex,pwVal) ) 

#define IDVB_NIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IDVB_NIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IDVB_NIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_NIT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IDVB_NIT_GetNextTable(This,ppNIT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppNIT) ) 

#define IDVB_NIT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IDVB_NIT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#define IDVB_NIT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_NIT_INTERFACE_DEFINED__ */


#ifndef __IDVB_SDT_INTERFACE_DEFINED__
#define __IDVB_SDT_INTERFACE_DEFINED__

/* interface IDVB_SDT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_SDT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02CAD8D3-FE43-48e2-90BD-450ED9A8A5FD")
    IDVB_SDT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportStreamId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEITScheduleFlag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEITPresentFollowingFlag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRunningStatus( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordFreeCAMode( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ IDVB_SDT **ppSDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_SDTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_SDT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_SDT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_SDT * This);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_SDT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IDVB_SDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IDVB_SDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IDVB_SDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_SDT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordEITScheduleFlag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEITScheduleFlag )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordEITPresentFollowingFlag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEITPresentFollowingFlag )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordRunningStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRunningStatus )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordFreeCAMode)
        HRESULT ( STDMETHODCALLTYPE *GetRecordFreeCAMode )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IDVB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IDVB_SDT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IDVB_SDT * This,
            /* [out] */ IDVB_SDT **ppSDT);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IDVB_SDT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IDVB_SDT * This);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IDVB_SDT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IDVB_SDTVtbl;

    interface IDVB_SDT
    {
        CONST_VTBL struct IDVB_SDTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_SDT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_SDT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_SDT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_SDT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IDVB_SDT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IDVB_SDT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IDVB_SDT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IDVB_SDT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_SDT_GetRecordServiceId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,dwRecordIndex,pwVal) ) 

#define IDVB_SDT_GetRecordEITScheduleFlag(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordEITScheduleFlag(This,dwRecordIndex,pfVal) ) 

#define IDVB_SDT_GetRecordEITPresentFollowingFlag(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordEITPresentFollowingFlag(This,dwRecordIndex,pfVal) ) 

#define IDVB_SDT_GetRecordRunningStatus(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRunningStatus(This,dwRecordIndex,pbVal) ) 

#define IDVB_SDT_GetRecordFreeCAMode(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordFreeCAMode(This,dwRecordIndex,pfVal) ) 

#define IDVB_SDT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IDVB_SDT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IDVB_SDT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_SDT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IDVB_SDT_GetNextTable(This,ppSDT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppSDT) ) 

#define IDVB_SDT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IDVB_SDT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#define IDVB_SDT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_SDT_INTERFACE_DEFINED__ */


#ifndef __IISDB_SDT_INTERFACE_DEFINED__
#define __IISDB_SDT_INTERFACE_DEFINED__

/* interface IISDB_SDT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_SDT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3F3DC9A2-BB32-4FB9-AE9E-D856848927A3")
    IISDB_SDT : public IDVB_SDT
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRecordEITUserDefinedFlags( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_SDTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_SDT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_SDT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_SDT * This);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_SDT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_SDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IISDB_SDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IISDB_SDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IISDB_SDT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordEITScheduleFlag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEITScheduleFlag )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordEITPresentFollowingFlag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEITPresentFollowingFlag )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordRunningStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRunningStatus )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordFreeCAMode)
        HRESULT ( STDMETHODCALLTYPE *GetRecordFreeCAMode )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IISDB_SDT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IISDB_SDT * This,
            /* [out] */ IDVB_SDT **ppSDT);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IISDB_SDT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IISDB_SDT * This);
        
        DECLSPEC_XFGVIRT(IDVB_SDT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_SDT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        DECLSPEC_XFGVIRT(IISDB_SDT, GetRecordEITUserDefinedFlags)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEITUserDefinedFlags )( 
            IISDB_SDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IISDB_SDTVtbl;

    interface IISDB_SDT
    {
        CONST_VTBL struct IISDB_SDTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_SDT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_SDT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_SDT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_SDT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IISDB_SDT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_SDT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IISDB_SDT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IISDB_SDT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IISDB_SDT_GetRecordServiceId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,dwRecordIndex,pwVal) ) 

#define IISDB_SDT_GetRecordEITScheduleFlag(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordEITScheduleFlag(This,dwRecordIndex,pfVal) ) 

#define IISDB_SDT_GetRecordEITPresentFollowingFlag(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordEITPresentFollowingFlag(This,dwRecordIndex,pfVal) ) 

#define IISDB_SDT_GetRecordRunningStatus(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRunningStatus(This,dwRecordIndex,pbVal) ) 

#define IISDB_SDT_GetRecordFreeCAMode(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordFreeCAMode(This,dwRecordIndex,pfVal) ) 

#define IISDB_SDT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IISDB_SDT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IISDB_SDT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_SDT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IISDB_SDT_GetNextTable(This,ppSDT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppSDT) ) 

#define IISDB_SDT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IISDB_SDT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#define IISDB_SDT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 


#define IISDB_SDT_GetRecordEITUserDefinedFlags(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordEITUserDefinedFlags(This,dwRecordIndex,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_SDT_INTERFACE_DEFINED__ */


#ifndef __IDVB_EIT_INTERFACE_DEFINED__
#define __IDVB_EIT_INTERFACE_DEFINED__

/* interface IDVB_EIT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_EIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("442DB029-02CB-4495-8B92-1C13375BCE99")
    IDVB_EIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportStreamId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentLastSectionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastTableId( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEventId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordStartTime( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDuration( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DURATION *pmdVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRunningStatus( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordFreeCAMode( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ IDVB_EIT **ppEIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_EITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_EIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_EIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_EIT * This);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_EIT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IDVB_EIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetServiceId )( 
            IDVB_EIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IDVB_EIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IDVB_EIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetSegmentLastSectionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentLastSectionNumber )( 
            IDVB_EIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetLastTableId)
        HRESULT ( STDMETHODCALLTYPE *GetLastTableId )( 
            IDVB_EIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_EIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordEventId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEventId )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordStartTime)
        HRESULT ( STDMETHODCALLTYPE *GetRecordStartTime )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordDuration)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDuration )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DURATION *pmdVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordRunningStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRunningStatus )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordFreeCAMode)
        HRESULT ( STDMETHODCALLTYPE *GetRecordFreeCAMode )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IDVB_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IDVB_EIT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IDVB_EIT * This,
            /* [out] */ IDVB_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IDVB_EIT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IDVB_EIT * This);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IDVB_EIT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IDVB_EITVtbl;

    interface IDVB_EIT
    {
        CONST_VTBL struct IDVB_EITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_EIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_EIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_EIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_EIT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IDVB_EIT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IDVB_EIT_GetServiceId(This,pwVal)	\
    ( (This)->lpVtbl -> GetServiceId(This,pwVal) ) 

#define IDVB_EIT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IDVB_EIT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IDVB_EIT_GetSegmentLastSectionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetSegmentLastSectionNumber(This,pbVal) ) 

#define IDVB_EIT_GetLastTableId(This,pbVal)	\
    ( (This)->lpVtbl -> GetLastTableId(This,pbVal) ) 

#define IDVB_EIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_EIT_GetRecordEventId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordEventId(This,dwRecordIndex,pwVal) ) 

#define IDVB_EIT_GetRecordStartTime(This,dwRecordIndex,pmdtVal)	\
    ( (This)->lpVtbl -> GetRecordStartTime(This,dwRecordIndex,pmdtVal) ) 

#define IDVB_EIT_GetRecordDuration(This,dwRecordIndex,pmdVal)	\
    ( (This)->lpVtbl -> GetRecordDuration(This,dwRecordIndex,pmdVal) ) 

#define IDVB_EIT_GetRecordRunningStatus(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRunningStatus(This,dwRecordIndex,pbVal) ) 

#define IDVB_EIT_GetRecordFreeCAMode(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordFreeCAMode(This,dwRecordIndex,pfVal) ) 

#define IDVB_EIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IDVB_EIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IDVB_EIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_EIT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IDVB_EIT_GetNextTable(This,ppEIT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppEIT) ) 

#define IDVB_EIT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IDVB_EIT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#define IDVB_EIT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_EIT_INTERFACE_DEFINED__ */


#ifndef __IDVB_EIT2_INTERFACE_DEFINED__
#define __IDVB_EIT2_INTERFACE_DEFINED__

/* interface IDVB_EIT2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_EIT2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("61A389E0-9B9E-4ba0-AEEA-5DDD159820EA")
    IDVB_EIT2 : public IDVB_EIT
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSegmentInfo( 
            /* [out] */ BYTE *pbTid,
            /* [out] */ BYTE *pbSegment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordSection( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_EIT2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_EIT2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_EIT2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_EIT2 * This);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_EIT2 * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IDVB_EIT2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetServiceId )( 
            IDVB_EIT2 * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IDVB_EIT2 * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IDVB_EIT2 * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetSegmentLastSectionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentLastSectionNumber )( 
            IDVB_EIT2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetLastTableId)
        HRESULT ( STDMETHODCALLTYPE *GetLastTableId )( 
            IDVB_EIT2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_EIT2 * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordEventId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEventId )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordStartTime)
        HRESULT ( STDMETHODCALLTYPE *GetRecordStartTime )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordDuration)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDuration )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DURATION *pmdVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordRunningStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRunningStatus )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordFreeCAMode)
        HRESULT ( STDMETHODCALLTYPE *GetRecordFreeCAMode )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IDVB_EIT2 * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IDVB_EIT2 * This,
            /* [out] */ IDVB_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IDVB_EIT2 * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IDVB_EIT2 * This);
        
        DECLSPEC_XFGVIRT(IDVB_EIT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IDVB_EIT2 * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        DECLSPEC_XFGVIRT(IDVB_EIT2, GetSegmentInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentInfo )( 
            IDVB_EIT2 * This,
            /* [out] */ BYTE *pbTid,
            /* [out] */ BYTE *pbSegment);
        
        DECLSPEC_XFGVIRT(IDVB_EIT2, GetRecordSection)
        HRESULT ( STDMETHODCALLTYPE *GetRecordSection )( 
            IDVB_EIT2 * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDVB_EIT2Vtbl;

    interface IDVB_EIT2
    {
        CONST_VTBL struct IDVB_EIT2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_EIT2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_EIT2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_EIT2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_EIT2_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IDVB_EIT2_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IDVB_EIT2_GetServiceId(This,pwVal)	\
    ( (This)->lpVtbl -> GetServiceId(This,pwVal) ) 

#define IDVB_EIT2_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IDVB_EIT2_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IDVB_EIT2_GetSegmentLastSectionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetSegmentLastSectionNumber(This,pbVal) ) 

#define IDVB_EIT2_GetLastTableId(This,pbVal)	\
    ( (This)->lpVtbl -> GetLastTableId(This,pbVal) ) 

#define IDVB_EIT2_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_EIT2_GetRecordEventId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordEventId(This,dwRecordIndex,pwVal) ) 

#define IDVB_EIT2_GetRecordStartTime(This,dwRecordIndex,pmdtVal)	\
    ( (This)->lpVtbl -> GetRecordStartTime(This,dwRecordIndex,pmdtVal) ) 

#define IDVB_EIT2_GetRecordDuration(This,dwRecordIndex,pmdVal)	\
    ( (This)->lpVtbl -> GetRecordDuration(This,dwRecordIndex,pmdVal) ) 

#define IDVB_EIT2_GetRecordRunningStatus(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRunningStatus(This,dwRecordIndex,pbVal) ) 

#define IDVB_EIT2_GetRecordFreeCAMode(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordFreeCAMode(This,dwRecordIndex,pfVal) ) 

#define IDVB_EIT2_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IDVB_EIT2_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IDVB_EIT2_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_EIT2_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IDVB_EIT2_GetNextTable(This,ppEIT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppEIT) ) 

#define IDVB_EIT2_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IDVB_EIT2_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#define IDVB_EIT2_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 


#define IDVB_EIT2_GetSegmentInfo(This,pbTid,pbSegment)	\
    ( (This)->lpVtbl -> GetSegmentInfo(This,pbTid,pbSegment) ) 

#define IDVB_EIT2_GetRecordSection(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordSection(This,dwRecordIndex,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_EIT2_INTERFACE_DEFINED__ */


#ifndef __IDVB_BAT_INTERFACE_DEFINED__
#define __IDVB_BAT_INTERFACE_DEFINED__

/* interface IDVB_BAT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_BAT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ECE9BB0C-43B6-4558-A0EC-1812C34CD6CA")
    IDVB_BAT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBouquetId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [in] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTransportStreamId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordOriginalNetworkId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ IDVB_BAT **ppBAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_BATVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_BAT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_BAT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_BAT * This);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_BAT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IDVB_BAT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetBouquetId)
        HRESULT ( STDMETHODCALLTYPE *GetBouquetId )( 
            IDVB_BAT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IDVB_BAT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IDVB_BAT * This,
            /* [in] */ DWORD dwIndex,
            /* [in] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IDVB_BAT * This,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_BAT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetRecordTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTransportStreamId )( 
            IDVB_BAT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetRecordOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordOriginalNetworkId )( 
            IDVB_BAT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IDVB_BAT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IDVB_BAT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IDVB_BAT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IDVB_BAT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IDVB_BAT * This,
            /* [out] */ IDVB_BAT **ppBAT);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IDVB_BAT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_BAT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IDVB_BAT * This);
        
        END_INTERFACE
    } IDVB_BATVtbl;

    interface IDVB_BAT
    {
        CONST_VTBL struct IDVB_BATVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_BAT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_BAT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_BAT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_BAT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IDVB_BAT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IDVB_BAT_GetBouquetId(This,pwVal)	\
    ( (This)->lpVtbl -> GetBouquetId(This,pwVal) ) 

#define IDVB_BAT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IDVB_BAT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IDVB_BAT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_BAT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_BAT_GetRecordTransportStreamId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordTransportStreamId(This,dwRecordIndex,pwVal) ) 

#define IDVB_BAT_GetRecordOriginalNetworkId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordOriginalNetworkId(This,dwRecordIndex,pwVal) ) 

#define IDVB_BAT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IDVB_BAT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IDVB_BAT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_BAT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IDVB_BAT_GetNextTable(This,ppBAT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppBAT) ) 

#define IDVB_BAT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IDVB_BAT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_BAT_INTERFACE_DEFINED__ */


#ifndef __IDVB_RST_INTERFACE_DEFINED__
#define __IDVB_RST_INTERFACE_DEFINED__

/* interface IDVB_RST */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_RST;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F47DCD04-1E23-4fb7-9F96-B40EEAD10B2B")
    IDVB_RST : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTransportStreamId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordOriginalNetworkId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEventId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRunningStatus( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_RSTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_RST * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_RST * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_RST * This);
        
        DECLSPEC_XFGVIRT(IDVB_RST, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_RST * This,
            /* [in] */ ISectionList *pSectionList);
        
        DECLSPEC_XFGVIRT(IDVB_RST, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_RST * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_RST, GetRecordTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTransportStreamId )( 
            IDVB_RST * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_RST, GetRecordOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordOriginalNetworkId )( 
            IDVB_RST * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_RST, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDVB_RST * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_RST, GetRecordEventId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEventId )( 
            IDVB_RST * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_RST, GetRecordRunningStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRunningStatus )( 
            IDVB_RST * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDVB_RSTVtbl;

    interface IDVB_RST
    {
        CONST_VTBL struct IDVB_RSTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_RST_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_RST_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_RST_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_RST_Initialize(This,pSectionList)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList) ) 

#define IDVB_RST_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_RST_GetRecordTransportStreamId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordTransportStreamId(This,dwRecordIndex,pwVal) ) 

#define IDVB_RST_GetRecordOriginalNetworkId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordOriginalNetworkId(This,dwRecordIndex,pwVal) ) 

#define IDVB_RST_GetRecordServiceId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,dwRecordIndex,pwVal) ) 

#define IDVB_RST_GetRecordEventId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordEventId(This,dwRecordIndex,pwVal) ) 

#define IDVB_RST_GetRecordRunningStatus(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRunningStatus(This,dwRecordIndex,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_RST_INTERFACE_DEFINED__ */


#ifndef __IDVB_ST_INTERFACE_DEFINED__
#define __IDVB_ST_INTERFACE_DEFINED__

/* interface IDVB_ST */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_ST;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4D5B9F23-2A02-45de-BCDA-5D5DBFBFBE62")
    IDVB_ST : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataLength( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [out] */ BYTE **ppData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_STVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_ST * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_ST * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_ST * This);
        
        DECLSPEC_XFGVIRT(IDVB_ST, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_ST * This,
            /* [in] */ ISectionList *pSectionList);
        
        DECLSPEC_XFGVIRT(IDVB_ST, GetDataLength)
        HRESULT ( STDMETHODCALLTYPE *GetDataLength )( 
            IDVB_ST * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_ST, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IDVB_ST * This,
            /* [out] */ BYTE **ppData);
        
        END_INTERFACE
    } IDVB_STVtbl;

    interface IDVB_ST
    {
        CONST_VTBL struct IDVB_STVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_ST_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_ST_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_ST_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_ST_Initialize(This,pSectionList)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList) ) 

#define IDVB_ST_GetDataLength(This,pwVal)	\
    ( (This)->lpVtbl -> GetDataLength(This,pwVal) ) 

#define IDVB_ST_GetData(This,ppData)	\
    ( (This)->lpVtbl -> GetData(This,ppData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_ST_INTERFACE_DEFINED__ */


#ifndef __IDVB_TDT_INTERFACE_DEFINED__
#define __IDVB_TDT_INTERFACE_DEFINED__

/* interface IDVB_TDT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_TDT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0780DC7D-D55C-4aef-97E6-6B75906E2796")
    IDVB_TDT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUTCTime( 
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_TDTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_TDT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_TDT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_TDT * This);
        
        DECLSPEC_XFGVIRT(IDVB_TDT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_TDT * This,
            /* [in] */ ISectionList *pSectionList);
        
        DECLSPEC_XFGVIRT(IDVB_TDT, GetUTCTime)
        HRESULT ( STDMETHODCALLTYPE *GetUTCTime )( 
            IDVB_TDT * This,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        END_INTERFACE
    } IDVB_TDTVtbl;

    interface IDVB_TDT
    {
        CONST_VTBL struct IDVB_TDTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_TDT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_TDT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_TDT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_TDT_Initialize(This,pSectionList)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList) ) 

#define IDVB_TDT_GetUTCTime(This,pmdtVal)	\
    ( (This)->lpVtbl -> GetUTCTime(This,pmdtVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_TDT_INTERFACE_DEFINED__ */


#ifndef __IDVB_TOT_INTERFACE_DEFINED__
#define __IDVB_TOT_INTERFACE_DEFINED__

/* interface IDVB_TOT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_TOT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83295D6A-FABA-4ee1-9B15-8067696910AE")
    IDVB_TOT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUTCTime( 
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_TOTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_TOT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_TOT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_TOT * This);
        
        DECLSPEC_XFGVIRT(IDVB_TOT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_TOT * This,
            /* [in] */ ISectionList *pSectionList);
        
        DECLSPEC_XFGVIRT(IDVB_TOT, GetUTCTime)
        HRESULT ( STDMETHODCALLTYPE *GetUTCTime )( 
            IDVB_TOT * This,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IDVB_TOT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IDVB_TOT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_TOT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IDVB_TOT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_TOT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IDVB_TOT * This,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } IDVB_TOTVtbl;

    interface IDVB_TOT
    {
        CONST_VTBL struct IDVB_TOTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_TOT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_TOT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_TOT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_TOT_Initialize(This,pSectionList)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList) ) 

#define IDVB_TOT_GetUTCTime(This,pmdtVal)	\
    ( (This)->lpVtbl -> GetUTCTime(This,pmdtVal) ) 

#define IDVB_TOT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IDVB_TOT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IDVB_TOT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_TOT_INTERFACE_DEFINED__ */


#ifndef __IDVB_DIT_INTERFACE_DEFINED__
#define __IDVB_DIT_INTERFACE_DEFINED__

/* interface IDVB_DIT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_DIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("91BFFDF9-9432-410f-86EF-1C228ED0AD70")
    IDVB_DIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransitionFlag( 
            /* [out] */ BOOL *pfVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_DITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_DIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_DIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_DIT * This);
        
        DECLSPEC_XFGVIRT(IDVB_DIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_DIT * This,
            /* [in] */ ISectionList *pSectionList);
        
        DECLSPEC_XFGVIRT(IDVB_DIT, GetTransitionFlag)
        HRESULT ( STDMETHODCALLTYPE *GetTransitionFlag )( 
            IDVB_DIT * This,
            /* [out] */ BOOL *pfVal);
        
        END_INTERFACE
    } IDVB_DITVtbl;

    interface IDVB_DIT
    {
        CONST_VTBL struct IDVB_DITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_DIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_DIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_DIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_DIT_Initialize(This,pSectionList)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList) ) 

#define IDVB_DIT_GetTransitionFlag(This,pfVal)	\
    ( (This)->lpVtbl -> GetTransitionFlag(This,pfVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_DIT_INTERFACE_DEFINED__ */


#ifndef __IDVB_SIT_INTERFACE_DEFINED__
#define __IDVB_SIT_INTERFACE_DEFINED__

/* interface IDVB_SIT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVB_SIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68CDCE53-8BEA-45c2-9D9D-ACF575A089B5")
    IDVB_SIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRunningStatus( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_SIT **ppSIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVB_SITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVB_SIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVB_SIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVB_SIT * This);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IDVB_SIT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IDVB_SIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IDVB_SIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IDVB_SIT * This,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDVB_SIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetRecordRunningStatus)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRunningStatus )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IDVB_SIT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IDVB_SIT * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ IDVB_SIT **ppSIT);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IDVB_SIT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IDVB_SIT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IDVB_SIT * This);
        
        END_INTERFACE
    } IDVB_SITVtbl;

    interface IDVB_SIT
    {
        CONST_VTBL struct IDVB_SITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVB_SIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVB_SIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVB_SIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVB_SIT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IDVB_SIT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IDVB_SIT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IDVB_SIT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IDVB_SIT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_SIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IDVB_SIT_GetRecordServiceId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,dwRecordIndex,pwVal) ) 

#define IDVB_SIT_GetRecordRunningStatus(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRunningStatus(This,dwRecordIndex,pbVal) ) 

#define IDVB_SIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IDVB_SIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IDVB_SIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IDVB_SIT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IDVB_SIT_GetNextTable(This,dwTimeout,ppSIT)	\
    ( (This)->lpVtbl -> GetNextTable(This,dwTimeout,ppSIT) ) 

#define IDVB_SIT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IDVB_SIT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVB_SIT_INTERFACE_DEFINED__ */


#ifndef __IISDB_BIT_INTERFACE_DEFINED__
#define __IISDB_BIT_INTERFACE_DEFINED__

/* interface IISDB_BIT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_BIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("537CD71E-0E46-4173-9001-BA043F3E49E2")
    IISDB_BIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBroadcastViewPropriety( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordBroadcasterId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_BITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_BIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_BIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_BIT * This);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_BIT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_BIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IISDB_BIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetBroadcastViewPropriety)
        HRESULT ( STDMETHODCALLTYPE *GetBroadcastViewPropriety )( 
            IISDB_BIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IISDB_BIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IISDB_BIT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IISDB_BIT * This,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IISDB_BIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetRecordBroadcasterId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordBroadcasterId )( 
            IISDB_BIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IISDB_BIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IISDB_BIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IISDB_BIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_BIT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_BIT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IISDB_BITVtbl;

    interface IISDB_BIT
    {
        CONST_VTBL struct IISDB_BITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_BIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_BIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_BIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_BIT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IISDB_BIT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_BIT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IISDB_BIT_GetBroadcastViewPropriety(This,pbVal)	\
    ( (This)->lpVtbl -> GetBroadcastViewPropriety(This,pbVal) ) 

#define IISDB_BIT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IISDB_BIT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IISDB_BIT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_BIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IISDB_BIT_GetRecordBroadcasterId(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordBroadcasterId(This,dwRecordIndex,pbVal) ) 

#define IISDB_BIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IISDB_BIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IISDB_BIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_BIT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_BIT_INTERFACE_DEFINED__ */


#ifndef __IISDB_NBIT_INTERFACE_DEFINED__
#define __IISDB_NBIT_INTERFACE_DEFINED__

/* interface IISDB_NBIT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_NBIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1B1863EF-08F1-40B7-A559-3B1EFF8CAFA6")
    IISDB_NBIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordInformationId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordInformationType( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptionBodyLocation( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordMessageSectionNumber( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordUserDefined( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordNumberOfKeys( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordKeys( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE **pbKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_NBITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_NBIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_NBIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_NBIT * This);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_NBIT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_NBIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IISDB_NBIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IISDB_NBIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordInformationId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordInformationId )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordInformationType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordInformationType )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordDescriptionBodyLocation)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptionBodyLocation )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordMessageSectionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordMessageSectionNumber )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordUserDefined)
        HRESULT ( STDMETHODCALLTYPE *GetRecordUserDefined )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordNumberOfKeys)
        HRESULT ( STDMETHODCALLTYPE *GetRecordNumberOfKeys )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordKeys)
        HRESULT ( STDMETHODCALLTYPE *GetRecordKeys )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE **pbKeys);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IISDB_NBIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_NBIT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_NBIT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IISDB_NBITVtbl;

    interface IISDB_NBIT
    {
        CONST_VTBL struct IISDB_NBITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_NBIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_NBIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_NBIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_NBIT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IISDB_NBIT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_NBIT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IISDB_NBIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IISDB_NBIT_GetRecordInformationId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordInformationId(This,dwRecordIndex,pwVal) ) 

#define IISDB_NBIT_GetRecordInformationType(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordInformationType(This,dwRecordIndex,pbVal) ) 

#define IISDB_NBIT_GetRecordDescriptionBodyLocation(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordDescriptionBodyLocation(This,dwRecordIndex,pbVal) ) 

#define IISDB_NBIT_GetRecordMessageSectionNumber(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordMessageSectionNumber(This,dwRecordIndex,pbVal) ) 

#define IISDB_NBIT_GetRecordUserDefined(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordUserDefined(This,dwRecordIndex,pbVal) ) 

#define IISDB_NBIT_GetRecordNumberOfKeys(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordNumberOfKeys(This,dwRecordIndex,pbVal) ) 

#define IISDB_NBIT_GetRecordKeys(This,dwRecordIndex,pbKeys)	\
    ( (This)->lpVtbl -> GetRecordKeys(This,dwRecordIndex,pbKeys) ) 

#define IISDB_NBIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IISDB_NBIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IISDB_NBIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_NBIT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_NBIT_INTERFACE_DEFINED__ */


#ifndef __IISDB_LDT_INTERFACE_DEFINED__
#define __IISDB_LDT_INTERFACE_DEFINED__

/* interface IISDB_LDT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_LDT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("141A546B-02FF-4FB9-A3A3-2F074B74A9A9")
    IISDB_LDT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalServiceId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportStreamId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptionId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_LDTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_LDT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_LDT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_LDT * This);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_LDT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_LDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetOriginalServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalServiceId )( 
            IISDB_LDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IISDB_LDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IISDB_LDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IISDB_LDT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetRecordDescriptionId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptionId )( 
            IISDB_LDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IISDB_LDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IISDB_LDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IISDB_LDT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_LDT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_LDT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IISDB_LDTVtbl;

    interface IISDB_LDT
    {
        CONST_VTBL struct IISDB_LDTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_LDT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_LDT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_LDT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_LDT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IISDB_LDT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_LDT_GetOriginalServiceId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalServiceId(This,pwVal) ) 

#define IISDB_LDT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IISDB_LDT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IISDB_LDT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IISDB_LDT_GetRecordDescriptionId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordDescriptionId(This,dwRecordIndex,pwVal) ) 

#define IISDB_LDT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IISDB_LDT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IISDB_LDT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_LDT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_LDT_INTERFACE_DEFINED__ */


#ifndef __IISDB_SDTT_INTERFACE_DEFINED__
#define __IISDB_SDTT_INTERFACE_DEFINED__

/* interface IISDB_SDTT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_SDTT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE60EF2D-813A-4DC7-BF92-EA13DAC85313")
    IISDB_SDTT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableIdExt( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportStreamId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordGroup( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTargetVersion( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordNewVersion( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDownloadLevel( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordVersionIndicator( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordScheduleTimeShiftInformation( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfSchedules( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordStartTimeByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDurationByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ MPEG_DURATION *pmdVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_SDTTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_SDTT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_SDTT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_SDTT * This);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_SDTT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_SDTT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetTableIdExt)
        HRESULT ( STDMETHODCALLTYPE *GetTableIdExt )( 
            IISDB_SDTT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IISDB_SDTT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IISDB_SDTT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetServiceId )( 
            IISDB_SDTT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IISDB_SDTT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordGroup)
        HRESULT ( STDMETHODCALLTYPE *GetRecordGroup )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordTargetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTargetVersion )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordNewVersion)
        HRESULT ( STDMETHODCALLTYPE *GetRecordNewVersion )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordDownloadLevel)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDownloadLevel )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordVersionIndicator)
        HRESULT ( STDMETHODCALLTYPE *GetRecordVersionIndicator )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordScheduleTimeShiftInformation)
        HRESULT ( STDMETHODCALLTYPE *GetRecordScheduleTimeShiftInformation )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordCountOfSchedules)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfSchedules )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordStartTimeByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordStartTimeByIndex )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordDurationByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDurationByIndex )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ MPEG_DURATION *pmdVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IISDB_SDTT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_SDTT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_SDTT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IISDB_SDTTVtbl;

    interface IISDB_SDTT
    {
        CONST_VTBL struct IISDB_SDTTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_SDTT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_SDTT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_SDTT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_SDTT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IISDB_SDTT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_SDTT_GetTableIdExt(This,pwVal)	\
    ( (This)->lpVtbl -> GetTableIdExt(This,pwVal) ) 

#define IISDB_SDTT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IISDB_SDTT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IISDB_SDTT_GetServiceId(This,pwVal)	\
    ( (This)->lpVtbl -> GetServiceId(This,pwVal) ) 

#define IISDB_SDTT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IISDB_SDTT_GetRecordGroup(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordGroup(This,dwRecordIndex,pbVal) ) 

#define IISDB_SDTT_GetRecordTargetVersion(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordTargetVersion(This,dwRecordIndex,pwVal) ) 

#define IISDB_SDTT_GetRecordNewVersion(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordNewVersion(This,dwRecordIndex,pwVal) ) 

#define IISDB_SDTT_GetRecordDownloadLevel(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordDownloadLevel(This,dwRecordIndex,pbVal) ) 

#define IISDB_SDTT_GetRecordVersionIndicator(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordVersionIndicator(This,dwRecordIndex,pbVal) ) 

#define IISDB_SDTT_GetRecordScheduleTimeShiftInformation(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordScheduleTimeShiftInformation(This,dwRecordIndex,pbVal) ) 

#define IISDB_SDTT_GetRecordCountOfSchedules(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfSchedules(This,dwRecordIndex,pdwVal) ) 

#define IISDB_SDTT_GetRecordStartTimeByIndex(This,dwRecordIndex,dwIndex,pmdtVal)	\
    ( (This)->lpVtbl -> GetRecordStartTimeByIndex(This,dwRecordIndex,dwIndex,pmdtVal) ) 

#define IISDB_SDTT_GetRecordDurationByIndex(This,dwRecordIndex,dwIndex,pmdVal)	\
    ( (This)->lpVtbl -> GetRecordDurationByIndex(This,dwRecordIndex,dwIndex,pmdVal) ) 

#define IISDB_SDTT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IISDB_SDTT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IISDB_SDTT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_SDTT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_SDTT_INTERFACE_DEFINED__ */


#ifndef __IISDB_CDT_INTERFACE_DEFINED__
#define __IISDB_CDT_INTERFACE_DEFINED__

/* interface IISDB_CDT */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_CDT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25FA92C2-8B80-4787-A841-3A0E8F17984B")
    IISDB_CDT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData,
            /* [in] */ BYTE bSectionNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDownloadDataId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSectionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalNetworkId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSizeOfDataModule( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataModule( 
            /* [out] */ BYTE **pbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_CDTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_CDT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_CDT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_CDT * This);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_CDT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData,
            /* [in] */ BYTE bSectionNumber);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_CDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetDownloadDataId)
        HRESULT ( STDMETHODCALLTYPE *GetDownloadDataId )( 
            IISDB_CDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetSectionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSectionNumber )( 
            IISDB_CDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetOriginalNetworkId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalNetworkId )( 
            IISDB_CDT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetDataType)
        HRESULT ( STDMETHODCALLTYPE *GetDataType )( 
            IISDB_CDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IISDB_CDT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IISDB_CDT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IISDB_CDT * This,
            /* [in] */ BYTE bTag,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetSizeOfDataModule)
        HRESULT ( STDMETHODCALLTYPE *GetSizeOfDataModule )( 
            IISDB_CDT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetDataModule)
        HRESULT ( STDMETHODCALLTYPE *GetDataModule )( 
            IISDB_CDT * This,
            /* [out] */ BYTE **pbData);
        
        DECLSPEC_XFGVIRT(IISDB_CDT, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_CDT * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IISDB_CDTVtbl;

    interface IISDB_CDT
    {
        CONST_VTBL struct IISDB_CDTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_CDT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_CDT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_CDT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_CDT_Initialize(This,pSectionList,pMPEGData,bSectionNumber)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData,bSectionNumber) ) 

#define IISDB_CDT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_CDT_GetDownloadDataId(This,pwVal)	\
    ( (This)->lpVtbl -> GetDownloadDataId(This,pwVal) ) 

#define IISDB_CDT_GetSectionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetSectionNumber(This,pbVal) ) 

#define IISDB_CDT_GetOriginalNetworkId(This,pwVal)	\
    ( (This)->lpVtbl -> GetOriginalNetworkId(This,pwVal) ) 

#define IISDB_CDT_GetDataType(This,pbVal)	\
    ( (This)->lpVtbl -> GetDataType(This,pbVal) ) 

#define IISDB_CDT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IISDB_CDT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IISDB_CDT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define IISDB_CDT_GetSizeOfDataModule(This,pdwVal)	\
    ( (This)->lpVtbl -> GetSizeOfDataModule(This,pdwVal) ) 

#define IISDB_CDT_GetDataModule(This,pbData)	\
    ( (This)->lpVtbl -> GetDataModule(This,pbData) ) 

#define IISDB_CDT_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_CDT_INTERFACE_DEFINED__ */


#ifndef __IISDB_EMM_INTERFACE_DEFINED__
#define __IISDB_EMM_INTERFACE_DEFINED__

/* interface IISDB_EMM */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IISDB_EMM;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0EDB556D-43AD-4938-9668-321B2FFECFD3")
    IISDB_EMM : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableIdExtension( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataBytes( 
            /* [out][in] */ WORD *pwBufferLength,
            /* [out] */ BYTE *pbBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSharedEmmMessage( 
            WORD *pwLength,
            BYTE **ppbMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndividualEmmMessage( 
            IUnknown *pUnknown,
            WORD *pwLength,
            BYTE **ppbMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionHash( 
            /* [out] */ DWORD *pdwVersionHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IISDB_EMMVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IISDB_EMM * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IISDB_EMM * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IISDB_EMM * This);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IISDB_EMM * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IISDB_EMM * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, GetTableIdExtension)
        HRESULT ( STDMETHODCALLTYPE *GetTableIdExtension )( 
            IISDB_EMM * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, GetDataBytes)
        HRESULT ( STDMETHODCALLTYPE *GetDataBytes )( 
            IISDB_EMM * This,
            /* [out][in] */ WORD *pwBufferLength,
            /* [out] */ BYTE *pbBuffer);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, GetSharedEmmMessage)
        HRESULT ( STDMETHODCALLTYPE *GetSharedEmmMessage )( 
            IISDB_EMM * This,
            WORD *pwLength,
            BYTE **ppbMessage);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, GetIndividualEmmMessage)
        HRESULT ( STDMETHODCALLTYPE *GetIndividualEmmMessage )( 
            IISDB_EMM * This,
            IUnknown *pUnknown,
            WORD *pwLength,
            BYTE **ppbMessage);
        
        DECLSPEC_XFGVIRT(IISDB_EMM, GetVersionHash)
        HRESULT ( STDMETHODCALLTYPE *GetVersionHash )( 
            IISDB_EMM * This,
            /* [out] */ DWORD *pdwVersionHash);
        
        END_INTERFACE
    } IISDB_EMMVtbl;

    interface IISDB_EMM
    {
        CONST_VTBL struct IISDB_EMMVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDB_EMM_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDB_EMM_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDB_EMM_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDB_EMM_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IISDB_EMM_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IISDB_EMM_GetTableIdExtension(This,pwVal)	\
    ( (This)->lpVtbl -> GetTableIdExtension(This,pwVal) ) 

#define IISDB_EMM_GetDataBytes(This,pwBufferLength,pbBuffer)	\
    ( (This)->lpVtbl -> GetDataBytes(This,pwBufferLength,pbBuffer) ) 

#define IISDB_EMM_GetSharedEmmMessage(This,pwLength,ppbMessage)	\
    ( (This)->lpVtbl -> GetSharedEmmMessage(This,pwLength,ppbMessage) ) 

#define IISDB_EMM_GetIndividualEmmMessage(This,pUnknown,pwLength,ppbMessage)	\
    ( (This)->lpVtbl -> GetIndividualEmmMessage(This,pUnknown,pwLength,ppbMessage) ) 

#define IISDB_EMM_GetVersionHash(This,pdwVersionHash)	\
    ( (This)->lpVtbl -> GetVersionHash(This,pdwVersionHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDB_EMM_INTERFACE_DEFINED__ */


#ifndef __IDvbServiceAttributeDescriptor_INTERFACE_DEFINED__
#define __IDvbServiceAttributeDescriptor_INTERFACE_DEFINED__

/* interface IDvbServiceAttributeDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbServiceAttributeDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0F37BD92-D6A1-4854-B950-3A969D27F30E")
    IDvbServiceAttributeDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordNumericSelectionFlag( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordVisibleServiceFlag( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BOOL *pfVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbServiceAttributeDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbServiceAttributeDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbServiceAttributeDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbServiceAttributeDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbServiceAttributeDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbServiceAttributeDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceAttributeDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbServiceAttributeDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceAttributeDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbServiceAttributeDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceAttributeDescriptor, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDvbServiceAttributeDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceAttributeDescriptor, GetRecordNumericSelectionFlag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordNumericSelectionFlag )( 
            IDvbServiceAttributeDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceAttributeDescriptor, GetRecordVisibleServiceFlag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordVisibleServiceFlag )( 
            IDvbServiceAttributeDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BOOL *pfVal);
        
        END_INTERFACE
    } IDvbServiceAttributeDescriptorVtbl;

    interface IDvbServiceAttributeDescriptor
    {
        CONST_VTBL struct IDvbServiceAttributeDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbServiceAttributeDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbServiceAttributeDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbServiceAttributeDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbServiceAttributeDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbServiceAttributeDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbServiceAttributeDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbServiceAttributeDescriptor_GetRecordServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,bRecordIndex,pwVal) ) 

#define IDvbServiceAttributeDescriptor_GetRecordNumericSelectionFlag(This,bRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordNumericSelectionFlag(This,bRecordIndex,pfVal) ) 

#define IDvbServiceAttributeDescriptor_GetRecordVisibleServiceFlag(This,bRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordVisibleServiceFlag(This,bRecordIndex,pfVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbServiceAttributeDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dvbsiparser_0000_0022 */
/* [local] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_dvbsiparser_0000_0022_0001
    {
        CRID_LOCATION_IN_DESCRIPTOR	= 0,
        CRID_LOCATION_IN_CIT	= 0x1,
        CRID_LOCATION_DVB_RESERVED1	= 0x2,
        CRID_LOCATION_DVB_RESERVED2	= 0x3
    } 	CRID_LOCATION;



extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0022_v0_0_s_ifspec;

#ifndef __IDvbContentIdentifierDescriptor_INTERFACE_DEFINED__
#define __IDvbContentIdentifierDescriptor_INTERFACE_DEFINED__

/* interface IDvbContentIdentifierDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbContentIdentifierDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05E0C1EA-F661-4053-9FBF-D93B28359838")
    IDvbContentIdentifierDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCrid( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbType,
            /* [out] */ BYTE *pbLocation,
            /* [out] */ BYTE *pbLength,
            /* [size_is][size_is][out] */ BYTE **ppbBytes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbContentIdentifierDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbContentIdentifierDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbContentIdentifierDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbContentIdentifierDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbContentIdentifierDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbContentIdentifierDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbContentIdentifierDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbContentIdentifierDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbContentIdentifierDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbContentIdentifierDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbContentIdentifierDescriptor, GetRecordCrid)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCrid )( 
            IDvbContentIdentifierDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbType,
            /* [out] */ BYTE *pbLocation,
            /* [out] */ BYTE *pbLength,
            /* [size_is][size_is][out] */ BYTE **ppbBytes);
        
        END_INTERFACE
    } IDvbContentIdentifierDescriptorVtbl;

    interface IDvbContentIdentifierDescriptor
    {
        CONST_VTBL struct IDvbContentIdentifierDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbContentIdentifierDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbContentIdentifierDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbContentIdentifierDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbContentIdentifierDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbContentIdentifierDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbContentIdentifierDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbContentIdentifierDescriptor_GetRecordCrid(This,bRecordIndex,pbType,pbLocation,pbLength,ppbBytes)	\
    ( (This)->lpVtbl -> GetRecordCrid(This,bRecordIndex,pbType,pbLocation,pbLength,ppbBytes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbContentIdentifierDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbDefaultAuthorityDescriptor_INTERFACE_DEFINED__
#define __IDvbDefaultAuthorityDescriptor_INTERFACE_DEFINED__

/* interface IDvbDefaultAuthorityDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbDefaultAuthorityDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05EC24D1-3A31-44e7-B408-67C60A352276")
    IDvbDefaultAuthorityDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultAuthority( 
            /* [out] */ BYTE *pbLength,
            /* [size_is][size_is][out] */ BYTE **ppbBytes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbDefaultAuthorityDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbDefaultAuthorityDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbDefaultAuthorityDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbDefaultAuthorityDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbDefaultAuthorityDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbDefaultAuthorityDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDefaultAuthorityDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbDefaultAuthorityDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDefaultAuthorityDescriptor, GetDefaultAuthority)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultAuthority )( 
            IDvbDefaultAuthorityDescriptor * This,
            /* [out] */ BYTE *pbLength,
            /* [size_is][size_is][out] */ BYTE **ppbBytes);
        
        END_INTERFACE
    } IDvbDefaultAuthorityDescriptorVtbl;

    interface IDvbDefaultAuthorityDescriptor
    {
        CONST_VTBL struct IDvbDefaultAuthorityDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbDefaultAuthorityDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbDefaultAuthorityDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbDefaultAuthorityDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbDefaultAuthorityDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbDefaultAuthorityDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbDefaultAuthorityDescriptor_GetDefaultAuthority(This,pbLength,ppbBytes)	\
    ( (This)->lpVtbl -> GetDefaultAuthority(This,pbLength,ppbBytes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbDefaultAuthorityDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbSatelliteDeliverySystemDescriptor_INTERFACE_DEFINED__
#define __IDvbSatelliteDeliverySystemDescriptor_INTERFACE_DEFINED__

/* interface IDvbSatelliteDeliverySystemDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbSatelliteDeliverySystemDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02F2225A-805B-4ec5-A9A6-F9B5913CD470")
    IDvbSatelliteDeliverySystemDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrequency( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOrbitalPosition( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWestEastFlag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPolarization( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetModulation( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSymbolRate( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFECInner( 
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbSatelliteDeliverySystemDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbSatelliteDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbSatelliteDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetFrequency )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetOrbitalPosition)
        HRESULT ( STDMETHODCALLTYPE *GetOrbitalPosition )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetWestEastFlag)
        HRESULT ( STDMETHODCALLTYPE *GetWestEastFlag )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetPolarization)
        HRESULT ( STDMETHODCALLTYPE *GetPolarization )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetModulation)
        HRESULT ( STDMETHODCALLTYPE *GetModulation )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetSymbolRate)
        HRESULT ( STDMETHODCALLTYPE *GetSymbolRate )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDvbSatelliteDeliverySystemDescriptor, GetFECInner)
        HRESULT ( STDMETHODCALLTYPE *GetFECInner )( 
            IDvbSatelliteDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbSatelliteDeliverySystemDescriptorVtbl;

    interface IDvbSatelliteDeliverySystemDescriptor
    {
        CONST_VTBL struct IDvbSatelliteDeliverySystemDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbSatelliteDeliverySystemDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbSatelliteDeliverySystemDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbSatelliteDeliverySystemDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbSatelliteDeliverySystemDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetFrequency(This,pdwVal)	\
    ( (This)->lpVtbl -> GetFrequency(This,pdwVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetOrbitalPosition(This,pwVal)	\
    ( (This)->lpVtbl -> GetOrbitalPosition(This,pwVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetWestEastFlag(This,pbVal)	\
    ( (This)->lpVtbl -> GetWestEastFlag(This,pbVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetPolarization(This,pbVal)	\
    ( (This)->lpVtbl -> GetPolarization(This,pbVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetModulation(This,pbVal)	\
    ( (This)->lpVtbl -> GetModulation(This,pbVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetSymbolRate(This,pdwVal)	\
    ( (This)->lpVtbl -> GetSymbolRate(This,pdwVal) ) 

#define IDvbSatelliteDeliverySystemDescriptor_GetFECInner(This,pbVal)	\
    ( (This)->lpVtbl -> GetFECInner(This,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbSatelliteDeliverySystemDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbCableDeliverySystemDescriptor_INTERFACE_DEFINED__
#define __IDvbCableDeliverySystemDescriptor_INTERFACE_DEFINED__

/* interface IDvbCableDeliverySystemDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbCableDeliverySystemDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DFB98E36-9E1A-4862-9946-993A4E59017B")
    IDvbCableDeliverySystemDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrequency( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFECOuter( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetModulation( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSymbolRate( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFECInner( 
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbCableDeliverySystemDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbCableDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbCableDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetFrequency )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetFECOuter)
        HRESULT ( STDMETHODCALLTYPE *GetFECOuter )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetModulation)
        HRESULT ( STDMETHODCALLTYPE *GetModulation )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetSymbolRate)
        HRESULT ( STDMETHODCALLTYPE *GetSymbolRate )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDvbCableDeliverySystemDescriptor, GetFECInner)
        HRESULT ( STDMETHODCALLTYPE *GetFECInner )( 
            IDvbCableDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbCableDeliverySystemDescriptorVtbl;

    interface IDvbCableDeliverySystemDescriptor
    {
        CONST_VTBL struct IDvbCableDeliverySystemDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbCableDeliverySystemDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbCableDeliverySystemDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbCableDeliverySystemDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbCableDeliverySystemDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbCableDeliverySystemDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbCableDeliverySystemDescriptor_GetFrequency(This,pdwVal)	\
    ( (This)->lpVtbl -> GetFrequency(This,pdwVal) ) 

#define IDvbCableDeliverySystemDescriptor_GetFECOuter(This,pbVal)	\
    ( (This)->lpVtbl -> GetFECOuter(This,pbVal) ) 

#define IDvbCableDeliverySystemDescriptor_GetModulation(This,pbVal)	\
    ( (This)->lpVtbl -> GetModulation(This,pbVal) ) 

#define IDvbCableDeliverySystemDescriptor_GetSymbolRate(This,pdwVal)	\
    ( (This)->lpVtbl -> GetSymbolRate(This,pdwVal) ) 

#define IDvbCableDeliverySystemDescriptor_GetFECInner(This,pbVal)	\
    ( (This)->lpVtbl -> GetFECInner(This,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbCableDeliverySystemDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbTerrestrialDeliverySystemDescriptor_INTERFACE_DEFINED__
#define __IDvbTerrestrialDeliverySystemDescriptor_INTERFACE_DEFINED__

/* interface IDvbTerrestrialDeliverySystemDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbTerrestrialDeliverySystemDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ED7E1B91-D12E-420c-B41D-A49D84FE1823")
    IDvbTerrestrialDeliverySystemDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCentreFrequency( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBandwidth( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConstellation( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHierarchyInformation( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodeRateHPStream( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodeRateLPStream( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuardInterval( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransmissionMode( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOtherFrequencyFlag( 
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbTerrestrialDeliverySystemDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbTerrestrialDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbTerrestrialDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetCentreFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetCentreFrequency )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetBandwidth)
        HRESULT ( STDMETHODCALLTYPE *GetBandwidth )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetConstellation)
        HRESULT ( STDMETHODCALLTYPE *GetConstellation )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetHierarchyInformation)
        HRESULT ( STDMETHODCALLTYPE *GetHierarchyInformation )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetCodeRateHPStream)
        HRESULT ( STDMETHODCALLTYPE *GetCodeRateHPStream )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetCodeRateLPStream)
        HRESULT ( STDMETHODCALLTYPE *GetCodeRateLPStream )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetGuardInterval)
        HRESULT ( STDMETHODCALLTYPE *GetGuardInterval )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetTransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *GetTransmissionMode )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrialDeliverySystemDescriptor, GetOtherFrequencyFlag)
        HRESULT ( STDMETHODCALLTYPE *GetOtherFrequencyFlag )( 
            IDvbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbTerrestrialDeliverySystemDescriptorVtbl;

    interface IDvbTerrestrialDeliverySystemDescriptor
    {
        CONST_VTBL struct IDvbTerrestrialDeliverySystemDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbTerrestrialDeliverySystemDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbTerrestrialDeliverySystemDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetCentreFrequency(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCentreFrequency(This,pdwVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetBandwidth(This,pbVal)	\
    ( (This)->lpVtbl -> GetBandwidth(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetConstellation(This,pbVal)	\
    ( (This)->lpVtbl -> GetConstellation(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetHierarchyInformation(This,pbVal)	\
    ( (This)->lpVtbl -> GetHierarchyInformation(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetCodeRateHPStream(This,pbVal)	\
    ( (This)->lpVtbl -> GetCodeRateHPStream(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetCodeRateLPStream(This,pbVal)	\
    ( (This)->lpVtbl -> GetCodeRateLPStream(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetGuardInterval(This,pbVal)	\
    ( (This)->lpVtbl -> GetGuardInterval(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetTransmissionMode(This,pbVal)	\
    ( (This)->lpVtbl -> GetTransmissionMode(This,pbVal) ) 

#define IDvbTerrestrialDeliverySystemDescriptor_GetOtherFrequencyFlag(This,pbVal)	\
    ( (This)->lpVtbl -> GetOtherFrequencyFlag(This,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbTerrestrialDeliverySystemDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbTerrestrial2DeliverySystemDescriptor_INTERFACE_DEFINED__
#define __IDvbTerrestrial2DeliverySystemDescriptor_INTERFACE_DEFINED__

/* interface IDvbTerrestrial2DeliverySystemDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbTerrestrial2DeliverySystemDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20EE9BE9-CD57-49ab-8F6E-1D07AEB8E482")
    IDvbTerrestrial2DeliverySystemDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTagExtension( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCentreFrequency( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPLPId( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetT2SystemId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMultipleInputMode( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBandwidth( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuardInterval( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransmissionMode( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCellId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOtherFrequencyFlag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTFSFlag( 
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbTerrestrial2DeliverySystemDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetTagExtension)
        HRESULT ( STDMETHODCALLTYPE *GetTagExtension )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetCentreFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetCentreFrequency )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetPLPId)
        HRESULT ( STDMETHODCALLTYPE *GetPLPId )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetT2SystemId)
        HRESULT ( STDMETHODCALLTYPE *GetT2SystemId )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetMultipleInputMode)
        HRESULT ( STDMETHODCALLTYPE *GetMultipleInputMode )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetBandwidth)
        HRESULT ( STDMETHODCALLTYPE *GetBandwidth )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetGuardInterval)
        HRESULT ( STDMETHODCALLTYPE *GetGuardInterval )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetTransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *GetTransmissionMode )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetCellId)
        HRESULT ( STDMETHODCALLTYPE *GetCellId )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetOtherFrequencyFlag)
        HRESULT ( STDMETHODCALLTYPE *GetOtherFrequencyFlag )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTerrestrial2DeliverySystemDescriptor, GetTFSFlag)
        HRESULT ( STDMETHODCALLTYPE *GetTFSFlag )( 
            IDvbTerrestrial2DeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbTerrestrial2DeliverySystemDescriptorVtbl;

    interface IDvbTerrestrial2DeliverySystemDescriptor
    {
        CONST_VTBL struct IDvbTerrestrial2DeliverySystemDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbTerrestrial2DeliverySystemDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbTerrestrial2DeliverySystemDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetTagExtension(This,pbVal)	\
    ( (This)->lpVtbl -> GetTagExtension(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetCentreFrequency(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCentreFrequency(This,pdwVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetPLPId(This,pbVal)	\
    ( (This)->lpVtbl -> GetPLPId(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetT2SystemId(This,pwVal)	\
    ( (This)->lpVtbl -> GetT2SystemId(This,pwVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetMultipleInputMode(This,pbVal)	\
    ( (This)->lpVtbl -> GetMultipleInputMode(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetBandwidth(This,pbVal)	\
    ( (This)->lpVtbl -> GetBandwidth(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetGuardInterval(This,pbVal)	\
    ( (This)->lpVtbl -> GetGuardInterval(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetTransmissionMode(This,pbVal)	\
    ( (This)->lpVtbl -> GetTransmissionMode(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetCellId(This,pwVal)	\
    ( (This)->lpVtbl -> GetCellId(This,pwVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetOtherFrequencyFlag(This,pbVal)	\
    ( (This)->lpVtbl -> GetOtherFrequencyFlag(This,pbVal) ) 

#define IDvbTerrestrial2DeliverySystemDescriptor_GetTFSFlag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTFSFlag(This,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbTerrestrial2DeliverySystemDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbFrequencyListDescriptor_INTERFACE_DEFINED__
#define __IDvbFrequencyListDescriptor_INTERFACE_DEFINED__

/* interface IDvbFrequencyListDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbFrequencyListDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1CADB613-E1DD-4512-AFA8-BB7A007EF8B1")
    IDvbFrequencyListDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodingType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCentreFrequency( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbFrequencyListDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbFrequencyListDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbFrequencyListDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbFrequencyListDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbFrequencyListDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbFrequencyListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbFrequencyListDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbFrequencyListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbFrequencyListDescriptor, GetCodingType)
        HRESULT ( STDMETHODCALLTYPE *GetCodingType )( 
            IDvbFrequencyListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbFrequencyListDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbFrequencyListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbFrequencyListDescriptor, GetRecordCentreFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCentreFrequency )( 
            IDvbFrequencyListDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        END_INTERFACE
    } IDvbFrequencyListDescriptorVtbl;

    interface IDvbFrequencyListDescriptor
    {
        CONST_VTBL struct IDvbFrequencyListDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbFrequencyListDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbFrequencyListDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbFrequencyListDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbFrequencyListDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbFrequencyListDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbFrequencyListDescriptor_GetCodingType(This,pbVal)	\
    ( (This)->lpVtbl -> GetCodingType(This,pbVal) ) 

#define IDvbFrequencyListDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbFrequencyListDescriptor_GetRecordCentreFrequency(This,bRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCentreFrequency(This,bRecordIndex,pdwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbFrequencyListDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbPrivateDataSpecifierDescriptor_INTERFACE_DEFINED__
#define __IDvbPrivateDataSpecifierDescriptor_INTERFACE_DEFINED__

/* interface IDvbPrivateDataSpecifierDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbPrivateDataSpecifierDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5660A019-E75A-4b82-9B4C-ED2256D165A2")
    IDvbPrivateDataSpecifierDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateDataSpecifier( 
            /* [out] */ DWORD *pdwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbPrivateDataSpecifierDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbPrivateDataSpecifierDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbPrivateDataSpecifierDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbPrivateDataSpecifierDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbPrivateDataSpecifierDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbPrivateDataSpecifierDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbPrivateDataSpecifierDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbPrivateDataSpecifierDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbPrivateDataSpecifierDescriptor, GetPrivateDataSpecifier)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateDataSpecifier )( 
            IDvbPrivateDataSpecifierDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        END_INTERFACE
    } IDvbPrivateDataSpecifierDescriptorVtbl;

    interface IDvbPrivateDataSpecifierDescriptor
    {
        CONST_VTBL struct IDvbPrivateDataSpecifierDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbPrivateDataSpecifierDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbPrivateDataSpecifierDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbPrivateDataSpecifierDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbPrivateDataSpecifierDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbPrivateDataSpecifierDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbPrivateDataSpecifierDescriptor_GetPrivateDataSpecifier(This,pdwVal)	\
    ( (This)->lpVtbl -> GetPrivateDataSpecifier(This,pdwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbPrivateDataSpecifierDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbLogicalChannelDescriptor_INTERFACE_DEFINED__
#define __IDvbLogicalChannelDescriptor_INTERFACE_DEFINED__

/* interface IDvbLogicalChannelDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbLogicalChannelDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF1EDAFF-3FFD-4cf7-8201-35756ACBF85F")
    IDvbLogicalChannelDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordLogicalChannelNumber( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbLogicalChannelDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbLogicalChannelDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbLogicalChannelDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbLogicalChannelDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbLogicalChannelDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbLogicalChannelDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbLogicalChannelDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDvbLogicalChannelDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordLogicalChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelNumber )( 
            IDvbLogicalChannelDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IDvbLogicalChannelDescriptorVtbl;

    interface IDvbLogicalChannelDescriptor
    {
        CONST_VTBL struct IDvbLogicalChannelDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbLogicalChannelDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbLogicalChannelDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbLogicalChannelDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbLogicalChannelDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbLogicalChannelDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbLogicalChannelDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbLogicalChannelDescriptor_GetRecordServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,bRecordIndex,pwVal) ) 

#define IDvbLogicalChannelDescriptor_GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbLogicalChannelDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbLogicalChannelDescriptor2_INTERFACE_DEFINED__
#define __IDvbLogicalChannelDescriptor2_INTERFACE_DEFINED__

/* interface IDvbLogicalChannelDescriptor2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbLogicalChannelDescriptor2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43ACA974-4BE8-4b98-BC17-9EAFD788B1D7")
    IDvbLogicalChannelDescriptor2 : public IDvbLogicalChannelDescriptor
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRecordLogicalChannelAndVisibility( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbLogicalChannelDescriptor2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbLogicalChannelDescriptor2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbLogicalChannelDescriptor2 * This);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordLogicalChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelNumber )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor2, GetRecordLogicalChannelAndVisibility)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelAndVisibility )( 
            IDvbLogicalChannelDescriptor2 * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IDvbLogicalChannelDescriptor2Vtbl;

    interface IDvbLogicalChannelDescriptor2
    {
        CONST_VTBL struct IDvbLogicalChannelDescriptor2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbLogicalChannelDescriptor2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbLogicalChannelDescriptor2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbLogicalChannelDescriptor2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbLogicalChannelDescriptor2_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbLogicalChannelDescriptor2_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbLogicalChannelDescriptor2_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbLogicalChannelDescriptor2_GetRecordServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,bRecordIndex,pwVal) ) 

#define IDvbLogicalChannelDescriptor2_GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal) ) 


#define IDvbLogicalChannelDescriptor2_GetRecordLogicalChannelAndVisibility(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelAndVisibility(This,bRecordIndex,pwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbLogicalChannelDescriptor2_INTERFACE_DEFINED__ */


#ifndef __IDvbLogicalChannel2Descriptor_INTERFACE_DEFINED__
#define __IDvbLogicalChannel2Descriptor_INTERFACE_DEFINED__

/* interface IDvbLogicalChannel2Descriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbLogicalChannel2Descriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F69C3747-8A30-4980-998C-01FE7F0BA35A")
    IDvbLogicalChannel2Descriptor : public IDvbLogicalChannelDescriptor2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCountOfLists( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListId( 
            /* [in] */ BYTE bListIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListNameW( 
            /* [in] */ BYTE bListIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListCountryCode( 
            /* [in] */ BYTE bListIndex,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListCountOfRecords( 
            /* [in] */ BYTE bChannelListIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListRecordServiceId( 
            /* [in] */ BYTE bListIndex,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListRecordLogicalChannelNumber( 
            /* [in] */ BYTE bListIndex,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListRecordLogicalChannelAndVisibility( 
            /* [in] */ BYTE bListIndex,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbLogicalChannel2DescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbLogicalChannel2Descriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbLogicalChannel2Descriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordLogicalChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelNumber )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor2, GetRecordLogicalChannelAndVisibility)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelAndVisibility )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetCountOfLists)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfLists )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListId)
        HRESULT ( STDMETHODCALLTYPE *GetListId )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bListIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListNameW)
        HRESULT ( STDMETHODCALLTYPE *GetListNameW )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bListIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListCountryCode)
        HRESULT ( STDMETHODCALLTYPE *GetListCountryCode )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bListIndex,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetListCountOfRecords )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bChannelListIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetListRecordServiceId )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bListIndex,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListRecordLogicalChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetListRecordLogicalChannelNumber )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bListIndex,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannel2Descriptor, GetListRecordLogicalChannelAndVisibility)
        HRESULT ( STDMETHODCALLTYPE *GetListRecordLogicalChannelAndVisibility )( 
            IDvbLogicalChannel2Descriptor * This,
            /* [in] */ BYTE bListIndex,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IDvbLogicalChannel2DescriptorVtbl;

    interface IDvbLogicalChannel2Descriptor
    {
        CONST_VTBL struct IDvbLogicalChannel2DescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbLogicalChannel2Descriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbLogicalChannel2Descriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbLogicalChannel2Descriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbLogicalChannel2Descriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbLogicalChannel2Descriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbLogicalChannel2Descriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbLogicalChannel2Descriptor_GetRecordServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,bRecordIndex,pwVal) ) 

#define IDvbLogicalChannel2Descriptor_GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal) ) 


#define IDvbLogicalChannel2Descriptor_GetRecordLogicalChannelAndVisibility(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelAndVisibility(This,bRecordIndex,pwVal) ) 


#define IDvbLogicalChannel2Descriptor_GetCountOfLists(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfLists(This,pbVal) ) 

#define IDvbLogicalChannel2Descriptor_GetListId(This,bListIndex,pbVal)	\
    ( (This)->lpVtbl -> GetListId(This,bListIndex,pbVal) ) 

#define IDvbLogicalChannel2Descriptor_GetListNameW(This,bListIndex,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetListNameW(This,bListIndex,convMode,pbstrName) ) 

#define IDvbLogicalChannel2Descriptor_GetListCountryCode(This,bListIndex,pszCode)	\
    ( (This)->lpVtbl -> GetListCountryCode(This,bListIndex,pszCode) ) 

#define IDvbLogicalChannel2Descriptor_GetListCountOfRecords(This,bChannelListIndex,pbVal)	\
    ( (This)->lpVtbl -> GetListCountOfRecords(This,bChannelListIndex,pbVal) ) 

#define IDvbLogicalChannel2Descriptor_GetListRecordServiceId(This,bListIndex,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetListRecordServiceId(This,bListIndex,bRecordIndex,pwVal) ) 

#define IDvbLogicalChannel2Descriptor_GetListRecordLogicalChannelNumber(This,bListIndex,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetListRecordLogicalChannelNumber(This,bListIndex,bRecordIndex,pwVal) ) 

#define IDvbLogicalChannel2Descriptor_GetListRecordLogicalChannelAndVisibility(This,bListIndex,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetListRecordLogicalChannelAndVisibility(This,bListIndex,bRecordIndex,pwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbLogicalChannel2Descriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbHDSimulcastLogicalChannelDescriptor_INTERFACE_DEFINED__
#define __IDvbHDSimulcastLogicalChannelDescriptor_INTERFACE_DEFINED__

/* interface IDvbHDSimulcastLogicalChannelDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbHDSimulcastLogicalChannelDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1EA8B738-A307-4680-9E26-D0A908C824F4")
    IDvbHDSimulcastLogicalChannelDescriptor : public IDvbLogicalChannelDescriptor2
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbHDSimulcastLogicalChannelDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor, GetRecordLogicalChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelNumber )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLogicalChannelDescriptor2, GetRecordLogicalChannelAndVisibility)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLogicalChannelAndVisibility )( 
            IDvbHDSimulcastLogicalChannelDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IDvbHDSimulcastLogicalChannelDescriptorVtbl;

    interface IDvbHDSimulcastLogicalChannelDescriptor
    {
        CONST_VTBL struct IDvbHDSimulcastLogicalChannelDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbHDSimulcastLogicalChannelDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbHDSimulcastLogicalChannelDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbHDSimulcastLogicalChannelDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbHDSimulcastLogicalChannelDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbHDSimulcastLogicalChannelDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbHDSimulcastLogicalChannelDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbHDSimulcastLogicalChannelDescriptor_GetRecordServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,bRecordIndex,pwVal) ) 

#define IDvbHDSimulcastLogicalChannelDescriptor_GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelNumber(This,bRecordIndex,pwVal) ) 


#define IDvbHDSimulcastLogicalChannelDescriptor_GetRecordLogicalChannelAndVisibility(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordLogicalChannelAndVisibility(This,bRecordIndex,pwVal) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbHDSimulcastLogicalChannelDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbDataBroadcastIDDescriptor_INTERFACE_DEFINED__
#define __IDvbDataBroadcastIDDescriptor_INTERFACE_DEFINED__

/* interface IDvbDataBroadcastIDDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbDataBroadcastIDDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5F26F518-65C8-4048-91F2-9290F59F7B90")
    IDvbDataBroadcastIDDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataBroadcastID( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIDSelectorBytes( 
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbDataBroadcastIDDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbDataBroadcastIDDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbDataBroadcastIDDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbDataBroadcastIDDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastIDDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbDataBroadcastIDDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastIDDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbDataBroadcastIDDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastIDDescriptor, GetDataBroadcastID)
        HRESULT ( STDMETHODCALLTYPE *GetDataBroadcastID )( 
            IDvbDataBroadcastIDDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastIDDescriptor, GetIDSelectorBytes)
        HRESULT ( STDMETHODCALLTYPE *GetIDSelectorBytes )( 
            IDvbDataBroadcastIDDescriptor * This,
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbDataBroadcastIDDescriptorVtbl;

    interface IDvbDataBroadcastIDDescriptor
    {
        CONST_VTBL struct IDvbDataBroadcastIDDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbDataBroadcastIDDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbDataBroadcastIDDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbDataBroadcastIDDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbDataBroadcastIDDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbDataBroadcastIDDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbDataBroadcastIDDescriptor_GetDataBroadcastID(This,pwVal)	\
    ( (This)->lpVtbl -> GetDataBroadcastID(This,pwVal) ) 

#define IDvbDataBroadcastIDDescriptor_GetIDSelectorBytes(This,pbLen,pbVal)	\
    ( (This)->lpVtbl -> GetIDSelectorBytes(This,pbLen,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbDataBroadcastIDDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbDataBroadcastDescriptor_INTERFACE_DEFINED__
#define __IDvbDataBroadcastDescriptor_INTERFACE_DEFINED__

/* interface IDvbDataBroadcastDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbDataBroadcastDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D1EBC1D6-8B60-4c20-9CAF-E59382E7C400")
    IDvbDataBroadcastDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataBroadcastID( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectorLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectorBytes( 
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLangID( 
            /* [out] */ ULONG *pulVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbDataBroadcastDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbDataBroadcastDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbDataBroadcastDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbDataBroadcastDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetDataBroadcastID)
        HRESULT ( STDMETHODCALLTYPE *GetDataBroadcastID )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetComponentTag)
        HRESULT ( STDMETHODCALLTYPE *GetComponentTag )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetSelectorLength)
        HRESULT ( STDMETHODCALLTYPE *GetSelectorLength )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetSelectorBytes)
        HRESULT ( STDMETHODCALLTYPE *GetSelectorBytes )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetLangID)
        HRESULT ( STDMETHODCALLTYPE *GetLangID )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ ULONG *pulVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetTextLength)
        HRESULT ( STDMETHODCALLTYPE *GetTextLength )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbDataBroadcastDescriptor, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            IDvbDataBroadcastDescriptor * This,
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbDataBroadcastDescriptorVtbl;

    interface IDvbDataBroadcastDescriptor
    {
        CONST_VTBL struct IDvbDataBroadcastDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbDataBroadcastDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbDataBroadcastDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbDataBroadcastDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbDataBroadcastDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbDataBroadcastDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbDataBroadcastDescriptor_GetDataBroadcastID(This,pwVal)	\
    ( (This)->lpVtbl -> GetDataBroadcastID(This,pwVal) ) 

#define IDvbDataBroadcastDescriptor_GetComponentTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentTag(This,pbVal) ) 

#define IDvbDataBroadcastDescriptor_GetSelectorLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetSelectorLength(This,pbVal) ) 

#define IDvbDataBroadcastDescriptor_GetSelectorBytes(This,pbLen,pbVal)	\
    ( (This)->lpVtbl -> GetSelectorBytes(This,pbLen,pbVal) ) 

#define IDvbDataBroadcastDescriptor_GetLangID(This,pulVal)	\
    ( (This)->lpVtbl -> GetLangID(This,pulVal) ) 

#define IDvbDataBroadcastDescriptor_GetTextLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetTextLength(This,pbVal) ) 

#define IDvbDataBroadcastDescriptor_GetText(This,pbLen,pbVal)	\
    ( (This)->lpVtbl -> GetText(This,pbLen,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbDataBroadcastDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dvbsiparser_0000_0036 */
/* [local] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_dvbsiparser_0000_0036_0001
    {
        DESC_LINKAGE_RESERVED0	= 0,
        DESC_LINKAGE_INFORMATION	= ( DESC_LINKAGE_RESERVED0 + 1 ) ,
        DESC_LINKAGE_EPG	= ( DESC_LINKAGE_INFORMATION + 1 ) ,
        DESC_LINKAGE_CA_REPLACEMENT	= ( DESC_LINKAGE_EPG + 1 ) ,
        DESC_LINKAGE_COMPLETE_NET_BOUQUET_SI	= ( DESC_LINKAGE_CA_REPLACEMENT + 1 ) ,
        DESC_LINKAGE_REPLACEMENT	= ( DESC_LINKAGE_COMPLETE_NET_BOUQUET_SI + 1 ) ,
        DESC_LINKAGE_DATA	= ( DESC_LINKAGE_REPLACEMENT + 1 ) ,
        DESC_LINKAGE_RESERVED1	= 0x7,
        DESC_LINKAGE_USER	= 0x8,
        DESC_LINKAGE_RESERVED2	= 0xff
    } 	DESC_LINKAGE_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0036_v0_0_s_ifspec;

#ifndef __IDvbLinkageDescriptor_INTERFACE_DEFINED__
#define __IDvbLinkageDescriptor_INTERFACE_DEFINED__

/* interface IDvbLinkageDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbLinkageDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1CDF8B31-994A-46fc-ACFD-6A6BE8934DD5")
    IDvbLinkageDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTSId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetONId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLinkageType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateDataLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateData( 
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbLinkageDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbLinkageDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbLinkageDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbLinkageDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetTSId)
        HRESULT ( STDMETHODCALLTYPE *GetTSId )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetONId)
        HRESULT ( STDMETHODCALLTYPE *GetONId )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetServiceId )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetLinkageType)
        HRESULT ( STDMETHODCALLTYPE *GetLinkageType )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetPrivateDataLength)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateDataLength )( 
            IDvbLinkageDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbLinkageDescriptor, GetPrivateData)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateData )( 
            IDvbLinkageDescriptor * This,
            /* [out][in] */ BYTE *pbLen,
            /* [out] */ BYTE *pbData);
        
        END_INTERFACE
    } IDvbLinkageDescriptorVtbl;

    interface IDvbLinkageDescriptor
    {
        CONST_VTBL struct IDvbLinkageDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbLinkageDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbLinkageDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbLinkageDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbLinkageDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbLinkageDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbLinkageDescriptor_GetTSId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTSId(This,pwVal) ) 

#define IDvbLinkageDescriptor_GetONId(This,pwVal)	\
    ( (This)->lpVtbl -> GetONId(This,pwVal) ) 

#define IDvbLinkageDescriptor_GetServiceId(This,pwVal)	\
    ( (This)->lpVtbl -> GetServiceId(This,pwVal) ) 

#define IDvbLinkageDescriptor_GetLinkageType(This,pbVal)	\
    ( (This)->lpVtbl -> GetLinkageType(This,pbVal) ) 

#define IDvbLinkageDescriptor_GetPrivateDataLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetPrivateDataLength(This,pbVal) ) 

#define IDvbLinkageDescriptor_GetPrivateData(This,pbLen,pbData)	\
    ( (This)->lpVtbl -> GetPrivateData(This,pbLen,pbData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbLinkageDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbTeletextDescriptor_INTERFACE_DEFINED__
#define __IDvbTeletextDescriptor_INTERFACE_DEFINED__

/* interface IDvbTeletextDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbTeletextDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9CD29D47-69C6-4f92-98A9-210AF1B7303A")
    IDvbTeletextDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordLangId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ ULONG *pulVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTeletextType( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordMagazineNumber( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordPageNumber( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbTeletextDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbTeletextDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbTeletextDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbTeletextDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbTeletextDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbTeletextDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbTeletextDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetRecordLangId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLangId )( 
            IDvbTeletextDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ ULONG *pulVal);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetRecordTeletextType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTeletextType )( 
            IDvbTeletextDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetRecordMagazineNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordMagazineNumber )( 
            IDvbTeletextDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbTeletextDescriptor, GetRecordPageNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordPageNumber )( 
            IDvbTeletextDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbTeletextDescriptorVtbl;

    interface IDvbTeletextDescriptor
    {
        CONST_VTBL struct IDvbTeletextDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbTeletextDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbTeletextDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbTeletextDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbTeletextDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbTeletextDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbTeletextDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbTeletextDescriptor_GetRecordLangId(This,bRecordIndex,pulVal)	\
    ( (This)->lpVtbl -> GetRecordLangId(This,bRecordIndex,pulVal) ) 

#define IDvbTeletextDescriptor_GetRecordTeletextType(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordTeletextType(This,bRecordIndex,pbVal) ) 

#define IDvbTeletextDescriptor_GetRecordMagazineNumber(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordMagazineNumber(This,bRecordIndex,pbVal) ) 

#define IDvbTeletextDescriptor_GetRecordPageNumber(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordPageNumber(This,bRecordIndex,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbTeletextDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbSubtitlingDescriptor_INTERFACE_DEFINED__
#define __IDvbSubtitlingDescriptor_INTERFACE_DEFINED__

/* interface IDvbSubtitlingDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbSubtitlingDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B25FE1D-FA23-4e50-9784-6DF8B26F8A49")
    IDvbSubtitlingDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordLangId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ ULONG *pulVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordSubtitlingType( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCompositionPageID( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordAncillaryPageID( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbSubtitlingDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbSubtitlingDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbSubtitlingDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbSubtitlingDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbSubtitlingDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbSubtitlingDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbSubtitlingDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetRecordLangId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLangId )( 
            IDvbSubtitlingDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ ULONG *pulVal);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetRecordSubtitlingType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordSubtitlingType )( 
            IDvbSubtitlingDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetRecordCompositionPageID)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCompositionPageID )( 
            IDvbSubtitlingDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbSubtitlingDescriptor, GetRecordAncillaryPageID)
        HRESULT ( STDMETHODCALLTYPE *GetRecordAncillaryPageID )( 
            IDvbSubtitlingDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IDvbSubtitlingDescriptorVtbl;

    interface IDvbSubtitlingDescriptor
    {
        CONST_VTBL struct IDvbSubtitlingDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbSubtitlingDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbSubtitlingDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbSubtitlingDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbSubtitlingDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbSubtitlingDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbSubtitlingDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbSubtitlingDescriptor_GetRecordLangId(This,bRecordIndex,pulVal)	\
    ( (This)->lpVtbl -> GetRecordLangId(This,bRecordIndex,pulVal) ) 

#define IDvbSubtitlingDescriptor_GetRecordSubtitlingType(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordSubtitlingType(This,bRecordIndex,pbVal) ) 

#define IDvbSubtitlingDescriptor_GetRecordCompositionPageID(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordCompositionPageID(This,bRecordIndex,pwVal) ) 

#define IDvbSubtitlingDescriptor_GetRecordAncillaryPageID(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordAncillaryPageID(This,bRecordIndex,pwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbSubtitlingDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbServiceDescriptor_INTERFACE_DEFINED__
#define __IDvbServiceDescriptor_INTERFACE_DEFINED__

/* interface IDvbServiceDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbServiceDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F9C7FBCF-E2D6-464d-B32D-2EF526E49290")
    IDvbServiceDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceProviderName( 
            /* [annotation][out] */ 
            _Outptr_  char **pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceProviderNameW( 
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceName( 
            /* [annotation][out] */ 
            _Outptr_  char **pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProcessedServiceName( 
            /* [out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceNameEmphasized( 
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbServiceDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbServiceDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbServiceDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbServiceDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceType)
        HRESULT ( STDMETHODCALLTYPE *GetServiceType )( 
            IDvbServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceProviderName)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProviderName )( 
            IDvbServiceDescriptor * This,
            /* [annotation][out] */ 
            _Outptr_  char **pszName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceProviderNameW)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProviderNameW )( 
            IDvbServiceDescriptor * This,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceName)
        HRESULT ( STDMETHODCALLTYPE *GetServiceName )( 
            IDvbServiceDescriptor * This,
            /* [annotation][out] */ 
            _Outptr_  char **pszName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetProcessedServiceName)
        HRESULT ( STDMETHODCALLTYPE *GetProcessedServiceName )( 
            IDvbServiceDescriptor * This,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceNameEmphasized)
        HRESULT ( STDMETHODCALLTYPE *GetServiceNameEmphasized )( 
            IDvbServiceDescriptor * This,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IDvbServiceDescriptorVtbl;

    interface IDvbServiceDescriptor
    {
        CONST_VTBL struct IDvbServiceDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbServiceDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbServiceDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbServiceDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbServiceDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbServiceDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbServiceDescriptor_GetServiceType(This,pbVal)	\
    ( (This)->lpVtbl -> GetServiceType(This,pbVal) ) 

#define IDvbServiceDescriptor_GetServiceProviderName(This,pszName)	\
    ( (This)->lpVtbl -> GetServiceProviderName(This,pszName) ) 

#define IDvbServiceDescriptor_GetServiceProviderNameW(This,pbstrName)	\
    ( (This)->lpVtbl -> GetServiceProviderNameW(This,pbstrName) ) 

#define IDvbServiceDescriptor_GetServiceName(This,pszName)	\
    ( (This)->lpVtbl -> GetServiceName(This,pszName) ) 

#define IDvbServiceDescriptor_GetProcessedServiceName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetProcessedServiceName(This,pbstrName) ) 

#define IDvbServiceDescriptor_GetServiceNameEmphasized(This,pbstrName)	\
    ( (This)->lpVtbl -> GetServiceNameEmphasized(This,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbServiceDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbServiceDescriptor2_INTERFACE_DEFINED__
#define __IDvbServiceDescriptor2_INTERFACE_DEFINED__

/* interface IDvbServiceDescriptor2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbServiceDescriptor2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6C76506-85AB-487C-9B2B-36416511E4A2")
    IDvbServiceDescriptor2 : public IDvbServiceDescriptor
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetServiceProviderNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbServiceDescriptor2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbServiceDescriptor2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbServiceDescriptor2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbServiceDescriptor2 * This);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbServiceDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbServiceDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceType)
        HRESULT ( STDMETHODCALLTYPE *GetServiceType )( 
            IDvbServiceDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceProviderName)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProviderName )( 
            IDvbServiceDescriptor2 * This,
            /* [annotation][out] */ 
            _Outptr_  char **pszName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceProviderNameW)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProviderNameW )( 
            IDvbServiceDescriptor2 * This,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceName)
        HRESULT ( STDMETHODCALLTYPE *GetServiceName )( 
            IDvbServiceDescriptor2 * This,
            /* [annotation][out] */ 
            _Outptr_  char **pszName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetProcessedServiceName)
        HRESULT ( STDMETHODCALLTYPE *GetProcessedServiceName )( 
            IDvbServiceDescriptor2 * This,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor, GetServiceNameEmphasized)
        HRESULT ( STDMETHODCALLTYPE *GetServiceNameEmphasized )( 
            IDvbServiceDescriptor2 * This,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor2, GetServiceProviderNameW)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProviderNameW )( 
            IDvbServiceDescriptor2 * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbServiceDescriptor2, GetServiceNameW)
        HRESULT ( STDMETHODCALLTYPE *GetServiceNameW )( 
            IDvbServiceDescriptor2 * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IDvbServiceDescriptor2Vtbl;

    interface IDvbServiceDescriptor2
    {
        CONST_VTBL struct IDvbServiceDescriptor2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbServiceDescriptor2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbServiceDescriptor2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbServiceDescriptor2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbServiceDescriptor2_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbServiceDescriptor2_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbServiceDescriptor2_GetServiceType(This,pbVal)	\
    ( (This)->lpVtbl -> GetServiceType(This,pbVal) ) 

#define IDvbServiceDescriptor2_GetServiceProviderName(This,pszName)	\
    ( (This)->lpVtbl -> GetServiceProviderName(This,pszName) ) 

#define IDvbServiceDescriptor2_GetServiceProviderNameW(This,pbstrName)	\
    ( (This)->lpVtbl -> GetServiceProviderNameW(This,pbstrName) ) 

#define IDvbServiceDescriptor2_GetServiceName(This,pszName)	\
    ( (This)->lpVtbl -> GetServiceName(This,pszName) ) 

#define IDvbServiceDescriptor2_GetProcessedServiceName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetProcessedServiceName(This,pbstrName) ) 

#define IDvbServiceDescriptor2_GetServiceNameEmphasized(This,pbstrName)	\
    ( (This)->lpVtbl -> GetServiceNameEmphasized(This,pbstrName) ) 


#define IDvbServiceDescriptor2_GetServiceProviderNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetServiceProviderNameW(This,convMode,pbstrName) ) 

#define IDvbServiceDescriptor2_GetServiceNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetServiceNameW(This,convMode,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbServiceDescriptor2_INTERFACE_DEFINED__ */


#ifndef __IDvbServiceListDescriptor_INTERFACE_DEFINED__
#define __IDvbServiceListDescriptor_INTERFACE_DEFINED__

/* interface IDvbServiceListDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbServiceListDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05DB0D8F-6008-491a-ACD3-7090952707D0")
    IDvbServiceListDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceType( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbServiceListDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbServiceListDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbServiceListDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbServiceListDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbServiceListDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbServiceListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceListDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbServiceListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceListDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbServiceListDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceListDescriptor, GetRecordServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceId )( 
            IDvbServiceListDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IDvbServiceListDescriptor, GetRecordServiceType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceType )( 
            IDvbServiceListDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbServiceListDescriptorVtbl;

    interface IDvbServiceListDescriptor
    {
        CONST_VTBL struct IDvbServiceListDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbServiceListDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbServiceListDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbServiceListDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbServiceListDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbServiceListDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbServiceListDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbServiceListDescriptor_GetRecordServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceId(This,bRecordIndex,pwVal) ) 

#define IDvbServiceListDescriptor_GetRecordServiceType(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordServiceType(This,bRecordIndex,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbServiceListDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbMultilingualServiceNameDescriptor_INTERFACE_DEFINED__
#define __IDvbMultilingualServiceNameDescriptor_INTERFACE_DEFINED__

/* interface IDvbMultilingualServiceNameDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbMultilingualServiceNameDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D80433B-B32C-47ef-987F-E78EBB773E34")
    IDvbMultilingualServiceNameDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordLangId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ ULONG *ulVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceProviderNameW( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceNameW( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbMultilingualServiceNameDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbMultilingualServiceNameDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbMultilingualServiceNameDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbMultilingualServiceNameDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbMultilingualServiceNameDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbMultilingualServiceNameDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbMultilingualServiceNameDescriptor, GetRecordLangId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordLangId )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ ULONG *ulVal);
        
        DECLSPEC_XFGVIRT(IDvbMultilingualServiceNameDescriptor, GetRecordServiceProviderNameW)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceProviderNameW )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbMultilingualServiceNameDescriptor, GetRecordServiceNameW)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceNameW )( 
            IDvbMultilingualServiceNameDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IDvbMultilingualServiceNameDescriptorVtbl;

    interface IDvbMultilingualServiceNameDescriptor
    {
        CONST_VTBL struct IDvbMultilingualServiceNameDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbMultilingualServiceNameDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbMultilingualServiceNameDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbMultilingualServiceNameDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbMultilingualServiceNameDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbMultilingualServiceNameDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbMultilingualServiceNameDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbMultilingualServiceNameDescriptor_GetRecordLangId(This,bRecordIndex,ulVal)	\
    ( (This)->lpVtbl -> GetRecordLangId(This,bRecordIndex,ulVal) ) 

#define IDvbMultilingualServiceNameDescriptor_GetRecordServiceProviderNameW(This,bRecordIndex,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetRecordServiceProviderNameW(This,bRecordIndex,convMode,pbstrName) ) 

#define IDvbMultilingualServiceNameDescriptor_GetRecordServiceNameW(This,bRecordIndex,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetRecordServiceNameW(This,bRecordIndex,convMode,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbMultilingualServiceNameDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbNetworkNameDescriptor_INTERFACE_DEFINED__
#define __IDvbNetworkNameDescriptor_INTERFACE_DEFINED__

/* interface IDvbNetworkNameDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbNetworkNameDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5B2A80CF-35B9-446C-B3E4-048B761DBC51")
    IDvbNetworkNameDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetworkName( 
            /* [annotation][out] */ 
            _Outptr_  char **pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetworkNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbNetworkNameDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbNetworkNameDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbNetworkNameDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbNetworkNameDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbNetworkNameDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbNetworkNameDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbNetworkNameDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbNetworkNameDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbNetworkNameDescriptor, GetNetworkName)
        HRESULT ( STDMETHODCALLTYPE *GetNetworkName )( 
            IDvbNetworkNameDescriptor * This,
            /* [annotation][out] */ 
            _Outptr_  char **pszName);
        
        DECLSPEC_XFGVIRT(IDvbNetworkNameDescriptor, GetNetworkNameW)
        HRESULT ( STDMETHODCALLTYPE *GetNetworkNameW )( 
            IDvbNetworkNameDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IDvbNetworkNameDescriptorVtbl;

    interface IDvbNetworkNameDescriptor
    {
        CONST_VTBL struct IDvbNetworkNameDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbNetworkNameDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbNetworkNameDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbNetworkNameDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbNetworkNameDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbNetworkNameDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbNetworkNameDescriptor_GetNetworkName(This,pszName)	\
    ( (This)->lpVtbl -> GetNetworkName(This,pszName) ) 

#define IDvbNetworkNameDescriptor_GetNetworkNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetNetworkNameW(This,convMode,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbNetworkNameDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbShortEventDescriptor_INTERFACE_DEFINED__
#define __IDvbShortEventDescriptor_INTERFACE_DEFINED__

/* interface IDvbShortEventDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbShortEventDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B170BE92-5B75-458E-9C6E-B0008231491A")
    IDvbShortEventDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbShortEventDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbShortEventDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbShortEventDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbShortEventDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbShortEventDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbShortEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbShortEventDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbShortEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbShortEventDescriptor, GetLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode )( 
            IDvbShortEventDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IDvbShortEventDescriptor, GetEventNameW)
        HRESULT ( STDMETHODCALLTYPE *GetEventNameW )( 
            IDvbShortEventDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDvbShortEventDescriptor, GetTextW)
        HRESULT ( STDMETHODCALLTYPE *GetTextW )( 
            IDvbShortEventDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        END_INTERFACE
    } IDvbShortEventDescriptorVtbl;

    interface IDvbShortEventDescriptor
    {
        CONST_VTBL struct IDvbShortEventDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbShortEventDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbShortEventDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbShortEventDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbShortEventDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbShortEventDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbShortEventDescriptor_GetLanguageCode(This,pszCode)	\
    ( (This)->lpVtbl -> GetLanguageCode(This,pszCode) ) 

#define IDvbShortEventDescriptor_GetEventNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetEventNameW(This,convMode,pbstrName) ) 

#define IDvbShortEventDescriptor_GetTextW(This,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetTextW(This,convMode,pbstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbShortEventDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbExtendedEventDescriptor_INTERFACE_DEFINED__
#define __IDvbExtendedEventDescriptor_INTERFACE_DEFINED__

/* interface IDvbExtendedEventDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbExtendedEventDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9B22ECA-85F4-499F-B1DB-EFA93A91EE57")
    IDvbExtendedEventDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptorNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastDescriptorNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordItemW( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrDesc,
            /* [out] */ BSTR *pbstrItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConcatenatedItemW( 
            /* [in] */ IDvbExtendedEventDescriptor *pFollowingDescriptor,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrDesc,
            /* [out] */ BSTR *pbstrItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConcatenatedTextW( 
            /* [in] */ IDvbExtendedEventDescriptor *FollowingDescriptor,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordItemRawBytes( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE **ppbRawItem,
            /* [out] */ BYTE *pbItemLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbExtendedEventDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbExtendedEventDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbExtendedEventDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbExtendedEventDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbExtendedEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbExtendedEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetDescriptorNumber)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptorNumber )( 
            IDvbExtendedEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetLastDescriptorNumber)
        HRESULT ( STDMETHODCALLTYPE *GetLastDescriptorNumber )( 
            IDvbExtendedEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode )( 
            IDvbExtendedEventDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbExtendedEventDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetRecordItemW)
        HRESULT ( STDMETHODCALLTYPE *GetRecordItemW )( 
            IDvbExtendedEventDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrDesc,
            /* [out] */ BSTR *pbstrItem);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetConcatenatedItemW)
        HRESULT ( STDMETHODCALLTYPE *GetConcatenatedItemW )( 
            IDvbExtendedEventDescriptor * This,
            /* [in] */ IDvbExtendedEventDescriptor *pFollowingDescriptor,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrDesc,
            /* [out] */ BSTR *pbstrItem);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetTextW)
        HRESULT ( STDMETHODCALLTYPE *GetTextW )( 
            IDvbExtendedEventDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetConcatenatedTextW)
        HRESULT ( STDMETHODCALLTYPE *GetConcatenatedTextW )( 
            IDvbExtendedEventDescriptor * This,
            /* [in] */ IDvbExtendedEventDescriptor *FollowingDescriptor,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        DECLSPEC_XFGVIRT(IDvbExtendedEventDescriptor, GetRecordItemRawBytes)
        HRESULT ( STDMETHODCALLTYPE *GetRecordItemRawBytes )( 
            IDvbExtendedEventDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE **ppbRawItem,
            /* [out] */ BYTE *pbItemLength);
        
        END_INTERFACE
    } IDvbExtendedEventDescriptorVtbl;

    interface IDvbExtendedEventDescriptor
    {
        CONST_VTBL struct IDvbExtendedEventDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbExtendedEventDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbExtendedEventDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbExtendedEventDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbExtendedEventDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbExtendedEventDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbExtendedEventDescriptor_GetDescriptorNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetDescriptorNumber(This,pbVal) ) 

#define IDvbExtendedEventDescriptor_GetLastDescriptorNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetLastDescriptorNumber(This,pbVal) ) 

#define IDvbExtendedEventDescriptor_GetLanguageCode(This,pszCode)	\
    ( (This)->lpVtbl -> GetLanguageCode(This,pszCode) ) 

#define IDvbExtendedEventDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbExtendedEventDescriptor_GetRecordItemW(This,bRecordIndex,convMode,pbstrDesc,pbstrItem)	\
    ( (This)->lpVtbl -> GetRecordItemW(This,bRecordIndex,convMode,pbstrDesc,pbstrItem) ) 

#define IDvbExtendedEventDescriptor_GetConcatenatedItemW(This,pFollowingDescriptor,convMode,pbstrDesc,pbstrItem)	\
    ( (This)->lpVtbl -> GetConcatenatedItemW(This,pFollowingDescriptor,convMode,pbstrDesc,pbstrItem) ) 

#define IDvbExtendedEventDescriptor_GetTextW(This,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetTextW(This,convMode,pbstrText) ) 

#define IDvbExtendedEventDescriptor_GetConcatenatedTextW(This,FollowingDescriptor,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetConcatenatedTextW(This,FollowingDescriptor,convMode,pbstrText) ) 

#define IDvbExtendedEventDescriptor_GetRecordItemRawBytes(This,bRecordIndex,ppbRawItem,pbItemLength)	\
    ( (This)->lpVtbl -> GetRecordItemRawBytes(This,bRecordIndex,ppbRawItem,pbItemLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbExtendedEventDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbComponentDescriptor_INTERFACE_DEFINED__
#define __IDvbComponentDescriptor_INTERFACE_DEFINED__

/* interface IDvbComponentDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbComponentDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("91E405CF-80E7-457F-9096-1B9D1CE32141")
    IDvbComponentDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamContent( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbComponentDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbComponentDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbComponentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbComponentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetStreamContent)
        HRESULT ( STDMETHODCALLTYPE *GetStreamContent )( 
            IDvbComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetComponentType)
        HRESULT ( STDMETHODCALLTYPE *GetComponentType )( 
            IDvbComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetComponentTag)
        HRESULT ( STDMETHODCALLTYPE *GetComponentTag )( 
            IDvbComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode )( 
            IDvbComponentDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IDvbComponentDescriptor, GetTextW)
        HRESULT ( STDMETHODCALLTYPE *GetTextW )( 
            IDvbComponentDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        END_INTERFACE
    } IDvbComponentDescriptorVtbl;

    interface IDvbComponentDescriptor
    {
        CONST_VTBL struct IDvbComponentDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbComponentDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbComponentDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbComponentDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbComponentDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbComponentDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbComponentDescriptor_GetStreamContent(This,pbVal)	\
    ( (This)->lpVtbl -> GetStreamContent(This,pbVal) ) 

#define IDvbComponentDescriptor_GetComponentType(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentType(This,pbVal) ) 

#define IDvbComponentDescriptor_GetComponentTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentTag(This,pbVal) ) 

#define IDvbComponentDescriptor_GetLanguageCode(This,pszCode)	\
    ( (This)->lpVtbl -> GetLanguageCode(This,pszCode) ) 

#define IDvbComponentDescriptor_GetTextW(This,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetTextW(This,convMode,pbstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbComponentDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbContentDescriptor_INTERFACE_DEFINED__
#define __IDvbContentDescriptor_INTERFACE_DEFINED__

/* interface IDvbContentDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbContentDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2E883881-A467-412A-9D63-6F2B6DA05BF0")
    IDvbContentDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordContentNibbles( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbValLevel1,
            /* [out] */ BYTE *pbValLevel2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordUserNibbles( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal1,
            /* [out] */ BYTE *pbVal2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbContentDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbContentDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbContentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbContentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbContentDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbContentDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbContentDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbContentDescriptor, GetRecordContentNibbles)
        HRESULT ( STDMETHODCALLTYPE *GetRecordContentNibbles )( 
            IDvbContentDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbValLevel1,
            /* [out] */ BYTE *pbValLevel2);
        
        DECLSPEC_XFGVIRT(IDvbContentDescriptor, GetRecordUserNibbles)
        HRESULT ( STDMETHODCALLTYPE *GetRecordUserNibbles )( 
            IDvbContentDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal1,
            /* [out] */ BYTE *pbVal2);
        
        END_INTERFACE
    } IDvbContentDescriptorVtbl;

    interface IDvbContentDescriptor
    {
        CONST_VTBL struct IDvbContentDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbContentDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbContentDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbContentDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbContentDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbContentDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbContentDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbContentDescriptor_GetRecordContentNibbles(This,bRecordIndex,pbValLevel1,pbValLevel2)	\
    ( (This)->lpVtbl -> GetRecordContentNibbles(This,bRecordIndex,pbValLevel1,pbValLevel2) ) 

#define IDvbContentDescriptor_GetRecordUserNibbles(This,bRecordIndex,pbVal1,pbVal2)	\
    ( (This)->lpVtbl -> GetRecordUserNibbles(This,bRecordIndex,pbVal1,pbVal2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbContentDescriptor_INTERFACE_DEFINED__ */


#ifndef __IDvbParentalRatingDescriptor_INTERFACE_DEFINED__
#define __IDvbParentalRatingDescriptor_INTERFACE_DEFINED__

/* interface IDvbParentalRatingDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvbParentalRatingDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3AD9DDE1-FB1B-4186-937F-22E6B5A72A10")
    IDvbParentalRatingDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRating( 
            /* [in] */ BYTE bRecordIndex,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCountryCode,
            /* [out] */ BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvbParentalRatingDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvbParentalRatingDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvbParentalRatingDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvbParentalRatingDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDvbParentalRatingDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDvbParentalRatingDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbParentalRatingDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IDvbParentalRatingDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbParentalRatingDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IDvbParentalRatingDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IDvbParentalRatingDescriptor, GetRecordRating)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRating )( 
            IDvbParentalRatingDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCountryCode,
            /* [out] */ BYTE *pbVal);
        
        END_INTERFACE
    } IDvbParentalRatingDescriptorVtbl;

    interface IDvbParentalRatingDescriptor
    {
        CONST_VTBL struct IDvbParentalRatingDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvbParentalRatingDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvbParentalRatingDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvbParentalRatingDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvbParentalRatingDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IDvbParentalRatingDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IDvbParentalRatingDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IDvbParentalRatingDescriptor_GetRecordRating(This,bRecordIndex,pszCountryCode,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRating(This,bRecordIndex,pszCountryCode,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvbParentalRatingDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbTerrestrialDeliverySystemDescriptor_INTERFACE_DEFINED__
#define __IIsdbTerrestrialDeliverySystemDescriptor_INTERFACE_DEFINED__

/* interface IIsdbTerrestrialDeliverySystemDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbTerrestrialDeliverySystemDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39FAE0A6-D151-44DD-A28A-765DE5991670")
    IIsdbTerrestrialDeliverySystemDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAreaCode( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuardInterval( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransmissionMode( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordFrequency( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbTerrestrialDeliverySystemDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetAreaCode)
        HRESULT ( STDMETHODCALLTYPE *GetAreaCode )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetGuardInterval)
        HRESULT ( STDMETHODCALLTYPE *GetGuardInterval )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetTransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *GetTransmissionMode )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTerrestrialDeliverySystemDescriptor, GetRecordFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetRecordFrequency )( 
            IIsdbTerrestrialDeliverySystemDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        END_INTERFACE
    } IIsdbTerrestrialDeliverySystemDescriptorVtbl;

    interface IIsdbTerrestrialDeliverySystemDescriptor
    {
        CONST_VTBL struct IIsdbTerrestrialDeliverySystemDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbTerrestrialDeliverySystemDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbTerrestrialDeliverySystemDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_GetAreaCode(This,pwVal)	\
    ( (This)->lpVtbl -> GetAreaCode(This,pwVal) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_GetGuardInterval(This,pbVal)	\
    ( (This)->lpVtbl -> GetGuardInterval(This,pbVal) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_GetTransmissionMode(This,pbVal)	\
    ( (This)->lpVtbl -> GetTransmissionMode(This,pbVal) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbTerrestrialDeliverySystemDescriptor_GetRecordFrequency(This,bRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordFrequency(This,bRecordIndex,pdwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbTerrestrialDeliverySystemDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbTSInformationDescriptor_INTERFACE_DEFINED__
#define __IIsdbTSInformationDescriptor_INTERFACE_DEFINED__

/* interface IIsdbTSInformationDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbTSInformationDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7AD183E-38F5-4210-B55F-EC8D601BBD47")
    IIsdbTSInformationDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRemoteControlKeyId( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTSNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTransmissionTypeInfo( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordNumberOfServices( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceIdByIndex( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bServiceIndex,
            /* [out] */ WORD *pdwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbTSInformationDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbTSInformationDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbTSInformationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbTSInformationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbTSInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbTSInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetRemoteControlKeyId)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteControlKeyId )( 
            IIsdbTSInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetTSNameW)
        HRESULT ( STDMETHODCALLTYPE *GetTSNameW )( 
            IIsdbTSInformationDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbTSInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetRecordTransmissionTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTransmissionTypeInfo )( 
            IIsdbTSInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetRecordNumberOfServices)
        HRESULT ( STDMETHODCALLTYPE *GetRecordNumberOfServices )( 
            IIsdbTSInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbTSInformationDescriptor, GetRecordServiceIdByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceIdByIndex )( 
            IIsdbTSInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bServiceIndex,
            /* [out] */ WORD *pdwVal);
        
        END_INTERFACE
    } IIsdbTSInformationDescriptorVtbl;

    interface IIsdbTSInformationDescriptor
    {
        CONST_VTBL struct IIsdbTSInformationDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbTSInformationDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbTSInformationDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbTSInformationDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbTSInformationDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbTSInformationDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbTSInformationDescriptor_GetRemoteControlKeyId(This,pbVal)	\
    ( (This)->lpVtbl -> GetRemoteControlKeyId(This,pbVal) ) 

#define IIsdbTSInformationDescriptor_GetTSNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetTSNameW(This,convMode,pbstrName) ) 

#define IIsdbTSInformationDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbTSInformationDescriptor_GetRecordTransmissionTypeInfo(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordTransmissionTypeInfo(This,bRecordIndex,pbVal) ) 

#define IIsdbTSInformationDescriptor_GetRecordNumberOfServices(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordNumberOfServices(This,bRecordIndex,pbVal) ) 

#define IIsdbTSInformationDescriptor_GetRecordServiceIdByIndex(This,bRecordIndex,bServiceIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordServiceIdByIndex(This,bRecordIndex,bServiceIndex,pdwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbTSInformationDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbDigitalCopyControlDescriptor_INTERFACE_DEFINED__
#define __IIsdbDigitalCopyControlDescriptor_INTERFACE_DEFINED__

/* interface IIsdbDigitalCopyControlDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbDigitalCopyControlDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1A28417E-266A-4BB8-A4BD-D782BCFB8161")
    IIsdbDigitalCopyControlDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCopyControl( 
            /* [out] */ BYTE *pbDigitalRecordingControlData,
            /* [out] */ BYTE *pbCopyControlType,
            /* [out] */ BYTE *pbAPSControlData,
            /* [out] */ BYTE *pbMaximumBitrate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCopyControl( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbComponentTag,
            /* [out] */ BYTE *pbDigitalRecordingControlData,
            /* [out] */ BYTE *pbCopyControlType,
            /* [out] */ BYTE *pbAPSControlData,
            /* [out] */ BYTE *pbMaximumBitrate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbDigitalCopyControlDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbDigitalCopyControlDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbDigitalCopyControlDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbDigitalCopyControlDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbDigitalCopyControlDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbDigitalCopyControlDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDigitalCopyControlDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbDigitalCopyControlDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDigitalCopyControlDescriptor, GetCopyControl)
        HRESULT ( STDMETHODCALLTYPE *GetCopyControl )( 
            IIsdbDigitalCopyControlDescriptor * This,
            /* [out] */ BYTE *pbDigitalRecordingControlData,
            /* [out] */ BYTE *pbCopyControlType,
            /* [out] */ BYTE *pbAPSControlData,
            /* [out] */ BYTE *pbMaximumBitrate);
        
        DECLSPEC_XFGVIRT(IIsdbDigitalCopyControlDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbDigitalCopyControlDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDigitalCopyControlDescriptor, GetRecordCopyControl)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCopyControl )( 
            IIsdbDigitalCopyControlDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbComponentTag,
            /* [out] */ BYTE *pbDigitalRecordingControlData,
            /* [out] */ BYTE *pbCopyControlType,
            /* [out] */ BYTE *pbAPSControlData,
            /* [out] */ BYTE *pbMaximumBitrate);
        
        END_INTERFACE
    } IIsdbDigitalCopyControlDescriptorVtbl;

    interface IIsdbDigitalCopyControlDescriptor
    {
        CONST_VTBL struct IIsdbDigitalCopyControlDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbDigitalCopyControlDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbDigitalCopyControlDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbDigitalCopyControlDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbDigitalCopyControlDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbDigitalCopyControlDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbDigitalCopyControlDescriptor_GetCopyControl(This,pbDigitalRecordingControlData,pbCopyControlType,pbAPSControlData,pbMaximumBitrate)	\
    ( (This)->lpVtbl -> GetCopyControl(This,pbDigitalRecordingControlData,pbCopyControlType,pbAPSControlData,pbMaximumBitrate) ) 

#define IIsdbDigitalCopyControlDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbDigitalCopyControlDescriptor_GetRecordCopyControl(This,bRecordIndex,pbComponentTag,pbDigitalRecordingControlData,pbCopyControlType,pbAPSControlData,pbMaximumBitrate)	\
    ( (This)->lpVtbl -> GetRecordCopyControl(This,bRecordIndex,pbComponentTag,pbDigitalRecordingControlData,pbCopyControlType,pbAPSControlData,pbMaximumBitrate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbDigitalCopyControlDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbAudioComponentDescriptor_INTERFACE_DEFINED__
#define __IIsdbAudioComponentDescriptor_INTERFACE_DEFINED__

/* interface IIsdbAudioComponentDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbAudioComponentDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("679D2002-2425-4BE4-A4C7-D6632A574F4D")
    IIsdbAudioComponentDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamContent( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSimulcastGroupTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetESMultiLingualFlag( 
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMainComponentFlag( 
            /* [out] */ BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQualityIndicator( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSamplingRate( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode2( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbAudioComponentDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbAudioComponentDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbAudioComponentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbAudioComponentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetStreamContent)
        HRESULT ( STDMETHODCALLTYPE *GetStreamContent )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetComponentType)
        HRESULT ( STDMETHODCALLTYPE *GetComponentType )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetComponentTag)
        HRESULT ( STDMETHODCALLTYPE *GetComponentTag )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetStreamType)
        HRESULT ( STDMETHODCALLTYPE *GetStreamType )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetSimulcastGroupTag)
        HRESULT ( STDMETHODCALLTYPE *GetSimulcastGroupTag )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetESMultiLingualFlag)
        HRESULT ( STDMETHODCALLTYPE *GetESMultiLingualFlag )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetMainComponentFlag)
        HRESULT ( STDMETHODCALLTYPE *GetMainComponentFlag )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetQualityIndicator)
        HRESULT ( STDMETHODCALLTYPE *GetQualityIndicator )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetSamplingRate)
        HRESULT ( STDMETHODCALLTYPE *GetSamplingRate )( 
            IIsdbAudioComponentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode )( 
            IIsdbAudioComponentDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetLanguageCode2)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode2 )( 
            IIsdbAudioComponentDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IIsdbAudioComponentDescriptor, GetTextW)
        HRESULT ( STDMETHODCALLTYPE *GetTextW )( 
            IIsdbAudioComponentDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        END_INTERFACE
    } IIsdbAudioComponentDescriptorVtbl;

    interface IIsdbAudioComponentDescriptor
    {
        CONST_VTBL struct IIsdbAudioComponentDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbAudioComponentDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbAudioComponentDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbAudioComponentDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbAudioComponentDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetStreamContent(This,pbVal)	\
    ( (This)->lpVtbl -> GetStreamContent(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetComponentType(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentType(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetComponentTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentTag(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetStreamType(This,pbVal)	\
    ( (This)->lpVtbl -> GetStreamType(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetSimulcastGroupTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetSimulcastGroupTag(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetESMultiLingualFlag(This,pfVal)	\
    ( (This)->lpVtbl -> GetESMultiLingualFlag(This,pfVal) ) 

#define IIsdbAudioComponentDescriptor_GetMainComponentFlag(This,pfVal)	\
    ( (This)->lpVtbl -> GetMainComponentFlag(This,pfVal) ) 

#define IIsdbAudioComponentDescriptor_GetQualityIndicator(This,pbVal)	\
    ( (This)->lpVtbl -> GetQualityIndicator(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetSamplingRate(This,pbVal)	\
    ( (This)->lpVtbl -> GetSamplingRate(This,pbVal) ) 

#define IIsdbAudioComponentDescriptor_GetLanguageCode(This,pszCode)	\
    ( (This)->lpVtbl -> GetLanguageCode(This,pszCode) ) 

#define IIsdbAudioComponentDescriptor_GetLanguageCode2(This,pszCode)	\
    ( (This)->lpVtbl -> GetLanguageCode2(This,pszCode) ) 

#define IIsdbAudioComponentDescriptor_GetTextW(This,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetTextW(This,convMode,pbstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbAudioComponentDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbDataContentDescriptor_INTERFACE_DEFINED__
#define __IIsdbDataContentDescriptor_INTERFACE_DEFINED__

/* interface IIsdbDataContentDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbDataContentDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A428100A-E646-4BD6-AA14-6087BDC08CD5")
    IIsdbDataContentDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataComponentId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEntryComponent( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectorLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectorBytes( 
            /* [in] */ BYTE bBufLength,
            /* [out] */ BYTE *pbBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordComponentRef( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbDataContentDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbDataContentDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbDataContentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbDataContentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbDataContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbDataContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetDataComponentId)
        HRESULT ( STDMETHODCALLTYPE *GetDataComponentId )( 
            IIsdbDataContentDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetEntryComponent)
        HRESULT ( STDMETHODCALLTYPE *GetEntryComponent )( 
            IIsdbDataContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetSelectorLength)
        HRESULT ( STDMETHODCALLTYPE *GetSelectorLength )( 
            IIsdbDataContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetSelectorBytes)
        HRESULT ( STDMETHODCALLTYPE *GetSelectorBytes )( 
            IIsdbDataContentDescriptor * This,
            /* [in] */ BYTE bBufLength,
            /* [out] */ BYTE *pbBuf);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbDataContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetRecordComponentRef)
        HRESULT ( STDMETHODCALLTYPE *GetRecordComponentRef )( 
            IIsdbDataContentDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode )( 
            IIsdbDataContentDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *pszCode);
        
        DECLSPEC_XFGVIRT(IIsdbDataContentDescriptor, GetTextW)
        HRESULT ( STDMETHODCALLTYPE *GetTextW )( 
            IIsdbDataContentDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        END_INTERFACE
    } IIsdbDataContentDescriptorVtbl;

    interface IIsdbDataContentDescriptor
    {
        CONST_VTBL struct IIsdbDataContentDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbDataContentDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbDataContentDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbDataContentDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbDataContentDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbDataContentDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbDataContentDescriptor_GetDataComponentId(This,pwVal)	\
    ( (This)->lpVtbl -> GetDataComponentId(This,pwVal) ) 

#define IIsdbDataContentDescriptor_GetEntryComponent(This,pbVal)	\
    ( (This)->lpVtbl -> GetEntryComponent(This,pbVal) ) 

#define IIsdbDataContentDescriptor_GetSelectorLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetSelectorLength(This,pbVal) ) 

#define IIsdbDataContentDescriptor_GetSelectorBytes(This,bBufLength,pbBuf)	\
    ( (This)->lpVtbl -> GetSelectorBytes(This,bBufLength,pbBuf) ) 

#define IIsdbDataContentDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbDataContentDescriptor_GetRecordComponentRef(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordComponentRef(This,bRecordIndex,pbVal) ) 

#define IIsdbDataContentDescriptor_GetLanguageCode(This,pszCode)	\
    ( (This)->lpVtbl -> GetLanguageCode(This,pszCode) ) 

#define IIsdbDataContentDescriptor_GetTextW(This,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetTextW(This,convMode,pbstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbDataContentDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbCAContractInformationDescriptor_INTERFACE_DEFINED__
#define __IIsdbCAContractInformationDescriptor_INTERFACE_DEFINED__

/* interface IIsdbCAContractInformationDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbCAContractInformationDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08E18B25-A28F-4E92-821E-4FCED5CC2291")
    IIsdbCAContractInformationDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCASystemId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAUnitId( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordComponentTag( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContractVerificationInfoLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContractVerificationInfo( 
            /* [in] */ BYTE bBufLength,
            /* [out] */ BYTE *pbBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFeeNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbCAContractInformationDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbCAContractInformationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbCAContractInformationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetCASystemId)
        HRESULT ( STDMETHODCALLTYPE *GetCASystemId )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetCAUnitId)
        HRESULT ( STDMETHODCALLTYPE *GetCAUnitId )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetRecordComponentTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordComponentTag )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetContractVerificationInfoLength)
        HRESULT ( STDMETHODCALLTYPE *GetContractVerificationInfoLength )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetContractVerificationInfo)
        HRESULT ( STDMETHODCALLTYPE *GetContractVerificationInfo )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [in] */ BYTE bBufLength,
            /* [out] */ BYTE *pbBuf);
        
        DECLSPEC_XFGVIRT(IIsdbCAContractInformationDescriptor, GetFeeNameW)
        HRESULT ( STDMETHODCALLTYPE *GetFeeNameW )( 
            IIsdbCAContractInformationDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IIsdbCAContractInformationDescriptorVtbl;

    interface IIsdbCAContractInformationDescriptor
    {
        CONST_VTBL struct IIsdbCAContractInformationDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbCAContractInformationDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbCAContractInformationDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbCAContractInformationDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbCAContractInformationDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbCAContractInformationDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbCAContractInformationDescriptor_GetCASystemId(This,pwVal)	\
    ( (This)->lpVtbl -> GetCASystemId(This,pwVal) ) 

#define IIsdbCAContractInformationDescriptor_GetCAUnitId(This,pbVal)	\
    ( (This)->lpVtbl -> GetCAUnitId(This,pbVal) ) 

#define IIsdbCAContractInformationDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbCAContractInformationDescriptor_GetRecordComponentTag(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordComponentTag(This,bRecordIndex,pbVal) ) 

#define IIsdbCAContractInformationDescriptor_GetContractVerificationInfoLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetContractVerificationInfoLength(This,pbVal) ) 

#define IIsdbCAContractInformationDescriptor_GetContractVerificationInfo(This,bBufLength,pbBuf)	\
    ( (This)->lpVtbl -> GetContractVerificationInfo(This,bBufLength,pbBuf) ) 

#define IIsdbCAContractInformationDescriptor_GetFeeNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetFeeNameW(This,convMode,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbCAContractInformationDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbEventGroupDescriptor_INTERFACE_DEFINED__
#define __IIsdbEventGroupDescriptor_INTERFACE_DEFINED__

/* interface IIsdbEventGroupDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbEventGroupDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94B06780-2E2A-44DC-A966-CC56FDABC6C2")
    IIsdbEventGroupDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGroupType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEvent( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwServiceId,
            /* [out] */ WORD *pwEventId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRefRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRefRecordEvent( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwOriginalNetworkId,
            /* [out] */ WORD *pwTransportStreamId,
            /* [out] */ WORD *pwServiceId,
            /* [out] */ WORD *pwEventId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbEventGroupDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbEventGroupDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbEventGroupDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbEventGroupDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbEventGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbEventGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetGroupType)
        HRESULT ( STDMETHODCALLTYPE *GetGroupType )( 
            IIsdbEventGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbEventGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetRecordEvent)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEvent )( 
            IIsdbEventGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwServiceId,
            /* [out] */ WORD *pwEventId);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetCountOfRefRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRefRecords )( 
            IIsdbEventGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEventGroupDescriptor, GetRefRecordEvent)
        HRESULT ( STDMETHODCALLTYPE *GetRefRecordEvent )( 
            IIsdbEventGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwOriginalNetworkId,
            /* [out] */ WORD *pwTransportStreamId,
            /* [out] */ WORD *pwServiceId,
            /* [out] */ WORD *pwEventId);
        
        END_INTERFACE
    } IIsdbEventGroupDescriptorVtbl;

    interface IIsdbEventGroupDescriptor
    {
        CONST_VTBL struct IIsdbEventGroupDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbEventGroupDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbEventGroupDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbEventGroupDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbEventGroupDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbEventGroupDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbEventGroupDescriptor_GetGroupType(This,pbVal)	\
    ( (This)->lpVtbl -> GetGroupType(This,pbVal) ) 

#define IIsdbEventGroupDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbEventGroupDescriptor_GetRecordEvent(This,bRecordIndex,pwServiceId,pwEventId)	\
    ( (This)->lpVtbl -> GetRecordEvent(This,bRecordIndex,pwServiceId,pwEventId) ) 

#define IIsdbEventGroupDescriptor_GetCountOfRefRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRefRecords(This,pbVal) ) 

#define IIsdbEventGroupDescriptor_GetRefRecordEvent(This,bRecordIndex,pwOriginalNetworkId,pwTransportStreamId,pwServiceId,pwEventId)	\
    ( (This)->lpVtbl -> GetRefRecordEvent(This,bRecordIndex,pwOriginalNetworkId,pwTransportStreamId,pwServiceId,pwEventId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbEventGroupDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbComponentGroupDescriptor_INTERFACE_DEFINED__
#define __IIsdbComponentGroupDescriptor_INTERFACE_DEFINED__

/* interface IIsdbComponentGroupDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbComponentGroupDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A494F17F-C592-47D8-8943-64C9A34BE7B9")
    IIsdbComponentGroupDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentGroupType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordGroupId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordNumberOfCAUnit( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCAUnitCAUnitId( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bCAUnitIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCAUnitNumberOfComponents( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bCAUnitIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCAUnitComponentTag( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bCAUnitIndex,
            /* [in] */ BYTE bComponentIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTotalBitRate( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTextW( 
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbComponentGroupDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbComponentGroupDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbComponentGroupDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbComponentGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbComponentGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetComponentGroupType)
        HRESULT ( STDMETHODCALLTYPE *GetComponentGroupType )( 
            IIsdbComponentGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbComponentGroupDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordGroupId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordGroupId )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordNumberOfCAUnit)
        HRESULT ( STDMETHODCALLTYPE *GetRecordNumberOfCAUnit )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordCAUnitCAUnitId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCAUnitCAUnitId )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bCAUnitIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordCAUnitNumberOfComponents)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCAUnitNumberOfComponents )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bCAUnitIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordCAUnitComponentTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCAUnitComponentTag )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ BYTE bCAUnitIndex,
            /* [in] */ BYTE bComponentIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordTotalBitRate)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTotalBitRate )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbComponentGroupDescriptor, GetRecordTextW)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTextW )( 
            IIsdbComponentGroupDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrText);
        
        END_INTERFACE
    } IIsdbComponentGroupDescriptorVtbl;

    interface IIsdbComponentGroupDescriptor
    {
        CONST_VTBL struct IIsdbComponentGroupDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbComponentGroupDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbComponentGroupDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbComponentGroupDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbComponentGroupDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetComponentGroupType(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentGroupType(This,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordGroupId(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordGroupId(This,bRecordIndex,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordNumberOfCAUnit(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordNumberOfCAUnit(This,bRecordIndex,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordCAUnitCAUnitId(This,bRecordIndex,bCAUnitIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordCAUnitCAUnitId(This,bRecordIndex,bCAUnitIndex,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordCAUnitNumberOfComponents(This,bRecordIndex,bCAUnitIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordCAUnitNumberOfComponents(This,bRecordIndex,bCAUnitIndex,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordCAUnitComponentTag(This,bRecordIndex,bCAUnitIndex,bComponentIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordCAUnitComponentTag(This,bRecordIndex,bCAUnitIndex,bComponentIndex,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordTotalBitRate(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordTotalBitRate(This,bRecordIndex,pbVal) ) 

#define IIsdbComponentGroupDescriptor_GetRecordTextW(This,bRecordIndex,convMode,pbstrText)	\
    ( (This)->lpVtbl -> GetRecordTextW(This,bRecordIndex,convMode,pbstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbComponentGroupDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbSeriesDescriptor_INTERFACE_DEFINED__
#define __IIsdbSeriesDescriptor_INTERFACE_DEFINED__

/* interface IIsdbSeriesDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbSeriesDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("07EF6370-1660-4F26-87FC-614ADAB24B11")
    IIsdbSeriesDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSeriesId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRepeatLabel( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProgramPattern( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExpireDate( 
            /* [out] */ BOOL *pfValid,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEpisodeNumber( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastEpisodeNumber( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSeriesNameW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbSeriesDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbSeriesDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbSeriesDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbSeriesDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetSeriesId)
        HRESULT ( STDMETHODCALLTYPE *GetSeriesId )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetRepeatLabel)
        HRESULT ( STDMETHODCALLTYPE *GetRepeatLabel )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetProgramPattern)
        HRESULT ( STDMETHODCALLTYPE *GetProgramPattern )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetExpireDate)
        HRESULT ( STDMETHODCALLTYPE *GetExpireDate )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ BOOL *pfValid,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetEpisodeNumber)
        HRESULT ( STDMETHODCALLTYPE *GetEpisodeNumber )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetLastEpisodeNumber)
        HRESULT ( STDMETHODCALLTYPE *GetLastEpisodeNumber )( 
            IIsdbSeriesDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbSeriesDescriptor, GetSeriesNameW)
        HRESULT ( STDMETHODCALLTYPE *GetSeriesNameW )( 
            IIsdbSeriesDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IIsdbSeriesDescriptorVtbl;

    interface IIsdbSeriesDescriptor
    {
        CONST_VTBL struct IIsdbSeriesDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbSeriesDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbSeriesDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbSeriesDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbSeriesDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbSeriesDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbSeriesDescriptor_GetSeriesId(This,pwVal)	\
    ( (This)->lpVtbl -> GetSeriesId(This,pwVal) ) 

#define IIsdbSeriesDescriptor_GetRepeatLabel(This,pbVal)	\
    ( (This)->lpVtbl -> GetRepeatLabel(This,pbVal) ) 

#define IIsdbSeriesDescriptor_GetProgramPattern(This,pbVal)	\
    ( (This)->lpVtbl -> GetProgramPattern(This,pbVal) ) 

#define IIsdbSeriesDescriptor_GetExpireDate(This,pfValid,pmdtVal)	\
    ( (This)->lpVtbl -> GetExpireDate(This,pfValid,pmdtVal) ) 

#define IIsdbSeriesDescriptor_GetEpisodeNumber(This,pwVal)	\
    ( (This)->lpVtbl -> GetEpisodeNumber(This,pwVal) ) 

#define IIsdbSeriesDescriptor_GetLastEpisodeNumber(This,pwVal)	\
    ( (This)->lpVtbl -> GetLastEpisodeNumber(This,pwVal) ) 

#define IIsdbSeriesDescriptor_GetSeriesNameW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetSeriesNameW(This,convMode,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbSeriesDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbDownloadContentDescriptor_INTERFACE_DEFINED__
#define __IIsdbDownloadContentDescriptor_INTERFACE_DEFINED__

/* interface IIsdbDownloadContentDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbDownloadContentDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5298661E-CB88-4F5F-A1DE-5F440C185B92")
    IIsdbDownloadContentDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ BOOL *pfReboot,
            /* [out] */ BOOL *pfAddOn,
            /* [out] */ BOOL *pfCompatibility,
            /* [out] */ BOOL *pfModuleInfo,
            /* [out] */ BOOL *pfTextInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentSize( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDownloadId( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeOutValueDII( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLeakRate( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompatiblityDescriptorLength( 
            /* [out] */ WORD *pwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompatiblityDescriptor( 
            /* [out] */ BYTE **ppbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordModuleId( 
            /* [in] */ WORD wRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordModuleSize( 
            /* [in] */ WORD wRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordModuleInfoLength( 
            /* [in] */ WORD wRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordModuleInfo( 
            /* [in] */ WORD wRecordIndex,
            /* [out] */ BYTE **ppbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextLanguageCode( 
            /* [annotation][out] */ 
            _Out_writes_(4)  char *szCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbDownloadContentDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbDownloadContentDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbDownloadContentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbDownloadContentDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ BOOL *pfReboot,
            /* [out] */ BOOL *pfAddOn,
            /* [out] */ BOOL *pfCompatibility,
            /* [out] */ BOOL *pfModuleInfo,
            /* [out] */ BOOL *pfTextInfo);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetComponentSize)
        HRESULT ( STDMETHODCALLTYPE *GetComponentSize )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetDownloadId)
        HRESULT ( STDMETHODCALLTYPE *GetDownloadId )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetTimeOutValueDII)
        HRESULT ( STDMETHODCALLTYPE *GetTimeOutValueDII )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetLeakRate)
        HRESULT ( STDMETHODCALLTYPE *GetLeakRate )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetComponentTag)
        HRESULT ( STDMETHODCALLTYPE *GetComponentTag )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetCompatiblityDescriptorLength)
        HRESULT ( STDMETHODCALLTYPE *GetCompatiblityDescriptorLength )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ WORD *pwLength);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetCompatiblityDescriptor)
        HRESULT ( STDMETHODCALLTYPE *GetCompatiblityDescriptor )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ BYTE **ppbData);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbDownloadContentDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetRecordModuleId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordModuleId )( 
            IIsdbDownloadContentDescriptor * This,
            /* [in] */ WORD wRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetRecordModuleSize)
        HRESULT ( STDMETHODCALLTYPE *GetRecordModuleSize )( 
            IIsdbDownloadContentDescriptor * This,
            /* [in] */ WORD wRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetRecordModuleInfoLength)
        HRESULT ( STDMETHODCALLTYPE *GetRecordModuleInfoLength )( 
            IIsdbDownloadContentDescriptor * This,
            /* [in] */ WORD wRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetRecordModuleInfo)
        HRESULT ( STDMETHODCALLTYPE *GetRecordModuleInfo )( 
            IIsdbDownloadContentDescriptor * This,
            /* [in] */ WORD wRecordIndex,
            /* [out] */ BYTE **ppbData);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetTextLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetTextLanguageCode )( 
            IIsdbDownloadContentDescriptor * This,
            /* [annotation][out] */ 
            _Out_writes_(4)  char *szCode);
        
        DECLSPEC_XFGVIRT(IIsdbDownloadContentDescriptor, GetTextW)
        HRESULT ( STDMETHODCALLTYPE *GetTextW )( 
            IIsdbDownloadContentDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrName);
        
        END_INTERFACE
    } IIsdbDownloadContentDescriptorVtbl;

    interface IIsdbDownloadContentDescriptor
    {
        CONST_VTBL struct IIsdbDownloadContentDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbDownloadContentDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbDownloadContentDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbDownloadContentDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbDownloadContentDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbDownloadContentDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbDownloadContentDescriptor_GetFlags(This,pfReboot,pfAddOn,pfCompatibility,pfModuleInfo,pfTextInfo)	\
    ( (This)->lpVtbl -> GetFlags(This,pfReboot,pfAddOn,pfCompatibility,pfModuleInfo,pfTextInfo) ) 

#define IIsdbDownloadContentDescriptor_GetComponentSize(This,pdwVal)	\
    ( (This)->lpVtbl -> GetComponentSize(This,pdwVal) ) 

#define IIsdbDownloadContentDescriptor_GetDownloadId(This,pdwVal)	\
    ( (This)->lpVtbl -> GetDownloadId(This,pdwVal) ) 

#define IIsdbDownloadContentDescriptor_GetTimeOutValueDII(This,pdwVal)	\
    ( (This)->lpVtbl -> GetTimeOutValueDII(This,pdwVal) ) 

#define IIsdbDownloadContentDescriptor_GetLeakRate(This,pdwVal)	\
    ( (This)->lpVtbl -> GetLeakRate(This,pdwVal) ) 

#define IIsdbDownloadContentDescriptor_GetComponentTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetComponentTag(This,pbVal) ) 

#define IIsdbDownloadContentDescriptor_GetCompatiblityDescriptorLength(This,pwLength)	\
    ( (This)->lpVtbl -> GetCompatiblityDescriptorLength(This,pwLength) ) 

#define IIsdbDownloadContentDescriptor_GetCompatiblityDescriptor(This,ppbData)	\
    ( (This)->lpVtbl -> GetCompatiblityDescriptor(This,ppbData) ) 

#define IIsdbDownloadContentDescriptor_GetCountOfRecords(This,pwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pwVal) ) 

#define IIsdbDownloadContentDescriptor_GetRecordModuleId(This,wRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordModuleId(This,wRecordIndex,pwVal) ) 

#define IIsdbDownloadContentDescriptor_GetRecordModuleSize(This,wRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordModuleSize(This,wRecordIndex,pdwVal) ) 

#define IIsdbDownloadContentDescriptor_GetRecordModuleInfoLength(This,wRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordModuleInfoLength(This,wRecordIndex,pbVal) ) 

#define IIsdbDownloadContentDescriptor_GetRecordModuleInfo(This,wRecordIndex,ppbData)	\
    ( (This)->lpVtbl -> GetRecordModuleInfo(This,wRecordIndex,ppbData) ) 

#define IIsdbDownloadContentDescriptor_GetTextLanguageCode(This,szCode)	\
    ( (This)->lpVtbl -> GetTextLanguageCode(This,szCode) ) 

#define IIsdbDownloadContentDescriptor_GetTextW(This,convMode,pbstrName)	\
    ( (This)->lpVtbl -> GetTextW(This,convMode,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbDownloadContentDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbLogoTransmissionDescriptor_INTERFACE_DEFINED__
#define __IIsdbLogoTransmissionDescriptor_INTERFACE_DEFINED__

/* interface IIsdbLogoTransmissionDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbLogoTransmissionDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E0103F49-4AE1-4F07-9098-756DB1FA88CD")
    IIsdbLogoTransmissionDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLogoTransmissionType( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLogoId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLogoVersion( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDownloadDataId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLogoCharW( 
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrChar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbLogoTransmissionDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbLogoTransmissionDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbLogoTransmissionDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetLogoTransmissionType)
        HRESULT ( STDMETHODCALLTYPE *GetLogoTransmissionType )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetLogoId)
        HRESULT ( STDMETHODCALLTYPE *GetLogoId )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetLogoVersion)
        HRESULT ( STDMETHODCALLTYPE *GetLogoVersion )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetDownloadDataId)
        HRESULT ( STDMETHODCALLTYPE *GetDownloadDataId )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbLogoTransmissionDescriptor, GetLogoCharW)
        HRESULT ( STDMETHODCALLTYPE *GetLogoCharW )( 
            IIsdbLogoTransmissionDescriptor * This,
            /* [in] */ DVB_STRCONV_MODE convMode,
            /* [out] */ BSTR *pbstrChar);
        
        END_INTERFACE
    } IIsdbLogoTransmissionDescriptorVtbl;

    interface IIsdbLogoTransmissionDescriptor
    {
        CONST_VTBL struct IIsdbLogoTransmissionDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbLogoTransmissionDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbLogoTransmissionDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbLogoTransmissionDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbLogoTransmissionDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbLogoTransmissionDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbLogoTransmissionDescriptor_GetLogoTransmissionType(This,pbVal)	\
    ( (This)->lpVtbl -> GetLogoTransmissionType(This,pbVal) ) 

#define IIsdbLogoTransmissionDescriptor_GetLogoId(This,pwVal)	\
    ( (This)->lpVtbl -> GetLogoId(This,pwVal) ) 

#define IIsdbLogoTransmissionDescriptor_GetLogoVersion(This,pwVal)	\
    ( (This)->lpVtbl -> GetLogoVersion(This,pwVal) ) 

#define IIsdbLogoTransmissionDescriptor_GetDownloadDataId(This,pwVal)	\
    ( (This)->lpVtbl -> GetDownloadDataId(This,pwVal) ) 

#define IIsdbLogoTransmissionDescriptor_GetLogoCharW(This,convMode,pbstrChar)	\
    ( (This)->lpVtbl -> GetLogoCharW(This,convMode,pbstrChar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbLogoTransmissionDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbSIParameterDescriptor_INTERFACE_DEFINED__
#define __IIsdbSIParameterDescriptor_INTERFACE_DEFINED__

/* interface IIsdbSIParameterDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbSIParameterDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F837DC36-867C-426a-9111-F62093951A45")
    IIsdbSIParameterDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterVersion( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUpdateTime( 
            /* [out] */ MPEG_DATE *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordNumberOfTable( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptionLength( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptionBytes( 
            /* [in] */ BYTE bRecordIndex,
            /* [out][in] */ BYTE *pbBufferLength,
            /* [out] */ BYTE *pbBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbSIParameterDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbSIParameterDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbSIParameterDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbSIParameterDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbSIParameterDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbSIParameterDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetParameterVersion)
        HRESULT ( STDMETHODCALLTYPE *GetParameterVersion )( 
            IIsdbSIParameterDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetUpdateTime)
        HRESULT ( STDMETHODCALLTYPE *GetUpdateTime )( 
            IIsdbSIParameterDescriptor * This,
            /* [out] */ MPEG_DATE *pVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetRecordNumberOfTable)
        HRESULT ( STDMETHODCALLTYPE *GetRecordNumberOfTable )( 
            IIsdbSIParameterDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetTableId)
        HRESULT ( STDMETHODCALLTYPE *GetTableId )( 
            IIsdbSIParameterDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetTableDescriptionLength)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptionLength )( 
            IIsdbSIParameterDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbSIParameterDescriptor, GetTableDescriptionBytes)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptionBytes )( 
            IIsdbSIParameterDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out][in] */ BYTE *pbBufferLength,
            /* [out] */ BYTE *pbBuffer);
        
        END_INTERFACE
    } IIsdbSIParameterDescriptorVtbl;

    interface IIsdbSIParameterDescriptor
    {
        CONST_VTBL struct IIsdbSIParameterDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbSIParameterDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbSIParameterDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbSIParameterDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbSIParameterDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbSIParameterDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbSIParameterDescriptor_GetParameterVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetParameterVersion(This,pbVal) ) 

#define IIsdbSIParameterDescriptor_GetUpdateTime(This,pVal)	\
    ( (This)->lpVtbl -> GetUpdateTime(This,pVal) ) 

#define IIsdbSIParameterDescriptor_GetRecordNumberOfTable(This,pbVal)	\
    ( (This)->lpVtbl -> GetRecordNumberOfTable(This,pbVal) ) 

#define IIsdbSIParameterDescriptor_GetTableId(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetTableId(This,bRecordIndex,pbVal) ) 

#define IIsdbSIParameterDescriptor_GetTableDescriptionLength(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetTableDescriptionLength(This,bRecordIndex,pbVal) ) 

#define IIsdbSIParameterDescriptor_GetTableDescriptionBytes(This,bRecordIndex,pbBufferLength,pbBuffer)	\
    ( (This)->lpVtbl -> GetTableDescriptionBytes(This,bRecordIndex,pbBufferLength,pbBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbSIParameterDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbEmergencyInformationDescriptor_INTERFACE_DEFINED__
#define __IIsdbEmergencyInformationDescriptor_INTERFACE_DEFINED__

/* interface IIsdbEmergencyInformationDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbEmergencyInformationDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BA6FA681-B973-4da1-9207-AC3E7F0341EB")
    IIsdbEmergencyInformationDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceId( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStartEndFlag( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignalLevel( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAreaCode( 
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD **ppwVal,
            /* [out] */ BYTE *pbNumAreaCodes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbEmergencyInformationDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbEmergencyInformationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbEmergencyInformationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetServiceId)
        HRESULT ( STDMETHODCALLTYPE *GetServiceId )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetStartEndFlag)
        HRESULT ( STDMETHODCALLTYPE *GetStartEndFlag )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pVal);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetSignalLevel)
        HRESULT ( STDMETHODCALLTYPE *GetSignalLevel )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbEmergencyInformationDescriptor, GetAreaCode)
        HRESULT ( STDMETHODCALLTYPE *GetAreaCode )( 
            IIsdbEmergencyInformationDescriptor * This,
            /* [in] */ BYTE bRecordIndex,
            /* [out] */ WORD **ppwVal,
            /* [out] */ BYTE *pbNumAreaCodes);
        
        END_INTERFACE
    } IIsdbEmergencyInformationDescriptorVtbl;

    interface IIsdbEmergencyInformationDescriptor
    {
        CONST_VTBL struct IIsdbEmergencyInformationDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbEmergencyInformationDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbEmergencyInformationDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbEmergencyInformationDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbEmergencyInformationDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbEmergencyInformationDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbEmergencyInformationDescriptor_GetCountOfRecords(This,pbVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pbVal) ) 

#define IIsdbEmergencyInformationDescriptor_GetServiceId(This,bRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetServiceId(This,bRecordIndex,pwVal) ) 

#define IIsdbEmergencyInformationDescriptor_GetStartEndFlag(This,bRecordIndex,pVal)	\
    ( (This)->lpVtbl -> GetStartEndFlag(This,bRecordIndex,pVal) ) 

#define IIsdbEmergencyInformationDescriptor_GetSignalLevel(This,bRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetSignalLevel(This,bRecordIndex,pbVal) ) 

#define IIsdbEmergencyInformationDescriptor_GetAreaCode(This,bRecordIndex,ppwVal,pbNumAreaCodes)	\
    ( (This)->lpVtbl -> GetAreaCode(This,bRecordIndex,ppwVal,pbNumAreaCodes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbEmergencyInformationDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbCADescriptor_INTERFACE_DEFINED__
#define __IIsdbCADescriptor_INTERFACE_DEFINED__

/* interface IIsdbCADescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbCADescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0570AA47-52BC-42ae-8CA5-969F41E81AEA")
    IIsdbCADescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCASystemId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReservedBits( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAPID( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateDataBytes( 
            /* [out][in] */ BYTE *pbBufferLength,
            /* [out] */ BYTE *pbBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbCADescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbCADescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbCADescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbCADescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbCADescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbCADescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCADescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbCADescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCADescriptor, GetCASystemId)
        HRESULT ( STDMETHODCALLTYPE *GetCASystemId )( 
            IIsdbCADescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbCADescriptor, GetReservedBits)
        HRESULT ( STDMETHODCALLTYPE *GetReservedBits )( 
            IIsdbCADescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCADescriptor, GetCAPID)
        HRESULT ( STDMETHODCALLTYPE *GetCAPID )( 
            IIsdbCADescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbCADescriptor, GetPrivateDataBytes)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateDataBytes )( 
            IIsdbCADescriptor * This,
            /* [out][in] */ BYTE *pbBufferLength,
            /* [out] */ BYTE *pbBuffer);
        
        END_INTERFACE
    } IIsdbCADescriptorVtbl;

    interface IIsdbCADescriptor
    {
        CONST_VTBL struct IIsdbCADescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbCADescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbCADescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbCADescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbCADescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbCADescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbCADescriptor_GetCASystemId(This,pwVal)	\
    ( (This)->lpVtbl -> GetCASystemId(This,pwVal) ) 

#define IIsdbCADescriptor_GetReservedBits(This,pbVal)	\
    ( (This)->lpVtbl -> GetReservedBits(This,pbVal) ) 

#define IIsdbCADescriptor_GetCAPID(This,pwVal)	\
    ( (This)->lpVtbl -> GetCAPID(This,pwVal) ) 

#define IIsdbCADescriptor_GetPrivateDataBytes(This,pbBufferLength,pbBuffer)	\
    ( (This)->lpVtbl -> GetPrivateDataBytes(This,pbBufferLength,pbBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbCADescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbCAServiceDescriptor_INTERFACE_DEFINED__
#define __IIsdbCAServiceDescriptor_INTERFACE_DEFINED__

/* interface IIsdbCAServiceDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbCAServiceDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39CBEB97-FF0B-42a7-9AB9-7B9CFE70A77A")
    IIsdbCAServiceDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCASystemId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCABroadcasterGroupId( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMessageControl( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceIds( 
            /* [out][in] */ BYTE *pbNumServiceIds,
            /* [out] */ WORD *pwServiceIds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbCAServiceDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbCAServiceDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbCAServiceDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbCAServiceDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbCAServiceDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbCAServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAServiceDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbCAServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAServiceDescriptor, GetCASystemId)
        HRESULT ( STDMETHODCALLTYPE *GetCASystemId )( 
            IIsdbCAServiceDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAServiceDescriptor, GetCABroadcasterGroupId)
        HRESULT ( STDMETHODCALLTYPE *GetCABroadcasterGroupId )( 
            IIsdbCAServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAServiceDescriptor, GetMessageControl)
        HRESULT ( STDMETHODCALLTYPE *GetMessageControl )( 
            IIsdbCAServiceDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbCAServiceDescriptor, GetServiceIds)
        HRESULT ( STDMETHODCALLTYPE *GetServiceIds )( 
            IIsdbCAServiceDescriptor * This,
            /* [out][in] */ BYTE *pbNumServiceIds,
            /* [out] */ WORD *pwServiceIds);
        
        END_INTERFACE
    } IIsdbCAServiceDescriptorVtbl;

    interface IIsdbCAServiceDescriptor
    {
        CONST_VTBL struct IIsdbCAServiceDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbCAServiceDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbCAServiceDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbCAServiceDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbCAServiceDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbCAServiceDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbCAServiceDescriptor_GetCASystemId(This,pwVal)	\
    ( (This)->lpVtbl -> GetCASystemId(This,pwVal) ) 

#define IIsdbCAServiceDescriptor_GetCABroadcasterGroupId(This,pbVal)	\
    ( (This)->lpVtbl -> GetCABroadcasterGroupId(This,pbVal) ) 

#define IIsdbCAServiceDescriptor_GetMessageControl(This,pbVal)	\
    ( (This)->lpVtbl -> GetMessageControl(This,pbVal) ) 

#define IIsdbCAServiceDescriptor_GetServiceIds(This,pbNumServiceIds,pwServiceIds)	\
    ( (This)->lpVtbl -> GetServiceIds(This,pbNumServiceIds,pwServiceIds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbCAServiceDescriptor_INTERFACE_DEFINED__ */


#ifndef __IIsdbHierarchicalTransmissionDescriptor_INTERFACE_DEFINED__
#define __IIsdbHierarchicalTransmissionDescriptor_INTERFACE_DEFINED__

/* interface IIsdbHierarchicalTransmissionDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIsdbHierarchicalTransmissionDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B7B3AE90-EE0B-446d-8769-F7E2AA266AA6")
    IIsdbHierarchicalTransmissionDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFutureUse1( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQualityLevel( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFutureUse2( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReferencePid( 
            /* [out] */ WORD *pwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsdbHierarchicalTransmissionDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIsdbHierarchicalTransmissionDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIsdbHierarchicalTransmissionDescriptor * This);
        
        DECLSPEC_XFGVIRT(IIsdbHierarchicalTransmissionDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbHierarchicalTransmissionDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbHierarchicalTransmissionDescriptor, GetFutureUse1)
        HRESULT ( STDMETHODCALLTYPE *GetFutureUse1 )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbHierarchicalTransmissionDescriptor, GetQualityLevel)
        HRESULT ( STDMETHODCALLTYPE *GetQualityLevel )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbHierarchicalTransmissionDescriptor, GetFutureUse2)
        HRESULT ( STDMETHODCALLTYPE *GetFutureUse2 )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IIsdbHierarchicalTransmissionDescriptor, GetReferencePid)
        HRESULT ( STDMETHODCALLTYPE *GetReferencePid )( 
            IIsdbHierarchicalTransmissionDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IIsdbHierarchicalTransmissionDescriptorVtbl;

    interface IIsdbHierarchicalTransmissionDescriptor
    {
        CONST_VTBL struct IIsdbHierarchicalTransmissionDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsdbHierarchicalTransmissionDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsdbHierarchicalTransmissionDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsdbHierarchicalTransmissionDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsdbHierarchicalTransmissionDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IIsdbHierarchicalTransmissionDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IIsdbHierarchicalTransmissionDescriptor_GetFutureUse1(This,pbVal)	\
    ( (This)->lpVtbl -> GetFutureUse1(This,pbVal) ) 

#define IIsdbHierarchicalTransmissionDescriptor_GetQualityLevel(This,pbVal)	\
    ( (This)->lpVtbl -> GetQualityLevel(This,pbVal) ) 

#define IIsdbHierarchicalTransmissionDescriptor_GetFutureUse2(This,pbVal)	\
    ( (This)->lpVtbl -> GetFutureUse2(This,pbVal) ) 

#define IIsdbHierarchicalTransmissionDescriptor_GetReferencePid(This,pwVal)	\
    ( (This)->lpVtbl -> GetReferencePid(This,pwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsdbHierarchicalTransmissionDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dvbsiparser_0000_0065 */
/* [local] */ 

#define COMPONENT_TAG_CAPTION_MIN            0x30
#define COMPONENT_TAG_CAPTION_MAX            0x37
#define COMPONENT_TAG_SUPERIMPOSE_MIN        0x38
#define COMPONENT_TAG_SUPERIMPOSE_MAX        0x3F




extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0065_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0065_v0_0_s_ifspec;

#ifndef __IPBDASiParser_INTERFACE_DEFINED__
#define __IPBDASiParser_INTERFACE_DEFINED__

/* interface IPBDASiParser */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IPBDASiParser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9DE49A74-ABA2-4a18-93E1-21F17F95C3C3")
    IPBDASiParser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEIT( 
            /* [in] */ DWORD dwSize,
            /* [in] */ BYTE *pBuffer,
            /* [out] */ IPBDA_EIT **ppEIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServices( 
            /* [in] */ DWORD dwSize,
            /* [in] */ const BYTE *pBuffer,
            /* [out] */ IPBDA_Services **ppServices) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPBDASiParserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPBDASiParser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPBDASiParser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPBDASiParser * This);
        
        DECLSPEC_XFGVIRT(IPBDASiParser, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IPBDASiParser * This,
            /* [in] */ IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IPBDASiParser, GetEIT)
        HRESULT ( STDMETHODCALLTYPE *GetEIT )( 
            IPBDASiParser * This,
            /* [in] */ DWORD dwSize,
            /* [in] */ BYTE *pBuffer,
            /* [out] */ IPBDA_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IPBDASiParser, GetServices)
        HRESULT ( STDMETHODCALLTYPE *GetServices )( 
            IPBDASiParser * This,
            /* [in] */ DWORD dwSize,
            /* [in] */ const BYTE *pBuffer,
            /* [out] */ IPBDA_Services **ppServices);
        
        END_INTERFACE
    } IPBDASiParserVtbl;

    interface IPBDASiParser
    {
        CONST_VTBL struct IPBDASiParserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPBDASiParser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPBDASiParser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPBDASiParser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPBDASiParser_Initialize(This,punk)	\
    ( (This)->lpVtbl -> Initialize(This,punk) ) 

#define IPBDASiParser_GetEIT(This,dwSize,pBuffer,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT(This,dwSize,pBuffer,ppEIT) ) 

#define IPBDASiParser_GetServices(This,dwSize,pBuffer,ppServices)	\
    ( (This)->lpVtbl -> GetServices(This,dwSize,pBuffer,ppServices) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPBDASiParser_INTERFACE_DEFINED__ */


#ifndef __IPBDA_EIT_INTERFACE_DEFINED__
#define __IPBDA_EIT_INTERFACE_DEFINED__

/* interface IPBDA_EIT */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IPBDA_EIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A35F2DEA-098F-4ebd-984C-2BD4C3C8CE0A")
    IPBDA_EIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ DWORD size,
            /* [size_is][in] */ const BYTE *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableId( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceIdx( 
            /* [out] */ ULONG64 *plwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEventId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ ULONG64 *plwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordStartTime( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDuration( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DURATION *pmdVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPBDA_EITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPBDA_EIT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPBDA_EIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPBDA_EIT * This);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD size,
            /* [size_is][in] */ const BYTE *pBuffer);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetTableId)
        HRESULT ( STDMETHODCALLTYPE *GetTableId )( 
            IPBDA_EIT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IPBDA_EIT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetServiceIdx)
        HRESULT ( STDMETHODCALLTYPE *GetServiceIdx )( 
            IPBDA_EIT * This,
            /* [out] */ ULONG64 *plwVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IPBDA_EIT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetRecordEventId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEventId )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ ULONG64 *plwVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetRecordStartTime)
        HRESULT ( STDMETHODCALLTYPE *GetRecordStartTime )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetRecordDuration)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDuration )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ MPEG_DURATION *pmdVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IPBDA_EIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IPBDA_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } IPBDA_EITVtbl;

    interface IPBDA_EIT
    {
        CONST_VTBL struct IPBDA_EITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPBDA_EIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPBDA_EIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPBDA_EIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPBDA_EIT_Initialize(This,size,pBuffer)	\
    ( (This)->lpVtbl -> Initialize(This,size,pBuffer) ) 

#define IPBDA_EIT_GetTableId(This,pbVal)	\
    ( (This)->lpVtbl -> GetTableId(This,pbVal) ) 

#define IPBDA_EIT_GetVersionNumber(This,pwVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pwVal) ) 

#define IPBDA_EIT_GetServiceIdx(This,plwVal)	\
    ( (This)->lpVtbl -> GetServiceIdx(This,plwVal) ) 

#define IPBDA_EIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IPBDA_EIT_GetRecordEventId(This,dwRecordIndex,plwVal)	\
    ( (This)->lpVtbl -> GetRecordEventId(This,dwRecordIndex,plwVal) ) 

#define IPBDA_EIT_GetRecordStartTime(This,dwRecordIndex,pmdtVal)	\
    ( (This)->lpVtbl -> GetRecordStartTime(This,dwRecordIndex,pmdtVal) ) 

#define IPBDA_EIT_GetRecordDuration(This,dwRecordIndex,pmdVal)	\
    ( (This)->lpVtbl -> GetRecordDuration(This,dwRecordIndex,pmdVal) ) 

#define IPBDA_EIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IPBDA_EIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IPBDA_EIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPBDA_EIT_INTERFACE_DEFINED__ */


#ifndef __IPBDA_Services_INTERFACE_DEFINED__
#define __IPBDA_Services_INTERFACE_DEFINED__

/* interface IPBDA_Services */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IPBDA_Services;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("944EAB37-EED4-4850-AFD2-77E7EFEB4427")
    IPBDA_Services : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ DWORD size,
            /* [size_is][in] */ BYTE *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ ULONG64 *pul64ServiceIdx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPBDA_ServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPBDA_Services * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPBDA_Services * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPBDA_Services * This);
        
        DECLSPEC_XFGVIRT(IPBDA_Services, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IPBDA_Services * This,
            /* [in] */ DWORD size,
            /* [size_is][in] */ BYTE *pBuffer);
        
        DECLSPEC_XFGVIRT(IPBDA_Services, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IPBDA_Services * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IPBDA_Services, GetRecordByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordByIndex )( 
            IPBDA_Services * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ ULONG64 *pul64ServiceIdx);
        
        END_INTERFACE
    } IPBDA_ServicesVtbl;

    interface IPBDA_Services
    {
        CONST_VTBL struct IPBDA_ServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPBDA_Services_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPBDA_Services_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPBDA_Services_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPBDA_Services_Initialize(This,size,pBuffer)	\
    ( (This)->lpVtbl -> Initialize(This,size,pBuffer) ) 

#define IPBDA_Services_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IPBDA_Services_GetRecordByIndex(This,dwRecordIndex,pul64ServiceIdx)	\
    ( (This)->lpVtbl -> GetRecordByIndex(This,dwRecordIndex,pul64ServiceIdx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPBDA_Services_INTERFACE_DEFINED__ */


#ifndef __IPBDAEntitlementDescriptor_INTERFACE_DEFINED__
#define __IPBDAEntitlementDescriptor_INTERFACE_DEFINED__

/* interface IPBDAEntitlementDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IPBDAEntitlementDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22632497-0DE3-4587-AADC-D8D99017E760")
    IPBDAEntitlementDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetToken( 
            /* [out] */ BYTE **ppbTokenBuffer,
            /* [out] */ DWORD *pdwTokenLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPBDAEntitlementDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPBDAEntitlementDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPBDAEntitlementDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPBDAEntitlementDescriptor * This);
        
        DECLSPEC_XFGVIRT(IPBDAEntitlementDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IPBDAEntitlementDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IPBDAEntitlementDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IPBDAEntitlementDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPBDAEntitlementDescriptor, GetToken)
        HRESULT ( STDMETHODCALLTYPE *GetToken )( 
            IPBDAEntitlementDescriptor * This,
            /* [out] */ BYTE **ppbTokenBuffer,
            /* [out] */ DWORD *pdwTokenLength);
        
        END_INTERFACE
    } IPBDAEntitlementDescriptorVtbl;

    interface IPBDAEntitlementDescriptor
    {
        CONST_VTBL struct IPBDAEntitlementDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPBDAEntitlementDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPBDAEntitlementDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPBDAEntitlementDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPBDAEntitlementDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IPBDAEntitlementDescriptor_GetLength(This,pwVal)	\
    ( (This)->lpVtbl -> GetLength(This,pwVal) ) 

#define IPBDAEntitlementDescriptor_GetToken(This,ppbTokenBuffer,pdwTokenLength)	\
    ( (This)->lpVtbl -> GetToken(This,ppbTokenBuffer,pdwTokenLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPBDAEntitlementDescriptor_INTERFACE_DEFINED__ */


#ifndef __IPBDAAttributesDescriptor_INTERFACE_DEFINED__
#define __IPBDAAttributesDescriptor_INTERFACE_DEFINED__

/* interface IPBDAAttributesDescriptor */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IPBDAAttributesDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("313B3620-3263-45a6-9533-968BEFBEAC03")
    IPBDAAttributesDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributePayload( 
            /* [out] */ BYTE **ppbAttributeBuffer,
            /* [out] */ DWORD *pdwAttributeLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPBDAAttributesDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPBDAAttributesDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPBDAAttributesDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPBDAAttributesDescriptor * This);
        
        DECLSPEC_XFGVIRT(IPBDAAttributesDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IPBDAAttributesDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IPBDAAttributesDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IPBDAAttributesDescriptor * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPBDAAttributesDescriptor, GetAttributePayload)
        HRESULT ( STDMETHODCALLTYPE *GetAttributePayload )( 
            IPBDAAttributesDescriptor * This,
            /* [out] */ BYTE **ppbAttributeBuffer,
            /* [out] */ DWORD *pdwAttributeLength);
        
        END_INTERFACE
    } IPBDAAttributesDescriptorVtbl;

    interface IPBDAAttributesDescriptor
    {
        CONST_VTBL struct IPBDAAttributesDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPBDAAttributesDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPBDAAttributesDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPBDAAttributesDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPBDAAttributesDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IPBDAAttributesDescriptor_GetLength(This,pwVal)	\
    ( (This)->lpVtbl -> GetLength(This,pwVal) ) 

#define IPBDAAttributesDescriptor_GetAttributePayload(This,ppbAttributeBuffer,pdwAttributeLength)	\
    ( (This)->lpVtbl -> GetAttributePayload(This,ppbAttributeBuffer,pdwAttributeLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPBDAAttributesDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dvbsiparser_0000_0070 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0070_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dvbsiparser_0000_0070_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


