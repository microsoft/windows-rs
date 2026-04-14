/*
        @doc MAILMSG PROPERTIES
        @module mailmsgprops.h | MailMsg Properties for SMTP and NNTP
*/

/*++

Copyright (c) 1999  Microsoft Corporation

Module Name:

    mailmsgprops.h

Abstract:

    This module contains the definitions for the MailMsg 
    Object property ID's.


--*/

#ifndef __MAILMSGPROPS_H__
#define __MAILMSGPROPS_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/*=======================================================================*/
// These macros are used to define the tables of property ID's.


#define IMMPID_START_LIST(name,start,guid)    struct __declspec(uuid(guid)) tagIMMPID_##name##_STRUCT;\
                                            typedef enum tagIMMPID_##name##_ENUM {\
                                                IMMPID_##name##_BEFORE__ = (start)-1,
#define IMMPID_END_LIST(name)                    IMMPID_##name##_AFTER__\
                                            } IMMPID_##name##_ENUM;


/*=======================================================================*/


// These are the per-message properties.

IMMPID_START_LIST(MP,0x1000,"13384CF0-B3C4-11d1-AA92-00AA006BC80B")

    // @const IMMPID | IMMPID_MP_RECIPIENT_LIST | 
    //   *** OBSOLETE ***
    IMMPID_MP_RECIPIENT_LIST,

    // @const IMMPID | IMMPID_MP_CONTENT_FILE_NAME | 
    //   *** OBSOLETE ***
    IMMPID_MP_CONTENT_FILE_NAME,

    // @const IMMPID | IMMPID_MP_SENDER_ADDRESS_SMTP | 
    //  ANSI String - SMTP Address of sender
    IMMPID_MP_SENDER_ADDRESS_SMTP,

    // @const IMMPID | IMMPID_MP_SENDER_ADDRESS_X500 | 
    //  ANSI String - X500 Address of sender
    IMMPID_MP_SENDER_ADDRESS_X500,

    // @const IMMPID | IMMPID_MP_SENDER_ADDRESS_X400 | 
    //  String - X400 Address of sender
    IMMPID_MP_SENDER_ADDRESS_X400,

    // @const IMMPID | IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN | 
    //  String - Legacy DN Address of sender
    IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN,

    // @const IMMPID | IMMPID_MP_DOMAIN_LIST | 
    //   *** OBSOLETE ***
    IMMPID_MP_DOMAIN_LIST,

    // @const IMMPID | IMMPID_MP_PICKUP_FILE_NAME | 
    //  ANSI String - Filename of msg file in pickup directory
    IMMPID_MP_PICKUP_FILE_NAME,

    // @const IMMPID | IMMPID_MP_AUTHENTICATED_USER_NAME | 
    //   *** OBSOLETE ***
    IMMPID_MP_AUTHENTICATED_USER_NAME,

    // @const IMMPID | IMMPID_MP_CONNECTION_IP_ADDRESS | 
    //  ANSI String - IP address of MTA or client that submitted this message
    IMMPID_MP_CONNECTION_IP_ADDRESS,

    // @const IMMPID | IMMPID_MP_HELO_DOMAIN | 
    //  ANSI String - Domain name used in HELO/EHLO when message was submitted
    IMMPID_MP_HELO_DOMAIN,

    // @const IMMPID | IMMPID_MP_EIGHTBIT_MIME_OPTION | 
    //  BOOL - TRUE if message body is 8-bit MIME
    IMMPID_MP_EIGHTBIT_MIME_OPTION,

    // @const IMMPID | IMMPID_MP_CHUNKING_OPTION | 
    //   *** OBSOLETE ***
    IMMPID_MP_CHUNKING_OPTION,

    // @const IMMPID | IMMPID_MP_BINARYMIME_OPTION | 
    //  BOOL - TRUE if message body is binary MIME
    IMMPID_MP_BINARYMIME_OPTION,

    // @const IMMPID | IMMPID_MP_REMOTE_AUTHENTICATION_TYPE | 
    //   *** OBSOLETE ***
    IMMPID_MP_REMOTE_AUTHENTICATION_TYPE,

    // @const IMMPID | IMMPID_MP_ERROR_CODE | 
    //   *** OBSOLETE ***
    IMMPID_MP_ERROR_CODE,

    // @const IMMPID | IMMPID_MP_DSN_ENVID_VALUE | 
    //  Value of RFC1891 ENVID of submitted message
    IMMPID_MP_DSN_ENVID_VALUE,

    // @const IMMPID | IMMPID_MP_DSN_RET_VALUE | 
    //  ANSI String of what follows ESMTP RET
    IMMPID_MP_DSN_RET_VALUE,                

    // @const IMMPID | IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE | 
    //   *** OBSOLETE ***
    IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE,    

    // @const IMMPID | IMMPID_MP_ARRIVAL_TIME | 
    //  FILETIME that message arrived in system
    IMMPID_MP_ARRIVAL_TIME,

    // @const IMMPID | IMMPID_MP_MESSAGE_STATUS | 
    // MP_STATUS_* value describing status of message system 
    IMMPID_MP_MESSAGE_STATUS,

    // @const IMMPID | IMMPID_MP_EXPIRE_DELAY | 
    //  FILETIME of Delay DSN Expiration
    IMMPID_MP_EXPIRE_DELAY,         

    // @const IMMPID | IMMPID_MP_EXPIRE_NDR | 
    //  FILETIME of NDR DSN Expiration
    IMMPID_MP_EXPIRE_NDR,           

    // @const IMMPID | IMMPID_MP_LOCAL_EXPIRE_DELAY | 
    //  FILETIME of Delay DSN Expiration for local recips
    IMMPID_MP_LOCAL_EXPIRE_DELAY,   

    // @const IMMPID | IMMPID_MP_LOCAL_EXPIRE_NDR | 
    //  FILETIME of NDR DSN Expiration for local recips
    IMMPID_MP_LOCAL_EXPIRE_NDR,     

    // @const IMMPID | IMMPID_MP_ARRIVAL_FILETIME | 
    //  FILETIME when message was submitted to queue
    IMMPID_MP_ARRIVAL_FILETIME,     

    // @const IMMPID | IMMPID_MP_HR_CAT_STATUS | 
    //  HRESULT MsgCat status (ie CAT_W_SOME_UNDELIVERABLE_MSGS)
    IMMPID_MP_HR_CAT_STATUS,        

    // @const IMMPID | IMMPID_MP_MSG_GUID | 
    //  String GUID ID which is only used if you want need to be able to replace 
    //  this message with another newer version.  This property is only 
    //  effective when used for a small amount of mail on the server, and is
    //  best suited for versioned mail (like DS replication).
    IMMPID_MP_MSG_GUID,        

    // @const IMMPID | IMMPID_MP_SUPERSEDES_MSG_GUID | 
    //  String GUID ID which this message superscedes.  If a message with with a 
    //  IMMPID_MP_MSG_GUID equal to this property is still on the server, then
    //  that message will not be sent out.

    IMMPID_MP_SUPERSEDES_MSG_GUID,

    // @const IMMPID | IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF | 
    //  Boolean which tells if the input was scanned for <CRLF>.<CRLF> in the msg
    // body
    IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF,

    // @const IMMPID | IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF | 
    //  Boolean which tells if the input had any <CRLF>.<CRLF>
    //
    IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF, 

    // @const IMMPID | IMMPID_MP_MSG_SIZE_HINT | 
    //  Provides an approximate size of the message content.  Does not convert
    //  message, or guarantee accuracy. For an accurate message size (but at 
    //  a higher performance cost) use IMailMsgProperties::GetContentSize.
    //
    IMMPID_MP_MSG_SIZE_HINT, 

    // @const IMMPID | IMMPID_MP_RFC822_MSG_ID | 
    //  String -- RFC 822 Message Id header.
    IMMPID_MP_RFC822_MSG_ID,

    // @const IMMPID | IMMPID_MP_RFC822_MSG_SUBJECT | 
    //  String -- RFC822 subject header
    IMMPID_MP_RFC822_MSG_SUBJECT,

    // @const IMMPID | IMMPID_MP_RFC822_FROM_ADDRESS | 
    //  String -- RFC822 from address
    IMMPID_MP_RFC822_FROM_ADDRESS,

    // @const IMMPID | IMMPID_MP_RFC822_TO_ADDRESS | 
    //  String -- RFC822 To address
    IMMPID_MP_RFC822_TO_ADDRESS,

    // @const IMMPID | IMMPID_MP_RFC822_CC_ADDRESS | 
    //  String -- RFC822 Cc address
    IMMPID_MP_RFC822_CC_ADDRESS,

    // @const IMMPID | IMMPID_MP_RFC822_BCC_ADDRESS | 
    //  String -- RFC822 Bcc address
    IMMPID_MP_RFC822_BCC_ADDRESS,

    // @const IMMPID | IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS | 
    //  String -- server IP Address
    IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS,

    // @const IMMPID | IMMPID_MP_SERVER_NAME | 
    //  String -- server name
    IMMPID_MP_SERVER_NAME,

    // @const IMMPID | IMMPID_MP_SERVER_VERSION | 
    //  String -- server version
    IMMPID_MP_SERVER_VERSION,

    // @const IMMPID | IMMPID_MP_NUM_RECIPIENTS | 
    //  DWORD -- number of recipients
    IMMPID_MP_NUM_RECIPIENTS,


    // @const IMMPID | IMMPID_MP_X_PRIORITY | 
    //  DWORD -- the priority of message
    IMMPID_MP_X_PRIORITY,

    // @const IMMPID | IMMPID_MP_FROM_ADDRESS | 
    //  String -- From: address, in form format:address 
    //  (ie, smtp:foo@microsoft.com).  RFC822 address comments are stripped
    IMMPID_MP_FROM_ADDRESS,
    
    // @const IMMPID | IMMPID_MP_SENDER_ADDRESS | 
    //  String -- Sender: address, in form format:address 
    //  (ie, smtp:foo@microsoft.com).  RFC822 address comments are stripped
    IMMPID_MP_SENDER_ADDRESS,

    // @const IMMPID | IMMPID_MP_DEFERRED_DELIVERY_FILETIME | 
    //  FILETIME -- Universal time to wait until before allowing
    //  message to be delivered.
    IMMPID_MP_DEFERRED_DELIVERY_FILETIME,

    // @const IMMPID | IMMPID_MP_SENDER_ADDRESS_OTHER | 
    //  ANSI String - with the format "type:address"
    IMMPID_MP_SENDER_ADDRESS_OTHER,

    // @const IMMPID | IMMPID_MP_ORIGINAL_ARRIVAL_TIME | 
    //  FILETIME that message arrived in first exchange system
    IMMPID_MP_ORIGINAL_ARRIVAL_TIME,

    // @const IMMPID | IMMPID_MP_MSG_CLASS | 
    //  The Message class of the mail, e.g. syste, replication, etc...
    IMMPID_MP_MSGCLASS,

    // @const IMMPID | IMMPID_MP_CONTENT_TYPE | 
    //  The content type of message.
    //  ANSI STRING.
    IMMPID_MP_CONTENT_TYPE,

    //@const IMMPID | IMMPID_MP_ENCRYPTION_TYPE |
    // DWORD - 0, no encryption
    //         1, signed only
    //         2, encrypted
    IMMPID_MP_ENCRYPTION_TYPE,

    //@const IMMPID | IMMPID_MP_CONNECTION_SERVER_PORT | 
    // DWORD - Port (if any) that this message was submitted on 
    IMMPID_MP_CONNECTION_SERVER_PORT,

    //@const IMMPID | IMMPID_MP_CLIENT_AUTH_USER | 
    // ASCII STRING - Name of authenticated user
    IMMPID_MP_CLIENT_AUTH_USER,

    //@const IMMPID | IMMPID_MP_CLIENT_AUTH_TYPE | 
    // ASCII STRING - Type of authentication used (AUTH keyword)
    IMMPID_MP_CLIENT_AUTH_TYPE,

    //@const IMMPID | IMMPID_MP_CRC_GLOBAL |
    // DWORD - Checksum for global properties (excluding the checksum properties!)
    IMMPID_MP_CRC_GLOBAL,

    //@const IMMPID | IMMPID_MP_CRC_RECIPS |
    // DWORD - Checksum for recipient properties
    IMMPID_MP_CRC_RECIPS,

    // @const IMMPID | IMMPID_MP_INBOUND_MAIL_FROM_AUTH | 
    //  ANSI String - Contents of the RFC2554 AUTH= string on a MAIL FROM on inbound messages
    IMMPID_MP_INBOUND_MAIL_FROM_AUTH,


    // Add new per-message properties above this line.
IMMPID_END_LIST(MP)


// Message classes
// @const DWORD | MP_MSGCLASS_SYSTEM
// for msgs of type -- system
#define MP_MSGCLASS_SYSTEM       1

// @const DWORD | MP_MSGCLASS_REPLICATION
// for msgs of type -- replication
#define MP_MSGCLASS_REPLICATION  2

// @const DWORD | MP_MSGCLASS_DELIVERY_REPORT
// for msgs of type -- delivery report
#define MP_MSGCLASS_DELIVERY_REPORT     3

// @const DWORD | MP_MSGCLASS_DELIVERY_REPORT
// for msgs of type -- non-delivery report
#define MP_MSGCLASS_NONDELIVERY_REPORT  4


// Message status property values defined:

// @const DWORD | MP_STATUS_SUCCESS | 
//  Initial status of message
#define MP_STATUS_SUCCESS                    0

// @const DWORD | MP_STATUS_RETRY | 
//  Status indicating retry.
#define MP_STATUS_RETRY                      1

// @const DWORD | MP_STATUS_ABORT_DELIVERY | 
//  Delivery of this message should be aborted and the message deleted.
#define MP_STATUS_ABORT_DELIVERY             2

// @const DWORD | MP_STATUS_BAD_MAIL | 
//  This message should be moved to badmail.
#define MP_STATUS_BAD_MAIL                   3

// @const DWORD | MP_STATUS_SUBMITTED | 
//  Message has been submitted for delivery.
#define MP_STATUS_SUBMITTED                  4

// @const DWORD | MP_STATUS_CATEGORIZED | 
//  Message has been categorized.
#define MP_STATUS_CATEGORIZED                5

// @const DWORD | MP_STATUS_ABANDON_DELIVERY | 
//  Delivery of this message should be abandoned until the service restarts
#define MP_STATUS_ABANDON_DELIVERY           6

//Per recipient flags for IMMPID_RP_RECIPIENT_FLAGS

// @const DWORD | RP_RECIP_FLAGS_RESERVED | 
//  You should not modify / use these bits
#define RP_RECIP_FLAGS_RESERVED     0x0000000F 

// @const DWORD | RP_DSN_NOTIFY_SUCCESS | 
//  Notify on success - set if RFC1891 NOTIFY=SUCCESS is used
#define RP_DSN_NOTIFY_SUCCESS       0x01000000 

// @const DWORD | RP_DSN_NOTIFY_FAILURE | 
//  Notify on failure - set if RFC1891 NOTIFY=FAILURE is used
#define RP_DSN_NOTIFY_FAILURE       0x02000000 

// @const DWORD | RP_DSN_NOTIFY_DELAY | 
//  Notify on delay - set if RFC1891 NOTIFY=DELAY is used
#define RP_DSN_NOTIFY_DELAY         0x04000000 

// @const DWORD | RP_DSN_NOTIFY_NEVER | 
//  Never notify - set if RFC1891 NOTIFY=NEVER is used
#define RP_DSN_NOTIFY_NEVER         0x08000000 

// @const DWORD | RP_DSN_NOTIFY_MASK | 
//  Mask of all notify parameters
#define RP_DSN_NOTIFY_MASK          0x0F000000

//The following flags can be used in searches, but should not be set directly

// @const DWORD | RP_HANDLED | 
//  Recipient has either been delivered or should not be delivered
//  (this flag is provided to check status of recipient... it should never be used
//  directly)
#define RP_HANDLED                  0x00000010 

// @const DWORD | RP_GENERAL_FAILURE | 
//  some form of hard failure happend
//  (this flag is provided to check status of recipient... it should never be used
//  directly)
#define RP_GENERAL_FAILURE          0x00000020 

// @const DWORD | RP_DSN_HANDLED | 
//  Final DSN has been sent (or no DSN needs to be sent)
//  (this flag is provided to check status of recipient... it should never be used
//  directly)
#define RP_DSN_HANDLED              0x00000040 

//The following constants define how a message can be RP_HANDLED

// @const DWORD | RP_DELIVERED | 
//  The recipient has been delivered successfully
#define RP_DELIVERED                0x00000110 

// @const DWORD | RP_DSN_SENT_NDR | 
//  NDR (FAILED DSN) for this recipient has been sent
#define RP_DSN_SENT_NDR             0x00000450 

// @const DWORD | RP_FAILED | 
//  Recipient has a hard failure
#define RP_FAILED                   0x00000830 

// @const DWORD | RP_UNRESOLVED | 
//  This recipient was not resolved by categorization
#define RP_UNRESOLVED               0x00001030 

// @const DWORD | RP_ENPANDED | 
//  ***OBSOLETE*** (replaced by RP_EXPANDED)
#define RP_ENPANDED                 0x00002010 

// @const DWORD | RP_EXPANDED | 
//  This recipient is an expanded DL
#define RP_EXPANDED                 0x00002010 

// @const DWORD | RP_DSN_SENT_DELAYED | 
//  At least one Delay DSN sent
#define RP_DSN_SENT_DELAYED         0x00004000 

// @const DWORD | RP_DSN_SENT_EXPANDED | 
//  Expanded DSN has been sent
#define RP_DSN_SENT_EXPANDED        0x00008040 

// @const DWORD | RP_DSN_SENT_RELAYED | 
//  Relayed DSN has been sent
#define RP_DSN_SENT_RELAYED         0x00010040 

// @const DWORD | RP_DSN_SENT_DELIVERED | 
//  Delivered DSN has been sent
#define RP_DSN_SENT_DELIVERED       0x00020040 


// @const DWORD | RP_REMOTE_MTA_NO_DSN | 
//  Remote MTA does not advertise DSN support (relay might be needed)
#define RP_REMOTE_MTA_NO_DSN        0x00080000 


// @const DWORD | RP_ERROR_CONTEXT_STORE | 
//  Error happened in store driver
#define RP_ERROR_CONTEXT_STORE      0x00100000 

// @const DWORD | RP_ERROR_CONTEXT_CAT | 
//  Error happened during categorization
#define RP_ERROR_CONTEXT_CAT        0x00200000 

// @const DWORD | RP_ERROR_CONTEXT_MTA | 
//  Error happened in a MTA (eg SMTP stack)
#define RP_ERROR_CONTEXT_MTA        0x00400000 


// @const DWORD | RP_VOLATILE_FLAGS_MASK | 
//  Flags that can be used for temp storage
#define RP_VOLATILE_FLAGS_MASK      0xF0000000 
                                               //while a component has access to recipients
                                               //Once control of recipients is passed, value
                                               //is un-defined.

// @const DWORD | RP_DSN_NOTIFY_INVALID | 
//  *** OBSOLETE ***
#define RP_DSN_NOTIFY_INVALID       0x00000000 

/*=======================================================================*/


// These are the per-recipient properties.

IMMPID_START_LIST(RP,0x2000,"79E82048-D320-11d1-9FF4-00C04FA37348")


    // @const IMMPID | IMMPID_RP_DSN_NOTIFY_SUCCESS | 
    //   *** OBSOLETE ***
    IMMPID_RP_DSN_NOTIFY_SUCCESS,

    // @const IMMPID | IMMPID_RP_DSN_NOTIFY_INVALID | 
    //   *** OBSOLETE ***
    IMMPID_RP_DSN_NOTIFY_INVALID,    

    // @const IMMPID | IMMPID_RP_ADDRESS_TYPE | 
    //   *** OBSOLETE ***
    IMMPID_RP_ADDRESS_TYPE,            

    // @const IMMPID | IMMPID_RP_ADDRESS | 
    //   *** OBSOLETE ***
    IMMPID_RP_ADDRESS,                

    // @const IMMPID | IMMPID_RP_ADDRESS_TYPE_SMTP | 
    //   *** OBSOLETE ***
    IMMPID_RP_ADDRESS_TYPE_SMTP,    

    // @const IMMPID | IMMPID_RP_ERROR_CODE | 
    //   HRESULT status code
    IMMPID_RP_ERROR_CODE,            

    // @const IMMPID | IMMPID_RP_ERROR_STRING | 
    //   *** OBSOLETE ***
    IMMPID_RP_ERROR_STRING,            

    // @const IMMPID | IMMPID_RP_DSN_NOTIFY_VALUE | 
    //   *** OBSOLETE ***
    IMMPID_RP_DSN_NOTIFY_VALUE,        

    // @const IMMPID | IMMPID_RP_DSN_ORCPT_VALUE | 
    //   ANSI string - \<address type\>;\<address\>
    IMMPID_RP_DSN_ORCPT_VALUE,        

    // @const IMMPID | IMMPID_RP_ADDRESS_SMTP | 
    //   ANSI string - SMTP address
    IMMPID_RP_ADDRESS_SMTP,            

    // @const IMMPID | IMMPID_RP_ADDRESS_X400 | 
    //   ANSI string - X.400 address
    IMMPID_RP_ADDRESS_X400,            

    // @const IMMPID | IMMPID_RP_ADDRESS_X500 | 
    //   ANSI string - X.500 address
    IMMPID_RP_ADDRESS_X500,            

    // @const IMMPID | IMMPID_RP_LEGACY_EX_DN | 
    //   ANSI string - DN for Exchange 5.5 and prev
    IMMPID_RP_LEGACY_EX_DN,            

    // @const IMMPID | IMMPID_RP_RECIPIENT_FLAGS | 
    //   Per-recipient DSN/delivery flags. Flag constansts start with RP_.
    IMMPID_RP_RECIPIENT_FLAGS,      

    // @const IMMPID | IMMPID_RP_SMTP_STATUS_STRING | 
    //   ANSI string - SMTP status string... if defined
    IMMPID_RP_SMTP_STATUS_STRING,   
                                    // *must* start with 3-digit status code

    // @const IMMPID | IMMPID_RP_DSN_PRE_CAT_ADDRESS | 
    //   Original address as received by MTA in
    IMMPID_RP_DSN_PRE_CAT_ADDRESS,  
                                    // IMMPID_RP_DSN_ORCPT_VALUE format

    // @const IMMPID | IMMPID_RP_MDB_GUID | 
    //   Categorizer stamps the MDB guid
    IMMPID_RP_MDB_GUID,             
                                    // for this recipient here

    // @const IMMPID | IMMPID_RP_USER_GUID | 
    //   Categorizer stamps the
    IMMPID_RP_USER_GUID,            
                                    // objectGUID of the user object here
    // @const IMMPID | IMMPID_RP_DOMAIN | 
    //   Alternate SMTP domain for categorization
    IMMPID_RP_DOMAIN,            

    // @const IMMPID | IMMPID_RP_ADDRESS_OTHER | 
    //   ANSI string - other address type
    IMMPID_RP_ADDRESS_OTHER,            

    // @const IMMPID | IMMPID_RP_DISPLAY_NAME |
    //   Unicode string - recipient display name
    IMMPID_RP_DISPLAY_NAME,

    // Add new per-recipient properties above this line.
IMMPID_END_LIST(RP)


/*=======================================================================*/


// These are per-message volatile properties - they are not persisted to
// the property stream.

IMMPID_START_LIST(MPV,0x3000,"CBE69706-C9BD-11d1-9FF2-00C04FA37348")

    // @const IMMPID | IMMPID_MPV_STORE_DRIVER_HANDLE | 
    //  Store driver context for this message.
    IMMPID_MPV_STORE_DRIVER_HANDLE,

    // @const IMMPID | IMMPID_MPV_MESSAGE_CREATION_FLAGS | 
    //  Flags set at creation of mailmsg.
    IMMPID_MPV_MESSAGE_CREATION_FLAGS,

    // @const IMMPID | IMMPID_MPV_MESSAGE_OPEN_HANDLES | 
    // The number of handles (property and content) open for this message.
    //   0 means no handles open.
    //   1 means property or content open.
    //   2 means both property and content open.
    IMMPID_MPV_MESSAGE_OPEN_HANDLES,

    // @const IMMPID | IMMPID_MPV_TOTAL_OPEN_HANDLES | 
    // The current total number of open message handles (of any type) 
    // on this server. 
    IMMPID_MPV_TOTAL_OPEN_HANDLES,

    // @const IMMPID | IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES | 
    // The current total number of open property streams on this server. 
    IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES,

    // @const IMMPID | IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES | 
    // The current total number of open content handles on this server. 
    IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES,

    // Add new per-message volatile properties above this line.
IMMPID_END_LIST(MPV)

// This is the structure for the IMMPID_MPV_STORE_DRIVER_HANDLE property.
typedef struct tagIMMP_MPV_STORE_DRIVER_HANDLE {
    GUID guidSignature;    // signature of the store driver whose handle this is
} IMMP_MPV_STORE_DRIVER_HANDLE;

// Define message object creation flags

// @const DWORD | MPV_INBOUND_CUTOFF_EXCEEDED | 
//  Mailmsg configured cutoff has been exceeded... if this is an external 
//  delivery attempt, we should indicate that we do not have sufficient 
//  resources to accept the mail.
#define MPV_INBOUND_CUTOFF_EXCEEDED            0x00000001

// @const DWORD | MPV_WRITE_CONTENT |
//  Indicates that the content of this message can be modified.
#define MPV_WRITE_CONTENT                      0x00000002

/*=======================================================================*/


// These are per-recipient volatile properties - they are not persisted to
// the property stream.

IMMPID_START_LIST(RPV,0x4000,"79E82049-D320-11d1-9FF4-00C04FA37348")

    // @const IMMPID | IMMPID_RPV_DONT_DELIVER | 
    //  IMMPID_RPV_DONT_DELIVER is a boolean.  If set to TRUE, mailmsg 
    //  ignores this recipient when doing WriteList() (the recipient will 
    //  not exist in the new list).

    IMMPID_RPV_DONT_DELIVER,

    // @const IMMPID | IMMPID_RPV_NO_NAME_COLLISIONS | 
    //  IMMPID_RPV_NO_NAME_COLLISIONS is a boolean.  If set to TRUE, mailmsg 
    //  does not detect duplicates with this recipient on future calls to 
    //  AddSecondary().

    IMMPID_RPV_NO_NAME_COLLISIONS,
    // Add new per-recipient volatile properties above this line.
IMMPID_END_LIST(RPV)


// These are defined for backwards-compatability.  They will be removed
// ASAP...

// @const DWORD | IMMPID_RP_DONT_DELIVER | 
//   *** OBSOLETE ***
#define IMMPID_RP_DONT_DELIVER            IMMPID_RPV_DONT_DELIVER

// @const DWORD | IMMPID_RP_NO_NAME_COLLISIONS | 
//   *** OBSOLETE ***
#define IMMPID_RP_NO_NAME_COLLISIONS    IMMPID_RPV_NO_NAME_COLLISIONS


/*=======================================================================*/

// These are the per-message properties for NNTP

IMMPID_START_LIST(NMP,0x6000,"7433a9aa-20e2-11d2-94d6-00c04fa379f1")

    // @const IMMPID | IMMPID_NMP_SECONDARY_GROUPS | 
    //  An array of pointers to INNTPPropertyBag objects for each of
    //  the newsgroups that the article is being posted into for the
    //  current driver.
    IMMPID_NMP_SECONDARY_GROUPS,        

    // @const IMMPID | IMMPID_NMP_SECONDARY_ARTNUM | 
    //  An array of article numbers for each of the newsgroups that
    //  the article is being posted into for the current driver.
    IMMPID_NMP_SECONDARY_ARTNUM,

    // @const IMMPID | IMMPID_NMP_PRIMARY_GROUP | 
    //  A pointer to the INNTPPropertyBag object which represents the
    //  primary group for the current driver.
    IMMPID_NMP_PRIMARY_GROUP,

    // @const IMMPID | IMMPID_NMP_PRIMARY_ARTID | 
    //  The primary article number for the primary group.
    IMMPID_NMP_PRIMARY_ARTID,

    // @const IMMPID | IMMPID_NMP_POST_TOKEN | 
    //   The HTOKEN representing the client context.
    IMMPID_NMP_POST_TOKEN,                

    // @const IMMPID | IMMPID_NMP_NEWSGROUP_LIST | 
    //   The string of newsgroups which this article is being stored in.
    IMMPID_NMP_NEWSGROUP_LIST,            

    // @const IMMPID | IMMPID_NMP_HEADERS | 
    //   A string containing the headers of the message.
    IMMPID_NMP_HEADERS,                    

    // @const IMMPID | IMMPID_NMP_NNTP_PROCESSING | 
    //   Flags which describe how the message should be processed.  The 
    //   possible values are: NMP_PROCESS_POST, NMP_PROCESS_CONTROL, 
    //   and NMP_PROCESS_MODERATOR.  
    IMMPID_NMP_NNTP_PROCESSING,            

    // @const IMMPID | IMMPID_NMP_NNTP_APPROVED_HEADER | 
    //   A string containing the Approved: header of the message  
    IMMPID_NMP_NNTP_APPROVED_HEADER,            

    // Add new per-message properties above this line.
IMMPID_END_LIST(NMP)

// flags for IMMPID_NMP_NNTP_PROCESSING

// @const DWORD | NMP_PROCESS_POST | 
//  Set this flag to allow the NNTP server to post this message.
#define NMP_PROCESS_POST            0x00000001

// @const DWORD | NMP_PROCESS_CONTROL | 
//  Set this flag to allow the NNTP server to process control headers in this
//  message.
#define NMP_PROCESS_CONTROL            0x00000002

// @const DWORD | NMP_PROCESS_MODERATOR | 
//  Set this flag to allow the NNTP server to run this message through the
//  default moderated posting path.
#define NMP_PROCESS_MODERATOR        0x00000004

/*=======================================================================*/


// This is the starting range for user-reserved properties

IMMPID_START_LIST(CPV,0x8000,"A2A76B2A-E52D-11d1-AA64-00C04FA35B82")

    // @const IMMPID | IMMPID_CP_START | 
    //  Start of range
    IMMPID_CP_START,
IMMPID_END_LIST(CPV)


/*=======================================================================*/

// This table collects the data about the properties so that range-
// checking can be performed.  If any new property ranges are
// defined, they must be added to this structure.


#define IMMPID_DECLARE_ENTRY(name)    {&_uuidof(tagIMMPID_##name##_STRUCT),\
                                     IMMPID_##name##_BEFORE__+1,\
                                     IMMPID_##name##_AFTER__-1}

extern const __declspec(selectany) struct tagIMMPID_GUIDLIST_ITEM {
                                       const GUID *pguid;
                                       DWORD dwStart;
                                       DWORD dwLast;
                                   } IMMPID_GUIDS[] = {IMMPID_DECLARE_ENTRY(MP),
                                                       IMMPID_DECLARE_ENTRY(RP),
                                                       IMMPID_DECLARE_ENTRY(MPV),
                                                       IMMPID_DECLARE_ENTRY(RPV),
                                                       {&GUID_NULL,0,0}};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MAILMSGPROPS_H_

