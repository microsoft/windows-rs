/*++

Copyright (c) 2000  Microsoft Corporation

Module Name:

        ws2bth.h

Abstract:

        Winsock 2 Bluetooth Annex definitions.
        
Notes:

        Change BT_* to BTH_*        

--*/

#ifndef __WS2BTH__H
#define __WS2BTH__H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) // nameless struct/union

#if (NTDDI_VERSION >= NTDDI_WINXPSP2)

#include <bthdef.h>

#include <pshpack1.h>

#define BT_PORT_ANY        ((ULONG)-1)
#define BT_PORT_MIN        0x1
#define BT_PORT_MAX        0xffff
#define BT_PORT_DYN_FIRST  0x1001

//
// These three definitions are duplicated in winsock2.h to reserve ordinals
//
#ifndef AF_BTH
#define AF_BTH  32
#endif
#ifndef PF_BTH
#define PF_BTH  AF_BTH
#endif
#ifndef NS_BTH
#define NS_BTH  16
#endif

typedef struct _SOCKADDR_BTH
{
    USHORT      addressFamily;  // Always AF_BTH
    BTH_ADDR    btAddr;         // Bluetooth device address
    GUID        serviceClassId; // [OPTIONAL] system will query SDP for port
    ULONG       port;           // RFCOMM channel or L2CAP PSM
} SOCKADDR_BTH, *PSOCKADDR_BTH;


DEFINE_GUID(SVCID_BTH_PROVIDER, 0x6aa63e0, 0x7d60, 0x41ff, 0xaf, 0xb2, 0x3e, 0xe6, 0xd2, 0xd9, 0x39, 0x2d);
#define BTH_ADDR_STRING_SIZE        12  // max size from WSAAddressToString

//
// Bluetooth protocol #s are assigned according to the Bluetooth
// Assigned Numbers portion of the Bluetooth Specification
//
#define BTHPROTO_RFCOMM  0x0003
#define BTHPROTO_L2CAP   0x0100

#define SOL_RFCOMM  BTHPROTO_RFCOMM
#define SOL_L2CAP   BTHPROTO_L2CAP
#define SOL_SDP     0x0101

//
// SOCKET OPTIONS
//
#define SO_BTH_AUTHENTICATE 0x80000001  // optlen=sizeof(ULONG), optval = &(ULONG)TRUE/FALSE 
#define SO_BTH_ENCRYPT      0x00000002  // optlen=sizeof(ULONG), optval = &(ULONG)TRUE/FALSE
#define SO_BTH_MTU          0x80000007  // optlen=sizeof(ULONG), optval = &mtu
#define SO_BTH_MTU_MAX      0x80000008  // optlen=sizeof(ULONG), optval = &max. mtu
#define SO_BTH_MTU_MIN      0x8000000a  // optlen=sizeof(ULONG), optval = &min. mtu

//
// Socket option parameters
//
// 3-DH5 => payload of 1021 => L2cap payload of 1017 => RFComm payload of 1011
#define RFCOMM_MAX_MTU      0x000003F3  // L2CAP MTU (1017) - RFCOMM header size (6)
#define RFCOMM_MIN_MTU      0x00000017  // RFCOMM spec sec 5.3 table 5.1

//
// NAME SERVICE PROVIDER DEFINITIONS
// For calling WSASetService
// and WSALookupServiceBegin, WSALookupServiceNext, WSALookupServiceEnd
// with Bluetooth-specific extensions
//
#define BTH_SDP_VERSION         1

//
// [OPTIONAL] passed in BLOB member of WSAQUERYSET
// QUERYSET and its lpBlob member are copied & converted
// to unicode in the system for non-unicode applications.  
// However, nothing is copied back upon return.  In 
// order for the system to return data such as pRecordHandle, 
// it much have an extra level of indirection from lpBlob 
//
typedef struct _BTH_SET_SERVICE {

        //
        // This version number will change when/if the binary format of
        // SDP records change, affecting the format of pRecord.
        // Set to BTH_SDP_VERSION by client, and returned by system
        //
        PULONG pSdpVersion;

        //
        // Handle to SDP record.  When BTH_SET_SERVICE structure is later 
        // passed to WSASetService RNRSERVICE_DELETE, this handle identifies the
        // record to delete.
        //
        HANDLE *pRecordHandle;
                        
        // 
        // COD_SERVICE_* bit(s) associated with this SDP record, which will be
        // advertised when the local radio is found during device inquiry.
        // When the last SDP record associated with a bit is deleted, that
        // service bit is no longer reported in repsonse to inquiries 
        //
        ULONG fCodService;    // COD_SERVICE_* bits

        ULONG Reserved[5];    // Reserved by system.  Must be zero.                
        ULONG ulRecordLength; // length of pRecord which follows
        UCHAR pRecord[1];     // SDP record as defined by bluetooth spec
        
} BTH_SET_SERVICE, *PBTH_SET_SERVICE;

//
// Default device inquiry duration in seconds
//
// The application thread will be blocked in WSALookupServiceBegin
// for the duration of the device inquiry, so this value needs to
// be balanced against the chance that a device that is actually
// present might not being found by Bluetooth in this time
//
// Paging improvements post-1.1 will cause devices to be 
// found generally uniformly in the 0-6 sec timeperiod
//
#define SDP_DEFAULT_INQUIRY_SECONDS         6
#define SDP_MAX_INQUIRY_SECONDS             60

//
// Default maximum number of devices to search for
//
#define SDP_DEFAULT_INQUIRY_MAX_RESPONSES   255

#define SDP_SERVICE_SEARCH_REQUEST           1
#define SDP_SERVICE_ATTRIBUTE_REQUEST        2
#define SDP_SERVICE_SEARCH_ATTRIBUTE_REQUEST 3

//
// [OPTIONAL] input restrictions on device inquiry
// Passed in BLOB of LUP_CONTAINERS (device) search
//
typedef struct _BTH_QUERY_DEVICE {
    ULONG   LAP;                    // reserved: must be 0 (GIAC inquiry only)
    UCHAR   length;                 // requested length of inquiry (seconds)       
} BTH_QUERY_DEVICE, *PBTH_QUERY_DEVICE;

//
// [OPTIONAL] Restrictions on searching for a particular service
// Passed in BLOB of !LUP_CONTAINERS (service) search
//
typedef struct _BTH_QUERY_SERVICE {
	ULONG                   type;           // one of SDP_SERVICE_*
	ULONG                   serviceHandle;
	SdpQueryUuid            uuids[MAX_UUIDS_IN_QUERY];
	ULONG                   numRange;
	SdpAttributeRange       pRange[1];
} BTH_QUERY_SERVICE, *PBTH_QUERY_SERVICE;

//
// BTHNS_RESULT_*
//
// Bluetooth specific flags returned from WSALookupServiceNext 
// in WSAQUERYSET.dwOutputFlags in response to device inquiry
//

//
// Local device is paired with remote device
//
#define BTHNS_RESULT_DEVICE_CONNECTED      0x00010000
#define BTHNS_RESULT_DEVICE_REMEMBERED     0x00020000
#define BTHNS_RESULT_DEVICE_AUTHENTICATED  0x00040000

//
// SOCKET IOCTLs
//

#define SIO_RFCOMM_SEND_COMMAND        _WSAIORW (IOC_VENDOR, 101)
#define SIO_RFCOMM_WAIT_COMMAND        _WSAIORW (IOC_VENDOR, 102)

//
// These IOCTLs are for test/validation/conformance and may only be
// present in debug/checked builds of the system
//
#define SIO_BTH_PING                      _WSAIORW (IOC_VENDOR, 8)
#define SIO_BTH_INFO                      _WSAIORW (IOC_VENDOR, 9)
#define SIO_RFCOMM_SESSION_FLOW_OFF       _WSAIORW (IOC_VENDOR, 103)
#define SIO_RFCOMM_TEST                   _WSAIORW (IOC_VENDOR, 104)
#define SIO_RFCOMM_USECFC                 _WSAIORW (IOC_VENDOR, 105)
/*      RESERVED                          _WSAIORW (IOC_VENDOR, 106) */


//
// SOCKET IOCTL DEFINITIONS
//

#ifndef BIT
#define BIT(b)   (1<<(b))
#endif

//
// Structure definition from Bluetooth RFCOMM spec, TS 07.10 5.4.6.3.7
//
typedef struct _RFCOMM_MSC_DATA {
    UCHAR       Signals;

        #define MSC_EA_BIT      EA_BIT
        #define MSC_FC_BIT      BIT(1)      // Flow control, clear if we can receive
        #define MSC_RTC_BIT     BIT(2)      // Ready to communicate, set when ready
        #define MSC_RTR_BIT     BIT(3)      // Ready to receive, set when ready
        #define MSC_RESERVED (BIT(4)|BIT(5))// Reserved by spec, must be 0
        #define MSC_IC_BIT      BIT(6)      // Incoming call
        #define MSC_DV_BIT      BIT(7)      // Data valid

    UCHAR       Break;

        #define MSC_BREAK_BIT   BIT(1)      // Set if sending break
        #define MSC_SET_BREAK_LENGTH(b, l)  ((b) = ((b)&0x3) | (((l)&0xf) << 4))

} RFCOMM_MSC_DATA, *PRFCOMM_MSC_DATA;

//
// Structure definition from Bluetooth RFCOMM spec, TS 07.10 5.4.6.3.10
//
typedef struct _RFCOMM_RLS_DATA {

    UCHAR       LineStatus;

        #define RLS_ERROR           0x01
        #define RLS_OVERRUN         0x02
        #define RLS_PARITY          0x04
        #define RLS_FRAMING         0x08

} RFCOMM_RLS_DATA, *PRFCOMM_RLS_DATA;

//
// Structure definition from Bluetooth RFCOMM spec, TS 07.10 5.4.6.3.9
//
typedef struct _RFCOMM_RPN_DATA {
    UCHAR       Baud;

        #define RPN_BAUD_2400       0
        #define RPN_BAUD_4800       1
        #define RPN_BAUD_7200       2
        #define RPN_BAUD_9600       3
        #define RPN_BAUD_19200      4
        #define RPN_BAUD_38400      5
        #define RPN_BAUD_57600      6
        #define RPN_BAUD_115200     7
        #define RPN_BAUD_230400     8

    UCHAR       Data;

        #define RPN_DATA_5          0x0
        #define RPN_DATA_6          0x1
        #define RPN_DATA_7          0x2
        #define RPN_DATA_8          0x3

        #define RPN_STOP_1          0x0
        #define RPN_STOP_1_5        0x4

        #define RPN_PARITY_NONE     0x00
        #define RPN_PARITY_ODD      0x08
        #define RPN_PARITY_EVEN     0x18
        #define RPN_PARITY_MARK     0x28
        #define RPN_PARITY_SPACE    0x38

    UCHAR       FlowControl;

        #define RPN_FLOW_X_IN       0x01
        #define RPN_FLOW_X_OUT      0x02
        #define RPN_FLOW_RTR_IN     0x04
        #define RPN_FLOW_RTR_OUT    0x08
        #define RPN_FLOW_RTC_IN     0x10
        #define RPN_FLOW_RTC_OUT    0x20

    UCHAR       XonChar;
    UCHAR       XoffChar;
    UCHAR       ParameterMask1;

        #define RPN_PARAM_BAUD      0x01
        #define RPN_PARAM_DATA      0x02
        #define RPN_PARAM_STOP      0x04
        #define RPN_PARAM_PARITY    0x08
        #define RPN_PARAM_P_TYPE    0x10
        #define RPN_PARAM_XON       0x20
        #define RPN_PARAM_XOFF      0x40

    UCHAR       ParameterMask2;

        #define RPN_PARAM_X_IN      0x01
        #define RPN_PARAM_X_OUT     0x02
        #define RPN_PARAM_RTR_IN    0x04
        #define RPN_PARAM_RTR_OUT   0x08
        #define RPN_PARAM_RTC_IN    0x10
        #define RPN_PARAM_RTC_OUT   0x20

} RFCOMM_RPN_DATA, *PRFCOMM_RPN_DATA;

#define RFCOMM_CMD_NONE             0
#define RFCOMM_CMD_MSC              1
#define RFCOMM_CMD_RLS              2
#define RFCOMM_CMD_RPN              3
#define RFCOMM_CMD_RPN_REQUEST      4
#define RFCOMM_CMD_RPN_RESPONSE     5
/*      RESERVED_CMD                6 */

typedef struct _RFCOMM_COMMAND
{
    ULONG       CmdType;          // one of RFCOMM_CMD_*
    union
    {
        RFCOMM_MSC_DATA    MSC;
        RFCOMM_RLS_DATA    RLS;
        RFCOMM_RPN_DATA    RPN;
    } Data;
} RFCOMM_COMMAND, *PRFCOMM_COMMAND;

//
// These structures are for test/validation/conformance and may only be
// present in debug/checked builds of the system
//
typedef struct _BTH_PING_REQ {
    BTH_ADDR  btAddr;
    UCHAR    dataLen;
    UCHAR    data[MAX_L2CAP_PING_DATA_LENGTH];
} BTH_PING_REQ, *PBTH_PING_REQ;

typedef struct _BTH_PING_RSP {
    UCHAR    dataLen;
    UCHAR    data[MAX_L2CAP_PING_DATA_LENGTH];
} BTH_PING_RSP, *PBTH_PING_RSP;

typedef struct _BTH_INFO_REQ {
    BTH_ADDR  btAddr;
    USHORT   infoType;
} BTH_INFO_REQ, *PBTH_INFO_REQ;

typedef struct _BTH_INFO_RSP {
    USHORT  result;
    UCHAR   dataLen;
    union {
        USHORT  connectionlessMTU;
        UCHAR   data[MAX_L2CAP_INFO_DATA_LENGTH];
    };
} BTH_INFO_RSP, *PBTH_INFO_RSP;

//
// WinCE compatible struct names
//
typedef struct _BTH_SET_SERVICE BTHNS_SETBLOB, *PBTHNS_SETBLOB;
typedef struct _BTH_QUERY_DEVICE BTHNS_INQUIRYBLOB, *PBTHNS_INQUIRYBLOB;
typedef struct _BTH_QUERY_SERVICE BTHNS_RESTRICTIONBLOB, *PBTHNS_RESTRICTIONBLOB;

#include <poppack.h>

#endif // (NTDDI_VERSION >= NTDDI_WINXPSP2)

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __WS2BTH__H
