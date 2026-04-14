
/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

Abstract:

    This module contains the WIN32 FAX APIs.

--*/



#ifndef _FAXAPI_
#define _FAXAPI_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef MIDL_PASS
#include <tapi.h>
#endif

#if !defined(_WINFAX_)
#define WINFAXAPI DECLSPEC_IMPORT
#else
#define WINFAXAPI
#endif

#ifdef __cplusplus
extern "C" {
#endif

//
// FAX ERROR CODES
//

#define FAX_ERR_START                                   7001L   // First fax specific error code

#define FAX_ERR_SRV_OUTOFMEMORY                         7001L
#define FAX_ERR_GROUP_NOT_FOUND                         7002L
#define FAX_ERR_BAD_GROUP_CONFIGURATION                 7003L
#define FAX_ERR_GROUP_IN_USE                            7004L
#define FAX_ERR_RULE_NOT_FOUND                          7005L
#define FAX_ERR_NOT_NTFS                                7006L
#define FAX_ERR_DIRECTORY_IN_USE                        7007L
#define FAX_ERR_FILE_ACCESS_DENIED                      7008L
#define FAX_ERR_MESSAGE_NOT_FOUND                       7009L
#define FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED               7010L
#define FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU               7011L
#define FAX_ERR_VERSION_MISMATCH                        7012L   // Fax client/server versions mismtach
#define FAX_ERR_RECIPIENTS_LIMIT                        7013L   // Recipients limit in a single broadcast

#define FAX_ERR_END                                     7013L   // Last fax specific error code


//
// MessageId: FAX_E_SRV_OUTOFMEMORY
//
// MessageText:
//
//  The fax server failed to allocate memory.
//
#define FAX_E_SRV_OUTOFMEMORY                MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_SRV_OUTOFMEMORY)

//
// MessageId: FAX_E_GROUP_NOT_FOUND
//
// MessageText:
//
//  The fax server failed to locate an outbound routing group by name.
//
#define FAX_E_GROUP_NOT_FOUND                MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_GROUP_NOT_FOUND)

//
// MessageId: FAX_E_BAD_GROUP_CONFIGURATION
//
// MessageText:
//
//  The fax server encountered an outbound routing group with bad configuration.
//
#define FAX_E_BAD_GROUP_CONFIGURATION        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_BAD_GROUP_CONFIGURATION)

//
// MessageId: FAX_E_GROUP_IN_USE
//
// MessageText:
//
//  The fax server cannot remove an outbound routing group because it is in use by one or more outbound routing rules.
//
#define FAX_E_GROUP_IN_USE                   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_GROUP_IN_USE)

//
// MessageId: FAX_E_RULE_NOT_FOUND
//
// MessageText:
//
//  The fax server failed to locate an outbound routing rule by country/region code and area code.
//
#define FAX_E_RULE_NOT_FOUND                 MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_RULE_NOT_FOUND)

//
// MessageId: FAX_E_NOT_NTFS
//
// MessageText:
//
//  The fax server cannot set an archive folder to a non-NTFS partition.
//
#define FAX_E_NOT_NTFS                       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_NOT_NTFS)

//
// MessageId: FAX_E_DIRECTORY_IN_USE
//
// MessageText:
//
//  The fax server cannot use the same folder for both the inbox and the sent-items archives.
//
#define FAX_E_DIRECTORY_IN_USE               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_DIRECTORY_IN_USE)

//
// MessageId: FAX_E_FILE_ACCESS_DENIED
//
// MessageText:
//
//  The fax server cannot access the specified file or folder.
//
#define FAX_E_FILE_ACCESS_DENIED             MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_FILE_ACCESS_DENIED)

//
// MessageId: FAX_E_MESSAGE_NOT_FOUND
//
// MessageText:
//
//  The fax server cannot find the job or message by its ID.
//
#define FAX_E_MESSAGE_NOT_FOUND              MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_MESSAGE_NOT_FOUND)

//
// MessageId: FAX_E_DEVICE_NUM_LIMIT_EXCEEDED
//
// MessageText:
//
//  The fax server cannot complete the operation because the number of active fax devices allowed for this version of Windows was exceeded.
//
#define FAX_E_DEVICE_NUM_LIMIT_EXCEEDED      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED)

//
// MessageId: FAX_E_NOT_SUPPORTED_ON_THIS_SKU
//
// MessageText:
//
//  The fax server cannot complete the operation because it is not supported for this version of Windows.
//
#define FAX_E_NOT_SUPPORTED_ON_THIS_SKU      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU)

//
// MessageId: FAX_E_VERSION_MISMATCH
//
// MessageText:
//
//  The fax server API version does not support the requested operation.
//
#define FAX_E_VERSION_MISMATCH               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_VERSION_MISMATCH)

//
// MessageId: FAX_E_RECIPIENT_LIMIT
//
// MessageText:
//
// The limit on the number of recipients for a single fax broadcast was reached.
//
#define FAX_E_RECIPIENTS_LIMIT               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, FAX_ERR_RECIPIENTS_LIMIT)

typedef enum
{
    FAXLOG_LEVEL_NONE            = 0,
    FAXLOG_LEVEL_MIN,
    FAXLOG_LEVEL_MED,
    FAXLOG_LEVEL_MAX
} FAX_ENUM_LOG_LEVELS;

typedef enum
{
    FAXLOG_CATEGORY_INIT        = 1,        // Initialization / shutdown
    FAXLOG_CATEGORY_OUTBOUND,               // Outbound messages
    FAXLOG_CATEGORY_INBOUND,                // Inbound messages
    FAXLOG_CATEGORY_UNKNOWN                 // Unknown category (all others)
} FAX_ENUM_LOG_CATEGORIES;

typedef struct _FAX_LOG_CATEGORYA
{
    LPCSTR              Name;                       // logging category name
    DWORD               Category;                   // logging category number
    DWORD               Level;                      // logging level for the category
} FAX_LOG_CATEGORYA, *PFAX_LOG_CATEGORYA;
typedef struct _FAX_LOG_CATEGORYW
{
    LPCWSTR             Name;                       // logging category name
    DWORD               Category;                   // logging category number
    DWORD               Level;                      // logging level for the category
} FAX_LOG_CATEGORYW, *PFAX_LOG_CATEGORYW;
#ifdef UNICODE
typedef FAX_LOG_CATEGORYW FAX_LOG_CATEGORY;
typedef PFAX_LOG_CATEGORYW PFAX_LOG_CATEGORY;
#else
typedef FAX_LOG_CATEGORYA FAX_LOG_CATEGORY;
typedef PFAX_LOG_CATEGORYA PFAX_LOG_CATEGORY;
#endif // UNICODE

typedef struct _FAX_TIME
{
    WORD    Hour;
    WORD    Minute;
} FAX_TIME, *PFAX_TIME;

typedef struct _FAX_CONFIGURATIONA
{
    DWORD               SizeOfStruct;                   // size of this structure
    DWORD               Retries;                        // number of retries for fax send
    DWORD               RetryDelay;                     // number of minutes between retries
    DWORD               DirtyDays;                      // number of days to keep an unsent job in the queue
    BOOL                Branding;                       // fsp should brand outgoing faxes
    BOOL                UseDeviceTsid;                  // server uses device tsid only
    BOOL                ServerCp;                       // clients must use cover pages on the server
    BOOL                PauseServerQueue;               // is the server queue paused?
    FAX_TIME            StartCheapTime;                 // start of discount rate period
    FAX_TIME            StopCheapTime;                  // end of discount rate period
    BOOL                ArchiveOutgoingFaxes;           // whether outgoing faxes should be archived
    LPCSTR              ArchiveDirectory;               // archive directory for outgoing faxes
    LPCSTR              Reserved;                       // Reserved; must be NULL
} FAX_CONFIGURATIONA, *PFAX_CONFIGURATIONA;
typedef struct _FAX_CONFIGURATIONW
{
    DWORD               SizeOfStruct;                   // size of this structure
    DWORD               Retries;                        // number of retries for fax send
    DWORD               RetryDelay;                     // number of minutes between retries
    DWORD               DirtyDays;                      // number of days to keep an unsent job in the queue
    BOOL                Branding;                       // fsp should brand outgoing faxes
    BOOL                UseDeviceTsid;                  // server uses device tsid only
    BOOL                ServerCp;                       // clients must use cover pages on the server
    BOOL                PauseServerQueue;               // is the server queue paused?
    FAX_TIME            StartCheapTime;                 // start of discount rate period
    FAX_TIME            StopCheapTime;                  // end of discount rate period
    BOOL                ArchiveOutgoingFaxes;           // whether outgoing faxes should be archived
    LPCWSTR             ArchiveDirectory;               // archive directory for outgoing faxes
    LPCWSTR             Reserved;                       // Reserved; must be NULL
} FAX_CONFIGURATIONW, *PFAX_CONFIGURATIONW;
#ifdef UNICODE
typedef FAX_CONFIGURATIONW FAX_CONFIGURATION;
typedef PFAX_CONFIGURATIONW PFAX_CONFIGURATION;
#else
typedef FAX_CONFIGURATIONA FAX_CONFIGURATION;
typedef PFAX_CONFIGURATIONA PFAX_CONFIGURATION;
#endif // UNICODE


//
// FaxSetJob() command codes
//

typedef enum
{
    JC_UNKNOWN      = 0,
    JC_DELETE,
    JC_PAUSE,
    JC_RESUME
} FAX_ENUM_JOB_COMMANDS;

#define JC_RESTART   JC_RESUME


//
// job type defines
//

#define JT_UNKNOWN                  0
#define JT_SEND                     1
#define JT_RECEIVE                  2
#define JT_ROUTING                  3
#define JT_FAIL_RECEIVE             4

//
// job status defines
//
#define JS_PENDING                  0x00000000
#define JS_INPROGRESS               0x00000001
#define JS_DELETING                 0x00000002
#define JS_FAILED                   0x00000004
#define JS_PAUSED                   0x00000008
#define JS_NOLINE                   0x00000010
#define JS_RETRYING                 0x00000020
#define JS_RETRIES_EXCEEDED         0x00000040


typedef struct _FAX_DEVICE_STATUSA
{
    DWORD               SizeOfStruct;               // size of this structure
    LPCSTR              CallerId;                   // caller id string
    LPCSTR              Csid;                       // station identifier
    DWORD               CurrentPage;                // current page
    DWORD               DeviceId;                   // permanent line id
    LPCSTR              DeviceName;                 // device name
    LPCSTR              DocumentName;               // document name
    DWORD               JobType;                    // send or receive?
    LPCSTR              PhoneNumber;                // sending phone number
    LPCSTR              RoutingString;              // routing information
    LPCSTR              SenderName;                 // sender name
    LPCSTR              RecipientName;              // recipient name
    DWORD               Size;                       // size in bytes of the document
    FILETIME            StartTime;                  // starting time of the fax send/receive
    DWORD               Status;                     // current status of the device, see FPS_??? masks
    LPCSTR              StatusString;               // status string if the Status field is zero.  this may be NULL.
    FILETIME            SubmittedTime;              // time the document was submitted
    DWORD               TotalPages;                 // total number of pages in this job
    LPCSTR              Tsid;                       // transmitting station identifier
    LPCSTR              UserName;                   // user that submitted the active job
} FAX_DEVICE_STATUSA, *PFAX_DEVICE_STATUSA;
typedef struct _FAX_DEVICE_STATUSW
{
    DWORD               SizeOfStruct;               // size of this structure
    LPCWSTR             CallerId;                   // caller id string
    LPCWSTR             Csid;                       // station identifier
    DWORD               CurrentPage;                // current page
    DWORD               DeviceId;                   // permanent line id
    LPCWSTR             DeviceName;                 // device name
    LPCWSTR             DocumentName;               // document name
    DWORD               JobType;                    // send or receive?
    LPCWSTR             PhoneNumber;                // sending phone number
    LPCWSTR             RoutingString;              // routing information
    LPCWSTR             SenderName;                 // sender name
    LPCWSTR             RecipientName;              // recipient name
    DWORD               Size;                       // size in bytes of the document
    FILETIME            StartTime;                  // starting time of the fax send/receive
    DWORD               Status;                     // current status of the device, see FPS_??? masks
    LPCWSTR             StatusString;               // status string if the Status field is zero.  this may be NULL.
    FILETIME            SubmittedTime;              // time the document was submitted
    DWORD               TotalPages;                 // total number of pages in this job
    LPCWSTR             Tsid;                       // transmitting station identifier
    LPCWSTR             UserName;                   // user that submitted the active job
} FAX_DEVICE_STATUSW, *PFAX_DEVICE_STATUSW;
#ifdef UNICODE
typedef FAX_DEVICE_STATUSW FAX_DEVICE_STATUS;
typedef PFAX_DEVICE_STATUSW PFAX_DEVICE_STATUS;
#else
typedef FAX_DEVICE_STATUSA FAX_DEVICE_STATUS;
typedef PFAX_DEVICE_STATUSA PFAX_DEVICE_STATUS;
#endif // UNICODE

typedef struct _FAX_JOB_ENTRYA
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               JobId;                      // fax job id
    LPCSTR              UserName;                   // user who submitted the job
    DWORD               JobType;                    // job type, see JT defines
    DWORD               QueueStatus;                // job queue status, see JS defines
    DWORD               Status;                     // current status of the device, see FPS_??? masks
    DWORD               Size;                       // size in bytes of the document
    DWORD               PageCount;                  // total page count
    LPCSTR              RecipientNumber;            // recipient fax number
    LPCSTR              RecipientName;              // recipient name
    LPCSTR              Tsid;                       // transmitter's id
    LPCSTR              SenderName;                 // sender name
    LPCSTR              SenderCompany;              // sender company
    LPCSTR              SenderDept;                 // sender department
    LPCSTR              BillingCode;                // billing code
    DWORD               ScheduleAction;             // when to schedule the fax, see JSA defines
    SYSTEMTIME          ScheduleTime;               // time to send the fax when JSA_SPECIFIC_TIME is used (must be local time)
    DWORD               DeliveryReportType;         // delivery report type, see DRT defines
    LPCSTR              DeliveryReportAddress;      // email address for delivery report (ndr or dr) thru MAPI / SMTP
    LPCSTR              DocumentName;               // document name
} FAX_JOB_ENTRYA, *PFAX_JOB_ENTRYA;
typedef struct _FAX_JOB_ENTRYW
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               JobId;                      // fax job id
    LPCWSTR             UserName;                   // user who submitted the job
    DWORD               JobType;                    // job type, see JT defines
    DWORD               QueueStatus;                // job queue status, see JS defines
    DWORD               Status;                     // current status of the device, see FPS_??? masks
    DWORD               Size;                       // size in bytes of the document
    DWORD               PageCount;                  // total page count
    LPCWSTR             RecipientNumber;            // recipient fax number
    LPCWSTR             RecipientName;              // recipient name
    LPCWSTR             Tsid;                       // transmitter's id
    LPCWSTR             SenderName;                 // sender name
    LPCWSTR             SenderCompany;              // sender company
    LPCWSTR             SenderDept;                 // sender department
    LPCWSTR             BillingCode;                // billing code
    DWORD               ScheduleAction;             // when to schedule the fax, see JSA defines
    SYSTEMTIME          ScheduleTime;               // time to send the fax when JSA_SPECIFIC_TIME is used (must be local time)
    DWORD               DeliveryReportType;         // delivery report type, see DRT defines
    LPCWSTR             DeliveryReportAddress;      // email address for delivery report (ndr or dr) thru MAPI / SMTP
    LPCWSTR             DocumentName;               // document name
} FAX_JOB_ENTRYW, *PFAX_JOB_ENTRYW;
#ifdef UNICODE
typedef FAX_JOB_ENTRYW FAX_JOB_ENTRY;
typedef PFAX_JOB_ENTRYW PFAX_JOB_ENTRY;
#else
typedef FAX_JOB_ENTRYA FAX_JOB_ENTRY;
typedef PFAX_JOB_ENTRYA PFAX_JOB_ENTRY;
#endif // UNICODE

//
// fax port state masks
//
// if you change these defines the be sure to
// change the resources in the fax service.
//

#define FPS_DIALING              0x20000001
#define FPS_SENDING              0x20000002
#define FPS_RECEIVING            0x20000004
#define FPS_COMPLETED            0x20000008
#define FPS_HANDLED              0x20000010
#define FPS_UNAVAILABLE          0x20000020
#define FPS_BUSY                 0x20000040
#define FPS_NO_ANSWER            0x20000080
#define FPS_BAD_ADDRESS          0x20000100
#define FPS_NO_DIAL_TONE         0x20000200
#define FPS_DISCONNECTED         0x20000400
#define FPS_FATAL_ERROR          0x20000800
#define FPS_NOT_FAX_CALL         0x20001000
#define FPS_CALL_DELAYED         0x20002000
#define FPS_CALL_BLACKLISTED     0x20004000
#define FPS_INITIALIZING         0x20008000
#define FPS_OFFLINE              0x20010000
#define FPS_RINGING              0x20020000

#define FPS_AVAILABLE            0x20100000
#define FPS_ABORTING             0x20200000
#define FPS_ROUTING              0x20400000
#define FPS_ANSWERED             0x20800000

//
// fax port capability mask
//

#define FPF_RECEIVE       0x00000001        // Automatically receive faxes
#define FPF_SEND          0x00000002
#define FPF_VIRTUAL       0x00000004

typedef struct _FAX_PORT_INFOA
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               DeviceId;                   // Device ID
    DWORD               State;                      // State of the device
    DWORD               Flags;                      // Device specific flags
    DWORD               Rings;                      // Number of rings before answer
    DWORD               Priority;                   // Device priority
    LPCSTR              DeviceName;                 // Device name
    LPCSTR              Tsid;                       // Device Tsid
    LPCSTR              Csid;                       // Device Csid
} FAX_PORT_INFOA, *PFAX_PORT_INFOA;
typedef struct _FAX_PORT_INFOW
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               DeviceId;                   // Device ID
    DWORD               State;                      // State of the device
    DWORD               Flags;                      // Device specific flags
    DWORD               Rings;                      // Number of rings before answer
    DWORD               Priority;                   // Device priority
    LPCWSTR             DeviceName;                 // Device name
    LPCWSTR             Tsid;                       // Device Tsid
    LPCWSTR             Csid;                       // Device Csid
} FAX_PORT_INFOW, *PFAX_PORT_INFOW;
#ifdef UNICODE
typedef FAX_PORT_INFOW FAX_PORT_INFO;
typedef PFAX_PORT_INFOW PFAX_PORT_INFO;
#else
typedef FAX_PORT_INFOA FAX_PORT_INFO;
typedef PFAX_PORT_INFOA PFAX_PORT_INFO;
#endif // UNICODE


typedef struct _FAX_ROUTING_METHODA
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               DeviceId;                   // device identifier
    BOOL                Enabled;                    // is this method enabled for this device?
    LPCSTR              DeviceName;                 // device name
    LPCSTR              Guid;                       // guid that identifies this routing method
    LPCSTR              FriendlyName;               // friendly name for this method
    LPCSTR              FunctionName;               // exported function name that identifies this method
    LPCSTR              ExtensionImageName;         // module (dll) name that implements this method
    LPCSTR              ExtensionFriendlyName;      // displayable string that identifies the extension
} FAX_ROUTING_METHODA, *PFAX_ROUTING_METHODA;
typedef struct _FAX_ROUTING_METHODW
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               DeviceId;                   // device identifier
    BOOL                Enabled;                    // is this method enabled for this device?
    LPCWSTR             DeviceName;                 // device name
    LPCWSTR             Guid;                       // guid that identifies this routing method
    LPCWSTR             FriendlyName;               // friendly name for this method
    LPCWSTR             FunctionName;               // exported function name that identifies this method
    LPCWSTR             ExtensionImageName;         // module (dll) name that implements this method
    LPCWSTR             ExtensionFriendlyName;      // displayable string that identifies the extension
} FAX_ROUTING_METHODW, *PFAX_ROUTING_METHODW;
#ifdef UNICODE
typedef FAX_ROUTING_METHODW FAX_ROUTING_METHOD;
typedef PFAX_ROUTING_METHODW PFAX_ROUTING_METHOD;
#else
typedef FAX_ROUTING_METHODA FAX_ROUTING_METHOD;
typedef PFAX_ROUTING_METHODA PFAX_ROUTING_METHOD;
#endif // UNICODE


typedef struct _FAX_GLOBAL_ROUTING_INFOA
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               Priority;                   // priority of this device
    LPCSTR              Guid;                       // guid that identifies this routing method
    LPCSTR              FriendlyName;               // friendly name for this method
    LPCSTR              FunctionName;               // exported function name that identifies this method
    LPCSTR              ExtensionImageName;         // module (dll) name that implements this method
    LPCSTR              ExtensionFriendlyName;      // displayable string that identifies the extension
} FAX_GLOBAL_ROUTING_INFOA, *PFAX_GLOBAL_ROUTING_INFOA;
typedef struct _FAX_GLOBAL_ROUTING_INFOW
{
    DWORD               SizeOfStruct;               // size of this structure
    DWORD               Priority;                   // priority of this device
    LPCWSTR             Guid;                       // guid that identifies this routing method
    LPCWSTR             FriendlyName;               // friendly name for this method
    LPCWSTR             FunctionName;               // exported function name that identifies this method
    LPCWSTR             ExtensionImageName;         // module (dll) name that implements this method
    LPCWSTR             ExtensionFriendlyName;      // displayable string that identifies the extension
} FAX_GLOBAL_ROUTING_INFOW, *PFAX_GLOBAL_ROUTING_INFOW;
#ifdef UNICODE
typedef FAX_GLOBAL_ROUTING_INFOW FAX_GLOBAL_ROUTING_INFO;
typedef PFAX_GLOBAL_ROUTING_INFOW PFAX_GLOBAL_ROUTING_INFO;
#else
typedef FAX_GLOBAL_ROUTING_INFOA FAX_GLOBAL_ROUTING_INFO;
typedef PFAX_GLOBAL_ROUTING_INFOA PFAX_GLOBAL_ROUTING_INFO;
#endif // UNICODE


typedef struct _FAX_COVERPAGE_INFOA
{
    DWORD               SizeOfStruct;               // Size of this structure
    //
    // general
    //
    LPCSTR              CoverPageName;              // coverpage document name
    BOOL                UseServerCoverPage;         // coverpage exists on the fax server
    //
    // Recipient information
    //
    LPCSTR              RecName;                    //
    LPCSTR              RecFaxNumber;               //
    LPCSTR              RecCompany;                 //
    LPCSTR              RecStreetAddress;           //
    LPCSTR              RecCity;                    //
    LPCSTR              RecState;                   //
    LPCSTR              RecZip;                     //
    LPCSTR              RecCountry;                 //
    LPCSTR              RecTitle;                   //
    LPCSTR              RecDepartment;              //
    LPCSTR              RecOfficeLocation;          //
    LPCSTR              RecHomePhone;               //
    LPCSTR              RecOfficePhone;             //
    //
    // Sender information
    //
    LPCSTR              SdrName;                    //
    LPCSTR              SdrFaxNumber;               //
    LPCSTR              SdrCompany;                 //
    LPCSTR              SdrAddress;                 //
    LPCSTR              SdrTitle;                   //
    LPCSTR              SdrDepartment;              //
    LPCSTR              SdrOfficeLocation;          //
    LPCSTR              SdrHomePhone;               //
    LPCSTR              SdrOfficePhone;             //
    //
    // Misc information
    //
    LPCSTR              Note;                       //
    LPCSTR              Subject;                    //
    SYSTEMTIME          TimeSent;                   // Time the fax was sent
    DWORD               PageCount;                  // Number of pages
} FAX_COVERPAGE_INFOA, *PFAX_COVERPAGE_INFOA;
typedef struct _FAX_COVERPAGE_INFOW
{
    DWORD               SizeOfStruct;               // Size of this structure
    //
    // general
    //
    LPCWSTR             CoverPageName;              // coverpage document name
    BOOL                UseServerCoverPage;         // coverpage exists on the fax server
    //
    // Recipient information
    //
    LPCWSTR             RecName;                    //
    LPCWSTR             RecFaxNumber;               //
    LPCWSTR             RecCompany;                 //
    LPCWSTR             RecStreetAddress;           //
    LPCWSTR             RecCity;                    //
    LPCWSTR             RecState;                   //
    LPCWSTR             RecZip;                     //
    LPCWSTR             RecCountry;                 //
    LPCWSTR             RecTitle;                   //
    LPCWSTR             RecDepartment;              //
    LPCWSTR             RecOfficeLocation;          //
    LPCWSTR             RecHomePhone;               //
    LPCWSTR             RecOfficePhone;             //
    //
    // Sender information
    //
    LPCWSTR             SdrName;                    //
    LPCWSTR             SdrFaxNumber;               //
    LPCWSTR             SdrCompany;                 //
    LPCWSTR             SdrAddress;                 //
    LPCWSTR             SdrTitle;                   //
    LPCWSTR             SdrDepartment;              //
    LPCWSTR             SdrOfficeLocation;          //
    LPCWSTR             SdrHomePhone;               //
    LPCWSTR             SdrOfficePhone;             //
    //
    // Misc information
    //
    LPCWSTR             Note;                       //
    LPCWSTR             Subject;                    //
    SYSTEMTIME          TimeSent;                   // Time the fax was sent
    DWORD               PageCount;                  // Number of pages
} FAX_COVERPAGE_INFOW, *PFAX_COVERPAGE_INFOW;
#ifdef UNICODE
typedef FAX_COVERPAGE_INFOW FAX_COVERPAGE_INFO;
typedef PFAX_COVERPAGE_INFOW PFAX_COVERPAGE_INFO;
#else
typedef FAX_COVERPAGE_INFOA FAX_COVERPAGE_INFO;
typedef PFAX_COVERPAGE_INFOA PFAX_COVERPAGE_INFO;
#endif // UNICODE

typedef enum
{
    JSA_NOW                  = 0,   // Send now
    JSA_SPECIFIC_TIME,              // Send at specific time
    JSA_DISCOUNT_PERIOD             // Send at server configured discount period
} FAX_ENUM_JOB_SEND_ATTRIBUTES;


#ifndef _FAXAPIP_

typedef enum
{
    DRT_NONE                = 0x0000,       // Do not send receipt
    DRT_EMAIL               = 0x0001,       // Send receipt by email
    DRT_INBOX               = 0x0002        // send receipt to local inbox
} FAX_ENUM_DELIVERY_REPORT_TYPES;

#endif // _FAXAPIP_



typedef struct _FAX_JOB_PARAMA
{
    DWORD               SizeOfStruct;               // size of this structure
    LPCSTR              RecipientNumber;            // recipient fax number
    LPCSTR              RecipientName;              // recipient name
    LPCSTR              Tsid;                       // transmitter's id
    LPCSTR              SenderName;                 // sender name
    LPCSTR              SenderCompany;              // sender company
    LPCSTR              SenderDept;                 // sender department
    LPCSTR              BillingCode;                // billing code
    DWORD               ScheduleAction;             // when to schedule the fax, see JSA defines
    SYSTEMTIME          ScheduleTime;               // time to send the fax when JSA_SPECIFIC_TIME is used (must be local time)
    DWORD               DeliveryReportType;         // delivery report type, see DRT defines
    LPCSTR              DeliveryReportAddress;      // email address for delivery report (ndr or dr) thru MAPI / SMTP
    LPCSTR              DocumentName;               // document name (optional)
    HCALL               CallHandle;                 // optional call handle
    DWORD_PTR           Reserved[3];                // reserved for ms use only
} FAX_JOB_PARAMA, *PFAX_JOB_PARAMA;
typedef struct _FAX_JOB_PARAMW
{
    DWORD               SizeOfStruct;               // size of this structure
    LPCWSTR             RecipientNumber;            // recipient fax number
    LPCWSTR             RecipientName;              // recipient name
    LPCWSTR             Tsid;                       // transmitter's id
    LPCWSTR             SenderName;                 // sender name
    LPCWSTR             SenderCompany;              // sender company
    LPCWSTR             SenderDept;                 // sender department
    LPCWSTR             BillingCode;                // billing code
    DWORD               ScheduleAction;             // when to schedule the fax, see JSA defines
    SYSTEMTIME          ScheduleTime;               // time to send the fax when JSA_SPECIFIC_TIME is used (must be local time)
    DWORD               DeliveryReportType;         // delivery report type, see DRT defines
    LPCWSTR             DeliveryReportAddress;      // email address for delivery report (ndr or dr) thru MAPI / SMTP
    LPCWSTR             DocumentName;               // document name (optional)
    HCALL               CallHandle;                 // optional call handle
    DWORD_PTR           Reserved[3];                // reserved for ms use only
} FAX_JOB_PARAMW, *PFAX_JOB_PARAMW;
#ifdef UNICODE
typedef FAX_JOB_PARAMW FAX_JOB_PARAM;
typedef PFAX_JOB_PARAMW PFAX_JOB_PARAM;
#else
typedef FAX_JOB_PARAMA FAX_JOB_PARAM;
typedef PFAX_JOB_PARAMA PFAX_JOB_PARAM;
#endif // UNICODE

//
// Event Ids
//
// FEI_NEVENTS is the number of events
//

#define FEI_DIALING                 0x00000001
#define FEI_SENDING                 0x00000002
#define FEI_RECEIVING               0x00000003
#define FEI_COMPLETED               0x00000004
#define FEI_BUSY                    0x00000005
#define FEI_NO_ANSWER               0x00000006
#define FEI_BAD_ADDRESS             0x00000007
#define FEI_NO_DIAL_TONE            0x00000008
#define FEI_DISCONNECTED            0x00000009
#define FEI_FATAL_ERROR             0x0000000a
#define FEI_NOT_FAX_CALL            0x0000000b
#define FEI_CALL_DELAYED            0x0000000c
#define FEI_CALL_BLACKLISTED        0x0000000d
#define FEI_RINGING                 0x0000000e
#define FEI_ABORTING                0x0000000f
#define FEI_ROUTING                 0x00000010
#define FEI_MODEM_POWERED_ON        0x00000011
#define FEI_MODEM_POWERED_OFF       0x00000012
#define FEI_IDLE                    0x00000013
#define FEI_FAXSVC_ENDED            0x00000014
#define FEI_ANSWERED                0x00000015
#define FEI_JOB_QUEUED              0x00000016
#define FEI_DELETED                 0x00000017
#define FEI_INITIALIZING            0x00000018
#define FEI_LINE_UNAVAILABLE        0x00000019
#define FEI_HANDLED                 0x0000001a
#define FEI_FAXSVC_STARTED          0x0000001b

#define FEI_NEVENTS                 FEI_FAXSVC_STARTED

typedef struct _FAX_EVENTA
{
    DWORD               SizeOfStruct;               // Size of this structure
    FILETIME            TimeStamp;                  // Timestamp for when the event was generated
    DWORD               DeviceId;                   // Permanent line id
    DWORD               EventId;                    // Current event id
    DWORD               JobId;                      // Fax Job Id, 0xffffffff indicates inactive job
} FAX_EVENTA, *PFAX_EVENTA;
typedef struct _FAX_EVENTW
{
    DWORD               SizeOfStruct;               // Size of this structure
    FILETIME            TimeStamp;                  // Timestamp for when the event was generated
    DWORD               DeviceId;                   // Permanent line id
    DWORD               EventId;                    // Current event id
    DWORD               JobId;                      // Fax Job Id, 0xffffffff indicates inactive job
} FAX_EVENTW, *PFAX_EVENTW;
#ifdef UNICODE
typedef FAX_EVENTW FAX_EVENT;
typedef PFAX_EVENTW PFAX_EVENT;
#else
typedef FAX_EVENTA FAX_EVENT;
typedef PFAX_EVENTA PFAX_EVENT;
#endif // UNICODE


typedef struct _FAX_PRINT_INFOA
{
    DWORD               SizeOfStruct;               // Size of this structure
    LPCSTR              DocName;                    // Document name that appears in the spooler
    LPCSTR              RecipientName;              // Recipient name
    LPCSTR              RecipientNumber;            // Recipient fax number (non-canonical number)
    LPCSTR              SenderName;                 // Sender name
    LPCSTR              SenderCompany;              // Sender company (optional)
    LPCSTR              SenderDept;                 // Sender department
    LPCSTR              SenderBillingCode;          // Billing code
    LPCSTR              Reserved;                   // Reserved; must be NULL
    LPCSTR              DrEmailAddress;             // E.Mail address for delivery report
    LPCSTR              OutputFileName;             // for print to file, resulting file name
} FAX_PRINT_INFOA, *PFAX_PRINT_INFOA;
typedef struct _FAX_PRINT_INFOW
{
    DWORD               SizeOfStruct;               // Size of this structure
    LPCWSTR             DocName;                    // Document name that appears in the spooler
    LPCWSTR             RecipientName;              // Recipient name
    LPCWSTR             RecipientNumber;            // Recipient fax number (non-canonical number)
    LPCWSTR             SenderName;                 // Sender name
    LPCWSTR             SenderCompany;              // Sender company (optional)
    LPCWSTR             SenderDept;                 // Sender department
    LPCWSTR             SenderBillingCode;          // Billing code
    LPCWSTR             Reserved;                   // Reserved; must be NULL
    LPCWSTR             DrEmailAddress;             // E.Mail address for delivery report
    LPCWSTR             OutputFileName;             // for print to file, resulting file name
} FAX_PRINT_INFOW, *PFAX_PRINT_INFOW;
#ifdef UNICODE
typedef FAX_PRINT_INFOW FAX_PRINT_INFO;
typedef PFAX_PRINT_INFOW PFAX_PRINT_INFO;
#else
typedef FAX_PRINT_INFOA FAX_PRINT_INFO;
typedef PFAX_PRINT_INFOA PFAX_PRINT_INFO;
#endif // UNICODE


typedef struct _FAX_CONTEXT_INFOA
{
    DWORD               SizeOfStruct;                           // Size of this structure
    HDC                 hDC;                                    // Device Context
    CHAR                ServerName[MAX_COMPUTERNAME_LENGTH+1];  // Server name
} FAX_CONTEXT_INFOA, *PFAX_CONTEXT_INFOA;
typedef struct _FAX_CONTEXT_INFOW
{
    DWORD               SizeOfStruct;                           // Size of this structure
    HDC                 hDC;                                    // Device Context
    WCHAR               ServerName[MAX_COMPUTERNAME_LENGTH+1];  // Server name
} FAX_CONTEXT_INFOW, *PFAX_CONTEXT_INFOW;
#ifdef UNICODE
typedef FAX_CONTEXT_INFOW FAX_CONTEXT_INFO;
typedef PFAX_CONTEXT_INFOW PFAX_CONTEXT_INFO;
#else
typedef FAX_CONTEXT_INFOA FAX_CONTEXT_INFO;
typedef PFAX_CONTEXT_INFOA PFAX_CONTEXT_INFO;
#endif // UNICODE


//
// prototypes
//

WINFAXAPI
BOOL
WINAPI
FaxConnectFaxServerA(
    IN  LPCSTR MachineName OPTIONAL,
    OUT LPHANDLE FaxHandle
    );
WINFAXAPI
BOOL
WINAPI
FaxConnectFaxServerW(
    IN  LPCWSTR MachineName OPTIONAL,
    OUT LPHANDLE FaxHandle
    );
#ifdef UNICODE
#define FaxConnectFaxServer  FaxConnectFaxServerW
#else
#define FaxConnectFaxServer  FaxConnectFaxServerA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXCONNECTFAXSERVERA)(
    IN  LPCSTR MachineName OPTIONAL,
    OUT LPHANDLE FaxHandle
    );
typedef BOOL
(WINAPI *PFAXCONNECTFAXSERVERW)(
    IN  LPCWSTR MachineName OPTIONAL,
    OUT LPHANDLE FaxHandle
    );
#ifdef UNICODE
#define PFAXCONNECTFAXSERVER  PFAXCONNECTFAXSERVERW
#else
#define PFAXCONNECTFAXSERVER  PFAXCONNECTFAXSERVERA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxClose(
    IN HANDLE FaxHandle
    );

typedef BOOL
(WINAPI *PFAXCLOSE)(
    IN HANDLE FaxHandle
    );

typedef enum
{
    PORT_OPEN_QUERY     = 1,
    PORT_OPEN_MODIFY
} FAX_ENUM_PORT_OPEN_TYPE;

WINFAXAPI
BOOL
WINAPI
FaxOpenPort(
    IN  HANDLE FaxHandle,
    IN  DWORD DeviceId,
    IN  DWORD Flags,
    OUT LPHANDLE FaxPortHandle
    );

typedef BOOL
(WINAPI *PFAXOPENPORT)(
    IN  HANDLE FaxHandle,
    IN  DWORD DeviceId,
    IN  DWORD Flags,
    OUT LPHANDLE FaxPortHandle
    );

WINFAXAPI
BOOL
WINAPI
FaxCompleteJobParamsA(
    IN OUT PFAX_JOB_PARAMA *JobParams,
    IN OUT PFAX_COVERPAGE_INFOA *CoverpageInfo
    );
WINFAXAPI
BOOL
WINAPI
FaxCompleteJobParamsW(
    IN OUT PFAX_JOB_PARAMW *JobParams,
    IN OUT PFAX_COVERPAGE_INFOW *CoverpageInfo
    );
#ifdef UNICODE
#define FaxCompleteJobParams  FaxCompleteJobParamsW
#else
#define FaxCompleteJobParams  FaxCompleteJobParamsA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXCOMPLETEJOBPARAMSA)(
    IN OUT PFAX_JOB_PARAMA *JobParams,
    IN OUT PFAX_COVERPAGE_INFOA *CoverpageInfo
    );
typedef BOOL
(WINAPI *PFAXCOMPLETEJOBPARAMSW)(
    IN OUT PFAX_JOB_PARAMW *JobParams,
    IN OUT PFAX_COVERPAGE_INFOW *CoverpageInfo
    );
#ifdef UNICODE
#define PFAXCOMPLETEJOBPARAMS  PFAXCOMPLETEJOBPARAMSW
#else
#define PFAXCOMPLETEJOBPARAMS  PFAXCOMPLETEJOBPARAMSA
#endif // !UNICODE



WINFAXAPI
BOOL
WINAPI
FaxSendDocumentA(
    IN HANDLE FaxHandle,
    IN LPCSTR FileName,
    IN PFAX_JOB_PARAMA JobParams,
    IN const FAX_COVERPAGE_INFOA *CoverpageInfo, OPTIONAL
    OUT LPDWORD FaxJobId OPTIONAL
    );
WINFAXAPI
BOOL
WINAPI
FaxSendDocumentW(
    IN HANDLE FaxHandle,
    IN LPCWSTR FileName,
    IN PFAX_JOB_PARAMW JobParams,
    IN const FAX_COVERPAGE_INFOW *CoverpageInfo, OPTIONAL
    OUT LPDWORD FaxJobId OPTIONAL
    );
#ifdef UNICODE
#define FaxSendDocument  FaxSendDocumentW
#else
#define FaxSendDocument  FaxSendDocumentA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSENDDOCUMENTA)(
    IN HANDLE FaxHandle,
    IN LPCSTR FileName,
    IN PFAX_JOB_PARAMA JobParams,
    IN const FAX_COVERPAGE_INFOA *CoverpageInfo, OPTIONAL
    OUT LPDWORD FaxJobId OPTIONAL
    );
typedef BOOL
(WINAPI *PFAXSENDDOCUMENTW)(
    IN HANDLE FaxHandle,
    IN LPCWSTR FileName,
    IN PFAX_JOB_PARAMW JobParams,
    IN const FAX_COVERPAGE_INFOW *CoverpageInfo, OPTIONAL
    OUT LPDWORD FaxJobId OPTIONAL
    );
#ifdef UNICODE
#define PFAXSENDDOCUMENT  PFAXSENDDOCUMENTW
#else
#define PFAXSENDDOCUMENT  PFAXSENDDOCUMENTA
#endif // !UNICODE

typedef BOOL
(CALLBACK *PFAX_RECIPIENT_CALLBACKA)(
    IN HANDLE FaxHandle,
    IN DWORD RecipientNumber,
    IN LPVOID Context,
    IN OUT PFAX_JOB_PARAMA JobParams,
    IN OUT PFAX_COVERPAGE_INFOA CoverpageInfo OPTIONAL
    );
typedef BOOL
(CALLBACK *PFAX_RECIPIENT_CALLBACKW)(
    IN HANDLE FaxHandle,
    IN DWORD RecipientNumber,
    IN LPVOID Context,
    IN OUT PFAX_JOB_PARAMW JobParams,
    IN OUT PFAX_COVERPAGE_INFOW CoverpageInfo OPTIONAL
    );
#ifdef UNICODE
#define PFAX_RECIPIENT_CALLBACK  PFAX_RECIPIENT_CALLBACKW
#else
#define PFAX_RECIPIENT_CALLBACK  PFAX_RECIPIENT_CALLBACKA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxSendDocumentForBroadcastA(
    IN HANDLE FaxHandle,
    IN LPCSTR FileName,
    OUT LPDWORD FaxJobId,
    IN PFAX_RECIPIENT_CALLBACKA FaxRecipientCallback,
    IN LPVOID Context
    );
WINFAXAPI
BOOL
WINAPI
FaxSendDocumentForBroadcastW(
    IN HANDLE FaxHandle,
    IN LPCWSTR FileName,
    OUT LPDWORD FaxJobId,
    IN PFAX_RECIPIENT_CALLBACKW FaxRecipientCallback,
    IN LPVOID Context
    );
#ifdef UNICODE
#define FaxSendDocumentForBroadcast  FaxSendDocumentForBroadcastW
#else
#define FaxSendDocumentForBroadcast  FaxSendDocumentForBroadcastA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSENDDOCUMENTFORBROADCASTA)(
    IN  HANDLE FaxHandle,
    IN  LPCSTR FileName,
    OUT LPDWORD FaxJobId,
    IN  PFAX_RECIPIENT_CALLBACKA FaxRecipientCallback,
    IN  LPVOID Context
    );
typedef BOOL
(WINAPI *PFAXSENDDOCUMENTFORBROADCASTW)(
    IN  HANDLE FaxHandle,
    IN  LPCWSTR FileName,
    OUT LPDWORD FaxJobId,
    IN  PFAX_RECIPIENT_CALLBACKW FaxRecipientCallback,
    IN  LPVOID Context
    );
#ifdef UNICODE
#define PFAXSENDDOCUMENTFORBROADCAST  PFAXSENDDOCUMENTFORBROADCASTW
#else
#define PFAXSENDDOCUMENTFORBROADCAST  PFAXSENDDOCUMENTFORBROADCASTA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxEnumJobsA(
    IN  HANDLE FaxHandle,
    OUT PFAX_JOB_ENTRYA *JobEntry,
    OUT LPDWORD JobsReturned
    );
WINFAXAPI
BOOL
WINAPI
FaxEnumJobsW(
    IN  HANDLE FaxHandle,
    OUT PFAX_JOB_ENTRYW *JobEntry,
    OUT LPDWORD JobsReturned
    );
#ifdef UNICODE
#define FaxEnumJobs  FaxEnumJobsW
#else
#define FaxEnumJobs  FaxEnumJobsA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXENUMJOBSA)(
    IN  HANDLE FaxHandle,
    OUT PFAX_JOB_ENTRYA *JobEntry,
    OUT LPDWORD JobsReturned
    );
typedef BOOL
(WINAPI *PFAXENUMJOBSW)(
    IN  HANDLE FaxHandle,
    OUT PFAX_JOB_ENTRYW *JobEntry,
    OUT LPDWORD JobsReturned
    );
#ifdef UNICODE
#define PFAXENUMJOBS  PFAXENUMJOBSW
#else
#define PFAXENUMJOBS  PFAXENUMJOBSA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxGetJobA(
   IN  HANDLE FaxHandle,
   IN  DWORD JobId,
   OUT PFAX_JOB_ENTRYA *JobEntry
   );
WINFAXAPI
BOOL
WINAPI
FaxGetJobW(
   IN  HANDLE FaxHandle,
   IN  DWORD JobId,
   OUT PFAX_JOB_ENTRYW *JobEntry
   );
#ifdef UNICODE
#define FaxGetJob  FaxGetJobW
#else
#define FaxGetJob  FaxGetJobA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXGETJOBA)(
    IN  HANDLE FaxHandle,
    IN  DWORD JobId,
    OUT PFAX_JOB_ENTRYA *JobEntry
    );
typedef BOOL
(WINAPI *PFAXGETJOBW)(
    IN  HANDLE FaxHandle,
    IN  DWORD JobId,
    OUT PFAX_JOB_ENTRYW *JobEntry
    );
#ifdef UNICODE
#define PFAXGETJOB  PFAXGETJOBW
#else
#define PFAXGETJOB  PFAXGETJOBA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxSetJobA(
   IN HANDLE FaxHandle,
   IN DWORD JobId,
   IN DWORD Command,
   IN const FAX_JOB_ENTRYA *JobEntry
   );
WINFAXAPI
BOOL
WINAPI
FaxSetJobW(
   IN HANDLE FaxHandle,
   IN DWORD JobId,
   IN DWORD Command,
   IN const FAX_JOB_ENTRYW *JobEntry
   );
#ifdef UNICODE
#define FaxSetJob  FaxSetJobW
#else
#define FaxSetJob  FaxSetJobA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSETJOBA)(
    IN HANDLE FaxHandle,
    IN DWORD JobId,
    IN DWORD Command,
    IN const FAX_JOB_ENTRYA *JobEntry
    );
typedef BOOL
(WINAPI *PFAXSETJOBW)(
    IN HANDLE FaxHandle,
    IN DWORD JobId,
    IN DWORD Command,
    IN const FAX_JOB_ENTRYW *JobEntry
    );
#ifdef UNICODE
#define PFAXSETJOB  PFAXSETJOBW
#else
#define PFAXSETJOB  PFAXSETJOBA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxGetPageData(
   IN HANDLE FaxHandle,
   IN DWORD JobId,
   OUT LPBYTE *Buffer,
   OUT LPDWORD BufferSize,
   OUT LPDWORD ImageWidth,
   OUT LPDWORD ImageHeight
   );

typedef BOOL
(WINAPI *PFAXGETPAGEDATA)(
   IN HANDLE FaxHandle,
   IN DWORD JobId,
   OUT LPBYTE *Buffer,
   OUT LPDWORD BufferSize,
   OUT LPDWORD ImageWidth,
   OUT LPDWORD ImageHeight
   );

WINFAXAPI
BOOL
WINAPI
FaxGetDeviceStatusA(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_DEVICE_STATUSA *DeviceStatus
    );
WINFAXAPI
BOOL
WINAPI
FaxGetDeviceStatusW(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_DEVICE_STATUSW *DeviceStatus
    );
#ifdef UNICODE
#define FaxGetDeviceStatus  FaxGetDeviceStatusW
#else
#define FaxGetDeviceStatus  FaxGetDeviceStatusA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXGETDEVICESTATUSA)(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_DEVICE_STATUSA *DeviceStatus
    );
typedef BOOL
(WINAPI *PFAXGETDEVICESTATUSW)(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_DEVICE_STATUSW *DeviceStatus
    );
#ifdef UNICODE
#define PFAXGETDEVICESTATUS  PFAXGETDEVICESTATUSW
#else
#define PFAXGETDEVICESTATUS  PFAXGETDEVICESTATUSA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxAbort(
    IN HANDLE FaxHandle,
    IN DWORD JobId
    );

typedef BOOL
(WINAPI *PFAXABORT)(
    IN HANDLE FaxHandle,
    IN DWORD JobId
    );

WINFAXAPI
BOOL
WINAPI
FaxGetConfigurationA(
    IN  HANDLE FaxHandle,
    OUT PFAX_CONFIGURATIONA *FaxConfig
    );
WINFAXAPI
BOOL
WINAPI
FaxGetConfigurationW(
    IN  HANDLE FaxHandle,
    OUT PFAX_CONFIGURATIONW *FaxConfig
    );
#ifdef UNICODE
#define FaxGetConfiguration  FaxGetConfigurationW
#else
#define FaxGetConfiguration  FaxGetConfigurationA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXGETCONFIGURATIONA)(
    IN  HANDLE FaxHandle,
    OUT PFAX_CONFIGURATIONA *FaxConfig
    );
typedef BOOL
(WINAPI *PFAXGETCONFIGURATIONW)(
    IN  HANDLE FaxHandle,
    OUT PFAX_CONFIGURATIONW *FaxConfig
    );
#ifdef UNICODE
#define PFAXGETCONFIGURATION  PFAXGETCONFIGURATIONW
#else
#define PFAXGETCONFIGURATION  PFAXGETCONFIGURATIONA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxSetConfigurationA(
    IN  HANDLE FaxHandle,
    IN  const FAX_CONFIGURATIONA *FaxConfig
    );
WINFAXAPI
BOOL
WINAPI
FaxSetConfigurationW(
    IN  HANDLE FaxHandle,
    IN  const FAX_CONFIGURATIONW *FaxConfig
    );
#ifdef UNICODE
#define FaxSetConfiguration  FaxSetConfigurationW
#else
#define FaxSetConfiguration  FaxSetConfigurationA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSETCONFIGURATIONA)(
    IN  HANDLE FaxHandle,
    IN  const FAX_CONFIGURATIONA *FaxConfig
    );
typedef BOOL
(WINAPI *PFAXSETCONFIGURATIONW)(
    IN  HANDLE FaxHandle,
    IN  const FAX_CONFIGURATIONW *FaxConfig
    );
#ifdef UNICODE
#define PFAXSETCONFIGURATION  PFAXSETCONFIGURATIONW
#else
#define PFAXSETCONFIGURATION  PFAXSETCONFIGURATIONA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxGetLoggingCategoriesA(
    IN  HANDLE FaxHandle,
    OUT PFAX_LOG_CATEGORYA *Categories,
    OUT LPDWORD NumberCategories
    );
WINFAXAPI
BOOL
WINAPI
FaxGetLoggingCategoriesW(
    IN  HANDLE FaxHandle,
    OUT PFAX_LOG_CATEGORYW *Categories,
    OUT LPDWORD NumberCategories
    );
#ifdef UNICODE
#define FaxGetLoggingCategories  FaxGetLoggingCategoriesW
#else
#define FaxGetLoggingCategories  FaxGetLoggingCategoriesA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXGETLOGGINGCATEGORIESA)(
    IN  HANDLE FaxHandle,
    OUT PFAX_LOG_CATEGORYA *Categories,
    OUT LPDWORD NumberCategories
    );
typedef BOOL
(WINAPI *PFAXGETLOGGINGCATEGORIESW)(
    IN  HANDLE FaxHandle,
    OUT PFAX_LOG_CATEGORYW *Categories,
    OUT LPDWORD NumberCategories
    );
#ifdef UNICODE
#define PFAXGETLOGGINGCATEGORIES  PFAXGETLOGGINGCATEGORIESW
#else
#define PFAXGETLOGGINGCATEGORIES  PFAXGETLOGGINGCATEGORIESA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxSetLoggingCategoriesA(
    IN  HANDLE FaxHandle,
    IN  const FAX_LOG_CATEGORYA *Categories,
    IN  DWORD NumberCategories
    );
WINFAXAPI
BOOL
WINAPI
FaxSetLoggingCategoriesW(
    IN  HANDLE FaxHandle,
    IN  const FAX_LOG_CATEGORYW *Categories,
    IN  DWORD NumberCategories
    );
#ifdef UNICODE
#define FaxSetLoggingCategories  FaxSetLoggingCategoriesW
#else
#define FaxSetLoggingCategories  FaxSetLoggingCategoriesA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSETLOGGINGCATEGORIESA)(
    IN  HANDLE FaxHandle,
    IN  const FAX_LOG_CATEGORYA *Categories,
    IN  DWORD NumberCategories
    );
typedef BOOL
(WINAPI *PFAXSETLOGGINGCATEGORIESW)(
    IN  HANDLE FaxHandle,
    IN  const FAX_LOG_CATEGORYW *Categories,
    IN  DWORD NumberCategories
    );
#ifdef UNICODE
#define PFAXSETLOGGINGCATEGORIES  PFAXSETLOGGINGCATEGORIESW
#else
#define PFAXSETLOGGINGCATEGORIES  PFAXSETLOGGINGCATEGORIESA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxEnumPortsA(
    IN  HANDLE FaxHandle,
    OUT PFAX_PORT_INFOA *PortInfo,
    OUT LPDWORD PortsReturned
    );
WINFAXAPI
BOOL
WINAPI
FaxEnumPortsW(
    IN  HANDLE FaxHandle,
    OUT PFAX_PORT_INFOW *PortInfo,
    OUT LPDWORD PortsReturned
    );
#ifdef UNICODE
#define FaxEnumPorts  FaxEnumPortsW
#else
#define FaxEnumPorts  FaxEnumPortsA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXENUMPORTSA)(
    IN  HANDLE FaxHandle,
    OUT PFAX_PORT_INFOA *PortInfo,
    OUT LPDWORD PortsReturned
    );
typedef BOOL
(WINAPI *PFAXENUMPORTSW)(
    IN  HANDLE FaxHandle,
    OUT PFAX_PORT_INFOW *PortInfo,
    OUT LPDWORD PortsReturned
    );
#ifdef UNICODE
#define PFAXENUMPORTS  PFAXENUMPORTSW
#else
#define PFAXENUMPORTS  PFAXENUMPORTSA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxGetPortA(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_PORT_INFOA *PortInfo
    );
WINFAXAPI
BOOL
WINAPI
FaxGetPortW(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_PORT_INFOW *PortInfo
    );
#ifdef UNICODE
#define FaxGetPort  FaxGetPortW
#else
#define FaxGetPort  FaxGetPortA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXGETPORTA)(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_PORT_INFOA *PortInfo
    );
typedef BOOL
(WINAPI *PFAXGETPORTW)(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_PORT_INFOW *PortInfo
    );
#ifdef UNICODE
#define PFAXGETPORT  PFAXGETPORTW
#else
#define PFAXGETPORT  PFAXGETPORTA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxSetPortA(
    IN  HANDLE FaxPortHandle,
    IN  const FAX_PORT_INFOA *PortInfo
    );
WINFAXAPI
BOOL
WINAPI
FaxSetPortW(
    IN  HANDLE FaxPortHandle,
    IN  const FAX_PORT_INFOW *PortInfo
    );
#ifdef UNICODE
#define FaxSetPort  FaxSetPortW
#else
#define FaxSetPort  FaxSetPortA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSETPORTA)(
    IN  HANDLE FaxPortHandle,
    IN  const FAX_PORT_INFOA *PortInfo
    );
typedef BOOL
(WINAPI *PFAXSETPORTW)(
    IN  HANDLE FaxPortHandle,
    IN  const FAX_PORT_INFOW *PortInfo
    );
#ifdef UNICODE
#define PFAXSETPORT  PFAXSETPORTW
#else
#define PFAXSETPORT  PFAXSETPORTA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxEnumRoutingMethodsA(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_ROUTING_METHODA *RoutingMethod,
    OUT LPDWORD MethodsReturned
    );
WINFAXAPI
BOOL
WINAPI
FaxEnumRoutingMethodsW(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_ROUTING_METHODW *RoutingMethod,
    OUT LPDWORD MethodsReturned
    );
#ifdef UNICODE
#define FaxEnumRoutingMethods  FaxEnumRoutingMethodsW
#else
#define FaxEnumRoutingMethods  FaxEnumRoutingMethodsA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXENUMROUTINGMETHODSA)(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_ROUTING_METHODA *RoutingMethod,
    OUT LPDWORD MethodsReturned
    );
typedef BOOL
(WINAPI *PFAXENUMROUTINGMETHODSW)(
    IN  HANDLE FaxPortHandle,
    OUT PFAX_ROUTING_METHODW *RoutingMethod,
    OUT LPDWORD MethodsReturned
    );
#ifdef UNICODE
#define PFAXENUMROUTINGMETHODS  PFAXENUMROUTINGMETHODSW
#else
#define PFAXENUMROUTINGMETHODS  PFAXENUMROUTINGMETHODSA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxEnableRoutingMethodA(
    IN  HANDLE FaxPortHandle,
    IN  LPCSTR RoutingGuid,
    IN  BOOL Enabled
    );
WINFAXAPI
BOOL
WINAPI
FaxEnableRoutingMethodW(
    IN  HANDLE FaxPortHandle,
    IN  LPCWSTR RoutingGuid,
    IN  BOOL Enabled
    );
#ifdef UNICODE
#define FaxEnableRoutingMethod  FaxEnableRoutingMethodW
#else
#define FaxEnableRoutingMethod  FaxEnableRoutingMethodA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXENABLEROUTINGMETHODA)(
    IN  HANDLE FaxPortHandle,
    IN  LPCSTR RoutingGuid,
    IN  BOOL Enabled
    );
typedef BOOL
(WINAPI *PFAXENABLEROUTINGMETHODW)(
    IN  HANDLE FaxPortHandle,
    IN  LPCWSTR RoutingGuid,
    IN  BOOL Enabled
    );
#ifdef UNICODE
#define PFAXENABLEROUTINGMETHOD  PFAXENABLEROUTINGMETHODW
#else
#define PFAXENABLEROUTINGMETHOD  PFAXENABLEROUTINGMETHODA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxEnumGlobalRoutingInfoA(
    IN  HANDLE FaxHandle,
    OUT PFAX_GLOBAL_ROUTING_INFOA *RoutingInfo,
    OUT LPDWORD MethodsReturned
    );
WINFAXAPI
BOOL
WINAPI
FaxEnumGlobalRoutingInfoW(
    IN  HANDLE FaxHandle,
    OUT PFAX_GLOBAL_ROUTING_INFOW *RoutingInfo,
    OUT LPDWORD MethodsReturned
    );
#ifdef UNICODE
#define FaxEnumGlobalRoutingInfo  FaxEnumGlobalRoutingInfoW
#else
#define FaxEnumGlobalRoutingInfo  FaxEnumGlobalRoutingInfoA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXENUMGLOBALROUTINGINFOA)(
    IN  HANDLE FaxHandle,
    OUT PFAX_GLOBAL_ROUTING_INFOA *RoutingInfo,
    OUT LPDWORD MethodsReturned
    );
typedef BOOL
(WINAPI *PFAXENUMGLOBALROUTINGINFOW)(
    IN  HANDLE FaxHandle,
    OUT PFAX_GLOBAL_ROUTING_INFOW *RoutingInfo,
    OUT LPDWORD MethodsReturned
    );
#ifdef UNICODE
#define PFAXENUMGLOBALROUTINGINFO  PFAXENUMGLOBALROUTINGINFOW
#else
#define PFAXENUMGLOBALROUTINGINFO  PFAXENUMGLOBALROUTINGINFOA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxSetGlobalRoutingInfoA(
    IN  HANDLE FaxHandle,
    IN  const FAX_GLOBAL_ROUTING_INFOA *RoutingInfo
    );
WINFAXAPI
BOOL
WINAPI
FaxSetGlobalRoutingInfoW(
    IN  HANDLE FaxHandle,
    IN  const FAX_GLOBAL_ROUTING_INFOW *RoutingInfo
    );
#ifdef UNICODE
#define FaxSetGlobalRoutingInfo  FaxSetGlobalRoutingInfoW
#else
#define FaxSetGlobalRoutingInfo  FaxSetGlobalRoutingInfoA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSETGLOBALROUTINGINFOA)(
    IN  HANDLE FaxPortHandle,
    IN  const FAX_GLOBAL_ROUTING_INFOA *RoutingInfo
    );
typedef BOOL
(WINAPI *PFAXSETGLOBALROUTINGINFOW)(
    IN  HANDLE FaxPortHandle,
    IN  const FAX_GLOBAL_ROUTING_INFOW *RoutingInfo
    );
#ifdef UNICODE
#define PFAXSETGLOBALROUTINGINFO  PFAXSETGLOBALROUTINGINFOW
#else
#define PFAXSETGLOBALROUTINGINFO  PFAXSETGLOBALROUTINGINFOA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxGetRoutingInfoA(
    IN  HANDLE FaxPortHandle,
    IN  LPCSTR RoutingGuid,
    OUT LPBYTE *RoutingInfoBuffer,
    OUT LPDWORD RoutingInfoBufferSize
    );
WINFAXAPI
BOOL
WINAPI
FaxGetRoutingInfoW(
    IN  HANDLE FaxPortHandle,
    IN  LPCWSTR RoutingGuid,
    OUT LPBYTE *RoutingInfoBuffer,
    OUT LPDWORD RoutingInfoBufferSize
    );
#ifdef UNICODE
#define FaxGetRoutingInfo  FaxGetRoutingInfoW
#else
#define FaxGetRoutingInfo  FaxGetRoutingInfoA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXGETROUTINGINFOA)(
    IN  HANDLE FaxPortHandle,
    IN  LPCSTR RoutingGuid,
    OUT LPBYTE *RoutingInfoBuffer,
    OUT LPDWORD RoutingInfoBufferSize
    );
typedef BOOL
(WINAPI *PFAXGETROUTINGINFOW)(
    IN  HANDLE FaxPortHandle,
    IN  LPCWSTR RoutingGuid,
    OUT LPBYTE *RoutingInfoBuffer,
    OUT LPDWORD RoutingInfoBufferSize
    );
#ifdef UNICODE
#define PFAXGETROUTINGINFO  PFAXGETROUTINGINFOW
#else
#define PFAXGETROUTINGINFO  PFAXGETROUTINGINFOA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxSetRoutingInfoA(
    IN  HANDLE FaxPortHandle,
    IN  LPCSTR RoutingGuid,
    IN  const BYTE *RoutingInfoBuffer,
    IN  DWORD RoutingInfoBufferSize
    );
WINFAXAPI
BOOL
WINAPI
FaxSetRoutingInfoW(
    IN  HANDLE FaxPortHandle,
    IN  LPCWSTR RoutingGuid,
    IN  const BYTE *RoutingInfoBuffer,
    IN  DWORD RoutingInfoBufferSize
    );
#ifdef UNICODE
#define FaxSetRoutingInfo  FaxSetRoutingInfoW
#else
#define FaxSetRoutingInfo  FaxSetRoutingInfoA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSETROUTINGINFOA)(
    IN  HANDLE FaxPortHandle,
    IN  LPCSTR RoutingGuid,
    IN  const BYTE *RoutingInfoBuffer,
    IN  DWORD RoutingInfoBufferSize
    );
typedef BOOL
(WINAPI *PFAXSETROUTINGINFOW)(
    IN  HANDLE FaxPortHandle,
    IN  LPCWSTR RoutingGuid,
    IN  const BYTE *RoutingInfoBuffer,
    IN  DWORD RoutingInfoBufferSize
    );
#ifdef UNICODE
#define PFAXSETROUTINGINFO  PFAXSETROUTINGINFOW
#else
#define PFAXSETROUTINGINFO  PFAXSETROUTINGINFOA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxInitializeEventQueue(
    IN HANDLE FaxHandle,
    IN HANDLE CompletionPort,
    IN ULONG_PTR CompletionKey,
    IN HWND hWnd,
    IN UINT MessageStart
    );

typedef BOOL
(WINAPI *PFAXINITIALIZEEVENTQUEUE)(
    IN HANDLE FaxHandle,
    IN HANDLE CompletionPort,
    IN ULONG_PTR CompletionKey,
    IN HWND hWnd,
    IN UINT MessageStart
    );

WINFAXAPI
VOID
WINAPI
FaxFreeBuffer(
    LPVOID Buffer
    );

typedef VOID
(WINAPI *PFAXFREEBUFFER)(
    LPVOID Buffer
    );

WINFAXAPI
BOOL
WINAPI
FaxStartPrintJobA(
    IN  LPCSTR PrinterName,
    IN  const FAX_PRINT_INFOA *PrintInfo,
    OUT LPDWORD FaxJobId,
    OUT PFAX_CONTEXT_INFOA FaxContextInfo
    );
WINFAXAPI
BOOL
WINAPI
FaxStartPrintJobW(
    IN  LPCWSTR PrinterName,
    IN  const FAX_PRINT_INFOW *PrintInfo,
    OUT LPDWORD FaxJobId,
    OUT PFAX_CONTEXT_INFOW FaxContextInfo
    );
#ifdef UNICODE
#define FaxStartPrintJob  FaxStartPrintJobW
#else
#define FaxStartPrintJob  FaxStartPrintJobA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXSTARTPRINTJOBA)(
    IN  LPCSTR PrinterName,
    IN  const FAX_PRINT_INFOA *PrintInfo,
    OUT LPDWORD FaxJobId,
    OUT PFAX_CONTEXT_INFOA FaxContextInfo
    );
typedef BOOL
(WINAPI *PFAXSTARTPRINTJOBW)(
    IN  LPCWSTR PrinterName,
    IN  const FAX_PRINT_INFOW *PrintInfo,
    OUT LPDWORD FaxJobId,
    OUT PFAX_CONTEXT_INFOW FaxContextInfo
    );
#ifdef UNICODE
#define PFAXSTARTPRINTJOB  PFAXSTARTPRINTJOBW
#else
#define PFAXSTARTPRINTJOB  PFAXSTARTPRINTJOBA
#endif // !UNICODE

WINFAXAPI
BOOL
WINAPI
FaxPrintCoverPageA(
    IN const FAX_CONTEXT_INFOA *FaxContextInfo,
    IN const FAX_COVERPAGE_INFOA *CoverPageInfo
    );
WINFAXAPI
BOOL
WINAPI
FaxPrintCoverPageW(
    IN const FAX_CONTEXT_INFOW *FaxContextInfo,
    IN const FAX_COVERPAGE_INFOW *CoverPageInfo
    );
#ifdef UNICODE
#define FaxPrintCoverPage  FaxPrintCoverPageW
#else
#define FaxPrintCoverPage  FaxPrintCoverPageA
#endif // !UNICODE

typedef BOOL
(WINAPI *PFAXPRINTCOVERPAGEA)(
    IN const FAX_CONTEXT_INFOA *FaxContextInfo,
    IN const FAX_COVERPAGE_INFOA *CoverPageInfo
    );
typedef BOOL
(WINAPI *PFAXPRINTCOVERPAGEW)(
    IN const FAX_CONTEXT_INFOW *FaxContextInfo,
    IN const FAX_COVERPAGE_INFOW *CoverPageInfo
    );
#ifdef UNICODE
#define PFAXPRINTCOVERPAGE  PFAXPRINTCOVERPAGEW
#else
#define PFAXPRINTCOVERPAGE  PFAXPRINTCOVERPAGEA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxRegisterServiceProviderW(
    IN LPCWSTR DeviceProvider,
    IN LPCWSTR FriendlyName,
    IN LPCWSTR ImageName,
    IN LPCWSTR TspName
    );

#define FaxRegisterServiceProvider  FaxRegisterServiceProviderW

typedef BOOL
(WINAPI *PFAXREGISTERSERVICEPROVIDERW)(
    IN LPCWSTR DeviceProvider,
    IN LPCWSTR FriendlyName,
    IN LPCWSTR ImageName,
    IN LPCWSTR TspName
    );

#define PFAXREGISTERSERVICEPROVIDER PFAXREGISTERSERVICEPROVIDERW


WINFAXAPI
BOOL
WINAPI
FaxUnregisterServiceProviderW(
    IN LPCWSTR DeviceProvider
    );

#define FaxUnregisterServiceProvider  FaxUnregisterServiceProviderW

typedef BOOL
(WINAPI *PFAXUNREGISTERSERVICEPROVIDERW)(
    IN LPCWSTR DeviceProvider
    );

#define PFAXUNREGISTERSERVICEPROVIDER PFAXUNREGISTERSERVICEPROVIDERW

typedef BOOL
(CALLBACK *PFAX_ROUTING_INSTALLATION_CALLBACKW)(
    IN HANDLE FaxHandle,
    IN LPVOID Context,
    IN OUT LPWSTR MethodName,
    IN OUT LPWSTR FriendlyName,
    IN OUT LPWSTR FunctionName,
    IN OUT LPWSTR Guid
    );

#define PFAX_ROUTING_INSTALLATION_CALLBACK PFAX_ROUTING_INSTALLATION_CALLBACKW


WINFAXAPI
BOOL
WINAPI
FaxRegisterRoutingExtensionW(
    IN HANDLE  FaxHandle,
    IN LPCWSTR ExtensionName,
    IN LPCWSTR FriendlyName,
    IN LPCWSTR ImageName,
    IN PFAX_ROUTING_INSTALLATION_CALLBACKW CallBack,
    IN LPVOID Context
    );

#define FaxRegisterRoutingExtension FaxRegisterRoutingExtensionW


typedef BOOL
(WINAPI *PFAXREGISTERROUTINGEXTENSIONW)(
    IN HANDLE  FaxHandle,
    IN LPCWSTR ExtensionName,
    IN LPCWSTR FriendlyName,
    IN LPCWSTR ImageName,
    IN PFAX_ROUTING_INSTALLATION_CALLBACKW CallBack,
    IN LPVOID Context
    );

#define PFAXREGISTERROUTINGEXTENSION PFAXREGISTERROUTINGEXTENSIONW

WINFAXAPI
BOOL
WINAPI
FaxUnregisterRoutingExtensionA(
    IN HANDLE           hFaxHandle,
    IN LPCSTR         lpctstrExtensionName
);
WINFAXAPI
BOOL
WINAPI
FaxUnregisterRoutingExtensionW(
    IN HANDLE           hFaxHandle,
    IN LPCWSTR         lpctstrExtensionName
);
#ifdef UNICODE
#define FaxUnregisterRoutingExtension  FaxUnregisterRoutingExtensionW
#else
#define FaxUnregisterRoutingExtension  FaxUnregisterRoutingExtensionA
#endif // !UNICODE


WINFAXAPI
BOOL
WINAPI
FaxAccessCheck(
    IN HANDLE FaxHandle,
    IN DWORD  AccessMask
    );

typedef BOOL
(WINAPI *PFAXACCESSCHECK)(
    IN HANDLE FaxHandle,
    IN DWORD  AccessMask
    );

//
// Fax Specific Access Rights
//

#define FAX_JOB_SUBMIT          (0x0001)
#define FAX_JOB_QUERY           (0x0002)
#define FAX_CONFIG_QUERY        (0x0004)
#define FAX_CONFIG_SET          (0x0008)
#define FAX_PORT_QUERY          (0x0010)
#define FAX_PORT_SET            (0x0020)
#define FAX_JOB_MANAGE          (0x0040)

#define FAX_READ                (STANDARD_RIGHTS_READ        |\
                                 FAX_JOB_QUERY               |\
                                 FAX_CONFIG_QUERY            |\
                                 FAX_PORT_QUERY)

#define FAX_WRITE               (STANDARD_RIGHTS_WRITE       |\
                                 FAX_JOB_SUBMIT )

#define FAX_ALL_ACCESS          (STANDARD_RIGHTS_ALL         |\
                                 FAX_JOB_SUBMIT              |\
                                 FAX_JOB_QUERY               |\
                                 FAX_CONFIG_QUERY            |\
                                 FAX_CONFIG_SET              |\
                                 FAX_PORT_QUERY              |\
                                 FAX_PORT_SET                |\
                                 FAX_JOB_MANAGE)



#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
