//
//  CDO message file
//
// IMPORTANT:
// if you add a new string do it at the end of the existing strings for the given section (cdosys, cdosvr or cdoex)
// see x5:191917 for more details
//
// Message categories
//
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
// MessageId: categoryHeader
//
// MessageText:
//
// Header
//
#define categoryHeader                   0x00000001L

//
// MessageId: categoryUnused
//
// MessageText:
//
// None
//
#define categoryUnused                   0x00000002L

//
// MessageId: categoryGeneral
//
// MessageText:
//
// General
//
#define categoryGeneral                  0x00000003L

//////////////////////////////////////////////////////////////////////////////
// Error code ranges in FACILITY_ITF. 
// Different libraries sharing the CDO name
// should not conflict in their error codes. Note that below 0x200 is
// reserved by COM.
//	0x200 through 0x5ff    - CDOEX and CDOSYS error codes
// 0x600 through 0x6ff    - workflow error codes
//	0x1000 through 0x1100
//	0x4000 through 0x4100
//
//////////////////////////////////////////////////////////////////////////////
// MessageId Ranges used by the localizer
//
// 1: 0x200 through 0x7ff, 0x4000 through 0x4fff: Library error messages, not localized.
// 2: 0x2000 through 0x2fff: Server error messages, localized in server languages.
// 3: 0x1000 through 0x1fff: Client text, localized in client languages.
//
//////////////////////////////////////////////////////////////////////////////
// Error strings for IErrorInfo
//
// MessageId = 0x200 through 0x7ff or 0x4000 through 0x4fff. Not localized.
//
// 0x200 available - was CDO_E_FAIL
//
// MessageId: CDO_E_UNCAUGHT_EXCEPTION
//
// MessageText:
//
// Exception %1 was generated at address %2
//
#define CDO_E_UNCAUGHT_EXCEPTION         0x80040201L

// 0x202 was CDO_E_NOT_BOUND, was renamed
//
// MessageId: CDO_E_NOT_OPENED
//
// MessageText:
//
// No data source has been opened for the object.
//
#define CDO_E_NOT_OPENED                 0x80040202L

//
// MessageId: CDO_E_UNSUPPORTED_DATASOURCE
//
// MessageText:
//
// The object does not support this type of data source.
//
#define CDO_E_UNSUPPORTED_DATASOURCE     0x80040203L

//
// MessageId: CDO_E_INVALID_PROPERTYNAME
//
// MessageText:
//
// The object does not support the requested property name or namespace.
//
#define CDO_E_INVALID_PROPERTYNAME       0x80040204L

//
// MessageId: CDO_E_PROP_UNSUPPORTED
//
// MessageText:
//
// The object does not support the requested property.
//
#define CDO_E_PROP_UNSUPPORTED           0x80040205L

// 0x206 - was CDO_E_NOT_INITIALIZED
//
// MessageId: CDO_E_INACTIVE
//
// MessageText:
//
// The object is not active. It may have been deleted or it may not have been opened.
//
#define CDO_E_INACTIVE                   0x80040206L

//
// MessageId: CDO_E_NO_SUPPORT_FOR_OBJECTS
//
// MessageText:
//
// The object does not support storing persistent state information for objects.
//
#define CDO_E_NO_SUPPORT_FOR_OBJECTS     0x80040207L

//
// MessageId: CDO_E_NOT_AVAILABLE
//
// MessageText:
//
// The requested property or feature, while supported, is not available at this time or in this context.
//
#define CDO_E_NOT_AVAILABLE              0x80040208L

//
// MessageId: CDO_E_NO_DEFAULT_DROP_DIR
//
// MessageText:
//
// No default drop directory has been configured for this server.
//
#define CDO_E_NO_DEFAULT_DROP_DIR        0x80040209L

//
// MessageId: CDO_E_SMTP_SERVER_REQUIRED
//
// MessageText:
//
// The SMTP server name is required, and was not found in the configuration source.
//
#define CDO_E_SMTP_SERVER_REQUIRED       0x8004020AL

//
// MessageId: CDO_E_NNTP_SERVER_REQUIRED
//
// MessageText:
//
// The NNTP server name is required, and was not found in the configuration source.
//
#define CDO_E_NNTP_SERVER_REQUIRED       0x8004020BL

//
// MessageId: CDO_E_RECIPIENT_MISSING
//
// MessageText:
//
// At least one recipient is required, but none were found.
//
#define CDO_E_RECIPIENT_MISSING          0x8004020CL

//
// MessageId: CDO_E_FROM_MISSING
//
// MessageText:
//
// At least one of the From or Sender fields is required, and neither was found.
//
#define CDO_E_FROM_MISSING               0x8004020DL

//
// MessageId: CDO_E_SENDER_REJECTED
//
// MessageText:
//
// The server rejected the sender address. The server response was: %1
//
#define CDO_E_SENDER_REJECTED            0x8004020EL

//
// MessageId: CDO_E_RECIPIENTS_REJECTED
//
// MessageText:
//
// The server rejected one or more recipient addresses. The server response was: %1
//
#define CDO_E_RECIPIENTS_REJECTED        0x8004020FL

//
// MessageId: CDO_E_NNTP_POST_FAILED
//
// MessageText:
//
// The message could not be posted to the NNTP server. The transport error code was %2. The server response was %1
//
#define CDO_E_NNTP_POST_FAILED           0x80040210L

//
// MessageId: CDO_E_SMTP_SEND_FAILED
//
// MessageText:
//
// The message could not be sent to the SMTP server. The transport error code was %2. The server response was %1
//
#define CDO_E_SMTP_SEND_FAILED           0x80040211L

//
// MessageId: CDO_E_CONNECTION_DROPPED
//
// MessageText:
//
// The transport lost its connection to the server.
//
#define CDO_E_CONNECTION_DROPPED         0x80040212L

//
// MessageId: CDO_E_FAILED_TO_CONNECT
//
// MessageText:
//
// The transport failed to connect to the server.
//
#define CDO_E_FAILED_TO_CONNECT          0x80040213L

//
// MessageId: CDO_E_INVALID_POST
//
// MessageText:
//
// The Subject, From, and Newsgroup fields are all required, and one or more was not found.
//
#define CDO_E_INVALID_POST               0x80040214L

//0x215 was CDO_E_DELETE_FAILED
//
// MessageId: CDO_E_AUTHENTICATION_FAILURE
//
// MessageText:
//
// The server rejected the logon attempt due to authentication failure. The server response was: %1
//
#define CDO_E_AUTHENTICATION_FAILURE     0x80040215L

//
// MessageId: CDO_E_INVALID_CONTENT_TYPE
//
// MessageText:
//
// The content type was not valid in this context. For example, the root of an MHTML message must be an HTML document.
//
#define CDO_E_INVALID_CONTENT_TYPE       0x80040216L

//
// MessageId: CDO_E_LOGON_FAILURE
//
// MessageText:
//
// The transport was unable to log on to the server.
//
#define CDO_E_LOGON_FAILURE              0x80040217L

//
// MessageId: CDO_E_HTTP_NOT_FOUND
//
// MessageText:
//
// The requested resource could not be found. The server response was: %1.
//
#define CDO_E_HTTP_NOT_FOUND             0x80040218L

//
// MessageId: CDO_E_HTTP_FORBIDDEN
//
// MessageText:
//
// Access to the requested resource is denied. The server response was: %1.
//
#define CDO_E_HTTP_FORBIDDEN             0x80040219L

//
// MessageId: CDO_E_HTTP_FAILED
//
// MessageText:
//
// The HTTP request failed.  The server response was: %1.
//
#define CDO_E_HTTP_FAILED                0x8004021AL

//
// MessageId: CDO_E_MULTIPART_NO_DATA
//
// MessageText:
//
// This is a multipart body part. It has no content other than the body parts contained within it.
//
#define CDO_E_MULTIPART_NO_DATA          0x8004021BL

// renamed, was CDO_E_INVALID_ENCODING_FOR_COMPOSITE
//
// MessageId: CDO_E_INVALID_ENCODING_FOR_MULTIPART
//
// MessageText:
//
// Multipart body parts must be encoded as 7bit, 8bit, or binary.
//
#define CDO_E_INVALID_ENCODING_FOR_MULTIPART 0x8004021CL

// 0x21d was CDO_E_PATHTOOLONG
//
// MessageId: CDO_E_UNSAFE_OPERATION
//
// MessageText:
//
// The requested operation is not allowed in secure mode.
//
#define CDO_E_UNSAFE_OPERATION           0x8004021DL

//
// MessageId: CDO_E_PROP_NOT_FOUND
//
// MessageText:
//
// The requested property was not found.
//
#define CDO_E_PROP_NOT_FOUND             0x8004021EL

// 0x21f available, was CDO_E_UNSUPPORTED_PROTOCOL
//
// MessageId: CDO_E_INVALID_SEND_OPTION
//
// MessageText:
//
// The "SendUsing" configuration value is invalid.
//
#define CDO_E_INVALID_SEND_OPTION        0x80040220L

//
// MessageId: CDO_E_INVALID_POST_OPTION
//
// MessageText:
//
// The "PostUsing" configuration value is invalid.
//
#define CDO_E_INVALID_POST_OPTION        0x80040221L

//
// MessageId: CDO_E_NO_PICKUP_DIR
//
// MessageText:
//
// The pickup directory path is required and was not specified. 
//
#define CDO_E_NO_PICKUP_DIR              0x80040222L

//
// MessageId: CDO_E_NOT_ALL_DELETED
//
// MessageText:
//
// One or more messages could not be deleted.
//
#define CDO_E_NOT_ALL_DELETED            0x80040223L

//
// MessageId: CDO_E_NO_METHOD
//
// MessageText:
//
// The requested operation is not available on the underlying object.
//
#define CDO_E_NO_METHOD                  0x80040224L

//0x225 available, was CDO_E_BYTEARRAY_REQUIRED
// 0x226 available, was CDO_E_BSTR_REQUIRED
//
// MessageId: CDO_E_PROP_READONLY
//
// MessageText:
//
// The property is read-only.
//
#define CDO_E_PROP_READONLY              0x80040227L

//
// MessageId: CDO_E_PROP_CANNOT_DELETE
//
// MessageText:
//
// The property cannot be deleted.
//
#define CDO_E_PROP_CANNOT_DELETE         0x80040228L

//
// MessageId: CDO_E_BAD_DATA
//
// MessageText:
//
// Data written to the object are inconsistent or invalid. 
//
#define CDO_E_BAD_DATA                   0x80040229L

//
// MessageId: CDO_E_PROP_NONHEADER
//
// MessageText:
//
// The requested property is not in the mail header namespace.
//
#define CDO_E_PROP_NONHEADER             0x8004022AL

//
// MessageId: CDO_E_INVALID_CHARSET
//
// MessageText:
//
// The requested character set is not installed on the computer.
//
#define CDO_E_INVALID_CHARSET            0x8004022BL

//
// MessageId: CDO_E_ADOSTREAM_NOT_BOUND
//
// MessageText:
//
// The ADO stream has not been opened.
//
#define CDO_E_ADOSTREAM_NOT_BOUND        0x8004022CL

//
// MessageId: CDO_E_CONTENTPROPXML_NOT_FOUND
//
// MessageText:
//
// The content properties are missing.
//
#define CDO_E_CONTENTPROPXML_NOT_FOUND   0x8004022DL

//
// MessageId: CDO_E_CONTENTPROPXML_WRONG_CHARSET
//
// MessageText:
//
// Content properties XML must be encoded using UTF-8.
//
#define CDO_E_CONTENTPROPXML_WRONG_CHARSET 0x8004022EL

//
// MessageId: CDO_E_CONTENTPROPXML_PARSE_FAILED
//
// MessageText:
//
// Failed to parse content properties XML.
//
#define CDO_E_CONTENTPROPXML_PARSE_FAILED 0x8004022FL

//
// MessageId: CDO_E_CONTENTPROPXML_CONVERT_FAILED
//
// MessageText:
//
// Failed to convert a property from XML to a requested type.
//
#define CDO_E_CONTENTPROPXML_CONVERT_FAILED 0x80040230L

//
// MessageId: CDO_E_NO_DIRECTORIES_SPECIFIED
//
// MessageText:
//
// No directories were specified for resolution.
//
#define CDO_E_NO_DIRECTORIES_SPECIFIED   0x80040231L

//
// MessageId: CDO_E_DIRECTORIES_UNREACHABLE
//
// MessageText:
//
// Failed to resolve against one or more of the specified directories.
//
#define CDO_E_DIRECTORIES_UNREACHABLE    0x80040232L

//
// MessageId: CDO_E_BAD_SENDER
//
// MessageText:
//
// Could not find the Sender's mailbox.
//
#define CDO_E_BAD_SENDER                 0x80040233L

//
// MessageId: CDO_E_SELF_BINDING
//
// MessageText:
//
// Binding to self is not allowed.
//
#define CDO_E_SELF_BINDING               0x80040234L

//
// MessageId: CDO_E_BAD_ATTENDEE_DATA
//
// MessageText:
//
// Data of the Attendees in the object are inconsistent or invalid. 
//
#define CDO_E_BAD_ATTENDEE_DATA          0x80040235L

//
// MessageId: CDO_E_ROLE_NOMORE_AVAILABLE
//
// MessageText:
//
// No more roles of the desired type are available - maxed out.
//
#define CDO_E_ROLE_NOMORE_AVAILABLE      0x80040236L

#ifdef CDOTASKS
//
// MessageId: CDO_E_BAD_TASKTYPE_ONASSIGN
//
// MessageText:
//
// The task with its current task type does not permit a call on Assign. Please call CreateUnassignedCopy instead.
//
#define CDO_E_BAD_TASKTYPE_ONASSIGN      0x80040237L

//
// MessageId: CDO_E_NOT_ASSIGNEDTO_USER
//
// MessageText:
//
// The task is not assigned to the current user.
//
#define CDO_E_NOT_ASSIGNEDTO_USER        0x80040238L

#endif
//
// MessageId: CDO_E_OUTOFDATE
//
// MessageText:
//
// The item being accessed or manipulated is out of date.
//
#define CDO_E_OUTOFDATE                  0x80040239L


//////////////////////////////////////////////////////////////////////////////
// parameter validation error messages
//
// MessageId: CDO_E_ARGUMENT1
//
// MessageText:
//
// The first argument is invalid
//
#define CDO_E_ARGUMENT1                  0x80044000L

//
// MessageId: CDO_E_ARGUMENT2
//
// MessageText:
//
// The second argument is invalid
//
#define CDO_E_ARGUMENT2                  0x80044001L

//
// MessageId: CDO_E_ARGUMENT3
//
// MessageText:
//
// The third argument is invalid
//
#define CDO_E_ARGUMENT3                  0x80044002L

//
// MessageId: CDO_E_ARGUMENT4
//
// MessageText:
//
// The fourth argument is invalid
//
#define CDO_E_ARGUMENT4                  0x80044003L

//
// MessageId: CDO_E_ARGUMENT5
//
// MessageText:
//
// The fifth argument is invalid
//
#define CDO_E_ARGUMENT5                  0x80044004L


//////////////////////////////////////////////////////////////////////////////
// Errors returned by MimeOle
//
// These are defined here to provide extended error info text descriptions.
// CDO should never explicitly return any of these errors; they may be passed
// through from MimeOle
// I expect there are numerous places where these errors could still slip through:
// MIME_E_NOT_FOUND
//
// MessageId: CDO_E_NOT_FOUND
//
// MessageText:
//
// The requested body part was not found in this message.
//
#define CDO_E_NOT_FOUND                  0x800CCE05L

// MIME_E_INVALID_ENCODINGTYPE
// Returned when MimeOLE tries to BinHex encode.  Can also be returned if an invalid encoding
// is specified, but CDO should protect from these cases.
//
// Also CDO returns this error when a client sets an invalid encoding
//
// MessageId: CDO_E_INVALID_ENCODING_TYPE
//
// MessageText:
//
// The content encoding type is invalid.
//
#define CDO_E_INVALID_ENCODING_TYPE      0x800CCE1DL

///////////////////////////////////////
// Headers and folder names
//
// MessageId = 0x1000 through 0x1fff, localized in client languages.
///////////////////////////////////////
//
// MessageId: IDS_ORIGINAL_MESSAGE
//
// MessageText:
//
// -----Original Message-----%0
//
#define IDS_ORIGINAL_MESSAGE             0x00011000L

//
// MessageId: IDS_FROM
//
// MessageText:
//
// From:%0
//
#define IDS_FROM                         0x00011001L

//
// MessageId: IDS_SENT
//
// MessageText:
//
// Sent:%0
//
#define IDS_SENT                         0x00011002L

//
// MessageId: IDS_POSTED_AT
//
// MessageText:
//
// Posted At:%0
//
#define IDS_POSTED_AT                    0x00011003L

//
// MessageId: IDS_TO
//
// MessageText:
//
// To:%0
//
#define IDS_TO                           0x00011004L

//
// MessageId: IDS_CC
//
// MessageText:
//
// Cc:%0
//
#define IDS_CC                           0x00011005L

//
// MessageId: IDS_POSTED_TO
//
// MessageText:
//
// Posted To:%0
//
#define IDS_POSTED_TO                    0x00011006L

//
// MessageId: IDS_CONVERSATION
//
// MessageText:
//
// Conversation:%0
//
#define IDS_CONVERSATION                 0x00011007L

//
// MessageId: IDS_SUBJECT
//
// MessageText:
//
// Subject:%0
//
#define IDS_SUBJECT                      0x00011008L

//
// MessageId: IDS_IMPORTANCE
//
// MessageText:
//
// Importance:%0
//
#define IDS_IMPORTANCE                   0x00011009L

//
// MessageId: IDS_ON_BEHALF_OF
//
// MessageText:
//
// on behalf of%0
//
#define IDS_ON_BEHALF_OF                 0x0001100AL

//
// MessageId: IDS_FW
//
// MessageText:
//
// FW:%0
//
#define IDS_FW                           0x0001100BL

//
// MessageId: IDS_RE
//
// MessageText:
//
// RE:%0
//
#define IDS_RE                           0x0001100CL

//
// MessageId: IDS_CODEPAGE
//
// MessageText:
//
// 1252%0
//
#define IDS_CODEPAGE                     0x0001100DL

#ifdef CDOSVR
//
// MessageId: IDS_CalendarFolder
//
// MessageText:
//
// Calendar%0
//
#define IDS_CalendarFolder               0x0001100EL

//
// MessageId: IDS_ContactsFolder
//
// MessageText:
//
// Contacts%0
//
#define IDS_ContactsFolder               0x0001100FL

//
// MessageId: IDS_DraftsFolder
//
// MessageText:
//
// Drafts%0
//
#define IDS_DraftsFolder                 0x00011010L

//
// MessageId: IDS_JournalFolder
//
// MessageText:
//
// Journal%0
//
#define IDS_JournalFolder                0x00011011L

//
// MessageId: IDS_NotesFolder
//
// MessageText:
//
// Notes%0
//
#define IDS_NotesFolder                  0x00011012L

//
// MessageId: IDS_TasksFolder
//
// MessageText:
//
// Tasks%0
//
#define IDS_TasksFolder                  0x00011013L

#endif
//
// MessageId: IDS_NewFolder
//
// MessageText:
//
// New Folder%0
//
#define IDS_NewFolder                    0x00011014L

//
// MessageId: IDS_Location
//
// MessageText:
//
// Location:%0
//
#define IDS_Location                     0x00011015L

//
// MessageId: IDS_StartTime
//
// MessageText:
//
// Start Time:%0
//
#define IDS_StartTime                    0x00011016L

//
// MessageId: IDS_EndTime
//
// MessageText:
//
// End Time:%0
//
#define IDS_EndTime                      0x00011017L

//
// MessageId: IDS_TimeZone
//
// MessageText:
//
// Time Zone:%0
//
#define IDS_TimeZone                     0x00011018L

//
// MessageId: IDS_LocalTime
//
// MessageText:
//
// Local time%0
//
#define IDS_LocalTime                    0x00011019L

//
// MessageId: IDS_Organizer
//
// MessageText:
//
// Organizer:%0
//
#define IDS_Organizer                    0x0001101AL

//
// MessageId: IDS_ApptType
//
// MessageText:
//
// Type:%0
//
#define IDS_ApptType                     0x0001101BL

//
// MessageId: IDS_SingleAppt
//
// MessageText:
//
// Single Appointment%0
//
#define IDS_SingleAppt                   0x0001101CL

//
// MessageId: IDS_SingleMtg
//
// MessageText:
//
// Single Meeting%0
//
#define IDS_SingleMtg                    0x0001101DL

//
// MessageId: IDS_RecurAppt
//
// MessageText:
//
// Recurring Appointment%0
//
#define IDS_RecurAppt                    0x0001101EL

//
// MessageId: IDS_RecurMtg
//
// MessageText:
//
// Recurring Meeting%0
//
#define IDS_RecurMtg                     0x0001101FL

///////////////////////////////////////
// time zones
//
// MessageId = 0x1100 through 0x11ff
///////////////////////////////////////
//
// MessageId: IDS_Universal
//
// MessageText:
//
// Universal Coordinated Time%0
//
#define IDS_Universal                    0x00011100L

//
// MessageId: IDS_Greenwich
//
// MessageText:
//
// (GMT) Greenwich Mean Time : Dublin, Edinburgh, Lisbon, London%0
//
#define IDS_Greenwich                    0x00011101L

//
// MessageId: IDS_Sarajevo
//
// MessageText:
//
// (GMT+01:00) Sarajevo, Skopje, Sofija, Vilnius, Warsaw, Zagreb%0
//
#define IDS_Sarajevo                     0x00011102L

//
// MessageId: IDS_Paris
//
// MessageText:
//
// (GMT+01:00) Brussels, Copenhagen, Madrid, Paris%0
//
#define IDS_Paris                        0x00011103L

//
// MessageId: IDS_Berlin
//
// MessageText:
//
// (GMT+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna%0
//
#define IDS_Berlin                       0x00011104L

//
// MessageId: IDS_EasternEurope
//
// MessageText:
//
// (GMT+02:00) Bucharest%0
//
#define IDS_EasternEurope                0x00011105L

//
// MessageId: IDS_Prague
//
// MessageText:
//
// (GMT+01:00) Belgrade, Bratislava, Budapest, Ljubljana, Prague%0
//
#define IDS_Prague                       0x00011106L

//
// MessageId: IDS_Athens
//
// MessageText:
//
// (GMT+02:00) Athens, Istanbul, Minsk%0
//
#define IDS_Athens                       0x00011107L

//
// MessageId: IDS_Brasilia
//
// MessageText:
//
// (GMT-03:00) Brasilia%0
//
#define IDS_Brasilia                     0x00011108L

//
// MessageId: IDS_Atlantic
//
// MessageText:
//
// (GMT-04:00) Atlantic Time (Canada)%0
//
#define IDS_Atlantic                     0x00011109L

//
// MessageId: IDS_Eastern
//
// MessageText:
//
// (GMT-05:00) Eastern Time (US & Canada)%0
//
#define IDS_Eastern                      0x0001110AL

//
// MessageId: IDS_Central
//
// MessageText:
//
// (GMT-06:00) Central Time (US & Canada)%0
//
#define IDS_Central                      0x0001110BL

//
// MessageId: IDS_Mountain
//
// MessageText:
//
// (GMT-07:00) Mountain Time (US & Canada)%0
//
#define IDS_Mountain                     0x0001110CL

//
// MessageId: IDS_Pacific
//
// MessageText:
//
// (GMT-08:00) Pacific Time (US & Canada)%0
//
#define IDS_Pacific                      0x0001110DL

//
// MessageId: IDS_Alaska
//
// MessageText:
//
// (GMT-09:00) Alaska%0
//
#define IDS_Alaska                       0x0001110EL

//
// MessageId: IDS_Hawaii
//
// MessageText:
//
// (GMT-10:00) Hawaii%0
//
#define IDS_Hawaii                       0x0001110FL

//
// MessageId: IDS_Midway
//
// MessageText:
//
// (GMT-11:00) Midway Island, Samoa%0
//
#define IDS_Midway                       0x00011110L

//
// MessageId: IDS_Wellington
//
// MessageText:
//
// (GMT+12:00) Auckland, Wellington%0
//
#define IDS_Wellington                   0x00011111L

//
// MessageId: IDS_Brisbane
//
// MessageText:
//
// (GMT+10:00) Brisbane%0
//
#define IDS_Brisbane                     0x00011112L

//
// MessageId: IDS_Adelaide
//
// MessageText:
//
// (GMT+09:30) Adelaide%0
//
#define IDS_Adelaide                     0x00011113L

//
// MessageId: IDS_Tokyo
//
// MessageText:
//
// (GMT+09:00) Osaka, Sapporo, Tokyo%0
//
#define IDS_Tokyo                        0x00011114L

//
// MessageId: IDS_Singapore
//
// MessageText:
//
// (GMT+08:00) Kuala Lumpur, Singapore%0
//
#define IDS_Singapore                    0x00011115L

//
// MessageId: IDS_Bangkok
//
// MessageText:
//
// (GMT+07:00) Bangkok, Hanoi, Jakarta%0
//
#define IDS_Bangkok                      0x00011116L

//
// MessageId: IDS_Bombay
//
// MessageText:
//
// (GMT+05:30) Kolkata, Chennai, Mumbai, New Delhi%0
//
#define IDS_Bombay                       0x00011117L

//
// MessageId: IDS_AbuDhabi
//
// MessageText:
//
// (GMT+04:00) Abu Dhabi, Muscat%0
//
#define IDS_AbuDhabi                     0x00011118L

//
// MessageId: IDS_Tehran
//
// MessageText:
//
// (GMT+03:30) Tehran%0
//
#define IDS_Tehran                       0x00011119L

//
// MessageId: IDS_Baghdad
//
// MessageText:
//
// (GMT+03:00) Baghdad%0
//
#define IDS_Baghdad                      0x0001111AL

//
// MessageId: IDS_Israel
//
// MessageText:
//
// (GMT+02:00) Jerusalem%0
//
#define IDS_Israel                       0x0001111BL

//
// MessageId: IDS_Newfoundland
//
// MessageText:
//
// (GMT-03:30) Newfoundland%0
//
#define IDS_Newfoundland                 0x0001111CL

//
// MessageId: IDS_Azores
//
// MessageText:
//
// (GMT-01:00) Azores%0
//
#define IDS_Azores                       0x0001111DL

//
// MessageId: IDS_MidAtlantic
//
// MessageText:
//
// (GMT-02:00) Mid-Atlantic%0
//
#define IDS_MidAtlantic                  0x0001111EL

//
// MessageId: IDS_Monrovia
//
// MessageText:
//
// (GMT) Casablanca, Monrovia%0
//
#define IDS_Monrovia                     0x0001111FL

//
// MessageId: IDS_BuenosAires
//
// MessageText:
//
// (GMT-03:00) Buenos Aires, Georgetown%0
//
#define IDS_BuenosAires                  0x00011120L

//
// MessageId: IDS_Caracas
//
// MessageText:
//
// (GMT-04:00) Caracas, La Paz%0
//
#define IDS_Caracas                      0x00011121L

//
// MessageId: IDS_Indiana
//
// MessageText:
//
// (GMT-05:00) Indiana (East)%0
//
#define IDS_Indiana                      0x00011122L

//
// MessageId: IDS_Bogota
//
// MessageText:
//
// (GMT-05:00) Bogota, Lima, Quito%0
//
#define IDS_Bogota                       0x00011123L

//
// MessageId: IDS_Saskatchewan
//
// MessageText:
//
// (GMT-06:00) Saskatchewan%0
//
#define IDS_Saskatchewan                 0x00011124L

//
// MessageId: IDS_Mexico
//
// MessageText:
//
// (GMT-06:00) Mexico City%0
//
#define IDS_Mexico                       0x00011125L

//
// MessageId: IDS_Arizona
//
// MessageText:
//
// (GMT-07:00) Arizona%0
//
#define IDS_Arizona                      0x00011126L

//
// MessageId: IDS_Eniwetok
//
// MessageText:
//
// (GMT-12:00) Eniwetok, Kwajalein%0
//
#define IDS_Eniwetok                     0x00011127L

//
// MessageId: IDS_Fiji
//
// MessageText:
//
// (GMT+12:00) Fiji Is., Kamchatka, Marshall Is.%0
//
#define IDS_Fiji                         0x00011128L

//
// MessageId: IDS_Magadan
//
// MessageText:
//
// (GMT+11:00) Magadan, Solomon Is., New Caledonia%0
//
#define IDS_Magadan                      0x00011129L

//
// MessageId: IDS_Hobart
//
// MessageText:
//
// (GMT+10:00) Hobart%0
//
#define IDS_Hobart                       0x0001112AL

//
// MessageId: IDS_Guam
//
// MessageText:
//
// (GMT+10:00) Guam, Port Moresby%0
//
#define IDS_Guam                         0x0001112BL

//
// MessageId: IDS_Darwin
//
// MessageText:
//
// (GMT+09:30) Darwin%0
//
#define IDS_Darwin                       0x0001112CL

//
// MessageId: IDS_Beijing
//
// MessageText:
//
// (GMT+08:00) Beijing, Chongqing, Hong Kong SAR, Urumqi%0
//
#define IDS_Beijing                      0x0001112DL

//
// MessageId: IDS_Almaty
//
// MessageText:
//
// (GMT+06:00) Almaty, Novosibirsk%0
//
#define IDS_Almaty                       0x0001112EL

//
// MessageId: IDS_Islamabad
//
// MessageText:
//
// (GMT+05:00) Islamabad, Karachi, Tashkent%0
//
#define IDS_Islamabad                    0x0001112FL

//
// MessageId: IDS_Kabul
//
// MessageText:
//
// (GMT+04:30) Kabul%0
//
#define IDS_Kabul                        0x00011130L

//
// MessageId: IDS_Cairo
//
// MessageText:
//
// (GMT+02:00) Cairo%0
//
#define IDS_Cairo                        0x00011131L

//
// MessageId: IDS_Harare
//
// MessageText:
//
// (GMT+02:00) Harare, Pretoria%0
//
#define IDS_Harare                       0x00011132L

//
// MessageId: IDS_Moscow
//
// MessageText:
//
// (GMT+03:00) Moscow, St. Petersburg, Volgograd%0
//
#define IDS_Moscow                       0x00011133L

//
// MessageId: IDS_CapeVerde
//
// MessageText:
//
// (GMT-01:00) Cape Verde Is.%0
//
#define IDS_CapeVerde                    0x00011134L

//
// MessageId: IDS_Caucasus
//
// MessageText:
//
// (GMT+04:00) Baku, Tbilisi, Yerevan%0
//
#define IDS_Caucasus                     0x00011135L

//
// MessageId: IDS_CentralAmerica
//
// MessageText:
//
// (GMT-06:00) Central America%0
//
#define IDS_CentralAmerica               0x00011136L

//
// MessageId: IDS_EastAfrica
//
// MessageText:
//
// (GMT+03:00) Nairobi%0
//
#define IDS_EastAfrica                   0x00011137L

//
// MessageId: IDS_Melbourne
//
// MessageText:
//
// (GMT+10:00) Canberra, Melbourne, Sydney%0
//
#define IDS_Melbourne                    0x00011138L

//
// MessageId: IDS_Ekaterinburg
//
// MessageText:
//
// (GMT+05:00) Ekaterinburg%0
//
#define IDS_Ekaterinburg                 0x00011139L

//
// MessageId: IDS_Helsinki
//
// MessageText:
//
// (GMT+02:00) Helsinki, Riga, Tallinn%0
//
#define IDS_Helsinki                     0x0001113AL

//
// MessageId: IDS_Greenland
//
// MessageText:
//
// (GMT-03:00) Greenland%0
//
#define IDS_Greenland                    0x0001113BL

//
// MessageId: IDS_Rangoon
//
// MessageText:
//
// (GMT+06:30) Yangon (Rangun)%0
//
#define IDS_Rangoon                      0x0001113CL

//
// MessageId: IDS_Nepal
//
// MessageText:
//
// (GMT+05:45) Kathmandu%0
//
#define IDS_Nepal                        0x0001113DL

//
// MessageId: IDS_Irkutsk
//
// MessageText:
//
// (GMT+08:00) Irkutsk, Ulaan Bataar%0
//
#define IDS_Irkutsk                      0x0001113EL

//
// MessageId: IDS_Krasnoyarsk
//
// MessageText:
//
// (GMT+07:00) Krasnoyarsk%0
//
#define IDS_Krasnoyarsk                  0x0001113FL

//
// MessageId: IDS_Santiago
//
// MessageText:
//
// (GMT-04:00) Santiago%0
//
#define IDS_Santiago                     0x00011140L

//
// MessageId: IDS_SriLanka
//
// MessageText:
//
// (GMT+06:00) Sri Jayawardenepura%0
//
#define IDS_SriLanka                     0x00011141L

//
// MessageId: IDS_Tonga
//
// MessageText:
//
// (GMT+13:00) Nuku'alofa%0
//
#define IDS_Tonga                        0x00011142L

//
// MessageId: IDS_Vladivostok
//
// MessageText:
//
// (GMT+10:00) Vladivostok%0
//
#define IDS_Vladivostok                  0x00011143L

//
// MessageId: IDS_WestCentralAfrica
//
// MessageText:
//
// (GMT+01:00) West Central Africa%0
//
#define IDS_WestCentralAfrica            0x00011144L

//
// MessageId: IDS_Yakutsk
//
// MessageText:
//
// (GMT+09:00) Yakutsk%0
//
#define IDS_Yakutsk                      0x00011145L

//
// MessageId: IDS_Dhaka
//
// MessageText:
//
// (GMT+06:00) Astana, Dhaka%0
//
#define IDS_Dhaka                        0x00011146L

//
// MessageId: IDS_Seoul
//
// MessageText:
//
// (GMT+09:00) Seoul%0
//
#define IDS_Seoul                        0x00011147L

//
// MessageId: IDS_Perth
//
// MessageText:
//
// (GMT+08:00) Perth%0
//
#define IDS_Perth                        0x00011148L

//
// MessageId: IDS_Arab
//
// MessageText:
//
// (GMT+03:00) Kuwait, Riyadh%0
//
#define IDS_Arab                         0x00011149L

//
// MessageId: IDS_Taipei
//
// MessageText:
//
// (GMT+08:00) Taipei%0
//
#define IDS_Taipei                       0x0001114AL

//
// MessageId: IDS_Sydney2000
//
// MessageText:
//
// (GMT+10:00) Canberra, Melbourne, Sydney (Year 2000 only)%0
//
#define IDS_Sydney2000                   0x0001114BL

//
// MessageId: IDS_Chihuahua
//
// MessageText:
//
// (GMT-07:00) Chihuahua, La Paz, Mazatlan%0
//
#define IDS_Chihuahua                    0x0001114CL

//
// MessageId: IDS_CanberraCommonwealthGames2006
//
// MessageText:
//
// (GMT+10:00) Canberra, Melbourne, Sydney (Commonwealth Games)%0
//
#define IDS_CanberraCommonwealthGames2006 0x0001114DL

//
// MessageId: IDS_AdelaideCommonwealthGames2006
//
// MessageText:
//
// (GMT+09:30) Adelaide (Commonwealth Games)%0
//
#define IDS_AdelaideCommonwealthGames2006 0x0001114EL

//
// MessageId: IDS_HobartCommonwealthGames2006
//
// MessageText:
//
// (GMT+10:00) Hobart (Commonwealth Games)%0
//
#define IDS_HobartCommonwealthGames2006  0x0001114FL

//
// MessageId: IDS_Tijuana
//
// MessageText:
//
// (GMT-08:00) Tijuana, Baja California%0
//
#define IDS_Tijuana                      0x00011150L

///////////////////////////////////////
// EXCDO messages
//
// MessageId = 0x2000 through 0x2fff, localized in server languages.
///////////////////////////////////////
#ifdef CDOSVR
//
// MessageId: evtMethodCalled
//
// MessageText:
//
// %1 Event on %2 called with Flags %3
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMethodCalled                  0x00032000L

//
// MessageId: evtMethodReturning
//
// MessageText:
//
// %1 Event Method is returning with HRESULT %2
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMethodReturning               0x00032001L

//
// MessageId: evtIsAborting
//
// MessageText:
//
// %1 Event Method is halting, HRESULT %2
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtIsAborting                    0xC0032002L

//
// MessageId: evtExpansionInitialized
//
// MessageText:
//
// Calendaring agent is initialized successfully.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionInitialized          0x00032003L

//
// MessageId: evtExpansionUnInitialized
//
// MessageText:
//
// Calendaring agent is stopping successfully.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionUnInitialized        0x00032004L

//
// MessageId: evtExpansionInitializeFailed
//
// MessageText:
//
// Calendaring agent failed to initialize with error %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionInitializeFailed     0xC0032005L

//
// MessageId: evtExpansionRegisterFailed
//
// MessageText:
//
// Calendaring recurring item expansion failed to register for notifications for MDB %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionRegisterFailed       0xC0032006L

//
// MessageId: evtExpansionMessageSaveChangesFailed
//
// MessageText:
//
// Calendaring agent failed in message save notification with error %1 on %2: %3.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionMessageSaveChangesFailed 0xC0032007L

//
// MessageId: evtExpansionMessageDeleteFailed
//
// MessageText:
//
// Calendaring agent failed in message delete notification with error %1 on %2: %3.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionMessageDeleteFailed  0xC0032008L

//
// MessageId: evtExpansionFolderSaveChangesFailed
//
// MessageText:
//
// Calendaring agent failed in folder save notification with error %1 on %2: %3.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionFolderSaveChangesFailed 0xC0032009L

//
// MessageId: evtExpansionTooManyInstancesPerDay
//
// MessageText:
//
// Calendaring agent truncated expansion after %1 instances per day.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionTooManyInstancesPerDay 0x8003200AL

//
// MessageId: evtMailboxCreateTotalFailure
//
// MessageText:
//
// The mailbox creation callback failed to create any folders in mailbox %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMailboxCreateTotalFailure     0xC003200BL

//
// MessageId: evtMailboxCreatePartialFailure
//
// MessageText:
//
// The mailbox creation callback failed to create one or more folders in mailbox %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMailboxCreatePartialFailure   0xC003200CL

//
// MessageId: evtUninitImplRestFailed
//
// MessageText:
//
// Calendaring agent failed to uninitialize implied restriction with error %1 on %2: %3.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUninitImplRestFailed          0xC003200DL

//
// MessageId: evtExpandSavingAppt
//
// MessageText:
//
// Calendaring agent failed with error code %1 while saving appointment.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpandSavingAppt              0xC003200EL

//
// MessageId: evtExpandDeletingAppt
//
// MessageText:
//
// Calendaring agent failed with error code %1 while deleting appointment.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpandDeletingAppt            0xC003200FL

//
// MessageId: evtExpandQuery
//
// MessageText:
//
// Calendaring agent failed with error code %1 while expanding recurring appointments.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpandQuery                   0xC0032010L

//
// MessageId: evtExpandFolderSetProps
//
// MessageText:
//
// Calendaring agent failed with error code %1 while cleaning up the calendar.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpandFolderSetProps          0xC0032011L

//
// MessageId: evtRegistryFailure
//
// MessageText:
//
// Calendaring agent failed to open registry with error code %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtRegistryFailure               0xC0032012L

//
// MessageId: evtExpStat
//
// MessageText:
//
// Calendaring agent failed in the following function with error code: %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpStat                       0xC0032013L

//
// MessageId: evtDumpFcn
//
// MessageText:
//
// Calendaring agent failed in the following function with error code %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtDumpFcn                       0xC0032014L

//
// MessageId: evtSaveDeleteFailFBUpdate
//
// MessageText:
//
// Calendaring agent failed to update the free/busy cache during an appointment save or delete operation.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtSaveDeleteFailFBUpdate        0xC0032015L

//
// MessageId: evtProcessingQueryCallback
//
// MessageText:
//
// Calendaring agent failed to update the free/busy cache and internal data while processing a query.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtProcessingQueryCallback       0xC0032016L

//
// MessageId: evtMailboxLocalizeTotalFailure
//
// MessageText:
//
// The mailbox creation callback failed to rename any folders in mailbox %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMailboxLocalizeTotalFailure   0xC0032017L

//
// MessageId: evtMailboxLocalizePartialFailure
//
// MessageText:
//
// The mailbox creation callback failed to rename one or more folders in mailbox %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMailboxLocalizePartialFailure 0xC0032018L

//
// MessageId: evtExpandMaster
//
// MessageText:
//
// The processing of the recurring appointment with the subject "%1" in mailbox: %2 failed with error code %3.  This appointment will not be visible in the web client or other non-MAPI client.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpandMaster                  0xC0032019L

//
// MessageId: evtExpansionInit
//
// MessageText:
//
// A transaction failed during initialization.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionInit                 0xC003201AL

//
// MessageId: evtFBGenerateMsg
//
// MessageText:
//
// The calendaring agent could not publish the free/busy information for "%1" because it failed reading the registry with error: %2.  
// The save/delete calendar operation for "%3" will not be completed.
// Please ensure that MSExchangeFBPublish agent is started.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtFBGenerateMsg                 0xC003201BL

//
// MessageId: evtExpansionInstExpiryInPublicMDB
//
// MessageText:
//
// Expiry of an instance for the recurring appointment with the subject "%1" in the folder "%2" was detected.  The master for that should be expired before this instance.  
// This instance will no longer appear in the web clients until the master is updated manually or through replication.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpansionInstExpiryInPublicMDB 0x8003201CL

//
// MessageId: evtUnhandledExceptionInitialization
//
// MessageText:
//
// Unhandled exception %1 caught during initialization of server agents.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionInitialization 0xC003201DL

//
// MessageId: evtUnhandledExceptionShutdown
//
// MessageText:
//
// Unhandled exception %1 caught trying to stop server agents.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionShutdown    0xC003201EL

//
// MessageId: evtUnhandledExceptionInitializationMDB
//
// MessageText:
//
// Unhandled exception %1 caught during initialization of server agents while an MDB is mounted.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionInitializationMDB 0xC003201FL

//
// MessageId: evtUnhandledExceptionShutdownMDB
//
// MessageText:
//
// Unhandled exception %1 caught trying to stop server agents while an MDB is unmounted.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionShutdownMDB 0xC0032020L

//
// MessageId: evtUnhandledExceptionMsgSaveChanges
//
// MessageText:
//
// Unhandled exception %1 caught during message save changes notification.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionMsgSaveChanges 0xC0032021L

//
// MessageId: evtUnhandledExceptionDelete
//
// MessageText:
//
// Unhandled exception %1 caught during deletion notification.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionDelete      0xC0032022L

//
// MessageId: evtUnhandledExceptionQuery
//
// MessageText:
//
// Unhandled exception %1 caught during query processing.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionQuery       0xC0032023L

//
// MessageId: evtUnhandledExceptionFolderSaveChanges
//
// MessageText:
//
// Unhandled exception %1 caught during folder save changes notification.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtUnhandledExceptionFolderSaveChanges 0xC0032024L

//
// MessageId: evtCorruptedCalendar
//
// MessageText:
//
// The calendar for the mailbox %1 is corrupted.  This calendar needs to be recreated.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtCorruptedCalendar             0xC0032025L

//
// MessageId: evtRebuildCalendar
//
// MessageText:
//
// An inconsistency was detected in %2: %3.  The calendar is being repaired.  If other errors occur with this calendar, please view the calendar using Microsoft Outlook Web Access.  If a problem persists, please recreate the calendar or the containing mailbox.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtRebuildCalendar               0x80032026L

//
// MessageId: evtCheckPrimaryCalendar
//
// MessageText:
//
// Calendar agent failed to determine the primary calendar for mailbox: %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtCheckPrimaryCalendar          0x80032027L

//
// MessageId: evtExpandMasterPF
//
// MessageText:
//
// The processing of the recurring appointment with the subject "%1" in public folder: %2 failed with error code %3.  This appointment will not be visible in the web client or other non-MAPI client.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtExpandMasterPF                0xC0032028L

//
// MessageId: evtCorruptedPFCalendar
//
// MessageText:
//
// The public folder %1 is corrupted.  This calendar needs to be recreated.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtCorruptedPFCalendar           0xC0032029L

//
// MessageId: evtRebuildPFCalendar
//
// MessageText:
//
// An inconsistency was detected in %1.  The calendar is being repaired.  If other errors occur with this calendar, please view the calendar using Microsoft Outlook Web Access.  If a problem persists, please recreate the public folder calendar.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtRebuildPFCalendar             0x8003202AL

//
// MessageId: evtMovingMailboxCallbackFailed
//
// MessageText:
//
// Clearing the calendar instances before a move failed for mailbox %1 with error %2.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtMovingMailboxCallbackFailed   0x8003202BL

//
// MessageId: evtGetMsgClassFromMaster
//
// MessageText:
//
// Calendaring agent failed with error %1 while attempting to retrieve the message class for a recurring appointment.  The message class for the instance
// when viewed with the Web Client will not be the same as the one in the series if it is not IPM.Appointment.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtGetMsgClassFromMaster         0x8003202CL

//
// MessageId: evtOpenURL
//
// MessageText:
//
// Calendaring agent failed with error %1 while attempting to open the status message. If the user is a delegate, the access rights to the mailbox might
// not be set up correctly.  Otherwise, the calendar for this mailbox could be corrupted. This could be fixed
// by moving the mailbox to another Exchange server, or exporting the calendar to a Personal Folder File (.PST), and re-importing this into a new calendar.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtOpenURL                       0x8003202DL

//
// MessageId: evtStatusMsgSent
//
// MessageText:
//
// The calendaring data message for %1: %2 has been sent to %3.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtStatusMsgSent                 0x4003202EL

//
// MessageId: evtSaveInvalidStartAppt
//
// MessageText:
//
// An appointment with the subject "%1" was saved to mailbox %2. The start time is missing, so the calendaring agent cannot publish the free/busy information for this appointment. To correct this, please delete the appointment and recreate it.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtSaveInvalidStartAppt          0xC003202FL

//
// MessageId: evtSaveInvalidEndAppt
//
// MessageText:
//
// An appointment with the subject "%1" was saved to mailbox %2. The end time and duration are missing. For the purposes of publishing free/busy information, this appointment has a duration of zero minutes. To correct this, please delete the appointment and recreate it.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtSaveInvalidEndAppt            0x80032030L

//
// MessageId: evtModifyInvalidAppt
//
// MessageText:
//
// An existing appointment with the subject "%1" in mailbox %2 was modified. The start time, or both the duration and end time of the existing appointment were missing.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtModifyInvalidAppt             0x80032031L

//
// MessageId: evtDeleteInvalidAppt
//
// MessageText:
//
// The existing appointment with the subject "%1" in mailbox %2 was deleted. The start time, or both the duration and end time were missing.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtDeleteInvalidAppt             0x80032032L

//
// MessageId: evtBindToInvalidStartAppt
//
// MessageText:
//
// The calendaring agent cannot bind to the appointment with the subject "%1" in mailbox %2 because the appointment is missing a start time. The calendaring agent will not publish free/busy information for this appointment. To correct this, please delete the appointment and recreate it.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBindToInvalidStartAppt        0xC0032033L

//
// MessageId: evtBindToInvalidEndAppt
//
// MessageText:
//
// The calendaring agent did bind to the appointment with the subject "%1" in mailbox %2, but the appointment is missing an end time and duration. For the purposes of publishing free/busy information, this appointment has a duration of zero minutes. To correct this, please delete the appointment and recreate it.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBindToInvalidEndAppt          0x80032034L

//
// MessageId: evtReloadLogType
//
// MessageText:
//
// The EXCDO logging level has been reloaded from the registry and is now set to 0x%1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtReloadLogType                 0x40032035L

//
// MessageId: evtBackgroundStarted
//
// MessageText:
//
// Calendaring agent has successfully started the background thread.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundStarted             0x40032036L

//
// MessageId: evtBackgroundStartFailed
//
// MessageText:
//
// Calendaring agent has failed to start the background thread due to %1.  
// Rendering of large calendars with the Web client will be slower.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundStartFailed         0x80032037L

//
// MessageId: evtBackgroundProcessing
//
// MessageText:
//
// Background calendaring agent is processing the calendar (%1) in the mailbox (%2) in the MDB (%3).
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundProcessing          0x40032038L

//
// MessageId: evtBackgroundProcessingNoCals
//
// MessageText:
//
// Background calendaring agent did not find any more calendars to process.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundProcessingNoCals    0x40032039L

//
// MessageId: evtBackgroundProcessingFinished
//
// MessageText:
//
// Background calendaring agent has finished processing the calendar (%1) in the mailbox (%2) in the MDB (%3).  The return code is %4.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundProcessingFinished  0x4003203AL

//
// MessageId: evtBackgroundAwakeDueToShutdown
//
// MessageText:
//
// Background calendaring agent is awakened due to shutdown.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundAwakeDueToShutdown  0x4003203BL

//
// MessageId: evtBackgroundAwake
//
// MessageText:
//
// Background calendaring agent is awakened due to calendars needing expansion.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundAwake               0x4003203CL

//
// MessageId: evtBackgroundTerminating
//
// MessageText:
//
// Background calendaring agent is terminating with return code %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundTerminating         0x4003203DL

//
// MessageId: evtBackgroundRegisterTaskFailed
//
// MessageText:
//
// The background calendaring agent failed with error code %1 while registering task.  
// The rendering of large calendars with the web client or Microsoft Internet Explorer will be slower.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundRegisterTaskFailed  0xC003203EL

//
// MessageId: evtBackgroundLogonFailed
//
// MessageText:
//
// The background calendaring agent failed to logon to the MDB, or create a session object on the MDB:%1.  
// The error code is %2.
// The rendering of large calendars with the web client or Microsoft Internet Explorer for the mailbox %3 with GUID:%4 will be slower.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundLogonFailed         0xC003203FL

//
// MessageId: evtBackgroundGetMachineToken
//
// MessageText:
//
// The background calendaring agent failed to retrieve the token required for logon on the MDB:%1.  
// The error code is %2.
// The rendering of large calendars with the web client or Microsoft Internet Explorer on this MDB will be slower.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundGetMachineToken     0xC0032040L

//
// MessageId: evtBackgroundSleepUntilNewCal
//
// MessageText:
//
// Background calendaring agent is sleeping until shutdown or a calendar is queued.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundSleepUntilNewCal    0x40032041L

//
// MessageId: evtBackgroundQueued
//
// MessageText:
//
// The calendar (%1) in mailbox (%2) in the MDB (%3) has been queued for background expansion.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBackgroundQueued              0x40032042L

//
// MessageId: evtBkExpDumpFcn
//
// MessageText:
//
// The background calendaring agent failed in the following function with error code %1.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtBkExpDumpFcn                  0xC0032043L

//
// MessageId: evtSaveDurationTooLong
//
// MessageText:
//
// An attempt was made to save an appointment with the subject "%1" to mailbox %2. The duration of the appointment was too large, so it was not saved.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtSaveDurationTooLong           0x80032044L

//
// MessageId: evtModifyDurationTooLong
//
// MessageText:
//
// An existing appointment with the subject "%1" in mailbox %2 was modified. The duration of the existing appointment was too large. Free/busy publishing may not be accurate.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtModifyDurationTooLong         0x80032045L

//
// MessageId: evtDeleteDurationTooLong
//
// MessageText:
//
// The existing appointment with the subject "%1" in mailbox %2 was deleted. The duration of the existing appointment was too large. Free/busy publishing may not be accurate.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtDeleteDurationTooLong         0x80032046L

//
// MessageId: evtTooManyIterationsForQuery
//
// MessageText:
//
// The recurring appointment expansion in mailbox %1 has taken too long. Some recurring appointment instances may not be visible at this time. To ensure that the calendar view is correct, please refresh the calendar view in Microsoft Outlook Web Access.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtTooManyIterationsForQuery     0x80032047L

//
// MessageId: evtTooManyIterationsForFreebusy
//
// MessageText:
//
// The recurring appointment expansion in mailbox %1 has taken too long. The free/busy information for this calendar may be inaccurate. This may be the result of many very old recurring appointments. To correct this, please remove them or change their start date to a more recent date.
// %n%nFor more information, click http://www.microsoft.com/contentredirect.asp.
//
#define evtTooManyIterationsForFreebusy  0x80032048L

#endif
