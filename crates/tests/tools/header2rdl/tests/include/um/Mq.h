 
/*++

Copyright (c) 1996-2001, Microsoft Corporation

Module Name:

    mq.h

Abstract:

    Master include file for Message Queuing applications

--*/

#ifndef __MQ_H__
#define __MQ_H__

#if defined (_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef __ITransaction_FWD_DEFINED__
#define __ITransaction_FWD_DEFINED__
typedef interface ITransaction ITransaction;
#endif  // __ITransaction_FWD_DEFINED__


#ifdef __midl
// This is the PROPVARIANT definition for marshaling.
typedef struct tag_inner_PROPVARIANT tagMQPROPVARIANT;

#else
// This is the standard C layout of the PROPVARIANT.
typedef struct tagPROPVARIANT tagMQPROPVARIANT;
#endif
typedef tagMQPROPVARIANT MQPROPVARIANT;


#define	PRLT	( 0 )

#define	PRLE	( 1 )

#define	PRGT	( 2 )

#define	PRGE	( 3 )

#define	PREQ	( 4 )

#define	PRNE	( 5 )

typedef struct tagMQPROPERTYRESTRICTION
    {
    ULONG rel;
    PROPID prop;
    MQPROPVARIANT prval;
    } 	MQPROPERTYRESTRICTION;

typedef struct tagMQRESTRICTION
    {
    /* [range] */ ULONG cRes;
    /* [size_is] */ MQPROPERTYRESTRICTION *paPropRes;
    } 	MQRESTRICTION;

typedef struct tagMQCOLUMNSET
    {
    /* [range] */ ULONG cCol;
    /* [size_is] */ PROPID *aCol;
    } 	MQCOLUMNSET;

#define	QUERY_SORTASCEND	( 0 )

#define	QUERY_SORTDESCEND	( 1 )

typedef struct tagMQSORTKEY
    {
    PROPID propColumn;
    ULONG dwOrder;
    } 	MQSORTKEY;

typedef struct tagMQSORTSET
    {
    /* [range] */ ULONG cCol;
    /* [size_is] */ MQSORTKEY *aCol;
    } 	MQSORTSET;


typedef HANDLE QUEUEHANDLE;

typedef PROPID MSGPROPID;
typedef struct tagMQMSGPROPS
{
    DWORD           cProp;
    _Field_size_(cProp) MSGPROPID*      aPropID;
    _Field_size_(cProp) MQPROPVARIANT*  aPropVar;
    _Field_size_opt_(cProp) HRESULT*        aStatus;
} MQMSGPROPS;


typedef PROPID QUEUEPROPID;
typedef struct tagMQQUEUEPROPS
{
    DWORD           cProp;
    _Field_size_(cProp) QUEUEPROPID*    aPropID;
    _Field_size_(cProp) MQPROPVARIANT*  aPropVar;
    _Field_size_opt_(cProp) HRESULT*        aStatus;
} MQQUEUEPROPS;


typedef PROPID QMPROPID;
typedef struct tagMQQMPROPS
{
    DWORD           cProp;
    _Field_size_(cProp) QMPROPID*       aPropID;
    _Field_size_(cProp) MQPROPVARIANT*  aPropVar;
    _Field_size_opt_(cProp) HRESULT*        aStatus;
} MQQMPROPS;


typedef struct tagMQPRIVATEPROPS
{
    DWORD           cProp;
    _Field_size_(cProp) QMPROPID*       aPropID;
    _Field_size_(cProp) MQPROPVARIANT*  aPropVar;
    _Field_size_opt_(cProp) HRESULT*        aStatus;
} MQPRIVATEPROPS;


typedef PROPID MGMTPROPID;
typedef struct tagMQMGMTPROPS
{
    DWORD cProp;
    _Field_size_(cProp) MGMTPROPID* aPropID;
    _Field_size_(cProp) MQPROPVARIANT* aPropVar;
    _Field_size_opt_(cProp) HRESULT* aStatus;
} MQMGMTPROPS;

typedef struct tagSEQUENCE_INFO
{
    LONGLONG SeqID;
    ULONG SeqNo; 
    ULONG PrevNo;
} SEQUENCE_INFO;


typedef enum tagMQConnectionState
{
	MQCONN_NOFAILURE = 0, 				// connection is in the process of establishment, no failures have occured
	MQCONN_ESTABLISH_PACKET_RECEIVED, // NATIVE SESSION ONLY: connection establishment packed has been received
	
	MQCONN_READY, 						// connection has been successfully established	and is ready to send messages			
	
	MQCONN_UNKNOWN_FAILURE = 0x80000000, 	// exact reason for failure cannot be determined
	MQCONN_PING_FAILURE, 				// ping didnt answer (udp to 3527)
	MQCONN_CREATE_SOCKET_FAILURE,		// these 4 states represent failure to establish TCP connection on 1801
	MQCONN_BIND_SOCKET_FAILURE,
	MQCONN_CONNECT_SOCKET_FAILURE,
	MQCONN_TCP_NOT_ENABLED,
	MQCONN_SEND_FAILURE,				// send operation on a socket (WSASend) failed
	MQCONN_NOT_READY,					// send operation failed because connection is not ready
	MQCONN_NAME_RESOLUTION_FAILURE, 	// dns failure 
	MQCONN_INVALID_SERVER_CERT, 		// could not validate server certificate in https scenario
	MQCONN_LIMIT_REACHED, 				// connection limit reached, can't establish new session to a specific destination
	MQCONN_REFUSED_BY_OTHER_SIDE,		// connection refused by other side due to any reason(quota, invalid packet, connection limit reached)
	MQCONN_ROUTING_FAILURE, 			// absense of DS connectivity prevented getting routing data
	MQCONN_OUT_OF_MEMORY,				// failure due to low memory (e.g. bad_alloc exception when trying to send a packet) 
} MQConnectionState; 



//********************************************************************
//  API FLAGS
//********************************************************************

//
//  MQOpenQueue - Access values
//
// MQ_MOVE_ACCESS is used to open a handle which can be used only as
// target handle in MQMoveMessage.
// When doing access check for the move access mode, we use MQ_PEEK_ACCESS,
// thus avoiding problems of sharing.
//
#define MQ_RECEIVE_ACCESS       0x00000001
#define MQ_SEND_ACCESS          0x00000002
#define MQ_MOVE_ACCESS          0x00000004
#define MQ_PEEK_ACCESS          0x00000020
#define MQ_ADMIN_ACCESS         0x00000080

//
//  MQOpenQueue - Share values
//
#define MQ_DENY_NONE            0x00000000
#define MQ_DENY_RECEIVE_SHARE   0x00000001

//
//  MQReceiveMessage - Action values
//
#define MQ_ACTION_RECEIVE       0x00000000
#define MQ_ACTION_PEEK_CURRENT  0x80000000
#define MQ_ACTION_PEEK_NEXT     0x80000001

//
//  MQReceiveMessageByLookupId - Action values
//
#if(_WIN32_WINNT >= 0x0501)
#define MQ_LOOKUP_PEEK_CURRENT    0x40000010
#define MQ_LOOKUP_PEEK_NEXT       0x40000011
#define MQ_LOOKUP_PEEK_PREV       0x40000012
#define MQ_LOOKUP_PEEK_FIRST      0x40000014
#define MQ_LOOKUP_PEEK_LAST       0x40000018
#define MQ_LOOKUP_RECEIVE_CURRENT 0x40000020
#define MQ_LOOKUP_RECEIVE_NEXT    0x40000021
#define MQ_LOOKUP_RECEIVE_PREV    0x40000022
#define MQ_LOOKUP_RECEIVE_FIRST   0x40000024
#define MQ_LOOKUP_RECEIVE_LAST    0x40000028
#define MQ_LOOKUP_RECEIVE_ALLOW_PEEK  0x40000120 //TBD: exact value of this constant
#endif

//
// MQSendMessage,  MQReceiveMessage:  special cases for the transaction parameter
//
#define MQ_NO_TRANSACTION             NULL
#define MQ_MTS_TRANSACTION            (ITransaction *)1
#define MQ_XA_TRANSACTION             (ITransaction *)2
#define MQ_SINGLE_MESSAGE             (ITransaction *)3


//********************************************************************
//  PRIORITY LIMITS
//********************************************************************

//
//  Message priorities
//
#define MQ_MIN_PRIORITY          0    // Minimal message priority
#define MQ_MAX_PRIORITY          7    // Maximal message priority


//********************************************************************
//  MESSAGE PROPERTIES
//********************************************************************
#define PROPID_M_BASE                    0
#define PROPID_M_CLASS                   (PROPID_M_BASE + 1)     /* VT_UI2           */
#define PROPID_M_MSGID                   (PROPID_M_BASE + 2)     /* VT_UI1|VT_VECTOR */
#define PROPID_M_CORRELATIONID           (PROPID_M_BASE + 3)     /* VT_UI1|VT_VECTOR */
#define PROPID_M_PRIORITY                (PROPID_M_BASE + 4)     /* VT_UI1           */
#define PROPID_M_DELIVERY                (PROPID_M_BASE + 5)     /* VT_UI1           */
#define PROPID_M_ACKNOWLEDGE             (PROPID_M_BASE + 6)     /* VT_UI1           */
#define PROPID_M_JOURNAL                 (PROPID_M_BASE + 7)     /* VT_UI1           */
#define PROPID_M_APPSPECIFIC             (PROPID_M_BASE + 8)     /* VT_UI4           */
#define PROPID_M_BODY                    (PROPID_M_BASE + 9)     /* VT_UI1|VT_VECTOR */
#define PROPID_M_BODY_SIZE               (PROPID_M_BASE + 10)    /* VT_UI4           */
#define PROPID_M_LABEL                   (PROPID_M_BASE + 11)    /* VT_LPWSTR        */
#define PROPID_M_LABEL_LEN               (PROPID_M_BASE + 12)    /* VT_UI4           */
#define PROPID_M_TIME_TO_REACH_QUEUE     (PROPID_M_BASE + 13)    /* VT_UI4           */
#define PROPID_M_TIME_TO_BE_RECEIVED     (PROPID_M_BASE + 14)    /* VT_UI4           */
#define PROPID_M_RESP_QUEUE              (PROPID_M_BASE + 15)    /* VT_LPWSTR        */
#define PROPID_M_RESP_QUEUE_LEN          (PROPID_M_BASE + 16)    /* VT_UI4           */
#define PROPID_M_ADMIN_QUEUE             (PROPID_M_BASE + 17)    /* VT_LPWSTR        */
#define PROPID_M_ADMIN_QUEUE_LEN         (PROPID_M_BASE + 18)    /* VT_UI4           */
#define PROPID_M_VERSION                 (PROPID_M_BASE + 19)    /* VT_UI4           */
#define PROPID_M_SENDERID                (PROPID_M_BASE + 20)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_SENDERID_LEN            (PROPID_M_BASE + 21)    /* VT_UI4           */
#define PROPID_M_SENDERID_TYPE           (PROPID_M_BASE + 22)    /* VT_UI4           */
#define PROPID_M_PRIV_LEVEL              (PROPID_M_BASE + 23)    /* VT_UI4           */
#define PROPID_M_AUTH_LEVEL              (PROPID_M_BASE + 24)    /* VT_UI4           */
#define PROPID_M_AUTHENTICATED           (PROPID_M_BASE + 25)    /* VT_UI1           */
#define PROPID_M_HASH_ALG                (PROPID_M_BASE + 26)    /* VT_UI4           */
#define PROPID_M_ENCRYPTION_ALG          (PROPID_M_BASE + 27)    /* VT_UI4           */
#define PROPID_M_SENDER_CERT             (PROPID_M_BASE + 28)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_SENDER_CERT_LEN         (PROPID_M_BASE + 29)    /* VT_UI4           */
#define PROPID_M_SRC_MACHINE_ID          (PROPID_M_BASE + 30)    /* VT_CLSID         */
#define PROPID_M_SENTTIME                (PROPID_M_BASE + 31)    /* VT_UI4           */
#define PROPID_M_ARRIVEDTIME             (PROPID_M_BASE + 32)    /* VT_UI4           */
#define PROPID_M_DEST_QUEUE              (PROPID_M_BASE + 33)    /* VT_LPWSTR        */
#define PROPID_M_DEST_QUEUE_LEN          (PROPID_M_BASE + 34)    /* VT_UI4           */
#define PROPID_M_EXTENSION               (PROPID_M_BASE + 35)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_EXTENSION_LEN           (PROPID_M_BASE + 36)    /* VT_UI4           */
#define PROPID_M_SECURITY_CONTEXT        (PROPID_M_BASE + 37)    /* VT_UI4           */
#define PROPID_M_CONNECTOR_TYPE          (PROPID_M_BASE + 38)    /* VT_CLSID         */
#define PROPID_M_XACT_STATUS_QUEUE       (PROPID_M_BASE + 39)    /* VT_LPWSTR        */
#define PROPID_M_XACT_STATUS_QUEUE_LEN   (PROPID_M_BASE + 40)    /* VT_UI4           */
#define PROPID_M_TRACE                   (PROPID_M_BASE + 41)    /* VT_UI1           */
#define PROPID_M_BODY_TYPE               (PROPID_M_BASE + 42)    /* VT_UI4           */
#define PROPID_M_DEST_SYMM_KEY           (PROPID_M_BASE + 43)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_DEST_SYMM_KEY_LEN       (PROPID_M_BASE + 44)    /* VT_UI4           */
#define PROPID_M_SIGNATURE               (PROPID_M_BASE + 45)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_SIGNATURE_LEN           (PROPID_M_BASE + 46)    /* VT_UI4           */
#define PROPID_M_PROV_TYPE               (PROPID_M_BASE + 47)    /* VT_UI4           */
#define PROPID_M_PROV_NAME               (PROPID_M_BASE + 48)    /* VT_LPWSTR        */
#define PROPID_M_PROV_NAME_LEN           (PROPID_M_BASE + 49)    /* VT_UI4           */
#define PROPID_M_FIRST_IN_XACT           (PROPID_M_BASE + 50)    /* VT_UI1           */
#define PROPID_M_LAST_IN_XACT            (PROPID_M_BASE + 51)    /* VT_UI1           */
#define PROPID_M_XACTID                  (PROPID_M_BASE + 52)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_AUTHENTICATED_EX        (PROPID_M_BASE + 53)    /* VT_UI1           */
#if(_WIN32_WINNT >= 0x0501)
#define PROPID_M_RESP_FORMAT_NAME        (PROPID_M_BASE + 54)    /* VT_LPWSTR        */
#define PROPID_M_RESP_FORMAT_NAME_LEN    (PROPID_M_BASE + 55)    /* VT_UI4           */
#define PROPID_M_DEST_FORMAT_NAME        (PROPID_M_BASE + 58)    /* VT_LPWSTR        */
#define PROPID_M_DEST_FORMAT_NAME_LEN    (PROPID_M_BASE + 59)    /* VT_UI4           */
#define PROPID_M_LOOKUPID                (PROPID_M_BASE + 60)    /* VT_UI8           */
#define PROPID_M_SOAP_ENVELOPE           (PROPID_M_BASE + 61)    /* VT_LPWSTR        */
#define PROPID_M_SOAP_ENVELOPE_LEN       (PROPID_M_BASE + 62)    /* VT_UI4           */
#define PROPID_M_COMPOUND_MESSAGE        (PROPID_M_BASE + 63)    /* VT_UI1|VT_VECTOR */
#define PROPID_M_COMPOUND_MESSAGE_SIZE   (PROPID_M_BASE + 64)    /* VT_UI4           */
#define PROPID_M_SOAP_HEADER             (PROPID_M_BASE + 65)    /* VT_LPWSTR        */
#define PROPID_M_SOAP_BODY               (PROPID_M_BASE + 66)    /* VT_LPWSTR        */
#define PROPID_M_DEADLETTER_QUEUE        (PROPID_M_BASE + 67)    /* VT_LPWSTR        */
#define PROPID_M_DEADLETTER_QUEUE_LEN    (PROPID_M_BASE + 68)    /* VT_UI4           */
#define PROPID_M_ABORT_COUNT             (PROPID_M_BASE + 69)    /* VT_UI4           */
#define PROPID_M_MOVE_COUNT              (PROPID_M_BASE + 70)    /* VT_UI4           */
#define PROPID_M_LAST_MOVE_TIME          (PROPID_M_BASE + 75)    /* VT_UI4           */
#endif

//
// Message Property Size
//
#define PROPID_M_MSGID_SIZE         20
#define PROPID_M_CORRELATIONID_SIZE 20
#define PROPID_M_XACTID_SIZE        20


//********************************************************************
//  MESSAGE CLASS VALUES
//********************************************************************
//
//  Message class values are 16 bits laid out as follows:
//
//   1 1 1 1 1 1
//   5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +-+-+-+-------+-----------------+
//  |S|R|H| Rsrve |    Class code   |
//  +-+-+-+-------+-----------------+
//
//  where
//
//  Class code must be 9 bits, to accomodate http too. (40X, 50X codes).
//
//      S - is the severity flag
//          0 - Normal Message/Positive Acknowledgment (ACK)
//          1 - Negative Acknowledgment (NACK)
//
//      R - is the receive flag
//          0 - Arrival ACK/NACK
//          1 - Receive ACK/NACK
//
//      H - is http flag
//          0 - no http
//          1 - http
//
//      Top 4 bits of Class code are reserved for user-specified acks


#define MQCLASS_CODE(s, r, code) ((USHORT)(((s) << 15) | ((r) << 14) | (code)))
#define MQCLASS_NACK(c)     ((c) & 0x8000)
#define MQCLASS_RECEIVE(c)  ((c) & 0x4000)

#define MQCLASS_NACK_HTTP(c) (((c) & 0xA000) == 0xA000)



//
//  Normal message
//
#define MQMSG_CLASS_NORMAL                      MQCLASS_CODE(0, 0, 0x00)

//
//  Report message
//
#define MQMSG_CLASS_REPORT                      MQCLASS_CODE(0, 0, 0x01)

//
//  Arrival acknowledgment. The message has reached its destination queue
//
#define MQMSG_CLASS_ACK_REACH_QUEUE             MQCLASS_CODE(0, 0, 0x02)

//
//  Receive acknowledgment. The message has been received by an application
//
#define MQMSG_CLASS_ACK_RECEIVE                 MQCLASS_CODE(0, 1, 0x00)


//-----------------------------------------------
//
//  Negative arrival acknowledgments
//

//
//  Destination queue cannot be reached, the queue may have been deleted
//
#define MQMSG_CLASS_NACK_BAD_DST_Q              MQCLASS_CODE(1, 0, 0x00)

//
//  The message was purged before reaching its destination queue
//
#define MQMSG_CLASS_NACK_PURGED                 MQCLASS_CODE(1, 0, 0x01)

//
//  Time to reach queue has expired
//
#define MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT    MQCLASS_CODE(1, 0, 0x02)

//
//  The message has exceeded the queue quota
//
#define MQMSG_CLASS_NACK_Q_EXCEED_QUOTA         MQCLASS_CODE(1, 0, 0x03)

//
//  The sender does not have send access rights to the queue.
//
#define MQMSG_CLASS_NACK_ACCESS_DENIED          MQCLASS_CODE(1, 0, 0x04)

//
//  The message hop count was exceeded
//
#define MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED     MQCLASS_CODE(1, 0, 0x05)

//
//  The message signature is bad. The message could not be authenticated.
//
#define MQMSG_CLASS_NACK_BAD_SIGNATURE          MQCLASS_CODE(1, 0, 0x06)

//
//  The message could not be decrypted.
//
#define MQMSG_CLASS_NACK_BAD_ENCRYPTION         MQCLASS_CODE(1, 0, 0x07)

//
//  The message could not be encrypted for the destination.
//
#define MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT      MQCLASS_CODE(1, 0, 0x08)

//
//  The message was sent to a non-transactional queue within a transaction.
//
#define MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q    MQCLASS_CODE(1, 0, 0x09)

//
//  The message was sent to a transactional queue not within a transaction.
//
#define MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG  MQCLASS_CODE(1, 0, 0x0A)

//
//  The requested crypto provider for encryption is not supported by the destination.
//
#define MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER  MQCLASS_CODE(1, 0, 0x0B)

//
// The QM GUID has changed and therefore the messages was thrown away.
//
#define MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED MQCLASS_CODE(1, 0, 0x0C)

//
// The message was sent by downlevel machines (earlier than msmq4) and the message size
// with the subqueue and IPV6 headers exceeded the max message size
//
#define MQMSG_CLASS_NACK_MESSAGE_TOO_LARGE MQCLASS_CODE(1, 0, 0x0D)


//-----------------------------------------------
//
//  Negative receive acknowledgments
//

//
//  The queue was deleted, after the message arrived
//
#define MQMSG_CLASS_NACK_Q_DELETED              MQCLASS_CODE(1, 1, 0x00)

//
//  The message was purged at the destination queue
//
#define MQMSG_CLASS_NACK_Q_PURGED               MQCLASS_CODE(1, 1, 0x01)

//
//  Time to receive has expired while the message was still in its destination queue
//  (generated by destination)
//
#define MQMSG_CLASS_NACK_RECEIVE_TIMEOUT        MQCLASS_CODE(1, 1, 0x02)

//
//  Time to receive has expired while the message was still in its local outgoing queue
//  (generated locally by sender)
//
#define MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER  MQCLASS_CODE(1, 1, 0x03)

//
// Message was rejected by a receiving application (by calling MQMarkRejected API)
//
#define MQMSG_CLASS_NACK_RECEIVE_REJECTED       MQCLASS_CODE(1, 1, 0x04)



//------ PROPID_M_ACKNOWLEDGE ---------------
#define MQMSG_ACKNOWLEDGMENT_NONE           0x00

#define MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL    0x01
#define MQMSG_ACKNOWLEDGMENT_POS_RECEIVE    0x02
#define MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL    0x04
#define MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE    0x08

#define MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE ((UCHAR)( \
            MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL ))

#define MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE ((UCHAR)( \
            MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL |  \
            MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL ))

#define MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE ((UCHAR)( \
            MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL |  \
            MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE ))

#define MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE ((UCHAR)( \
            MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL |  \
            MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE |  \
            MQMSG_ACKNOWLEDGMENT_POS_RECEIVE ))

//------ PROPID_M_DELIVERY ------------------
#define MQMSG_DELIVERY_EXPRESS              0
#define MQMSG_DELIVERY_RECOVERABLE          1

//----- PROPID_M_JOURNAL --------------------
#define MQMSG_JOURNAL_NONE                  0
#define MQMSG_DEADLETTER                    1
#define MQMSG_JOURNAL                       2

//----- PROPID_M_TRACE ----------------------
#define MQMSG_TRACE_NONE                    0
#define MQMSG_SEND_ROUTE_TO_REPORT_QUEUE    1

//----- PROPID_M_SENDERID_TYPE --------------
#define MQMSG_SENDERID_TYPE_NONE            0
#define MQMSG_SENDERID_TYPE_SID             1

//----- PROPID_M_PRIV_LEVEL -----------------
#define MQMSG_PRIV_LEVEL_NONE               0
#define MQMSG_PRIV_LEVEL_BODY_BASE          0x01
#define MQMSG_PRIV_LEVEL_BODY_ENHANCED      0x03
#define MQMSG_PRIV_LEVEL_BODY_AES           0x05

//----- PROPID_M_AUTH_LEVEL -----------------
#define MQMSG_AUTH_LEVEL_NONE               0
#define MQMSG_AUTH_LEVEL_ALWAYS             1

//
// MQMSG_AUTH_LEVEL_MSMQxx are obsolete
// you should use MQMSG_AUTH_LEVEL_SIGxx
//
#define MQMSG_AUTH_LEVEL_MSMQ10             2
#define MQMSG_AUTH_LEVEL_MSMQ20             4

#define MQMSG_AUTH_LEVEL_SIG10              2
#define MQMSG_AUTH_LEVEL_SIG20              4
#define MQMSG_AUTH_LEVEL_SIG30              8


//----- PROPID_M_AUTHENTICATED -----------------
//----- PROPID_M_AUTHENTICATED_EX --------------
#define MQMSG_AUTHENTICATION_NOT_REQUESTED  0
#define MQMSG_AUTHENTICATION_REQUESTED      1

//
// MQMSG_AUTHENTICATION_REQUESTED_EX is obsolete
// use the values MQMSG_AUTHENTICATED_SIGxx
// for PROPID_M_AUTHENTICATED_EX
//
#define MQMSG_AUTHENTICATION_REQUESTED_EX   3

#define MQMSG_AUTHENTICATED_SIG10           1
#define MQMSG_AUTHENTICATED_SIG20           3
#define MQMSG_AUTHENTICATED_SIG30           5
#define MQMSG_AUTHENTICATED_SIGXML          9
#define MQMSG_AUTHENTICATED_QM_MESSAGE      11


//----- PROPID_M_FIRST_IN_XACT --------------
#define MQMSG_NOT_FIRST_IN_XACT             0
#define MQMSG_FIRST_IN_XACT                 1

//----- PROPID_M_LAST_IN_XACT  --------------
#define MQMSG_NOT_LAST_IN_XACT              0
#define MQMSG_LAST_IN_XACT                  1



//********************************************************************
//  QUEUE PROPERTIES
//********************************************************************
#define PROPID_Q_BASE           100
#define PROPID_Q_INSTANCE       (PROPID_Q_BASE +  1)  /* VT_CLSID     */
#define PROPID_Q_TYPE           (PROPID_Q_BASE +  2)  /* VT_CLSID     */
#define PROPID_Q_PATHNAME       (PROPID_Q_BASE +  3)  /* VT_LPWSTR    */
#define PROPID_Q_JOURNAL        (PROPID_Q_BASE +  4)  /* VT_UI1       */
#define PROPID_Q_QUOTA          (PROPID_Q_BASE +  5)  /* VT_UI4       */
#define PROPID_Q_BASEPRIORITY   (PROPID_Q_BASE +  6)  /* VT_I2        */
#define PROPID_Q_JOURNAL_QUOTA  (PROPID_Q_BASE +  7)  /* VT_UI4       */
#define PROPID_Q_LABEL          (PROPID_Q_BASE +  8)  /* VT_LPWSTR    */
#define PROPID_Q_CREATE_TIME    (PROPID_Q_BASE +  9)  /* VT_I4        */
#define PROPID_Q_MODIFY_TIME    (PROPID_Q_BASE + 10)  /* VT_I4        */
#define PROPID_Q_AUTHENTICATE   (PROPID_Q_BASE + 11)  /* VT_UI1       */
#define PROPID_Q_PRIV_LEVEL     (PROPID_Q_BASE + 12)  /* VT_UI4       */
#define PROPID_Q_TRANSACTION    (PROPID_Q_BASE + 13)  /* VT_UI1       */
#define PROPID_Q_PATHNAME_DNS  (PROPID_Q_BASE + 24)  /* VT_LPWSTR    */
#define PROPID_Q_MULTICAST_ADDRESS (PROPID_Q_BASE + 25)  /* VT_LPWSTR */
#define PROPID_Q_ADS_PATH      (PROPID_Q_BASE + 26)  /* VT_LPWSTR    */


//----- PROPID_Q_JOURNAL ------------------
#define MQ_JOURNAL_NONE     (unsigned char)0
#define MQ_JOURNAL          (unsigned char)1

//----- PROPID_Q_TYPE ------------------
//  {55EE8F32-CCE9-11cf-B108-0020AFD61CE9}
#define MQ_QTYPE_REPORT {0x55ee8f32, 0xcce9, 0x11cf, \
                        {0xb1, 0x8, 0x0, 0x20, 0xaf, 0xd6, 0x1c, 0xe9}}

//  {55EE8F33-CCE9-11cf-B108-0020AFD61CE9}
#define MQ_QTYPE_TEST   {0x55ee8f33, 0xcce9, 0x11cf, \
                        {0xb1, 0x8, 0x0, 0x20, 0xaf, 0xd6, 0x1c, 0xe9}}

//----- PROPID_Q_TRANSACTION ------------------
#define MQ_TRANSACTIONAL_NONE     (unsigned char)0
#define MQ_TRANSACTIONAL          (unsigned char)1

//----- PROPID_Q_AUTHENTICATE ------------------
#define MQ_AUTHENTICATE_NONE      (unsigned char)0
#define MQ_AUTHENTICATE           (unsigned char)1

//----- PROPID_Q_PRIV_LEVEL ------------------
#define MQ_PRIV_LEVEL_NONE        (unsigned long)0
#define MQ_PRIV_LEVEL_OPTIONAL    (unsigned long)1
#define MQ_PRIV_LEVEL_BODY        (unsigned long)2


//********************************************************************
//  MACHINE PROPERTIES
//********************************************************************
#define PROPID_QM_BASE 200

#define PROPID_QM_SITE_ID                   (PROPID_QM_BASE +  1) /* VT_CLSID            */
#define PROPID_QM_MACHINE_ID                (PROPID_QM_BASE +  2) /* VT_CLSID            */
#define PROPID_QM_PATHNAME                  (PROPID_QM_BASE +  3) /* VT_LPWSTR           */
#define PROPID_QM_CONNECTION                (PROPID_QM_BASE +  4) /* VT_LPWSTR|VT_VECTOR */
#define PROPID_QM_ENCRYPTION_PK             (PROPID_QM_BASE +  5) /* VT_UI1|VT_VECTOR    */
#define PROPID_QM_ENCRYPTION_PK_BASE        (PROPID_QM_BASE + 31)  /* VT_UI1|VT_VECTOR  */
#define PROPID_QM_ENCRYPTION_PK_ENHANCED    (PROPID_QM_BASE + 32)  /* VT_UI1|VT_VECTOR  */
#define PROPID_QM_PATHNAME_DNS              (PROPID_QM_BASE + 33)  /* VT_LPWSTR         */
#define PROPID_QM_ENCRYPTION_PK_AES        (PROPID_QM_BASE + 44)  /* VT_UI1|VT_VECTOR  */

//********************************************************************
//  PRIVATE COMPUTER PROPERTIES
//********************************************************************
#define PROPID_PC_BASE 5800

#define PROPID_PC_VERSION             (PROPID_PC_BASE + 1) /* VT_UI4            */
#define PROPID_PC_DS_ENABLED          (PROPID_PC_BASE + 2) /* VT_BOOL           */

//********************************************************************
//  LOCAL ADMIN MSMQ MACHINE PROPERTIES
//********************************************************************
#define PROPID_MGMT_MSMQ_BASE           0
#define PROPID_MGMT_MSMQ_ACTIVEQUEUES   (PROPID_MGMT_MSMQ_BASE + 1) /* VT_LPWSTR | VT_VECTOR  */
#define PROPID_MGMT_MSMQ_PRIVATEQ       (PROPID_MGMT_MSMQ_BASE + 2) /* VT_LPWSTR | VT_VECTOR  */
#define PROPID_MGMT_MSMQ_DSSERVER       (PROPID_MGMT_MSMQ_BASE + 3) /* VT_LPWSTR */
#define PROPID_MGMT_MSMQ_CONNECTED      (PROPID_MGMT_MSMQ_BASE + 4) /* VT_LPWSTR */
#define PROPID_MGMT_MSMQ_TYPE           (PROPID_MGMT_MSMQ_BASE + 5) /* VT_LPWSTR */
#define PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES (PROPID_MGMT_QUEUE_BASE + 6)    /* VT_UI8    */


//
// Returned Value for PROPID_MGMT_MSMQ_CONNECTED property
//
#define MSMQ_CONNECTED      L"CONNECTED"
#define MSMQ_DISCONNECTED   L"DISCONNECTED"


//********************************************************************
//  LOCAL ADMIN MSMQ QUEUE PROPERTIES
//********************************************************************
#define PROPID_MGMT_QUEUE_BASE                  0
#define PROPID_MGMT_QUEUE_PATHNAME              (PROPID_MGMT_QUEUE_BASE + 1)    /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_FORMATNAME            (PROPID_MGMT_QUEUE_BASE + 2)    /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_TYPE                  (PROPID_MGMT_QUEUE_BASE + 3)    /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_LOCATION              (PROPID_MGMT_QUEUE_BASE + 4)    /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_XACT                  (PROPID_MGMT_QUEUE_BASE + 5)    /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_FOREIGN               (PROPID_MGMT_QUEUE_BASE + 6)    /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_MESSAGE_COUNT         (PROPID_MGMT_QUEUE_BASE + 7)    /* VT_UI4    */
#define PROPID_MGMT_QUEUE_BYTES_IN_QUEUE        (PROPID_MGMT_QUEUE_BASE + 8)    /* VT_UI4    */
#define PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT (PROPID_MGMT_QUEUE_BASE + 9)    /* VT_UI4    */
#define PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL      (PROPID_MGMT_QUEUE_BASE + 10)   /* VT_UI4    */
#define PROPID_MGMT_QUEUE_STATE                 (PROPID_MGMT_QUEUE_BASE + 11)   /* VT_LPWSTR */
#define PROPID_MGMT_QUEUE_NEXTHOPS              (PROPID_MGMT_QUEUE_BASE + 12)   /* VT_LPWSTR|VT_VECTOR  */
#define PROPID_MGMT_QUEUE_EOD_LAST_ACK          (PROPID_MGMT_QUEUE_BASE + 13)   /* VT_BLOB   */
#define PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME     (PROPID_MGMT_QUEUE_BASE + 14)   /* VT_I4     */
#define PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT    (PROPID_MGMT_QUEUE_BASE + 15)   /* VT_UI4    */
#define PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK     (PROPID_MGMT_QUEUE_BASE + 16)   /* VT_BLOB   */
#define PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK      (PROPID_MGMT_QUEUE_BASE + 17)   /* VT_BLOB   */
#define PROPID_MGMT_QUEUE_EOD_NEXT_SEQ          (PROPID_MGMT_QUEUE_BASE + 18)   /* VT_BLOB   */
#define PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT     (PROPID_MGMT_QUEUE_BASE + 19)   /* VT_UI4    */
#define PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT      (PROPID_MGMT_QUEUE_BASE + 20)   /* VT_UI4    */
#define PROPID_MGMT_QUEUE_EOD_RESEND_TIME       (PROPID_MGMT_QUEUE_BASE + 21)   /* VT_I4     */
#define PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL   (PROPID_MGMT_QUEUE_BASE + 22)   /* VT_UI4    */
#define PROPID_MGMT_QUEUE_EOD_RESEND_COUNT      (PROPID_MGMT_QUEUE_BASE + 23)   /* VT_UI4    */
#define PROPID_MGMT_QUEUE_EOD_SOURCE_INFO       (PROPID_MGMT_QUEUE_BASE + 24)   /* VT_VARIANT|VT_VECTOR */
#define PROPID_MGMT_QUEUE_CONNECTION_HISTORY    (PROPID_MGMT_QUEUE_BASE + 25)   /* VT_BLOB | VT_VECTOR */
#define PROPID_MGMT_QUEUE_SUBQUEUE_COUNT        (PROPID_MGMT_QUEUE_BASE + 26)   /* VT_UI4 */
#define PROPID_MGMT_QUEUE_SUBQUEUE_NAMES        (PROPID_MGMT_QUEUE_BASE + 27)   /* VT_LPWSTR|VT_VECTOR  */


//
// Alternative names for "Bytes in ..."
//
#define PROPID_MGMT_QUEUE_USED_QUOTA            PROPID_MGMT_QUEUE_BYTES_IN_QUEUE
#define PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA    PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL

//
// Returned value for PROPID_MGMT_QUEUE_TYPE
//
#define MGMT_QUEUE_TYPE_PUBLIC      L"PUBLIC"
#define MGMT_QUEUE_TYPE_PRIVATE     L"PRIVATE"
#define MGMT_QUEUE_TYPE_MACHINE     L"MACHINE"
#define MGMT_QUEUE_TYPE_CONNECTOR   L"CONNECTOR"
#define MGMT_QUEUE_TYPE_MULTICAST   L"MULTICAST"

//
// Returned value for PROPID_MGMT_QUEUE_STATE
//
#define MGMT_QUEUE_STATE_LOCAL          L"LOCAL CONNECTION"
#define MGMT_QUEUE_STATE_NONACTIVE      L"INACTIVE"
#define MGMT_QUEUE_STATE_WAITING        L"WAITING"
#define MGMT_QUEUE_STATE_NEED_VALIDATE  L"NEED VALIDATION"
#define MGMT_QUEUE_STATE_ONHOLD         L"ONHOLD"
#define MGMT_QUEUE_STATE_CONNECTED      L"CONNECTED"
#define MGMT_QUEUE_STATE_DISCONNECTING  L"DISCONNECTING"
#define MGMT_QUEUE_STATE_DISCONNECTED   L"DISCONNECTED"
#define MGMT_QUEUE_STATE_LOCKED   L"LOCKED"

//
// Returned value for PROPID_MGMT_QUEUE_LOCATION
//
#define MGMT_QUEUE_LOCAL_LOCATION   L"LOCAL"
#define MGMT_QUEUE_REMOTE_LOCATION  L"REMOTE"

//
// Returned Value for PROPID_MGMT_QUEUE_XACT and PROPID_MGMT_QUEUE_FOREIGN
//

#define MGMT_QUEUE_UNKNOWN_TYPE     L"UNKNOWN"

//
// Obselete names left for backword compatibility.
//

#define MGMT_QUEUE_CORRECT_TYPE     L"YES"
#define MGMT_QUEUE_INCORRECT_TYPE   L"NO"

//
// Names for Returned Value for PROPID_MGMT_QUEUE_XACT
//

//#define MGMT_QUEUE_UNKNOWN_TYPE       L"UNKNOWN"
#define MGMT_QUEUE_TRANSACTIONAL_TYPE   L"YES"
#define MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE   L"NO"

//
// Names for Returned Value for PROPID_MGMT_QUEUE_FOREIGN
//

//#define MGMT_QUEUE_UNKNOWN_TYPE       L"UNKNOWN"
#define MGMT_QUEUE_FOREIGN_TYPE         L"YES"
#define MGMT_QUEUE_NOT_FOREIGN_TYPE     L"NO"

//
// Object parameter values for MQMgmtAction API
//
#define MO_MACHINE_TOKEN    L"MACHINE"
#define MO_QUEUE_TOKEN      L"QUEUE"

//
// Action parameter values for MQMgmtAction API
//
#define MACHINE_ACTION_CONNECT      L"CONNECT"
#define MACHINE_ACTION_DISCONNECT   L"DISCONNECT"
#define MACHINE_ACTION_TIDY         L"TIDY"

#define QUEUE_ACTION_PAUSE      L"PAUSE"
#define QUEUE_ACTION_RESUME     L"RESUME"
#define QUEUE_ACTION_EOD_RESEND L"EOD_RESEND"

//
// LONG_LIVED is the default for PROPID_M_TIME_TO_REACH_QUEUE. If calls
// to MQSendMessage() specify this value, or not specify this property at
// all, then the actual timeout is obtained from Active Directory.
//
#define LONG_LIVED    0xfffffffe

#define MQ_MAX_Q_NAME_LEN      124   // Maximal WCHAR length of a queue name.
#define MQ_MAX_Q_LABEL_LEN     124
#define MQ_MAX_MSG_LABEL_LEN   250


//+-----------------------------------------
//
// Flags for MQRegisterCertificate()
//
//+-----------------------------------------

#define MQCERT_REGISTER_ALWAYS        0x01
#define MQCERT_REGISTER_IF_NOT_EXIST  0x02


//********************************************************************
//  SECURITY Flags (Queue access control)
//********************************************************************

#define MQSEC_DELETE_MESSAGE                0x1
#define MQSEC_PEEK_MESSAGE                  0x2
#define MQSEC_WRITE_MESSAGE                 0x4
#define MQSEC_DELETE_JOURNAL_MESSAGE        0x8
#define MQSEC_SET_QUEUE_PROPERTIES          0x10
#define MQSEC_GET_QUEUE_PROPERTIES          0x20
#define MQSEC_DELETE_QUEUE                  DELETE
#define MQSEC_GET_QUEUE_PERMISSIONS         READ_CONTROL
#define MQSEC_CHANGE_QUEUE_PERMISSIONS      WRITE_DAC
#define MQSEC_TAKE_QUEUE_OWNERSHIP          WRITE_OWNER

#define MQSEC_RECEIVE_MESSAGE               (MQSEC_DELETE_MESSAGE | \
                                             MQSEC_PEEK_MESSAGE)

#define MQSEC_RECEIVE_JOURNAL_MESSAGE       (MQSEC_DELETE_JOURNAL_MESSAGE | \
                                             MQSEC_PEEK_MESSAGE)

#define MQSEC_QUEUE_GENERIC_READ            (MQSEC_GET_QUEUE_PROPERTIES | \
                                             MQSEC_GET_QUEUE_PERMISSIONS | \
                                             MQSEC_RECEIVE_MESSAGE | \
                                             MQSEC_RECEIVE_JOURNAL_MESSAGE)

#define MQSEC_QUEUE_GENERIC_WRITE           (MQSEC_GET_QUEUE_PROPERTIES | \
                                             MQSEC_GET_QUEUE_PERMISSIONS | \
                                             MQSEC_WRITE_MESSAGE)

#define MQSEC_QUEUE_GENERIC_EXECUTE         0

#define MQSEC_QUEUE_GENERIC_ALL             (MQSEC_RECEIVE_MESSAGE | \
                                             MQSEC_RECEIVE_JOURNAL_MESSAGE | \
                                             MQSEC_WRITE_MESSAGE | \
                                             MQSEC_SET_QUEUE_PROPERTIES | \
                                             MQSEC_GET_QUEUE_PROPERTIES | \
                                             MQSEC_DELETE_QUEUE | \
                                             MQSEC_GET_QUEUE_PERMISSIONS | \
                                             MQSEC_CHANGE_QUEUE_PERMISSIONS | \
                                             MQSEC_TAKE_QUEUE_OWNERSHIP)

/////////////////////////////////////////////////////////////////////////
//
// Message Queuing Success values
//
//
/////////////////////////////////////////////////////////////////////////

#define MQ_OK                       ((HRESULT)0L)

/////////////////////////////////////////////////////////////////////////
//
// Message Queuing Information values
//
//
/////////////////////////////////////////////////////////////////////////

//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//


//
// Define the severity codes
//


//
// MessageId: MQ_INFORMATION_PROPERTY
//
// MessageText:
//
// One or more of the properties passed resulted in a warning, but the function completed.
//
#define MQ_INFORMATION_PROPERTY          ((HRESULT)0x400E0001L)

//
// MessageId: MQ_INFORMATION_ILLEGAL_PROPERTY
//
// MessageText:
//
// The property ID is invalid.
//
#define MQ_INFORMATION_ILLEGAL_PROPERTY  ((HRESULT)0x400E0002L)

//
// MessageId: MQ_INFORMATION_PROPERTY_IGNORED
//
// MessageText:
//
// The property specified was ignored for this operation (this occurs,
// for example, when PROPID_M_SENDERID is passed to SendMessage()).
//
#define MQ_INFORMATION_PROPERTY_IGNORED  ((HRESULT)0x400E0003L)

//
// MessageId: MQ_INFORMATION_UNSUPPORTED_PROPERTY
//
// MessageText:
//
// The property specified is not supported and was ignored for this operation.
//
#define MQ_INFORMATION_UNSUPPORTED_PROPERTY ((HRESULT)0x400E0004L)

//
// MessageId: MQ_INFORMATION_DUPLICATE_PROPERTY
//
// MessageText:
//
// The property specified is already in the property identifier array.
// The duplicate was ignored for this operation.
//
#define MQ_INFORMATION_DUPLICATE_PROPERTY ((HRESULT)0x400E0005L)

//
// MessageId: MQ_INFORMATION_OPERATION_PENDING
//
// MessageText:
//
// An asynchronous operation is currently pending.
//
#define MQ_INFORMATION_OPERATION_PENDING ((HRESULT)0x400E0006L)

//
// MessageId: MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL
//
// MessageText:
//
// The format name buffer supplied to MQCreateQueue was too small
// to hold the format name, however the queue was created successfully.
//
#define MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL ((HRESULT)0x400E0009L)

//
// MessageId: MQ_INFORMATION_INTERNAL_USER_CERT_EXIST
//
// MessageText:
//
// An internal Message Queuing certificate already exists for this user.
//
#define MQ_INFORMATION_INTERNAL_USER_CERT_EXIST ((HRESULT)0x400E000AL)

//
// MessageId: MQ_INFORMATION_OWNER_IGNORED
//
// MessageText:
//
// The queue owner was not set during the processing of this call to MQSetQueueSecurity().
//
#define MQ_INFORMATION_OWNER_IGNORED     ((HRESULT)0x400E000BL)

/////////////////////////////////////////////////////////////////////////
//
//  Message Queuing Error values
//
//
/////////////////////////////////////////////////////////////////////////

//
// MessageId: MQ_ERROR
//
// MessageText:
//
// Generic error code.
//
#define MQ_ERROR                         ((HRESULT)0xC00E0001L)

//
// MessageId: MQ_ERROR_PROPERTY
//
// MessageText:
//
// One or more of the properties passed are invalid.
//
#define MQ_ERROR_PROPERTY                ((HRESULT)0xC00E0002L)

//
// MessageId: MQ_ERROR_QUEUE_NOT_FOUND
//
// MessageText:
//
// The queue does not exist or you do not have sufficient permissions to perform the operation.
//
#define MQ_ERROR_QUEUE_NOT_FOUND         ((HRESULT)0xC00E0003L)

//
// MessageId: MQ_ERROR_QUEUE_NOT_ACTIVE
//
// MessageText:
//
// The queue is not open or may not exist.
//
#define MQ_ERROR_QUEUE_NOT_ACTIVE        ((HRESULT)0xC00E0004L)

//
// MessageId: MQ_ERROR_QUEUE_EXISTS
//
// MessageText:
//
// A queue with the same path name already exists.
//
#define MQ_ERROR_QUEUE_EXISTS            ((HRESULT)0xC00E0005L)

//
// MessageId: MQ_ERROR_INVALID_PARAMETER
//
// MessageText:
//
// An invalid parameter was passed to a function.
//
#define MQ_ERROR_INVALID_PARAMETER       ((HRESULT)0xC00E0006L)

//
// MessageId: MQ_ERROR_INVALID_HANDLE
//
// MessageText:
//
// An invalid handle was passed to a function.
//
#define MQ_ERROR_INVALID_HANDLE          ((HRESULT)0xC00E0007L)

//
// MessageId: MQ_ERROR_OPERATION_CANCELLED
//
// MessageText:
//
// The operation was canceled before it could be completed.
//
#define MQ_ERROR_OPERATION_CANCELLED     ((HRESULT)0xC00E0008L)

//
// MessageId: MQ_ERROR_SHARING_VIOLATION
//
// MessageText:
//
// There is a sharing violation. The queue is already open for exclusive retrieval.
//
#define MQ_ERROR_SHARING_VIOLATION       ((HRESULT)0xC00E0009L)

//
// MessageId: MQ_ERROR_SERVICE_NOT_AVAILABLE
//
// MessageText:
//
// The Message Queuing service is not available
//
#define MQ_ERROR_SERVICE_NOT_AVAILABLE   ((HRESULT)0xC00E000BL)

//
// MessageId: MQ_ERROR_MACHINE_NOT_FOUND
//
// MessageText:
//
// The computer specified cannot be found.
//
#define MQ_ERROR_MACHINE_NOT_FOUND       ((HRESULT)0xC00E000DL)

//
// MessageId: MQ_ERROR_ILLEGAL_SORT
//
// MessageText:
//
// The sort operation specified in MQLocateBegin is invalid (for example, there are duplicate columns).
//
#define MQ_ERROR_ILLEGAL_SORT            ((HRESULT)0xC00E0010L)

//
// MessageId: MQ_ERROR_ILLEGAL_USER
//
// MessageText:
//
// The user specified is not a valid user.
//
#define MQ_ERROR_ILLEGAL_USER            ((HRESULT)0xC00E0011L)

//
// MessageId: MQ_ERROR_NO_DS
//
// MessageText:
//
// A connection with Active Directory Domain Services cannot be established. Verify that there are sufficient permissions to perform this operation.
//
#define MQ_ERROR_NO_DS                   ((HRESULT)0xC00E0013L)

//
// MessageId: MQ_ERROR_ILLEGAL_QUEUE_PATHNAME
//
// MessageText:
//
// The queue path name specified is invalid.
//
#define MQ_ERROR_ILLEGAL_QUEUE_PATHNAME  ((HRESULT)0xC00E0014L)

//
// MessageId: MQ_ERROR_ILLEGAL_PROPERTY_VALUE
//
// MessageText:
//
// The property value specified is invalid.
//
#define MQ_ERROR_ILLEGAL_PROPERTY_VALUE  ((HRESULT)0xC00E0018L)

//
// MessageId: MQ_ERROR_ILLEGAL_PROPERTY_VT
//
// MessageText:
//
// The VARTYPE value specified is invalid.
//
#define MQ_ERROR_ILLEGAL_PROPERTY_VT     ((HRESULT)0xC00E0019L)

//
// MessageId: MQ_ERROR_BUFFER_OVERFLOW
//
// MessageText:
//
// The buffer supplied to MQReceiveMessage for message property retrieval
// is too small. The message was not removed from the queue, but the part
// of the message property that was in the buffer was copied.
//
#define MQ_ERROR_BUFFER_OVERFLOW         ((HRESULT)0xC00E001AL)

//
// MessageId: MQ_ERROR_IO_TIMEOUT
//
// MessageText:
//
// The time specified for MQReceiveMessage to wait for the message elapsed.
//
#define MQ_ERROR_IO_TIMEOUT              ((HRESULT)0xC00E001BL)

//
// MessageId: MQ_ERROR_ILLEGAL_CURSOR_ACTION
//
// MessageText:
//
// The MQ_ACTION_PEEK_NEXT value specified for MQReceiveMessage cannot be used with
// the current cursor position.
//
#define MQ_ERROR_ILLEGAL_CURSOR_ACTION   ((HRESULT)0xC00E001CL)

//
// MessageId: MQ_ERROR_MESSAGE_ALREADY_RECEIVED
//
// MessageText:
//
// The message at which the cursor is currently pointing was removed from
// the queue by another process or by another call to MQReceiveMessage
// without the use of this cursor.
//
#define MQ_ERROR_MESSAGE_ALREADY_RECEIVED ((HRESULT)0xC00E001DL)

//
// MessageId: MQ_ERROR_ILLEGAL_FORMATNAME
//
// MessageText:
//
// The format name specified is invalid.
//
#define MQ_ERROR_ILLEGAL_FORMATNAME      ((HRESULT)0xC00E001EL)

//
// MessageId: MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL
//
// MessageText:
//
// The format name buffer supplied to the API was too small
// to hold the format name.
//
#define MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL ((HRESULT)0xC00E001FL)

//
// MessageId: MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION
//
// MessageText:
//
// Operations of the type requested (for example, deleting a queue using a direct format name)
// are not supported for the format name specified.
//
#define MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION ((HRESULT)0xC00E0020L)

//
// MessageId: MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR
//
// MessageText:
//
// The specified security descriptor is invalid.
//
#define MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR ((HRESULT)0xC00E0021L)

//
// MessageId: MQ_ERROR_SENDERID_BUFFER_TOO_SMALL
//
// MessageText:
//
// The size of the buffer for the user ID property is too small.
//
#define MQ_ERROR_SENDERID_BUFFER_TOO_SMALL ((HRESULT)0xC00E0022L)

//
// MessageId: MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL
//
// MessageText:
//
// The size of the buffer passed to MQGetQueueSecurity is too small.
//
#define MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL ((HRESULT)0xC00E0023L)

//
// MessageId: MQ_ERROR_CANNOT_IMPERSONATE_CLIENT
//
// MessageText:
//
// The security credentials cannot be verified because the RPC server
// cannot impersonate the client application.
//
#define MQ_ERROR_CANNOT_IMPERSONATE_CLIENT ((HRESULT)0xC00E0024L)

//
// MessageId: MQ_ERROR_ACCESS_DENIED
//
// MessageText:
//
// Access is denied.
//
#define MQ_ERROR_ACCESS_DENIED           ((HRESULT)0xC00E0025L)

//
// MessageId: MQ_ERROR_PRIVILEGE_NOT_HELD
//
// MessageText:
//
// The client does not have sufficient security privileges to perform the operation.
//
#define MQ_ERROR_PRIVILEGE_NOT_HELD      ((HRESULT)0xC00E0026L)

//
// MessageId: MQ_ERROR_INSUFFICIENT_RESOURCES
//
// MessageText:
//
// There are insufficient resources to perform this operation.
//
#define MQ_ERROR_INSUFFICIENT_RESOURCES  ((HRESULT)0xC00E0027L)

//
// MessageId: MQ_ERROR_USER_BUFFER_TOO_SMALL
//
// MessageText:
//
// The request failed because the user buffer is too small to hold the information returned.
//
#define MQ_ERROR_USER_BUFFER_TOO_SMALL   ((HRESULT)0xC00E0028L)

//
// MessageId: MQ_ERROR_MESSAGE_STORAGE_FAILED
//
// MessageText:
//
// A recoverable or journal message could not be stored. The message was not sent.
//
#define MQ_ERROR_MESSAGE_STORAGE_FAILED  ((HRESULT)0xC00E002AL)

//
// MessageId: MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL
//
// MessageText:
//
// The buffer for the user certificate property is too small.
//
#define MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL ((HRESULT)0xC00E002BL)

//
// MessageId: MQ_ERROR_INVALID_CERTIFICATE
//
// MessageText:
//
// The user certificate is invalid.
//
#define MQ_ERROR_INVALID_CERTIFICATE     ((HRESULT)0xC00E002CL)

//
// MessageId: MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE
//
// MessageText:
//
// The internal Message Queuing certificate is corrupted.
//
#define MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE ((HRESULT)0xC00E002DL)

//
// MessageId: MQ_ERROR_INTERNAL_USER_CERT_EXIST
//
// MessageText:
//
// An internal Message Queuing certificate already exists for this user.
//
#define MQ_ERROR_INTERNAL_USER_CERT_EXIST ((HRESULT)0xC00E002EL)

//
// MessageId: MQ_ERROR_NO_INTERNAL_USER_CERT
//
// MessageText:
//
// No internal Message Queuing certificate exists for the user.
//
#define MQ_ERROR_NO_INTERNAL_USER_CERT   ((HRESULT)0xC00E002FL)

//
// MessageId: MQ_ERROR_CORRUPTED_SECURITY_DATA
//
// MessageText:
//
// A cryptographic function failed.
//
#define MQ_ERROR_CORRUPTED_SECURITY_DATA ((HRESULT)0xC00E0030L)

//
// MessageId: MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE
//
// MessageText:
//
// The personal certificate store is corrupted.
//
#define MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE ((HRESULT)0xC00E0031L)

//
// MessageId: MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION
//
// MessageText:
//
// The computer does not support encryption operations.
//
#define MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION ((HRESULT)0xC00E0033L)

//
// MessageId: MQ_ERROR_BAD_SECURITY_CONTEXT
//
// MessageText:
//
// The security context is invalid.
//
#define MQ_ERROR_BAD_SECURITY_CONTEXT    ((HRESULT)0xC00E0035L)

//
// MessageId: MQ_ERROR_COULD_NOT_GET_USER_SID
//
// MessageText:
//
// The SID cannot be obtained from the thread token.
//
#define MQ_ERROR_COULD_NOT_GET_USER_SID  ((HRESULT)0xC00E0036L)

//
// MessageId: MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO
//
// MessageText:
//
// The account information for the user cannot be obtained.
//
#define MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO ((HRESULT)0xC00E0037L)

//
// MessageId: MQ_ERROR_ILLEGAL_MQCOLUMNS
//
// MessageText:
//
// The MQCOLUMNS parameter is invalid.
//
#define MQ_ERROR_ILLEGAL_MQCOLUMNS       ((HRESULT)0xC00E0038L)

//
// MessageId: MQ_ERROR_ILLEGAL_PROPID
//
// MessageText:
//
// A property identifier is invalid.
//
#define MQ_ERROR_ILLEGAL_PROPID          ((HRESULT)0xC00E0039L)

//
// MessageId: MQ_ERROR_ILLEGAL_RELATION
//
// MessageText:
//
// A relationship parameter is invalid.
//
#define MQ_ERROR_ILLEGAL_RELATION        ((HRESULT)0xC00E003AL)

//
// MessageId: MQ_ERROR_ILLEGAL_PROPERTY_SIZE
//
// MessageText:
//
// The size of the buffer for the message identifier or correlation identifier is invalid.
//
#define MQ_ERROR_ILLEGAL_PROPERTY_SIZE   ((HRESULT)0xC00E003BL)

//
// MessageId: MQ_ERROR_ILLEGAL_RESTRICTION_PROPID
//
// MessageText:
//
// A property identifier specified in MQRESTRICTION is invalid.
//
#define MQ_ERROR_ILLEGAL_RESTRICTION_PROPID ((HRESULT)0xC00E003CL)

//
// MessageId: MQ_ERROR_ILLEGAL_MQQUEUEPROPS
//
// MessageText:
//
// Either the pointer to the MQQUEUEPROPS structure has a null value, or no properties are specified in it.
//
#define MQ_ERROR_ILLEGAL_MQQUEUEPROPS    ((HRESULT)0xC00E003DL)

//
// MessageId: MQ_ERROR_PROPERTY_NOTALLOWED
//
// MessageText:
//
// The property identifier specified (for example, PROPID_Q_INSTANCE in MQSetQueueProperties)
// is invalid for the operation requested.
//
#define MQ_ERROR_PROPERTY_NOTALLOWED     ((HRESULT)0xC00E003EL)

//
// MessageId: MQ_ERROR_INSUFFICIENT_PROPERTIES
//
// MessageText:
//
// Not all the properties required for the operation were specified
// for the input parameters.
//
#define MQ_ERROR_INSUFFICIENT_PROPERTIES ((HRESULT)0xC00E003FL)

//
// MessageId: MQ_ERROR_MACHINE_EXISTS
//
// MessageText:
//
// The MSMQ Configuration (msmq) object already exists in Active Directory Domain Services.
//
#define MQ_ERROR_MACHINE_EXISTS          ((HRESULT)0xC00E0040L)

//
// MessageId: MQ_ERROR_ILLEGAL_MQQMPROPS
//
// MessageText:
//
// Either the pointer to the MQQMROPS structure has a null value, or no properties are specified in it.
//
#define MQ_ERROR_ILLEGAL_MQQMPROPS       ((HRESULT)0xC00E0041L)

//
// MessageId: MQ_ERROR_DS_IS_FULL
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_DS_IS_FULL              ((HRESULT)0xC00E0042L)

//
// MessageId: MQ_ERROR_DS_ERROR
//
// MessageText:
//
// There is an internal Active Directory Domain Services error.
//
#define MQ_ERROR_DS_ERROR                ((HRESULT)0xC00E0043L)

//
// MessageId: MQ_ERROR_INVALID_OWNER
//
// MessageText:
//
// The object owner is invalid (for example, MQCreateQueue failed because the QM
// object is invalid).
//
#define MQ_ERROR_INVALID_OWNER           ((HRESULT)0xC00E0044L)

//
// MessageId: MQ_ERROR_UNSUPPORTED_ACCESS_MODE
//
// MessageText:
//
// The access mode specified is unsupported.
//
#define MQ_ERROR_UNSUPPORTED_ACCESS_MODE ((HRESULT)0xC00E0045L)

//
// MessageId: MQ_ERROR_RESULT_BUFFER_TOO_SMALL
//
// MessageText:
//
// The result buffer specified is too small.
//
#define MQ_ERROR_RESULT_BUFFER_TOO_SMALL ((HRESULT)0xC00E0046L)

//
// MessageId: MQ_ERROR_DELETE_CN_IN_USE
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_DELETE_CN_IN_USE        ((HRESULT)0xC00E0048L)

//
// MessageId: MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER
//
// MessageText:
//
// There was no response from the object owner.
//
#define MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER ((HRESULT)0xC00E0049L)

//
// MessageId: MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE
//
// MessageText:
//
// The object owner is not available.
//
#define MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE ((HRESULT)0xC00E004AL)

//
// MessageId: MQ_ERROR_QUEUE_NOT_AVAILABLE
//
// MessageText:
//
// An error occurred while reading from a queue located on a remote computer.
//
#define MQ_ERROR_QUEUE_NOT_AVAILABLE     ((HRESULT)0xC00E004BL)

//
// MessageId: MQ_ERROR_DTC_CONNECT
//
// MessageText:
//
// A connection cannot be established with the Distributed Transaction Coordinator.
//
#define MQ_ERROR_DTC_CONNECT             ((HRESULT)0xC00E004CL)

//
// MessageId: MQ_ERROR_TRANSACTION_IMPORT
//
// MessageText:
//
// The transaction specified cannot be imported.
//
#define MQ_ERROR_TRANSACTION_IMPORT      ((HRESULT)0xC00E004EL)

//
// MessageId: MQ_ERROR_TRANSACTION_USAGE
//
// MessageText:
//
// An attempted action cannot be performed within a transaction.
//
#define MQ_ERROR_TRANSACTION_USAGE       ((HRESULT)0xC00E0050L)

//
// MessageId: MQ_ERROR_TRANSACTION_SEQUENCE
//
// MessageText:
//
// The transaction's operation sequence is incorrect.
//
#define MQ_ERROR_TRANSACTION_SEQUENCE    ((HRESULT)0xC00E0051L)

//
// MessageId: MQ_ERROR_MISSING_CONNECTOR_TYPE
//
// MessageText:
//
// The connector type message property is not specified. This property is required for sending an acknowledgment message or a secure message.
//
#define MQ_ERROR_MISSING_CONNECTOR_TYPE  ((HRESULT)0xC00E0055L)

//
// MessageId: MQ_ERROR_STALE_HANDLE
//
// MessageText:
//
// The Message Queuing service was restarted. Any open queue handles should be closed.
//
#define MQ_ERROR_STALE_HANDLE            ((HRESULT)0xC00E0056L)

//
// MessageId: MQ_ERROR_TRANSACTION_ENLIST
//
// MessageText:
//
// The transaction specified cannot be enlisted.
//
#define MQ_ERROR_TRANSACTION_ENLIST      ((HRESULT)0xC00E0058L)

//
// MessageId: MQ_ERROR_QUEUE_DELETED
//
// MessageText:
//
// The queue was deleted. Messages cannot be received anymore using this
// queue handle. The handle should be closed.
//
#define MQ_ERROR_QUEUE_DELETED           ((HRESULT)0xC00E005AL)

//
// MessageId: MQ_ERROR_ILLEGAL_CONTEXT
//
// MessageText:
//
// The context parameter for MQLocateBegin is invalid.
//
#define MQ_ERROR_ILLEGAL_CONTEXT         ((HRESULT)0xC00E005BL)

//
// MessageId: MQ_ERROR_ILLEGAL_SORT_PROPID
//
// MessageText:
//
// An invalid property identifier is specified in MQSORTSET.
//
#define MQ_ERROR_ILLEGAL_SORT_PROPID     ((HRESULT)0xC00E005CL)

//
// MessageId: MQ_ERROR_LABEL_TOO_LONG
//
// MessageText:
//
// The message label is too long. Its length should be less than or equal to MQ_MAX_MSG_LABEL_LEN.
//
#define MQ_ERROR_LABEL_TOO_LONG          ((HRESULT)0xC00E005DL)

//
// MessageId: MQ_ERROR_LABEL_BUFFER_TOO_SMALL
//
// MessageText:
//
// The label buffer supplied to the API is too small.
//
#define MQ_ERROR_LABEL_BUFFER_TOO_SMALL  ((HRESULT)0xC00E005EL)

//
// MessageId: MQ_ERROR_MQIS_SERVER_EMPTY
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_MQIS_SERVER_EMPTY       ((HRESULT)0xC00E005FL)

//
// MessageId: MQ_ERROR_MQIS_READONLY_MODE
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_MQIS_READONLY_MODE      ((HRESULT)0xC00E0060L)

//
// MessageId: MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL
//
// MessageText:
//
// The buffer passed for the symmetric key is too small.
//
#define MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL ((HRESULT)0xC00E0061L)

//
// MessageId: MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL
//
// MessageText:
//
// The buffer passed for the signature property is too small.
//
#define MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL ((HRESULT)0xC00E0062L)

//
// MessageId: MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL
//
// MessageText:
//
// The buffer passed for the provider name property is too small.
//
#define MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL ((HRESULT)0xC00E0063L)

//
// MessageId: MQ_ERROR_ILLEGAL_OPERATION
//
// MessageText:
//
// The operation is invalid for a foreign message queuing system.
//
#define MQ_ERROR_ILLEGAL_OPERATION       ((HRESULT)0xC00E0064L)

//
// MessageId: MQ_ERROR_WRITE_NOT_ALLOWED
//
// MessageText:
//
// Obsolete; another MQIS server is being installed. Write operations to the database are not allowed at this stage.
//
#define MQ_ERROR_WRITE_NOT_ALLOWED       ((HRESULT)0xC00E0065L)

//
// MessageId: MQ_ERROR_WKS_CANT_SERVE_CLIENT
//
// MessageText:
//
// Independent clients cannot support dependent clients. A Message Queuing server is required.
//
#define MQ_ERROR_WKS_CANT_SERVE_CLIENT   ((HRESULT)0xC00E0066L)

//
// MessageId: MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW
//
// MessageText:
//
// The number of dependent clients served by the Message Queuing server reached its upper limit.
//
#define MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW ((HRESULT)0xC00E0067L)

//
// MessageId: MQ_CORRUPTED_QUEUE_WAS_DELETED
//
// MessageText:
//
// The file %1 for the queue %2 in the Lqs folder was deleted because it was corrupted.
//
#define MQ_CORRUPTED_QUEUE_WAS_DELETED   ((HRESULT)0xC00E0068L)

//
// MessageId: MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE
//
// MessageText:
//
// The remote computer is not available.
//
#define MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE ((HRESULT)0xC00E0069L)

//
// MessageId: MQ_ERROR_UNSUPPORTED_OPERATION
//
// MessageText:
//
// This operation is not supported for Message Queuing installed in workgroup mode.
//
#define MQ_ERROR_UNSUPPORTED_OPERATION   ((HRESULT)0xC00E006AL)

//
// MessageId: MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED
//
// MessageText:
//
// The cryptographic service provider %1 is not supported by Message Queuing.
//
#define MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED ((HRESULT)0xC00E006BL)

//
// MessageId: MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR
//
// MessageText:
//
// The security descriptors for the cryptographic keys cannot be set.
//
#define MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR ((HRESULT)0xC00E006CL)

//
// MessageId: MQ_ERROR_CERTIFICATE_NOT_PROVIDED
//
// MessageText:
//
// A user attempted to send an authenticated message without a certificate.
//
#define MQ_ERROR_CERTIFICATE_NOT_PROVIDED ((HRESULT)0xC00E006DL)

//
// MessageId: MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED
//
// MessageText:
//
// The column PROPID_Q_PATHNAME_DNS is not supported for the MQLocateBegin API.
//
#define MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED ((HRESULT)0xC00E006EL)

//
// MessageId: MQ_ERROR_CANNOT_CREATE_CERT_STORE
//
// MessageText:
//
// A certificate store cannot be created for the internal certificate.
//
#define MQ_ERROR_CANNOT_CREATE_CERT_STORE ((HRESULT)0xC00E006FL)

//
// MessageId: MQ_ERROR_CANNOT_OPEN_CERT_STORE
//
// MessageText:
//
// The certificate store for the internal certificate cannot be opened.
//
#define MQ_ERROR_CANNOT_OPEN_CERT_STORE  ((HRESULT)0xC00E0070L)

//
// MessageId: MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION
//
// MessageText:
//
// This operation is invalid for an MsmqServices object.
//
#define MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION ((HRESULT)0xC00E0071L)

//
// MessageId: MQ_ERROR_CANNOT_GRANT_ADD_GUID
//
// MessageText:
//
// The Add GUID permission cannot be granted to the current user.
//
#define MQ_ERROR_CANNOT_GRANT_ADD_GUID   ((HRESULT)0xC00E0072L)

//
// MessageId: MQ_ERROR_CANNOT_LOAD_MSMQOCM
//
// MessageText:
//
// Obsolete: The dynamic-link library Msmqocm.dll cannot be loaded.
//
#define MQ_ERROR_CANNOT_LOAD_MSMQOCM     ((HRESULT)0xC00E0073L)

//
// MessageId: MQ_ERROR_NO_ENTRY_POINT_MSMQOCM
//
// MessageText:
//
// An entry point cannot be located in Msmqocm.dll.
//
#define MQ_ERROR_NO_ENTRY_POINT_MSMQOCM  ((HRESULT)0xC00E0074L)

//
// MessageId: MQ_ERROR_NO_MSMQ_SERVERS_ON_DC
//
// MessageText:
//
// Message Queuing servers cannot be found on domain controllers.
//
#define MQ_ERROR_NO_MSMQ_SERVERS_ON_DC   ((HRESULT)0xC00E0075L)

//
// MessageId: MQ_ERROR_CANNOT_JOIN_DOMAIN
//
// MessageText:
//
// The computer joined the domain, but Message Queuing will continue to run in workgroup mode because it failed to register itself in Active Directory Domain Services.
//
#define MQ_ERROR_CANNOT_JOIN_DOMAIN      ((HRESULT)0xC00E0076L)

//
// MessageId: MQ_ERROR_CANNOT_CREATE_ON_GC
//
// MessageText:
//
// The object was not created on the Global Catalog server specified.
//
#define MQ_ERROR_CANNOT_CREATE_ON_GC     ((HRESULT)0xC00E0077L)

//
// MessageId: MQ_ERROR_GUID_NOT_MATCHING
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_GUID_NOT_MATCHING       ((HRESULT)0xC00E0078L)

//
// MessageId: MQ_ERROR_PUBLIC_KEY_NOT_FOUND
//
// MessageText:
//
// The public key for the computer %1 cannot be found.
//
#define MQ_ERROR_PUBLIC_KEY_NOT_FOUND    ((HRESULT)0xC00E0079L)

//
// MessageId: MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST
//
// MessageText:
//
// The public key for the computer %1 does not exist.
//
#define MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST ((HRESULT)0xC00E007AL)

//
// MessageId: MQ_ERROR_ILLEGAL_MQPRIVATEPROPS
//
// MessageText:
//
// The parameters in MQPRIVATEPROPS are invalid. Either the pointer to the MQPRIVATEPROPS structure has a null value, or no properties are specified in it.
//
#define MQ_ERROR_ILLEGAL_MQPRIVATEPROPS  ((HRESULT)0xC00E007BL)

//
// MessageId: MQ_ERROR_NO_GC_IN_DOMAIN
//
// MessageText:
//
// Global Catalog servers cannot be found in the domain specified.
//
#define MQ_ERROR_NO_GC_IN_DOMAIN         ((HRESULT)0xC00E007CL)

//
// MessageId: MQ_ERROR_NO_MSMQ_SERVERS_ON_GC
//
// MessageText:
//
// No Message Queuing servers were found on Global Catalog servers.
//
#define MQ_ERROR_NO_MSMQ_SERVERS_ON_GC   ((HRESULT)0xC00E007DL)

//
// MessageId: MQ_ERROR_CANNOT_GET_DN
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_CANNOT_GET_DN           ((HRESULT)0xC00E007EL)

//
// MessageId: MQ_ERROR_CANNOT_HASH_DATA_EX
//
// MessageText:
//
// Data for an authenticated message cannot be hashed.
//
#define MQ_ERROR_CANNOT_HASH_DATA_EX     ((HRESULT)0xC00E007FL)

//
// MessageId: MQ_ERROR_CANNOT_SIGN_DATA_EX
//
// MessageText:
//
// Data cannot be signed before sending an authenticated message.
//
#define MQ_ERROR_CANNOT_SIGN_DATA_EX     ((HRESULT)0xC00E0080L)

//
// MessageId: MQ_ERROR_CANNOT_CREATE_HASH_EX
//
// MessageText:
//
// A hash object cannot be created for an authenticated message.
//
#define MQ_ERROR_CANNOT_CREATE_HASH_EX   ((HRESULT)0xC00E0081L)

//
// MessageId: MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX
//
// MessageText:
//
// The signature of the message received is not valid.
//
#define MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX ((HRESULT)0xC00E0082L)

//
// MessageId: MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS
//
// MessageText:
//
// The object that will be deleted is owned by a primary site controller. The operation cannot be performed.
//
#define MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS ((HRESULT)0xC00E0083L)

//
// MessageId: MQ_ERROR_NO_MQUSER_OU
//
// MessageText:
//
// There is no MSMQ Users organizational unit object in Active Directory Domain Services for the domain. Please create one manually.
//
#define MQ_ERROR_NO_MQUSER_OU            ((HRESULT)0xC00E0084L)

//
// MessageId: MQ_ERROR_CANNOT_LOAD_MQAD
//
// MessageText:
//
// The dynamic-link library Mqad.dll cannot be loaded.
//
#define MQ_ERROR_CANNOT_LOAD_MQAD        ((HRESULT)0xC00E0085L)

//
// MessageId: MQ_ERROR_CANNOT_LOAD_MQDSSRV
//
// MessageText:
//
// Obsolete, kept for backward compatibility
//
#define MQ_ERROR_CANNOT_LOAD_MQDSSRV     ((HRESULT)0xC00E0086L)

//
// MessageId: MQ_ERROR_PROPERTIES_CONFLICT
//
// MessageText:
//
// Two or more of the properties passed cannot co-exist.
// For example, you cannot set both PROPID_M_RESP_QUEUE and PROPID_M_RESP_FORMAT_NAME when sending a message.
//
#define MQ_ERROR_PROPERTIES_CONFLICT     ((HRESULT)0xC00E0087L)

//
// MessageId: MQ_ERROR_MESSAGE_NOT_FOUND
//
// MessageText:
//
// The message does not exist or was removed from the queue.
//
#define MQ_ERROR_MESSAGE_NOT_FOUND       ((HRESULT)0xC00E0088L)

//
// MessageId: MQ_ERROR_CANT_RESOLVE_SITES
//
// MessageText:
//
// The sites where the computer resides cannot be resolved. Check that the subnets in your network are configured correctly in Active Directory Domain Services and that each site is configured with the appropriate subnet.
//
#define MQ_ERROR_CANT_RESOLVE_SITES      ((HRESULT)0xC00E0089L)

//
// MessageId: MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS
//
// MessageText:
//
// This operation is not supported by dependent clients.
//
#define MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS ((HRESULT)0xC00E008AL)

//
// MessageId: MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER
//
// MessageText:
//
// This operation is not supported by the remote Message Queuing service. For example, MQReceiveMessageByLookupId is not supported by MSMQ 1.0/2.0.
//
#define MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER ((HRESULT)0xC00E008BL)

//
// MessageId: MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS
//
// MessageText:
//
// The object whose properties are being retrieved from Active Directory Domain Services does not belong to the class requested.
//
#define MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS ((HRESULT)0xC00E008CL)

//
// MessageId: MQ_ERROR_MULTI_SORT_KEYS
//
// MessageText:
//
// The value of cCol in MQSORTSET cannot be greater than 1. Active Directory Domain Services supports only a single sort key.
//
#define MQ_ERROR_MULTI_SORT_KEYS         ((HRESULT)0xC00E008DL)

//
// MessageId: MQ_ERROR_GC_NEEDED
//
// MessageText:
//
// An MSMQ Configuration (msmq) object with the GUID supplied cannot be created. By default, an Active Directory Domain Services forest does not support adding an object with a supplied GUID.
//
#define MQ_ERROR_GC_NEEDED               ((HRESULT)0xC00E008EL)

//
// MessageId: MQ_ERROR_DS_BIND_ROOT_FOREST
//
// MessageText:
//
// Binding to the forest root failed. This error usually indicates a problem in the DNS configuration.
//
#define MQ_ERROR_DS_BIND_ROOT_FOREST     ((HRESULT)0xC00E008FL)

//
// MessageId: MQ_ERROR_DS_LOCAL_USER
//
// MessageText:
//
// A local user is authenticated as an anonymous user and cannot access Active Directory Domain Services. You need to log on as a domain user to access Active Directory Domain Services.
//
#define MQ_ERROR_DS_LOCAL_USER           ((HRESULT)0xC00E0090L)

//
// MessageId: MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED
//
// MessageText:
//
// The column PROPID_Q_ADS_PATH is not supported for the MQLocateBegin API.
//
#define MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED ((HRESULT)0xC00E0091L)

//
// MessageId: MQ_ERROR_BAD_XML_FORMAT
//
// MessageText:
//
// The given property is not a valid XML document.
//
#define MQ_ERROR_BAD_XML_FORMAT          ((HRESULT)0xC00E0092L)

//
// MessageId: MQ_ERROR_UNSUPPORTED_CLASS
//
// MessageText:
//
// The Active Directory Domain Services object specified is not an instance of a supported class.
//
#define MQ_ERROR_UNSUPPORTED_CLASS       ((HRESULT)0xC00E0093L)

//
// MessageId: MQ_ERROR_UNINITIALIZED_OBJECT
//
// MessageText:
//
// The MSMQManagement object must be initialized before it is used.
//
#define MQ_ERROR_UNINITIALIZED_OBJECT    ((HRESULT)0xC00E0094L)

//
// MessageId: MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS
//
// MessageText:
//
// The object that will be created should be owned by a primary site controller. The operation cannot be performed.
//
#define MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS ((HRESULT)0xC00E0095L)

//
// MessageId: MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS
//
// MessageText:
//
// The object that will be updated is owned by a primary site controller. The operation cannot be performed.
//
#define MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS ((HRESULT)0xC00E0096L)

//
// MessageId: MQ_ERROR_RESOLVE_ADDRESS
//
// MessageText:
//
// Message Queuing is not able to resolve the address specified by the user. The address may be wrong or DNS look-up for address failed.
//
#define MQ_ERROR_RESOLVE_ADDRESS         ((HRESULT)0xC00E0099L)

//
// MessageId: MQ_ERROR_TOO_MANY_PROPERTIES
//
// MessageText:
//
// Too many properties passed to the function. Message Queuing can process up to 128 properties in one call.
//
#define MQ_ERROR_TOO_MANY_PROPERTIES     ((HRESULT)0xC00E009AL)

//
// MessageId: MQ_ERROR_MESSAGE_NOT_AUTHENTICATED
//
// MessageText:
//
// The queue only accepts authenticated messages.
//
#define MQ_ERROR_MESSAGE_NOT_AUTHENTICATED ((HRESULT)0xC00E009BL)

//
// MessageId: MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION
//
// MessageText:
//
// The message is currently being processed under a transaction. Till the transaction outcome is determined, the message cannot be processed in any other transaction.
//
#define MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION ((HRESULT)0xC00E009CL)


#ifdef __cplusplus
extern "C"
{
#endif

//********************************************************************
//  RECEIVE CALLBACK
//********************************************************************

typedef
VOID
(APIENTRY *PMQRECEIVECALLBACK)(
    HRESULT hrStatus,
    QUEUEHANDLE hSource,
    DWORD dwTimeout,
    DWORD dwAction,
    MQMSGPROPS* pMessageProps,
    LPOVERLAPPED lpOverlapped,
    HANDLE hCursor
    );


//********************************************************************
// MSMQ API
//********************************************************************

HRESULT
APIENTRY
MQCreateQueue(
    _In_opt_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Inout_ MQQUEUEPROPS* pQueueProps,
    _Out_writes_opt_(*lpdwFormatNameLength)LPWSTR lpwcsFormatName,
    _Inout_ LPDWORD lpdwFormatNameLength
    );

HRESULT
APIENTRY
MQDeleteQueue(
    _In_ LPCWSTR lpwcsFormatName
    );

HRESULT
APIENTRY
MQLocateBegin(
    _In_opt_ LPCWSTR lpwcsContext,
    _In_opt_ MQRESTRICTION* pRestriction,
    _In_ MQCOLUMNSET* pColumns,
    _In_ MQSORTSET* pSort,
    _Out_ PHANDLE phEnum
    );

HRESULT
APIENTRY
MQLocateNext(
    _In_ HANDLE hEnum,
    _Inout_ DWORD* pcProps,
    _Out_ MQPROPVARIANT aPropVar[]
    );

HRESULT
APIENTRY
MQLocateEnd(
    _In_ HANDLE hEnum
    );

HRESULT
APIENTRY
MQOpenQueue(
    _In_ LPCWSTR lpwcsFormatName,
    _In_ DWORD dwAccess,
    _In_ DWORD dwShareMode,
    _Out_ QUEUEHANDLE* phQueue
    );

HRESULT
APIENTRY
MQSendMessage(
    _In_ QUEUEHANDLE hDestinationQueue,
    _In_ MQMSGPROPS* pMessageProps,
    _In_opt_ ITransaction *pTransaction
    );

HRESULT
APIENTRY
MQReceiveMessage(
    _In_ QUEUEHANDLE hSource,
    _In_ DWORD dwTimeout,
    _In_ DWORD dwAction,
    _Inout_opt_ MQMSGPROPS* pMessageProps,
    _Inout_opt_ LPOVERLAPPED lpOverlapped,
    _In_opt_ PMQRECEIVECALLBACK fnReceiveCallback,
    _In_opt_ HANDLE hCursor,
    _In_opt_ ITransaction* pTransaction
    );

#if(_WIN32_WINNT >= 0x0501)

HRESULT
APIENTRY
MQReceiveMessageByLookupId(
    _In_ QUEUEHANDLE hSource,
    _In_ ULONGLONG ullLookupId,
    _In_ DWORD dwLookupAction,
    _Inout_opt_ MQMSGPROPS* pMessageProps,
    _Inout_opt_ LPOVERLAPPED lpOverlapped,
    _In_opt_ PMQRECEIVECALLBACK fnReceiveCallback,
    _In_opt_ ITransaction *pTransaction
    );

#endif

HRESULT
APIENTRY
MQCreateCursor(
    _In_ QUEUEHANDLE hQueue,
    _Out_ PHANDLE phCursor
    );

HRESULT
APIENTRY
MQCloseCursor(
    _In_ HANDLE hCursor
    );

HRESULT
APIENTRY
MQCloseQueue(
    _In_ QUEUEHANDLE hQueue
    );

HRESULT
APIENTRY
MQSetQueueProperties(
    _In_ LPCWSTR lpwcsFormatName,
    _Inout_ MQQUEUEPROPS* pQueueProps
    );

HRESULT
APIENTRY
MQGetQueueProperties(
    _In_ LPCWSTR lpwcsFormatName,
    _Inout_ MQQUEUEPROPS* pQueueProps
    );

HRESULT
APIENTRY
MQGetQueueSecurity(
    _In_ LPCWSTR lpwcsFormatName,
    _In_ SECURITY_INFORMATION RequestedInformation,
    _Out_writes_bytes_(nLength)  PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpnLengthNeeded
    );

HRESULT
APIENTRY
MQSetQueueSecurity(
    _In_ LPCWSTR lpwcsFormatName,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_opt_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

HRESULT
APIENTRY
MQPathNameToFormatName(
    _In_ LPCWSTR lpwcsPathName,
    _Out_writes_(*lpdwFormatNameLength)  LPWSTR lpwcsFormatName,
    _Inout_ LPDWORD lpdwFormatNameLength
    );

HRESULT
APIENTRY
MQHandleToFormatName(
    _In_ QUEUEHANDLE hQueue,
    _Out_writes_(*lpdwFormatNameLength) LPWSTR lpwcsFormatName,
    _Inout_ LPDWORD lpdwFormatNameLength
    );

HRESULT
APIENTRY
MQInstanceToFormatName(
    _In_ GUID* pGuid,
    _Out_writes_(*lpdwFormatNameLength)  LPWSTR lpwcsFormatName,
    _Inout_ LPDWORD lpdwFormatNameLength
    );

#if(_WIN32_WINNT >= 0x0501)

HRESULT
APIENTRY
MQADsPathToFormatName(
    _In_ LPCWSTR lpwcsADsPath,
    _Out_writes_(*lpdwFormatNameLength) LPWSTR lpwcsFormatName,
    _Inout_ LPDWORD lpdwFormatNameLength
    );

#endif

VOID
APIENTRY
MQFreeMemory(
    _In_ PVOID pvMemory
    );

HRESULT
APIENTRY
MQGetMachineProperties(
    _In_opt_ LPCWSTR lpwcsMachineName,
    _In_opt_ const GUID* pguidMachineId,
    _Inout_ MQQMPROPS* pQMProps
    );

HRESULT
APIENTRY
MQGetSecurityContext(
    _In_reads_bytes_opt_(dwCertBufferLength) PVOID lpCertBuffer,
    _In_ DWORD dwCertBufferLength,
    _Out_ HANDLE* phSecurityContext
    );

HRESULT
APIENTRY
MQGetSecurityContextEx(
    _In_reads_bytes_opt_(dwCertBufferLength) PVOID lpCertBuffer,
    _In_ DWORD dwCertBufferLength,
    _Out_ HANDLE* phSecurityContext
    );

VOID
APIENTRY
MQFreeSecurityContext(
    _In_ HANDLE hSecurityContext
    );

HRESULT
APIENTRY
MQRegisterCertificate(
    _In_ DWORD dwFlags,
    _In_ PVOID lpCertBuffer,
    _In_ DWORD dwCertBufferLength
    );

HRESULT
APIENTRY
MQBeginTransaction(
    _Out_ ITransaction **ppTransaction
    );

HRESULT
APIENTRY
MQGetOverlappedResult(
    _In_ LPOVERLAPPED lpOverlapped
    );

HRESULT
APIENTRY
MQGetPrivateComputerInformation(
    _In_opt_ LPCWSTR lpwcsComputerName,
    _Inout_ MQPRIVATEPROPS* pPrivateProps
    );

HRESULT
APIENTRY
MQPurgeQueue(
    _In_ QUEUEHANDLE hQueue
    );

HRESULT
APIENTRY
MQMgmtGetInfo(
    _In_opt_ LPCWSTR pComputerName,
    _In_ LPCWSTR pObjectName,
    _Inout_ MQMGMTPROPS* pMgmtProps
    );

HRESULT
APIENTRY
MQMgmtAction(
    _In_opt_ LPCWSTR pComputerName,
    _In_ LPCWSTR pObjectName,
    _In_ LPCWSTR pAction
    );

HRESULT
APIENTRY
MQMarkMessageRejected(
    _In_ HANDLE hQueue,
    _In_ ULONGLONG ullLookupId
    );

HRESULT
APIENTRY
MQMoveMessage(
    _In_ QUEUEHANDLE hSourceQueue,
    _In_ QUEUEHANDLE hDestinationQueue,
    _In_ ULONGLONG ullLookupId,
    _In_opt_ ITransaction *pTransaction
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __MQ_H__

