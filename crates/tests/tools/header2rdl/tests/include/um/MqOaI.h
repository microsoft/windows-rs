

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


#ifndef __mqoai_h__
#define __mqoai_h__

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

#ifndef __IMSMQQuery_FWD_DEFINED__
#define __IMSMQQuery_FWD_DEFINED__
typedef interface IMSMQQuery IMSMQQuery;

#endif 	/* __IMSMQQuery_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfo_FWD_DEFINED__
#define __IMSMQQueueInfo_FWD_DEFINED__
typedef interface IMSMQQueueInfo IMSMQQueueInfo;

#endif 	/* __IMSMQQueueInfo_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfo2_FWD_DEFINED__
#define __IMSMQQueueInfo2_FWD_DEFINED__
typedef interface IMSMQQueueInfo2 IMSMQQueueInfo2;

#endif 	/* __IMSMQQueueInfo2_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfo3_FWD_DEFINED__
#define __IMSMQQueueInfo3_FWD_DEFINED__
typedef interface IMSMQQueueInfo3 IMSMQQueueInfo3;

#endif 	/* __IMSMQQueueInfo3_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfo4_FWD_DEFINED__
#define __IMSMQQueueInfo4_FWD_DEFINED__
typedef interface IMSMQQueueInfo4 IMSMQQueueInfo4;

#endif 	/* __IMSMQQueueInfo4_FWD_DEFINED__ */


#ifndef __IMSMQQueue_FWD_DEFINED__
#define __IMSMQQueue_FWD_DEFINED__
typedef interface IMSMQQueue IMSMQQueue;

#endif 	/* __IMSMQQueue_FWD_DEFINED__ */


#ifndef __IMSMQQueue2_FWD_DEFINED__
#define __IMSMQQueue2_FWD_DEFINED__
typedef interface IMSMQQueue2 IMSMQQueue2;

#endif 	/* __IMSMQQueue2_FWD_DEFINED__ */


#ifndef __IMSMQQueue3_FWD_DEFINED__
#define __IMSMQQueue3_FWD_DEFINED__
typedef interface IMSMQQueue3 IMSMQQueue3;

#endif 	/* __IMSMQQueue3_FWD_DEFINED__ */


#ifndef __IMSMQQueue4_FWD_DEFINED__
#define __IMSMQQueue4_FWD_DEFINED__
typedef interface IMSMQQueue4 IMSMQQueue4;

#endif 	/* __IMSMQQueue4_FWD_DEFINED__ */


#ifndef __IMSMQMessage_FWD_DEFINED__
#define __IMSMQMessage_FWD_DEFINED__
typedef interface IMSMQMessage IMSMQMessage;

#endif 	/* __IMSMQMessage_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfos_FWD_DEFINED__
#define __IMSMQQueueInfos_FWD_DEFINED__
typedef interface IMSMQQueueInfos IMSMQQueueInfos;

#endif 	/* __IMSMQQueueInfos_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfos2_FWD_DEFINED__
#define __IMSMQQueueInfos2_FWD_DEFINED__
typedef interface IMSMQQueueInfos2 IMSMQQueueInfos2;

#endif 	/* __IMSMQQueueInfos2_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfos3_FWD_DEFINED__
#define __IMSMQQueueInfos3_FWD_DEFINED__
typedef interface IMSMQQueueInfos3 IMSMQQueueInfos3;

#endif 	/* __IMSMQQueueInfos3_FWD_DEFINED__ */


#ifndef __IMSMQQueueInfos4_FWD_DEFINED__
#define __IMSMQQueueInfos4_FWD_DEFINED__
typedef interface IMSMQQueueInfos4 IMSMQQueueInfos4;

#endif 	/* __IMSMQQueueInfos4_FWD_DEFINED__ */


#ifndef __IMSMQEvent_FWD_DEFINED__
#define __IMSMQEvent_FWD_DEFINED__
typedef interface IMSMQEvent IMSMQEvent;

#endif 	/* __IMSMQEvent_FWD_DEFINED__ */


#ifndef __IMSMQEvent2_FWD_DEFINED__
#define __IMSMQEvent2_FWD_DEFINED__
typedef interface IMSMQEvent2 IMSMQEvent2;

#endif 	/* __IMSMQEvent2_FWD_DEFINED__ */


#ifndef __IMSMQEvent3_FWD_DEFINED__
#define __IMSMQEvent3_FWD_DEFINED__
typedef interface IMSMQEvent3 IMSMQEvent3;

#endif 	/* __IMSMQEvent3_FWD_DEFINED__ */


#ifndef __IMSMQTransaction_FWD_DEFINED__
#define __IMSMQTransaction_FWD_DEFINED__
typedef interface IMSMQTransaction IMSMQTransaction;

#endif 	/* __IMSMQTransaction_FWD_DEFINED__ */


#ifndef __IMSMQCoordinatedTransactionDispenser_FWD_DEFINED__
#define __IMSMQCoordinatedTransactionDispenser_FWD_DEFINED__
typedef interface IMSMQCoordinatedTransactionDispenser IMSMQCoordinatedTransactionDispenser;

#endif 	/* __IMSMQCoordinatedTransactionDispenser_FWD_DEFINED__ */


#ifndef __IMSMQTransactionDispenser_FWD_DEFINED__
#define __IMSMQTransactionDispenser_FWD_DEFINED__
typedef interface IMSMQTransactionDispenser IMSMQTransactionDispenser;

#endif 	/* __IMSMQTransactionDispenser_FWD_DEFINED__ */


#ifndef __IMSMQQuery2_FWD_DEFINED__
#define __IMSMQQuery2_FWD_DEFINED__
typedef interface IMSMQQuery2 IMSMQQuery2;

#endif 	/* __IMSMQQuery2_FWD_DEFINED__ */


#ifndef __IMSMQQuery3_FWD_DEFINED__
#define __IMSMQQuery3_FWD_DEFINED__
typedef interface IMSMQQuery3 IMSMQQuery3;

#endif 	/* __IMSMQQuery3_FWD_DEFINED__ */


#ifndef __IMSMQQuery4_FWD_DEFINED__
#define __IMSMQQuery4_FWD_DEFINED__
typedef interface IMSMQQuery4 IMSMQQuery4;

#endif 	/* __IMSMQQuery4_FWD_DEFINED__ */


#ifndef __MSMQQuery_FWD_DEFINED__
#define __MSMQQuery_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQQuery MSMQQuery;
#else
typedef struct MSMQQuery MSMQQuery;
#endif /* __cplusplus */

#endif 	/* __MSMQQuery_FWD_DEFINED__ */


#ifndef __IMSMQMessage2_FWD_DEFINED__
#define __IMSMQMessage2_FWD_DEFINED__
typedef interface IMSMQMessage2 IMSMQMessage2;

#endif 	/* __IMSMQMessage2_FWD_DEFINED__ */


#ifndef __IMSMQMessage3_FWD_DEFINED__
#define __IMSMQMessage3_FWD_DEFINED__
typedef interface IMSMQMessage3 IMSMQMessage3;

#endif 	/* __IMSMQMessage3_FWD_DEFINED__ */


#ifndef __IMSMQMessage4_FWD_DEFINED__
#define __IMSMQMessage4_FWD_DEFINED__
typedef interface IMSMQMessage4 IMSMQMessage4;

#endif 	/* __IMSMQMessage4_FWD_DEFINED__ */


#ifndef __MSMQMessage_FWD_DEFINED__
#define __MSMQMessage_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQMessage MSMQMessage;
#else
typedef struct MSMQMessage MSMQMessage;
#endif /* __cplusplus */

#endif 	/* __MSMQMessage_FWD_DEFINED__ */


#ifndef __MSMQQueue_FWD_DEFINED__
#define __MSMQQueue_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQQueue MSMQQueue;
#else
typedef struct MSMQQueue MSMQQueue;
#endif /* __cplusplus */

#endif 	/* __MSMQQueue_FWD_DEFINED__ */


#ifndef __IMSMQPrivateEvent_FWD_DEFINED__
#define __IMSMQPrivateEvent_FWD_DEFINED__
typedef interface IMSMQPrivateEvent IMSMQPrivateEvent;

#endif 	/* __IMSMQPrivateEvent_FWD_DEFINED__ */


#ifndef ___DMSMQEventEvents_FWD_DEFINED__
#define ___DMSMQEventEvents_FWD_DEFINED__
typedef interface _DMSMQEventEvents _DMSMQEventEvents;

#endif 	/* ___DMSMQEventEvents_FWD_DEFINED__ */


#ifndef __MSMQEvent_FWD_DEFINED__
#define __MSMQEvent_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQEvent MSMQEvent;
#else
typedef struct MSMQEvent MSMQEvent;
#endif /* __cplusplus */

#endif 	/* __MSMQEvent_FWD_DEFINED__ */


#ifndef __MSMQQueueInfo_FWD_DEFINED__
#define __MSMQQueueInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQQueueInfo MSMQQueueInfo;
#else
typedef struct MSMQQueueInfo MSMQQueueInfo;
#endif /* __cplusplus */

#endif 	/* __MSMQQueueInfo_FWD_DEFINED__ */


#ifndef __MSMQQueueInfos_FWD_DEFINED__
#define __MSMQQueueInfos_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQQueueInfos MSMQQueueInfos;
#else
typedef struct MSMQQueueInfos MSMQQueueInfos;
#endif /* __cplusplus */

#endif 	/* __MSMQQueueInfos_FWD_DEFINED__ */


#ifndef __IMSMQTransaction2_FWD_DEFINED__
#define __IMSMQTransaction2_FWD_DEFINED__
typedef interface IMSMQTransaction2 IMSMQTransaction2;

#endif 	/* __IMSMQTransaction2_FWD_DEFINED__ */


#ifndef __IMSMQTransaction3_FWD_DEFINED__
#define __IMSMQTransaction3_FWD_DEFINED__
typedef interface IMSMQTransaction3 IMSMQTransaction3;

#endif 	/* __IMSMQTransaction3_FWD_DEFINED__ */


#ifndef __MSMQTransaction_FWD_DEFINED__
#define __MSMQTransaction_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQTransaction MSMQTransaction;
#else
typedef struct MSMQTransaction MSMQTransaction;
#endif /* __cplusplus */

#endif 	/* __MSMQTransaction_FWD_DEFINED__ */


#ifndef __IMSMQCoordinatedTransactionDispenser2_FWD_DEFINED__
#define __IMSMQCoordinatedTransactionDispenser2_FWD_DEFINED__
typedef interface IMSMQCoordinatedTransactionDispenser2 IMSMQCoordinatedTransactionDispenser2;

#endif 	/* __IMSMQCoordinatedTransactionDispenser2_FWD_DEFINED__ */


#ifndef __IMSMQCoordinatedTransactionDispenser3_FWD_DEFINED__
#define __IMSMQCoordinatedTransactionDispenser3_FWD_DEFINED__
typedef interface IMSMQCoordinatedTransactionDispenser3 IMSMQCoordinatedTransactionDispenser3;

#endif 	/* __IMSMQCoordinatedTransactionDispenser3_FWD_DEFINED__ */


#ifndef __MSMQCoordinatedTransactionDispenser_FWD_DEFINED__
#define __MSMQCoordinatedTransactionDispenser_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQCoordinatedTransactionDispenser MSMQCoordinatedTransactionDispenser;
#else
typedef struct MSMQCoordinatedTransactionDispenser MSMQCoordinatedTransactionDispenser;
#endif /* __cplusplus */

#endif 	/* __MSMQCoordinatedTransactionDispenser_FWD_DEFINED__ */


#ifndef __IMSMQTransactionDispenser2_FWD_DEFINED__
#define __IMSMQTransactionDispenser2_FWD_DEFINED__
typedef interface IMSMQTransactionDispenser2 IMSMQTransactionDispenser2;

#endif 	/* __IMSMQTransactionDispenser2_FWD_DEFINED__ */


#ifndef __IMSMQTransactionDispenser3_FWD_DEFINED__
#define __IMSMQTransactionDispenser3_FWD_DEFINED__
typedef interface IMSMQTransactionDispenser3 IMSMQTransactionDispenser3;

#endif 	/* __IMSMQTransactionDispenser3_FWD_DEFINED__ */


#ifndef __MSMQTransactionDispenser_FWD_DEFINED__
#define __MSMQTransactionDispenser_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQTransactionDispenser MSMQTransactionDispenser;
#else
typedef struct MSMQTransactionDispenser MSMQTransactionDispenser;
#endif /* __cplusplus */

#endif 	/* __MSMQTransactionDispenser_FWD_DEFINED__ */


#ifndef __IMSMQApplication_FWD_DEFINED__
#define __IMSMQApplication_FWD_DEFINED__
typedef interface IMSMQApplication IMSMQApplication;

#endif 	/* __IMSMQApplication_FWD_DEFINED__ */


#ifndef __IMSMQApplication2_FWD_DEFINED__
#define __IMSMQApplication2_FWD_DEFINED__
typedef interface IMSMQApplication2 IMSMQApplication2;

#endif 	/* __IMSMQApplication2_FWD_DEFINED__ */


#ifndef __IMSMQApplication3_FWD_DEFINED__
#define __IMSMQApplication3_FWD_DEFINED__
typedef interface IMSMQApplication3 IMSMQApplication3;

#endif 	/* __IMSMQApplication3_FWD_DEFINED__ */


#ifndef __MSMQApplication_FWD_DEFINED__
#define __MSMQApplication_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQApplication MSMQApplication;
#else
typedef struct MSMQApplication MSMQApplication;
#endif /* __cplusplus */

#endif 	/* __MSMQApplication_FWD_DEFINED__ */


#ifndef __IMSMQDestination_FWD_DEFINED__
#define __IMSMQDestination_FWD_DEFINED__
typedef interface IMSMQDestination IMSMQDestination;

#endif 	/* __IMSMQDestination_FWD_DEFINED__ */


#ifndef __IMSMQPrivateDestination_FWD_DEFINED__
#define __IMSMQPrivateDestination_FWD_DEFINED__
typedef interface IMSMQPrivateDestination IMSMQPrivateDestination;

#endif 	/* __IMSMQPrivateDestination_FWD_DEFINED__ */


#ifndef __MSMQDestination_FWD_DEFINED__
#define __MSMQDestination_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQDestination MSMQDestination;
#else
typedef struct MSMQDestination MSMQDestination;
#endif /* __cplusplus */

#endif 	/* __MSMQDestination_FWD_DEFINED__ */


#ifndef __IMSMQCollection_FWD_DEFINED__
#define __IMSMQCollection_FWD_DEFINED__
typedef interface IMSMQCollection IMSMQCollection;

#endif 	/* __IMSMQCollection_FWD_DEFINED__ */


#ifndef __MSMQCollection_FWD_DEFINED__
#define __MSMQCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQCollection MSMQCollection;
#else
typedef struct MSMQCollection MSMQCollection;
#endif /* __cplusplus */

#endif 	/* __MSMQCollection_FWD_DEFINED__ */


#ifndef __IMSMQManagement_FWD_DEFINED__
#define __IMSMQManagement_FWD_DEFINED__
typedef interface IMSMQManagement IMSMQManagement;

#endif 	/* __IMSMQManagement_FWD_DEFINED__ */


#ifndef __MSMQManagement_FWD_DEFINED__
#define __MSMQManagement_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQManagement MSMQManagement;
#else
typedef struct MSMQManagement MSMQManagement;
#endif /* __cplusplus */

#endif 	/* __MSMQManagement_FWD_DEFINED__ */


#ifndef __IMSMQOutgoingQueueManagement_FWD_DEFINED__
#define __IMSMQOutgoingQueueManagement_FWD_DEFINED__
typedef interface IMSMQOutgoingQueueManagement IMSMQOutgoingQueueManagement;

#endif 	/* __IMSMQOutgoingQueueManagement_FWD_DEFINED__ */


#ifndef __MSMQOutgoingQueueManagement_FWD_DEFINED__
#define __MSMQOutgoingQueueManagement_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQOutgoingQueueManagement MSMQOutgoingQueueManagement;
#else
typedef struct MSMQOutgoingQueueManagement MSMQOutgoingQueueManagement;
#endif /* __cplusplus */

#endif 	/* __MSMQOutgoingQueueManagement_FWD_DEFINED__ */


#ifndef __IMSMQQueueManagement_FWD_DEFINED__
#define __IMSMQQueueManagement_FWD_DEFINED__
typedef interface IMSMQQueueManagement IMSMQQueueManagement;

#endif 	/* __IMSMQQueueManagement_FWD_DEFINED__ */


#ifndef __MSMQQueueManagement_FWD_DEFINED__
#define __MSMQQueueManagement_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSMQQueueManagement MSMQQueueManagement;
#else
typedef struct MSMQQueueManagement MSMQQueueManagement;
#endif /* __cplusplus */

#endif 	/* __MSMQQueueManagement_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mqoai_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mqoai_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mqoai_0000_0000_v0_0_s_ifspec;


#ifndef __MSMQ_LIBRARY_DEFINED__
#define __MSMQ_LIBRARY_DEFINED__

/* library MSMQ */
/* [version][lcid][helpstringdll][helpstring][uuid] */ 

#ifndef MIDL_INTERFACE
#if _MSC_VER >= 1100
#define MIDL_INTERFACE(x)   struct __declspec(uuid(x)) __declspec(novtable)
#else
#define MIDL_INTERFACE(x)   struct
#endif //_MSC_VER
#endif //MIDL_INTERFACE

typedef short Boolean;

typedef unsigned char BYTE;

typedef unsigned long ULONG;

typedef unsigned long DWORD;

typedef int BOOL;





















/* [helpstringcontext] */ 
enum MQCALG
    {
        MQMSG_CALG_MD2	= ( ( 0x8000 + 0 )  + 1 ) ,
        MQMSG_CALG_MD4	= ( ( 0x8000 + 0 )  + 2 ) ,
        MQMSG_CALG_MD5	= ( ( 0x8000 + 0 )  + 3 ) ,
        MQMSG_CALG_SHA	= ( ( 0x8000 + 0 )  + 4 ) ,
        MQMSG_CALG_SHA1	= ( ( 0x8000 + 0 )  + 4 ) ,
        MQMSG_CALG_MAC	= ( ( 0x8000 + 0 )  + 5 ) ,
        MQMSG_CALG_RSA_SIGN	= ( ( 0x2000 + 0x400 )  + 0 ) ,
        MQMSG_CALG_DSS_SIGN	= ( ( 0x2000 + 0x200 )  + 0 ) ,
        MQMSG_CALG_RSA_KEYX	= ( ( 0xa000 + 0x400 )  + 0 ) ,
        MQMSG_CALG_DES	= ( ( 0x6000 + 0x600 )  + 1 ) ,
        MQMSG_CALG_RC2	= ( ( 0x6000 + 0x600 )  + 2 ) ,
        MQMSG_CALG_RC4	= ( ( 0x6000 + 0x800 )  + 1 ) ,
        MQMSG_CALG_SEAL	= ( ( 0x6000 + 0x800 )  + 2 ) 
    } ;
/* [helpstringcontext] */ 
enum MQTRANSACTION
    {
        MQ_NO_TRANSACTION	= 0,
        MQ_MTS_TRANSACTION	= 1,
        MQ_XA_TRANSACTION	= 2,
        MQ_SINGLE_MESSAGE	= 3
    } ;
/* [helpstringcontext] */ 
enum RELOPS
    {
        REL_NOP	= 0,
        REL_EQ	= ( REL_NOP + 1 ) ,
        REL_NEQ	= ( REL_EQ + 1 ) ,
        REL_LT	= ( REL_NEQ + 1 ) ,
        REL_GT	= ( REL_LT + 1 ) ,
        REL_LE	= ( REL_GT + 1 ) ,
        REL_GE	= ( REL_LE + 1 ) 
    } ;
/* [helpstringcontext] */ 
enum MQCERT_REGISTER
    {
        MQCERT_REGISTER_ALWAYS	= 1,
        MQCERT_REGISTER_IF_NOT_EXIST	= 2
    } ;
/* [helpstringcontext] */ 
enum MQMSGCURSOR
    {
        MQMSG_FIRST	= 0,
        MQMSG_CURRENT	= 1,
        MQMSG_NEXT	= 2
    } ;
/* [helpstringcontext] */ 
enum MQMSGCLASS
    {
        MQMSG_CLASS_NORMAL	= ( ( 0 + 0 )  + 0 ) ,
        MQMSG_CLASS_REPORT	= ( ( 0 + 0 )  + 0x1 ) ,
        MQMSG_CLASS_ACK_REACH_QUEUE	= ( ( 0 + 0 )  + 0x2 ) ,
        MQMSG_CLASS_ACK_RECEIVE	= ( ( 0 + 0x4000 )  + 0 ) ,
        MQMSG_CLASS_NACK_BAD_DST_Q	= ( ( 0x8000 + 0 )  + 0 ) ,
        MQMSG_CLASS_NACK_PURGED	= ( ( 0x8000 + 0 )  + 0x1 ) ,
        MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT	= ( ( 0x8000 + 0 )  + 0x2 ) ,
        MQMSG_CLASS_NACK_Q_EXCEED_QUOTA	= ( ( 0x8000 + 0 )  + 0x3 ) ,
        MQMSG_CLASS_NACK_ACCESS_DENIED	= ( ( 0x8000 + 0 )  + 0x4 ) ,
        MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED	= ( ( 0x8000 + 0 )  + 0x5 ) ,
        MQMSG_CLASS_NACK_BAD_SIGNATURE	= ( ( 0x8000 + 0 )  + 0x6 ) ,
        MQMSG_CLASS_NACK_BAD_ENCRYPTION	= ( ( 0x8000 + 0 )  + 0x7 ) ,
        MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT	= ( ( 0x8000 + 0 )  + 0x8 ) ,
        MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q	= ( ( 0x8000 + 0 )  + 0x9 ) ,
        MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG	= ( ( 0x8000 + 0 )  + 0xa ) ,
        MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER	= ( ( 0x8000 + 0 )  + 0xb ) ,
        MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED	= ( ( 0x8000 + 0 )  + 0xc ) ,
        MQMSG_CLASS_NACK_Q_DELETED	= ( ( 0x8000 + 0x4000 )  + 0 ) ,
        MQMSG_CLASS_NACK_Q_PURGED	= ( ( 0x8000 + 0x4000 )  + 0x1 ) ,
        MQMSG_CLASS_NACK_RECEIVE_TIMEOUT	= ( ( 0x8000 + 0x4000 )  + 0x2 ) ,
        MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER	= ( ( 0x8000 + 0x4000 )  + 0x3 ) 
    } ;
/* [helpstringcontext] */ 
enum MQMSGDELIVERY
    {
        MQMSG_DELIVERY_EXPRESS	= 0,
        MQMSG_DELIVERY_RECOVERABLE	= 1
    } ;
/* [helpstringcontext] */ 
enum MQMSGACKNOWLEDGEMENT
    {
        MQMSG_ACKNOWLEDGMENT_NONE	= 0,
        MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL	= 0x1,
        MQMSG_ACKNOWLEDGMENT_POS_RECEIVE	= 0x2,
        MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL	= 0x4,
        MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE	= 0x8,
        MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE	= 0x4,
        MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE	= ( 0x4 + 0x1 ) ,
        MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE	= ( 0x4 + 0x8 ) ,
        MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE	= ( ( 0x4 + 0x8 )  + 0x2 ) 
    } ;
/* [helpstringcontext] */ 
enum MQMSGJOURNAL
    {
        MQMSG_JOURNAL_NONE	= 0,
        MQMSG_DEADLETTER	= 1,
        MQMSG_JOURNAL	= 2
    } ;
/* [helpstringcontext] */ 
enum MQMSGTRACE
    {
        MQMSG_TRACE_NONE	= 0,
        MQMSG_SEND_ROUTE_TO_REPORT_QUEUE	= 1
    } ;
/* [helpstringcontext] */ 
enum MQMSGSENDERIDTYPE
    {
        MQMSG_SENDERID_TYPE_NONE	= 0,
        MQMSG_SENDERID_TYPE_SID	= 1
    } ;
/* [helpstringcontext] */ 
enum MQMSGPRIVLEVEL
    {
        MQMSG_PRIV_LEVEL_NONE	= 0,
        MQMSG_PRIV_LEVEL_BODY_BASE	= 1,
        MQMSG_PRIV_LEVEL_BODY_ENHANCED	= 3
    } ;
/* [helpstringcontext] */ 
enum MQMSGAUTHLEVEL
    {
        MQMSG_AUTH_LEVEL_NONE	= 0,
        MQMSG_AUTH_LEVEL_ALWAYS	= 1,
        MQMSG_AUTH_LEVEL_MSMQ10	= 2,
        MQMSG_AUTH_LEVEL_SIG10	= 2,
        MQMSG_AUTH_LEVEL_MSMQ20	= 4,
        MQMSG_AUTH_LEVEL_SIG20	= 4,
        MQMSG_AUTH_LEVEL_SIG30	= 8
    } ;
/* [helpstringcontext] */ 
enum MQMSGIDSIZE
    {
        MQMSG_MSGID_SIZE	= 20,
        MQMSG_CORRELATIONID_SIZE	= 20,
        MQMSG_XACTID_SIZE	= 20
    } ;
/* [helpstringcontext] */ 
enum MQMSGMAX
    {
        MQ_MAX_MSG_LABEL_LEN	= 249
    } ;
/* [helpstringcontext] */ 
enum MQMSGAUTHENTICATION
    {
        MQMSG_AUTHENTICATION_NOT_REQUESTED	= 0,
        MQMSG_AUTHENTICATION_REQUESTED	= 1,
        MQMSG_AUTHENTICATED_SIG10	= 1,
        MQMSG_AUTHENTICATION_REQUESTED_EX	= 3,
        MQMSG_AUTHENTICATED_SIG20	= 3,
        MQMSG_AUTHENTICATED_SIG30	= 5,
        MQMSG_AUTHENTICATED_SIGXML	= 9
    } ;
/* [helpstringcontext] */ 
enum MQSHARE
    {
        MQ_DENY_NONE	= 0,
        MQ_DENY_RECEIVE_SHARE	= 1
    } ;
/* [helpstringcontext] */ 
enum MQACCESS
    {
        MQ_RECEIVE_ACCESS	= 1,
        MQ_SEND_ACCESS	= 2,
        MQ_PEEK_ACCESS	= 0x20,
        MQ_ADMIN_ACCESS	= 0x80
    } ;
/* [helpstringcontext] */ 
enum MQJOURNAL
    {
        MQ_JOURNAL_NONE	= 0,
        MQ_JOURNAL	= 1
    } ;
/* [helpstringcontext] */ 
enum MQTRANSACTIONAL
    {
        MQ_TRANSACTIONAL_NONE	= 0,
        MQ_TRANSACTIONAL	= 1
    } ;
/* [helpstringcontext] */ 
enum MQAUTHENTICATE
    {
        MQ_AUTHENTICATE_NONE	= 0,
        MQ_AUTHENTICATE	= 1
    } ;
/* [helpstringcontext] */ 
enum MQPRIVLEVEL
    {
        MQ_PRIV_LEVEL_NONE	= 0,
        MQ_PRIV_LEVEL_OPTIONAL	= 1,
        MQ_PRIV_LEVEL_BODY	= 2
    } ;
/* [helpstringcontext] */ 
enum MQPRIORITY
    {
        MQ_MIN_PRIORITY	= 0,
        MQ_MAX_PRIORITY	= 7
    } ;
/* [helpstringcontext] */ 
enum MQMAX
    {
        MQ_MAX_Q_NAME_LEN	= 124,
        MQ_MAX_Q_LABEL_LEN	= 124
    } ;
/* [helpstringcontext] */ 
enum QUEUE_TYPE
    {
        MQ_TYPE_PUBLIC	= 0,
        MQ_TYPE_PRIVATE	= ( MQ_TYPE_PUBLIC + 1 ) ,
        MQ_TYPE_MACHINE	= ( MQ_TYPE_PRIVATE + 1 ) ,
        MQ_TYPE_CONNECTOR	= ( MQ_TYPE_MACHINE + 1 ) ,
        MQ_TYPE_MULTICAST	= ( MQ_TYPE_CONNECTOR + 1 ) 
    } ;
/* [helpstringcontext] */ 
enum FOREIGN_STATUS
    {
        MQ_STATUS_FOREIGN	= 0,
        MQ_STATUS_NOT_FOREIGN	= ( MQ_STATUS_FOREIGN + 1 ) ,
        MQ_STATUS_UNKNOWN	= ( MQ_STATUS_NOT_FOREIGN + 1 ) 
    } ;

enum XACT_STATUS
    {
        MQ_XACT_STATUS_XACT	= 0,
        MQ_XACT_STATUS_NOT_XACT	= ( MQ_XACT_STATUS_XACT + 1 ) ,
        MQ_XACT_STATUS_UNKNOWN	= ( MQ_XACT_STATUS_NOT_XACT + 1 ) 
    } ;
/* [helpstringcontext] */ 
enum QUEUE_STATE
    {
        MQ_QUEUE_STATE_LOCAL_CONNECTION	= 0,
        MQ_QUEUE_STATE_DISCONNECTED	= ( MQ_QUEUE_STATE_LOCAL_CONNECTION + 1 ) ,
        MQ_QUEUE_STATE_WAITING	= ( MQ_QUEUE_STATE_DISCONNECTED + 1 ) ,
        MQ_QUEUE_STATE_NEEDVALIDATE	= ( MQ_QUEUE_STATE_WAITING + 1 ) ,
        MQ_QUEUE_STATE_ONHOLD	= ( MQ_QUEUE_STATE_NEEDVALIDATE + 1 ) ,
        MQ_QUEUE_STATE_NONACTIVE	= ( MQ_QUEUE_STATE_ONHOLD + 1 ) ,
        MQ_QUEUE_STATE_CONNECTED	= ( MQ_QUEUE_STATE_NONACTIVE + 1 ) ,
        MQ_QUEUE_STATE_DISCONNECTING	= ( MQ_QUEUE_STATE_CONNECTED + 1 ) ,
        MQ_QUEUE_STATE_LOCKED	= ( MQ_QUEUE_STATE_DISCONNECTING + 1 ) 
    } ;
/* [helpstringcontext] */ 
enum MQDEFAULT
    {
        DEFAULT_M_PRIORITY	= 3,
        DEFAULT_M_DELIVERY	= 0,
        DEFAULT_M_ACKNOWLEDGE	= 0,
        DEFAULT_M_JOURNAL	= 0,
        DEFAULT_M_APPSPECIFIC	= 0,
        DEFAULT_M_PRIV_LEVEL	= 0,
        DEFAULT_M_AUTH_LEVEL	= 0,
        DEFAULT_M_SENDERID_TYPE	= 1,
        DEFAULT_Q_JOURNAL	= 0,
        DEFAULT_Q_BASEPRIORITY	= 0,
        DEFAULT_Q_QUOTA	= 0xffffffff,
        DEFAULT_Q_JOURNAL_QUOTA	= 0xffffffff,
        DEFAULT_Q_TRANSACTION	= 0,
        DEFAULT_Q_AUTHENTICATE	= 0,
        DEFAULT_Q_PRIV_LEVEL	= 1,
        DEFAULT_M_LOOKUPID	= 0
    } ;
/* [helpstringcontext] */ 
enum MQERROR
    {
        MQ_ERROR	= 0xc00e0001,
        MQ_ERROR_PROPERTY	= 0xc00e0002,
        MQ_ERROR_QUEUE_NOT_FOUND	= 0xc00e0003,
        MQ_ERROR_QUEUE_NOT_ACTIVE	= 0xc00e0004,
        MQ_ERROR_QUEUE_EXISTS	= 0xc00e0005,
        MQ_ERROR_INVALID_PARAMETER	= 0xc00e0006,
        MQ_ERROR_INVALID_HANDLE	= 0xc00e0007,
        MQ_ERROR_OPERATION_CANCELLED	= 0xc00e0008,
        MQ_ERROR_SHARING_VIOLATION	= 0xc00e0009,
        MQ_ERROR_SERVICE_NOT_AVAILABLE	= 0xc00e000b,
        MQ_ERROR_MACHINE_NOT_FOUND	= 0xc00e000d,
        MQ_ERROR_ILLEGAL_SORT	= 0xc00e0010,
        MQ_ERROR_ILLEGAL_USER	= 0xc00e0011,
        MQ_ERROR_NO_DS	= 0xc00e0013,
        MQ_ERROR_ILLEGAL_QUEUE_PATHNAME	= 0xc00e0014,
        MQ_ERROR_ILLEGAL_PROPERTY_VALUE	= 0xc00e0018,
        MQ_ERROR_ILLEGAL_PROPERTY_VT	= 0xc00e0019,
        MQ_ERROR_BUFFER_OVERFLOW	= 0xc00e001a,
        MQ_ERROR_IO_TIMEOUT	= 0xc00e001b,
        MQ_ERROR_ILLEGAL_CURSOR_ACTION	= 0xc00e001c,
        MQ_ERROR_MESSAGE_ALREADY_RECEIVED	= 0xc00e001d,
        MQ_ERROR_ILLEGAL_FORMATNAME	= 0xc00e001e,
        MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL	= 0xc00e001f,
        MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION	= 0xc00e0020,
        MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR	= 0xc00e0021,
        MQ_ERROR_SENDERID_BUFFER_TOO_SMALL	= 0xc00e0022,
        MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL	= 0xc00e0023,
        MQ_ERROR_CANNOT_IMPERSONATE_CLIENT	= 0xc00e0024,
        MQ_ERROR_ACCESS_DENIED	= 0xc00e0025,
        MQ_ERROR_PRIVILEGE_NOT_HELD	= 0xc00e0026,
        MQ_ERROR_INSUFFICIENT_RESOURCES	= 0xc00e0027,
        MQ_ERROR_USER_BUFFER_TOO_SMALL	= 0xc00e0028,
        MQ_ERROR_MESSAGE_STORAGE_FAILED	= 0xc00e002a,
        MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL	= 0xc00e002b,
        MQ_ERROR_INVALID_CERTIFICATE	= 0xc00e002c,
        MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE	= 0xc00e002d,
        MQ_ERROR_INTERNAL_USER_CERT_EXIST	= 0xc00e002e,
        MQ_ERROR_NO_INTERNAL_USER_CERT	= 0xc00e002f,
        MQ_ERROR_CORRUPTED_SECURITY_DATA	= 0xc00e0030,
        MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE	= 0xc00e0031,
        MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION	= 0xc00e0033,
        MQ_ERROR_BAD_SECURITY_CONTEXT	= 0xc00e0035,
        MQ_ERROR_COULD_NOT_GET_USER_SID	= 0xc00e0036,
        MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO	= 0xc00e0037,
        MQ_ERROR_ILLEGAL_MQCOLUMNS	= 0xc00e0038,
        MQ_ERROR_ILLEGAL_PROPID	= 0xc00e0039,
        MQ_ERROR_ILLEGAL_RELATION	= 0xc00e003a,
        MQ_ERROR_ILLEGAL_PROPERTY_SIZE	= 0xc00e003b,
        MQ_ERROR_ILLEGAL_RESTRICTION_PROPID	= 0xc00e003c,
        MQ_ERROR_ILLEGAL_MQQUEUEPROPS	= 0xc00e003d,
        MQ_ERROR_PROPERTY_NOTALLOWED	= 0xc00e003e,
        MQ_ERROR_INSUFFICIENT_PROPERTIES	= 0xc00e003f,
        MQ_ERROR_MACHINE_EXISTS	= 0xc00e0040,
        MQ_ERROR_ILLEGAL_MQQMPROPS	= 0xc00e0041,
        MQ_ERROR_DS_IS_FULL	= 0xc00e0042L,
        MQ_ERROR_DS_ERROR	= 0xc00e0043,
        MQ_ERROR_INVALID_OWNER	= 0xc00e0044,
        MQ_ERROR_UNSUPPORTED_ACCESS_MODE	= 0xc00e0045,
        MQ_ERROR_RESULT_BUFFER_TOO_SMALL	= 0xc00e0046,
        MQ_ERROR_DELETE_CN_IN_USE	= 0xc00e0048L,
        MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER	= 0xc00e0049,
        MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE	= 0xc00e004a,
        MQ_ERROR_QUEUE_NOT_AVAILABLE	= 0xc00e004b,
        MQ_ERROR_DTC_CONNECT	= 0xc00e004c,
        MQ_ERROR_TRANSACTION_IMPORT	= 0xc00e004e,
        MQ_ERROR_TRANSACTION_USAGE	= 0xc00e0050,
        MQ_ERROR_TRANSACTION_SEQUENCE	= 0xc00e0051,
        MQ_ERROR_MISSING_CONNECTOR_TYPE	= 0xc00e0055,
        MQ_ERROR_STALE_HANDLE	= 0xc00e0056,
        MQ_ERROR_TRANSACTION_ENLIST	= 0xc00e0058,
        MQ_ERROR_QUEUE_DELETED	= 0xc00e005a,
        MQ_ERROR_ILLEGAL_CONTEXT	= 0xc00e005b,
        MQ_ERROR_ILLEGAL_SORT_PROPID	= 0xc00e005c,
        MQ_ERROR_LABEL_TOO_LONG	= 0xc00e005d,
        MQ_ERROR_LABEL_BUFFER_TOO_SMALL	= 0xc00e005e,
        MQ_ERROR_MQIS_SERVER_EMPTY	= 0xc00e005fL,
        MQ_ERROR_MQIS_READONLY_MODE	= 0xc00e0060L,
        MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL	= 0xc00e0061,
        MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL	= 0xc00e0062,
        MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL	= 0xc00e0063,
        MQ_ERROR_ILLEGAL_OPERATION	= 0xc00e0064,
        MQ_ERROR_WRITE_NOT_ALLOWED	= 0xc00e0065L,
        MQ_ERROR_WKS_CANT_SERVE_CLIENT	= 0xc00e0066L,
        MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW	= 0xc00e0067L,
        MQ_CORRUPTED_QUEUE_WAS_DELETED	= 0xc00e0068L,
        MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE	= 0xc00e0069L,
        MQ_ERROR_UNSUPPORTED_OPERATION	= 0xc00e006aL,
        MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED	= 0xc00e006bL,
        MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR	= 0xc00e006cL,
        MQ_ERROR_CERTIFICATE_NOT_PROVIDED	= 0xc00e006dL,
        MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED	= 0xc00e006eL,
        MQ_ERROR_CANT_CREATE_CERT_STORE	= 0xc00e006fL,
        MQ_ERROR_CANNOT_CREATE_CERT_STORE	= 0xc00e006fL,
        MQ_ERROR_CANT_OPEN_CERT_STORE	= 0xc00e0070L,
        MQ_ERROR_CANNOT_OPEN_CERT_STORE	= 0xc00e0070L,
        MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION	= 0xc00e0071L,
        MQ_ERROR_CANNOT_GRANT_ADD_GUID	= 0xc00e0072L,
        MQ_ERROR_CANNOT_LOAD_MSMQOCM	= 0xc00e0073L,
        MQ_ERROR_NO_ENTRY_POINT_MSMQOCM	= 0xc00e0074L,
        MQ_ERROR_NO_MSMQ_SERVERS_ON_DC	= 0xc00e0075L,
        MQ_ERROR_CANNOT_JOIN_DOMAIN	= 0xc00e0076L,
        MQ_ERROR_CANNOT_CREATE_ON_GC	= 0xc00e0077L,
        MQ_ERROR_GUID_NOT_MATCHING	= 0xc00e0078L,
        MQ_ERROR_PUBLIC_KEY_NOT_FOUND	= 0xc00e0079L,
        MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST	= 0xc00e007aL,
        MQ_ERROR_ILLEGAL_MQPRIVATEPROPS	= 0xc00e007bL,
        MQ_ERROR_NO_GC_IN_DOMAIN	= 0xc00e007cL,
        MQ_ERROR_NO_MSMQ_SERVERS_ON_GC	= 0xc00e007dL,
        MQ_ERROR_CANNOT_GET_DN	= 0xc00e007eL,
        MQ_ERROR_CANNOT_HASH_DATA_EX	= 0xc00e007fL,
        MQ_ERROR_CANNOT_SIGN_DATA_EX	= 0xc00e0080L,
        MQ_ERROR_CANNOT_CREATE_HASH_EX	= 0xc00e0081L,
        MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX	= 0xc00e0082L,
        MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS	= 0xc00e0083L,
        MQ_ERROR_NO_MQUSER_OU	= 0xc00e0084L,
        MQ_ERROR_CANNOT_LOAD_MQAD	= 0xc00e0085L,
        MQ_ERROR_CANNOT_LOAD_MQDSSRV	= 0xc00e0086L,
        MQ_ERROR_PROPERTIES_CONFLICT	= 0xc00e0087L,
        MQ_ERROR_MESSAGE_NOT_FOUND	= 0xc00e0088L,
        MQ_ERROR_CANT_RESOLVE_SITES	= 0xc00e0089L,
        MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS	= 0xc00e008aL,
        MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER	= 0xc00e008bL,
        MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS	= 0xc00e008cL,
        MQ_ERROR_MULTI_SORT_KEYS	= 0xc00e008dL,
        MQ_ERROR_GC_NEEDED	= 0xc00e008eL,
        MQ_ERROR_DS_BIND_ROOT_FOREST	= 0xc00e008fL,
        MQ_ERROR_DS_LOCAL_USER	= 0xc00e0090L,
        MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED	= 0xc00e0091L,
        MQ_ERROR_BAD_XML_FORMAT	= 0xc00e0092L,
        MQ_ERROR_UNSUPPORTED_CLASS	= 0xc00e0093,
        MQ_ERROR_UNINITIALIZED_OBJECT	= 0xc00e0094,
        MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS	= 0xc00e0095,
        MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS	= 0xc00e0096
    } ;
/* [helpstringcontext] */ 
enum MQWARNING
    {
        MQ_INFORMATION_PROPERTY	= 0x400e0001,
        MQ_INFORMATION_ILLEGAL_PROPERTY	= 0x400e0002,
        MQ_INFORMATION_PROPERTY_IGNORED	= 0x400e0003,
        MQ_INFORMATION_UNSUPPORTED_PROPERTY	= 0x400e0004,
        MQ_INFORMATION_DUPLICATE_PROPERTY	= 0x400e0005,
        MQ_INFORMATION_OPERATION_PENDING	= 0x400e0006,
        MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL	= 0x400e0009,
        MQ_INFORMATION_INTERNAL_USER_CERT_EXIST	= 0x400e000aL,
        MQ_INFORMATION_OWNER_IGNORED	= 0x400e000bL
    } ;

EXTERN_C const IID LIBID_MSMQ;

#ifndef __IMSMQQuery_INTERFACE_DEFINED__
#define __IMSMQQuery_INTERFACE_DEFINED__

/* interface IMSMQQuery */
/* [object][nonextensible][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQuery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E072-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQQuery : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE LookupQueue( 
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos **ppqinfos) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQuery * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQuery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQuery * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQuery * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQuery * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQuery * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQuery * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQuery, LookupQueue)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *LookupQueue )( 
            __RPC__in IMSMQQuery * This,
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos **ppqinfos);
        
        END_INTERFACE
    } IMSMQQueryVtbl;

    interface IMSMQQuery
    {
        CONST_VTBL struct IMSMQQueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQuery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQuery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQuery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQuery_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQuery_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQuery_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQuery_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQuery_LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos)	\
    ( (This)->lpVtbl -> LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQuery_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfo_INTERFACE_DEFINED__
#define __IMSMQQueueInfo_INTERFACE_DEFINED__

/* interface IMSMQQueueInfo */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E07B-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQQueueInfo : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ServiceTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ServiceTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidServiceType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PathName( 
            /* [in] */ __RPC__in BSTR bstrPathName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_FormatName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_FormatName( 
            /* [in] */ __RPC__in BSTR bstrFormatName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsTransactional( 
            /* [retval][out] */ __RPC__out Boolean *pisTransactional) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Quota( 
            /* [retval][out] */ __RPC__out long *plQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Quota( 
            /* [in] */ long lQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BasePriority( 
            /* [retval][out] */ __RPC__out long *plBasePriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_BasePriority( 
            /* [in] */ long lBasePriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CreateTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ModifyTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Authenticate( 
            /* [retval][out] */ __RPC__out long *plAuthenticate) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Authenticate( 
            /* [in] */ long lAuthenticate) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_JournalQuota( 
            /* [retval][out] */ __RPC__out long *plJournalQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_JournalQuota( 
            /* [in] */ long lJournalQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsWorldReadable( 
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue **ppq) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Update( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfo * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_QueueGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueGuid )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_ServiceTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_ServiceTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ __RPC__in BSTR bstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_PathName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathName )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_PathName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PathName )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ __RPC__in BSTR bstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_FormatName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_FormatName )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ __RPC__in BSTR bstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_IsTransactional)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsTransactional )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out Boolean *pisTransactional);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_Quota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Quota )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out long *plQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_Quota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Quota )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long lQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_BasePriority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BasePriority )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out long *plBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_BasePriority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_BasePriority )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long lBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_CreateTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CreateTime )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_ModifyTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ModifyTime )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_Authenticate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Authenticate )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out long *plAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_Authenticate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Authenticate )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long lAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_JournalQuota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_JournalQuota )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out long *plJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, put_JournalQuota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_JournalQuota )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long lJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, get_IsWorldReadable)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorldReadable )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, Create)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, Delete)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMSMQQueueInfo * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, Open)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMSMQQueueInfo * This,
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue **ppq);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, Refresh)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IMSMQQueueInfo * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo, Update)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in IMSMQQueueInfo * This);
        
        END_INTERFACE
    } IMSMQQueueInfoVtbl;

    interface IMSMQQueueInfo
    {
        CONST_VTBL struct IMSMQQueueInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfo_get_QueueGuid(This,pbstrGuidQueue)	\
    ( (This)->lpVtbl -> get_QueueGuid(This,pbstrGuidQueue) ) 

#define IMSMQQueueInfo_get_ServiceTypeGuid(This,pbstrGuidServiceType)	\
    ( (This)->lpVtbl -> get_ServiceTypeGuid(This,pbstrGuidServiceType) ) 

#define IMSMQQueueInfo_put_ServiceTypeGuid(This,bstrGuidServiceType)	\
    ( (This)->lpVtbl -> put_ServiceTypeGuid(This,bstrGuidServiceType) ) 

#define IMSMQQueueInfo_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQQueueInfo_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQQueueInfo_get_PathName(This,pbstrPathName)	\
    ( (This)->lpVtbl -> get_PathName(This,pbstrPathName) ) 

#define IMSMQQueueInfo_put_PathName(This,bstrPathName)	\
    ( (This)->lpVtbl -> put_PathName(This,bstrPathName) ) 

#define IMSMQQueueInfo_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQQueueInfo_put_FormatName(This,bstrFormatName)	\
    ( (This)->lpVtbl -> put_FormatName(This,bstrFormatName) ) 

#define IMSMQQueueInfo_get_IsTransactional(This,pisTransactional)	\
    ( (This)->lpVtbl -> get_IsTransactional(This,pisTransactional) ) 

#define IMSMQQueueInfo_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQQueueInfo_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQQueueInfo_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQQueueInfo_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQQueueInfo_get_Quota(This,plQuota)	\
    ( (This)->lpVtbl -> get_Quota(This,plQuota) ) 

#define IMSMQQueueInfo_put_Quota(This,lQuota)	\
    ( (This)->lpVtbl -> put_Quota(This,lQuota) ) 

#define IMSMQQueueInfo_get_BasePriority(This,plBasePriority)	\
    ( (This)->lpVtbl -> get_BasePriority(This,plBasePriority) ) 

#define IMSMQQueueInfo_put_BasePriority(This,lBasePriority)	\
    ( (This)->lpVtbl -> put_BasePriority(This,lBasePriority) ) 

#define IMSMQQueueInfo_get_CreateTime(This,pvarCreateTime)	\
    ( (This)->lpVtbl -> get_CreateTime(This,pvarCreateTime) ) 

#define IMSMQQueueInfo_get_ModifyTime(This,pvarModifyTime)	\
    ( (This)->lpVtbl -> get_ModifyTime(This,pvarModifyTime) ) 

#define IMSMQQueueInfo_get_Authenticate(This,plAuthenticate)	\
    ( (This)->lpVtbl -> get_Authenticate(This,plAuthenticate) ) 

#define IMSMQQueueInfo_put_Authenticate(This,lAuthenticate)	\
    ( (This)->lpVtbl -> put_Authenticate(This,lAuthenticate) ) 

#define IMSMQQueueInfo_get_JournalQuota(This,plJournalQuota)	\
    ( (This)->lpVtbl -> get_JournalQuota(This,plJournalQuota) ) 

#define IMSMQQueueInfo_put_JournalQuota(This,lJournalQuota)	\
    ( (This)->lpVtbl -> put_JournalQuota(This,lJournalQuota) ) 

#define IMSMQQueueInfo_get_IsWorldReadable(This,pisWorldReadable)	\
    ( (This)->lpVtbl -> get_IsWorldReadable(This,pisWorldReadable) ) 

#define IMSMQQueueInfo_Create(This,IsTransactional,IsWorldReadable)	\
    ( (This)->lpVtbl -> Create(This,IsTransactional,IsWorldReadable) ) 

#define IMSMQQueueInfo_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IMSMQQueueInfo_Open(This,Access,ShareMode,ppq)	\
    ( (This)->lpVtbl -> Open(This,Access,ShareMode,ppq) ) 

#define IMSMQQueueInfo_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IMSMQQueueInfo_Update(This)	\
    ( (This)->lpVtbl -> Update(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfo_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfo2_INTERFACE_DEFINED__
#define __IMSMQQueueInfo2_INTERFACE_DEFINED__

/* interface IMSMQQueueInfo2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD174A80-89CF-11D2-B0F2-00E02C074F6B")
    IMSMQQueueInfo2 : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ServiceTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ServiceTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidServiceType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PathName( 
            /* [in] */ __RPC__in BSTR bstrPathName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_FormatName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_FormatName( 
            /* [in] */ __RPC__in BSTR bstrFormatName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsTransactional( 
            /* [retval][out] */ __RPC__out Boolean *pisTransactional) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Quota( 
            /* [retval][out] */ __RPC__out long *plQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Quota( 
            /* [in] */ long lQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BasePriority( 
            /* [retval][out] */ __RPC__out long *plBasePriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_BasePriority( 
            /* [in] */ long lBasePriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CreateTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ModifyTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Authenticate( 
            /* [retval][out] */ __RPC__out long *plAuthenticate) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Authenticate( 
            /* [in] */ long lAuthenticate) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_JournalQuota( 
            /* [retval][out] */ __RPC__out long *plJournalQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_JournalQuota( 
            /* [in] */ long lJournalQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsWorldReadable( 
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue2 **ppq) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Update( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathNameDNS( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathNameDNS) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Security( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSecurity) = 0;
        
        virtual /* [id][propput][hidden] */ HRESULT STDMETHODCALLTYPE put_Security( 
            /* [in] */ VARIANT varSecurity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfo2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfo2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_QueueGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueGuid )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_ServiceTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_ServiceTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ __RPC__in BSTR bstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_PathName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathName )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_PathName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PathName )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ __RPC__in BSTR bstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_FormatName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_FormatName )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ __RPC__in BSTR bstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_IsTransactional)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsTransactional )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out Boolean *pisTransactional);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_Quota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Quota )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out long *plQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_Quota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Quota )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long lQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_BasePriority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BasePriority )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out long *plBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_BasePriority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_BasePriority )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long lBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_CreateTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CreateTime )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_ModifyTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ModifyTime )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_Authenticate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Authenticate )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out long *plAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_Authenticate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Authenticate )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long lAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_JournalQuota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_JournalQuota )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out long *plJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_JournalQuota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_JournalQuota )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long lJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_IsWorldReadable)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorldReadable )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, Create)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, Delete)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMSMQQueueInfo2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, Open)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue2 **ppq);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, Refresh)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IMSMQQueueInfo2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, Update)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in IMSMQQueueInfo2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_PathNameDNS)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathNameDNS )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathNameDNS);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, get_Security)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSecurity);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo2, put_Security)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Security )( 
            __RPC__in IMSMQQueueInfo2 * This,
            /* [in] */ VARIANT varSecurity);
        
        END_INTERFACE
    } IMSMQQueueInfo2Vtbl;

    interface IMSMQQueueInfo2
    {
        CONST_VTBL struct IMSMQQueueInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfo2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfo2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfo2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfo2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfo2_get_QueueGuid(This,pbstrGuidQueue)	\
    ( (This)->lpVtbl -> get_QueueGuid(This,pbstrGuidQueue) ) 

#define IMSMQQueueInfo2_get_ServiceTypeGuid(This,pbstrGuidServiceType)	\
    ( (This)->lpVtbl -> get_ServiceTypeGuid(This,pbstrGuidServiceType) ) 

#define IMSMQQueueInfo2_put_ServiceTypeGuid(This,bstrGuidServiceType)	\
    ( (This)->lpVtbl -> put_ServiceTypeGuid(This,bstrGuidServiceType) ) 

#define IMSMQQueueInfo2_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQQueueInfo2_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQQueueInfo2_get_PathName(This,pbstrPathName)	\
    ( (This)->lpVtbl -> get_PathName(This,pbstrPathName) ) 

#define IMSMQQueueInfo2_put_PathName(This,bstrPathName)	\
    ( (This)->lpVtbl -> put_PathName(This,bstrPathName) ) 

#define IMSMQQueueInfo2_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQQueueInfo2_put_FormatName(This,bstrFormatName)	\
    ( (This)->lpVtbl -> put_FormatName(This,bstrFormatName) ) 

#define IMSMQQueueInfo2_get_IsTransactional(This,pisTransactional)	\
    ( (This)->lpVtbl -> get_IsTransactional(This,pisTransactional) ) 

#define IMSMQQueueInfo2_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQQueueInfo2_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQQueueInfo2_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQQueueInfo2_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQQueueInfo2_get_Quota(This,plQuota)	\
    ( (This)->lpVtbl -> get_Quota(This,plQuota) ) 

#define IMSMQQueueInfo2_put_Quota(This,lQuota)	\
    ( (This)->lpVtbl -> put_Quota(This,lQuota) ) 

#define IMSMQQueueInfo2_get_BasePriority(This,plBasePriority)	\
    ( (This)->lpVtbl -> get_BasePriority(This,plBasePriority) ) 

#define IMSMQQueueInfo2_put_BasePriority(This,lBasePriority)	\
    ( (This)->lpVtbl -> put_BasePriority(This,lBasePriority) ) 

#define IMSMQQueueInfo2_get_CreateTime(This,pvarCreateTime)	\
    ( (This)->lpVtbl -> get_CreateTime(This,pvarCreateTime) ) 

#define IMSMQQueueInfo2_get_ModifyTime(This,pvarModifyTime)	\
    ( (This)->lpVtbl -> get_ModifyTime(This,pvarModifyTime) ) 

#define IMSMQQueueInfo2_get_Authenticate(This,plAuthenticate)	\
    ( (This)->lpVtbl -> get_Authenticate(This,plAuthenticate) ) 

#define IMSMQQueueInfo2_put_Authenticate(This,lAuthenticate)	\
    ( (This)->lpVtbl -> put_Authenticate(This,lAuthenticate) ) 

#define IMSMQQueueInfo2_get_JournalQuota(This,plJournalQuota)	\
    ( (This)->lpVtbl -> get_JournalQuota(This,plJournalQuota) ) 

#define IMSMQQueueInfo2_put_JournalQuota(This,lJournalQuota)	\
    ( (This)->lpVtbl -> put_JournalQuota(This,lJournalQuota) ) 

#define IMSMQQueueInfo2_get_IsWorldReadable(This,pisWorldReadable)	\
    ( (This)->lpVtbl -> get_IsWorldReadable(This,pisWorldReadable) ) 

#define IMSMQQueueInfo2_Create(This,IsTransactional,IsWorldReadable)	\
    ( (This)->lpVtbl -> Create(This,IsTransactional,IsWorldReadable) ) 

#define IMSMQQueueInfo2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IMSMQQueueInfo2_Open(This,Access,ShareMode,ppq)	\
    ( (This)->lpVtbl -> Open(This,Access,ShareMode,ppq) ) 

#define IMSMQQueueInfo2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IMSMQQueueInfo2_Update(This)	\
    ( (This)->lpVtbl -> Update(This) ) 

#define IMSMQQueueInfo2_get_PathNameDNS(This,pbstrPathNameDNS)	\
    ( (This)->lpVtbl -> get_PathNameDNS(This,pbstrPathNameDNS) ) 

#define IMSMQQueueInfo2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQueueInfo2_get_Security(This,pvarSecurity)	\
    ( (This)->lpVtbl -> get_Security(This,pvarSecurity) ) 

#define IMSMQQueueInfo2_put_Security(This,varSecurity)	\
    ( (This)->lpVtbl -> put_Security(This,varSecurity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfo2_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfo3_INTERFACE_DEFINED__
#define __IMSMQQueueInfo3_INTERFACE_DEFINED__

/* interface IMSMQQueueInfo3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfo3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b1d-2168-11d3-898c-00e02c074f6b")
    IMSMQQueueInfo3 : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ServiceTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ServiceTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidServiceType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PathName( 
            /* [in] */ __RPC__in BSTR bstrPathName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_FormatName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_FormatName( 
            /* [in] */ __RPC__in BSTR bstrFormatName) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsTransactional( 
            /* [retval][out] */ __RPC__out Boolean *pisTransactional) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Quota( 
            /* [retval][out] */ __RPC__out long *plQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Quota( 
            /* [in] */ long lQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BasePriority( 
            /* [retval][out] */ __RPC__out long *plBasePriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_BasePriority( 
            /* [in] */ long lBasePriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CreateTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ModifyTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Authenticate( 
            /* [retval][out] */ __RPC__out long *plAuthenticate) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Authenticate( 
            /* [in] */ long lAuthenticate) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_JournalQuota( 
            /* [retval][out] */ __RPC__out long *plJournalQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_JournalQuota( 
            /* [in] */ long lJournalQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsWorldReadable( 
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue3 **ppq) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Update( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathNameDNS( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathNameDNS) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Security( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSecurity) = 0;
        
        virtual /* [id][propput][hidden] */ HRESULT STDMETHODCALLTYPE put_Security( 
            /* [in] */ VARIANT varSecurity) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsTransactional2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisTransactional) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsWorldReadable2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisWorldReadable) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MulticastAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMulticastAddress) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MulticastAddress( 
            /* [in] */ __RPC__in BSTR bstrMulticastAddress) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ADsPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfo3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfo3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfo3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfo3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_QueueGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueGuid )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_ServiceTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_ServiceTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in BSTR bstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_PathName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathName )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_PathName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PathName )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in BSTR bstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_FormatName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_FormatName )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in BSTR bstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_IsTransactional)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsTransactional )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out Boolean *pisTransactional);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_Quota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Quota )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out long *plQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_Quota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Quota )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long lQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_BasePriority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BasePriority )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out long *plBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_BasePriority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_BasePriority )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long lBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_CreateTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CreateTime )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_ModifyTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ModifyTime )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_Authenticate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Authenticate )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out long *plAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_Authenticate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Authenticate )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long lAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_JournalQuota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_JournalQuota )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out long *plJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_JournalQuota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_JournalQuota )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long lJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_IsWorldReadable)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorldReadable )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, Create)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, Delete)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMSMQQueueInfo3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, Open)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue3 **ppq);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, Refresh)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IMSMQQueueInfo3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, Update)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in IMSMQQueueInfo3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_PathNameDNS)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathNameDNS )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathNameDNS);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_Security)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSecurity);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_Security)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Security )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ VARIANT varSecurity);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_IsTransactional2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsTransactional2 )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisTransactional);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_IsWorldReadable2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorldReadable2 )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_MulticastAddress)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MulticastAddress )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMulticastAddress);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, put_MulticastAddress)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MulticastAddress )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [in] */ __RPC__in BSTR bstrMulticastAddress);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo3, get_ADsPath)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IMSMQQueueInfo3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath);
        
        END_INTERFACE
    } IMSMQQueueInfo3Vtbl;

    interface IMSMQQueueInfo3
    {
        CONST_VTBL struct IMSMQQueueInfo3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfo3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfo3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfo3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfo3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfo3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfo3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfo3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfo3_get_QueueGuid(This,pbstrGuidQueue)	\
    ( (This)->lpVtbl -> get_QueueGuid(This,pbstrGuidQueue) ) 

#define IMSMQQueueInfo3_get_ServiceTypeGuid(This,pbstrGuidServiceType)	\
    ( (This)->lpVtbl -> get_ServiceTypeGuid(This,pbstrGuidServiceType) ) 

#define IMSMQQueueInfo3_put_ServiceTypeGuid(This,bstrGuidServiceType)	\
    ( (This)->lpVtbl -> put_ServiceTypeGuid(This,bstrGuidServiceType) ) 

#define IMSMQQueueInfo3_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQQueueInfo3_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQQueueInfo3_get_PathName(This,pbstrPathName)	\
    ( (This)->lpVtbl -> get_PathName(This,pbstrPathName) ) 

#define IMSMQQueueInfo3_put_PathName(This,bstrPathName)	\
    ( (This)->lpVtbl -> put_PathName(This,bstrPathName) ) 

#define IMSMQQueueInfo3_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQQueueInfo3_put_FormatName(This,bstrFormatName)	\
    ( (This)->lpVtbl -> put_FormatName(This,bstrFormatName) ) 

#define IMSMQQueueInfo3_get_IsTransactional(This,pisTransactional)	\
    ( (This)->lpVtbl -> get_IsTransactional(This,pisTransactional) ) 

#define IMSMQQueueInfo3_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQQueueInfo3_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQQueueInfo3_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQQueueInfo3_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQQueueInfo3_get_Quota(This,plQuota)	\
    ( (This)->lpVtbl -> get_Quota(This,plQuota) ) 

#define IMSMQQueueInfo3_put_Quota(This,lQuota)	\
    ( (This)->lpVtbl -> put_Quota(This,lQuota) ) 

#define IMSMQQueueInfo3_get_BasePriority(This,plBasePriority)	\
    ( (This)->lpVtbl -> get_BasePriority(This,plBasePriority) ) 

#define IMSMQQueueInfo3_put_BasePriority(This,lBasePriority)	\
    ( (This)->lpVtbl -> put_BasePriority(This,lBasePriority) ) 

#define IMSMQQueueInfo3_get_CreateTime(This,pvarCreateTime)	\
    ( (This)->lpVtbl -> get_CreateTime(This,pvarCreateTime) ) 

#define IMSMQQueueInfo3_get_ModifyTime(This,pvarModifyTime)	\
    ( (This)->lpVtbl -> get_ModifyTime(This,pvarModifyTime) ) 

#define IMSMQQueueInfo3_get_Authenticate(This,plAuthenticate)	\
    ( (This)->lpVtbl -> get_Authenticate(This,plAuthenticate) ) 

#define IMSMQQueueInfo3_put_Authenticate(This,lAuthenticate)	\
    ( (This)->lpVtbl -> put_Authenticate(This,lAuthenticate) ) 

#define IMSMQQueueInfo3_get_JournalQuota(This,plJournalQuota)	\
    ( (This)->lpVtbl -> get_JournalQuota(This,plJournalQuota) ) 

#define IMSMQQueueInfo3_put_JournalQuota(This,lJournalQuota)	\
    ( (This)->lpVtbl -> put_JournalQuota(This,lJournalQuota) ) 

#define IMSMQQueueInfo3_get_IsWorldReadable(This,pisWorldReadable)	\
    ( (This)->lpVtbl -> get_IsWorldReadable(This,pisWorldReadable) ) 

#define IMSMQQueueInfo3_Create(This,IsTransactional,IsWorldReadable)	\
    ( (This)->lpVtbl -> Create(This,IsTransactional,IsWorldReadable) ) 

#define IMSMQQueueInfo3_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IMSMQQueueInfo3_Open(This,Access,ShareMode,ppq)	\
    ( (This)->lpVtbl -> Open(This,Access,ShareMode,ppq) ) 

#define IMSMQQueueInfo3_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IMSMQQueueInfo3_Update(This)	\
    ( (This)->lpVtbl -> Update(This) ) 

#define IMSMQQueueInfo3_get_PathNameDNS(This,pbstrPathNameDNS)	\
    ( (This)->lpVtbl -> get_PathNameDNS(This,pbstrPathNameDNS) ) 

#define IMSMQQueueInfo3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQueueInfo3_get_Security(This,pvarSecurity)	\
    ( (This)->lpVtbl -> get_Security(This,pvarSecurity) ) 

#define IMSMQQueueInfo3_put_Security(This,varSecurity)	\
    ( (This)->lpVtbl -> put_Security(This,varSecurity) ) 

#define IMSMQQueueInfo3_get_IsTransactional2(This,pisTransactional)	\
    ( (This)->lpVtbl -> get_IsTransactional2(This,pisTransactional) ) 

#define IMSMQQueueInfo3_get_IsWorldReadable2(This,pisWorldReadable)	\
    ( (This)->lpVtbl -> get_IsWorldReadable2(This,pisWorldReadable) ) 

#define IMSMQQueueInfo3_get_MulticastAddress(This,pbstrMulticastAddress)	\
    ( (This)->lpVtbl -> get_MulticastAddress(This,pbstrMulticastAddress) ) 

#define IMSMQQueueInfo3_put_MulticastAddress(This,bstrMulticastAddress)	\
    ( (This)->lpVtbl -> put_MulticastAddress(This,bstrMulticastAddress) ) 

#define IMSMQQueueInfo3_get_ADsPath(This,pbstrADsPath)	\
    ( (This)->lpVtbl -> get_ADsPath(This,pbstrADsPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfo3_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfo4_INTERFACE_DEFINED__
#define __IMSMQQueueInfo4_INTERFACE_DEFINED__

/* interface IMSMQQueueInfo4 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfo4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b21-2168-11d3-898c-00e02c074f6b")
    IMSMQQueueInfo4 : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ServiceTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ServiceTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidServiceType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PathName( 
            /* [in] */ __RPC__in BSTR bstrPathName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_FormatName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_FormatName( 
            /* [in] */ __RPC__in BSTR bstrFormatName) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsTransactional( 
            /* [retval][out] */ __RPC__out Boolean *pisTransactional) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Quota( 
            /* [retval][out] */ __RPC__out long *plQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Quota( 
            /* [in] */ long lQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BasePriority( 
            /* [retval][out] */ __RPC__out long *plBasePriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_BasePriority( 
            /* [in] */ long lBasePriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CreateTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ModifyTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Authenticate( 
            /* [retval][out] */ __RPC__out long *plAuthenticate) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Authenticate( 
            /* [in] */ long lAuthenticate) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_JournalQuota( 
            /* [retval][out] */ __RPC__out long *plJournalQuota) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_JournalQuota( 
            /* [in] */ long lJournalQuota) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsWorldReadable( 
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue4 **ppq) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Update( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathNameDNS( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathNameDNS) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Security( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSecurity) = 0;
        
        virtual /* [id][propput][hidden] */ HRESULT STDMETHODCALLTYPE put_Security( 
            /* [in] */ VARIANT varSecurity) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsTransactional2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisTransactional) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsWorldReadable2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisWorldReadable) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MulticastAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMulticastAddress) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MulticastAddress( 
            /* [in] */ __RPC__in BSTR bstrMulticastAddress) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ADsPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfo4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfo4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfo4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfo4 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_QueueGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueGuid )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidQueue);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_ServiceTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_ServiceTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceTypeGuid )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in BSTR bstrGuidServiceType);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_PathName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathName )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_PathName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PathName )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in BSTR bstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_FormatName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_FormatName )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in BSTR bstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_IsTransactional)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsTransactional )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out Boolean *pisTransactional);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_Quota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Quota )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out long *plQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_Quota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Quota )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long lQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_BasePriority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BasePriority )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out long *plBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_BasePriority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_BasePriority )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long lBasePriority);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_CreateTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CreateTime )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCreateTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_ModifyTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ModifyTime )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModifyTime);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_Authenticate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Authenticate )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out long *plAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_Authenticate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Authenticate )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long lAuthenticate);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_JournalQuota)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_JournalQuota )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out long *plJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_JournalQuota)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_JournalQuota )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long lJournalQuota);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_IsWorldReadable)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorldReadable )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out Boolean *pisWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, Create)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [optional][in] */ __RPC__in VARIANT *IsTransactional,
            /* [optional][in] */ __RPC__in VARIANT *IsWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, Delete)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMSMQQueueInfo4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, Open)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ long Access,
            /* [in] */ long ShareMode,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueue4 **ppq);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, Refresh)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IMSMQQueueInfo4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, Update)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in IMSMQQueueInfo4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_PathNameDNS)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathNameDNS )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathNameDNS);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_Security)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSecurity);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_Security)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Security )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ VARIANT varSecurity);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_IsTransactional2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsTransactional2 )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisTransactional);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_IsWorldReadable2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorldReadable2 )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisWorldReadable);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_MulticastAddress)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MulticastAddress )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMulticastAddress);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, put_MulticastAddress)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MulticastAddress )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [in] */ __RPC__in BSTR bstrMulticastAddress);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfo4, get_ADsPath)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IMSMQQueueInfo4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath);
        
        END_INTERFACE
    } IMSMQQueueInfo4Vtbl;

    interface IMSMQQueueInfo4
    {
        CONST_VTBL struct IMSMQQueueInfo4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfo4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfo4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfo4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfo4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfo4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfo4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfo4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfo4_get_QueueGuid(This,pbstrGuidQueue)	\
    ( (This)->lpVtbl -> get_QueueGuid(This,pbstrGuidQueue) ) 

#define IMSMQQueueInfo4_get_ServiceTypeGuid(This,pbstrGuidServiceType)	\
    ( (This)->lpVtbl -> get_ServiceTypeGuid(This,pbstrGuidServiceType) ) 

#define IMSMQQueueInfo4_put_ServiceTypeGuid(This,bstrGuidServiceType)	\
    ( (This)->lpVtbl -> put_ServiceTypeGuid(This,bstrGuidServiceType) ) 

#define IMSMQQueueInfo4_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQQueueInfo4_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQQueueInfo4_get_PathName(This,pbstrPathName)	\
    ( (This)->lpVtbl -> get_PathName(This,pbstrPathName) ) 

#define IMSMQQueueInfo4_put_PathName(This,bstrPathName)	\
    ( (This)->lpVtbl -> put_PathName(This,bstrPathName) ) 

#define IMSMQQueueInfo4_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQQueueInfo4_put_FormatName(This,bstrFormatName)	\
    ( (This)->lpVtbl -> put_FormatName(This,bstrFormatName) ) 

#define IMSMQQueueInfo4_get_IsTransactional(This,pisTransactional)	\
    ( (This)->lpVtbl -> get_IsTransactional(This,pisTransactional) ) 

#define IMSMQQueueInfo4_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQQueueInfo4_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQQueueInfo4_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQQueueInfo4_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQQueueInfo4_get_Quota(This,plQuota)	\
    ( (This)->lpVtbl -> get_Quota(This,plQuota) ) 

#define IMSMQQueueInfo4_put_Quota(This,lQuota)	\
    ( (This)->lpVtbl -> put_Quota(This,lQuota) ) 

#define IMSMQQueueInfo4_get_BasePriority(This,plBasePriority)	\
    ( (This)->lpVtbl -> get_BasePriority(This,plBasePriority) ) 

#define IMSMQQueueInfo4_put_BasePriority(This,lBasePriority)	\
    ( (This)->lpVtbl -> put_BasePriority(This,lBasePriority) ) 

#define IMSMQQueueInfo4_get_CreateTime(This,pvarCreateTime)	\
    ( (This)->lpVtbl -> get_CreateTime(This,pvarCreateTime) ) 

#define IMSMQQueueInfo4_get_ModifyTime(This,pvarModifyTime)	\
    ( (This)->lpVtbl -> get_ModifyTime(This,pvarModifyTime) ) 

#define IMSMQQueueInfo4_get_Authenticate(This,plAuthenticate)	\
    ( (This)->lpVtbl -> get_Authenticate(This,plAuthenticate) ) 

#define IMSMQQueueInfo4_put_Authenticate(This,lAuthenticate)	\
    ( (This)->lpVtbl -> put_Authenticate(This,lAuthenticate) ) 

#define IMSMQQueueInfo4_get_JournalQuota(This,plJournalQuota)	\
    ( (This)->lpVtbl -> get_JournalQuota(This,plJournalQuota) ) 

#define IMSMQQueueInfo4_put_JournalQuota(This,lJournalQuota)	\
    ( (This)->lpVtbl -> put_JournalQuota(This,lJournalQuota) ) 

#define IMSMQQueueInfo4_get_IsWorldReadable(This,pisWorldReadable)	\
    ( (This)->lpVtbl -> get_IsWorldReadable(This,pisWorldReadable) ) 

#define IMSMQQueueInfo4_Create(This,IsTransactional,IsWorldReadable)	\
    ( (This)->lpVtbl -> Create(This,IsTransactional,IsWorldReadable) ) 

#define IMSMQQueueInfo4_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IMSMQQueueInfo4_Open(This,Access,ShareMode,ppq)	\
    ( (This)->lpVtbl -> Open(This,Access,ShareMode,ppq) ) 

#define IMSMQQueueInfo4_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IMSMQQueueInfo4_Update(This)	\
    ( (This)->lpVtbl -> Update(This) ) 

#define IMSMQQueueInfo4_get_PathNameDNS(This,pbstrPathNameDNS)	\
    ( (This)->lpVtbl -> get_PathNameDNS(This,pbstrPathNameDNS) ) 

#define IMSMQQueueInfo4_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQueueInfo4_get_Security(This,pvarSecurity)	\
    ( (This)->lpVtbl -> get_Security(This,pvarSecurity) ) 

#define IMSMQQueueInfo4_put_Security(This,varSecurity)	\
    ( (This)->lpVtbl -> put_Security(This,varSecurity) ) 

#define IMSMQQueueInfo4_get_IsTransactional2(This,pisTransactional)	\
    ( (This)->lpVtbl -> get_IsTransactional2(This,pisTransactional) ) 

#define IMSMQQueueInfo4_get_IsWorldReadable2(This,pisWorldReadable)	\
    ( (This)->lpVtbl -> get_IsWorldReadable2(This,pisWorldReadable) ) 

#define IMSMQQueueInfo4_get_MulticastAddress(This,pbstrMulticastAddress)	\
    ( (This)->lpVtbl -> get_MulticastAddress(This,pbstrMulticastAddress) ) 

#define IMSMQQueueInfo4_put_MulticastAddress(This,bstrMulticastAddress)	\
    ( (This)->lpVtbl -> put_MulticastAddress(This,bstrMulticastAddress) ) 

#define IMSMQQueueInfo4_get_ADsPath(This,pbstrADsPath)	\
    ( (This)->lpVtbl -> get_ADsPath(This,pbstrADsPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfo4_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueue_INTERFACE_DEFINED__
#define __IMSMQQueue_INTERFACE_DEFINED__

/* interface IMSMQQueue */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E076-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQQueue : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Access( 
            /* [retval][out] */ __RPC__out long *plAccess) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ShareMode( 
            /* [retval][out] */ __RPC__out long *plShareMode) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfo) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out long *plHandle) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsOpen( 
            /* [retval][out] */ __RPC__out Boolean *pisOpen) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EnableNotification( 
            /* [in] */ __RPC__in_opt IMSMQEvent *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueue * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueue, get_Access)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Access )( 
            __RPC__in IMSMQQueue * This,
            /* [retval][out] */ __RPC__out long *plAccess);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, get_ShareMode)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ShareMode )( 
            __RPC__in IMSMQQueue * This,
            /* [retval][out] */ __RPC__out long *plShareMode);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, get_QueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueInfo )( 
            __RPC__in IMSMQQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfo);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, get_Handle)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in IMSMQQueue * This,
            /* [retval][out] */ __RPC__out long *plHandle);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, get_IsOpen)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen )( 
            __RPC__in IMSMQQueue * This,
            /* [retval][out] */ __RPC__out Boolean *pisOpen);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, Close)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMSMQQueue * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, Receive)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive )( 
            __RPC__in IMSMQQueue * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, Peek)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek )( 
            __RPC__in IMSMQQueue * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, EnableNotification)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EnableNotification )( 
            __RPC__in IMSMQQueue * This,
            /* [in] */ __RPC__in_opt IMSMQEvent *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueue * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, ReceiveCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent )( 
            __RPC__in IMSMQQueue * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, PeekNext)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext )( 
            __RPC__in IMSMQQueue * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue, PeekCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent )( 
            __RPC__in IMSMQQueue * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        END_INTERFACE
    } IMSMQQueueVtbl;

    interface IMSMQQueue
    {
        CONST_VTBL struct IMSMQQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueue_get_Access(This,plAccess)	\
    ( (This)->lpVtbl -> get_Access(This,plAccess) ) 

#define IMSMQQueue_get_ShareMode(This,plShareMode)	\
    ( (This)->lpVtbl -> get_ShareMode(This,plShareMode) ) 

#define IMSMQQueue_get_QueueInfo(This,ppqinfo)	\
    ( (This)->lpVtbl -> get_QueueInfo(This,ppqinfo) ) 

#define IMSMQQueue_get_Handle(This,plHandle)	\
    ( (This)->lpVtbl -> get_Handle(This,plHandle) ) 

#define IMSMQQueue_get_IsOpen(This,pisOpen)	\
    ( (This)->lpVtbl -> get_IsOpen(This,pisOpen) ) 

#define IMSMQQueue_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMSMQQueue_Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue_Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue_EnableNotification(This,Event,Cursor,ReceiveTimeout)	\
    ( (This)->lpVtbl -> EnableNotification(This,Event,Cursor,ReceiveTimeout) ) 

#define IMSMQQueue_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueue_ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue_PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue_PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueue_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueue2_INTERFACE_DEFINED__
#define __IMSMQQueue2_INTERFACE_DEFINED__

/* interface IMSMQQueue2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueue2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EF0574E0-06D8-11D3-B100-00E02C074F6B")
    IMSMQQueue2 : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Access( 
            /* [retval][out] */ __RPC__out long *plAccess) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ShareMode( 
            /* [retval][out] */ __RPC__out long *plShareMode) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfo) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out long *plHandle) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsOpen( 
            /* [retval][out] */ __RPC__out Boolean *pisOpen) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive_v1( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EnableNotification( 
            /* [in] */ __RPC__in_opt IMSMQEvent2 *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent_v1( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueue2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueue2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueue2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueue2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueue2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueue2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueue2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueue2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, get_Access)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Access )( 
            __RPC__in IMSMQQueue2 * This,
            /* [retval][out] */ __RPC__out long *plAccess);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, get_ShareMode)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ShareMode )( 
            __RPC__in IMSMQQueue2 * This,
            /* [retval][out] */ __RPC__out long *plShareMode);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, get_QueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueInfo )( 
            __RPC__in IMSMQQueue2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfo);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, get_Handle)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in IMSMQQueue2 * This,
            /* [retval][out] */ __RPC__out long *plHandle);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, get_IsOpen)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen )( 
            __RPC__in IMSMQQueue2 * This,
            /* [retval][out] */ __RPC__out Boolean *pisOpen);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, Close)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMSMQQueue2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, Receive_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive_v1 )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, Peek_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek_v1 )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, EnableNotification)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EnableNotification )( 
            __RPC__in IMSMQQueue2 * This,
            /* [in] */ __RPC__in_opt IMSMQEvent2 *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueue2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, ReceiveCurrent_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent_v1 )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, PeekNext_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext_v1 )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, PeekCurrent_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent_v1 )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, Receive)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, Peek)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, ReceiveCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, PeekNext)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, PeekCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent )( 
            __RPC__in IMSMQQueue2 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage2 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueue2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQQueue2Vtbl;

    interface IMSMQQueue2
    {
        CONST_VTBL struct IMSMQQueue2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueue2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueue2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueue2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueue2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueue2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueue2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueue2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueue2_get_Access(This,plAccess)	\
    ( (This)->lpVtbl -> get_Access(This,plAccess) ) 

#define IMSMQQueue2_get_ShareMode(This,plShareMode)	\
    ( (This)->lpVtbl -> get_ShareMode(This,plShareMode) ) 

#define IMSMQQueue2_get_QueueInfo(This,ppqinfo)	\
    ( (This)->lpVtbl -> get_QueueInfo(This,ppqinfo) ) 

#define IMSMQQueue2_get_Handle(This,plHandle)	\
    ( (This)->lpVtbl -> get_Handle(This,plHandle) ) 

#define IMSMQQueue2_get_IsOpen(This,pisOpen)	\
    ( (This)->lpVtbl -> get_IsOpen(This,pisOpen) ) 

#define IMSMQQueue2_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMSMQQueue2_Receive_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Receive_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue2_Peek_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Peek_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue2_EnableNotification(This,Event,Cursor,ReceiveTimeout)	\
    ( (This)->lpVtbl -> EnableNotification(This,Event,Cursor,ReceiveTimeout) ) 

#define IMSMQQueue2_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueue2_ReceiveCurrent_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue2_PeekNext_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue2_PeekCurrent_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue2_Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue2_Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue2_ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue2_PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue2_PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueue2_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueue3_INTERFACE_DEFINED__
#define __IMSMQQueue3_INTERFACE_DEFINED__

/* interface IMSMQQueue3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueue3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b1b-2168-11d3-898c-00e02c074f6b")
    IMSMQQueue3 : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Access( 
            /* [retval][out] */ __RPC__out long *plAccess) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ShareMode( 
            /* [retval][out] */ __RPC__out long *plShareMode) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfo) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out long *plHandle) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsOpen( 
            /* [retval][out] */ __RPC__out Boolean *pisOpen) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive_v1( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EnableNotification( 
            /* [in] */ __RPC__in_opt IMSMQEvent3 *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent_v1( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_Handle2( 
            /* [retval][out] */ __RPC__out VARIANT *pvarHandle) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveNextByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceivePreviousByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveFirstByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveLastByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNextByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekPreviousByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekFirstByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekLastByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Purge( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsOpen2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisOpen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueue3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueue3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueue3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueue3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueue3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_Access)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Access )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__out long *plAccess);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_ShareMode)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ShareMode )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__out long *plShareMode);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_QueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueInfo )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfo);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_Handle)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__out long *plHandle);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_IsOpen)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__out Boolean *pisOpen);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Close)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMSMQQueue3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Receive_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive_v1 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Peek_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek_v1 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, EnableNotification)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EnableNotification )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ __RPC__in_opt IMSMQEvent3 *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueue3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceiveCurrent_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent_v1 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekNext_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext_v1 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekCurrent_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent_v1 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Receive)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Peek)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceiveCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekNext)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_Handle2)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Handle2 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarHandle);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceiveByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceiveNextByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveNextByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceivePreviousByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceivePreviousByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceiveFirstByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveFirstByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, ReceiveLastByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveLastByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekNextByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNextByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekPreviousByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekPreviousByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekFirstByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekFirstByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, PeekLastByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekLastByLookupId )( 
            __RPC__in IMSMQQueue3 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage3 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, Purge)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Purge )( 
            __RPC__in IMSMQQueue3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue3, get_IsOpen2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen2 )( 
            __RPC__in IMSMQQueue3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisOpen);
        
        END_INTERFACE
    } IMSMQQueue3Vtbl;

    interface IMSMQQueue3
    {
        CONST_VTBL struct IMSMQQueue3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueue3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueue3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueue3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueue3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueue3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueue3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueue3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueue3_get_Access(This,plAccess)	\
    ( (This)->lpVtbl -> get_Access(This,plAccess) ) 

#define IMSMQQueue3_get_ShareMode(This,plShareMode)	\
    ( (This)->lpVtbl -> get_ShareMode(This,plShareMode) ) 

#define IMSMQQueue3_get_QueueInfo(This,ppqinfo)	\
    ( (This)->lpVtbl -> get_QueueInfo(This,ppqinfo) ) 

#define IMSMQQueue3_get_Handle(This,plHandle)	\
    ( (This)->lpVtbl -> get_Handle(This,plHandle) ) 

#define IMSMQQueue3_get_IsOpen(This,pisOpen)	\
    ( (This)->lpVtbl -> get_IsOpen(This,pisOpen) ) 

#define IMSMQQueue3_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMSMQQueue3_Receive_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Receive_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue3_Peek_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Peek_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue3_EnableNotification(This,Event,Cursor,ReceiveTimeout)	\
    ( (This)->lpVtbl -> EnableNotification(This,Event,Cursor,ReceiveTimeout) ) 

#define IMSMQQueue3_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueue3_ReceiveCurrent_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue3_PeekNext_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue3_PeekCurrent_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue3_Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQueue3_get_Handle2(This,pvarHandle)	\
    ( (This)->lpVtbl -> get_Handle2(This,pvarHandle) ) 

#define IMSMQQueue3_ReceiveByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_ReceiveNextByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveNextByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_ReceivePreviousByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceivePreviousByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_ReceiveFirstByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveFirstByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_ReceiveLastByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveLastByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekNextByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekNextByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekPreviousByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekPreviousByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekFirstByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekFirstByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_PeekLastByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekLastByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue3_Purge(This)	\
    ( (This)->lpVtbl -> Purge(This) ) 

#define IMSMQQueue3_get_IsOpen2(This,pisOpen)	\
    ( (This)->lpVtbl -> get_IsOpen2(This,pisOpen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueue3_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueue4_INTERFACE_DEFINED__
#define __IMSMQQueue4_INTERFACE_DEFINED__

/* interface IMSMQQueue4 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueue4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b20-2168-11d3-898c-00e02c074f6b")
    IMSMQQueue4 : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Access( 
            /* [retval][out] */ __RPC__out long *plAccess) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ShareMode( 
            /* [retval][out] */ __RPC__out long *plShareMode) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfo) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out long *plHandle) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsOpen( 
            /* [retval][out] */ __RPC__out Boolean *pisOpen) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive_v1( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EnableNotification( 
            /* [in] */ __RPC__in_opt IMSMQEvent3 *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent_v1( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent_v1( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Receive( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Peek( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNext( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekCurrent( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_Handle2( 
            /* [retval][out] */ __RPC__out VARIANT *pvarHandle) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveNextByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceivePreviousByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveFirstByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveLastByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekNextByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekPreviousByLookupId( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekFirstByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE PeekLastByLookupId( 
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Purge( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsOpen2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisOpen) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE ReceiveByLookupIdAllowPeek( 
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueue4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueue4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueue4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueue4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueue4 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_Access)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Access )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__out long *plAccess);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_ShareMode)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ShareMode )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__out long *plShareMode);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_QueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueInfo )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfo);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_Handle)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__out long *plHandle);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_IsOpen)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__out Boolean *pisOpen);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Close)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMSMQQueue4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Receive_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive_v1 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Peek_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek_v1 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, EnableNotification)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EnableNotification )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ __RPC__in_opt IMSMQEvent3 *Event,
            /* [optional][in] */ __RPC__in VARIANT *Cursor,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueue4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveCurrent_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent_v1 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekNext_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext_v1 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekCurrent_v1)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent_v1 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Receive)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Receive )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Peek)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Peek )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveCurrent )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekNext)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNext )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekCurrent)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekCurrent )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *ReceiveTimeout,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_Handle2)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Handle2 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarHandle);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveNextByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveNextByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceivePreviousByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceivePreviousByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveFirstByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveFirstByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveLastByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveLastByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekNextByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekNextByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekPreviousByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekPreviousByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekFirstByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekFirstByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, PeekLastByLookupId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *PeekLastByLookupId )( 
            __RPC__in IMSMQQueue4 * This,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, Purge)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Purge )( 
            __RPC__in IMSMQQueue4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, get_IsOpen2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen2 )( 
            __RPC__in IMSMQQueue4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisOpen);
        
        DECLSPEC_XFGVIRT(IMSMQQueue4, ReceiveByLookupIdAllowPeek)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *ReceiveByLookupIdAllowPeek )( 
            __RPC__in IMSMQQueue4 * This,
            /* [in] */ VARIANT LookupId,
            /* [optional][in] */ __RPC__in VARIANT *Transaction,
            /* [optional][in] */ __RPC__in VARIANT *WantDestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *WantBody,
            /* [optional][in] */ __RPC__in VARIANT *WantConnectorType,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQMessage4 **ppmsg);
        
        END_INTERFACE
    } IMSMQQueue4Vtbl;

    interface IMSMQQueue4
    {
        CONST_VTBL struct IMSMQQueue4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueue4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueue4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueue4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueue4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueue4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueue4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueue4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueue4_get_Access(This,plAccess)	\
    ( (This)->lpVtbl -> get_Access(This,plAccess) ) 

#define IMSMQQueue4_get_ShareMode(This,plShareMode)	\
    ( (This)->lpVtbl -> get_ShareMode(This,plShareMode) ) 

#define IMSMQQueue4_get_QueueInfo(This,ppqinfo)	\
    ( (This)->lpVtbl -> get_QueueInfo(This,ppqinfo) ) 

#define IMSMQQueue4_get_Handle(This,plHandle)	\
    ( (This)->lpVtbl -> get_Handle(This,plHandle) ) 

#define IMSMQQueue4_get_IsOpen(This,pisOpen)	\
    ( (This)->lpVtbl -> get_IsOpen(This,pisOpen) ) 

#define IMSMQQueue4_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMSMQQueue4_Receive_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Receive_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue4_Peek_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> Peek_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue4_EnableNotification(This,Event,Cursor,ReceiveTimeout)	\
    ( (This)->lpVtbl -> EnableNotification(This,Event,Cursor,ReceiveTimeout) ) 

#define IMSMQQueue4_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueue4_ReceiveCurrent_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent_v1(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue4_PeekNext_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue4_PeekCurrent_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent_v1(This,WantDestinationQueue,WantBody,ReceiveTimeout,ppmsg) ) 

#define IMSMQQueue4_Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> Receive(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> Peek(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveCurrent(This,Transaction,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekNext(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekCurrent(This,WantDestinationQueue,WantBody,ReceiveTimeout,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQueue4_get_Handle2(This,pvarHandle)	\
    ( (This)->lpVtbl -> get_Handle2(This,pvarHandle) ) 

#define IMSMQQueue4_ReceiveByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_ReceiveNextByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveNextByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_ReceivePreviousByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceivePreviousByLookupId(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_ReceiveFirstByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveFirstByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_ReceiveLastByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveLastByLookupId(This,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekNextByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekNextByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekPreviousByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekPreviousByLookupId(This,LookupId,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekFirstByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekFirstByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_PeekLastByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> PeekLastByLookupId(This,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#define IMSMQQueue4_Purge(This)	\
    ( (This)->lpVtbl -> Purge(This) ) 

#define IMSMQQueue4_get_IsOpen2(This,pisOpen)	\
    ( (This)->lpVtbl -> get_IsOpen2(This,pisOpen) ) 

#define IMSMQQueue4_ReceiveByLookupIdAllowPeek(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg)	\
    ( (This)->lpVtbl -> ReceiveByLookupIdAllowPeek(This,LookupId,Transaction,WantDestinationQueue,WantBody,WantConnectorType,ppmsg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueue4_INTERFACE_DEFINED__ */


#ifndef __IMSMQMessage_INTERFACE_DEFINED__
#define __IMSMQMessage_INTERFACE_DEFINED__

/* interface IMSMQMessage */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQMessage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E074-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQMessage : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Class( 
            /* [retval][out] */ __RPC__out long *plClass) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthLevel( 
            /* [retval][out] */ __RPC__out long *plAuthLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthLevel( 
            /* [in] */ long lAuthLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsAuthenticated( 
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Delivery( 
            /* [retval][out] */ __RPC__out long *plDelivery) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Delivery( 
            /* [in] */ long lDelivery) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Trace( 
            /* [retval][out] */ __RPC__out long *plTrace) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Trace( 
            /* [in] */ long lTrace) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *plPriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lPriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AppSpecific( 
            /* [retval][out] */ __RPC__out long *plAppSpecific) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AppSpecific( 
            /* [in] */ long lAppSpecific) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SourceMachineGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BodyLength( 
            /* [retval][out] */ __RPC__out long *pcbBody) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ __RPC__out VARIANT *pvarBody) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Body( 
            /* [in] */ VARIANT varBody) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CorrelationId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_CorrelationId( 
            /* [in] */ VARIANT varMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Ack( 
            /* [retval][out] */ __RPC__out long *plAck) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Ack( 
            /* [in] */ long lAck) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReachQueue( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReachQueue( 
            /* [in] */ long lMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReceive( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReceive( 
            /* [in] */ long lMaxTimeToReceive) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__out long *plHashAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ long lHashAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_EncryptAlgorithm( 
            /* [retval][out] */ __RPC__out long *plEncryptAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_EncryptAlgorithm( 
            /* [in] */ long lEncryptAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SentTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ArrivedTime( 
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoDest) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderCertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderCertificate( 
            /* [in] */ VARIANT varSenderCert) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderIdType( 
            /* [retval][out] */ __RPC__out long *plSenderIdType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderIdType( 
            /* [in] */ long lSenderIdType) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Send( 
            /* [in] */ __RPC__in_opt IMSMQQueue *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE AttachCurrentSecurityContext( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQMessageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQMessage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQMessage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQMessage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQMessage * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Class)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_AuthLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthLevel )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_AuthLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthLevel )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_IsAuthenticated)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsAuthenticated )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Delivery)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Delivery )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Delivery)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Delivery )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Trace)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Trace )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Trace)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Trace )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Priority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Priority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_ResponseQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, putref_ResponseQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_AppSpecific)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AppSpecific )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_AppSpecific)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AppSpecific )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_SourceMachineGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SourceMachineGuid )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_BodyLength)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BodyLength )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *pcbBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Body)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Body)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_AdminQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, putref_AdminQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Id)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_CorrelationId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CorrelationId )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_CorrelationId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_CorrelationId )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ VARIANT varMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Ack)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Ack )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Ack)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Ack )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_MaxTimeToReachQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_MaxTimeToReachQueue)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_MaxTimeToReceive)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_MaxTimeToReceive)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_HashAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_HashAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_EncryptAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_EncryptAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_SentTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SentTime )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_ArrivedTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ArrivedTime )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_DestinationQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationQueueInfo )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoDest);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_SenderCertificate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderCertificate )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_SenderCertificate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderCertificate )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ VARIANT varSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_SenderId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderId )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, get_SenderIdType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderIdType )( 
            __RPC__in IMSMQMessage * This,
            /* [retval][out] */ __RPC__out long *plSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, put_SenderIdType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderIdType )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ long lSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, Send)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IMSMQMessage * This,
            /* [in] */ __RPC__in_opt IMSMQQueue *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction);
        
        DECLSPEC_XFGVIRT(IMSMQMessage, AttachCurrentSecurityContext)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *AttachCurrentSecurityContext )( 
            __RPC__in IMSMQMessage * This);
        
        END_INTERFACE
    } IMSMQMessageVtbl;

    interface IMSMQMessage
    {
        CONST_VTBL struct IMSMQMessageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQMessage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQMessage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQMessage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQMessage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQMessage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQMessage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQMessage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQMessage_get_Class(This,plClass)	\
    ( (This)->lpVtbl -> get_Class(This,plClass) ) 

#define IMSMQMessage_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQMessage_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQMessage_get_AuthLevel(This,plAuthLevel)	\
    ( (This)->lpVtbl -> get_AuthLevel(This,plAuthLevel) ) 

#define IMSMQMessage_put_AuthLevel(This,lAuthLevel)	\
    ( (This)->lpVtbl -> put_AuthLevel(This,lAuthLevel) ) 

#define IMSMQMessage_get_IsAuthenticated(This,pisAuthenticated)	\
    ( (This)->lpVtbl -> get_IsAuthenticated(This,pisAuthenticated) ) 

#define IMSMQMessage_get_Delivery(This,plDelivery)	\
    ( (This)->lpVtbl -> get_Delivery(This,plDelivery) ) 

#define IMSMQMessage_put_Delivery(This,lDelivery)	\
    ( (This)->lpVtbl -> put_Delivery(This,lDelivery) ) 

#define IMSMQMessage_get_Trace(This,plTrace)	\
    ( (This)->lpVtbl -> get_Trace(This,plTrace) ) 

#define IMSMQMessage_put_Trace(This,lTrace)	\
    ( (This)->lpVtbl -> put_Trace(This,lTrace) ) 

#define IMSMQMessage_get_Priority(This,plPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,plPriority) ) 

#define IMSMQMessage_put_Priority(This,lPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lPriority) ) 

#define IMSMQMessage_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQMessage_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQMessage_get_ResponseQueueInfo(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo(This,ppqinfoResponse) ) 

#define IMSMQMessage_putref_ResponseQueueInfo(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo(This,pqinfoResponse) ) 

#define IMSMQMessage_get_AppSpecific(This,plAppSpecific)	\
    ( (This)->lpVtbl -> get_AppSpecific(This,plAppSpecific) ) 

#define IMSMQMessage_put_AppSpecific(This,lAppSpecific)	\
    ( (This)->lpVtbl -> put_AppSpecific(This,lAppSpecific) ) 

#define IMSMQMessage_get_SourceMachineGuid(This,pbstrGuidSrcMachine)	\
    ( (This)->lpVtbl -> get_SourceMachineGuid(This,pbstrGuidSrcMachine) ) 

#define IMSMQMessage_get_BodyLength(This,pcbBody)	\
    ( (This)->lpVtbl -> get_BodyLength(This,pcbBody) ) 

#define IMSMQMessage_get_Body(This,pvarBody)	\
    ( (This)->lpVtbl -> get_Body(This,pvarBody) ) 

#define IMSMQMessage_put_Body(This,varBody)	\
    ( (This)->lpVtbl -> put_Body(This,varBody) ) 

#define IMSMQMessage_get_AdminQueueInfo(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo(This,ppqinfoAdmin) ) 

#define IMSMQMessage_putref_AdminQueueInfo(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo(This,pqinfoAdmin) ) 

#define IMSMQMessage_get_Id(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_Id(This,pvarMsgId) ) 

#define IMSMQMessage_get_CorrelationId(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_CorrelationId(This,pvarMsgId) ) 

#define IMSMQMessage_put_CorrelationId(This,varMsgId)	\
    ( (This)->lpVtbl -> put_CorrelationId(This,varMsgId) ) 

#define IMSMQMessage_get_Ack(This,plAck)	\
    ( (This)->lpVtbl -> get_Ack(This,plAck) ) 

#define IMSMQMessage_put_Ack(This,lAck)	\
    ( (This)->lpVtbl -> put_Ack(This,lAck) ) 

#define IMSMQMessage_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQMessage_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQMessage_get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue) ) 

#define IMSMQMessage_put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue) ) 

#define IMSMQMessage_get_MaxTimeToReceive(This,plMaxTimeToReceive)	\
    ( (This)->lpVtbl -> get_MaxTimeToReceive(This,plMaxTimeToReceive) ) 

#define IMSMQMessage_put_MaxTimeToReceive(This,lMaxTimeToReceive)	\
    ( (This)->lpVtbl -> put_MaxTimeToReceive(This,lMaxTimeToReceive) ) 

#define IMSMQMessage_get_HashAlgorithm(This,plHashAlg)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,plHashAlg) ) 

#define IMSMQMessage_put_HashAlgorithm(This,lHashAlg)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,lHashAlg) ) 

#define IMSMQMessage_get_EncryptAlgorithm(This,plEncryptAlg)	\
    ( (This)->lpVtbl -> get_EncryptAlgorithm(This,plEncryptAlg) ) 

#define IMSMQMessage_put_EncryptAlgorithm(This,lEncryptAlg)	\
    ( (This)->lpVtbl -> put_EncryptAlgorithm(This,lEncryptAlg) ) 

#define IMSMQMessage_get_SentTime(This,pvarSentTime)	\
    ( (This)->lpVtbl -> get_SentTime(This,pvarSentTime) ) 

#define IMSMQMessage_get_ArrivedTime(This,plArrivedTime)	\
    ( (This)->lpVtbl -> get_ArrivedTime(This,plArrivedTime) ) 

#define IMSMQMessage_get_DestinationQueueInfo(This,ppqinfoDest)	\
    ( (This)->lpVtbl -> get_DestinationQueueInfo(This,ppqinfoDest) ) 

#define IMSMQMessage_get_SenderCertificate(This,pvarSenderCert)	\
    ( (This)->lpVtbl -> get_SenderCertificate(This,pvarSenderCert) ) 

#define IMSMQMessage_put_SenderCertificate(This,varSenderCert)	\
    ( (This)->lpVtbl -> put_SenderCertificate(This,varSenderCert) ) 

#define IMSMQMessage_get_SenderId(This,pvarSenderId)	\
    ( (This)->lpVtbl -> get_SenderId(This,pvarSenderId) ) 

#define IMSMQMessage_get_SenderIdType(This,plSenderIdType)	\
    ( (This)->lpVtbl -> get_SenderIdType(This,plSenderIdType) ) 

#define IMSMQMessage_put_SenderIdType(This,lSenderIdType)	\
    ( (This)->lpVtbl -> put_SenderIdType(This,lSenderIdType) ) 

#define IMSMQMessage_Send(This,DestinationQueue,Transaction)	\
    ( (This)->lpVtbl -> Send(This,DestinationQueue,Transaction) ) 

#define IMSMQMessage_AttachCurrentSecurityContext(This)	\
    ( (This)->lpVtbl -> AttachCurrentSecurityContext(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQMessage_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfos_INTERFACE_DEFINED__
#define __IMSMQQueueInfos_INTERFACE_DEFINED__

/* interface IMSMQQueueInfos */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfos;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E07D-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQQueueInfos : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoNext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfosVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfos * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfos * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfos * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfos * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfos * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfos * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfos * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueueInfos * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos, Next)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IMSMQQueueInfos * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoNext);
        
        END_INTERFACE
    } IMSMQQueueInfosVtbl;

    interface IMSMQQueueInfos
    {
        CONST_VTBL struct IMSMQQueueInfosVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfos_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfos_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfos_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfos_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfos_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfos_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfos_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfos_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueueInfos_Next(This,ppqinfoNext)	\
    ( (This)->lpVtbl -> Next(This,ppqinfoNext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfos_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfos2_INTERFACE_DEFINED__
#define __IMSMQQueueInfos2_INTERFACE_DEFINED__

/* interface IMSMQQueueInfos2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfos2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b0f-2168-11d3-898c-00e02c074f6b")
    IMSMQQueueInfos2 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoNext) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfos2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfos2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfos2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfos2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfos2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfos2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfos2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfos2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos2, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueueInfos2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos2, Next)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IMSMQQueueInfos2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoNext);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueueInfos2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQQueueInfos2Vtbl;

    interface IMSMQQueueInfos2
    {
        CONST_VTBL struct IMSMQQueueInfos2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfos2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfos2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfos2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfos2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfos2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfos2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfos2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfos2_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueueInfos2_Next(This,ppqinfoNext)	\
    ( (This)->lpVtbl -> Next(This,ppqinfoNext) ) 

#define IMSMQQueueInfos2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfos2_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfos3_INTERFACE_DEFINED__
#define __IMSMQQueueInfos3_INTERFACE_DEFINED__

/* interface IMSMQQueueInfos3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfos3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b1e-2168-11d3-898c-00e02c074f6b")
    IMSMQQueueInfos3 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoNext) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfos3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfos3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfos3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfos3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfos3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfos3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfos3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfos3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos3, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueueInfos3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos3, Next)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IMSMQQueueInfos3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoNext);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueueInfos3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQQueueInfos3Vtbl;

    interface IMSMQQueueInfos3
    {
        CONST_VTBL struct IMSMQQueueInfos3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfos3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfos3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfos3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfos3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfos3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfos3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfos3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfos3_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueueInfos3_Next(This,ppqinfoNext)	\
    ( (This)->lpVtbl -> Next(This,ppqinfoNext) ) 

#define IMSMQQueueInfos3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfos3_INTERFACE_DEFINED__ */


#ifndef __IMSMQQueueInfos4_INTERFACE_DEFINED__
#define __IMSMQQueueInfos4_INTERFACE_DEFINED__

/* interface IMSMQQueueInfos4 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueInfos4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b22-2168-11d3-898c-00e02c074f6b")
    IMSMQQueueInfos4 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoNext) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueInfos4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueInfos4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueInfos4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueInfos4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueInfos4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueInfos4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueInfos4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueInfos4 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos4, Reset)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMSMQQueueInfos4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos4, Next)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IMSMQQueueInfos4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoNext);
        
        DECLSPEC_XFGVIRT(IMSMQQueueInfos4, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQueueInfos4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQQueueInfos4Vtbl;

    interface IMSMQQueueInfos4
    {
        CONST_VTBL struct IMSMQQueueInfos4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueInfos4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueInfos4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueInfos4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueInfos4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueInfos4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueInfos4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueInfos4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueInfos4_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMSMQQueueInfos4_Next(This,ppqinfoNext)	\
    ( (This)->lpVtbl -> Next(This,ppqinfoNext) ) 

#define IMSMQQueueInfos4_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueInfos4_INTERFACE_DEFINED__ */


#ifndef __IMSMQEvent_INTERFACE_DEFINED__
#define __IMSMQEvent_INTERFACE_DEFINED__

/* interface IMSMQEvent */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E077-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQEvent : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQEvent * This,
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
    } IMSMQEventVtbl;

    interface IMSMQEvent
    {
        CONST_VTBL struct IMSMQEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQEvent_INTERFACE_DEFINED__ */


#ifndef __IMSMQEvent2_INTERFACE_DEFINED__
#define __IMSMQEvent2_INTERFACE_DEFINED__

/* interface IMSMQEvent2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b12-2168-11d3-898c-00e02c074f6b")
    IMSMQEvent2 : public IMSMQEvent
    {
    public:
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQEvent2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQEvent2Vtbl;

    interface IMSMQEvent2
    {
        CONST_VTBL struct IMSMQEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 



#define IMSMQEvent2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQEvent2_INTERFACE_DEFINED__ */


#ifndef __IMSMQEvent3_INTERFACE_DEFINED__
#define __IMSMQEvent3_INTERFACE_DEFINED__

/* interface IMSMQEvent3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQEvent3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b1c-2168-11d3-898c-00e02c074f6b")
    IMSMQEvent3 : public IMSMQEvent2
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQEvent3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQEvent3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQEvent3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQEvent3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQEvent3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQEvent3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQEvent3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQEvent3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQEvent2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQEvent3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQEvent3Vtbl;

    interface IMSMQEvent3
    {
        CONST_VTBL struct IMSMQEvent3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQEvent3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQEvent3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQEvent3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQEvent3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQEvent3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQEvent3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQEvent3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 



#define IMSMQEvent3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQEvent3_INTERFACE_DEFINED__ */


#ifndef __IMSMQTransaction_INTERFACE_DEFINED__
#define __IMSMQTransaction_INTERFACE_DEFINED__

/* interface IMSMQTransaction */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQTransaction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E07F-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQTransaction : public IDispatch
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Transaction( 
            /* [retval][out] */ __RPC__out long *plTransaction) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Commit( 
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *grfTC,
            /* [optional][in] */ __RPC__in VARIANT *grfRM) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Abort( 
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *fAsync) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQTransactionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQTransaction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQTransaction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQTransaction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQTransaction * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQTransaction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQTransaction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQTransaction * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, get_Transaction)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Transaction )( 
            __RPC__in IMSMQTransaction * This,
            /* [retval][out] */ __RPC__out long *plTransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, Commit)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IMSMQTransaction * This,
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *grfTC,
            /* [optional][in] */ __RPC__in VARIANT *grfRM);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, Abort)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IMSMQTransaction * This,
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *fAsync);
        
        END_INTERFACE
    } IMSMQTransactionVtbl;

    interface IMSMQTransaction
    {
        CONST_VTBL struct IMSMQTransactionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQTransaction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQTransaction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQTransaction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQTransaction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQTransaction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQTransaction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQTransaction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQTransaction_get_Transaction(This,plTransaction)	\
    ( (This)->lpVtbl -> get_Transaction(This,plTransaction) ) 

#define IMSMQTransaction_Commit(This,fRetaining,grfTC,grfRM)	\
    ( (This)->lpVtbl -> Commit(This,fRetaining,grfTC,grfRM) ) 

#define IMSMQTransaction_Abort(This,fRetaining,fAsync)	\
    ( (This)->lpVtbl -> Abort(This,fRetaining,fAsync) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQTransaction_INTERFACE_DEFINED__ */


#ifndef __IMSMQCoordinatedTransactionDispenser_INTERFACE_DEFINED__
#define __IMSMQCoordinatedTransactionDispenser_INTERFACE_DEFINED__

/* interface IMSMQCoordinatedTransactionDispenser */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQCoordinatedTransactionDispenser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E081-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQCoordinatedTransactionDispenser : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE BeginTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction **ptransaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQCoordinatedTransactionDispenserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQCoordinatedTransactionDispenser * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQCoordinatedTransactionDispenser, BeginTransaction)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *BeginTransaction )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction **ptransaction);
        
        END_INTERFACE
    } IMSMQCoordinatedTransactionDispenserVtbl;

    interface IMSMQCoordinatedTransactionDispenser
    {
        CONST_VTBL struct IMSMQCoordinatedTransactionDispenserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQCoordinatedTransactionDispenser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQCoordinatedTransactionDispenser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQCoordinatedTransactionDispenser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQCoordinatedTransactionDispenser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQCoordinatedTransactionDispenser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQCoordinatedTransactionDispenser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQCoordinatedTransactionDispenser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQCoordinatedTransactionDispenser_BeginTransaction(This,ptransaction)	\
    ( (This)->lpVtbl -> BeginTransaction(This,ptransaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQCoordinatedTransactionDispenser_INTERFACE_DEFINED__ */


#ifndef __IMSMQTransactionDispenser_INTERFACE_DEFINED__
#define __IMSMQTransactionDispenser_INTERFACE_DEFINED__

/* interface IMSMQTransactionDispenser */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQTransactionDispenser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E083-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQTransactionDispenser : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE BeginTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction **ptransaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQTransactionDispenserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQTransactionDispenser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQTransactionDispenser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQTransactionDispenser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQTransactionDispenser * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQTransactionDispenser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQTransactionDispenser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQTransactionDispenser * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQTransactionDispenser, BeginTransaction)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *BeginTransaction )( 
            __RPC__in IMSMQTransactionDispenser * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction **ptransaction);
        
        END_INTERFACE
    } IMSMQTransactionDispenserVtbl;

    interface IMSMQTransactionDispenser
    {
        CONST_VTBL struct IMSMQTransactionDispenserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQTransactionDispenser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQTransactionDispenser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQTransactionDispenser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQTransactionDispenser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQTransactionDispenser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQTransactionDispenser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQTransactionDispenser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQTransactionDispenser_BeginTransaction(This,ptransaction)	\
    ( (This)->lpVtbl -> BeginTransaction(This,ptransaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQTransactionDispenser_INTERFACE_DEFINED__ */


#ifndef __IMSMQQuery2_INTERFACE_DEFINED__
#define __IMSMQQuery2_INTERFACE_DEFINED__

/* interface IMSMQQuery2 */
/* [object][nonextensible][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQuery2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b0e-2168-11d3-898c-00e02c074f6b")
    IMSMQQuery2 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE LookupQueue( 
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos2 **ppqinfos) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQuery2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQuery2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQuery2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQuery2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQuery2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQuery2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQuery2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQuery2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQuery2, LookupQueue)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *LookupQueue )( 
            __RPC__in IMSMQQuery2 * This,
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos2 **ppqinfos);
        
        DECLSPEC_XFGVIRT(IMSMQQuery2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQuery2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQQuery2Vtbl;

    interface IMSMQQuery2
    {
        CONST_VTBL struct IMSMQQuery2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQuery2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQuery2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQuery2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQuery2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQuery2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQuery2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQuery2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQuery2_LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos)	\
    ( (This)->lpVtbl -> LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos) ) 

#define IMSMQQuery2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQuery2_INTERFACE_DEFINED__ */


#ifndef __IMSMQQuery3_INTERFACE_DEFINED__
#define __IMSMQQuery3_INTERFACE_DEFINED__

/* interface IMSMQQuery3 */
/* [object][nonextensible][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQuery3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b19-2168-11d3-898c-00e02c074f6b")
    IMSMQQuery3 : public IDispatch
    {
    public:
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE LookupQueue_v2( 
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos3 **ppqinfos) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE LookupQueue( 
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *MulticastAddress,
            /* [optional][in] */ __RPC__in VARIANT *RelMulticastAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos3 **ppqinfos) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQuery3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQuery3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQuery3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQuery3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQuery3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQuery3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQuery3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQuery3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQuery3, LookupQueue_v2)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *LookupQueue_v2 )( 
            __RPC__in IMSMQQuery3 * This,
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos3 **ppqinfos);
        
        DECLSPEC_XFGVIRT(IMSMQQuery3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQuery3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQuery3, LookupQueue)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *LookupQueue )( 
            __RPC__in IMSMQQuery3 * This,
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *MulticastAddress,
            /* [optional][in] */ __RPC__in VARIANT *RelMulticastAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos3 **ppqinfos);
        
        END_INTERFACE
    } IMSMQQuery3Vtbl;

    interface IMSMQQuery3
    {
        CONST_VTBL struct IMSMQQuery3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQuery3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQuery3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQuery3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQuery3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQuery3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQuery3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQuery3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQuery3_LookupQueue_v2(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos)	\
    ( (This)->lpVtbl -> LookupQueue_v2(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos) ) 

#define IMSMQQuery3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQuery3_LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,MulticastAddress,RelMulticastAddress,ppqinfos)	\
    ( (This)->lpVtbl -> LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,MulticastAddress,RelMulticastAddress,ppqinfos) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQuery3_INTERFACE_DEFINED__ */


#ifndef __IMSMQQuery4_INTERFACE_DEFINED__
#define __IMSMQQuery4_INTERFACE_DEFINED__

/* interface IMSMQQuery4 */
/* [object][nonextensible][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQuery4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b24-2168-11d3-898c-00e02c074f6b")
    IMSMQQuery4 : public IDispatch
    {
    public:
        virtual /* [hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE LookupQueue_v2( 
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos4 **ppqinfos) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE LookupQueue( 
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *MulticastAddress,
            /* [optional][in] */ __RPC__in VARIANT *RelMulticastAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos4 **ppqinfos) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQuery4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQuery4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQuery4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQuery4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQuery4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQuery4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQuery4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQuery4 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQQuery4, LookupQueue_v2)
        /* [hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *LookupQueue_v2 )( 
            __RPC__in IMSMQQuery4 * This,
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos4 **ppqinfos);
        
        DECLSPEC_XFGVIRT(IMSMQQuery4, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQQuery4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQQuery4, LookupQueue)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *LookupQueue )( 
            __RPC__in IMSMQQuery4 * This,
            /* [optional][in] */ __RPC__in VARIANT *QueueGuid,
            /* [optional][in] */ __RPC__in VARIANT *ServiceTypeGuid,
            /* [optional][in] */ __RPC__in VARIANT *Label,
            /* [optional][in] */ __RPC__in VARIANT *CreateTime,
            /* [optional][in] */ __RPC__in VARIANT *ModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *RelServiceType,
            /* [optional][in] */ __RPC__in VARIANT *RelLabel,
            /* [optional][in] */ __RPC__in VARIANT *RelCreateTime,
            /* [optional][in] */ __RPC__in VARIANT *RelModifyTime,
            /* [optional][in] */ __RPC__in VARIANT *MulticastAddress,
            /* [optional][in] */ __RPC__in VARIANT *RelMulticastAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfos4 **ppqinfos);
        
        END_INTERFACE
    } IMSMQQuery4Vtbl;

    interface IMSMQQuery4
    {
        CONST_VTBL struct IMSMQQuery4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQuery4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQuery4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQuery4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQuery4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQuery4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQuery4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQuery4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQuery4_LookupQueue_v2(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos)	\
    ( (This)->lpVtbl -> LookupQueue_v2(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,ppqinfos) ) 

#define IMSMQQuery4_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQQuery4_LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,MulticastAddress,RelMulticastAddress,ppqinfos)	\
    ( (This)->lpVtbl -> LookupQueue(This,QueueGuid,ServiceTypeGuid,Label,CreateTime,ModifyTime,RelServiceType,RelLabel,RelCreateTime,RelModifyTime,MulticastAddress,RelMulticastAddress,ppqinfos) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQuery4_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQQuery;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E073-DCCD-11d0-AA4B-0060970DEBAE")
MSMQQuery;
#endif

#ifndef __IMSMQMessage2_INTERFACE_DEFINED__
#define __IMSMQMessage2_INTERFACE_DEFINED__

/* interface IMSMQMessage2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQMessage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D9933BE0-A567-11D2-B0F3-00E02C074F6B")
    IMSMQMessage2 : public IDispatch
    {
    public:
        virtual /* [id][propget][hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Class( 
            /* [retval][out] */ __RPC__out long *plClass) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthLevel( 
            /* [retval][out] */ __RPC__out long *plAuthLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthLevel( 
            /* [in] */ long lAuthLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsAuthenticated( 
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Delivery( 
            /* [retval][out] */ __RPC__out long *plDelivery) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Delivery( 
            /* [in] */ long lDelivery) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Trace( 
            /* [retval][out] */ __RPC__out long *plTrace) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Trace( 
            /* [in] */ long lTrace) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *plPriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lPriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo_v1( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo_v1( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AppSpecific( 
            /* [retval][out] */ __RPC__out long *plAppSpecific) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AppSpecific( 
            /* [in] */ long lAppSpecific) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SourceMachineGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BodyLength( 
            /* [retval][out] */ __RPC__out long *pcbBody) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ __RPC__out VARIANT *pvarBody) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Body( 
            /* [in] */ VARIANT varBody) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo_v1( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo_v1( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CorrelationId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_CorrelationId( 
            /* [in] */ VARIANT varMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Ack( 
            /* [retval][out] */ __RPC__out long *plAck) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Ack( 
            /* [in] */ long lAck) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReachQueue( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReachQueue( 
            /* [in] */ long lMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReceive( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReceive( 
            /* [in] */ long lMaxTimeToReceive) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__out long *plHashAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ long lHashAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_EncryptAlgorithm( 
            /* [retval][out] */ __RPC__out long *plEncryptAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_EncryptAlgorithm( 
            /* [in] */ long lEncryptAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SentTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ArrivedTime( 
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoDest) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderCertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderCertificate( 
            /* [in] */ VARIANT varSenderCert) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderIdType( 
            /* [retval][out] */ __RPC__out long *plSenderIdType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderIdType( 
            /* [in] */ long lSenderIdType) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Send( 
            /* [in] */ __RPC__in_opt IMSMQQueue2 *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE AttachCurrentSecurityContext( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderVersion( 
            /* [retval][out] */ __RPC__out long *plSenderVersion) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Extension( 
            /* [retval][out] */ __RPC__out VARIANT *pvarExtension) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Extension( 
            /* [in] */ VARIANT varExtension) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ConnectorTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidConnectorType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ConnectorTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidConnectorType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionStatusQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoXactStatus) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationSymmetricKey( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDestSymmKey) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_DestinationSymmetricKey( 
            /* [in] */ VARIANT varDestSymmKey) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Signature( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSignature) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Signature( 
            /* [in] */ VARIANT varSignature) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthenticationProviderType( 
            /* [retval][out] */ __RPC__out long *plAuthProvType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthenticationProviderType( 
            /* [in] */ long lAuthProvType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthenticationProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAuthProvName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthenticationProviderName( 
            /* [in] */ __RPC__in BSTR bstrAuthProvName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderId( 
            /* [in] */ VARIANT varSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MsgClass( 
            /* [retval][out] */ __RPC__out long *plMsgClass) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MsgClass( 
            /* [in] */ long lMsgClass) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarXactId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsFirstInTransaction( 
            /* [retval][out] */ __RPC__out Boolean *pisFirstInXact) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsLastInTransaction( 
            /* [retval][out] */ __RPC__out Boolean *pisLastInXact) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoResponse) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoAdmin) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ReceivedAuthenticationLevel( 
            /* [retval][out] */ __RPC__out short *psReceivedAuthenticationLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQMessage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQMessage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQMessage2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQMessage2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQMessage2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Class)
        /* [id][propget][hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_AuthLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthLevel )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_AuthLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthLevel )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_IsAuthenticated)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsAuthenticated )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Delivery)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Delivery )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Delivery)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Delivery )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Trace)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Trace )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Trace)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Trace )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Priority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Priority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_ResponseQueueInfo_v1)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo_v1 )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, putref_ResponseQueueInfo_v1)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo_v1 )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_AppSpecific)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AppSpecific )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_AppSpecific)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AppSpecific )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_SourceMachineGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SourceMachineGuid )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_BodyLength)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BodyLength )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *pcbBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Body)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Body)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_AdminQueueInfo_v1)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo_v1 )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, putref_AdminQueueInfo_v1)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo_v1 )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Id)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_CorrelationId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CorrelationId )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_CorrelationId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_CorrelationId )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Ack)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Ack )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Ack)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Ack )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_MaxTimeToReachQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_MaxTimeToReachQueue)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_MaxTimeToReceive)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_MaxTimeToReceive)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_HashAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_HashAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_EncryptAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_EncryptAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_SentTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SentTime )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_ArrivedTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ArrivedTime )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_DestinationQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationQueueInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoDest);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_SenderCertificate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderCertificate )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_SenderCertificate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderCertificate )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_SenderId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderId )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_SenderIdType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderIdType )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_SenderIdType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderIdType )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, Send)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in_opt IMSMQQueue2 *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, AttachCurrentSecurityContext)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *AttachCurrentSecurityContext )( 
            __RPC__in IMSMQMessage2 * This);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_SenderVersion)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderVersion )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plSenderVersion);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Extension)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Extension )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarExtension);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Extension)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Extension )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varExtension);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_ConnectorTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectorTypeGuid )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidConnectorType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_ConnectorTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectorTypeGuid )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrGuidConnectorType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_TransactionStatusQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionStatusQueueInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoXactStatus);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_DestinationSymmetricKey)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationSymmetricKey )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDestSymmKey);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_DestinationSymmetricKey)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationSymmetricKey )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varDestSymmKey);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Signature)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Signature )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSignature);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_Signature)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Signature )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varSignature);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_AuthenticationProviderType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationProviderType )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plAuthProvType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_AuthenticationProviderType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationProviderType )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lAuthProvType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_AuthenticationProviderName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationProviderName )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAuthProvName);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_AuthenticationProviderName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationProviderName )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrAuthProvName);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_SenderId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderId )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ VARIANT varSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_MsgClass)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MsgClass )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out long *plMsgClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, put_MsgClass)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MsgClass )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ long lMsgClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_TransactionId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionId )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarXactId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_IsFirstInTransaction)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstInTransaction )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out Boolean *pisFirstInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_IsLastInTransaction)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsLastInTransaction )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out Boolean *pisLastInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_ResponseQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, putref_ResponseQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_AdminQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, putref_AdminQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo )( 
            __RPC__in IMSMQMessage2 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage2, get_ReceivedAuthenticationLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ReceivedAuthenticationLevel )( 
            __RPC__in IMSMQMessage2 * This,
            /* [retval][out] */ __RPC__out short *psReceivedAuthenticationLevel);
        
        END_INTERFACE
    } IMSMQMessage2Vtbl;

    interface IMSMQMessage2
    {
        CONST_VTBL struct IMSMQMessage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQMessage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQMessage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQMessage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQMessage2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQMessage2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQMessage2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQMessage2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQMessage2_get_Class(This,plClass)	\
    ( (This)->lpVtbl -> get_Class(This,plClass) ) 

#define IMSMQMessage2_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQMessage2_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQMessage2_get_AuthLevel(This,plAuthLevel)	\
    ( (This)->lpVtbl -> get_AuthLevel(This,plAuthLevel) ) 

#define IMSMQMessage2_put_AuthLevel(This,lAuthLevel)	\
    ( (This)->lpVtbl -> put_AuthLevel(This,lAuthLevel) ) 

#define IMSMQMessage2_get_IsAuthenticated(This,pisAuthenticated)	\
    ( (This)->lpVtbl -> get_IsAuthenticated(This,pisAuthenticated) ) 

#define IMSMQMessage2_get_Delivery(This,plDelivery)	\
    ( (This)->lpVtbl -> get_Delivery(This,plDelivery) ) 

#define IMSMQMessage2_put_Delivery(This,lDelivery)	\
    ( (This)->lpVtbl -> put_Delivery(This,lDelivery) ) 

#define IMSMQMessage2_get_Trace(This,plTrace)	\
    ( (This)->lpVtbl -> get_Trace(This,plTrace) ) 

#define IMSMQMessage2_put_Trace(This,lTrace)	\
    ( (This)->lpVtbl -> put_Trace(This,lTrace) ) 

#define IMSMQMessage2_get_Priority(This,plPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,plPriority) ) 

#define IMSMQMessage2_put_Priority(This,lPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lPriority) ) 

#define IMSMQMessage2_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQMessage2_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQMessage2_get_ResponseQueueInfo_v1(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo_v1(This,ppqinfoResponse) ) 

#define IMSMQMessage2_putref_ResponseQueueInfo_v1(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo_v1(This,pqinfoResponse) ) 

#define IMSMQMessage2_get_AppSpecific(This,plAppSpecific)	\
    ( (This)->lpVtbl -> get_AppSpecific(This,plAppSpecific) ) 

#define IMSMQMessage2_put_AppSpecific(This,lAppSpecific)	\
    ( (This)->lpVtbl -> put_AppSpecific(This,lAppSpecific) ) 

#define IMSMQMessage2_get_SourceMachineGuid(This,pbstrGuidSrcMachine)	\
    ( (This)->lpVtbl -> get_SourceMachineGuid(This,pbstrGuidSrcMachine) ) 

#define IMSMQMessage2_get_BodyLength(This,pcbBody)	\
    ( (This)->lpVtbl -> get_BodyLength(This,pcbBody) ) 

#define IMSMQMessage2_get_Body(This,pvarBody)	\
    ( (This)->lpVtbl -> get_Body(This,pvarBody) ) 

#define IMSMQMessage2_put_Body(This,varBody)	\
    ( (This)->lpVtbl -> put_Body(This,varBody) ) 

#define IMSMQMessage2_get_AdminQueueInfo_v1(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo_v1(This,ppqinfoAdmin) ) 

#define IMSMQMessage2_putref_AdminQueueInfo_v1(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo_v1(This,pqinfoAdmin) ) 

#define IMSMQMessage2_get_Id(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_Id(This,pvarMsgId) ) 

#define IMSMQMessage2_get_CorrelationId(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_CorrelationId(This,pvarMsgId) ) 

#define IMSMQMessage2_put_CorrelationId(This,varMsgId)	\
    ( (This)->lpVtbl -> put_CorrelationId(This,varMsgId) ) 

#define IMSMQMessage2_get_Ack(This,plAck)	\
    ( (This)->lpVtbl -> get_Ack(This,plAck) ) 

#define IMSMQMessage2_put_Ack(This,lAck)	\
    ( (This)->lpVtbl -> put_Ack(This,lAck) ) 

#define IMSMQMessage2_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQMessage2_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQMessage2_get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue) ) 

#define IMSMQMessage2_put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue) ) 

#define IMSMQMessage2_get_MaxTimeToReceive(This,plMaxTimeToReceive)	\
    ( (This)->lpVtbl -> get_MaxTimeToReceive(This,plMaxTimeToReceive) ) 

#define IMSMQMessage2_put_MaxTimeToReceive(This,lMaxTimeToReceive)	\
    ( (This)->lpVtbl -> put_MaxTimeToReceive(This,lMaxTimeToReceive) ) 

#define IMSMQMessage2_get_HashAlgorithm(This,plHashAlg)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,plHashAlg) ) 

#define IMSMQMessage2_put_HashAlgorithm(This,lHashAlg)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,lHashAlg) ) 

#define IMSMQMessage2_get_EncryptAlgorithm(This,plEncryptAlg)	\
    ( (This)->lpVtbl -> get_EncryptAlgorithm(This,plEncryptAlg) ) 

#define IMSMQMessage2_put_EncryptAlgorithm(This,lEncryptAlg)	\
    ( (This)->lpVtbl -> put_EncryptAlgorithm(This,lEncryptAlg) ) 

#define IMSMQMessage2_get_SentTime(This,pvarSentTime)	\
    ( (This)->lpVtbl -> get_SentTime(This,pvarSentTime) ) 

#define IMSMQMessage2_get_ArrivedTime(This,plArrivedTime)	\
    ( (This)->lpVtbl -> get_ArrivedTime(This,plArrivedTime) ) 

#define IMSMQMessage2_get_DestinationQueueInfo(This,ppqinfoDest)	\
    ( (This)->lpVtbl -> get_DestinationQueueInfo(This,ppqinfoDest) ) 

#define IMSMQMessage2_get_SenderCertificate(This,pvarSenderCert)	\
    ( (This)->lpVtbl -> get_SenderCertificate(This,pvarSenderCert) ) 

#define IMSMQMessage2_put_SenderCertificate(This,varSenderCert)	\
    ( (This)->lpVtbl -> put_SenderCertificate(This,varSenderCert) ) 

#define IMSMQMessage2_get_SenderId(This,pvarSenderId)	\
    ( (This)->lpVtbl -> get_SenderId(This,pvarSenderId) ) 

#define IMSMQMessage2_get_SenderIdType(This,plSenderIdType)	\
    ( (This)->lpVtbl -> get_SenderIdType(This,plSenderIdType) ) 

#define IMSMQMessage2_put_SenderIdType(This,lSenderIdType)	\
    ( (This)->lpVtbl -> put_SenderIdType(This,lSenderIdType) ) 

#define IMSMQMessage2_Send(This,DestinationQueue,Transaction)	\
    ( (This)->lpVtbl -> Send(This,DestinationQueue,Transaction) ) 

#define IMSMQMessage2_AttachCurrentSecurityContext(This)	\
    ( (This)->lpVtbl -> AttachCurrentSecurityContext(This) ) 

#define IMSMQMessage2_get_SenderVersion(This,plSenderVersion)	\
    ( (This)->lpVtbl -> get_SenderVersion(This,plSenderVersion) ) 

#define IMSMQMessage2_get_Extension(This,pvarExtension)	\
    ( (This)->lpVtbl -> get_Extension(This,pvarExtension) ) 

#define IMSMQMessage2_put_Extension(This,varExtension)	\
    ( (This)->lpVtbl -> put_Extension(This,varExtension) ) 

#define IMSMQMessage2_get_ConnectorTypeGuid(This,pbstrGuidConnectorType)	\
    ( (This)->lpVtbl -> get_ConnectorTypeGuid(This,pbstrGuidConnectorType) ) 

#define IMSMQMessage2_put_ConnectorTypeGuid(This,bstrGuidConnectorType)	\
    ( (This)->lpVtbl -> put_ConnectorTypeGuid(This,bstrGuidConnectorType) ) 

#define IMSMQMessage2_get_TransactionStatusQueueInfo(This,ppqinfoXactStatus)	\
    ( (This)->lpVtbl -> get_TransactionStatusQueueInfo(This,ppqinfoXactStatus) ) 

#define IMSMQMessage2_get_DestinationSymmetricKey(This,pvarDestSymmKey)	\
    ( (This)->lpVtbl -> get_DestinationSymmetricKey(This,pvarDestSymmKey) ) 

#define IMSMQMessage2_put_DestinationSymmetricKey(This,varDestSymmKey)	\
    ( (This)->lpVtbl -> put_DestinationSymmetricKey(This,varDestSymmKey) ) 

#define IMSMQMessage2_get_Signature(This,pvarSignature)	\
    ( (This)->lpVtbl -> get_Signature(This,pvarSignature) ) 

#define IMSMQMessage2_put_Signature(This,varSignature)	\
    ( (This)->lpVtbl -> put_Signature(This,varSignature) ) 

#define IMSMQMessage2_get_AuthenticationProviderType(This,plAuthProvType)	\
    ( (This)->lpVtbl -> get_AuthenticationProviderType(This,plAuthProvType) ) 

#define IMSMQMessage2_put_AuthenticationProviderType(This,lAuthProvType)	\
    ( (This)->lpVtbl -> put_AuthenticationProviderType(This,lAuthProvType) ) 

#define IMSMQMessage2_get_AuthenticationProviderName(This,pbstrAuthProvName)	\
    ( (This)->lpVtbl -> get_AuthenticationProviderName(This,pbstrAuthProvName) ) 

#define IMSMQMessage2_put_AuthenticationProviderName(This,bstrAuthProvName)	\
    ( (This)->lpVtbl -> put_AuthenticationProviderName(This,bstrAuthProvName) ) 

#define IMSMQMessage2_put_SenderId(This,varSenderId)	\
    ( (This)->lpVtbl -> put_SenderId(This,varSenderId) ) 

#define IMSMQMessage2_get_MsgClass(This,plMsgClass)	\
    ( (This)->lpVtbl -> get_MsgClass(This,plMsgClass) ) 

#define IMSMQMessage2_put_MsgClass(This,lMsgClass)	\
    ( (This)->lpVtbl -> put_MsgClass(This,lMsgClass) ) 

#define IMSMQMessage2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQMessage2_get_TransactionId(This,pvarXactId)	\
    ( (This)->lpVtbl -> get_TransactionId(This,pvarXactId) ) 

#define IMSMQMessage2_get_IsFirstInTransaction(This,pisFirstInXact)	\
    ( (This)->lpVtbl -> get_IsFirstInTransaction(This,pisFirstInXact) ) 

#define IMSMQMessage2_get_IsLastInTransaction(This,pisLastInXact)	\
    ( (This)->lpVtbl -> get_IsLastInTransaction(This,pisLastInXact) ) 

#define IMSMQMessage2_get_ResponseQueueInfo(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo(This,ppqinfoResponse) ) 

#define IMSMQMessage2_putref_ResponseQueueInfo(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo(This,pqinfoResponse) ) 

#define IMSMQMessage2_get_AdminQueueInfo(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo(This,ppqinfoAdmin) ) 

#define IMSMQMessage2_putref_AdminQueueInfo(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo(This,pqinfoAdmin) ) 

#define IMSMQMessage2_get_ReceivedAuthenticationLevel(This,psReceivedAuthenticationLevel)	\
    ( (This)->lpVtbl -> get_ReceivedAuthenticationLevel(This,psReceivedAuthenticationLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQMessage2_INTERFACE_DEFINED__ */


#ifndef __IMSMQMessage3_INTERFACE_DEFINED__
#define __IMSMQMessage3_INTERFACE_DEFINED__

/* interface IMSMQMessage3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQMessage3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b1a-2168-11d3-898c-00e02c074f6b")
    IMSMQMessage3 : public IDispatch
    {
    public:
        virtual /* [id][propget][hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Class( 
            /* [retval][out] */ __RPC__out long *plClass) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthLevel( 
            /* [retval][out] */ __RPC__out long *plAuthLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthLevel( 
            /* [in] */ long lAuthLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsAuthenticated( 
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Delivery( 
            /* [retval][out] */ __RPC__out long *plDelivery) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Delivery( 
            /* [in] */ long lDelivery) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Trace( 
            /* [retval][out] */ __RPC__out long *plTrace) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Trace( 
            /* [in] */ long lTrace) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *plPriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lPriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo_v1( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo_v1( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AppSpecific( 
            /* [retval][out] */ __RPC__out long *plAppSpecific) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AppSpecific( 
            /* [in] */ long lAppSpecific) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SourceMachineGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BodyLength( 
            /* [retval][out] */ __RPC__out long *pcbBody) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ __RPC__out VARIANT *pvarBody) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Body( 
            /* [in] */ VARIANT varBody) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo_v1( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo_v1( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CorrelationId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_CorrelationId( 
            /* [in] */ VARIANT varMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Ack( 
            /* [retval][out] */ __RPC__out long *plAck) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Ack( 
            /* [in] */ long lAck) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReachQueue( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReachQueue( 
            /* [in] */ long lMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReceive( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReceive( 
            /* [in] */ long lMaxTimeToReceive) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__out long *plHashAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ long lHashAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_EncryptAlgorithm( 
            /* [retval][out] */ __RPC__out long *plEncryptAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_EncryptAlgorithm( 
            /* [in] */ long lEncryptAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SentTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ArrivedTime( 
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoDest) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderCertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderCertificate( 
            /* [in] */ VARIANT varSenderCert) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderIdType( 
            /* [retval][out] */ __RPC__out long *plSenderIdType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderIdType( 
            /* [in] */ long lSenderIdType) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Send( 
            /* [in] */ __RPC__in_opt IDispatch *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction) = 0;
        
        virtual /* [helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE AttachCurrentSecurityContext( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderVersion( 
            /* [retval][out] */ __RPC__out long *plSenderVersion) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Extension( 
            /* [retval][out] */ __RPC__out VARIANT *pvarExtension) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Extension( 
            /* [in] */ VARIANT varExtension) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ConnectorTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidConnectorType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ConnectorTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidConnectorType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionStatusQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoXactStatus) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationSymmetricKey( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDestSymmKey) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_DestinationSymmetricKey( 
            /* [in] */ VARIANT varDestSymmKey) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Signature( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSignature) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Signature( 
            /* [in] */ VARIANT varSignature) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthenticationProviderType( 
            /* [retval][out] */ __RPC__out long *plAuthProvType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthenticationProviderType( 
            /* [in] */ long lAuthProvType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthenticationProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAuthProvName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthenticationProviderName( 
            /* [in] */ __RPC__in BSTR bstrAuthProvName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderId( 
            /* [in] */ VARIANT varSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MsgClass( 
            /* [retval][out] */ __RPC__out long *plMsgClass) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MsgClass( 
            /* [in] */ long lMsgClass) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarXactId) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsFirstInTransaction( 
            /* [retval][out] */ __RPC__out Boolean *pisFirstInXact) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsLastInTransaction( 
            /* [retval][out] */ __RPC__out Boolean *pisLastInXact) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo_v2( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoResponse) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo_v2( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoResponse) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo_v2( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoAdmin) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo_v2( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ReceivedAuthenticationLevel( 
            /* [retval][out] */ __RPC__out short *psReceivedAuthenticationLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoResponse) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo3 *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoAdmin) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo3 *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseDestination( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestResponse) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseDestination( 
            /* [in] */ __RPC__in_opt IDispatch *pdestResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Destination( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestDestination) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_LookupId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarLookupId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsAuthenticated2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisAuthenticated) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsFirstInTransaction2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisFirstInXact) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsLastInTransaction2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisLastInXact) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE AttachCurrentSecurityContext2( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SoapEnvelope( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSoapEnvelope) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CompoundMessage( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCompoundMessage) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SoapHeader( 
            /* [in] */ __RPC__in BSTR bstrSoapHeader) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SoapBody( 
            /* [in] */ __RPC__in BSTR bstrSoapBody) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQMessage3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQMessage3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQMessage3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQMessage3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQMessage3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Class)
        /* [id][propget][hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AuthLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthLevel )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_AuthLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthLevel )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_IsAuthenticated)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsAuthenticated )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Delivery)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Delivery )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Delivery)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Delivery )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Trace)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Trace )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Trace)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Trace )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Priority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Priority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ResponseQueueInfo_v1)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo_v1 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_ResponseQueueInfo_v1)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo_v1 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AppSpecific)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AppSpecific )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_AppSpecific)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AppSpecific )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SourceMachineGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SourceMachineGuid )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_BodyLength)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BodyLength )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *pcbBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Body)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Body)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AdminQueueInfo_v1)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo_v1 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_AdminQueueInfo_v1)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo_v1 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Id)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_CorrelationId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CorrelationId )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_CorrelationId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_CorrelationId )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Ack)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Ack )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Ack)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Ack )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_MaxTimeToReachQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_MaxTimeToReachQueue)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_MaxTimeToReceive)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_MaxTimeToReceive)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_HashAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_HashAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_EncryptAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_EncryptAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SentTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SentTime )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ArrivedTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ArrivedTime )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_DestinationQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationQueueInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoDest);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SenderCertificate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderCertificate )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_SenderCertificate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderCertificate )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SenderId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderId )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SenderIdType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderIdType )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_SenderIdType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderIdType )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, Send)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IDispatch *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, AttachCurrentSecurityContext)
        /* [helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *AttachCurrentSecurityContext )( 
            __RPC__in IMSMQMessage3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SenderVersion)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderVersion )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plSenderVersion);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Extension)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Extension )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarExtension);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Extension)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Extension )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varExtension);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ConnectorTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectorTypeGuid )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidConnectorType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_ConnectorTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectorTypeGuid )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in BSTR bstrGuidConnectorType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_TransactionStatusQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionStatusQueueInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoXactStatus);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_DestinationSymmetricKey)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationSymmetricKey )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDestSymmKey);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_DestinationSymmetricKey)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationSymmetricKey )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varDestSymmKey);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Signature)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Signature )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSignature);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_Signature)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Signature )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varSignature);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AuthenticationProviderType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationProviderType )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plAuthProvType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_AuthenticationProviderType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationProviderType )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lAuthProvType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AuthenticationProviderName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationProviderName )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAuthProvName);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_AuthenticationProviderName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationProviderName )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in BSTR bstrAuthProvName);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_SenderId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderId )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ VARIANT varSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_MsgClass)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MsgClass )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out long *plMsgClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_MsgClass)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MsgClass )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ long lMsgClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_TransactionId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionId )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarXactId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_IsFirstInTransaction)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstInTransaction )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out Boolean *pisFirstInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_IsLastInTransaction)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsLastInTransaction )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out Boolean *pisLastInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ResponseQueueInfo_v2)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo_v2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_ResponseQueueInfo_v2)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo_v2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AdminQueueInfo_v2)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo_v2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_AdminQueueInfo_v2)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo_v2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ReceivedAuthenticationLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ReceivedAuthenticationLevel )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out short *psReceivedAuthenticationLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ResponseQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_ResponseQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo3 *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_AdminQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo3 **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_AdminQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo3 *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_ResponseDestination)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseDestination )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, putref_ResponseDestination)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseDestination )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in_opt IDispatch *pdestResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_Destination)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Destination )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestDestination);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_LookupId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_LookupId )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarLookupId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_IsAuthenticated2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsAuthenticated2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisAuthenticated);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_IsFirstInTransaction2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstInTransaction2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisFirstInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_IsLastInTransaction2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsLastInTransaction2 )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisLastInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, AttachCurrentSecurityContext2)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *AttachCurrentSecurityContext2 )( 
            __RPC__in IMSMQMessage3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_SoapEnvelope)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SoapEnvelope )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSoapEnvelope);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, get_CompoundMessage)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CompoundMessage )( 
            __RPC__in IMSMQMessage3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCompoundMessage);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_SoapHeader)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SoapHeader )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in BSTR bstrSoapHeader);
        
        DECLSPEC_XFGVIRT(IMSMQMessage3, put_SoapBody)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SoapBody )( 
            __RPC__in IMSMQMessage3 * This,
            /* [in] */ __RPC__in BSTR bstrSoapBody);
        
        END_INTERFACE
    } IMSMQMessage3Vtbl;

    interface IMSMQMessage3
    {
        CONST_VTBL struct IMSMQMessage3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQMessage3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQMessage3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQMessage3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQMessage3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQMessage3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQMessage3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQMessage3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQMessage3_get_Class(This,plClass)	\
    ( (This)->lpVtbl -> get_Class(This,plClass) ) 

#define IMSMQMessage3_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQMessage3_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQMessage3_get_AuthLevel(This,plAuthLevel)	\
    ( (This)->lpVtbl -> get_AuthLevel(This,plAuthLevel) ) 

#define IMSMQMessage3_put_AuthLevel(This,lAuthLevel)	\
    ( (This)->lpVtbl -> put_AuthLevel(This,lAuthLevel) ) 

#define IMSMQMessage3_get_IsAuthenticated(This,pisAuthenticated)	\
    ( (This)->lpVtbl -> get_IsAuthenticated(This,pisAuthenticated) ) 

#define IMSMQMessage3_get_Delivery(This,plDelivery)	\
    ( (This)->lpVtbl -> get_Delivery(This,plDelivery) ) 

#define IMSMQMessage3_put_Delivery(This,lDelivery)	\
    ( (This)->lpVtbl -> put_Delivery(This,lDelivery) ) 

#define IMSMQMessage3_get_Trace(This,plTrace)	\
    ( (This)->lpVtbl -> get_Trace(This,plTrace) ) 

#define IMSMQMessage3_put_Trace(This,lTrace)	\
    ( (This)->lpVtbl -> put_Trace(This,lTrace) ) 

#define IMSMQMessage3_get_Priority(This,plPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,plPriority) ) 

#define IMSMQMessage3_put_Priority(This,lPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lPriority) ) 

#define IMSMQMessage3_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQMessage3_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQMessage3_get_ResponseQueueInfo_v1(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo_v1(This,ppqinfoResponse) ) 

#define IMSMQMessage3_putref_ResponseQueueInfo_v1(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo_v1(This,pqinfoResponse) ) 

#define IMSMQMessage3_get_AppSpecific(This,plAppSpecific)	\
    ( (This)->lpVtbl -> get_AppSpecific(This,plAppSpecific) ) 

#define IMSMQMessage3_put_AppSpecific(This,lAppSpecific)	\
    ( (This)->lpVtbl -> put_AppSpecific(This,lAppSpecific) ) 

#define IMSMQMessage3_get_SourceMachineGuid(This,pbstrGuidSrcMachine)	\
    ( (This)->lpVtbl -> get_SourceMachineGuid(This,pbstrGuidSrcMachine) ) 

#define IMSMQMessage3_get_BodyLength(This,pcbBody)	\
    ( (This)->lpVtbl -> get_BodyLength(This,pcbBody) ) 

#define IMSMQMessage3_get_Body(This,pvarBody)	\
    ( (This)->lpVtbl -> get_Body(This,pvarBody) ) 

#define IMSMQMessage3_put_Body(This,varBody)	\
    ( (This)->lpVtbl -> put_Body(This,varBody) ) 

#define IMSMQMessage3_get_AdminQueueInfo_v1(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo_v1(This,ppqinfoAdmin) ) 

#define IMSMQMessage3_putref_AdminQueueInfo_v1(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo_v1(This,pqinfoAdmin) ) 

#define IMSMQMessage3_get_Id(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_Id(This,pvarMsgId) ) 

#define IMSMQMessage3_get_CorrelationId(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_CorrelationId(This,pvarMsgId) ) 

#define IMSMQMessage3_put_CorrelationId(This,varMsgId)	\
    ( (This)->lpVtbl -> put_CorrelationId(This,varMsgId) ) 

#define IMSMQMessage3_get_Ack(This,plAck)	\
    ( (This)->lpVtbl -> get_Ack(This,plAck) ) 

#define IMSMQMessage3_put_Ack(This,lAck)	\
    ( (This)->lpVtbl -> put_Ack(This,lAck) ) 

#define IMSMQMessage3_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQMessage3_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQMessage3_get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue) ) 

#define IMSMQMessage3_put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue) ) 

#define IMSMQMessage3_get_MaxTimeToReceive(This,plMaxTimeToReceive)	\
    ( (This)->lpVtbl -> get_MaxTimeToReceive(This,plMaxTimeToReceive) ) 

#define IMSMQMessage3_put_MaxTimeToReceive(This,lMaxTimeToReceive)	\
    ( (This)->lpVtbl -> put_MaxTimeToReceive(This,lMaxTimeToReceive) ) 

#define IMSMQMessage3_get_HashAlgorithm(This,plHashAlg)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,plHashAlg) ) 

#define IMSMQMessage3_put_HashAlgorithm(This,lHashAlg)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,lHashAlg) ) 

#define IMSMQMessage3_get_EncryptAlgorithm(This,plEncryptAlg)	\
    ( (This)->lpVtbl -> get_EncryptAlgorithm(This,plEncryptAlg) ) 

#define IMSMQMessage3_put_EncryptAlgorithm(This,lEncryptAlg)	\
    ( (This)->lpVtbl -> put_EncryptAlgorithm(This,lEncryptAlg) ) 

#define IMSMQMessage3_get_SentTime(This,pvarSentTime)	\
    ( (This)->lpVtbl -> get_SentTime(This,pvarSentTime) ) 

#define IMSMQMessage3_get_ArrivedTime(This,plArrivedTime)	\
    ( (This)->lpVtbl -> get_ArrivedTime(This,plArrivedTime) ) 

#define IMSMQMessage3_get_DestinationQueueInfo(This,ppqinfoDest)	\
    ( (This)->lpVtbl -> get_DestinationQueueInfo(This,ppqinfoDest) ) 

#define IMSMQMessage3_get_SenderCertificate(This,pvarSenderCert)	\
    ( (This)->lpVtbl -> get_SenderCertificate(This,pvarSenderCert) ) 

#define IMSMQMessage3_put_SenderCertificate(This,varSenderCert)	\
    ( (This)->lpVtbl -> put_SenderCertificate(This,varSenderCert) ) 

#define IMSMQMessage3_get_SenderId(This,pvarSenderId)	\
    ( (This)->lpVtbl -> get_SenderId(This,pvarSenderId) ) 

#define IMSMQMessage3_get_SenderIdType(This,plSenderIdType)	\
    ( (This)->lpVtbl -> get_SenderIdType(This,plSenderIdType) ) 

#define IMSMQMessage3_put_SenderIdType(This,lSenderIdType)	\
    ( (This)->lpVtbl -> put_SenderIdType(This,lSenderIdType) ) 

#define IMSMQMessage3_Send(This,DestinationQueue,Transaction)	\
    ( (This)->lpVtbl -> Send(This,DestinationQueue,Transaction) ) 

#define IMSMQMessage3_AttachCurrentSecurityContext(This)	\
    ( (This)->lpVtbl -> AttachCurrentSecurityContext(This) ) 

#define IMSMQMessage3_get_SenderVersion(This,plSenderVersion)	\
    ( (This)->lpVtbl -> get_SenderVersion(This,plSenderVersion) ) 

#define IMSMQMessage3_get_Extension(This,pvarExtension)	\
    ( (This)->lpVtbl -> get_Extension(This,pvarExtension) ) 

#define IMSMQMessage3_put_Extension(This,varExtension)	\
    ( (This)->lpVtbl -> put_Extension(This,varExtension) ) 

#define IMSMQMessage3_get_ConnectorTypeGuid(This,pbstrGuidConnectorType)	\
    ( (This)->lpVtbl -> get_ConnectorTypeGuid(This,pbstrGuidConnectorType) ) 

#define IMSMQMessage3_put_ConnectorTypeGuid(This,bstrGuidConnectorType)	\
    ( (This)->lpVtbl -> put_ConnectorTypeGuid(This,bstrGuidConnectorType) ) 

#define IMSMQMessage3_get_TransactionStatusQueueInfo(This,ppqinfoXactStatus)	\
    ( (This)->lpVtbl -> get_TransactionStatusQueueInfo(This,ppqinfoXactStatus) ) 

#define IMSMQMessage3_get_DestinationSymmetricKey(This,pvarDestSymmKey)	\
    ( (This)->lpVtbl -> get_DestinationSymmetricKey(This,pvarDestSymmKey) ) 

#define IMSMQMessage3_put_DestinationSymmetricKey(This,varDestSymmKey)	\
    ( (This)->lpVtbl -> put_DestinationSymmetricKey(This,varDestSymmKey) ) 

#define IMSMQMessage3_get_Signature(This,pvarSignature)	\
    ( (This)->lpVtbl -> get_Signature(This,pvarSignature) ) 

#define IMSMQMessage3_put_Signature(This,varSignature)	\
    ( (This)->lpVtbl -> put_Signature(This,varSignature) ) 

#define IMSMQMessage3_get_AuthenticationProviderType(This,plAuthProvType)	\
    ( (This)->lpVtbl -> get_AuthenticationProviderType(This,plAuthProvType) ) 

#define IMSMQMessage3_put_AuthenticationProviderType(This,lAuthProvType)	\
    ( (This)->lpVtbl -> put_AuthenticationProviderType(This,lAuthProvType) ) 

#define IMSMQMessage3_get_AuthenticationProviderName(This,pbstrAuthProvName)	\
    ( (This)->lpVtbl -> get_AuthenticationProviderName(This,pbstrAuthProvName) ) 

#define IMSMQMessage3_put_AuthenticationProviderName(This,bstrAuthProvName)	\
    ( (This)->lpVtbl -> put_AuthenticationProviderName(This,bstrAuthProvName) ) 

#define IMSMQMessage3_put_SenderId(This,varSenderId)	\
    ( (This)->lpVtbl -> put_SenderId(This,varSenderId) ) 

#define IMSMQMessage3_get_MsgClass(This,plMsgClass)	\
    ( (This)->lpVtbl -> get_MsgClass(This,plMsgClass) ) 

#define IMSMQMessage3_put_MsgClass(This,lMsgClass)	\
    ( (This)->lpVtbl -> put_MsgClass(This,lMsgClass) ) 

#define IMSMQMessage3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQMessage3_get_TransactionId(This,pvarXactId)	\
    ( (This)->lpVtbl -> get_TransactionId(This,pvarXactId) ) 

#define IMSMQMessage3_get_IsFirstInTransaction(This,pisFirstInXact)	\
    ( (This)->lpVtbl -> get_IsFirstInTransaction(This,pisFirstInXact) ) 

#define IMSMQMessage3_get_IsLastInTransaction(This,pisLastInXact)	\
    ( (This)->lpVtbl -> get_IsLastInTransaction(This,pisLastInXact) ) 

#define IMSMQMessage3_get_ResponseQueueInfo_v2(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo_v2(This,ppqinfoResponse) ) 

#define IMSMQMessage3_putref_ResponseQueueInfo_v2(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo_v2(This,pqinfoResponse) ) 

#define IMSMQMessage3_get_AdminQueueInfo_v2(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo_v2(This,ppqinfoAdmin) ) 

#define IMSMQMessage3_putref_AdminQueueInfo_v2(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo_v2(This,pqinfoAdmin) ) 

#define IMSMQMessage3_get_ReceivedAuthenticationLevel(This,psReceivedAuthenticationLevel)	\
    ( (This)->lpVtbl -> get_ReceivedAuthenticationLevel(This,psReceivedAuthenticationLevel) ) 

#define IMSMQMessage3_get_ResponseQueueInfo(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo(This,ppqinfoResponse) ) 

#define IMSMQMessage3_putref_ResponseQueueInfo(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo(This,pqinfoResponse) ) 

#define IMSMQMessage3_get_AdminQueueInfo(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo(This,ppqinfoAdmin) ) 

#define IMSMQMessage3_putref_AdminQueueInfo(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo(This,pqinfoAdmin) ) 

#define IMSMQMessage3_get_ResponseDestination(This,ppdestResponse)	\
    ( (This)->lpVtbl -> get_ResponseDestination(This,ppdestResponse) ) 

#define IMSMQMessage3_putref_ResponseDestination(This,pdestResponse)	\
    ( (This)->lpVtbl -> putref_ResponseDestination(This,pdestResponse) ) 

#define IMSMQMessage3_get_Destination(This,ppdestDestination)	\
    ( (This)->lpVtbl -> get_Destination(This,ppdestDestination) ) 

#define IMSMQMessage3_get_LookupId(This,pvarLookupId)	\
    ( (This)->lpVtbl -> get_LookupId(This,pvarLookupId) ) 

#define IMSMQMessage3_get_IsAuthenticated2(This,pisAuthenticated)	\
    ( (This)->lpVtbl -> get_IsAuthenticated2(This,pisAuthenticated) ) 

#define IMSMQMessage3_get_IsFirstInTransaction2(This,pisFirstInXact)	\
    ( (This)->lpVtbl -> get_IsFirstInTransaction2(This,pisFirstInXact) ) 

#define IMSMQMessage3_get_IsLastInTransaction2(This,pisLastInXact)	\
    ( (This)->lpVtbl -> get_IsLastInTransaction2(This,pisLastInXact) ) 

#define IMSMQMessage3_AttachCurrentSecurityContext2(This)	\
    ( (This)->lpVtbl -> AttachCurrentSecurityContext2(This) ) 

#define IMSMQMessage3_get_SoapEnvelope(This,pbstrSoapEnvelope)	\
    ( (This)->lpVtbl -> get_SoapEnvelope(This,pbstrSoapEnvelope) ) 

#define IMSMQMessage3_get_CompoundMessage(This,pvarCompoundMessage)	\
    ( (This)->lpVtbl -> get_CompoundMessage(This,pvarCompoundMessage) ) 

#define IMSMQMessage3_put_SoapHeader(This,bstrSoapHeader)	\
    ( (This)->lpVtbl -> put_SoapHeader(This,bstrSoapHeader) ) 

#define IMSMQMessage3_put_SoapBody(This,bstrSoapBody)	\
    ( (This)->lpVtbl -> put_SoapBody(This,bstrSoapBody) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQMessage3_INTERFACE_DEFINED__ */


#ifndef __IMSMQMessage4_INTERFACE_DEFINED__
#define __IMSMQMessage4_INTERFACE_DEFINED__

/* interface IMSMQMessage4 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQMessage4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b23-2168-11d3-898c-00e02c074f6b")
    IMSMQMessage4 : public IDispatch
    {
    public:
        virtual /* [id][propget][hidden][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Class( 
            /* [retval][out] */ __RPC__out long *plClass) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivLevel( 
            /* [retval][out] */ __RPC__out long *plPrivLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PrivLevel( 
            /* [in] */ long lPrivLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthLevel( 
            /* [retval][out] */ __RPC__out long *plAuthLevel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthLevel( 
            /* [in] */ long lAuthLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsAuthenticated( 
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Delivery( 
            /* [retval][out] */ __RPC__out long *plDelivery) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Delivery( 
            /* [in] */ long lDelivery) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Trace( 
            /* [retval][out] */ __RPC__out long *plTrace) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Trace( 
            /* [in] */ long lTrace) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *plPriority) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lPriority) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Journal( 
            /* [retval][out] */ __RPC__out long *plJournal) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Journal( 
            /* [in] */ long lJournal) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo_v1( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo_v1( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AppSpecific( 
            /* [retval][out] */ __RPC__out long *plAppSpecific) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AppSpecific( 
            /* [in] */ long lAppSpecific) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SourceMachineGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BodyLength( 
            /* [retval][out] */ __RPC__out long *pcbBody) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ __RPC__out VARIANT *pvarBody) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Body( 
            /* [in] */ VARIANT varBody) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo_v1( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo_v1( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CorrelationId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_CorrelationId( 
            /* [in] */ VARIANT varMsgId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Ack( 
            /* [retval][out] */ __RPC__out long *plAck) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Ack( 
            /* [in] */ long lAck) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Label( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Label( 
            /* [in] */ __RPC__in BSTR bstrLabel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReachQueue( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReachQueue( 
            /* [in] */ long lMaxTimeToReachQueue) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MaxTimeToReceive( 
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MaxTimeToReceive( 
            /* [in] */ long lMaxTimeToReceive) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_HashAlgorithm( 
            /* [retval][out] */ __RPC__out long *plHashAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_HashAlgorithm( 
            /* [in] */ long lHashAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_EncryptAlgorithm( 
            /* [retval][out] */ __RPC__out long *plEncryptAlg) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_EncryptAlgorithm( 
            /* [in] */ long lEncryptAlg) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SentTime( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ArrivedTime( 
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoDest) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderCertificate( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderCertificate( 
            /* [in] */ VARIANT varSenderCert) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderIdType( 
            /* [retval][out] */ __RPC__out long *plSenderIdType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderIdType( 
            /* [in] */ long lSenderIdType) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Send( 
            /* [in] */ __RPC__in_opt IDispatch *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction) = 0;
        
        virtual /* [helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE AttachCurrentSecurityContext( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SenderVersion( 
            /* [retval][out] */ __RPC__out long *plSenderVersion) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Extension( 
            /* [retval][out] */ __RPC__out VARIANT *pvarExtension) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Extension( 
            /* [in] */ VARIANT varExtension) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ConnectorTypeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidConnectorType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ConnectorTypeGuid( 
            /* [in] */ __RPC__in BSTR bstrGuidConnectorType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionStatusQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoXactStatus) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DestinationSymmetricKey( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDestSymmKey) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_DestinationSymmetricKey( 
            /* [in] */ VARIANT varDestSymmKey) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Signature( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSignature) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Signature( 
            /* [in] */ VARIANT varSignature) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthenticationProviderType( 
            /* [retval][out] */ __RPC__out long *plAuthProvType) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthenticationProviderType( 
            /* [in] */ long lAuthProvType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AuthenticationProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAuthProvName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_AuthenticationProviderName( 
            /* [in] */ __RPC__in BSTR bstrAuthProvName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SenderId( 
            /* [in] */ VARIANT varSenderId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MsgClass( 
            /* [retval][out] */ __RPC__out long *plMsgClass) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_MsgClass( 
            /* [in] */ long lMsgClass) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarXactId) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsFirstInTransaction( 
            /* [retval][out] */ __RPC__out Boolean *pisFirstInXact) = 0;
        
        virtual /* [id][propget][helpstringcontext][hidden] */ HRESULT STDMETHODCALLTYPE get_IsLastInTransaction( 
            /* [retval][out] */ __RPC__out Boolean *pisLastInXact) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo_v2( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoResponse) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo_v2( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoResponse) = 0;
        
        virtual /* [hidden][id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo_v2( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoAdmin) = 0;
        
        virtual /* [hidden][id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo_v2( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ReceivedAuthenticationLevel( 
            /* [retval][out] */ __RPC__out short *psReceivedAuthenticationLevel) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoResponse) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo4 *pqinfoResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_AdminQueueInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoAdmin) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_AdminQueueInfo( 
            /* [in] */ __RPC__in_opt IMSMQQueueInfo4 *pqinfoAdmin) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ResponseDestination( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestResponse) = 0;
        
        virtual /* [id][propputref][helpstringcontext] */ HRESULT STDMETHODCALLTYPE putref_ResponseDestination( 
            /* [in] */ __RPC__in_opt IDispatch *pdestResponse) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Destination( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestDestination) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_LookupId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarLookupId) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsAuthenticated2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisAuthenticated) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsFirstInTransaction2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisFirstInXact) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsLastInTransaction2( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisLastInXact) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE AttachCurrentSecurityContext2( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_SoapEnvelope( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSoapEnvelope) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_CompoundMessage( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCompoundMessage) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SoapHeader( 
            /* [in] */ __RPC__in BSTR bstrSoapHeader) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_SoapBody( 
            /* [in] */ __RPC__in BSTR bstrSoapBody) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQMessage4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQMessage4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQMessage4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQMessage4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQMessage4 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Class)
        /* [id][propget][hidden][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_PrivLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivLevel )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_PrivLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PrivLevel )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lPrivLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AuthLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthLevel )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_AuthLevel)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthLevel )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lAuthLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_IsAuthenticated)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsAuthenticated )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out Boolean *pisAuthenticated);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Delivery)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Delivery )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Delivery)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Delivery )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lDelivery);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Trace)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Trace )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Trace)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Trace )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lTrace);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Priority)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Priority)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lPriority);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Journal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Journal )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Journal)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Journal )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lJournal);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ResponseQueueInfo_v1)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo_v1 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_ResponseQueueInfo_v1)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo_v1 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AppSpecific)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AppSpecific )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_AppSpecific)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AppSpecific )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lAppSpecific);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SourceMachineGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SourceMachineGuid )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidSrcMachine);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_BodyLength)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BodyLength )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *pcbBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Body)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Body)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AdminQueueInfo_v1)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo_v1 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_AdminQueueInfo_v1)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo_v1 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Id)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_CorrelationId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CorrelationId )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_CorrelationId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_CorrelationId )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varMsgId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Ack)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Ack )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Ack)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Ack )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lAck);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Label)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Label )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Label)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Label )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in BSTR bstrLabel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_MaxTimeToReachQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_MaxTimeToReachQueue)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReachQueue )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lMaxTimeToReachQueue);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_MaxTimeToReceive)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_MaxTimeToReceive)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MaxTimeToReceive )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lMaxTimeToReceive);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_HashAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_HashAlgorithm )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_HashAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_HashAlgorithm )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lHashAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_EncryptAlgorithm)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_EncryptAlgorithm)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_EncryptAlgorithm )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lEncryptAlg);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SentTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SentTime )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSentTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ArrivedTime)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ArrivedTime )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *plArrivedTime);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_DestinationQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationQueueInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoDest);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SenderCertificate)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderCertificate )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_SenderCertificate)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderCertificate )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varSenderCert);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SenderId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderId )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SenderIdType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderIdType )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_SenderIdType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderIdType )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lSenderIdType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, Send)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IDispatch *DestinationQueue,
            /* [optional][in] */ __RPC__in VARIANT *Transaction);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, AttachCurrentSecurityContext)
        /* [helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *AttachCurrentSecurityContext )( 
            __RPC__in IMSMQMessage4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SenderVersion)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SenderVersion )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plSenderVersion);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Extension)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Extension )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarExtension);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Extension)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Extension )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varExtension);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ConnectorTypeGuid)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectorTypeGuid )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuidConnectorType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_ConnectorTypeGuid)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectorTypeGuid )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in BSTR bstrGuidConnectorType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_TransactionStatusQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionStatusQueueInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoXactStatus);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_DestinationSymmetricKey)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationSymmetricKey )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDestSymmKey);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_DestinationSymmetricKey)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_DestinationSymmetricKey )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varDestSymmKey);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Signature)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Signature )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSignature);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_Signature)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Signature )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varSignature);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AuthenticationProviderType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationProviderType )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plAuthProvType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_AuthenticationProviderType)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationProviderType )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lAuthProvType);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AuthenticationProviderName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationProviderName )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAuthProvName);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_AuthenticationProviderName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationProviderName )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in BSTR bstrAuthProvName);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_SenderId)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SenderId )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ VARIANT varSenderId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_MsgClass)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MsgClass )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out long *plMsgClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_MsgClass)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_MsgClass )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ long lMsgClass);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_TransactionId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionId )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarXactId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_IsFirstInTransaction)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstInTransaction )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out Boolean *pisFirstInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_IsLastInTransaction)
        /* [id][propget][helpstringcontext][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IsLastInTransaction )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out Boolean *pisLastInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ResponseQueueInfo_v2)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo_v2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_ResponseQueueInfo_v2)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo_v2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AdminQueueInfo_v2)
        /* [hidden][id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo_v2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo2 **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_AdminQueueInfo_v2)
        /* [hidden][id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo_v2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo2 *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ReceivedAuthenticationLevel)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ReceivedAuthenticationLevel )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out short *psReceivedAuthenticationLevel);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ResponseQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_ResponseQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseQueueInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo4 *pqinfoResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_AdminQueueInfo)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_AdminQueueInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQQueueInfo4 **ppqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_AdminQueueInfo)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_AdminQueueInfo )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IMSMQQueueInfo4 *pqinfoAdmin);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_ResponseDestination)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ResponseDestination )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, putref_ResponseDestination)
        /* [id][propputref][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *putref_ResponseDestination )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in_opt IDispatch *pdestResponse);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_Destination)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Destination )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdestDestination);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_LookupId)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_LookupId )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarLookupId);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_IsAuthenticated2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsAuthenticated2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisAuthenticated);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_IsFirstInTransaction2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsFirstInTransaction2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisFirstInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_IsLastInTransaction2)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsLastInTransaction2 )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pisLastInXact);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, AttachCurrentSecurityContext2)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *AttachCurrentSecurityContext2 )( 
            __RPC__in IMSMQMessage4 * This);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_SoapEnvelope)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_SoapEnvelope )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSoapEnvelope);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, get_CompoundMessage)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_CompoundMessage )( 
            __RPC__in IMSMQMessage4 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCompoundMessage);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_SoapHeader)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SoapHeader )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in BSTR bstrSoapHeader);
        
        DECLSPEC_XFGVIRT(IMSMQMessage4, put_SoapBody)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_SoapBody )( 
            __RPC__in IMSMQMessage4 * This,
            /* [in] */ __RPC__in BSTR bstrSoapBody);
        
        END_INTERFACE
    } IMSMQMessage4Vtbl;

    interface IMSMQMessage4
    {
        CONST_VTBL struct IMSMQMessage4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQMessage4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQMessage4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQMessage4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQMessage4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQMessage4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQMessage4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQMessage4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQMessage4_get_Class(This,plClass)	\
    ( (This)->lpVtbl -> get_Class(This,plClass) ) 

#define IMSMQMessage4_get_PrivLevel(This,plPrivLevel)	\
    ( (This)->lpVtbl -> get_PrivLevel(This,plPrivLevel) ) 

#define IMSMQMessage4_put_PrivLevel(This,lPrivLevel)	\
    ( (This)->lpVtbl -> put_PrivLevel(This,lPrivLevel) ) 

#define IMSMQMessage4_get_AuthLevel(This,plAuthLevel)	\
    ( (This)->lpVtbl -> get_AuthLevel(This,plAuthLevel) ) 

#define IMSMQMessage4_put_AuthLevel(This,lAuthLevel)	\
    ( (This)->lpVtbl -> put_AuthLevel(This,lAuthLevel) ) 

#define IMSMQMessage4_get_IsAuthenticated(This,pisAuthenticated)	\
    ( (This)->lpVtbl -> get_IsAuthenticated(This,pisAuthenticated) ) 

#define IMSMQMessage4_get_Delivery(This,plDelivery)	\
    ( (This)->lpVtbl -> get_Delivery(This,plDelivery) ) 

#define IMSMQMessage4_put_Delivery(This,lDelivery)	\
    ( (This)->lpVtbl -> put_Delivery(This,lDelivery) ) 

#define IMSMQMessage4_get_Trace(This,plTrace)	\
    ( (This)->lpVtbl -> get_Trace(This,plTrace) ) 

#define IMSMQMessage4_put_Trace(This,lTrace)	\
    ( (This)->lpVtbl -> put_Trace(This,lTrace) ) 

#define IMSMQMessage4_get_Priority(This,plPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,plPriority) ) 

#define IMSMQMessage4_put_Priority(This,lPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lPriority) ) 

#define IMSMQMessage4_get_Journal(This,plJournal)	\
    ( (This)->lpVtbl -> get_Journal(This,plJournal) ) 

#define IMSMQMessage4_put_Journal(This,lJournal)	\
    ( (This)->lpVtbl -> put_Journal(This,lJournal) ) 

#define IMSMQMessage4_get_ResponseQueueInfo_v1(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo_v1(This,ppqinfoResponse) ) 

#define IMSMQMessage4_putref_ResponseQueueInfo_v1(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo_v1(This,pqinfoResponse) ) 

#define IMSMQMessage4_get_AppSpecific(This,plAppSpecific)	\
    ( (This)->lpVtbl -> get_AppSpecific(This,plAppSpecific) ) 

#define IMSMQMessage4_put_AppSpecific(This,lAppSpecific)	\
    ( (This)->lpVtbl -> put_AppSpecific(This,lAppSpecific) ) 

#define IMSMQMessage4_get_SourceMachineGuid(This,pbstrGuidSrcMachine)	\
    ( (This)->lpVtbl -> get_SourceMachineGuid(This,pbstrGuidSrcMachine) ) 

#define IMSMQMessage4_get_BodyLength(This,pcbBody)	\
    ( (This)->lpVtbl -> get_BodyLength(This,pcbBody) ) 

#define IMSMQMessage4_get_Body(This,pvarBody)	\
    ( (This)->lpVtbl -> get_Body(This,pvarBody) ) 

#define IMSMQMessage4_put_Body(This,varBody)	\
    ( (This)->lpVtbl -> put_Body(This,varBody) ) 

#define IMSMQMessage4_get_AdminQueueInfo_v1(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo_v1(This,ppqinfoAdmin) ) 

#define IMSMQMessage4_putref_AdminQueueInfo_v1(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo_v1(This,pqinfoAdmin) ) 

#define IMSMQMessage4_get_Id(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_Id(This,pvarMsgId) ) 

#define IMSMQMessage4_get_CorrelationId(This,pvarMsgId)	\
    ( (This)->lpVtbl -> get_CorrelationId(This,pvarMsgId) ) 

#define IMSMQMessage4_put_CorrelationId(This,varMsgId)	\
    ( (This)->lpVtbl -> put_CorrelationId(This,varMsgId) ) 

#define IMSMQMessage4_get_Ack(This,plAck)	\
    ( (This)->lpVtbl -> get_Ack(This,plAck) ) 

#define IMSMQMessage4_put_Ack(This,lAck)	\
    ( (This)->lpVtbl -> put_Ack(This,lAck) ) 

#define IMSMQMessage4_get_Label(This,pbstrLabel)	\
    ( (This)->lpVtbl -> get_Label(This,pbstrLabel) ) 

#define IMSMQMessage4_put_Label(This,bstrLabel)	\
    ( (This)->lpVtbl -> put_Label(This,bstrLabel) ) 

#define IMSMQMessage4_get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> get_MaxTimeToReachQueue(This,plMaxTimeToReachQueue) ) 

#define IMSMQMessage4_put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue)	\
    ( (This)->lpVtbl -> put_MaxTimeToReachQueue(This,lMaxTimeToReachQueue) ) 

#define IMSMQMessage4_get_MaxTimeToReceive(This,plMaxTimeToReceive)	\
    ( (This)->lpVtbl -> get_MaxTimeToReceive(This,plMaxTimeToReceive) ) 

#define IMSMQMessage4_put_MaxTimeToReceive(This,lMaxTimeToReceive)	\
    ( (This)->lpVtbl -> put_MaxTimeToReceive(This,lMaxTimeToReceive) ) 

#define IMSMQMessage4_get_HashAlgorithm(This,plHashAlg)	\
    ( (This)->lpVtbl -> get_HashAlgorithm(This,plHashAlg) ) 

#define IMSMQMessage4_put_HashAlgorithm(This,lHashAlg)	\
    ( (This)->lpVtbl -> put_HashAlgorithm(This,lHashAlg) ) 

#define IMSMQMessage4_get_EncryptAlgorithm(This,plEncryptAlg)	\
    ( (This)->lpVtbl -> get_EncryptAlgorithm(This,plEncryptAlg) ) 

#define IMSMQMessage4_put_EncryptAlgorithm(This,lEncryptAlg)	\
    ( (This)->lpVtbl -> put_EncryptAlgorithm(This,lEncryptAlg) ) 

#define IMSMQMessage4_get_SentTime(This,pvarSentTime)	\
    ( (This)->lpVtbl -> get_SentTime(This,pvarSentTime) ) 

#define IMSMQMessage4_get_ArrivedTime(This,plArrivedTime)	\
    ( (This)->lpVtbl -> get_ArrivedTime(This,plArrivedTime) ) 

#define IMSMQMessage4_get_DestinationQueueInfo(This,ppqinfoDest)	\
    ( (This)->lpVtbl -> get_DestinationQueueInfo(This,ppqinfoDest) ) 

#define IMSMQMessage4_get_SenderCertificate(This,pvarSenderCert)	\
    ( (This)->lpVtbl -> get_SenderCertificate(This,pvarSenderCert) ) 

#define IMSMQMessage4_put_SenderCertificate(This,varSenderCert)	\
    ( (This)->lpVtbl -> put_SenderCertificate(This,varSenderCert) ) 

#define IMSMQMessage4_get_SenderId(This,pvarSenderId)	\
    ( (This)->lpVtbl -> get_SenderId(This,pvarSenderId) ) 

#define IMSMQMessage4_get_SenderIdType(This,plSenderIdType)	\
    ( (This)->lpVtbl -> get_SenderIdType(This,plSenderIdType) ) 

#define IMSMQMessage4_put_SenderIdType(This,lSenderIdType)	\
    ( (This)->lpVtbl -> put_SenderIdType(This,lSenderIdType) ) 

#define IMSMQMessage4_Send(This,DestinationQueue,Transaction)	\
    ( (This)->lpVtbl -> Send(This,DestinationQueue,Transaction) ) 

#define IMSMQMessage4_AttachCurrentSecurityContext(This)	\
    ( (This)->lpVtbl -> AttachCurrentSecurityContext(This) ) 

#define IMSMQMessage4_get_SenderVersion(This,plSenderVersion)	\
    ( (This)->lpVtbl -> get_SenderVersion(This,plSenderVersion) ) 

#define IMSMQMessage4_get_Extension(This,pvarExtension)	\
    ( (This)->lpVtbl -> get_Extension(This,pvarExtension) ) 

#define IMSMQMessage4_put_Extension(This,varExtension)	\
    ( (This)->lpVtbl -> put_Extension(This,varExtension) ) 

#define IMSMQMessage4_get_ConnectorTypeGuid(This,pbstrGuidConnectorType)	\
    ( (This)->lpVtbl -> get_ConnectorTypeGuid(This,pbstrGuidConnectorType) ) 

#define IMSMQMessage4_put_ConnectorTypeGuid(This,bstrGuidConnectorType)	\
    ( (This)->lpVtbl -> put_ConnectorTypeGuid(This,bstrGuidConnectorType) ) 

#define IMSMQMessage4_get_TransactionStatusQueueInfo(This,ppqinfoXactStatus)	\
    ( (This)->lpVtbl -> get_TransactionStatusQueueInfo(This,ppqinfoXactStatus) ) 

#define IMSMQMessage4_get_DestinationSymmetricKey(This,pvarDestSymmKey)	\
    ( (This)->lpVtbl -> get_DestinationSymmetricKey(This,pvarDestSymmKey) ) 

#define IMSMQMessage4_put_DestinationSymmetricKey(This,varDestSymmKey)	\
    ( (This)->lpVtbl -> put_DestinationSymmetricKey(This,varDestSymmKey) ) 

#define IMSMQMessage4_get_Signature(This,pvarSignature)	\
    ( (This)->lpVtbl -> get_Signature(This,pvarSignature) ) 

#define IMSMQMessage4_put_Signature(This,varSignature)	\
    ( (This)->lpVtbl -> put_Signature(This,varSignature) ) 

#define IMSMQMessage4_get_AuthenticationProviderType(This,plAuthProvType)	\
    ( (This)->lpVtbl -> get_AuthenticationProviderType(This,plAuthProvType) ) 

#define IMSMQMessage4_put_AuthenticationProviderType(This,lAuthProvType)	\
    ( (This)->lpVtbl -> put_AuthenticationProviderType(This,lAuthProvType) ) 

#define IMSMQMessage4_get_AuthenticationProviderName(This,pbstrAuthProvName)	\
    ( (This)->lpVtbl -> get_AuthenticationProviderName(This,pbstrAuthProvName) ) 

#define IMSMQMessage4_put_AuthenticationProviderName(This,bstrAuthProvName)	\
    ( (This)->lpVtbl -> put_AuthenticationProviderName(This,bstrAuthProvName) ) 

#define IMSMQMessage4_put_SenderId(This,varSenderId)	\
    ( (This)->lpVtbl -> put_SenderId(This,varSenderId) ) 

#define IMSMQMessage4_get_MsgClass(This,plMsgClass)	\
    ( (This)->lpVtbl -> get_MsgClass(This,plMsgClass) ) 

#define IMSMQMessage4_put_MsgClass(This,lMsgClass)	\
    ( (This)->lpVtbl -> put_MsgClass(This,lMsgClass) ) 

#define IMSMQMessage4_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#define IMSMQMessage4_get_TransactionId(This,pvarXactId)	\
    ( (This)->lpVtbl -> get_TransactionId(This,pvarXactId) ) 

#define IMSMQMessage4_get_IsFirstInTransaction(This,pisFirstInXact)	\
    ( (This)->lpVtbl -> get_IsFirstInTransaction(This,pisFirstInXact) ) 

#define IMSMQMessage4_get_IsLastInTransaction(This,pisLastInXact)	\
    ( (This)->lpVtbl -> get_IsLastInTransaction(This,pisLastInXact) ) 

#define IMSMQMessage4_get_ResponseQueueInfo_v2(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo_v2(This,ppqinfoResponse) ) 

#define IMSMQMessage4_putref_ResponseQueueInfo_v2(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo_v2(This,pqinfoResponse) ) 

#define IMSMQMessage4_get_AdminQueueInfo_v2(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo_v2(This,ppqinfoAdmin) ) 

#define IMSMQMessage4_putref_AdminQueueInfo_v2(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo_v2(This,pqinfoAdmin) ) 

#define IMSMQMessage4_get_ReceivedAuthenticationLevel(This,psReceivedAuthenticationLevel)	\
    ( (This)->lpVtbl -> get_ReceivedAuthenticationLevel(This,psReceivedAuthenticationLevel) ) 

#define IMSMQMessage4_get_ResponseQueueInfo(This,ppqinfoResponse)	\
    ( (This)->lpVtbl -> get_ResponseQueueInfo(This,ppqinfoResponse) ) 

#define IMSMQMessage4_putref_ResponseQueueInfo(This,pqinfoResponse)	\
    ( (This)->lpVtbl -> putref_ResponseQueueInfo(This,pqinfoResponse) ) 

#define IMSMQMessage4_get_AdminQueueInfo(This,ppqinfoAdmin)	\
    ( (This)->lpVtbl -> get_AdminQueueInfo(This,ppqinfoAdmin) ) 

#define IMSMQMessage4_putref_AdminQueueInfo(This,pqinfoAdmin)	\
    ( (This)->lpVtbl -> putref_AdminQueueInfo(This,pqinfoAdmin) ) 

#define IMSMQMessage4_get_ResponseDestination(This,ppdestResponse)	\
    ( (This)->lpVtbl -> get_ResponseDestination(This,ppdestResponse) ) 

#define IMSMQMessage4_putref_ResponseDestination(This,pdestResponse)	\
    ( (This)->lpVtbl -> putref_ResponseDestination(This,pdestResponse) ) 

#define IMSMQMessage4_get_Destination(This,ppdestDestination)	\
    ( (This)->lpVtbl -> get_Destination(This,ppdestDestination) ) 

#define IMSMQMessage4_get_LookupId(This,pvarLookupId)	\
    ( (This)->lpVtbl -> get_LookupId(This,pvarLookupId) ) 

#define IMSMQMessage4_get_IsAuthenticated2(This,pisAuthenticated)	\
    ( (This)->lpVtbl -> get_IsAuthenticated2(This,pisAuthenticated) ) 

#define IMSMQMessage4_get_IsFirstInTransaction2(This,pisFirstInXact)	\
    ( (This)->lpVtbl -> get_IsFirstInTransaction2(This,pisFirstInXact) ) 

#define IMSMQMessage4_get_IsLastInTransaction2(This,pisLastInXact)	\
    ( (This)->lpVtbl -> get_IsLastInTransaction2(This,pisLastInXact) ) 

#define IMSMQMessage4_AttachCurrentSecurityContext2(This)	\
    ( (This)->lpVtbl -> AttachCurrentSecurityContext2(This) ) 

#define IMSMQMessage4_get_SoapEnvelope(This,pbstrSoapEnvelope)	\
    ( (This)->lpVtbl -> get_SoapEnvelope(This,pbstrSoapEnvelope) ) 

#define IMSMQMessage4_get_CompoundMessage(This,pvarCompoundMessage)	\
    ( (This)->lpVtbl -> get_CompoundMessage(This,pvarCompoundMessage) ) 

#define IMSMQMessage4_put_SoapHeader(This,bstrSoapHeader)	\
    ( (This)->lpVtbl -> put_SoapHeader(This,bstrSoapHeader) ) 

#define IMSMQMessage4_put_SoapBody(This,bstrSoapBody)	\
    ( (This)->lpVtbl -> put_SoapBody(This,bstrSoapBody) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQMessage4_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQMessage;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E075-DCCD-11d0-AA4B-0060970DEBAE")
MSMQMessage;
#endif

EXTERN_C const CLSID CLSID_MSMQQueue;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E079-DCCD-11d0-AA4B-0060970DEBAE")
MSMQQueue;
#endif

#ifndef __IMSMQPrivateEvent_INTERFACE_DEFINED__
#define __IMSMQPrivateEvent_INTERFACE_DEFINED__

/* interface IMSMQPrivateEvent */
/* [object][dual][hidden][uuid] */ 


EXTERN_C const IID IID_IMSMQPrivateEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7AB3341-C9D3-11d1-BB47-0080C7C5A2C0")
    IMSMQPrivateEvent : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Hwnd( 
            /* [retval][out] */ __RPC__out long *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireArrivedEvent( 
            /* [in] */ __RPC__in_opt IMSMQQueue *pq,
            /* [in] */ long msgcursor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireArrivedErrorEvent( 
            /* [in] */ __RPC__in_opt IMSMQQueue *pq,
            /* [in] */ HRESULT hrStatus,
            /* [in] */ long msgcursor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQPrivateEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQPrivateEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQPrivateEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQPrivateEvent * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQPrivateEvent, get_Hwnd)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Hwnd )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [retval][out] */ __RPC__out long *phwnd);
        
        DECLSPEC_XFGVIRT(IMSMQPrivateEvent, FireArrivedEvent)
        HRESULT ( STDMETHODCALLTYPE *FireArrivedEvent )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [in] */ __RPC__in_opt IMSMQQueue *pq,
            /* [in] */ long msgcursor);
        
        DECLSPEC_XFGVIRT(IMSMQPrivateEvent, FireArrivedErrorEvent)
        HRESULT ( STDMETHODCALLTYPE *FireArrivedErrorEvent )( 
            __RPC__in IMSMQPrivateEvent * This,
            /* [in] */ __RPC__in_opt IMSMQQueue *pq,
            /* [in] */ HRESULT hrStatus,
            /* [in] */ long msgcursor);
        
        END_INTERFACE
    } IMSMQPrivateEventVtbl;

    interface IMSMQPrivateEvent
    {
        CONST_VTBL struct IMSMQPrivateEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQPrivateEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQPrivateEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQPrivateEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQPrivateEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQPrivateEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQPrivateEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQPrivateEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQPrivateEvent_get_Hwnd(This,phwnd)	\
    ( (This)->lpVtbl -> get_Hwnd(This,phwnd) ) 

#define IMSMQPrivateEvent_FireArrivedEvent(This,pq,msgcursor)	\
    ( (This)->lpVtbl -> FireArrivedEvent(This,pq,msgcursor) ) 

#define IMSMQPrivateEvent_FireArrivedErrorEvent(This,pq,hrStatus,msgcursor)	\
    ( (This)->lpVtbl -> FireArrivedErrorEvent(This,pq,hrStatus,msgcursor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQPrivateEvent_INTERFACE_DEFINED__ */


#ifndef ___DMSMQEventEvents_DISPINTERFACE_DEFINED__
#define ___DMSMQEventEvents_DISPINTERFACE_DEFINED__

/* dispinterface _DMSMQEventEvents */
/* [hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID DIID__DMSMQEventEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("D7D6E078-DCCD-11d0-AA4B-0060970DEBAE")
    _DMSMQEventEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _DMSMQEventEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _DMSMQEventEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _DMSMQEventEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _DMSMQEventEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _DMSMQEventEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _DMSMQEventEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _DMSMQEventEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _DMSMQEventEvents * This,
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
    } _DMSMQEventEventsVtbl;

    interface _DMSMQEventEvents
    {
        CONST_VTBL struct _DMSMQEventEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _DMSMQEventEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _DMSMQEventEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _DMSMQEventEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _DMSMQEventEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _DMSMQEventEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _DMSMQEventEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _DMSMQEventEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___DMSMQEventEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQEvent;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E07A-DCCD-11d0-AA4B-0060970DEBAE")
MSMQEvent;
#endif

EXTERN_C const CLSID CLSID_MSMQQueueInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E07C-DCCD-11d0-AA4B-0060970DEBAE")
MSMQQueueInfo;
#endif

EXTERN_C const CLSID CLSID_MSMQQueueInfos;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E07E-DCCD-11d0-AA4B-0060970DEBAE")
MSMQQueueInfos;
#endif

#ifndef __IMSMQTransaction2_INTERFACE_DEFINED__
#define __IMSMQTransaction2_INTERFACE_DEFINED__

/* interface IMSMQTransaction2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQTransaction2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2CE0C5B0-6E67-11D2-B0E6-00E02C074F6B")
    IMSMQTransaction2 : public IMSMQTransaction
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE InitNew( 
            /* [in] */ VARIANT varTransaction) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQTransaction2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQTransaction2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQTransaction2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQTransaction2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, get_Transaction)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Transaction )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [retval][out] */ __RPC__out long *plTransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, Commit)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *grfTC,
            /* [optional][in] */ __RPC__in VARIANT *grfRM);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, Abort)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *fAsync);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction2, InitNew)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [in] */ VARIANT varTransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQTransaction2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQTransaction2Vtbl;

    interface IMSMQTransaction2
    {
        CONST_VTBL struct IMSMQTransaction2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQTransaction2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQTransaction2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQTransaction2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQTransaction2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQTransaction2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQTransaction2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQTransaction2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQTransaction2_get_Transaction(This,plTransaction)	\
    ( (This)->lpVtbl -> get_Transaction(This,plTransaction) ) 

#define IMSMQTransaction2_Commit(This,fRetaining,grfTC,grfRM)	\
    ( (This)->lpVtbl -> Commit(This,fRetaining,grfTC,grfRM) ) 

#define IMSMQTransaction2_Abort(This,fRetaining,fAsync)	\
    ( (This)->lpVtbl -> Abort(This,fRetaining,fAsync) ) 


#define IMSMQTransaction2_InitNew(This,varTransaction)	\
    ( (This)->lpVtbl -> InitNew(This,varTransaction) ) 

#define IMSMQTransaction2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQTransaction2_INTERFACE_DEFINED__ */


#ifndef __IMSMQTransaction3_INTERFACE_DEFINED__
#define __IMSMQTransaction3_INTERFACE_DEFINED__

/* interface IMSMQTransaction3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQTransaction3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b13-2168-11d3-898c-00e02c074f6b")
    IMSMQTransaction3 : public IMSMQTransaction2
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ITransaction( 
            /* [retval][out] */ __RPC__out VARIANT *pvarITransaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQTransaction3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQTransaction3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQTransaction3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQTransaction3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, get_Transaction)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Transaction )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [retval][out] */ __RPC__out long *plTransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, Commit)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *grfTC,
            /* [optional][in] */ __RPC__in VARIANT *grfRM);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction, Abort)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [optional][in] */ __RPC__in VARIANT *fRetaining,
            /* [optional][in] */ __RPC__in VARIANT *fAsync);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction2, InitNew)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [in] */ VARIANT varTransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQTransaction3, get_ITransaction)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ITransaction )( 
            __RPC__in IMSMQTransaction3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarITransaction);
        
        END_INTERFACE
    } IMSMQTransaction3Vtbl;

    interface IMSMQTransaction3
    {
        CONST_VTBL struct IMSMQTransaction3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQTransaction3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQTransaction3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQTransaction3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQTransaction3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQTransaction3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQTransaction3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQTransaction3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQTransaction3_get_Transaction(This,plTransaction)	\
    ( (This)->lpVtbl -> get_Transaction(This,plTransaction) ) 

#define IMSMQTransaction3_Commit(This,fRetaining,grfTC,grfRM)	\
    ( (This)->lpVtbl -> Commit(This,fRetaining,grfTC,grfRM) ) 

#define IMSMQTransaction3_Abort(This,fRetaining,fAsync)	\
    ( (This)->lpVtbl -> Abort(This,fRetaining,fAsync) ) 


#define IMSMQTransaction3_InitNew(This,varTransaction)	\
    ( (This)->lpVtbl -> InitNew(This,varTransaction) ) 

#define IMSMQTransaction3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 


#define IMSMQTransaction3_get_ITransaction(This,pvarITransaction)	\
    ( (This)->lpVtbl -> get_ITransaction(This,pvarITransaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQTransaction3_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQTransaction;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E080-DCCD-11d0-AA4B-0060970DEBAE")
MSMQTransaction;
#endif

#ifndef __IMSMQCoordinatedTransactionDispenser2_INTERFACE_DEFINED__
#define __IMSMQCoordinatedTransactionDispenser2_INTERFACE_DEFINED__

/* interface IMSMQCoordinatedTransactionDispenser2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQCoordinatedTransactionDispenser2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b10-2168-11d3-898c-00e02c074f6b")
    IMSMQCoordinatedTransactionDispenser2 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE BeginTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction2 **ptransaction) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQCoordinatedTransactionDispenser2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQCoordinatedTransactionDispenser2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQCoordinatedTransactionDispenser2, BeginTransaction)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *BeginTransaction )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction2 **ptransaction);
        
        DECLSPEC_XFGVIRT(IMSMQCoordinatedTransactionDispenser2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQCoordinatedTransactionDispenser2Vtbl;

    interface IMSMQCoordinatedTransactionDispenser2
    {
        CONST_VTBL struct IMSMQCoordinatedTransactionDispenser2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQCoordinatedTransactionDispenser2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQCoordinatedTransactionDispenser2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQCoordinatedTransactionDispenser2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQCoordinatedTransactionDispenser2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQCoordinatedTransactionDispenser2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQCoordinatedTransactionDispenser2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQCoordinatedTransactionDispenser2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQCoordinatedTransactionDispenser2_BeginTransaction(This,ptransaction)	\
    ( (This)->lpVtbl -> BeginTransaction(This,ptransaction) ) 

#define IMSMQCoordinatedTransactionDispenser2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQCoordinatedTransactionDispenser2_INTERFACE_DEFINED__ */


#ifndef __IMSMQCoordinatedTransactionDispenser3_INTERFACE_DEFINED__
#define __IMSMQCoordinatedTransactionDispenser3_INTERFACE_DEFINED__

/* interface IMSMQCoordinatedTransactionDispenser3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQCoordinatedTransactionDispenser3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b14-2168-11d3-898c-00e02c074f6b")
    IMSMQCoordinatedTransactionDispenser3 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE BeginTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction3 **ptransaction) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQCoordinatedTransactionDispenser3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQCoordinatedTransactionDispenser3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQCoordinatedTransactionDispenser3, BeginTransaction)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *BeginTransaction )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction3 **ptransaction);
        
        DECLSPEC_XFGVIRT(IMSMQCoordinatedTransactionDispenser3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQCoordinatedTransactionDispenser3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQCoordinatedTransactionDispenser3Vtbl;

    interface IMSMQCoordinatedTransactionDispenser3
    {
        CONST_VTBL struct IMSMQCoordinatedTransactionDispenser3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQCoordinatedTransactionDispenser3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQCoordinatedTransactionDispenser3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQCoordinatedTransactionDispenser3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQCoordinatedTransactionDispenser3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQCoordinatedTransactionDispenser3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQCoordinatedTransactionDispenser3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQCoordinatedTransactionDispenser3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQCoordinatedTransactionDispenser3_BeginTransaction(This,ptransaction)	\
    ( (This)->lpVtbl -> BeginTransaction(This,ptransaction) ) 

#define IMSMQCoordinatedTransactionDispenser3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQCoordinatedTransactionDispenser3_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQCoordinatedTransactionDispenser;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E082-DCCD-11d0-AA4B-0060970DEBAE")
MSMQCoordinatedTransactionDispenser;
#endif

#ifndef __IMSMQTransactionDispenser2_INTERFACE_DEFINED__
#define __IMSMQTransactionDispenser2_INTERFACE_DEFINED__

/* interface IMSMQTransactionDispenser2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQTransactionDispenser2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b11-2168-11d3-898c-00e02c074f6b")
    IMSMQTransactionDispenser2 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE BeginTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction2 **ptransaction) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQTransactionDispenser2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQTransactionDispenser2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQTransactionDispenser2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQTransactionDispenser2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQTransactionDispenser2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQTransactionDispenser2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQTransactionDispenser2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQTransactionDispenser2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQTransactionDispenser2, BeginTransaction)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *BeginTransaction )( 
            __RPC__in IMSMQTransactionDispenser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction2 **ptransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransactionDispenser2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQTransactionDispenser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQTransactionDispenser2Vtbl;

    interface IMSMQTransactionDispenser2
    {
        CONST_VTBL struct IMSMQTransactionDispenser2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQTransactionDispenser2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQTransactionDispenser2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQTransactionDispenser2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQTransactionDispenser2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQTransactionDispenser2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQTransactionDispenser2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQTransactionDispenser2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQTransactionDispenser2_BeginTransaction(This,ptransaction)	\
    ( (This)->lpVtbl -> BeginTransaction(This,ptransaction) ) 

#define IMSMQTransactionDispenser2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQTransactionDispenser2_INTERFACE_DEFINED__ */


#ifndef __IMSMQTransactionDispenser3_INTERFACE_DEFINED__
#define __IMSMQTransactionDispenser3_INTERFACE_DEFINED__

/* interface IMSMQTransactionDispenser3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQTransactionDispenser3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b15-2168-11d3-898c-00e02c074f6b")
    IMSMQTransactionDispenser3 : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE BeginTransaction( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction3 **ptransaction) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQTransactionDispenser3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQTransactionDispenser3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQTransactionDispenser3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQTransactionDispenser3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQTransactionDispenser3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQTransactionDispenser3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQTransactionDispenser3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQTransactionDispenser3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQTransactionDispenser3, BeginTransaction)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *BeginTransaction )( 
            __RPC__in IMSMQTransactionDispenser3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQTransaction3 **ptransaction);
        
        DECLSPEC_XFGVIRT(IMSMQTransactionDispenser3, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQTransactionDispenser3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQTransactionDispenser3Vtbl;

    interface IMSMQTransactionDispenser3
    {
        CONST_VTBL struct IMSMQTransactionDispenser3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQTransactionDispenser3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQTransactionDispenser3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQTransactionDispenser3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQTransactionDispenser3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQTransactionDispenser3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQTransactionDispenser3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQTransactionDispenser3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQTransactionDispenser3_BeginTransaction(This,ptransaction)	\
    ( (This)->lpVtbl -> BeginTransaction(This,ptransaction) ) 

#define IMSMQTransactionDispenser3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQTransactionDispenser3_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQTransactionDispenser;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E084-DCCD-11d0-AA4B-0060970DEBAE")
MSMQTransactionDispenser;
#endif

#ifndef __IMSMQApplication_INTERFACE_DEFINED__
#define __IMSMQApplication_INTERFACE_DEFINED__

/* interface IMSMQApplication */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7D6E085-DCCD-11d0-AA4B-0060970DEBAE")
    IMSMQApplication : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE MachineIdOfMachineName( 
            /* [in] */ __RPC__in BSTR MachineName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQApplication * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQApplication * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQApplication * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQApplication * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQApplication, MachineIdOfMachineName)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *MachineIdOfMachineName )( 
            __RPC__in IMSMQApplication * This,
            /* [in] */ __RPC__in BSTR MachineName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuid);
        
        END_INTERFACE
    } IMSMQApplicationVtbl;

    interface IMSMQApplication
    {
        CONST_VTBL struct IMSMQApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQApplication_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQApplication_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQApplication_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQApplication_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQApplication_MachineIdOfMachineName(This,MachineName,pbstrGuid)	\
    ( (This)->lpVtbl -> MachineIdOfMachineName(This,MachineName,pbstrGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQApplication_INTERFACE_DEFINED__ */


#ifndef __IMSMQApplication2_INTERFACE_DEFINED__
#define __IMSMQApplication2_INTERFACE_DEFINED__

/* interface IMSMQApplication2 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQApplication2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12A30900-7300-11D2-B0E6-00E02C074F6B")
    IMSMQApplication2 : public IMSMQApplication
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE RegisterCertificate( 
            /* [optional][in] */ __RPC__in VARIANT *Flags,
            /* [optional][in] */ __RPC__in VARIANT *ExternalCertificate) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE MachineNameOfMachineId( 
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachineName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MSMQVersionMajor( 
            /* [retval][out] */ __RPC__out short *psMSMQVersionMajor) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MSMQVersionMinor( 
            /* [retval][out] */ __RPC__out short *psMSMQVersionMinor) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MSMQVersionBuild( 
            /* [retval][out] */ __RPC__out short *psMSMQVersionBuild) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsDsEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsDsEnabled) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQApplication2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQApplication2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQApplication2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQApplication2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQApplication2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQApplication2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQApplication2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQApplication2 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQApplication, MachineIdOfMachineName)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *MachineIdOfMachineName )( 
            __RPC__in IMSMQApplication2 * This,
            /* [in] */ __RPC__in BSTR MachineName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuid);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, RegisterCertificate)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *RegisterCertificate )( 
            __RPC__in IMSMQApplication2 * This,
            /* [optional][in] */ __RPC__in VARIANT *Flags,
            /* [optional][in] */ __RPC__in VARIANT *ExternalCertificate);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, MachineNameOfMachineId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *MachineNameOfMachineId )( 
            __RPC__in IMSMQApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachineName);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_MSMQVersionMajor)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MSMQVersionMajor )( 
            __RPC__in IMSMQApplication2 * This,
            /* [retval][out] */ __RPC__out short *psMSMQVersionMajor);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_MSMQVersionMinor)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MSMQVersionMinor )( 
            __RPC__in IMSMQApplication2 * This,
            /* [retval][out] */ __RPC__out short *psMSMQVersionMinor);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_MSMQVersionBuild)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MSMQVersionBuild )( 
            __RPC__in IMSMQApplication2 * This,
            /* [retval][out] */ __RPC__out short *psMSMQVersionBuild);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_IsDsEnabled)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsDsEnabled )( 
            __RPC__in IMSMQApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsDsEnabled);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQApplication2Vtbl;

    interface IMSMQApplication2
    {
        CONST_VTBL struct IMSMQApplication2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQApplication2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQApplication2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQApplication2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQApplication2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQApplication2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQApplication2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQApplication2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQApplication2_MachineIdOfMachineName(This,MachineName,pbstrGuid)	\
    ( (This)->lpVtbl -> MachineIdOfMachineName(This,MachineName,pbstrGuid) ) 


#define IMSMQApplication2_RegisterCertificate(This,Flags,ExternalCertificate)	\
    ( (This)->lpVtbl -> RegisterCertificate(This,Flags,ExternalCertificate) ) 

#define IMSMQApplication2_MachineNameOfMachineId(This,bstrGuid,pbstrMachineName)	\
    ( (This)->lpVtbl -> MachineNameOfMachineId(This,bstrGuid,pbstrMachineName) ) 

#define IMSMQApplication2_get_MSMQVersionMajor(This,psMSMQVersionMajor)	\
    ( (This)->lpVtbl -> get_MSMQVersionMajor(This,psMSMQVersionMajor) ) 

#define IMSMQApplication2_get_MSMQVersionMinor(This,psMSMQVersionMinor)	\
    ( (This)->lpVtbl -> get_MSMQVersionMinor(This,psMSMQVersionMinor) ) 

#define IMSMQApplication2_get_MSMQVersionBuild(This,psMSMQVersionBuild)	\
    ( (This)->lpVtbl -> get_MSMQVersionBuild(This,psMSMQVersionBuild) ) 

#define IMSMQApplication2_get_IsDsEnabled(This,pfIsDsEnabled)	\
    ( (This)->lpVtbl -> get_IsDsEnabled(This,pfIsDsEnabled) ) 

#define IMSMQApplication2_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQApplication2_INTERFACE_DEFINED__ */


#ifndef __IMSMQApplication3_INTERFACE_DEFINED__
#define __IMSMQApplication3_INTERFACE_DEFINED__

/* interface IMSMQApplication3 */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQApplication3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b1f-2168-11d3-898c-00e02c074f6b")
    IMSMQApplication3 : public IMSMQApplication2
    {
    public:
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ActiveQueues( 
            /* [retval][out] */ __RPC__out VARIANT *pvActiveQueues) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PrivateQueues( 
            /* [retval][out] */ __RPC__out VARIANT *pvPrivateQueues) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_DirectoryServiceServer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDirectoryServiceServer) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsConnected( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsConnected) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BytesInAllQueues( 
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInAllQueues) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_Machine( 
            /* [in] */ __RPC__in BSTR bstrMachine) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Machine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachine) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Connect( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Tidy( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQApplication3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQApplication3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQApplication3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQApplication3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQApplication3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQApplication3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQApplication3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQApplication3 * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQApplication, MachineIdOfMachineName)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *MachineIdOfMachineName )( 
            __RPC__in IMSMQApplication3 * This,
            /* [in] */ __RPC__in BSTR MachineName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGuid);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, RegisterCertificate)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *RegisterCertificate )( 
            __RPC__in IMSMQApplication3 * This,
            /* [optional][in] */ __RPC__in VARIANT *Flags,
            /* [optional][in] */ __RPC__in VARIANT *ExternalCertificate);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, MachineNameOfMachineId)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *MachineNameOfMachineId )( 
            __RPC__in IMSMQApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachineName);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_MSMQVersionMajor)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MSMQVersionMajor )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out short *psMSMQVersionMajor);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_MSMQVersionMinor)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MSMQVersionMinor )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out short *psMSMQVersionMinor);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_MSMQVersionBuild)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MSMQVersionBuild )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out short *psMSMQVersionBuild);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_IsDsEnabled)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsDsEnabled )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsDsEnabled);
        
        DECLSPEC_XFGVIRT(IMSMQApplication2, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, get_ActiveQueues)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveQueues )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvActiveQueues);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, get_PrivateQueues)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateQueues )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvPrivateQueues);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, get_DirectoryServiceServer)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_DirectoryServiceServer )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDirectoryServiceServer);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, get_IsConnected)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsConnected )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsConnected);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, get_BytesInAllQueues)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BytesInAllQueues )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInAllQueues);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, put_Machine)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_Machine )( 
            __RPC__in IMSMQApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrMachine);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, get_Machine)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Machine )( 
            __RPC__in IMSMQApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachine);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, Connect)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IMSMQApplication3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, Disconnect)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IMSMQApplication3 * This);
        
        DECLSPEC_XFGVIRT(IMSMQApplication3, Tidy)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Tidy )( 
            __RPC__in IMSMQApplication3 * This);
        
        END_INTERFACE
    } IMSMQApplication3Vtbl;

    interface IMSMQApplication3
    {
        CONST_VTBL struct IMSMQApplication3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQApplication3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQApplication3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQApplication3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQApplication3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQApplication3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQApplication3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQApplication3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQApplication3_MachineIdOfMachineName(This,MachineName,pbstrGuid)	\
    ( (This)->lpVtbl -> MachineIdOfMachineName(This,MachineName,pbstrGuid) ) 


#define IMSMQApplication3_RegisterCertificate(This,Flags,ExternalCertificate)	\
    ( (This)->lpVtbl -> RegisterCertificate(This,Flags,ExternalCertificate) ) 

#define IMSMQApplication3_MachineNameOfMachineId(This,bstrGuid,pbstrMachineName)	\
    ( (This)->lpVtbl -> MachineNameOfMachineId(This,bstrGuid,pbstrMachineName) ) 

#define IMSMQApplication3_get_MSMQVersionMajor(This,psMSMQVersionMajor)	\
    ( (This)->lpVtbl -> get_MSMQVersionMajor(This,psMSMQVersionMajor) ) 

#define IMSMQApplication3_get_MSMQVersionMinor(This,psMSMQVersionMinor)	\
    ( (This)->lpVtbl -> get_MSMQVersionMinor(This,psMSMQVersionMinor) ) 

#define IMSMQApplication3_get_MSMQVersionBuild(This,psMSMQVersionBuild)	\
    ( (This)->lpVtbl -> get_MSMQVersionBuild(This,psMSMQVersionBuild) ) 

#define IMSMQApplication3_get_IsDsEnabled(This,pfIsDsEnabled)	\
    ( (This)->lpVtbl -> get_IsDsEnabled(This,pfIsDsEnabled) ) 

#define IMSMQApplication3_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 


#define IMSMQApplication3_get_ActiveQueues(This,pvActiveQueues)	\
    ( (This)->lpVtbl -> get_ActiveQueues(This,pvActiveQueues) ) 

#define IMSMQApplication3_get_PrivateQueues(This,pvPrivateQueues)	\
    ( (This)->lpVtbl -> get_PrivateQueues(This,pvPrivateQueues) ) 

#define IMSMQApplication3_get_DirectoryServiceServer(This,pbstrDirectoryServiceServer)	\
    ( (This)->lpVtbl -> get_DirectoryServiceServer(This,pbstrDirectoryServiceServer) ) 

#define IMSMQApplication3_get_IsConnected(This,pfIsConnected)	\
    ( (This)->lpVtbl -> get_IsConnected(This,pfIsConnected) ) 

#define IMSMQApplication3_get_BytesInAllQueues(This,pvBytesInAllQueues)	\
    ( (This)->lpVtbl -> get_BytesInAllQueues(This,pvBytesInAllQueues) ) 

#define IMSMQApplication3_put_Machine(This,bstrMachine)	\
    ( (This)->lpVtbl -> put_Machine(This,bstrMachine) ) 

#define IMSMQApplication3_get_Machine(This,pbstrMachine)	\
    ( (This)->lpVtbl -> get_Machine(This,pbstrMachine) ) 

#define IMSMQApplication3_Connect(This)	\
    ( (This)->lpVtbl -> Connect(This) ) 

#define IMSMQApplication3_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IMSMQApplication3_Tidy(This)	\
    ( (This)->lpVtbl -> Tidy(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQApplication3_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQApplication;

#ifdef __cplusplus

class DECLSPEC_UUID("D7D6E086-DCCD-11d0-AA4B-0060970DEBAE")
MSMQApplication;
#endif

#ifndef __IMSMQDestination_INTERFACE_DEFINED__
#define __IMSMQDestination_INTERFACE_DEFINED__

/* interface IMSMQDestination */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQDestination;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b16-2168-11d3-898c-00e02c074f6b")
    IMSMQDestination : public IDispatch
    {
    public:
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Open( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsOpen( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsOpen) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_IADs( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppIADs) = 0;
        
        virtual /* [id][propputref][hidden] */ HRESULT STDMETHODCALLTYPE putref_IADs( 
            /* [in] */ __RPC__in_opt IDispatch *pIADs) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ADsPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_ADsPath( 
            /* [in] */ __RPC__in BSTR bstrADsPath) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_PathName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_PathName( 
            /* [in] */ __RPC__in BSTR bstrPathName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_FormatName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName) = 0;
        
        virtual /* [id][propput][helpstringcontext] */ HRESULT STDMETHODCALLTYPE put_FormatName( 
            /* [in] */ __RPC__in BSTR bstrFormatName) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Destinations( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDestinations) = 0;
        
        virtual /* [id][propputref][hidden] */ HRESULT STDMETHODCALLTYPE putref_Destinations( 
            /* [in] */ __RPC__in_opt IDispatch *pDestinations) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Properties( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQDestinationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQDestination * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQDestination * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQDestination * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQDestination * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQDestination, Open)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMSMQDestination * This);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, Close)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMSMQDestination * This);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_IsOpen)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsOpen )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsOpen);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_IADs)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_IADs )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppIADs);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, putref_IADs)
        /* [id][propputref][hidden] */ HRESULT ( STDMETHODCALLTYPE *putref_IADs )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in_opt IDispatch *pIADs);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_ADsPath)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, put_ADsPath)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_ADsPath )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in BSTR bstrADsPath);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_PathName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_PathName )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, put_PathName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_PathName )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in BSTR bstrPathName);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, put_FormatName)
        /* [id][propput][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *put_FormatName )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in BSTR bstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_Destinations)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Destinations )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDestinations);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, putref_Destinations)
        /* [id][propputref][hidden] */ HRESULT ( STDMETHODCALLTYPE *putref_Destinations )( 
            __RPC__in IMSMQDestination * This,
            /* [in] */ __RPC__in_opt IDispatch *pDestinations);
        
        DECLSPEC_XFGVIRT(IMSMQDestination, get_Properties)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Properties )( 
            __RPC__in IMSMQDestination * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppcolProperties);
        
        END_INTERFACE
    } IMSMQDestinationVtbl;

    interface IMSMQDestination
    {
        CONST_VTBL struct IMSMQDestinationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQDestination_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQDestination_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQDestination_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQDestination_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQDestination_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQDestination_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQDestination_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQDestination_Open(This)	\
    ( (This)->lpVtbl -> Open(This) ) 

#define IMSMQDestination_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMSMQDestination_get_IsOpen(This,pfIsOpen)	\
    ( (This)->lpVtbl -> get_IsOpen(This,pfIsOpen) ) 

#define IMSMQDestination_get_IADs(This,ppIADs)	\
    ( (This)->lpVtbl -> get_IADs(This,ppIADs) ) 

#define IMSMQDestination_putref_IADs(This,pIADs)	\
    ( (This)->lpVtbl -> putref_IADs(This,pIADs) ) 

#define IMSMQDestination_get_ADsPath(This,pbstrADsPath)	\
    ( (This)->lpVtbl -> get_ADsPath(This,pbstrADsPath) ) 

#define IMSMQDestination_put_ADsPath(This,bstrADsPath)	\
    ( (This)->lpVtbl -> put_ADsPath(This,bstrADsPath) ) 

#define IMSMQDestination_get_PathName(This,pbstrPathName)	\
    ( (This)->lpVtbl -> get_PathName(This,pbstrPathName) ) 

#define IMSMQDestination_put_PathName(This,bstrPathName)	\
    ( (This)->lpVtbl -> put_PathName(This,bstrPathName) ) 

#define IMSMQDestination_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQDestination_put_FormatName(This,bstrFormatName)	\
    ( (This)->lpVtbl -> put_FormatName(This,bstrFormatName) ) 

#define IMSMQDestination_get_Destinations(This,ppDestinations)	\
    ( (This)->lpVtbl -> get_Destinations(This,ppDestinations) ) 

#define IMSMQDestination_putref_Destinations(This,pDestinations)	\
    ( (This)->lpVtbl -> putref_Destinations(This,pDestinations) ) 

#define IMSMQDestination_get_Properties(This,ppcolProperties)	\
    ( (This)->lpVtbl -> get_Properties(This,ppcolProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQDestination_INTERFACE_DEFINED__ */


#ifndef __IMSMQPrivateDestination_INTERFACE_DEFINED__
#define __IMSMQPrivateDestination_INTERFACE_DEFINED__

/* interface IMSMQPrivateDestination */
/* [object][dual][hidden][uuid] */ 


EXTERN_C const IID IID_IMSMQPrivateDestination;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eba96b17-2168-11d3-898c-00e02c074f6b")
    IMSMQPrivateDestination : public IDispatch
    {
    public:
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out VARIANT *pvarHandle) = 0;
        
        virtual /* [id][propput][hidden] */ HRESULT STDMETHODCALLTYPE put_Handle( 
            /* [in] */ VARIANT varHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQPrivateDestinationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQPrivateDestination * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQPrivateDestination * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQPrivateDestination * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQPrivateDestination * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQPrivateDestination * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQPrivateDestination * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQPrivateDestination * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQPrivateDestination, get_Handle)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in IMSMQPrivateDestination * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarHandle);
        
        DECLSPEC_XFGVIRT(IMSMQPrivateDestination, put_Handle)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Handle )( 
            __RPC__in IMSMQPrivateDestination * This,
            /* [in] */ VARIANT varHandle);
        
        END_INTERFACE
    } IMSMQPrivateDestinationVtbl;

    interface IMSMQPrivateDestination
    {
        CONST_VTBL struct IMSMQPrivateDestinationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQPrivateDestination_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQPrivateDestination_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQPrivateDestination_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQPrivateDestination_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQPrivateDestination_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQPrivateDestination_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQPrivateDestination_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQPrivateDestination_get_Handle(This,pvarHandle)	\
    ( (This)->lpVtbl -> get_Handle(This,pvarHandle) ) 

#define IMSMQPrivateDestination_put_Handle(This,varHandle)	\
    ( (This)->lpVtbl -> put_Handle(This,varHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQPrivateDestination_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQDestination;

#ifdef __cplusplus

class DECLSPEC_UUID("eba96b18-2168-11d3-898c-00e02c074f6b")
MSMQDestination;
#endif

#ifndef __IMSMQCollection_INTERFACE_DEFINED__
#define __IMSMQCollection_INTERFACE_DEFINED__

/* interface IMSMQCollection */
/* [object][oleautomation][dual][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0188AC2F-ECB3-4173-9779-635CA2039C72")
    IMSMQCollection : public IDispatch
    {
    public:
        virtual /* [id][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ __RPC__in VARIANT *Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarRet) = 0;
        
        virtual /* [propget][id][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [restricted][id] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQCollection * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQCollection, Item)
        /* [id][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IMSMQCollection * This,
            /* [in] */ __RPC__in VARIANT *Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarRet);
        
        DECLSPEC_XFGVIRT(IMSMQCollection, get_Count)
        /* [propget][id][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IMSMQCollection * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        DECLSPEC_XFGVIRT(IMSMQCollection, _NewEnum)
        /* [restricted][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in IMSMQCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } IMSMQCollectionVtbl;

    interface IMSMQCollection
    {
        CONST_VTBL struct IMSMQCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQCollection_Item(This,Index,pvarRet)	\
    ( (This)->lpVtbl -> Item(This,Index,pvarRet) ) 

#define IMSMQCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define IMSMQCollection__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQCollection_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("f72b9031-2f0c-43e8-924e-e6052cdc493f")
MSMQCollection;
#endif

#ifndef __IMSMQManagement_INTERFACE_DEFINED__
#define __IMSMQManagement_INTERFACE_DEFINED__

/* interface IMSMQManagement */
/* [object][dual][hidden][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQManagement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BE5F0241-E489-4957-8CC4-A452FCF3E23E")
    IMSMQManagement : public IDispatch
    {
    public:
        virtual /* [id][helpstringcontext] */ HRESULT STDMETHODCALLTYPE Init( 
            /* [optional][in] */ __RPC__in VARIANT *Machine,
            /* [optional][in] */ __RPC__in VARIANT *Pathname,
            /* [optional][in] */ __RPC__in VARIANT *FormatName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_FormatName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_Machine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachine) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_MessageCount( 
            /* [retval][out] */ __RPC__out long *plMessageCount) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_ForeignStatus( 
            /* [retval][out] */ __RPC__out long *plForeignStatus) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_QueueType( 
            /* [retval][out] */ __RPC__out long *plQueueType) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_IsLocal( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsLocal) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_TransactionalStatus( 
            /* [retval][out] */ __RPC__out long *plTransactionalStatus) = 0;
        
        virtual /* [id][propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BytesInQueue( 
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInQueue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQManagementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQManagement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQManagement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQManagement * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQManagement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQManagement * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQManagement, Init)
        /* [id][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IMSMQManagement * This,
            /* [optional][in] */ __RPC__in VARIANT *Machine,
            /* [optional][in] */ __RPC__in VARIANT *Pathname,
            /* [optional][in] */ __RPC__in VARIANT *FormatName);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_Machine)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Machine )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachine);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_MessageCount)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MessageCount )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__out long *plMessageCount);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_ForeignStatus)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ForeignStatus )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__out long *plForeignStatus);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_QueueType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueType )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__out long *plQueueType);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_IsLocal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsLocal )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsLocal);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_TransactionalStatus)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionalStatus )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__out long *plTransactionalStatus);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_BytesInQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BytesInQueue )( 
            __RPC__in IMSMQManagement * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInQueue);
        
        END_INTERFACE
    } IMSMQManagementVtbl;

    interface IMSMQManagement
    {
        CONST_VTBL struct IMSMQManagementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQManagement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQManagement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQManagement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQManagement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQManagement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQManagement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQManagement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQManagement_Init(This,Machine,Pathname,FormatName)	\
    ( (This)->lpVtbl -> Init(This,Machine,Pathname,FormatName) ) 

#define IMSMQManagement_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQManagement_get_Machine(This,pbstrMachine)	\
    ( (This)->lpVtbl -> get_Machine(This,pbstrMachine) ) 

#define IMSMQManagement_get_MessageCount(This,plMessageCount)	\
    ( (This)->lpVtbl -> get_MessageCount(This,plMessageCount) ) 

#define IMSMQManagement_get_ForeignStatus(This,plForeignStatus)	\
    ( (This)->lpVtbl -> get_ForeignStatus(This,plForeignStatus) ) 

#define IMSMQManagement_get_QueueType(This,plQueueType)	\
    ( (This)->lpVtbl -> get_QueueType(This,plQueueType) ) 

#define IMSMQManagement_get_IsLocal(This,pfIsLocal)	\
    ( (This)->lpVtbl -> get_IsLocal(This,pfIsLocal) ) 

#define IMSMQManagement_get_TransactionalStatus(This,plTransactionalStatus)	\
    ( (This)->lpVtbl -> get_TransactionalStatus(This,plTransactionalStatus) ) 

#define IMSMQManagement_get_BytesInQueue(This,pvBytesInQueue)	\
    ( (This)->lpVtbl -> get_BytesInQueue(This,pvBytesInQueue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQManagement_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQManagement;

#ifdef __cplusplus

class DECLSPEC_UUID("39CE96FE-F4C5-4484-A143-4C2D5D324229")
MSMQManagement;
#endif

#ifndef __IMSMQOutgoingQueueManagement_INTERFACE_DEFINED__
#define __IMSMQOutgoingQueueManagement_INTERFACE_DEFINED__

/* interface IMSMQOutgoingQueueManagement */
/* [object][dual][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQOutgoingQueueManagement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64C478FB-F9B0-4695-8A7F-439AC94326D3")
    IMSMQOutgoingQueueManagement : public IMSMQManagement
    {
    public:
        virtual /* [propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out long *plState) = 0;
        
        virtual /* [propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_NextHops( 
            /* [retval][out] */ __RPC__out VARIANT *pvNextHops) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EodGetSendInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IMSMQCollection **ppCollection) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EodResend( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQOutgoingQueueManagementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQOutgoingQueueManagement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQOutgoingQueueManagement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQOutgoingQueueManagement * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQManagement, Init)
        /* [id][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [optional][in] */ __RPC__in VARIANT *Machine,
            /* [optional][in] */ __RPC__in VARIANT *Pathname,
            /* [optional][in] */ __RPC__in VARIANT *FormatName);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_Machine)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Machine )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachine);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_MessageCount)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MessageCount )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plMessageCount);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_ForeignStatus)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ForeignStatus )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plForeignStatus);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_QueueType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueType )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plQueueType);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_IsLocal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsLocal )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsLocal);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_TransactionalStatus)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionalStatus )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plTransactionalStatus);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_BytesInQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BytesInQueue )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInQueue);
        
        DECLSPEC_XFGVIRT(IMSMQOutgoingQueueManagement, get_State)
        /* [propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plState);
        
        DECLSPEC_XFGVIRT(IMSMQOutgoingQueueManagement, get_NextHops)
        /* [propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_NextHops )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT *pvNextHops);
        
        DECLSPEC_XFGVIRT(IMSMQOutgoingQueueManagement, EodGetSendInfo)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EodGetSendInfo )( 
            __RPC__in IMSMQOutgoingQueueManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt IMSMQCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IMSMQOutgoingQueueManagement, Resume)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IMSMQOutgoingQueueManagement * This);
        
        DECLSPEC_XFGVIRT(IMSMQOutgoingQueueManagement, Pause)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMSMQOutgoingQueueManagement * This);
        
        DECLSPEC_XFGVIRT(IMSMQOutgoingQueueManagement, EodResend)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EodResend )( 
            __RPC__in IMSMQOutgoingQueueManagement * This);
        
        END_INTERFACE
    } IMSMQOutgoingQueueManagementVtbl;

    interface IMSMQOutgoingQueueManagement
    {
        CONST_VTBL struct IMSMQOutgoingQueueManagementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQOutgoingQueueManagement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQOutgoingQueueManagement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQOutgoingQueueManagement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQOutgoingQueueManagement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQOutgoingQueueManagement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQOutgoingQueueManagement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQOutgoingQueueManagement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQOutgoingQueueManagement_Init(This,Machine,Pathname,FormatName)	\
    ( (This)->lpVtbl -> Init(This,Machine,Pathname,FormatName) ) 

#define IMSMQOutgoingQueueManagement_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQOutgoingQueueManagement_get_Machine(This,pbstrMachine)	\
    ( (This)->lpVtbl -> get_Machine(This,pbstrMachine) ) 

#define IMSMQOutgoingQueueManagement_get_MessageCount(This,plMessageCount)	\
    ( (This)->lpVtbl -> get_MessageCount(This,plMessageCount) ) 

#define IMSMQOutgoingQueueManagement_get_ForeignStatus(This,plForeignStatus)	\
    ( (This)->lpVtbl -> get_ForeignStatus(This,plForeignStatus) ) 

#define IMSMQOutgoingQueueManagement_get_QueueType(This,plQueueType)	\
    ( (This)->lpVtbl -> get_QueueType(This,plQueueType) ) 

#define IMSMQOutgoingQueueManagement_get_IsLocal(This,pfIsLocal)	\
    ( (This)->lpVtbl -> get_IsLocal(This,pfIsLocal) ) 

#define IMSMQOutgoingQueueManagement_get_TransactionalStatus(This,plTransactionalStatus)	\
    ( (This)->lpVtbl -> get_TransactionalStatus(This,plTransactionalStatus) ) 

#define IMSMQOutgoingQueueManagement_get_BytesInQueue(This,pvBytesInQueue)	\
    ( (This)->lpVtbl -> get_BytesInQueue(This,pvBytesInQueue) ) 


#define IMSMQOutgoingQueueManagement_get_State(This,plState)	\
    ( (This)->lpVtbl -> get_State(This,plState) ) 

#define IMSMQOutgoingQueueManagement_get_NextHops(This,pvNextHops)	\
    ( (This)->lpVtbl -> get_NextHops(This,pvNextHops) ) 

#define IMSMQOutgoingQueueManagement_EodGetSendInfo(This,ppCollection)	\
    ( (This)->lpVtbl -> EodGetSendInfo(This,ppCollection) ) 

#define IMSMQOutgoingQueueManagement_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IMSMQOutgoingQueueManagement_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMSMQOutgoingQueueManagement_EodResend(This)	\
    ( (This)->lpVtbl -> EodResend(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQOutgoingQueueManagement_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQOutgoingQueueManagement;

#ifdef __cplusplus

class DECLSPEC_UUID("0188401c-247a-4fed-99c6-bf14119d7055")
MSMQOutgoingQueueManagement;
#endif

#ifndef __IMSMQQueueManagement_INTERFACE_DEFINED__
#define __IMSMQQueueManagement_INTERFACE_DEFINED__

/* interface IMSMQQueueManagement */
/* [object][dual][helpstringcontext][uuid] */ 


EXTERN_C const IID IID_IMSMQQueueManagement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7FBE7759-5760-444d-B8A5-5E7AB9A84CCE")
    IMSMQQueueManagement : public IMSMQManagement
    {
    public:
        virtual /* [propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_JournalMessageCount( 
            /* [retval][out] */ __RPC__out long *plJournalMessageCount) = 0;
        
        virtual /* [propget][helpstringcontext] */ HRESULT STDMETHODCALLTYPE get_BytesInJournal( 
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInJournal) = 0;
        
        virtual /* [helpstringcontext] */ HRESULT STDMETHODCALLTYPE EodGetReceiveInfo( 
            /* [retval][out] */ __RPC__out VARIANT *pvCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSMQQueueManagementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSMQQueueManagement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSMQQueueManagement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMSMQQueueManagement * This,
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
        
        DECLSPEC_XFGVIRT(IMSMQManagement, Init)
        /* [id][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [optional][in] */ __RPC__in VARIANT *Machine,
            /* [optional][in] */ __RPC__in VARIANT *Pathname,
            /* [optional][in] */ __RPC__in VARIANT *FormatName);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_FormatName)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_FormatName )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFormatName);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_Machine)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_Machine )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachine);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_MessageCount)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_MessageCount )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plMessageCount);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_ForeignStatus)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_ForeignStatus )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plForeignStatus);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_QueueType)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_QueueType )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plQueueType);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_IsLocal)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_IsLocal )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsLocal);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_TransactionalStatus)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_TransactionalStatus )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plTransactionalStatus);
        
        DECLSPEC_XFGVIRT(IMSMQManagement, get_BytesInQueue)
        /* [id][propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BytesInQueue )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInQueue);
        
        DECLSPEC_XFGVIRT(IMSMQQueueManagement, get_JournalMessageCount)
        /* [propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_JournalMessageCount )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out long *plJournalMessageCount);
        
        DECLSPEC_XFGVIRT(IMSMQQueueManagement, get_BytesInJournal)
        /* [propget][helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *get_BytesInJournal )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBytesInJournal);
        
        DECLSPEC_XFGVIRT(IMSMQQueueManagement, EodGetReceiveInfo)
        /* [helpstringcontext] */ HRESULT ( STDMETHODCALLTYPE *EodGetReceiveInfo )( 
            __RPC__in IMSMQQueueManagement * This,
            /* [retval][out] */ __RPC__out VARIANT *pvCollection);
        
        END_INTERFACE
    } IMSMQQueueManagementVtbl;

    interface IMSMQQueueManagement
    {
        CONST_VTBL struct IMSMQQueueManagementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSMQQueueManagement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSMQQueueManagement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSMQQueueManagement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSMQQueueManagement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMSMQQueueManagement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMSMQQueueManagement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMSMQQueueManagement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMSMQQueueManagement_Init(This,Machine,Pathname,FormatName)	\
    ( (This)->lpVtbl -> Init(This,Machine,Pathname,FormatName) ) 

#define IMSMQQueueManagement_get_FormatName(This,pbstrFormatName)	\
    ( (This)->lpVtbl -> get_FormatName(This,pbstrFormatName) ) 

#define IMSMQQueueManagement_get_Machine(This,pbstrMachine)	\
    ( (This)->lpVtbl -> get_Machine(This,pbstrMachine) ) 

#define IMSMQQueueManagement_get_MessageCount(This,plMessageCount)	\
    ( (This)->lpVtbl -> get_MessageCount(This,plMessageCount) ) 

#define IMSMQQueueManagement_get_ForeignStatus(This,plForeignStatus)	\
    ( (This)->lpVtbl -> get_ForeignStatus(This,plForeignStatus) ) 

#define IMSMQQueueManagement_get_QueueType(This,plQueueType)	\
    ( (This)->lpVtbl -> get_QueueType(This,plQueueType) ) 

#define IMSMQQueueManagement_get_IsLocal(This,pfIsLocal)	\
    ( (This)->lpVtbl -> get_IsLocal(This,pfIsLocal) ) 

#define IMSMQQueueManagement_get_TransactionalStatus(This,plTransactionalStatus)	\
    ( (This)->lpVtbl -> get_TransactionalStatus(This,plTransactionalStatus) ) 

#define IMSMQQueueManagement_get_BytesInQueue(This,pvBytesInQueue)	\
    ( (This)->lpVtbl -> get_BytesInQueue(This,pvBytesInQueue) ) 


#define IMSMQQueueManagement_get_JournalMessageCount(This,plJournalMessageCount)	\
    ( (This)->lpVtbl -> get_JournalMessageCount(This,plJournalMessageCount) ) 

#define IMSMQQueueManagement_get_BytesInJournal(This,pvBytesInJournal)	\
    ( (This)->lpVtbl -> get_BytesInJournal(This,pvBytesInJournal) ) 

#define IMSMQQueueManagement_EodGetReceiveInfo(This,pvCollection)	\
    ( (This)->lpVtbl -> EodGetReceiveInfo(This,pvCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSMQQueueManagement_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MSMQQueueManagement;

#ifdef __cplusplus

class DECLSPEC_UUID("33b6d07e-f27d-42fa-b2d7-bf82e11e9374")
MSMQQueueManagement;
#endif
#endif /* __MSMQ_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mqoai_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mqoai_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mqoai_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


