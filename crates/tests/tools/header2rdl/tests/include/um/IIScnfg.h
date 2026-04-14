/*++



   Copyright (c) 1997-1999 Microsoft Corporation

   Module  Name :

       iiscnfg.h

   Abstract:

        Contains public Metadata IDs used by IIS.

   Environment:

      Win32 User Mode

--*/

#ifndef _IISCNFG_H_
#define _IISCNFG_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Paths
//

#define IIS_MD_LOCAL_MACHINE_PATH       "LM"

//
// Name of the default publishing root under an instance
//

#define IIS_MD_INSTANCE_ROOT            "Root"

//
//  ISAPI Filters are kept in a list under the instances and the service (for
//  global filters) in the following format:
//
//  LM/W3Svc/<Instance>/Filters
//      MD_FILTER_LOAD_ORDER  "Filter1, Filter2, Filter3"
//
//  LM/W3Svc/<Instance>/Filters/Filter1
//      MD_FILTER_IMAGE_PATH  "d:\inetsrv\myfilter.dll"
//
//  LM/W3Svc/<Instance>/Filters/Filter2
//      MD_FILTER_IMAGE_PATH  "d:\inetsrv\otherfilter.dll"
//

#define IIS_MD_ISAPI_FILTERS            "/Filters"

//
// Path below each service to the key that publishes service information
//

#define IIS_MD_SVC_INFO_PATH            "Info"

//
// ADSI schema properties path
//

#define IIS_MD_ADSI_SCHEMA_PATH_A       "/Schema"
#define IIS_MD_ADSI_SCHEMA_PATH_W       L"/Schema"
#define IIS_MD_ADSI_METAID_BEGIN        130000

//
// user types
//
// There are two user types:
//
//   Server configuration - All the properties for configuring the server that
//      are not applicable to files and directories - such as Port, Host name,
//      Server comment, Connection timeout etc.
//
//  File/Dir configuration - All the properties that can be configured down to
//      the files and directories - such as Access permissions (Read, Write etc),
//      Extension mapping, IP Security etc.
//

#define IIS_MD_UT_SERVER                1   // Server configuration parameters
#define IIS_MD_UT_FILE                  2   // File/Dir inheritable properties
#define IIS_MD_UT_WAM                 100   // Web Application configuration parameters
#define ASP_MD_UT_APP                 101   // ASP application configuration parameters
#define IIS_MD_UT_END_RESERVED       2000   // All user types below this are
                                            // reserved for IIS services


//
//  Metabase property IDs must be unique.  This table defines reserved ranges
//

#define IIS_MD_ID_BEGIN_RESERVED    0x00000001      // IIS reserved range
#define IIS_MD_ID_END_RESERVED      0x00007fff
#define ASP_MD_ID_BEGIN_RESERVED    0x00007000      // ASP reserved range, subrange of IIS.
#define ASP_MD_ID_END_RESERVED      0x000074ff
#define WAM_MD_ID_BEGIN_RESERVED    0x00007500      // ASP reserved range, subrange of IIS.
#define WAM_MD_ID_END_RESERVED      0x00007fff
#define FP_MD_ID_BEGIN_RESERVED     0x00008000      // Front page reserved range
#define FP_MD_ID_END_RESERVED       0x00008fff
#define SMTP_MD_ID_BEGIN_RESERVED   0x00009000
#define SMTP_MD_ID_END_RESERVED     0x00009fff
#define POP3_MD_ID_BEGIN_RESERVED   0x0000a000
#define POP3_MD_ID_END_RESERVED     0x0000afff
#define NNTP_MD_ID_BEGIN_RESERVED   0x0000b000
#define NNTP_MD_ID_END_RESERVED     0x0000bfff
#define IMAP_MD_ID_BEGIN_RESERVED   0x0000c000
#define IMAP_MD_ID_END_RESERVED     0x0000cfff
#define MSCS_MD_ID_BEGIN_RESERVED   0x0000d000
#define MSCS_MD_ID_END_RESERVED     0x0000dfff
#define APPCTR_MD_ID_BEGIN_RESERVED 0x0000e000
#define APPCTR_MD_ID_END_RESERVED   0x0000efff

#define USER_MD_ID_BASE_RESERVED    0x0000ffff

//
//  General server related attributes - these should be added in the metabase
//  with a user type of IIS_MD_UT_SERVER
//

#define IIS_MD_SERVER_BASE              1000

//
//  These are global to all services and should only be set at
//  the IIS root
//

#define MD_MAX_BANDWIDTH                (IIS_MD_SERVER_BASE+0  )
#define MD_KEY_TYPE                     (IIS_MD_SERVER_BASE+2  )
#define MD_MAX_BANDWIDTH_BLOCKED        (IIS_MD_SERVER_BASE+3  )
#define MD_SCHEMA_METAID                (IIS_MD_SERVER_BASE+4  )

//
//  These properties are applicable to both HTTP and FTP virtual
//  servers
//

#define MD_SERVER_COMMAND               (IIS_MD_SERVER_BASE+12 )
#define MD_CONNECTION_TIMEOUT           (IIS_MD_SERVER_BASE+13 )
#define MD_MAX_CONNECTIONS              (IIS_MD_SERVER_BASE+14 )
#define MD_SERVER_COMMENT               (IIS_MD_SERVER_BASE+15 )
#define MD_SERVER_STATE                 (IIS_MD_SERVER_BASE+16 )
#define MD_SERVER_AUTOSTART             (IIS_MD_SERVER_BASE+17 )
#define MD_SERVER_SIZE                  (IIS_MD_SERVER_BASE+18 )
#define MD_SERVER_LISTEN_BACKLOG        (IIS_MD_SERVER_BASE+19 )
#define MD_SERVER_LISTEN_TIMEOUT        (IIS_MD_SERVER_BASE+20 )
#define MD_DOWNLEVEL_ADMIN_INSTANCE     (IIS_MD_SERVER_BASE+21 )
#define MD_LEVELS_TO_SCAN               (IIS_MD_SERVER_BASE+22 )
#define MD_SERVER_BINDINGS              (IIS_MD_SERVER_BASE+23 )
#define MD_MAX_ENDPOINT_CONNECTIONS     (IIS_MD_SERVER_BASE+24 )
#define MD_SERVER_CONFIGURATION_INFO    (IIS_MD_SERVER_BASE+27 )
#define MD_IISADMIN_EXTENSIONS          (IIS_MD_SERVER_BASE+28 )
#define MD_DISABLE_SOCKET_POOLING       (IIS_MD_SERVER_BASE+29 )
#define MD_METADATA_ID_REGISTRATION     (IIS_MD_SERVER_BASE+30 )


//
//  These properties are specific to HTTP and belong to the website
//

#define IIS_MD_HTTP_BASE                2000

#define MD_SECURE_BINDINGS              (IIS_MD_HTTP_BASE+21 )

#define MD_BINDINGS                     (IIS_MD_HTTP_BASE+22 )
#define MD_ENABLEDPROTOCOLS             (IIS_MD_HTTP_BASE+23 )


#define MD_FILTER_LOAD_ORDER            (IIS_MD_HTTP_BASE+40 )
#define MD_FILTER_IMAGE_PATH            (IIS_MD_HTTP_BASE+41 )
#define MD_FILTER_STATE                 (IIS_MD_HTTP_BASE+42 )
#define MD_FILTER_ENABLED               (IIS_MD_HTTP_BASE+43 )
#define MD_FILTER_FLAGS                 (IIS_MD_HTTP_BASE+44 )
#define MD_FILTER_DESCRIPTION           (IIS_MD_HTTP_BASE+45 )
#define MD_FILTER_ENABLE_CACHE          (IIS_MD_HTTP_BASE+46 )

#define MD_ADV_NOTIFY_PWD_EXP_IN_DAYS   (IIS_MD_HTTP_BASE+63 )
#define MD_ADV_CACHE_TTL                (IIS_MD_HTTP_BASE+64 )
#define MD_NET_LOGON_WKS                (IIS_MD_HTTP_BASE+65 )
#define MD_USE_HOST_NAME                (IIS_MD_HTTP_BASE+66 )

#define MD_AUTH_CHANGE_FLAGS            (IIS_MD_HTTP_BASE+68 )

#define MD_PROCESS_NTCR_IF_LOGGED_ON    (IIS_MD_HTTP_BASE+70 )

#define MD_FRONTPAGE_WEB                (IIS_MD_HTTP_BASE+72 )
#define MD_IN_PROCESS_ISAPI_APPS        (IIS_MD_HTTP_BASE+73 )


#define MD_AUTH_CHANGE_URL              (IIS_MD_HTTP_BASE+60 )
#define MD_AUTH_EXPIRED_URL             (IIS_MD_HTTP_BASE+61 )
#define MD_AUTH_EXPIRED_UNSECUREURL     (IIS_MD_HTTP_BASE+67 )

#define MD_ALLOW_PATH_INFO_FOR_SCRIPT_MAPPINGS  ( IIS_MD_HTTP_BASE+95)

#define MD_APP_FRIENDLY_NAME            (IIS_MD_HTTP_BASE+102)
#define MD_APP_ROOT                     (IIS_MD_HTTP_BASE+103)
#define MD_APP_ISOLATED                 (IIS_MD_HTTP_BASE+104)
#define MD_APP_WAM_CLSID                (IIS_MD_HTTP_BASE+105)
#define MD_APP_PACKAGE_ID               (IIS_MD_HTTP_BASE+106)
#define MD_APP_PACKAGE_NAME             (IIS_MD_HTTP_BASE+107)
#define MD_APP_OOP_RECOVER_LIMIT        (IIS_MD_HTTP_BASE+110)
#define MD_APP_PERIODIC_RESTART_TIME    (IIS_MD_HTTP_BASE+111)
#define MD_APP_PERIODIC_RESTART_REQUESTS  (IIS_MD_HTTP_BASE+112)
#define MD_APP_PERIODIC_RESTART_SCHEDULE  (IIS_MD_HTTP_BASE+113)
#define MD_APP_SHUTDOWN_TIME_LIMIT      (IIS_MD_HTTP_BASE+114)


#define MD_ADMIN_INSTANCE               (IIS_MD_HTTP_BASE+115)
// This is only used by setup & UI
#define MD_NOT_DELETABLE                (IIS_MD_HTTP_BASE+116)


#define MD_APP_TRACE_URL_LIST           (IIS_MD_HTTP_BASE+118)
#define MD_CENTRAL_W3C_LOGGING_ENABLED  (IIS_MD_HTTP_BASE+119)


#define MD_CUSTOM_ERROR_DESC            (IIS_MD_HTTP_BASE+120)

//
//  Client Access License parameters
//
#define MD_CAL_VC_PER_CONNECT           (IIS_MD_HTTP_BASE+130)
#define MD_CAL_AUTH_RESERVE_TIMEOUT     (IIS_MD_HTTP_BASE+131)
#define MD_CAL_SSL_RESERVE_TIMEOUT      (IIS_MD_HTTP_BASE+132)
#define MD_CAL_W3_ERROR                 (IIS_MD_HTTP_BASE+133)

//
// CPU Accounting and Throttling Properties
//

//
// The enabled flags are per Application or CGI
//

#define MD_CPU_CGI_ENABLED              (IIS_MD_HTTP_BASE+140)
#define MD_CPU_APP_ENABLED              (IIS_MD_HTTP_BASE+141)
#define MD_CPU_LIMITS_ENABLED           (IIS_MD_HTTP_BASE+143)

#define MD_CPU_RESET_INTERVAL           (IIS_MD_HTTP_BASE+144)

#define MD_CPU_LOGGING_INTERVAL         (IIS_MD_HTTP_BASE+145)
#define MD_CPU_LOGGING_OPTIONS          (IIS_MD_HTTP_BASE+146)
#define MD_CPU_CGI_LIMIT                (IIS_MD_HTTP_BASE+148)
#define MD_CPU_LIMIT_LOGEVENT           (IIS_MD_HTTP_BASE+149)
#define MD_CPU_LIMIT_PRIORITY           (IIS_MD_HTTP_BASE+150)
#define MD_CPU_LIMIT_PROCSTOP           (IIS_MD_HTTP_BASE+151)
#define MD_CPU_LIMIT_PAUSE              (IIS_MD_HTTP_BASE+152)


#define MD_SET_HOST_NAME                (IIS_MD_HTTP_BASE+154)

//
// Valid values for CPU Accounting's MD_CPU_LOGGING_OPTIONS field
//

#define MD_CPU_DISABLE_ALL_LOGGING      0x0
#define MD_CPU_ENABLE_ALL_PROC_LOGGING  0x1
#define MD_CPU_ENABLE_CGI_LOGGING       0x2
#define MD_CPU_ENABLE_APP_LOGGING       0x4

//
// Valid values for CPU Accounting's MD_CPU_LOGGING_MASK field
// This defines which fields will be logged
//

#define MD_CPU_ENABLE_EVENT             0x01
#define MD_CPU_ENABLE_PROC_TYPE         0x02
#define MD_CPU_ENABLE_USER_TIME         0x04
#define MD_CPU_ENABLE_KERNEL_TIME       0x08
#define MD_CPU_ENABLE_PAGE_FAULTS       0x10
#define MD_CPU_ENABLE_TOTAL_PROCS       0x20
#define MD_CPU_ENABLE_ACTIVE_PROCS      0x40
#define MD_CPU_ENABLE_TERMINATED_PROCS  0x80

#define MD_CPU_ENABLE_LOGGING           0x80000000


//
//  Site Server properties
//


//
// Properties to disable/restrict request handlers.
//

#define MD_ISAPI_RESTRICTION_LIST           (IIS_MD_HTTP_BASE+163)
#define MD_CGI_RESTRICTION_LIST             (IIS_MD_HTTP_BASE+164)
#define MD_RESTRICTION_LIST_CUSTOM_DESC     (IIS_MD_HTTP_BASE+165)
#define MD_SECURITY_SETUP_REQUIRED          (IIS_MD_HTTP_BASE+166)
#define MD_APP_DEPENDENCIES                 (IIS_MD_HTTP_BASE+167)
#define MD_WEB_SVC_EXT_RESTRICTION_LIST     (IIS_MD_HTTP_BASE+168)

#define MD_MD_SERVER_SS_AUTH_MAPPING        (IIS_MD_HTTP_BASE+200)

//
// valid values for MD_CERT_CHECK_MODE
//

#define MD_CERT_NO_REVOC_CHECK                   0x00000001
#define MD_CERT_CACHE_RETRIEVAL_ONLY             0x00000002
#define MD_CERT_CHECK_REVOCATION_FRESHNESS_TIME  0x00000004
#define MD_CERT_NO_USAGE_CHECK                   0x00010000

//
// HTTP Compression properties.  All are global and unheritable.
//

#define MD_HC_COMPRESSION_DIRECTORY             (IIS_MD_HTTP_BASE+210)
#define MD_HC_CACHE_CONTROL_HEADER              (IIS_MD_HTTP_BASE+211)
#define MD_HC_EXPIRES_HEADER                    (IIS_MD_HTTP_BASE+212)
#define MD_HC_DO_DYNAMIC_COMPRESSION            (IIS_MD_HTTP_BASE+213)
#define MD_HC_DO_STATIC_COMPRESSION             (IIS_MD_HTTP_BASE+214)
#define MD_HC_DO_ON_DEMAND_COMPRESSION          (IIS_MD_HTTP_BASE+215)
#define MD_HC_DO_DISK_SPACE_LIMITING            (IIS_MD_HTTP_BASE+216)
#define MD_HC_NO_COMPRESSION_FOR_HTTP_10        (IIS_MD_HTTP_BASE+217)
#define MD_HC_NO_COMPRESSION_FOR_PROXIES        (IIS_MD_HTTP_BASE+218)
#define MD_HC_NO_COMPRESSION_FOR_RANGE          (IIS_MD_HTTP_BASE+219)
#define MD_HC_SEND_CACHE_HEADERS                (IIS_MD_HTTP_BASE+220)
#define MD_HC_MAX_DISK_SPACE_USAGE              (IIS_MD_HTTP_BASE+221)
#define MD_HC_IO_BUFFER_SIZE                    (IIS_MD_HTTP_BASE+222)
#define MD_HC_COMPRESSION_BUFFER_SIZE           (IIS_MD_HTTP_BASE+223)
#define MD_HC_MAX_QUEUE_LENGTH                  (IIS_MD_HTTP_BASE+224)
#define MD_HC_FILES_DELETED_PER_DISK_FREE       (IIS_MD_HTTP_BASE+225)
#define MD_HC_MIN_FILE_SIZE_FOR_COMP            (IIS_MD_HTTP_BASE+226)
#define MD_HC_COMPRESSION_DLL                   (IIS_MD_HTTP_BASE+237)
#define MD_HC_FILE_EXTENSIONS                   (IIS_MD_HTTP_BASE+238)
#define MD_HC_MIME_TYPE                         (IIS_MD_HTTP_BASE+239)
#define MD_HC_PRIORITY                          (IIS_MD_HTTP_BASE+240)
#define MD_HC_DYNAMIC_COMPRESSION_LEVEL         (IIS_MD_HTTP_BASE+241)
#define MD_HC_ON_DEMAND_COMP_LEVEL              (IIS_MD_HTTP_BASE+242)
#define MD_HC_CREATE_FLAGS                      (IIS_MD_HTTP_BASE+243)
#define MD_HC_SCRIPT_FILE_EXTENSIONS            (IIS_MD_HTTP_BASE+244)

#define MD_HC_DO_NAMESPACE_DYNAMIC_COMPRESSION  (IIS_MD_HTTP_BASE+255)
#define MD_HC_DO_NAMESPACE_STATIC_COMPRESSION   (IIS_MD_HTTP_BASE+256)

//
// Generic property indicating a failure status code - Can be used under
// any component that can fail (virtual directory, filters, applications etc)
//

#define MD_WIN32_ERROR                          (IIS_MD_SERVER_BASE+99 )

//
// Virtual root properties - note MD_ACCESS_PERM is also generally set at
// the virtual directory.  These are used for both HTTP and FTP
//

#define IIS_MD_VR_BASE                  3000

#define MD_VR_PATH                      (IIS_MD_VR_BASE+1 )
#define MD_VR_USERNAME                  (IIS_MD_VR_BASE+2 )
#define MD_VR_PASSWORD                  (IIS_MD_VR_BASE+3 )
#define MD_VR_PASSTHROUGH               (IIS_MD_VR_BASE+6 )
#define MD_VR_NO_CACHE                  (IIS_MD_VR_BASE+7 )
#define MD_VR_IGNORE_TRANSLATE          (IIS_MD_VR_BASE+8 )


//
//  Logging related attributes
//

#define IIS_MD_LOG_BASE                     4000

#define MD_LOG_TYPE                         (IIS_MD_LOG_BASE+0  )
#define MD_LOGFILE_DIRECTORY                (IIS_MD_LOG_BASE+1  )
#define MD_LOG_UNUSED1                      (IIS_MD_LOG_BASE+2  )
#define MD_LOGFILE_PERIOD                   (IIS_MD_LOG_BASE+3  )
#define MD_LOGFILE_TRUNCATE_SIZE            (IIS_MD_LOG_BASE+4  )
#define MD_LOG_PLUGIN_MOD_ID                (IIS_MD_LOG_BASE+5  )
#define MD_LOG_PLUGIN_UI_ID                 (IIS_MD_LOG_BASE+6  )
#define MD_LOGSQL_DATA_SOURCES              (IIS_MD_LOG_BASE+7  )
#define MD_LOGSQL_TABLE_NAME                (IIS_MD_LOG_BASE+8  )
#define MD_LOGSQL_USER_NAME                 (IIS_MD_LOG_BASE+9  )
#define MD_LOGSQL_PASSWORD                  (IIS_MD_LOG_BASE+10 )
#define MD_LOG_PLUGIN_ORDER                 (IIS_MD_LOG_BASE+11 )
#define MD_LOG_PLUGINS_AVAILABLE            (IIS_MD_LOG_BASE+12 )
#define MD_LOGEXT_FIELD_MASK                (IIS_MD_LOG_BASE+13 )
#define MD_LOGEXT_FIELD_MASK2               (IIS_MD_LOG_BASE+14 )

//
// Allow W3C logging file naming and rollover based on Local Time
//

#define MD_LOGFILE_LOCALTIME_ROLLOVER       (IIS_MD_LOG_BASE+15 )

#define IIS_MD_LOG_LAST                     MD_LOGFILE_LOCALTIME_ROLLOVER

//
// Global Flag to denote that IIS will generate one centralized
// binary log file rather than a separate file per web site
//
#define MD_GLOBAL_BINARY_LOGGING_ENABLED    (IIS_MD_LOG_BASE+16 )

//
// Log type
//

#define MD_LOG_TYPE_DISABLED            0
#define MD_LOG_TYPE_ENABLED             1

//
// LOGGING values
//

#define MD_LOGFILE_PERIOD_NONE          0
#define MD_LOGFILE_PERIOD_MAXSIZE       0
#define MD_LOGFILE_PERIOD_DAILY         1
#define MD_LOGFILE_PERIOD_WEEKLY        2
#define MD_LOGFILE_PERIOD_MONTHLY       3
#define MD_LOGFILE_PERIOD_HOURLY        4

//
// Field masks for extended logging
// Fields are logged in order of increasing mask value
//

#define MD_EXTLOG_DATE                  0x00000001
#define MD_EXTLOG_TIME                  0x00000002
#define MD_EXTLOG_CLIENT_IP             0x00000004
#define MD_EXTLOG_USERNAME              0x00000008
#define MD_EXTLOG_SITE_NAME             0x00000010
#define MD_EXTLOG_COMPUTER_NAME         0x00000020
#define MD_EXTLOG_SERVER_IP             0x00000040
#define MD_EXTLOG_METHOD                0x00000080
#define MD_EXTLOG_URI_STEM              0x00000100
#define MD_EXTLOG_URI_QUERY             0x00000200
#define MD_EXTLOG_HTTP_STATUS           0x00000400
#define MD_EXTLOG_WIN32_STATUS          0x00000800
#define MD_EXTLOG_BYTES_SENT            0x00001000
#define MD_EXTLOG_BYTES_RECV            0x00002000
#define MD_EXTLOG_TIME_TAKEN            0x00004000
#define MD_EXTLOG_SERVER_PORT           0x00008000
#define MD_EXTLOG_USER_AGENT            0x00010000
#define MD_EXTLOG_COOKIE                0x00020000
#define MD_EXTLOG_REFERER               0x00040000
#define MD_EXTLOG_PROTOCOL_VERSION      0x00080000
#define MD_EXTLOG_HOST                  0x00100000
#define MD_EXTLOG_HTTP_SUB_STATUS       0x00200000


#define MD_DEFAULT_EXTLOG_FIELDS        (MD_EXTLOG_CLIENT_IP | \
                                         MD_EXTLOG_TIME      | \
                                         MD_EXTLOG_METHOD    | \
                                         MD_EXTLOG_URI_STEM  | \
                                         MD_EXTLOG_HTTP_STATUS | \
                                         MD_EXTLOG_HTTP_SUB_STATUS )

//
// Custom Logging related attributes
//

#define IIS_MD_LOGCUSTOM_BASE           4500

//
// Custom Logging configuration attributes
//

#define MD_LOGCUSTOM_PROPERTY_NAME      (IIS_MD_LOGCUSTOM_BASE+1 )
#define MD_LOGCUSTOM_PROPERTY_HEADER    (IIS_MD_LOGCUSTOM_BASE+2 )
#define MD_LOGCUSTOM_PROPERTY_ID        (IIS_MD_LOGCUSTOM_BASE+3 )
#define MD_LOGCUSTOM_PROPERTY_MASK      (IIS_MD_LOGCUSTOM_BASE+4 )
#define MD_LOGCUSTOM_PROPERTY_DATATYPE  (IIS_MD_LOGCUSTOM_BASE+5 )
#define MD_LOGCUSTOM_SERVICES_STRING    (IIS_MD_LOGCUSTOM_BASE+6 )      // MultiSZ List of services that the property is applicable to.

#define MD_CPU_LOGGING_MASK             (IIS_MD_LOGCUSTOM_BASE+7 )

#define MD_LOGCUSTOM_PROPERTY_NODE_ID   (IIS_MD_LOGCUSTOM_BASE+8 )


#define IIS_MD_LOGCUSTOM_LAST           MD_LOGCUSTOM_PROPERTY_NODE_ID

//
// Valid values for Custom Logging's MD_LOGCUSTOM_PROPERTY_DATATYPE field
//

#define MD_LOGCUSTOM_DATATYPE_INT       0
#define MD_LOGCUSTOM_DATATYPE_UINT      1
#define MD_LOGCUSTOM_DATATYPE_LONG      2
#define MD_LOGCUSTOM_DATATYPE_ULONG     3
#define MD_LOGCUSTOM_DATATYPE_FLOAT     4
#define MD_LOGCUSTOM_DATATYPE_DOUBLE    5
#define MD_LOGCUSTOM_DATATYPE_LPSTR     6
#define MD_LOGCUSTOM_DATATYPE_LPWSTR    7


//
//  ISAPI Filter Notification Flags
//

#define MD_NOTIFY_SECURE_PORT           0x00000001
#define MD_NOTIFY_NONSECURE_PORT        0x00000002

#define MD_NOTIFY_READ_RAW_DATA         0x00008000
#define MD_NOTIFY_PREPROC_HEADERS       0x00004000
#define MD_NOTIFY_AUTHENTICATION        0x00002000
#define MD_NOTIFY_URL_MAP               0x00001000
#define MD_NOTIFY_ACCESS_DENIED         0x00000800
#define MD_NOTIFY_SEND_RESPONSE         0x00000040
#define MD_NOTIFY_SEND_RAW_DATA         0x00000400
#define MD_NOTIFY_LOG                   0x00000200
#define MD_NOTIFY_END_OF_REQUEST        0x00000080
#define MD_NOTIFY_END_OF_NET_SESSION    0x00000100
#define MD_NOTIFY_AUTH_COMPLETE         0x04000000
//
//  ISAPI Filter ordering flags
//

#define MD_NOTIFY_ORDER_HIGH            0x00080000
#define MD_NOTIFY_ORDER_MEDIUM          0x00040000
#define MD_NOTIFY_ORDER_LOW             0x00020000
#define MD_NOTIFY_ORDER_DEFAULT         MD_NOTIFY_ORDER_LOW

#define MD_NOTIFY_ORDER_MASK            (MD_NOTIFY_ORDER_HIGH   |    \
                                         MD_NOTIFY_ORDER_MEDIUM |    \
                                         MD_NOTIFY_ORDER_LOW)


//
//  These are FTP specific properties
//

#define IIS_MD_FTP_BASE                 5000

#define MD_EXIT_MESSAGE                 (IIS_MD_FTP_BASE+1  )
#define MD_GREETING_MESSAGE             (IIS_MD_FTP_BASE+2  )
#define MD_MAX_CLIENTS_MESSAGE          (IIS_MD_FTP_BASE+3  )
#define MD_MSDOS_DIR_OUTPUT             (IIS_MD_FTP_BASE+4  )
#define MD_ALLOW_ANONYMOUS              (IIS_MD_FTP_BASE+5  )
#define MD_ANONYMOUS_ONLY               (IIS_MD_FTP_BASE+6  )
#define MD_LOG_ANONYMOUS                (IIS_MD_FTP_BASE+7  )
#define MD_LOG_NONANONYMOUS             (IIS_MD_FTP_BASE+8  )
#define MD_ALLOW_REPLACE_ON_RENAME      (IIS_MD_FTP_BASE+9  )
#define MD_SHOW_4_DIGIT_YEAR            (IIS_MD_FTP_BASE+10 )
#define MD_BANNER_MESSAGE               (IIS_MD_FTP_BASE+11 )
#define MD_USER_ISOLATION               (IIS_MD_FTP_BASE+12 )
#define MD_FTP_LOG_IN_UTF_8             (IIS_MD_FTP_BASE+13 )
#define MD_AD_CONNECTIONS_USERNAME      (IIS_MD_FTP_BASE+14 )
#define MD_AD_CONNECTIONS_PASSWORD      (IIS_MD_FTP_BASE+15 )
#define MD_PASSIVE_PORT_RANGE           (IIS_MD_FTP_BASE+16 )
#define MD_SUPPRESS_DEFAULT_BANNER      (IIS_MD_FTP_BASE+17 )
#define MD_FTP_PASV_RESPONSE_IP         (IIS_MD_FTP_BASE+18 )
#define MD_FTP_KEEP_PARTIAL_UPLOADS     (IIS_MD_FTP_BASE+19 )
#define MD_FTP_UTF8_FILE_NAMES          (IIS_MD_FTP_BASE+20 )

//
//  FTPS properties.
//

#define MD_FTPS_SECURE_CONTROL_CHANNEL  (IIS_MD_FTP_BASE+50 )
#define MD_FTPS_SECURE_DATA_CHANNEL     (IIS_MD_FTP_BASE+51 )
#define MD_FTPS_SECURE_ANONYMOUS        (IIS_MD_FTP_BASE+52 )
#define MD_FTPS_128_BITS                (IIS_MD_FTP_BASE+53 )
#define MD_FTPS_ALLOW_CCC               (IIS_MD_FTP_BASE+54 )


//
//  These are SSL specific properties
//

#define IIS_MD_SSL_BASE                 5500

#define MD_SSL_PUBLIC_KEY               ( IIS_MD_SSL_BASE+0 )
#define MD_SSL_PRIVATE_KEY              ( IIS_MD_SSL_BASE+1 )
#define MD_SSL_KEY_PASSWORD             ( IIS_MD_SSL_BASE+2 )
#define MD_SSL_KEY_REQUEST              ( IIS_MD_SSL_BASE+3 )

//
// These are server certificate properties
//
//
// These are Certificate Trust List properties
//

//
// Metabase property that defines whether to use DS mapper or not
//
#define MD_SSL_USE_DS_MAPPER            ( IIS_MD_SSL_BASE+19 )


#define MD_SSL_ALWAYS_NEGO_CLIENT_CERT  ( IIS_MD_SSL_BASE+21 )

//
// Metabase properties that are used by the CertWiz ActiveX control, that
// is used for the Certificate/CTL UI management tool
//

//
// Metabase properties used for Fortezza certificates
//

//
// Metabase properties that are used by the CertWiz ActiveX control to keep
// track of the user's entry history, and whether DEBUG is enabled.  We keep
// these private properties on a per VS basis.
//

//  File and Directory related properties - these should be added in the
//  metabase with a user type of IIS_MD_UT_FILE
//

#define IIS_MD_FILE_PROP_BASE           6000

#define MD_AUTHORIZATION                (IIS_MD_FILE_PROP_BASE )
#define MD_REALM                        (IIS_MD_FILE_PROP_BASE+1 )
#define MD_HTTP_EXPIRES                 (IIS_MD_FILE_PROP_BASE+2 )
#define MD_HTTP_PICS                    (IIS_MD_FILE_PROP_BASE+3 )
#define MD_HTTP_CUSTOM                  (IIS_MD_FILE_PROP_BASE+4 )
#define MD_DIRECTORY_BROWSING           (IIS_MD_FILE_PROP_BASE+5 )
#define MD_DEFAULT_LOAD_FILE            (IIS_MD_FILE_PROP_BASE+6 )
#define MD_CUSTOM_ERROR                 (IIS_MD_FILE_PROP_BASE+8 )
#define MD_FOOTER_DOCUMENT              (IIS_MD_FILE_PROP_BASE+9 )
#define MD_FOOTER_ENABLED               (IIS_MD_FILE_PROP_BASE+10 )
#define MD_HTTP_REDIRECT                (IIS_MD_FILE_PROP_BASE+11 )
#define MD_DEFAULT_LOGON_DOMAIN         (IIS_MD_FILE_PROP_BASE+12 )
#define MD_LOGON_METHOD                 (IIS_MD_FILE_PROP_BASE+13 )
#define MD_SCRIPT_MAPS                  (IIS_MD_FILE_PROP_BASE+14 )
#define MD_MIME_MAP                     (IIS_MD_FILE_PROP_BASE+15 )
#define MD_ACCESS_PERM                  (IIS_MD_FILE_PROP_BASE+16 )
#define MD_IP_SEC                       (IIS_MD_FILE_PROP_BASE+19 )
#define MD_ANONYMOUS_USER_NAME          (IIS_MD_FILE_PROP_BASE+20 )
#define MD_ANONYMOUS_PWD                (IIS_MD_FILE_PROP_BASE+21 )
#define MD_ANONYMOUS_USE_SUBAUTH        (IIS_MD_FILE_PROP_BASE+22 )
#define MD_DONT_LOG                     (IIS_MD_FILE_PROP_BASE+23 )
#define MD_ADMIN_ACL                    (IIS_MD_FILE_PROP_BASE+27 )
#define MD_SSI_EXEC_DISABLED            (IIS_MD_FILE_PROP_BASE+28 )
#define MD_DO_REVERSE_DNS               (IIS_MD_FILE_PROP_BASE+29 )
#define MD_SSL_ACCESS_PERM              (IIS_MD_FILE_PROP_BASE+30 )
#define MD_AUTHORIZATION_PERSISTENCE    (IIS_MD_FILE_PROP_BASE+31 )
#define MD_NTAUTHENTICATION_PROVIDERS   (IIS_MD_FILE_PROP_BASE+32 )
#define MD_SCRIPT_TIMEOUT               (IIS_MD_FILE_PROP_BASE+33 )
#define MD_CACHE_EXTENSIONS             (IIS_MD_FILE_PROP_BASE+34 )
#define MD_CREATE_PROCESS_AS_USER       (IIS_MD_FILE_PROP_BASE+35 )
#define MD_CREATE_PROC_NEW_CONSOLE      (IIS_MD_FILE_PROP_BASE+36 )
#define MD_POOL_IDC_TIMEOUT             (IIS_MD_FILE_PROP_BASE+37 )
#define MD_ALLOW_KEEPALIVES             (IIS_MD_FILE_PROP_BASE+38 )
#define MD_IS_CONTENT_INDEXED           (IIS_MD_FILE_PROP_BASE+39 )
#define MD_CC_NO_CACHE                  (IIS_MD_FILE_PROP_BASE+41 )
#define MD_CC_MAX_AGE                   (IIS_MD_FILE_PROP_BASE+42 )
#define MD_CC_OTHER                     (IIS_MD_FILE_PROP_BASE+43 )
#define MD_REDIRECT_HEADERS             (IIS_MD_FILE_PROP_BASE+44 )
#define MD_UPLOAD_READAHEAD_SIZE        (IIS_MD_FILE_PROP_BASE+45 )
#define MD_PUT_READ_SIZE                (IIS_MD_FILE_PROP_BASE+46 )
#define MD_USE_DIGEST_SSP               (IIS_MD_FILE_PROP_BASE+47 )
#define MD_ENABLE_URL_AUTHORIZATION     (IIS_MD_FILE_PROP_BASE+48 )
#define MD_URL_AUTHORIZATION_STORE_NAME (IIS_MD_FILE_PROP_BASE+49 )
#define MD_URL_AUTHORIZATION_SCOPE_NAME (IIS_MD_FILE_PROP_BASE+50 )
#define MD_MAX_REQUEST_ENTITY_ALLOWED   (IIS_MD_FILE_PROP_BASE+51 )
#define MD_PASSPORT_REQUIRE_AD_MAPPING  (IIS_MD_FILE_PROP_BASE+52 )
#define MD_URL_AUTHORIZATION_IMPERSONATION_LEVEL    (IIS_MD_FILE_PROP_BASE+53 )
#define MD_HTTP_FORWARDER_CUSTOM        (IIS_MD_FILE_PROP_BASE+54 )
#define MD_CUSTOM_DEPLOYMENT_DATA       (IIS_MD_FILE_PROP_BASE+55 )
#define MD_HTTPERRORS_EXISTING_RESPONSE (IIS_MD_FILE_PROP_BASE+56 )


#define ASP_MD_SERVER_BASE                  7000

#define MD_ASP_BUFFERINGON                  (ASP_MD_SERVER_BASE + 0)
#define MD_ASP_LOGERRORREQUESTS             (ASP_MD_SERVER_BASE + 1)
#define MD_ASP_SCRIPTERRORSSENTTOBROWSER    (ASP_MD_SERVER_BASE + 2)
#define MD_ASP_SCRIPTERRORMESSAGE           (ASP_MD_SERVER_BASE + 3)
#define MD_ASP_SCRIPTFILECACHESIZE          (ASP_MD_SERVER_BASE + 4)
#define MD_ASP_SCRIPTENGINECACHEMAX         (ASP_MD_SERVER_BASE + 5)
#define MD_ASP_SCRIPTTIMEOUT                (ASP_MD_SERVER_BASE + 6)
#define MD_ASP_SESSIONTIMEOUT               (ASP_MD_SERVER_BASE + 7)
#define MD_ASP_ENABLEPARENTPATHS            (ASP_MD_SERVER_BASE + 8)
#define MD_ASP_MEMFREEFACTOR                (ASP_MD_SERVER_BASE + 9)    // OBSOLETE
#define MD_ASP_MINUSEDBLOCKS                (ASP_MD_SERVER_BASE + 10)   // OBSOLETE
#define MD_ASP_ALLOWSESSIONSTATE            (ASP_MD_SERVER_BASE + 11)
#define MD_ASP_SCRIPTLANGUAGE               (ASP_MD_SERVER_BASE + 12)
#define MD_ASP_QUEUETIMEOUT                 (ASP_MD_SERVER_BASE + 13)
#define MD_ASP_ALLOWOUTOFPROCCOMPONENTS     (ASP_MD_SERVER_BASE + 14)
#define MD_ASP_ALLOWOUTOFPROCCMPNTS         (MD_ASP_ALLOWOUTOFPROCCOMPONENTS)   // Deprecated.  Use MD_ASP_ALLOWOUTOFPROCCMPNTS
#define MD_ASP_EXCEPTIONCATCHENABLE         (ASP_MD_SERVER_BASE + 15)
#define MD_ASP_CODEPAGE                     (ASP_MD_SERVER_BASE + 16)
#define MD_ASP_SCRIPTLANGUAGELIST           (ASP_MD_SERVER_BASE + 17)
#define MD_ASP_ENABLESERVERDEBUG            (ASP_MD_SERVER_BASE + 18)
#define MD_ASP_ENABLECLIENTDEBUG            (ASP_MD_SERVER_BASE + 19)
#define MD_ASP_TRACKTHREADINGMODEL          (ASP_MD_SERVER_BASE + 20)
// added for IIS 5.0
#define MD_ASP_ENABLEASPHTMLFALLBACK        (ASP_MD_SERVER_BASE + 21)
#define MD_ASP_ENABLECHUNKEDENCODING        (ASP_MD_SERVER_BASE + 22)
#define MD_ASP_ENABLETYPELIBCACHE           (ASP_MD_SERVER_BASE + 23)
#define MD_ASP_ERRORSTONTLOG                (ASP_MD_SERVER_BASE + 24)
#define MD_ASP_PROCESSORTHREADMAX           (ASP_MD_SERVER_BASE + 25)
#define MD_ASP_REQEUSTQUEUEMAX              (ASP_MD_SERVER_BASE + 26)
#define MD_ASP_ENABLEAPPLICATIONRESTART     (ASP_MD_SERVER_BASE + 27)
#define MD_ASP_QUEUECONNECTIONTESTTIME      (ASP_MD_SERVER_BASE + 28)
#define MD_ASP_SESSIONMAX                   (ASP_MD_SERVER_BASE + 29)

// thread gate
#define MD_ASP_THREADGATEENABLED            (ASP_MD_SERVER_BASE + 30)
#define MD_ASP_THREADGATETIMESLICE          (ASP_MD_SERVER_BASE + 31)
#define MD_ASP_THREADGATESLEEPDELAY         (ASP_MD_SERVER_BASE + 32)
#define MD_ASP_THREADGATESLEEPMAX           (ASP_MD_SERVER_BASE + 33)
#define MD_ASP_THREADGATELOADLOW            (ASP_MD_SERVER_BASE + 34)
#define MD_ASP_THREADGATELOADHIGH           (ASP_MD_SERVER_BASE + 35)

// added IIS5.1

// persist template cache
#define MD_ASP_DISKTEMPLATECACHEDIRECTORY   (ASP_MD_SERVER_BASE + 36)
#define MD_ASP_MAXDISKTEMPLATECACHEFILES    (ASP_MD_SERVER_BASE + 40)
#define MD_ASP_EXECUTEINMTA                 (ASP_MD_SERVER_BASE + 41)
#define MD_ASP_LCID                         (ASP_MD_SERVER_BASE + 42)
#define MD_ASP_KEEPSESSIONIDSECURE          (ASP_MD_SERVER_BASE + 43)

// added IIS6.0

// Services without components integration
#define MD_ASP_SERVICE_FLAGS                (ASP_MD_SERVER_BASE + 44)
#define MD_ASP_SERVICE_FLAG_TRACKER         (ASP_MD_SERVER_BASE + 45)
#define MD_ASP_SERVICE_FLAG_FUSION          (ASP_MD_SERVER_BASE + 46)
#define MD_ASP_SERVICE_FLAG_PARTITIONS      (ASP_MD_SERVER_BASE + 47)
#define MD_ASP_SERVICE_PARTITION_ID         (ASP_MD_SERVER_BASE + 48)
#define MD_ASP_SERVICE_SXS_NAME             (ASP_MD_SERVER_BASE + 49)

// Valid flags for MD_ASP_SERVICE_FLAGS property
#define MD_ASP_SERVICE_ENABLE_TRACKER       1
#define MD_ASP_SERVICE_ENABLE_SXS           2
#define MD_ASP_SERVICE_USE_PARTITION        4

// Line number calculation flag.
#define MD_ASP_CALCLINENUMBER               (ASP_MD_SERVER_BASE + 50)

#define MD_ASP_RUN_ONEND_ANON               (ASP_MD_SERVER_BASE + 51)

#define MD_ASP_BUFFER_LIMIT                 (ASP_MD_SERVER_BASE + 52)

#define MD_ASP_MAX_REQUEST_ENTITY_ALLOWED   (ASP_MD_SERVER_BASE + 53)
#define MD_ASP_MAXREQUESTENTITY             MD_ASP_MAX_REQUEST_ENTITY_ALLOWED

#define MD_ASP_ID_LAST                      (ASP_MD_SERVER_BASE + 53)

//
//  Valid values for WAM
//
#define WAM_MD_SERVER_BASE                  7500

#define MD_WAM_USER_NAME                    (WAM_MD_SERVER_BASE+1)
#define MD_WAM_PWD                          (WAM_MD_SERVER_BASE+2)

//
//  Valid values for WEBDAV
//
#define WEBDAV_MD_SERVER_BASE               8500

#define MD_WEBDAV_MAX_ATTRIBUTES_PER_ELEMENT   (WEBDAV_MD_SERVER_BASE+1)

// added IIS6

//
//  Valid values for APP POOL
//

#define IIS_MD_APPPOOL_BASE 9000

#define MD_APPPOOL_PERIODIC_RESTART_TIME              (IIS_MD_APPPOOL_BASE + 1)
#define MD_APPPOOL_PERIODIC_RESTART_REQUEST_COUNT     (IIS_MD_APPPOOL_BASE + 2)
#define MD_APPPOOL_MAX_PROCESS_COUNT                  (IIS_MD_APPPOOL_BASE + 3)
#define MD_APPPOOL_PINGING_ENABLED                    (IIS_MD_APPPOOL_BASE + 4)
#define MD_APPPOOL_IDLE_TIMEOUT                       (IIS_MD_APPPOOL_BASE + 5)
#define MD_APPPOOL_RAPID_FAIL_PROTECTION_ENABLED      (IIS_MD_APPPOOL_BASE + 6)
#define MD_APPPOOL_SMP_AFFINITIZED                    (IIS_MD_APPPOOL_BASE + 7)
#define MD_APPPOOL_SMP_AFFINITIZED_PROCESSOR_MASK     (IIS_MD_APPPOOL_BASE + 8)
#define MD_APPPOOL_ORPHAN_PROCESSES_FOR_DEBUGGING     (IIS_MD_APPPOOL_BASE + 9)
#define MD_APPPOOL_STARTUP_TIMELIMIT                  (IIS_MD_APPPOOL_BASE + 11)
#define MD_APPPOOL_SHUTDOWN_TIMELIMIT                 (IIS_MD_APPPOOL_BASE + 12)
#define MD_APPPOOL_PING_INTERVAL                      (IIS_MD_APPPOOL_BASE + 13)
#define MD_APPPOOL_PING_RESPONSE_TIMELIMIT            (IIS_MD_APPPOOL_BASE + 14)
#define MD_APPPOOL_DISALLOW_OVERLAPPING_ROTATION      (IIS_MD_APPPOOL_BASE + 15)
#define MD_APPPOOL_UL_APPPOOL_QUEUE_LENGTH            (IIS_MD_APPPOOL_BASE + 17)
#define MD_APPPOOL_DISALLOW_ROTATION_ON_CONFIG_CHANGE (IIS_MD_APPPOOL_BASE + 18)
#define MD_APPPOOL_PERIODIC_RESTART_SCHEDULE          (IIS_MD_APPPOOL_BASE + 20)
#define MD_APPPOOL_IDENTITY_TYPE                      (IIS_MD_APPPOOL_BASE + 21)
#define MD_CPU_ACTION                                 (IIS_MD_APPPOOL_BASE + 22)
#define MD_CPU_LIMIT                                  (IIS_MD_APPPOOL_BASE + 23)
#define MD_APPPOOL_PERIODIC_RESTART_MEMORY            (IIS_MD_APPPOOL_BASE + 24)
#define MD_APPPOOL_COMMAND                            (IIS_MD_APPPOOL_BASE + 26)
#define MD_APPPOOL_STATE                              (IIS_MD_APPPOOL_BASE + 27)
#define MD_APPPOOL_AUTO_START                         (IIS_MD_APPPOOL_BASE + 28)
#define MD_RAPID_FAIL_PROTECTION_INTERVAL             (IIS_MD_APPPOOL_BASE + 29)
#define MD_RAPID_FAIL_PROTECTION_MAX_CRASHES          (IIS_MD_APPPOOL_BASE + 30)
#define MD_APPPOOL_ORPHAN_ACTION_EXE                  (IIS_MD_APPPOOL_BASE + 31)
#define MD_APPPOOL_ORPHAN_ACTION_PARAMS               (IIS_MD_APPPOOL_BASE + 32)
#define MB_DONT_IMPERSONATE                           (IIS_MD_APPPOOL_BASE + 33)

//
// Load balancer properties
//
#define MD_LOAD_BALANCER_CAPABILITIES                 (IIS_MD_APPPOOL_BASE + 34)

//
//  Valid values for APP POOL
//
#define MD_APPPOOL_AUTO_SHUTDOWN_EXE                  (IIS_MD_APPPOOL_BASE + 35)
#define MD_APPPOOL_AUTO_SHUTDOWN_PARAMS               (IIS_MD_APPPOOL_BASE + 36)
#define MD_APP_POOL_LOG_EVENT_ON_RECYCLE              (IIS_MD_APPPOOL_BASE + 37)
#define MD_APPPOOL_PERIODIC_RESTART_PRIVATE_MEMORY    (IIS_MD_APPPOOL_BASE + 38)
#define MD_APPPOOL_MANAGED_RUNTIME_VERSION            (IIS_MD_APPPOOL_BASE + 39)
#define MD_APPPOOL_32_BIT_APP_ON_WIN64                (IIS_MD_APPPOOL_BASE + 40)
#define MD_APPPOOL_MANAGED_PIPELINE_MODE              (IIS_MD_APPPOOL_BASE + 41)
#define MD_APP_POOL_LOG_EVENT_ON_PROCESSMODEL         (IIS_MD_APPPOOL_BASE + 42)
#define MD_APPPOOL_EMULATION_ON_WINARM64              (IIS_MD_APPPOOL_BASE + 43)

//
// Valid values for MD_APP_POOL_LOG_EVENT_ON_PROCESSMODEL
//
#define MD_APP_POOL_PROCESSMODEL_IDLE_TIMEOUT         1

//
// Valid values for MD_APP_POOL_LOG_EVENT_ON_RECYCLE
//
#define MD_APP_POOL_RECYCLE_TIME                      1
#define MD_APP_POOL_RECYCLE_REQUESTS                  2
#define MD_APP_POOL_RECYCLE_SCHEDULE                  4
#define MD_APP_POOL_RECYCLE_MEMORY                    8
#define MD_APP_POOL_RECYCLE_ISAPI_UNHEALTHY           16
#define MD_APP_POOL_RECYCLE_ON_DEMAND                 32
#define MD_APP_POOL_RECYCLE_CONFIG_CHANGE             64
#define MD_APP_POOL_RECYCLE_PRIVATE_MEMORY            128

//
// Valid values for MD_CPU_ACTION
//

#define MD_CPU_NO_ACTION                              0
#define MD_CPU_KILL_W3WP                              1
#define MD_CPU_TRACE                                  2
#define MD_CPU_THROTTLE                               3

//
// Valid values for MD_APPPOOL_COMMAND
//

#define MD_APPPOOL_COMMAND_START                      1
#define MD_APPPOOL_COMMAND_STOP                       2

//
// Valid values for MD_APPPOOL_STATE
//

#define MD_APPPOOL_STATE_STARTING                     1
#define MD_APPPOOL_STATE_STARTED                      2
#define MD_APPPOOL_STATE_STOPPING                     3
#define MD_APPPOOL_STATE_STOPPED                      4

//
// Valid values for MD_APPPOOL_IDENTITY_TYPE
//
#define MD_APPPOOL_IDENTITY_TYPE_LOCALSYSTEM          0
#define MD_APPPOOL_IDENTITY_TYPE_LOCALSERVICE         1
#define MD_APPPOOL_IDENTITY_TYPE_NETWORKSERVICE       2
#define MD_APPPOOL_IDENTITY_TYPE_SPECIFICUSER         3

//
// Valid values for MD_LOAD_BALANCER_CAPABILITIES
//
#define MD_LOAD_BALANCER_CAPABILITIES_BASIC           1
#define MD_LOAD_BALANCER_CAPABILITIES_SOPHISTICATED   2

#define IIS_MD_APP_BASE                               9100
#define MD_APP_APPPOOL_ID                             (IIS_MD_APP_BASE+1)
#define MD_APP_ALLOW_TRANSIENT_REGISTRATION           (IIS_MD_APP_BASE+2)
#define MD_APP_AUTO_START                             (IIS_MD_APP_BASE+3)
#define MD_APPPOOL_PERIODIC_RESTART_CONNECTIONS       (IIS_MD_APP_BASE+4)

//
// TODO: These are duplicate definitions. Remove them if no one is using it.
//

#define MD_APPPOOL_APPPOOL_ID                         (IIS_MD_APP_BASE + 101)
#define MD_APPPOOL_ALLOW_TRANSIENT_REGISTRATION       (IIS_MD_APP_BASE + 102)
// commented out so we can build
//#define MD_APPPOOL_AUTO_START                         (IIS_MD_APP_BASE + 103)


#define IIS_MD_GLOBAL_BASE                              9200
#define MD_MAX_GLOBAL_BANDWIDTH                         (IIS_MD_GLOBAL_BASE+1)
#define MD_MAX_GLOBAL_CONNECTIONS                       (IIS_MD_GLOBAL_BASE+2)
#define MD_GLOBAL_STANDARD_APP_MODE_ENABLED             (IIS_MD_GLOBAL_BASE+3)
#define MD_HEADER_WAIT_TIMEOUT                          (IIS_MD_GLOBAL_BASE+4)
#define MD_MIN_FILE_BYTES_PER_SEC                       (IIS_MD_GLOBAL_BASE+5)
#define MD_GLOBAL_LOG_IN_UTF_8                          (IIS_MD_GLOBAL_BASE+6)
#define MD_DEMAND_START_THRESHOLD                       (IIS_MD_GLOBAL_BASE+7)

#define MD_GLOBAL_SESSIONKEY                                 9999
#define MD_ROOT_ENABLE_EDIT_WHILE_RUNNING                    9998
#define MD_GLOBAL_CHANGE_NUMBER                              9997
#define MD_ROOT_ENABLE_HISTORY                               9996
#define MD_ROOT_MAX_HISTORY_FILES                            9995
#define MD_GLOBAL_EDIT_WHILE_RUNNING_MAJOR_VERSION_NUMBER    9994
#define MD_GLOBAL_EDIT_WHILE_RUNNING_MINOR_VERSION_NUMBER    9993
#define MD_GLOBAL_XMLSCHEMATIMESTAMP                         9992
#define MD_GLOBAL_BINSCHEMATIMESTAMP                         9991
#define MD_COMMENTS                                          9990
#define MD_LOCATION                                          9989
#define MD_MAX_ERROR_FILES                                   9988
#define MD_STOP_LISTENING                                    9987

//
//  Valid values for MD_AUTHORIZATION
//

#define MD_AUTH_ANONYMOUS               0x00000001
#define MD_AUTH_BASIC                   0x00000002
#define MD_AUTH_NT                      0x00000004    // Use NT auth provider (like NTLM)
#define MD_AUTH_MD5                     0x00000010
#define MD_AUTH_PASSPORT                0x00000040

//
//  Valid values for MD_AUTHORIZATION_PERSISTENCE
//


#define MD_AUTH_SINGLEREQUEST                   0x00000040
#define MD_AUTH_SINGLEREQUESTIFPROXY            0x00000080
#define MD_AUTH_SINGLEREQUESTALWAYSIFPROXY      0x00000100

//
//  Valid values for MD_ACCESS_PERM
//

#define MD_ACCESS_READ                  0x00000001    // Allow for Read
#define MD_ACCESS_WRITE                 0x00000002    // Allow for Write
#define MD_ACCESS_EXECUTE               0x00000004    // Allow for Execute
#define MD_ACCESS_SOURCE                0x00000010    // Apply access mask to source
#define MD_ACCESS_SCRIPT                0x00000200    // Allow for Script execution
#define MD_ACCESS_NO_REMOTE_WRITE       0x00000400    // Local host access only
#define MD_ACCESS_NO_REMOTE_READ        0x00001000    // Local host access only
#define MD_ACCESS_NO_REMOTE_EXECUTE     0x00002000    // Local host access only
#define MD_ACCESS_NO_REMOTE_SCRIPT      0x00004000    // Local host access only
#define MD_ACCESS_NO_PHYSICAL_DIR       0x00008000    // VR maps to no physical path

#define MD_NONSSL_ACCESS_MASK           (MD_ACCESS_READ|                \
                                         MD_ACCESS_WRITE|               \
                                         MD_ACCESS_EXECUTE|             \
                                         MD_ACCESS_SOURCE|              \
                                         MD_ACCESS_SCRIPT|              \
                                         MD_ACCESS_NO_REMOTE_READ|      \
                                         MD_ACCESS_NO_REMOTE_WRITE|     \
                                         MD_ACCESS_NO_REMOTE_EXECUTE|   \
                                         MD_ACCESS_NO_REMOTE_SCRIPT|    \
                                         MD_ACCESS_NO_PHYSICAL_DIR      \
                                         )
//
//  Valid values for MD_SSL_ACCESS_PERM
//

#define MD_ACCESS_SSL                   0x00000008    // Require SSL
#define MD_ACCESS_NEGO_CERT             0x00000020    // Allow client SSL certs
#define MD_ACCESS_REQUIRE_CERT          0x00000040    // Require client SSL certs
#define MD_ACCESS_MAP_CERT              0x00000080    // Map SSL cert to NT account
#define MD_ACCESS_SSL128                0x00000100    // Require 128 bit SSL

#define MD_SSL_ACCESS_MASK              (MD_ACCESS_SSL|\
                                         MD_ACCESS_NEGO_CERT|\
                                         MD_ACCESS_REQUIRE_CERT|\
                                         MD_ACCESS_MAP_CERT|\
                                         MD_ACCESS_SSL128)

#define MD_ACCESS_MASK                  0x0000ffff

//
//  Valid values for MD_DIRECTORY_BROWSING
//

#define MD_DIRBROW_SHOW_DATE            0x00000002
#define MD_DIRBROW_SHOW_TIME            0x00000004
#define MD_DIRBROW_SHOW_SIZE            0x00000008
#define MD_DIRBROW_SHOW_EXTENSION       0x00000010
#define MD_DIRBROW_LONG_DATE            0x00000020

#define MD_DIRBROW_ENABLED              0x80000000  // Allow directory browsing
#define MD_DIRBROW_LOADDEFAULT          0x40000000  // Load default doc if exists

#define MD_DIRBROW_MASK                 (MD_DIRBROW_SHOW_DATE      |    \
                                         MD_DIRBROW_SHOW_TIME      |    \
                                         MD_DIRBROW_SHOW_SIZE      |    \
                                         MD_DIRBROW_SHOW_EXTENSION |    \
                                         MD_DIRBROW_LONG_DATE      |    \
                                         MD_DIRBROW_LOADDEFAULT    |    \
                                         MD_DIRBROW_ENABLED)



//
//  Valid values for MD_LOGON_METHOD
//

#define MD_LOGON_INTERACTIVE        0
#define MD_LOGON_BATCH              1
#define MD_LOGON_NETWORK            2
#define MD_LOGON_NETWORK_CLEARTEXT  3

//
//  Valid values for MD_PASSPORT_REQUIRE_AD_MAPPING
//

#define MD_PASSPORT_NO_MAPPING      0
#define MD_PASSPORT_TRY_MAPPING     1
#define MD_PASSPORT_NEED_MAPPING    2

//
// Valid values for MD_NOTIFY_EXAUTH
//

#define MD_NOTIFEXAUTH_NTLMSSL  1

//
//  Valid values for MD_FILTER_STATE
//

#define MD_FILTER_STATE_LOADED          1
#define MD_FILTER_STATE_UNLOADED        4

//
//  Valid values for MD_SERVER_STATE
//

#define MD_SERVER_STATE_STARTING        1
#define MD_SERVER_STATE_STARTED         2
#define MD_SERVER_STATE_STOPPING        3
#define MD_SERVER_STATE_STOPPED         4
#define MD_SERVER_STATE_PAUSING         5
#define MD_SERVER_STATE_PAUSED          6
#define MD_SERVER_STATE_CONTINUING      7

//
//  Valid values for MD_SERVER_COMMAND
//

#define MD_SERVER_COMMAND_START         1
#define MD_SERVER_COMMAND_STOP          2
#define MD_SERVER_COMMAND_PAUSE         3
#define MD_SERVER_COMMAND_CONTINUE      4

//
//  Valid values for MD_SERVER_SIZE
//

#define MD_SERVER_SIZE_SMALL            0
#define MD_SERVER_SIZE_MEDIUM           1
#define MD_SERVER_SIZE_LARGE            2

//
// Valid values for MD_SERVER_CONFIG_INFO
//

#define MD_SERVER_CONFIG_SSL_40         0x00000001
#define MD_SERVER_CONFIG_SSL_128        0x00000002
#define MD_SERVER_CONFIG_ALLOW_ENCRYPT  0x00000004
#define MD_SERVER_CONFIG_AUTO_PW_SYNC   0x00000008

#define MD_SERVER_CONFIGURATION_MASK   (MD_SERVER_CONFIG_SSL_40       | \
                                        MD_SERVER_CONFIG_SSL_128      | \
                                        MD_SERVER_CONFIG_ENCRYPT      | \
                                        MD_SERVER_CONFIG_AUTO_PW_SYNC)

//
// Valid values for MD_SCRIPT_MAPS flag field
//

#define MD_SCRIPTMAPFLAG_SCRIPT                     0x00000001
#define MD_SCRIPTMAPFLAG_CHECK_PATH_INFO            0x00000004

#ifdef REMOVE   // SteveBr
//
//  Bogus value - do not use
//
#define MD_SCRIPTMAPFLAG_ALLOWED_ON_READ_DIR        0x00000001
#endif // REMOVE


//
// Valid values for MD_AUTH_CHANGE_ENABLE
//

#define MD_AUTH_CHANGE_UNSECURE     0x00000001
#define MD_AUTH_CHANGE_DISABLE      0x00000002
#define MD_AUTH_ADVNOTIFY_DISABLE   0x00000004

//
// Valid values for MD_NET_LOGON_WKS
//

#define MD_NETLOGON_WKS_NONE        0
#define MD_NETLOGON_WKS_IP          1
#define MD_NETLOGON_WKS_DNS         2

//
//  Valide substatus errors for MD_CUSTOM_ERROR
//

#define MD_ERROR_SUB400_INVALID_DESTINATION         1
#define MD_ERROR_SUB400_INVALID_DEPTH               2
#define MD_ERROR_SUB400_INVALID_IF                  3
#define MD_ERROR_SUB400_INVALID_OVERWRITE           4
#define MD_ERROR_SUB400_INVALID_TRANSLATE           5
#define MD_ERROR_SUB400_INVALID_REQUEST_BODY        6
#define MD_ERROR_SUB400_INVALID_CONTENT_LENGTH      7
#define MD_ERROR_SUB400_INVALID_TIMEOUT             8
#define MD_ERROR_SUB400_INVALID_LOCK_TOKEN          9
#define MD_ERROR_SUB400_INVALID_XFF_HEADER         10
#define MD_ERROR_SUB400_INVALID_WEBSOCKET_REQUEST  11

#define MD_ERROR_SUB401_LOGON                   1
#define MD_ERROR_SUB401_LOGON_CONFIG            2
#define MD_ERROR_SUB401_LOGON_ACL               3
#define MD_ERROR_SUB401_FILTER                  4
#define MD_ERROR_SUB401_APPLICATION             5
#define MD_ERROR_SUB401_URLAUTH_POLICY          7

#define MD_ERROR_SUB403_EXECUTE_ACCESS_DENIED   1
#define MD_ERROR_SUB403_READ_ACCESS_DENIED      2
#define MD_ERROR_SUB403_WRITE_ACCESS_DENIED     3
#define MD_ERROR_SUB403_SSL_REQUIRED            4
#define MD_ERROR_SUB403_SSL128_REQUIRED         5
#define MD_ERROR_SUB403_ADDR_REJECT             6
#define MD_ERROR_SUB403_CERT_REQUIRED           7
#define MD_ERROR_SUB403_SITE_ACCESS_DENIED      8
#define MD_ERROR_SUB403_TOO_MANY_USERS          9
#define MD_ERROR_SUB403_INVALID_CNFG           10
#define MD_ERROR_SUB403_PWD_CHANGE             11
#define MD_ERROR_SUB403_MAPPER_DENY_ACCESS     12
#define MD_ERROR_SUB403_CERT_REVOKED           13
#define MD_ERROR_SUB403_DIR_LIST_DENIED        14
#define MD_ERROR_SUB403_CAL_EXCEEDED           15
#define MD_ERROR_SUB403_CERT_BAD               16
#define MD_ERROR_SUB403_CERT_TIME_INVALID      17
#define MD_ERROR_SUB403_APPPOOL_DENIED         18
#define MD_ERROR_SUB403_INSUFFICIENT_PRIVILEGE_FOR_CGI  19
#define MD_ERROR_SUB403_PASSPORT_LOGIN_FAILURE 20
#define MD_ERROR_SUB403_SOURCE_ACCESS_DENIED   21
#define MD_ERROR_SUB403_INFINITE_DEPTH_DENIED  22
#define MD_ERROR_SUB403_LOCK_TOKEN_REQUIRED    23
#define MD_ERROR_SUB403_VALIDATION_FAILURE     24

#define MD_ERROR_SUB404_SITE_NOT_FOUND            1
#define MD_ERROR_SUB404_DENIED_BY_POLICY          2
#define MD_ERROR_SUB404_DENIED_BY_MIMEMAP         3
#define MD_ERROR_SUB404_NO_HANDLER                4
#define MD_ERROR_SUB404_URL_SEQUENCE_DENIED       5
#define MD_ERROR_SUB404_VERB_DENIED               6
#define MD_ERROR_SUB404_FILE_EXTENSION_DENIED     7
#define MD_ERROR_SUB404_HIDDEN_SEGMENT            8
#define MD_ERROR_SUB404_FILE_ATTRIBUTE_HIDDEN     9
#define MD_ERROR_SUB404_URL_DOUBLE_ESCAPED       11
#define MD_ERROR_SUB404_URL_HAS_HIGH_BIT_CHARS   12
#define MD_ERROR_SUB404_URL_TOO_LONG             14
#define MD_ERROR_SUB404_QUERY_STRING_TOO_LONG    15
#define MD_ERROR_SUB404_STATICFILE_DAV           16
#define MD_ERROR_SUB404_PRECONDITIONED_HANDLER   17
#define MD_ERROR_SUB404_QUERY_STRING_SEQUENCE_DENIED 18
#define MD_ERROR_SUB404_DENIED_BY_FILTERING_RULE 19
#define MD_ERROR_SUB404_TOO_MANY_URL_SEGMENTS    20

#define MD_ERROR_SUB413_CONTENT_LENGTH_TOO_LARGE  1

#define MD_ERROR_SUB423_LOCK_TOKEN_SUBMITTED    1
#define MD_ERROR_SUB423_NO_CONFLICTING_LOCK     2

#define MD_ERROR_SUB500_UNC_ACCESS             16
#define MD_ERROR_SUB500_URLAUTH_NO_STORE       17
#define MD_ERROR_SUB500_URLAUTH_STORE_ERROR    18
#define MD_ERROR_SUB500_BAD_METADATA           19
#define MD_ERROR_SUB500_URLAUTH_NO_SCOPE       20
#define MD_ERROR_SUB500_HANDLERS_MODULE        21
#define MD_ERROR_SUB500_ASPNET_MODULES         22
#define MD_ERROR_SUB500_ASPNET_HANDLERS        23
#define MD_ERROR_SUB500_ASPNET_IMPERSONATION   24

#define MD_ERROR_SUB502_TIMEOUT                 1
#define MD_ERROR_SUB502_PREMATURE_EXIT          2
#define MD_ERROR_SUB502_ARR_CONNECTION_ERROR    3
#define MD_ERROR_SUB502_ARR_NO_SERVER           4

#define MD_ERROR_SUB503_CPU_LIMIT               1
#define MD_ERROR_SUB503_APP_CONCURRENT          2
#define MD_ERROR_SUB503_ASPNET_QUEUE_FULL       3
#define MD_ERROR_SUB503_FASTCGI_QUEUE_FULL      4
#define MD_ERROR_SUB503_CONNECTION_LIMIT        5

//
// Valid access rights for ACE entries in MD_ADMIN_ACL
//

#define MD_ACR_READ                 0x00000001
#define MD_ACR_WRITE                0x00000002
#define MD_ACR_RESTRICTED_WRITE     0x00000020
#define MD_ACR_UNSECURE_PROPS_READ  0x00000080
#define MD_ACR_ENUM_KEYS            0x00000008
#define MD_ACR_WRITE_DAC            0x00040000

//
// Valid modes for MD_USER_ISOLATION
//

#define MD_USER_ISOLATION_NONE      0
#define MD_USER_ISOLATION_BASIC     1
#define MD_USER_ISOLATION_AD        2
#define MD_USER_ISOLATION_LAST      2

//
// MD_IP_SEC binary format description
//

/*

  This object is composed of 4 lists : 2 lists ( deny & grant ) of network addresses,
  the only allowed family is AF_INET.
  Each of this list is composed of sublists, one for each ( network address family,
  significant subnet mask ) combination. The significant subnet mask is stored as
  ( number of bytes all 1 ( 0xff ), bitmask in last byte ).
  This is followed by 2 lists ( deny & grant ) of DNS names. Each of these lists is
  composed of sublists, based on then number of components in the DNS name
  e.g. "microsoft.com" has 2 components, "www.msft.com" has 3.

Header:
    SELFREFINDEX    iDenyAddr;      // address deny list
                                    // points to ADDRESS_HEADER
    SELFREFINDEX    iGrantAddr;     // address grant list
                                    // points to ADDRESS_HEADER
    SELFREFINDEX    iDenyName;      // DNS name deny list
                                    // points to NAME_HEADER
    SELFREFINDEX    iGrantName;     // DNS name grant list
                                    // points to NAME_HEADER
    DWORD           dwFlags;
    DWORD           cRefSize;       // size of reference area ( in bytes )

ADDRESS_HEADER :
    DWORD               cEntries;   // # of Entries[]
    DWORD               cAddresses; // total # of addresses in all
                                    // ADDRESS_LIST_ENTRY
    ADDRESS_LIST_ENTRY  Entries[];

ADDRESS_LIST_ENTRY :
    DWORD           iFamily;
    DWORD           cAddresses;
    DWORD           cFullBytes;
    DWORD           LastByte;
    SELFREFINDEX    iFirstAddress;  // points to array of addresses

NAME_HEADER :
    DWORD           cEntries;
    DWORD           cNames;         // total # of names for all Entries[]
    NAME_LIST_ENTRY Entries[];

Name list entry :
    DWORD           cComponents;    // # of DNS components
    DWORD           cNames;
    SELFREFINDEX    iName[];        // array of references to DNS names

This is followed by address arrays & names pointed to by iFirstAddress & iName
Names are '\0' delimited

SELFREFINDEX is a DWORD offset from start of structure with high bit set to 1

*/

//
// Macros
//

#define MD_SET_DATA_RECORD(_pMDR, _id, _attr, _utype, _dtype, _dlen, _pData) \
            { \
            (_pMDR)->dwMDIdentifier=(_id);      \
            (_pMDR)->dwMDAttributes=(_attr);    \
            (_pMDR)->dwMDUserType=(_utype);     \
            (_pMDR)->dwMDDataType=(_dtype);     \
            (_pMDR)->dwMDDataLen=(_dlen);       \
            (_pMDR)->pbMDData=(LPBYTE)(_pData); \
            }

//
// IIS ADSI Admin Object class names
//

#define IIS_CLASS_COMPUTER             "IIsComputer"
#define IIS_CLASS_WEB_SERVICE          "IIsWebService"
#define IIS_CLASS_WEB_SERVER           "IIsWebServer"
#define IIS_CLASS_WEB_INFO             "IIsWebInfo"
#define IIS_CLASS_WEB_DIR              "IIsWebDirectory"
#define IIS_CLASS_WEB_VDIR             "IIsWebVirtualDir"
#define IIS_CLASS_WEB_FILE             "IIsWebFile"
#define IIS_CLASS_FTP_SERVICE          "IIsFtpService"
#define IIS_CLASS_FTP_SERVER           "IIsFtpServer"
#define IIS_CLASS_FTP_INFO             "IIsFtpInfo"
#define IIS_CLASS_FTP_VDIR             "IIsFtpVirtualDir"
#define IIS_CLASS_FILTERS              "IIsFilters"
#define IIS_CLASS_FILTER               "IIsFilter"
#define IIS_CLASS_LOG_MODULES          "IIsLogModules"
#define IIS_CLASS_LOG_MODULE           "IIsLogModule"
#define IIS_CLASS_MIMEMAP              "IIsMimeMap"
#define IIS_CLASS_CERTMAPPER           "IIsCertMapper"
#define IIS_CLASS_COMPRESS_SCHEMES     "IIsCompressionSchemes"
#define IIS_CLASS_COMPRESS_SCHEME      "IIsCompressionScheme"

#define IIS_CLASS_COMPUTER_W           L"IIsComputer"
#define IIS_CLASS_WEB_SERVICE_W        L"IIsWebService"
#define IIS_CLASS_WEB_SERVER_W         L"IIsWebServer"
#define IIS_CLASS_WEB_INFO_W           L"IIsWebInfo"
#define IIS_CLASS_WEB_DIR_W            L"IIsWebDirectory"
#define IIS_CLASS_WEB_VDIR_W           L"IIsWebVirtualDir"
#define IIS_CLASS_WEB_FILE_W           L"IIsWebFile"
#define IIS_CLASS_FTP_SERVICE_W        L"IIsFtpService"
#define IIS_CLASS_FTP_SERVER_W         L"IIsFtpServer"
#define IIS_CLASS_FTP_INFO_W           L"IIsFtpInfo"
#define IIS_CLASS_FTP_VDIR_W           L"IIsFtpVirtualDir"
#define IIS_CLASS_FILTERS_W            L"IIsFilters"
#define IIS_CLASS_FILTER_W             L"IIsFilter"
#define IIS_CLASS_LOG_MODULES_W        L"IIsLogModules"
#define IIS_CLASS_LOG_MODULE_W         L"IIsLogModule"
#define IIS_CLASS_MIMEMAP_W            L"IIsMimeMap"
#define IIS_CLASS_CERTMAPPER_W         L"IIsCertMapper"
#define IIS_CLASS_COMPRESS_SCHEMES_W   L"IIsCompressionSchemes"
#define IIS_CLASS_COMPRESS_SCHEME_W    L"IIsCompressionScheme"


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _IISCNFG_H_
