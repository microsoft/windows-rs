/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1996-2000 Microsoft Corporation

Module Name:

    ntdsapi.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for public NTDS APIs other than directory interfaces like LDAP.

Environment:

    User Mode - Win32

Notes:

--*/


#ifndef _NTDSAPI_H_
#define _NTDSAPI_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


#include <schedule.h>
#include <dsparse.h>

#if !defined(_NTDSAPI_)
#define NTDSAPI DECLSPEC_IMPORT
#if !defined(_NTDSAPI_POSTXP_ASLIB_)
#define NTDSAPI_POSTXP DECLSPEC_IMPORT
#else
#define NTDSAPI_POSTXP
#endif
#else
#define NTDSAPI
#define NTDSAPI_POSTXP
#endif

#ifdef __cplusplus
extern "C" {
#endif

//////////////////////////////////////////////////////////////////////////
//                                                                      //
// Data definitions                                                     //
//                                                                      //
//////////////////////////////////////////////////////////////////////////

#ifdef MIDL_PASS
typedef GUID UUID;
typedef void * RPC_AUTH_IDENTITY_HANDLE;
typedef void VOID;
#endif


// Following constants define the Active Directory Behavior
// Version numbers.
#define DS_BEHAVIOR_WIN2000                            0
#define DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS         1
#define DS_BEHAVIOR_WIN2003                            2
#define DS_BEHAVIOR_WIN2008                            3
#define DS_BEHAVIOR_WIN2008R2                          4
#define DS_BEHAVIOR_WIN2012                            5
#define DS_BEHAVIOR_WIN2012R2                          6
#define DS_BEHAVIOR_WIN2016                            7
#define DS_BEHAVIOR_WIN2025                            10 // Note: WS2019 and WS2022 did not have native functional levels

// Deprecated constants
#define DS_BEHAVIOR_LONGHORN        DS_BEHAVIOR_WIN2008
#define DS_BEHAVIOR_WIN7            DS_BEHAVIOR_WIN2008R2
#define DS_BEHAVIOR_WIN8            DS_BEHAVIOR_WIN2012
#define DS_BEHAVIOR_WINBLUE         DS_BEHAVIOR_WIN2012R2
#define DS_BEHAVIOR_WINTHRESHOLD    DS_BEHAVIOR_WIN2016

#define DS_DEFAULT_LOCALE                                           \
           (MAKELCID(MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US),  \
                     SORT_DEFAULT))

#define DS_DEFAULT_LOCALE_COMPARE_FLAGS    (NORM_IGNORECASE     |   \
                                            NORM_IGNOREKANATYPE |   \
                                            NORM_IGNORENONSPACE |   \
                                            NORM_IGNOREWIDTH    |   \
                                            SORT_STRINGSORT )

// When booted to DS mode, this event is signalled when the DS has completed
// its initial sync attempts.  The period of time between system startup and
// this event's state being set is indeterminate from the local service's
// standpoint.  In the meantime the contents of the DS should be considered
// incomplete / out-dated, and the machine will not be advertised as a domain
// controller to off-machine clients.  Other local services that rely on
// information published in the DS should avoid accessing (or at least
// relying on) the contents of the DS until this event is set.
#define DS_SYNCED_EVENT_NAME    "NTDSInitialSyncsCompleted"
#define DS_SYNCED_EVENT_NAME_W L"NTDSInitialSyncsCompleted"

// Permissions bits used in security descriptors in the directory.
#ifndef _DS_CONTROL_BITS_DEFINED_
#define _DS_CONTROL_BITS_DEFINED_
#define ACTRL_DS_OPEN                           0x00000000
#define ACTRL_DS_CREATE_CHILD                   0x00000001
#define ACTRL_DS_DELETE_CHILD                   0x00000002
#define ACTRL_DS_LIST                           0x00000004
#define ACTRL_DS_SELF                           0x00000008
#define ACTRL_DS_READ_PROP                      0x00000010
#define ACTRL_DS_WRITE_PROP                     0x00000020
#define ACTRL_DS_DELETE_TREE                    0x00000040
#define ACTRL_DS_LIST_OBJECT                    0x00000080
#define ACTRL_DS_CONTROL_ACCESS                 0x00000100

// generic read
#define DS_GENERIC_READ          ((STANDARD_RIGHTS_READ)     | \
                                  (ACTRL_DS_LIST)            | \
                                  (ACTRL_DS_READ_PROP)       | \
                                  (ACTRL_DS_LIST_OBJECT))

// generic execute
#define DS_GENERIC_EXECUTE       ((STANDARD_RIGHTS_EXECUTE)  | \
                                  (ACTRL_DS_LIST))
// generic right
#define DS_GENERIC_WRITE         ((STANDARD_RIGHTS_WRITE)    | \
                                  (ACTRL_DS_SELF)            | \
                                  (ACTRL_DS_WRITE_PROP))
// generic all

#define DS_GENERIC_ALL           ((STANDARD_RIGHTS_REQUIRED) | \
                                  (ACTRL_DS_CREATE_CHILD)    | \
                                  (ACTRL_DS_DELETE_CHILD)    | \
                                  (ACTRL_DS_DELETE_TREE)     | \
                                  (ACTRL_DS_READ_PROP)       | \
                                  (ACTRL_DS_WRITE_PROP)      | \
                                  (ACTRL_DS_LIST)            | \
                                  (ACTRL_DS_LIST_OBJECT)     | \
                                  (ACTRL_DS_CONTROL_ACCESS)  | \
                                  (ACTRL_DS_SELF))
#endif

typedef enum
{
    // unknown name type
    DS_UNKNOWN_NAME = 0,

    // eg: CN=User Name,OU=Users,DC=Example,DC=Microsoft,DC=Com
    DS_FQDN_1779_NAME = 1,

    // eg: Example\UserN
    // Domain-only version includes trailing '\\'.
    DS_NT4_ACCOUNT_NAME = 2,

    // Probably "User Name" but could be something else.  I.e. The
    // display name is not necessarily the defining RDN.
    DS_DISPLAY_NAME = 3,

    // obsolete - see #define later
    // DS_DOMAIN_SIMPLE_NAME = 4,

    // obsolete - see #define later
    // DS_ENTERPRISE_SIMPLE_NAME = 5,

    // String-ized GUID as returned by IIDFromString().
    // eg: {4fa050f0-f561-11cf-bdd9-00aa003a77b6}
    DS_UNIQUE_ID_NAME = 6,

    // eg: example.microsoft.com/software/user name
    // Domain-only version includes trailing '/'.
    DS_CANONICAL_NAME = 7,

    // eg: usern@example.microsoft.com
    DS_USER_PRINCIPAL_NAME = 8,

    // Same as DS_CANONICAL_NAME except that rightmost '/' is
    // replaced with '\n' - even in domain-only case.
    // eg: example.microsoft.com/software\nuser name
    DS_CANONICAL_NAME_EX = 9,

    // eg: www/www.microsoft.com@example.com - generalized service principal
    // names.
    DS_SERVICE_PRINCIPAL_NAME = 10,

    // This is the string representation of a SID.  Invalid for formatDesired.
    // See sddl.h for SID binary <--> text conversion routines.
    // eg: S-1-5-21-397955417-626881126-188441444-501
    DS_SID_OR_SID_HISTORY_NAME = 11,

    // Pseudo-name format so GetUserNameEx can return the DNS domain name to
    // a caller.  This level is not supported by the DS APIs.
    DS_DNS_DOMAIN_NAME = 12

} DS_NAME_FORMAT;

// Map old name formats to closest new format so that old code builds
// against new headers w/o errors and still gets (almost) correct result.

#define DS_DOMAIN_SIMPLE_NAME       DS_USER_PRINCIPAL_NAME
#define DS_ENTERPRISE_SIMPLE_NAME   DS_USER_PRINCIPAL_NAME

typedef enum
{
    DS_NAME_NO_FLAGS = 0x0,

    // Perform a syntactical mapping at the client (if possible) without
    // going out on the wire.  Returns DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING
    // if a purely syntactical mapping is not possible.
    DS_NAME_FLAG_SYNTACTICAL_ONLY = 0x1,

    // Force a trip to the DC for evaluation, even if this could be
    // locally cracked syntactically.
    DS_NAME_FLAG_EVAL_AT_DC = 0x2,

    // The call fails if the DC is not a GC
    DS_NAME_FLAG_GCVERIFY = 0x4,

    // Enable cross forest trust referral
    DS_NAME_FLAG_TRUST_REFERRAL = 0x8

} DS_NAME_FLAGS;

typedef enum
{
    DS_NAME_NO_ERROR = 0,

    // Generic processing error.
    DS_NAME_ERROR_RESOLVING = 1,

    // Couldn't find the name at all - or perhaps caller doesn't have
    // rights to see it.
    DS_NAME_ERROR_NOT_FOUND = 2,

    // Input name mapped to more than one output name.
    DS_NAME_ERROR_NOT_UNIQUE = 3,

    // Input name found, but not the associated output format.
    // Can happen if object doesn't have all the required attributes.
    DS_NAME_ERROR_NO_MAPPING = 4,

    // Unable to resolve entire name, but was able to determine which
    // domain object resides in.  Thus DS_NAME_RESULT_ITEM?.pDomain
    // is valid on return.
    DS_NAME_ERROR_DOMAIN_ONLY = 5,

    // Unable to perform a purely syntactical mapping at the client
    // without going out on the wire.
    DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING = 6,

    // The name is from an external trusted forest.
    DS_NAME_ERROR_TRUST_REFERRAL = 7

} DS_NAME_ERROR;

#define DS_NAME_LEGAL_FLAGS (DS_NAME_FLAG_SYNTACTICAL_ONLY)

typedef enum {

    // "paulle-nec.ntwksta.ms.com"
    DS_SPN_DNS_HOST = 0,

    // "cn=paulle-nec,ou=computers,dc=ntwksta,dc=ms,dc=com"
    DS_SPN_DN_HOST = 1,

    // "paulle-nec"
    DS_SPN_NB_HOST = 2,

    // "ntdev.ms.com"
    DS_SPN_DOMAIN = 3,

    // "ntdev"
    DS_SPN_NB_DOMAIN = 4,

    // "cn=anRpcService,cn=RPC Services,cn=system,dc=ms,dc=com"
    // "cn=aWsService,cn=Winsock Services,cn=system,dc=ms,dc=com"
    // "cn=aService,dc=itg,dc=ms,dc=com"
    // "www.ms.com", "ftp.ms.com", "ldap.ms.com"
    // "products.ms.com"
    DS_SPN_SERVICE = 5

} DS_SPN_NAME_TYPE;

typedef enum {                          // example:
        DS_SPN_ADD_SPN_OP = 0,          // add SPNs
        DS_SPN_REPLACE_SPN_OP = 1,      // set all SPNs
        DS_SPN_DELETE_SPN_OP = 2        // Delete SPNs
} DS_SPN_WRITE_OP;

typedef struct
{
    DWORD                   status;     // DS_NAME_ERROR
#ifdef MIDL_PASS
    [string,unique] CHAR    *pDomain;   // DNS domain
    [string,unique] CHAR    *pName;     // name in requested format
#else
    LPSTR                   pDomain;    // DNS domain
    LPSTR                   pName;      // name in requested format
#endif

} DS_NAME_RESULT_ITEMA, *PDS_NAME_RESULT_ITEMA;

typedef struct
{
    DWORD                   cItems;     // item count
#ifdef MIDL_PASS
    [size_is(cItems)] PDS_NAME_RESULT_ITEMA rItems;
#else
    PDS_NAME_RESULT_ITEMA    rItems;    // item array
#endif

} DS_NAME_RESULTA, *PDS_NAME_RESULTA;

typedef struct
{
    DWORD                   status;     // DS_NAME_ERROR
#ifdef MIDL_PASS
    [string,unique] WCHAR   *pDomain;   // DNS domain
    [string,unique] WCHAR   *pName;     // name in requested format
#else
    LPWSTR                  pDomain;    // DNS domain
    LPWSTR                  pName;      // name in requested format
#endif

} DS_NAME_RESULT_ITEMW, *PDS_NAME_RESULT_ITEMW;

typedef struct
{
    DWORD                   cItems;     // item count
#ifdef MIDL_PASS
    [size_is(cItems)] PDS_NAME_RESULT_ITEMW rItems;
#else
    PDS_NAME_RESULT_ITEMW    rItems;    // item array
#endif

} DS_NAME_RESULTW, *PDS_NAME_RESULTW;

#ifdef UNICODE
#define DS_NAME_RESULT DS_NAME_RESULTW
#define PDS_NAME_RESULT PDS_NAME_RESULTW
#define DS_NAME_RESULT_ITEM DS_NAME_RESULT_ITEMW
#define PDS_NAME_RESULT_ITEM PDS_NAME_RESULT_ITEMW
#else
#define DS_NAME_RESULT DS_NAME_RESULTA
#define PDS_NAME_RESULT PDS_NAME_RESULTA
#define DS_NAME_RESULT_ITEM DS_NAME_RESULT_ITEMA
#define PDS_NAME_RESULT_ITEM PDS_NAME_RESULT_ITEMA
#endif

// Public replication option flags

// ********************
// DsBindWithSpnEx flags
// ********************
// Allow the Bind to use delegate service level, so that you can
// do ntdsapi operations that require delegation, such as
// DsAddSidHistory, and DsReplicaSyncAll().  Most operations do
// not require DELEGATE so this flag should only be specified
// if you need it, because if you bind to a rogue server with
// the DELEGATE flag, you'll allow the rogue server to use your
// credentials to connect back to a non-rogue server and perform
// operations other than you intended.
#define  NTDSAPI_BIND_ALLOW_DELEGATION		(0x00000001)
// With AD/AM, a single machine, could have multiple "AD's" on a
// single server.  Since DsBindXxxx() will not pick an AD/AM
// instance without an instance specifier ( ":389" ), it can be
// difficult (well impossible) to determine from just a server
// name, what the instance annotation or instance guid is.  This
// option will take a server name and find the first available
// AD or AD/AM instance.  WARNING: The results could be non-
// deterministic on a server w/ multiple instances.
#define  NTDSAPI_BIND_FIND_BINDING          (0x00000002)
// We have a family of API's for binding called DsBindWithSpn.
// Would anyone who called these and passed in a non-NULL SPN
// ever want to negotiate down to something that doesn't use that
// SPN?  No, this is a security hole.  So, for backwards compatibility
// if you call without an SPN, we'll create one for you, and attempt
// to use it, and allow negotiation to do it's thing if it doesn't
// work.
#define  NTDSAPI_BIND_FORCE_KERBEROS     (0x00000004)



// ********************
// Replica Sync flags
// These flag values are used both as input to DsReplicaSync and
// as output from DsReplicaGetInfo, PENDING_OPS, DS_REPL_OPW.ulOptions
// ********************

// Perform this operation asynchronously.
#define DS_REPSYNC_ASYNCHRONOUS_OPERATION 0x00000001

// Writeable replica.  Otherwise, read-only.
#define DS_REPSYNC_WRITEABLE              0x00000002

// This is a periodic sync request as scheduled by the admin.
#define DS_REPSYNC_PERIODIC               0x00000004

// Use inter-site messaging
#define DS_REPSYNC_INTERSITE_MESSAGING    0x00000008

// Sync starting from scratch (i.e., at the first USN).
#define DS_REPSYNC_FULL                   0x00000020

// This is a notification of an update that was marked urgent.
#define DS_REPSYNC_URGENT                 0x00000040

// Don't discard this synchronization request, even if a similar
// sync is pending.
#define DS_REPSYNC_NO_DISCARD             0x00000080

// Sync even if link is currently disabled.
#define DS_REPSYNC_FORCE                  0x00000100

// Causes the source DSA to check if a reps-to is present for the local DSA
// (aka the destination). If not, one is added.  This ensures that
// source sends change notifications.
#define DS_REPSYNC_ADD_REFERENCE          0x00000200

// A sync from this source has never completed (e.g., a new source).
#define DS_REPSYNC_NEVER_COMPLETED        0x00000400

// When this sync is complete, requests a sync in the opposite direction.
#define DS_REPSYNC_TWO_WAY                0x00000800

// Do not request change notifications from this source.
#define DS_REPSYNC_NEVER_NOTIFY           0x00001000

// Sync the NC from this source when the DSA is started.
#define DS_REPSYNC_INITIAL                0x00002000

// Use compression when replicating.  Saves message size (e.g., network
// bandwidth) at the expense of extra CPU overhead at both the source and
// destination servers.
#define DS_REPSYNC_USE_COMPRESSION        0x00004000

// Sync was abandoned for lack of updates (W2K, W2K3)
#define DS_REPSYNC_ABANDONED              0x00008000

// Special secret processing
#define DS_REPSYNC_SELECT_SECRETS         0x00008000

// Initial sync in progress
#define DS_REPSYNC_INITIAL_IN_PROGRESS    0x00010000

// Partial Attribute Set sync in progress
#define DS_REPSYNC_PARTIAL_ATTRIBUTE_SET  0x00020000

// Sync is being retried
#define DS_REPSYNC_REQUEUE                0x00040000

// Sync is a notification request from a source
#define DS_REPSYNC_NOTIFICATION           0x00080000

// Sync is a special form which requests to establish contact
// now and do the rest of the sync later
#define DS_REPSYNC_ASYNCHRONOUS_REPLICA   0x00100000

// Request critical objects only
#define DS_REPSYNC_CRITICAL               0x00200000

// A full synchronization is in progress
#define DS_REPSYNC_FULL_IN_PROGRESS       0x00400000

// Synchronization request was previously preempted
#define DS_REPSYNC_PREEMPTED              0x00800000

// Non GC readonly replica
#define DS_REPSYNC_NONGC_RO_REPLICA       0x01000000

// ********************
// Replica Add flags
// ********************

// Perform this operation asynchronously.
#define DS_REPADD_ASYNCHRONOUS_OPERATION  0x00000001

// Create a writeable replica.  Otherwise, read-only.
#define DS_REPADD_WRITEABLE               0x00000002

// Sync the NC from this source when the DSA is started.
#define DS_REPADD_INITIAL                 0x00000004

// Sync the NC from this source periodically, as defined by the
// schedule passed in the preptimesSync argument.
#define DS_REPADD_PERIODIC                0x00000008

// Sync from the source DSA via an Intersite Messaging Service (ISM) transport
// (e.g., SMTP) rather than native DS RPC.
#define DS_REPADD_INTERSITE_MESSAGING     0x00000010

// Don't replicate the NC now -- just save enough state such that we
// know to replicate it later.
#define DS_REPADD_ASYNCHRONOUS_REPLICA     0x00000020

// Disable notification-based synchronization for the NC from this source.
// This is expected to be a temporary state; the similar flag
// DS_REPADD_NEVER_NOTIFY should be used if the disable is to be more permanent.
#define DS_REPADD_DISABLE_NOTIFICATION     0x00000040

// Disable periodic synchronization for the NC from this source
#define DS_REPADD_DISABLE_PERIODIC         0x00000080

// Use compression when replicating.  Saves message size (e.g., network
// bandwidth) at the expense of extra CPU overhead at both the source and
// destination servers.
#define DS_REPADD_USE_COMPRESSION          0x00000100

// Do not request change notifications from this source.  When this flag is
// set, the source will not notify the destination when changes occur.
// Recommended for all intersite replication, which may occur over WAN links.
// This is expected to be a more or less permanent state; the similar flag
// DS_REPADD_DISABLE_NOTIFICATION should be used if notifications are to be
// disabled only temporarily.
#define DS_REPADD_NEVER_NOTIFY             0x00000200

// When this sync is complete, requests a sync in the opposite direction.
#define DS_REPADD_TWO_WAY                  0x00000400

// Request critical objects only
// Critical only is only allowed while installing
// A critical only sync does not bring all objects in the partition. It
// replicates just the ones necessary for minimal directory operation.
// A normal, non-critical sync must be performed before the partition
// can be considered fully synchronized.
#define DS_REPADD_CRITICAL                 0x00000800

// Special secret processing
#define DS_REPADD_SELECT_SECRETS           0x00001000

// Non GC RO Replica
#define DS_REPADD_NONGC_RO_REPLICA         0x01000000



// ********************
// Replica Delete flags
// ********************

// Perform this operation asynchronously.
#define DS_REPDEL_ASYNCHRONOUS_OPERATION 0x00000001

// The replica being deleted is writeable.
#define DS_REPDEL_WRITEABLE               0x00000002

// Replica is a mail-based replica
#define DS_REPDEL_INTERSITE_MESSAGING     0x00000004

// Ignore any error generated by contacting the source to tell it to scratch
// this server from its Reps-To for this NC.
#define DS_REPDEL_IGNORE_ERRORS           0x00000008

// Do not contact the source telling it to scratch this server from its
// Rep-To for this NC.  Otherwise, if the link is RPC-based, the source will
// be contacted.
#define DS_REPDEL_LOCAL_ONLY              0x00000010

// Delete all the objects in the NC
// "No source" is incompatible with (and rejected for) writeable NCs.  This is
// valid only for read-only NCs, and then only if the NC has no source.  This
// can occur when the NC has been partially deleted (in which case the KCC
// periodically calls the delete API with the "no source" flag set).
#define DS_REPDEL_NO_SOURCE               0x00000020

// Allow deletion of read-only replica even if it sources
// other read-only replicas.
#define DS_REPDEL_REF_OK                  0x00000040


// ********************
// Replica Modify flags
// ********************

// Perform this operation asynchronously.
#define DS_REPMOD_ASYNCHRONOUS_OPERATION  0x00000001

// The replica is writeable.
#define DS_REPMOD_WRITEABLE               0x00000002


// ********************
// Replica Modify fields
// ********************

#define DS_REPMOD_UPDATE_FLAGS             0x00000001
#define DS_REPMOD_UPDATE_INSTANCE          0x00000002
#define DS_REPMOD_UPDATE_ADDRESS           DS_REPMOD_UPDATE_INSTANCE
#define DS_REPMOD_UPDATE_SCHEDULE          0x00000004
#define DS_REPMOD_UPDATE_RESULT            0x00000008
#define DS_REPMOD_UPDATE_TRANSPORT         0x00000010

// ********************
// Update Refs fields
// ********************

// Perform this operation asynchronously.
#define DS_REPUPD_ASYNCHRONOUS_OPERATION  0x00000001

// The replica being deleted is writeable.
#define DS_REPUPD_WRITEABLE               0x00000002

// Add a reference
#define DS_REPUPD_ADD_REFERENCE           0x00000004

// Remove a reference
#define DS_REPUPD_DELETE_REFERENCE        0x00000008

// Use GCSPN while notifying replica partner
#define DS_REPUPD_REFERENCE_GCSPN         0x00000010


// ********************
//  NC Related Flags
// ********************
//
// Instance Type bits, specifies flags for NC head creation.
//
#define DS_INSTANCETYPE_IS_NC_HEAD        0x00000001 // This if what to specify on an object to indicate it's an NC Head.
#define DS_INSTANCETYPE_NC_IS_WRITEABLE   0x00000004 // This is to indicate that the NC Head is writeable.
#define DS_INSTANCETYPE_NC_COMING         0x00000010 // This is to indicate that this NC is still replicating in objects to this DC, and may not be a complete NC.
#define DS_INSTANCETYPE_NC_GOING          0x00000020 // This is to indicate that this NC is in the process of being removed from this DC, and may not be a complete NC.

// ********************
//  xxx_OPT_xxx Flags
// ********************

// These macros define bit flags which can be set in the "options" attribute
// of objects of the specified object class.

// Bit flags valid for options attribute on NTDS-DSA objects.
//
#define NTDSDSA_OPT_IS_GC                     ( 1 << 0 ) /* DSA is a global catalog */
#define NTDSDSA_OPT_DISABLE_INBOUND_REPL      ( 1 << 1 ) /* disable inbound replication */
#define NTDSDSA_OPT_DISABLE_OUTBOUND_REPL     ( 1 << 2 ) /* disable outbound replication */
#define NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE    ( 1 << 3 ) /* disable logical conn xlation */
#define NTDSDSA_OPT_DISABLE_SPN_REGISTRATION  ( 1 << 4 ) /* disable SPN registration for ADAM */
#define NTDSDSA_OPT_GENERATE_OWN_TOPO         ( 1 << 5 ) /* create own site topology */
#define NTDSDSA_OPT_BLOCK_RPC                 ( 1 << 6 ) /* do not connect to this server via any RPC calls */

// Bit flags for options attribute on NTDS-Connection objects.
//
// The reasons that two bits are required to control notification are as follows.
// We must support existing connections with the old behavior and the UI does not
// create manual connections with the new bit set.
// The default for existing and manually created connections with bits 2 and 3
// clear must be the standard prior behavior: notification for intra-site and
// no notification for inter-site.
// We need a way to distinguish a old connection which desires the default
// notification rules, and a new connection for which we desire to explicitly
// control the notification state as passed down from a site link.  Thus we
// have a new bit to say we are overriding the default, and a new bit to indicate
// what the overridden default shall be.
//
#define NTDSCONN_OPT_IS_GENERATED ( 1 << 0 )  /* object generated by DS, not admin */
#define NTDSCONN_OPT_TWOWAY_SYNC  ( 1 << 1 )  /* force sync in opposite direction at end of sync */
#define NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT (1 << 2 )  // Do not use defaults to determine notification
#define NTDSCONN_OPT_USE_NOTIFY   (1 << 3) // Does source notify destination

// For intra-site connections, this bit has no meaning.
// For inter-site connections, this bit means:
//  0 - Compression of replication data enabled
//  1 - Compression of replication data disabled
#define NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION    (1 << 4)

// For connections whose IS_GENERATED bit is 0, this bit has no effect.
// For KCC-generated connections, this bit indicates that the schedule attribute
// is owned by the user and should not be touched by the KCC.
#define NTDSCONN_OPT_USER_OWNED_SCHEDULE    (1 << 5)

// This is the default rodc connection - 1 per rodc for FRS's uses
#define NTDSCONN_OPT_RODC_TOPOLOGY            (1 << 6)

// Connection reasons
//
// Values for "reason for connection".  A connection can be needed for
// more than one reason.
//
#define NTDSCONN_KCC_NO_REASON                ( 0 )               // 000
#define NTDSCONN_KCC_GC_TOPOLOGY              ( 1 << 0 )          // 001
#define NTDSCONN_KCC_RING_TOPOLOGY            ( 1 << 1 )          // 002
#define NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY   ( 1 << 2 )          // 004
#define NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY   ( 1 << 3 )          // 008
#define NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY   ( 1 << 4 ) // 010
#define NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY    (1 << 5)            // 020
#define NTDSCONN_KCC_INTERSITE_TOPOLOGY       (1 << 6)            // 040
#define NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY (1 << 7)            // 080
#define NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY   (1 << 8)            // 100
#define NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY (1 << 9)           // 200

//
// The high 4 bits of the options attribute are used by NTFRS to assign priority
// for inbound connections. Bit 31 is used to force FRS to ignore schedule during
// the initial sync. Bits 30 - 28 are used to specify a priority between 0-7.
//

#define FRSCONN_PRIORITY_MASK		      0x70000000
#define FRSCONN_MAX_PRIORITY		      0x8

#define NTDSCONN_OPT_IGNORE_SCHEDULE_MASK 0x80000000

#define	NTDSCONN_IGNORE_SCHEDULE(_options_)\
        (((_options_) & NTDSCONN_OPT_IGNORE_SCHEDULE_MASK) >> 31)

#define	FRSCONN_GET_PRIORITY(_options_)    \
        (((((_options_) & FRSCONN_PRIORITY_MASK) >> 28) != 0 ) ? \
         (((_options_) & FRSCONN_PRIORITY_MASK) >> 28) :        \
         FRSCONN_MAX_PRIORITY                                   \
        )

// Bit flags for options attribute on NTDS-Site-Settings objects.
//
#define NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED     ( 1 << 0 ) /* automatic topology gen disabled */
#define NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED      ( 1 << 1 ) /* automatic topology cleanup disabled */
#define NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED     ( 1 << 2 ) /* automatic minimum hops topology disabled */
#define NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED ( 1 << 3 ) /* automatic stale server detection disabled */
#define NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED ( 1 << 4 ) /* automatic inter-site topology gen disabled */
#define NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED      ( 1 << 5 ) /* group memberships for users enabled */
#define NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR   ( 1 << 6 ) /* force KCC to operate in Whistler behavior mode */
#define NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION        ( 1 << 7 ) /* force KCC to use the Windows 2000 ISTG election algorithm */
#define NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED ( 1 << 8 ) /* prevent the KCC from randomly picking a bridgehead when creating a connection */
#define NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED   ( 1 << 9 ) /* allow the KCC to use hashing when creating a replication schedule */
#define NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED     ( 1 << 10 ) /* create static failover connections */

// The following two options allow the new kcc spanning tree algorithm, which replaces the ISM,
// to override the ISM policy settings. These two flags are analogous to those found on the
// transport object.

// default without flag: schedules are significant
// This flag causes the KCC in W2K3 mode to ignore schedules
#define NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES ( 1 << 11 ) // Schedules disabled

// default without flag: links transitive (bridges not required) auto site link bridging enabled
// This flag causes the KCC in W2K3 mode to require bridges
#define NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED (1 << 12 ) // siteLink bridges are required

// How many redundant connections will be generated
#define NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY 2

// Bit flags for options attribute on Inter-Site-Transport objects
//
// Note, the sense of the flag should be such that the default state or
// behavior corresponds to the flag NOT being present. Put another way, the
// flag should state the OPPOSITE of the default
//
// default: schedules are significant
#define NTDSTRANSPORT_OPT_IGNORE_SCHEDULES ( 1 << 0 ) // Schedules disabled

// default: links transitive (bridges not required)
#define NTDSTRANSPORT_OPT_BRIDGES_REQUIRED (1 << 1 ) // siteLink bridges are required

// Bit flags for options attribute on site-Connection objects
//
// These are not realized in the DS, but are built up in the KCC
#define NTDSSITECONN_OPT_USE_NOTIFY ( 1 << 0 ) // Use notification on this link
#define NTDSSITECONN_OPT_TWOWAY_SYNC ( 1 << 1 )  /* force sync in opposite direction at end of sync */

// This bit means:
//  0 - Compression of replication data across this site connection enabled
//  1 - Compression of replication data across this site connection disabled
#define NTDSSITECONN_OPT_DISABLE_COMPRESSION ( 1 << 2 )

// Bit flags for options attribute on site-Link objects
// Note that these options are AND-ed along a site-link path
//
#define NTDSSITELINK_OPT_USE_NOTIFY ( 1 << 0 ) // Use notification on this link
#define NTDSSITELINK_OPT_TWOWAY_SYNC ( 1 << 1 )  /* force sync in opposite direction at end of sync */

// This bit means:
//  0 - Compression of replication data across this site link enabled
//  1 - Compression of replication data across this site link disabled
#define NTDSSITELINK_OPT_DISABLE_COMPRESSION ( 1 << 2 )


// ***********************
// Well Known Object Guids
// ***********************

#define GUID_USERS_CONTAINER_A                      "a9d1ca15768811d1aded00c04fd8d5cd"
#define GUID_COMPUTRS_CONTAINER_A                   "aa312825768811d1aded00c04fd8d5cd"
#define GUID_SYSTEMS_CONTAINER_A                    "ab1d30f3768811d1aded00c04fd8d5cd"
#define GUID_DOMAIN_CONTROLLERS_CONTAINER_A         "a361b2ffffd211d1aa4b00c04fd7d83a"
#define GUID_INFRASTRUCTURE_CONTAINER_A             "2fbac1870ade11d297c400c04fd8d5cd"
#define GUID_DELETED_OBJECTS_CONTAINER_A            "18e2ea80684f11d2b9aa00c04f79f805"
#define GUID_LOSTANDFOUND_CONTAINER_A               "ab8153b7768811d1aded00c04fd8d5cd"
#define GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_A  "22b70c67d56e4efb91e9300fca3dc1aa"
#define GUID_PROGRAM_DATA_CONTAINER_A               "09460c08ae1e4a4ea0f64aee7daa1e5a"
#define GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_A     "f4be92a4c777485e878e9421d53087db"
#define GUID_NTDS_QUOTAS_CONTAINER_A                "6227f0af1fc2410d8e3bb10615bb5b0f"

#define GUID_USERS_CONTAINER_W                      L"a9d1ca15768811d1aded00c04fd8d5cd"
#define GUID_COMPUTRS_CONTAINER_W                   L"aa312825768811d1aded00c04fd8d5cd"
#define GUID_SYSTEMS_CONTAINER_W                    L"ab1d30f3768811d1aded00c04fd8d5cd"
#define GUID_DOMAIN_CONTROLLERS_CONTAINER_W         L"a361b2ffffd211d1aa4b00c04fd7d83a"
#define GUID_INFRASTRUCTURE_CONTAINER_W             L"2fbac1870ade11d297c400c04fd8d5cd"
#define GUID_DELETED_OBJECTS_CONTAINER_W            L"18e2ea80684f11d2b9aa00c04f79f805"
#define GUID_LOSTANDFOUND_CONTAINER_W               L"ab8153b7768811d1aded00c04fd8d5cd"
#define GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_W  L"22b70c67d56e4efb91e9300fca3dc1aa"
#define GUID_PROGRAM_DATA_CONTAINER_W               L"09460c08ae1e4a4ea0f64aee7daa1e5a"
#define GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_W     L"f4be92a4c777485e878e9421d53087db"
#define GUID_NTDS_QUOTAS_CONTAINER_W                L"6227f0af1fc2410d8e3bb10615bb5b0f"
#define GUID_MANAGED_SERVICE_ACCOUNTS_CONTAINER_W   L"1EB93889E40C45DF9F0C64D23BBB6237"
#define GUID_KEYS_CONTAINER_W                       L"683A24E2E8164BD3AF86AC3C2CF3F981"

#define GUID_USERS_CONTAINER_BYTE                       "\xa9\xd1\xca\x15\x76\x88\x11\xd1\xad\xed\x00\xc0\x4f\xd8\xd5\xcd"
#define GUID_COMPUTRS_CONTAINER_BYTE                    "\xaa\x31\x28\x25\x76\x88\x11\xd1\xad\xed\x00\xc0\x4f\xd8\xd5\xcd"
#define GUID_SYSTEMS_CONTAINER_BYTE                     "\xab\x1d\x30\xf3\x76\x88\x11\xd1\xad\xed\x00\xc0\x4f\xd8\xd5\xcd"
#define GUID_DOMAIN_CONTROLLERS_CONTAINER_BYTE          "\xa3\x61\xb2\xff\xff\xd2\x11\xd1\xaa\x4b\x00\xc0\x4f\xd7\xd8\x3a"
#define GUID_INFRASTRUCTURE_CONTAINER_BYTE              "\x2f\xba\xc1\x87\x0a\xde\x11\xd2\x97\xc4\x00\xc0\x4f\xd8\xd5\xcd"
#define GUID_DELETED_OBJECTS_CONTAINER_BYTE             "\x18\xe2\xea\x80\x68\x4f\x11\xd2\xb9\xaa\x00\xc0\x4f\x79\xf8\x05"
#define GUID_LOSTANDFOUND_CONTAINER_BYTE                "\xab\x81\x53\xb7\x76\x88\x11\xd1\xad\xed\x00\xc0\x4f\xd8\xd5\xcd"
#define GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_BYTE   "\x22\xb7\x0c\x67\xd5\x6e\x4e\xfb\x91\xe9\x30\x0f\xca\x3d\xc1\xaa"
#define GUID_PROGRAM_DATA_CONTAINER_BYTE                "\x09\x46\x0c\x08\xae\x1e\x4a\x4e\xa0\xf6\x4a\xee\x7d\xaa\x1e\x5a"
#define GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_BYTE      "\xf4\xbe\x92\xa4\xc7\x77\x48\x5e\x87\x8e\x94\x21\xd5\x30\x87\xdb"
#define GUID_NTDS_QUOTAS_CONTAINER_BYTE                 "\x62\x27\xf0\xaf\x1f\xc2\x41\x0d\x8e\x3b\xb1\x06\x15\xbb\x5b\x0f"


//////////////////////////////////////////////////////////////////////////
//                                                                      //
// Prototypes                                                           //
//                                                                      //
//////////////////////////////////////////////////////////////////////////

// DSBind takes two optional input parameters which identify whether the
// caller found a domain controller themselves via DsGetDcName or whether
// a domain controller should be found using default parameters.
// Behavior of the possible combinations are outlined below.
//
// DomainControllerName(value), DnsDomainName(NULL)
//
//      The value for DomainControllerName is assumed to have been
//      obtained via DsGetDcName (i.e. Field with the same name in a
//      DOMAIN_CONTROLLER_INFO struct on return from DsGetDcName call.)
//      The client is bound to the domain controller at this name.
//
//      Mutual authentication will be performed using an SPN of
//      LDAP/DomainControllerName provided DomainControllerName
//      is not a NETBIOS name or IP address - i.e. it must be a
//      DNS host name.
//
//      For AD/AM, the DomainControllerName is treated as a Binding String,
//      where the first part is the ServerName (some sort of Network
//      identifier - such as DNS, IP address, NetBios Name, etc), and the
//      2nd part is the public LDAP port.  The AD/AM RPC interface uses
//      the LDAP port as our public annotation for RPC.  Ex:
//
//          MyNetBiosName:3030
//          192.0.0.1:2020
//          server1.microsoft.com:389
//
//      Alternatively, clients can use DsBindByInstance() to specify a
//      a specific RPC Annotation, and an even more specific InstanceGuid,
//      (or "objectGuid" in RPC) which is the "objectGuid" off the servers'
//      DSA (aka "NTDS Settings") object.
//
// DomainControllerName(value), DnsDomainName(value)
//
//      DsBind will connect to the server identified by DomainControllerName.
//
//      Mutual authentication will be performed using an SPN of
//      LDAP/DomainControllerName/DnsDomainName provided neither value
//      is a NETBIOS names or IP address - i.e. they must be
//      valid DNS names.
//
// DomainControllerName(NULL), DnsDomainName(NULL)
//
//      DsBind will attempt to find to a global catalog and fail if one
//      can not be found.
//
//      Mutual authentication will be performed using an SPN of
//      GC/DnsHostName/ForestName where DnsHostName and ForestName
//      represent the DomainControllerName and DnsForestName fields
//      respectively of the DOMAIN_CONTROLLER_INFO returned by the
//      DsGetDcName call used to find a global catalog.
//
// DomainControllerName(NULL), DnsDomainName(value)
//
//      DsBind will attempt to find a domain controller for the domain
//      identified by DnsDomainName and fail if one can not be found.
//
//      Mutual authentication will be performed using an SPN of
//      LDAP/DnsHostName/DnsDomainName where DnsDomainName is that
//      provided by the caller and DnsHostName is that returned by
//      DsGetDcName for the domain specified - provided DnsDomainName
//      is a valid DNS domain name - i.e. not a NETBIOS domain name.

_Check_return_
NTDSAPI
DWORD
WINAPI
DsBindW(
    _In_opt_ LPCWSTR         DomainControllerName,      // in, optional
    _In_opt_ LPCWSTR         DnsDomainName,             // in, optional
    _Out_ HANDLE          *phDS);

_Check_return_
NTDSAPI
DWORD
WINAPI
DsBindA(
    _In_opt_ LPCSTR          DomainControllerName,      // in, optional
    _In_opt_ LPCSTR          DnsDomainName,             // in, optional
    _Out_ HANDLE          *phDS);

#ifdef UNICODE
#define DsBind DsBindW
#else
#define DsBind DsBindA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsBindWithCredW(
    _In_opt_ LPCWSTR         DomainControllerName,      // in, optional
    _In_opt_ LPCWSTR         DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _Out_ HANDLE          *phDS);

_Check_return_
NTDSAPI
DWORD
WINAPI
DsBindWithCredA(
    _In_opt_ LPCSTR          DomainControllerName,      // in, optional
    _In_opt_ LPCSTR          DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _Out_ HANDLE          *phDS);

#ifdef UNICODE
#define DsBindWithCred DsBindWithCredW
#else
#define DsBindWithCred DsBindWithCredA
#endif

//
// DsBindWithSpn{A|W} allows the caller to specify the service principal
// name (SPN) which will be used for mutual authentication against
// the destination server.  Do not provide an SPN if you are expecting
// DsBind to find a server for you as SPNs are machine specific and its
// unlikely the SPN you provide matches the server DsBind finds for you.
// Providing a NULL ServicePrincipalName argument results in behavior
// identical to DsBindWithCred{A|W}.
//

_Check_return_
NTDSAPI
DWORD
WINAPI
DsBindWithSpnW(
    _In_opt_ LPCWSTR         DomainControllerName,      // in, optional
    _In_opt_ LPCWSTR         DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _In_opt_ LPCWSTR         ServicePrincipalName,      // in, optional
    _Out_ HANDLE          *phDS);

_Check_return_
NTDSAPI
DWORD
WINAPI
DsBindWithSpnA(
    _In_opt_ LPCSTR          DomainControllerName,      // in, optional
    _In_opt_ LPCSTR          DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _In_opt_ LPCSTR          ServicePrincipalName,      // in, optional
    _Out_ HANDLE          *phDS);

#ifdef UNICODE
#define DsBindWithSpn DsBindWithSpnW
#else
#define DsBindWithSpn DsBindWithSpnA
#endif

//
// DsBindWithSpnEx{A|W} allows you all the options of the previous
// DsBindWithSpn(), plus the added benefit of specifying some optional
// Binding flags.  Currently if you pass NTDSAPI_BIND_ALLOW_DELEGATION,
// you will get the exact old behaviour.  If you can avoid it, you
// should not specify this flag, see flag above for details.
//

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindWithSpnExW(
    _In_opt_ LPCWSTR         DomainControllerName,      // in, optional
    _In_opt_ LPCWSTR         DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _In_opt_ LPCWSTR         ServicePrincipalName,      // in, optional
    _In_opt_ DWORD           BindFlags,                 // in, optional
    _Out_ HANDLE          *phDS);

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindWithSpnExA(
    _In_opt_ LPCSTR          DomainControllerName,      // in, optional
    _In_opt_ LPCSTR          DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _In_opt_ LPCSTR          ServicePrincipalName,      // in, optional
    _In_opt_ DWORD           BindFlags,                 // in, optional
    _Out_ HANDLE          *phDS);

#ifdef UNICODE
#define DsBindWithSpnEx DsBindWithSpnExW
#else
#define DsBindWithSpnEx DsBindWithSpnExA
#endif

//
// DsBindByInstance{A|W} Allows the explicit binding to any AD/AM
// or AD instance by Annotation or InstanceGuid.  For binding to
// an AD instance the Annotation and InstanceGuid can be left NULL.
// To Bind to an AD/AM instance one or the other must be specified
// to specify the AD/AM instance desired.
//

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindByInstanceW(
    _In_opt_ LPCWSTR         ServerName,                // in, optional
    _In_opt_ LPCWSTR         Annotation,                // in, optional
    _In_opt_ GUID *          InstanceGuid,              // in, optional
    _In_opt_ LPCWSTR         DnsDomainName,             // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _In_opt_ LPCWSTR         ServicePrincipalName,      // in, optional
    _In_opt_ DWORD           BindFlags,                 // in, optional
    _Out_ HANDLE          *phDS);

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindByInstanceA(
    _In_opt_ LPCSTR         ServerName,                 // in, optional
    _In_opt_ LPCSTR         Annotation,                 // in, optional
    _In_opt_ GUID *         InstanceGuid,               // in, optional
    _In_opt_ LPCSTR         DnsDomainName,              // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity,     // in, optional
    _In_opt_ LPCSTR         ServicePrincipalName,       // in, optional
    _In_opt_ DWORD          BindFlags,                  // in, optional
    _Out_ HANDLE         *phDS);

#ifdef UNICODE
#define DsBindByInstance DsBindByInstanceW
#else
#define DsBindByInstance DsBindByInstanceA
#endif

//
// DsBindToISTG{A|W} allows the caller to bind to the server which
// holds the Inter-Site Topology Generator role in the specified site.
// The site name should be the RDN of a site.  If no site is specified,
// the function will try to bind to the ISTG in a nearby site.
//

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindToISTGW(
    _In_opt_ LPCWSTR         SiteName,                  // in, optional
    _Out_ HANDLE          *phDS);

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindToISTGA(
    _In_opt_ LPCSTR          SiteName,                  // in, optional
    _Out_ HANDLE          *phDS);

#ifdef UNICODE
#define DsBindToISTG DsBindToISTGW
#else
#define DsBindToISTG DsBindToISTGA
#endif

//
// DsBindingSetTimeout allows the caller to specify a timeout value
// which will be honored by all RPC calls using the specified binding
// handle. RPC calls which take longer the timeout value are canceled.
//

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsBindingSetTimeout(
    _In_ HANDLE          hDS,                        // in
    _In_ ULONG           cTimeoutSecs                // in
    );

//
// DsUnBind
//

NTDSAPI
DWORD
WINAPI
DsUnBindW(
    _In_ HANDLE          *phDS);             // in

NTDSAPI
DWORD
WINAPI
DsUnBindA(
    _In_ HANDLE          *phDS);             // in

#ifdef UNICODE
#define DsUnBind DsUnBindW
#else
#define DsUnBind DsUnBindA
#endif

//
// DsMakePasswordCredentials
//
// This function constructs a credential structure which is suitable for input
// to the DsBindWithCredentials function, or the ldap_open function (winldap.h)
// The credential must be freed using DsFreeCredential.
//
// None of the input parameters may be present indicating a null, default
// credential.  Otherwise the username must be present.  If the domain or
// password are null, they default to empty strings.  The domain name may be
// null when the username is fully qualified, for example UPN format.
//

_Check_return_
NTDSAPI
DWORD
WINAPI
DsMakePasswordCredentialsW(
    _In_opt_ LPCWSTR User,
    _In_opt_ LPCWSTR Domain,
    _In_opt_ LPCWSTR Password,
    _Out_ RPC_AUTH_IDENTITY_HANDLE *pAuthIdentity
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsMakePasswordCredentialsA(
    _In_opt_ LPCSTR User,
    _In_opt_ LPCSTR Domain,
    _In_opt_ LPCSTR Password,
    _Out_ RPC_AUTH_IDENTITY_HANDLE *pAuthIdentity
    );

#ifdef UNICODE
#define DsMakePasswordCredentials DsMakePasswordCredentialsW
#else
#define DsMakePasswordCredentials DsMakePasswordCredentialsA
#endif

NTDSAPI
VOID
WINAPI
DsFreePasswordCredentials(
    _In_ RPC_AUTH_IDENTITY_HANDLE AuthIdentity
    );

#define DsFreePasswordCredentialsW DsFreePasswordCredentials
#define DsFreePasswordCredentialsA DsFreePasswordCredentials

//
// DsCrackNames
//

_Check_return_
NTDSAPI
DWORD
WINAPI
DsCrackNamesW(
    _In_opt_ HANDLE          hDS,                // in
    _In_ DS_NAME_FLAGS       flags,              // in
    _In_ DS_NAME_FORMAT      formatOffered,      // in
    _In_ DS_NAME_FORMAT      formatDesired,      // in
    _In_ DWORD               cNames,             // in
    _In_reads_(cNames) const LPCWSTR *rpNames,  // in
    _Outptr_ PDS_NAME_RESULTW *ppResult);     // out


_Check_return_
NTDSAPI
DWORD
WINAPI
DsCrackNamesA(
    _In_opt_ HANDLE          hDS,                // in
    _In_ DS_NAME_FLAGS       flags,              // in
    _In_ DS_NAME_FORMAT      formatOffered,      // in
    _In_ DS_NAME_FORMAT      formatDesired,      // in
    _In_ DWORD               cNames,             // in
    _In_reads_(cNames) const LPCSTR *rpNames,   // in
    _Outptr_ PDS_NAME_RESULTA *ppResult);     // out

#ifdef UNICODE
#define DsCrackNames DsCrackNamesW
#else
#define DsCrackNames DsCrackNamesA
#endif

//
// DsFreeNameResult
//

NTDSAPI
void
WINAPI
DsFreeNameResultW(
    _In_ DS_NAME_RESULTW *pResult);          // in

NTDSAPI
void
WINAPI
DsFreeNameResultA(
    _In_ DS_NAME_RESULTA *pResult);          // in

#ifdef UNICODE
#define DsFreeNameResult DsFreeNameResultW
#else
#define DsFreeNameResult DsFreeNameResultA
#endif

// ==========================================================
// DsGetSPN -- server's call to gets SPNs for a service name by which it is
// known to clients. N.B.: there may be more than one name by which clients
// know it the SPNs are then passed to DsAddAccountSpn to register them in
// the DS
//
//      IN SpnNameType eType,
//      IN LPCTSTR ServiceClass,
// kind of service -- "http", "ldap", "ftp", etc.
//      IN LPCTSTR ServiceName OPTIONAL,
// name of service -- DN or DNS; not needed for host-based
//      IN USHORT InstancePort,
// port number (0 => default) for instances
//      IN USHORT cInstanceNames,
// count of extra instance names and ports (0=>use gethostbyname)
//      IN LPCTSTR InstanceNames[] OPTIONAL,
// extra instance names (not used for host names)
//      IN USHORT InstancePorts[] OPTIONAL,
// extra instance ports (0 => default)
//      IN OUT PULONG pcSpn,    // count of SPNs
//      IN OUT LPTSTR * prpszSPN[]
// a bunch of SPNs for this service; free with DsFreeSpnArray

_Check_return_
NTDSAPI
DWORD
WINAPI
DsGetSpnA(
    _In_ DS_SPN_NAME_TYPE ServiceType,
    _In_ LPCSTR ServiceClass,
    _In_opt_ LPCSTR ServiceName,
    _In_ USHORT InstancePort,
    _In_ USHORT cInstanceNames,
    _In_reads_opt_(cInstanceNames) LPCSTR *pInstanceNames,
    _In_reads_opt_(cInstanceNames) const USHORT *pInstancePorts,
    _Out_ DWORD *pcSpn,
    _Outptr_result_buffer_ (*pcSpn) LPSTR **prpszSpn
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsGetSpnW(
    _In_ DS_SPN_NAME_TYPE ServiceType,
    _In_ LPCWSTR ServiceClass,
    _In_opt_ LPCWSTR ServiceName,
    _In_ USHORT InstancePort,
    _In_ USHORT cInstanceNames,
    _In_reads_opt_(cInstanceNames) LPCWSTR *pInstanceNames,
    _In_reads_opt_(cInstanceNames) const USHORT *pInstancePorts,
    _Out_ DWORD *pcSpn,
    _Outptr_result_buffer_(*pcSpn) LPWSTR **prpszSpn
    );

#ifdef UNICODE
#define DsGetSpn DsGetSpnW
#else
#define DsGetSpn DsGetSpnA
#endif

// ==========================================================
// DsFreeSpnArray() -- Free array returned by DsGetSpn{A,W}

NTDSAPI
void
WINAPI
DsFreeSpnArrayA(
    _In_ DWORD cSpn,
    _Inout_updates_to_(cSpn, 0) LPSTR *rpszSpn
    );

NTDSAPI
void
WINAPI
DsFreeSpnArrayW(
    _In_ DWORD cSpn,
    _Inout_updates_to_(cSpn, 0) LPWSTR *rpszSpn
    );

#ifdef UNICODE
#define DsFreeSpnArray DsFreeSpnArrayW
#else
#define DsFreeSpnArray DsFreeSpnArrayA
#endif

// ==========================================================
// DsWriteAccountSpn -- set or add SPNs for an account object
// Usually done by service itself, or perhaps by an admin.
//
// This call is RPC'd to the DC where the account object is stored, so it can
// securely enforce policy on what SPNs are allowed on the account. Direct LDAP
// writes to the SPN property are not allowed -- all writes must come through
// this RPC call. (Reads via // LDAP are OK.)
//
// The account object can be a machine accout, or a service (user) account.
//
// If called by the service to register itself, it can most easily get
// the names by calling DsGetSpn with each of the names that
// clients can use to find the service.
//
// IN SpnWriteOp eOp,                   // set, add
// IN LPCTSTR   pszAccount,             // DN of account to which to add SPN
// IN int       cSPN,                   // count of SPNs to add to account
// IN LPCTSTR   rpszSPN[]               // SPNs to add to altSecID property

_Check_return_
NTDSAPI
DWORD
WINAPI
DsWriteAccountSpnA(
    _In_ HANDLE hDS,
    _In_ DS_SPN_WRITE_OP Operation,
    _In_ LPCSTR pszAccount,
    _In_ DWORD cSpn,
    _In_reads_(cSpn) LPCSTR *rpszSpn
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsWriteAccountSpnW(
    _In_ HANDLE hDS,
    _In_ DS_SPN_WRITE_OP Operation,
    _In_ LPCWSTR pszAccount,
    _In_ DWORD cSpn,
    _In_reads_(cSpn) LPCWSTR *rpszSpn
    );

#ifdef UNICODE
#define DsWriteAccountSpn DsWriteAccountSpnW
#else
#define DsWriteAccountSpn DsWriteAccountSpnA
#endif

/*++

Routine Description:

Constructs a Service Principal Name suitable to identify the desired server.
The service class and part of a dns hostname must be supplied.

This routine is a simplified wrapper to DsMakeSpn.
The ServiceName is made canonical by resolving through DNS.
Guid-based dns names are not supported.

NOTE:
This routine is no longer recommended for use. In order to be secure, an SPN
should be constructed purely on the client without reliance on other services,
such as DNS, which may be spoofed.

The simplified SPN constructed looks like this:

ServiceClass / ServiceName / ServiceName

The instance name portion (2nd position) is always defaulted.  The port and
referrer fields are not used.

Arguments:

    ServiceClass - Class of service, defined by the service, can be any
        string unique to the service

    ServiceName - dns hostname, fully qualified or not
       Stringized IP address is also resolved if necessary

    pcSpnLength - IN, maximum length of buffer, in chars
                  OUT, space utilized, in chars, including terminator

    pszSpn - Buffer, atleast of length *pcSpnLength

Return Value:

    WINAPI - Win32 error code

--*/

_Check_return_
NTDSAPI
DWORD
WINAPI
DsClientMakeSpnForTargetServerW(
    _In_ LPCWSTR ServiceClass,
    _In_ LPCWSTR ServiceName,
    _Inout_ DWORD *pcSpnLength,
    _Out_writes_to_ (*pcSpnLength, *pcSpnLength) LPWSTR pszSpn
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsClientMakeSpnForTargetServerA(
    _In_ LPCSTR ServiceClass,
    _In_ LPCSTR ServiceName,
    _Inout_ DWORD *pcSpnLength,
    _Out_writes_to_(*pcSpnLength, *pcSpnLength) LPSTR pszSpn
    );

#ifdef UNICODE
#define DsClientMakeSpnForTargetServer DsClientMakeSpnForTargetServerW
#else
#define DsClientMakeSpnForTargetServer DsClientMakeSpnForTargetServerA
#endif

/*++

Routine Description:

Register Service Principal Names for a server application.

This routine does the following:
1. Enumerates a list of server SPNs using DsGetSpn and the provided class
2. Determines the domain of the current user context
3. Determines the DN of the current user context if not supplied
4. Locates a domain controller
5. Binds to the domain controller
6. Uses DsWriteAccountSpn to write the SPNs on the named object DN
7. Unbinds

Construct server SPNs for this service, and write them to the right object.

If the userObjectDn is specified, the SPN is written to that object.

Otherwise the Dn is defaulted, to the user object, then computer.

Now, bind to the DS, and register the name on the object for the
user this service is running as.  So, if we're running as local
system, we'll register it on the computer object itself.  If we're
running as a domain user, we'll add the SPN to the user's object.

Arguments:

    Operation - What should be done with the values: add, replace or delete
    ServiceClass - Unique string identifying service
    UserObjectDN - Optional, dn of object to write SPN to

Return Value:

    WINAPI - Win32 error code

--*/

_Check_return_
NTDSAPI
DWORD
WINAPI
DsServerRegisterSpnA(
    DS_SPN_WRITE_OP Operation,
    _In_ LPCSTR ServiceClass,
    _In_opt_ LPCSTR UserObjectDN
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsServerRegisterSpnW(
    DS_SPN_WRITE_OP Operation,
    _In_ LPCWSTR ServiceClass,
    _In_opt_ LPCWSTR UserObjectDN
    );

#ifdef UNICODE
#define DsServerRegisterSpn DsServerRegisterSpnW
#else
#define DsServerRegisterSpn DsServerRegisterSpnA
#endif

// DsReplicaSync.  The server that this call is executing on is called the
// destination.  The destination's naming context will be brought up to date
// with respect to a source system.  The source system is identified by the
// uuid.  The uuid is that of the source system's "NTDS Settings" object.
// The destination system must already be configured such that the source
// system is one of the systems from which it recieves replication data
// ("replication from"). This is usually done automatically by the KCC.
//
//  PARAMETERS:
//      pNC (DSNAME *)
//          Name of the NC to synchronize.
//      puuidSourceDRA (SZ)
//          objectGuid of DSA with which to synchronize the replica.
//      ulOptions (ULONG)
//          Bitwise OR of zero or more flags
//   RETURNS: WIN32 STATUS

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaSyncA(
    _In_ HANDLE hDS,
    _In_ LPCSTR NameContext,
    _In_ const UUID *pUuidDsaSrc,
    ULONG Options
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaSyncW(
    _In_ HANDLE hDS,
    _In_ LPCWSTR NameContext,
    _In_ const UUID *pUuidDsaSrc,
    ULONG Options
    );

#ifdef UNICODE
#define DsReplicaSync DsReplicaSyncW
#else
#define DsReplicaSync DsReplicaSyncA
#endif

// DsReplicaAdd
//
/*
Description:
   This call is executed on the destination.  It causes the destination to
   add a "replication from" reference to the indicated source system.

The source server is identified by string name, not uuid as with Sync.
The DsaSrcAddress parameter is the transport specific address of the source
DSA, usually its guid-based dns name.  The guid in the guid-based dns name is
the object-guid of that server's ntds-dsa (settings) object.

Arguments:

    pNC (IN) - NC for which to add the replica.
    pSourceDsaDN (IN) - DN of the source DSA's ntdsDsa object.  Required if
        ulOptions includes DS_REPADD_ASYNCHRONOUS_REPLICA; ignored otherwise.

    pTransportDN (IN) - DN of the interSiteTransport object representing the
        transport by which to communicate with the source server.  Required if
        ulOptions includes INTERSITE_MESSAGING; ignored otherwise.

    pszSourceDsaAddress (IN) - Transport-specific address of the source DSA.

    pSchedule (IN) - Schedule by which to replicate the NC from this
        source in the future.

    ulOptions (IN) - flags
    RETURNS: WIN32 STATUS
*/

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaAddA(
    _In_ HANDLE hDS,
    _In_ LPCSTR NameContext,
    _In_ LPCSTR SourceDsaDn,
    _In_ LPCSTR TransportDn,
    _In_ LPCSTR SourceDsaAddress,
    _In_opt_ const PSCHEDULE pSchedule,
    DWORD Options
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaAddW(
    _In_ HANDLE hDS,
    _In_ LPCWSTR NameContext,
    _In_ LPCWSTR SourceDsaDn,
    _In_ LPCWSTR TransportDn,
    _In_ LPCWSTR SourceDsaAddress,
    _In_opt_ const PSCHEDULE pSchedule,
    DWORD Options
    );

#ifdef UNICODE
#define DsReplicaAdd DsReplicaAddW
#else
#define DsReplicaAdd DsReplicaAddA
#endif

// DsReplicaDel
//
// The server that this call is executing on is the destination.  The call
// causes the destination to remove a "replication from" reference to the
// indicated source server.
// The source server is identified by string name, not uuid as with Sync.
// The DsaSrc parameter is the transport specific address of the source DSA,
// usually its guid-based dns name.  The guid in the guid-based dns name is
// the object-guid of that server's ntds-dsa (settings) object.
// If you remove an NC from a given destination and specify the "no source"
// flag, then the entire replica will be removed from the DC.
//
//  PARAMETERS:
//      pNC (DSNAME *)
//          Name of the NC for which to delete a source.
//      pszSourceDRA (SZ)
//          DSA for which to delete the source.
//      ulOptions (ULONG)
//          Bitwise OR of zero or more flags
//
//
//   RETURNS: WIN32 STATUS

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaDelA(
    _In_ HANDLE hDS,
    _In_ LPCSTR NameContext,
    _In_ LPCSTR DsaSrc,
    ULONG Options
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaDelW(
    _In_ HANDLE hDS,
    _In_ LPCWSTR NameContext,
    _In_ LPCWSTR DsaSrc,
    ULONG Options
    );

#ifdef UNICODE
#define DsReplicaDel DsReplicaDelW
#else
#define DsReplicaDel DsReplicaDelA
#endif

// DsReplicaModify
//
//
//  Modify a source for a given naming context
//
//  The value must already exist.
//
//  Either the UUID or the address may be used to identify the current value.
//  If a UUID is specified, the UUID will be used for comparison.  Otherwise,
//  the address will be used for comparison.
//
//  PARAMETERS:
//      pNC (DSNAME *)
//          Name of the NC for which the Reps-From should be modified.
//      puuidSourceDRA (UUID *)
//          Guid of the DSA object for the source server. May be NULL if:
//            . ulModifyFields does not include DS_REPMOD_UPDATE_INSTANCE and
//            . pmtxSourceDRA is non-NULL.
//      puuidTransportObj (UUID *)
//          objectGuid of the transport by which replication is to be performed
//          Ignored if ulModifyFields does not include
//          DS_REPMOD_UPDATE_TRANSPORT.
//      pszSourceDRA (SZ)
//          DSA for which the reference should be added or deleted.  Ignored if
//          puuidSourceDRA is non-NULL and ulModifyFields does not include
//          DS_REPMOD_UPDATE_INSTANCE.
//      prtSchedule (REPLTIMES *)
//          Periodic replication schedule for this replica.  Ignored if
//          ulModifyFields does not include DS_REPMOD_UPDATE_SCHEDULE.
//      ulReplicaFlags (ULONG)
//          Flags to set for this replica.  Ignored if ulModifyFields does not
//          include DS_REPMOD_UPDATE_FLAGS.
//      ulModifyFields (ULONG)
//          Fields to update.  One or more of the following bit flags:
//              UPDATE_ADDRESS
//                  Update the MTX_ADDR associated with the referenced server.
//              UPDATE_SCHEDULE
//                  Update the periodic replication schedule associated with
//                  the replica.
//              UPDATE_FLAGS
//                  Update the flags associated with the replica.
//              UPDATE_TRANSPORT
//                  Update the transport associated with the replica.
//      ulOptions (ULONG)
//          Bitwise OR of zero or more of the following:
//              DS_REPMOD_ASYNCHRONOUS_OPERATION
//                  Perform this operation asynchronously.
//   RETURNS: WIN32 STATUS

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaModifyA(
    _In_ HANDLE hDS,
    _In_ LPCSTR NameContext,
    _In_opt_ const UUID *pUuidSourceDsa,
    _Reserved_ LPCSTR TransportDn,
    _In_ LPCSTR SourceDsaAddress,
    _In_opt_ const PSCHEDULE pSchedule,
    _Reserved_ DWORD ReplicaFlags,
    DWORD ModifyFields,
    DWORD Options
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaModifyW(
    _In_ HANDLE hDS,
    _In_ LPCWSTR NameContext,
    _In_opt_ const UUID *pUuidSourceDsa,
    _Reserved_ LPCWSTR TransportDn,
    _In_ LPCWSTR SourceDsaAddress,
    _In_opt_ const PSCHEDULE pSchedule,
    DWORD ReplicaFlags,
    DWORD ModifyFields,
    DWORD Options
    );

#ifdef UNICODE
#define DsReplicaModify DsReplicaModifyW
#else
#define DsReplicaModify DsReplicaModifyA
#endif

// DsReplicaUpdateRefs
//
// In this case, the RPC is being executed on the "source" of destination-sourc
// replication relationship.  This function tells the source that it no longer
// supplies replication information to the indicated destination system.
// Add or remove a target server from the Reps-To property on the given NC.
// Add/remove a reference given the DSNAME of the corresponding NTDS-DSA
// object.
//
//  PARAMETERS:
//      pNC (DSNAME *)
//          Name of the NC for which the Reps-To should be modified.
//      DsaDest (SZ)
//          Network address of DSA for which the reference should be added
//          or deleted.
//      pUuidDsaDest (UUID *)
//          objectGuid of the DSA object for which the reference should be
//          added or deleted.
//      ulOptions (ULONG)
//          Bitwise OR of zero or more of the following:
//              DS_REPUPD_ASYNC_OP
//                  Perform this operation asynchronously.
//              DS_REPUPD_ADD_REFERENCE
//                  Add the given server to the Reps-To property.
//              DS_REPUPD_DEL_REFERENCE
//                  Remove the given server from the Reps-To property.
//          Note that ADD_REF and DEL_REF may be paired to perform
//          "add or update".
//
//   RETURNS: WIN32 STATUS

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaUpdateRefsA(
    _In_ HANDLE hDS,
    _In_ LPCSTR NameContext,
    _In_ LPCSTR DsaDest,
    _In_ const UUID *pUuidDsaDest,
    ULONG Options
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaUpdateRefsW(
    _In_ HANDLE hDS,
    _In_ LPCWSTR NameContext,
    _In_ LPCWSTR DsaDest,
    _In_ const UUID *pUuidDsaDest,
    ULONG Options
    );

#ifdef UNICODE
#define DsReplicaUpdateRefs DsReplicaUpdateRefsW
#else
#define DsReplicaUpdateRefs DsReplicaUpdateRefsA
#endif

// Friends of DsReplicaSyncAll

typedef enum {

    DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER	= 0,
    DS_REPSYNCALL_WIN32_ERROR_REPLICATING		= 1,
    DS_REPSYNCALL_SERVER_UNREACHABLE		= 2

} DS_REPSYNCALL_ERROR;

typedef enum {

    DS_REPSYNCALL_EVENT_ERROR			= 0,
    DS_REPSYNCALL_EVENT_SYNC_STARTED		= 1,
    DS_REPSYNCALL_EVENT_SYNC_COMPLETED		= 2,
    DS_REPSYNCALL_EVENT_FINISHED			= 3

} DS_REPSYNCALL_EVENT;

// Friends of DsReplicaSyncAll

typedef struct {
    LPSTR			pszSrcId;
    LPSTR			pszDstId;
    LPSTR                       pszNC;
    GUID *                      pguidSrc;
    GUID *                      pguidDst;
} DS_REPSYNCALL_SYNCA, * PDS_REPSYNCALL_SYNCA;

typedef struct {
    LPWSTR			pszSrcId;
    LPWSTR			pszDstId;
    LPWSTR                      pszNC;
    GUID *                      pguidSrc;
    GUID *                      pguidDst;
} DS_REPSYNCALL_SYNCW, * PDS_REPSYNCALL_SYNCW;

typedef struct {
    LPSTR			pszSvrId;
    DS_REPSYNCALL_ERROR		error;
    DWORD			dwWin32Err;
    LPSTR			pszSrcId;
} DS_REPSYNCALL_ERRINFOA, * PDS_REPSYNCALL_ERRINFOA;

typedef struct {
    LPWSTR			pszSvrId;
    DS_REPSYNCALL_ERROR		error;
    DWORD			dwWin32Err;
    LPWSTR			pszSrcId;
} DS_REPSYNCALL_ERRINFOW, * PDS_REPSYNCALL_ERRINFOW;

typedef struct {
    DS_REPSYNCALL_EVENT		event;
    DS_REPSYNCALL_ERRINFOA *	pErrInfo;
    DS_REPSYNCALL_SYNCA *	pSync;
} DS_REPSYNCALL_UPDATEA, * PDS_REPSYNCALL_UPDATEA;

typedef struct {
    DS_REPSYNCALL_EVENT		event;
    DS_REPSYNCALL_ERRINFOW *	pErrInfo;
    DS_REPSYNCALL_SYNCW *	pSync;
} DS_REPSYNCALL_UPDATEW, * PDS_REPSYNCALL_UPDATEW;

#ifdef UNICODE
#define DS_REPSYNCALL_SYNC DS_REPSYNCALL_SYNCW
#define DS_REPSYNCALL_ERRINFO DS_REPSYNCALL_ERRINFOW
#define DS_REPSYNCALL_UPDATE DS_REPSYNCALL_UPDATEW
#define PDS_REPSYNCALL_SYNC PDS_REPSYNCALL_SYNCW
#define PDS_REPSYNCALL_ERRINFO PDS_REPSYNCALL_ERRINFOW
#define PDS_REPSYNCALL_UPDATE PDS_REPSYNCALL_UPDATEW
#else
#define DS_REPSYNCALL_SYNC DS_REPSYNCALL_SYNCA
#define DS_REPSYNCALL_ERRINFO DS_REPSYNCALL_ERRINFOA
#define DS_REPSYNCALL_UPDATE DS_REPSYNCALL_UPDATEA
#define PDS_REPSYNCALL_SYNC PDS_REPSYNCALL_SYNCA
#define PDS_REPSYNCALL_ERRINFO PDS_REPSYNCALL_ERRINFOA
#define PDS_REPSYNCALL_UPDATE PDS_REPSYNCALL_UPDATEA
#endif

// **********************
// Replica SyncAll flags
// **********************

// This option has no effect.
#define DS_REPSYNCALL_NO_OPTIONS			0x00000000

// Ordinarily, if a server cannot be contacted, DsReplicaSyncAll tries to
// route around it and replicate from as many servers as possible.  Enabling
// this option will cause DsReplicaSyncAll to generate a fatal error if any
// server cannot be contacted, or if any server is unreachable (due to a
// disconnected or broken topology.)
#define	DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE	0x00000001

// This option disables transitive replication; syncs will only be performed
// with adjacent servers and no DsBind calls will be made.
#define DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY	0x00000002

// Ordinarily, when DsReplicaSyncAll encounters a non-fatal error, it returns
// the GUID DNS of the relevant server(s).  Enabling this option causes
// DsReplicaSyncAll to return the servers' DNs instead.
#define DS_REPSYNCALL_ID_SERVERS_BY_DN			0x00000004

// This option disables all syncing.  The topology will still be analyzed and
// unavailable / unreachable servers will still be identified.
#define DS_REPSYNCALL_DO_NOT_SYNC			0x00000008

// Ordinarily, DsReplicaSyncAll attempts to bind to all servers before
// generating the topology.  If a server cannot be contacted, DsReplicaSyncAll
// excludes that server from the topology and tries to route around it.  If
// this option is enabled, checking will be bypassed and DsReplicaSyncAll will
// assume all servers are responding.  This will speed operation of
// DsReplicaSyncAll, but if some servers are not responding, some transitive
// replications may be blocked.
#define DS_REPSYNCALL_SKIP_INITIAL_CHECK		0x00000010

// Push mode. Push changes from the home server out to all partners using
// transitive replication.  This reverses the direction of replication, and
// the order of execution of the replication sets from the usual "pulling"
// mode of execution.
#define DS_REPSYNCALL_PUSH_CHANGES_OUTWARD              0x00000020

// Cross site boundaries.  By default, the only servers that are considered are
// those in the same site as the home system.  With this option, all servers in
// the enterprise, across all sites, are eligible.  They must be connected by
// a synchronous (RPC) transport, however.
#define DS_REPSYNCALL_CROSS_SITE_BOUNDARIES             0x00000040

// DsReplicaSyncAll.  Syncs the destination server with all other servers
// in the site.
//
//  PARAMETERS:
//	hDS		(IN) - A DS connection bound to the destination server.
//	pszNameContext	(IN) - The naming context to synchronize
//	ulFlags		(IN) - Bitwise OR of zero or more flags
//	pFnCallBack	(IN, OPTIONAL) - Callback function for message-passing.
//	pCallbackData	(IN, OPTIONAL) - A pointer that will be passed to the
//				first argument of the callback function.
//	pErrors		(OUT, OPTIONAL) - Pointer to a (PDS_REPSYNCALL_ERRINFO *)
//				object that will hold an array of error structures.

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaSyncAllA (
    _In_ HANDLE				hDS,
    _In_ LPCSTR				pszNameContext,
    ULONG				ulFlags,
    __callback BOOL (__stdcall *			pFnCallBack) (LPVOID, PDS_REPSYNCALL_UPDATEA),
    _In_opt_ LPVOID				pCallbackData,
    _Outptr_result_maybenull_z_ PDS_REPSYNCALL_ERRINFOA **		pErrors
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaSyncAllW (
    _In_ HANDLE				hDS,
    _In_ LPCWSTR				pszNameContext,
    ULONG				ulFlags,
    __callback BOOL (__stdcall *			pFnCallBack) (LPVOID, PDS_REPSYNCALL_UPDATEW),
    _In_opt_ LPVOID				pCallbackData,
    _Outptr_result_maybenull_z_ PDS_REPSYNCALL_ERRINFOW **		pErrors
    );

#ifdef UNICODE
#define DsReplicaSyncAll DsReplicaSyncAllW
#else
#define DsReplicaSyncAll DsReplicaSyncAllA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsRemoveDsServerW(
    _In_ HANDLE  hDs,             // in
    _In_ LPWSTR  ServerDN,        // in
    _In_opt_ LPWSTR  DomainDN,        // in,  optional
    _Out_opt_ BOOL   *fLastDcInDomain, // out, optional
    BOOL    fCommit          // in
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsRemoveDsServerA(
    _In_ HANDLE  hDs,              // in
    _In_ LPSTR   ServerDN,         // in
    _In_opt_ LPSTR   DomainDN,         // in,  optional
    _Out_opt_ BOOL   *fLastDcInDomain,  // out, optional
    BOOL    fCommit           // in
    );

#ifdef UNICODE
#define DsRemoveDsServer DsRemoveDsServerW
#else
#define DsRemoveDsServer DsRemoveDsServerA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsRemoveDsDomainW(
    _In_ HANDLE  hDs,               // in
    _In_ LPWSTR  DomainDN           // in
    );

_Check_return_
NTDSAPI
DWORD
WINAPI
DsRemoveDsDomainA(
    _In_ HANDLE  hDs,               // in
    _In_ LPSTR   DomainDN           // in
    );

#ifdef UNICODE
#define DsRemoveDsDomain DsRemoveDsDomainW
#else
#define DsRemoveDsDomain DsRemoveDsDomainA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListSitesA(
    _In_ HANDLE              hDs,            // in
    _Outptr_ PDS_NAME_RESULTA    *ppSites);      // out

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListSitesW(
    _In_ HANDLE              hDs,            // in
    _Outptr_ PDS_NAME_RESULTW    *ppSites);      // out

#ifdef UNICODE
#define DsListSites DsListSitesW
#else
#define DsListSites DsListSitesA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListServersInSiteA(
    _In_ HANDLE              hDs,            // in
    _In_ LPCSTR              site,           // in
    _Outptr_ PDS_NAME_RESULTA    *ppServers);    // out

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListServersInSiteW(
    _In_ HANDLE              hDs,            // in
    _In_ LPCWSTR             site,           // in
    _Outptr_ PDS_NAME_RESULTW    *ppServers);    // out

#ifdef UNICODE
#define DsListServersInSite DsListServersInSiteW
#else
#define DsListServersInSite DsListServersInSiteA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListDomainsInSiteA(
    _In_ HANDLE              hDs,            // in
    _In_ LPCSTR              site,           // in
    _Outptr_ PDS_NAME_RESULTA    *ppDomains);    // out

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListDomainsInSiteW(
    _In_ HANDLE              hDs,            // in
    _In_ LPCWSTR             site,           // in
    _Outptr_ PDS_NAME_RESULTW    *ppDomains);    // out

#ifdef UNICODE
#define DsListDomainsInSite DsListDomainsInSiteW
#else
#define DsListDomainsInSite DsListDomainsInSiteA
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListServersForDomainInSiteA(
    _In_ HANDLE              hDs,            // in
    _In_ LPCSTR              domain,         // in
    _In_ LPCSTR              site,           // in
    _Outptr_ PDS_NAME_RESULTA    *ppServers);    // out

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListServersForDomainInSiteW(
    _In_ HANDLE              hDs,            // in
    _In_ LPCWSTR             domain,         // in
    _In_ LPCWSTR             site,           // in
    _Outptr_ PDS_NAME_RESULTW    *ppServers);    // out

#ifdef UNICODE
#define DsListServersForDomainInSite DsListServersForDomainInSiteW
#else
#define DsListServersForDomainInSite DsListServersForDomainInSiteA
#endif

// Define indices for DsListInfoForServer return data.  Check status
// for each field as a given value may not be present.

#define DS_LIST_DSA_OBJECT_FOR_SERVER       0
#define DS_LIST_DNS_HOST_NAME_FOR_SERVER    1
#define DS_LIST_ACCOUNT_OBJECT_FOR_SERVER   2

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListInfoForServerA(
    _In_ HANDLE              hDs,            // in
    _In_ LPCSTR              server,         // in
    _Outptr_ PDS_NAME_RESULTA    *ppInfo);       // out

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListInfoForServerW(
    _In_ HANDLE              hDs,            // in
    _In_ LPCWSTR             server,         // in
    _Outptr_ PDS_NAME_RESULTW    *ppInfo);       // out

#ifdef UNICODE
#define DsListInfoForServer DsListInfoForServerW
#else
#define DsListInfoForServer DsListInfoForServerA
#endif

// Define indices for DsListRoles return data.  Check status for
// each field as a given value may not be present.

#define DS_ROLE_SCHEMA_OWNER                0
#define DS_ROLE_DOMAIN_OWNER                1
#define DS_ROLE_PDC_OWNER                   2
#define DS_ROLE_RID_OWNER                   3
#define DS_ROLE_INFRASTRUCTURE_OWNER        4

_Check_return_
NTDSAPI
DWORD
WINAPI
DsListRolesA(
    _In_ HANDLE              hDs,            // in
    _Outptr_ PDS_NAME_RESULTA    *ppRoles);      // out


_Check_return_
NTDSAPI
DWORD
WINAPI
DsListRolesW(
    _In_ HANDLE              hDs,            // in
    _Outptr_ PDS_NAME_RESULTW    *ppRoles);      // out

#ifdef UNICODE
#define DsListRoles DsListRolesW
#else
#define DsListRoles DsListRolesA
#endif

//
// DsQuerySitesByCost{A|W} allows the caller to determine the
// communication cost between the From Site and each of the sites
// in the list of To Sites. The costs are returned in the rgSiteInfo
// structure which must be freed with DsQuerySitesFree.
//
// The Site Names should all be passed as RDNs. For example, if the
// site's DN is "CN=Foo,CN=Sites,CN=Configuration,...", the RDN is
// simply "Foo".
//

typedef struct {
    DWORD               errorCode;
    DWORD               cost;
} DS_SITE_COST_INFO, *PDS_SITE_COST_INFO;

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsQuerySitesByCostW(
    _In_ HANDLE              hDS,            // in
    _In_ LPWSTR          pwszFromSite,   // in
    _In_reads_ (cToSites) LPWSTR          *rgwszToSites,   // in
    _In_ DWORD               cToSites,       // in
    _Reserved_ DWORD               dwFlags,        // in
    _Outptr_ PDS_SITE_COST_INFO *prgSiteInfo     // out
    );

_Check_return_
NTDSAPI_POSTXP
DWORD
WINAPI
DsQuerySitesByCostA(
    _In_ HANDLE              hDS,            // in
    _In_ LPSTR           pszFromSite,    // in
    _In_reads_ (cToSites) LPSTR   *rgszToSites,    // in
    DWORD               cToSites,       // in
    _Reserved_ DWORD               dwFlags,        // in
    _Outptr_ PDS_SITE_COST_INFO *prgSiteInfo     // out
    );

#ifdef UNICODE
#define DsQuerySitesByCost DsQuerySitesByCostW
#else
#define DsQuerySitesByCost DsQuerySitesByCostA
#endif

//
// DsQuerySitesByCost will free the site info array returned
// from DsQuerySitesByCost{A|W}.
//
VOID
WINAPI
DsQuerySitesFree(
    _In_ PDS_SITE_COST_INFO  rgSiteInfo
    );

// Definitions required for DsMapSchemaGuid routines.

#define DS_SCHEMA_GUID_NOT_FOUND            0
#define DS_SCHEMA_GUID_ATTR                 1
#define DS_SCHEMA_GUID_ATTR_SET             2
#define DS_SCHEMA_GUID_CLASS                3
#define DS_SCHEMA_GUID_CONTROL_RIGHT        4

typedef struct
{
    GUID                    guid;       // mapped GUID
    DWORD                   guidType;   // DS_SCHEMA_GUID_* value
#ifdef MIDL_PASS
    [string,unique] CHAR    *pName;     // might be NULL
#else
    LPSTR                   pName;      // might be NULL
#endif

} DS_SCHEMA_GUID_MAPA, *PDS_SCHEMA_GUID_MAPA;

typedef struct
{
    GUID                    guid;       // mapped GUID
    DWORD                   guidType;   // DS_SCHEMA_GUID_* value
#ifdef MIDL_PASS
    [string,unique] WCHAR   *pName;     // might be NULL
#else
    LPWSTR                  pName;      // might be NULL
#endif

} DS_SCHEMA_GUID_MAPW, *PDS_SCHEMA_GUID_MAPW;

_Check_return_
NTDSAPI
DWORD
WINAPI
DsMapSchemaGuidsA(
    _In_ HANDLE                  hDs,            // in
    DWORD                   cGuids,         // in
    _In_reads_(cGuids) GUID                    *rGuids,        // in
    _Outptr_ DS_SCHEMA_GUID_MAPA     **ppGuidMap);   // out

NTDSAPI
VOID
WINAPI
DsFreeSchemaGuidMapA(
    _In_ PDS_SCHEMA_GUID_MAPA    pGuidMap);      // in

_Check_return_
NTDSAPI
DWORD
WINAPI
DsMapSchemaGuidsW(
    _In_ HANDLE                  hDs,            // in
    DWORD                   cGuids,         // in
    _In_reads_(cGuids) GUID                    *rGuids,        // in
    _Outptr_ DS_SCHEMA_GUID_MAPW     **ppGuidMap);   // out

NTDSAPI
VOID
WINAPI
DsFreeSchemaGuidMapW(
    _In_ PDS_SCHEMA_GUID_MAPW    pGuidMap);      // in

#ifdef UNICODE
#define DS_SCHEMA_GUID_MAP DS_SCHEMA_GUID_MAPW
#define PDS_SCHEMA_GUID_MAP PDS_SCHEMA_GUID_MAPW
#define DsMapSchemaGuids DsMapSchemaGuidsW
#define DsFreeSchemaGuidMap DsFreeSchemaGuidMapW
#else
#define DS_SCHEMA_GUID_MAP DS_SCHEMA_GUID_MAPA
#define PDS_SCHEMA_GUID_MAP PDS_SCHEMA_GUID_MAPA
#define DsMapSchemaGuids DsMapSchemaGuidsA
#define DsFreeSchemaGuidMap DsFreeSchemaGuidMapA
#endif

typedef struct
{
#ifdef MIDL_PASS
    [string,unique] CHAR    *NetbiosName;           // might be NULL
    [string,unique] CHAR    *DnsHostName;           // might be NULL
    [string,unique] CHAR    *SiteName;              // might be NULL
    [string,unique] CHAR    *ComputerObjectName;    // might be NULL
    [string,unique] CHAR    *ServerObjectName;      // might be NULL
#else
    LPSTR                   NetbiosName;            // might be NULL
    LPSTR                   DnsHostName;            // might be NULL
    LPSTR                   SiteName;               // might be NULL
    LPSTR                   ComputerObjectName;     // might be NULL
    LPSTR                   ServerObjectName;       // might be NULL
#endif
    BOOL                    fIsPdc;
    BOOL                    fDsEnabled;

} DS_DOMAIN_CONTROLLER_INFO_1A, *PDS_DOMAIN_CONTROLLER_INFO_1A;

typedef struct
{
#ifdef MIDL_PASS
    [string,unique] WCHAR   *NetbiosName;           // might be NULL
    [string,unique] WCHAR   *DnsHostName;           // might be NULL
    [string,unique] WCHAR   *SiteName;              // might be NULL
    [string,unique] WCHAR   *ComputerObjectName;    // might be NULL
    [string,unique] WCHAR   *ServerObjectName;      // might be NULL
#else
    LPWSTR                  NetbiosName;            // might be NULL
    LPWSTR                  DnsHostName;            // might be NULL
    LPWSTR                  SiteName;               // might be NULL
    LPWSTR                  ComputerObjectName;     // might be NULL
    LPWSTR                  ServerObjectName;       // might be NULL
#endif
    BOOL                    fIsPdc;
    BOOL                    fDsEnabled;

} DS_DOMAIN_CONTROLLER_INFO_1W, *PDS_DOMAIN_CONTROLLER_INFO_1W;

typedef struct
{
#ifdef MIDL_PASS
    [string,unique] CHAR    *NetbiosName;           // might be NULL
    [string,unique] CHAR    *DnsHostName;           // might be NULL
    [string,unique] CHAR    *SiteName;              // might be NULL
    [string,unique] CHAR    *SiteObjectName;        // might be NULL
    [string,unique] CHAR    *ComputerObjectName;    // might be NULL
    [string,unique] CHAR    *ServerObjectName;      // might be NULL
    [string,unique] CHAR    *NtdsDsaObjectName;     // might be NULL
#else
    LPSTR                   NetbiosName;            // might be NULL
    LPSTR                   DnsHostName;            // might be NULL
    LPSTR                   SiteName;               // might be NULL
    LPSTR                   SiteObjectName;         // might be NULL
    LPSTR                   ComputerObjectName;     // might be NULL
    LPSTR                   ServerObjectName;       // might be NULL
    LPSTR                   NtdsDsaObjectName;      // might be NULL
#endif
    BOOL                    fIsPdc;
    BOOL                    fDsEnabled;
    BOOL                    fIsGc;

    // Valid iff SiteObjectName non-NULL.
    GUID                    SiteObjectGuid;
    // Valid iff ComputerObjectName non-NULL.
    GUID                    ComputerObjectGuid;
    // Valid iff ServerObjectName non-NULL;
    GUID                    ServerObjectGuid;
    // Valid iff fDsEnabled is TRUE.
    GUID                    NtdsDsaObjectGuid;

} DS_DOMAIN_CONTROLLER_INFO_2A, *PDS_DOMAIN_CONTROLLER_INFO_2A;

typedef struct
{
#ifdef MIDL_PASS
    [string,unique] WCHAR   *NetbiosName;           // might be NULL
    [string,unique] WCHAR   *DnsHostName;           // might be NULL
    [string,unique] WCHAR   *SiteName;              // might be NULL
    [string,unique] WCHAR   *SiteObjectName;        // might be NULL
    [string,unique] WCHAR   *ComputerObjectName;    // might be NULL
    [string,unique] WCHAR   *ServerObjectName;      // might be NULL
    [string,unique] WCHAR   *NtdsDsaObjectName;     // might be NULL
#else
    LPWSTR                  NetbiosName;            // might be NULL
    LPWSTR                  DnsHostName;            // might be NULL
    LPWSTR                  SiteName;               // might be NULL
    LPWSTR                  SiteObjectName;         // might be NULL
    LPWSTR                  ComputerObjectName;     // might be NULL
    LPWSTR                  ServerObjectName;       // might be NULL
    LPWSTR                  NtdsDsaObjectName;      // might be NULL
#endif
    BOOL                    fIsPdc;
    BOOL                    fDsEnabled;
    BOOL                    fIsGc;

    // Valid iff SiteObjectName non-NULL.
    GUID                    SiteObjectGuid;
    // Valid iff ComputerObjectName non-NULL.
    GUID                    ComputerObjectGuid;
    // Valid iff ServerObjectName non-NULL;
    GUID                    ServerObjectGuid;
    // Valid iff fDsEnabled is TRUE.
    GUID                    NtdsDsaObjectGuid;

} DS_DOMAIN_CONTROLLER_INFO_2W, *PDS_DOMAIN_CONTROLLER_INFO_2W;

typedef struct
{
#ifdef MIDL_PASS
    [string,unique] CHAR    *NetbiosName;           // might be NULL
    [string,unique] CHAR    *DnsHostName;           // might be NULL
    [string,unique] CHAR    *SiteName;              // might be NULL
    [string,unique] CHAR    *SiteObjectName;        // might be NULL
    [string,unique] CHAR    *ComputerObjectName;    // might be NULL
    [string,unique] CHAR    *ServerObjectName;      // might be NULL
    [string,unique] CHAR    *NtdsDsaObjectName;     // might be NULL
#else
    LPSTR                   NetbiosName;            // might be NULL
    LPSTR                   DnsHostName;            // might be NULL
    LPSTR                   SiteName;               // might be NULL
    LPSTR                   SiteObjectName;         // might be NULL
    LPSTR                   ComputerObjectName;     // might be NULL
    LPSTR                   ServerObjectName;       // might be NULL
    LPSTR                   NtdsDsaObjectName;      // might be NULL
#endif
    BOOL                    fIsPdc;
    BOOL                    fDsEnabled;
    BOOL                    fIsGc;
    BOOL                    fIsRodc;

    // Valid iff SiteObjectName non-NULL.
    GUID                    SiteObjectGuid;
    // Valid iff ComputerObjectName non-NULL.
    GUID                    ComputerObjectGuid;
    // Valid iff ServerObjectName non-NULL;
    GUID                    ServerObjectGuid;
    // Valid iff fDsEnabled is TRUE.
    GUID                    NtdsDsaObjectGuid;

} DS_DOMAIN_CONTROLLER_INFO_3A, *PDS_DOMAIN_CONTROLLER_INFO_3A;

typedef struct
{
#ifdef MIDL_PASS
    [string,unique] WCHAR   *NetbiosName;           // might be NULL
    [string,unique] WCHAR   *DnsHostName;           // might be NULL
    [string,unique] WCHAR   *SiteName;              // might be NULL
    [string,unique] WCHAR   *SiteObjectName;        // might be NULL
    [string,unique] WCHAR   *ComputerObjectName;    // might be NULL
    [string,unique] WCHAR   *ServerObjectName;      // might be NULL
    [string,unique] WCHAR   *NtdsDsaObjectName;     // might be NULL
#else
    LPWSTR                  NetbiosName;            // might be NULL
    LPWSTR                  DnsHostName;            // might be NULL
    LPWSTR                  SiteName;               // might be NULL
    LPWSTR                  SiteObjectName;         // might be NULL
    LPWSTR                  ComputerObjectName;     // might be NULL
    LPWSTR                  ServerObjectName;       // might be NULL
    LPWSTR                  NtdsDsaObjectName;      // might be NULL
#endif
    BOOL                    fIsPdc;
    BOOL                    fDsEnabled;
    BOOL                    fIsGc;
    BOOL                    fIsRodc;

    // Valid iff SiteObjectName non-NULL.
    GUID                    SiteObjectGuid;
    // Valid iff ComputerObjectName non-NULL.
    GUID                    ComputerObjectGuid;
    // Valid iff ServerObjectName non-NULL;
    GUID                    ServerObjectGuid;
    // Valid iff fDsEnabled is TRUE.
    GUID                    NtdsDsaObjectGuid;

} DS_DOMAIN_CONTROLLER_INFO_3W, *PDS_DOMAIN_CONTROLLER_INFO_3W;

// The following APIs strictly find domain controller account objects
// in the DS and return information associated with them.  As such, they
// may return entries which correspond to domain controllers long since
// decommissioned, etc. and there is no guarantee that there exists a
// physical domain controller at all.  Use DsGetDcName (dsgetdc.h) to find
// live domain controllers for a domain.

_Check_return_
_When_(InfoLevel == 1, _At_(ppInfo, _Outptr_result_bytebuffer_(*pcOut * sizeof(DS_DOMAIN_CONTROLLER_INFO_1A))))
_When_(InfoLevel == 2, _At_(ppInfo, _Outptr_result_bytebuffer_(*pcOut * sizeof(DS_DOMAIN_CONTROLLER_INFO_2A))))
_When_(InfoLevel == 3, _At_(ppInfo, _Outptr_result_bytebuffer_(*pcOut * sizeof(DS_DOMAIN_CONTROLLER_INFO_3A))))
NTDSAPI
DWORD
WINAPI
DsGetDomainControllerInfoA(
    _In_ HANDLE                          hDs,            // in
    _In_ LPCSTR                          DomainName,     // in
    _In_ DWORD                           InfoLevel,      // in
    _Out_ DWORD                          *pcOut,         // out
    _Outptr_ VOID                     **ppInfo);      // out

_Check_return_
_When_(InfoLevel == 1, _At_(ppInfo, _Outptr_result_bytebuffer_(*pcOut * sizeof(DS_DOMAIN_CONTROLLER_INFO_1W))))
_When_(InfoLevel == 2, _At_(ppInfo, _Outptr_result_bytebuffer_(*pcOut * sizeof(DS_DOMAIN_CONTROLLER_INFO_2W))))
_When_(InfoLevel == 3, _At_(ppInfo, _Outptr_result_bytebuffer_(*pcOut * sizeof(DS_DOMAIN_CONTROLLER_INFO_3W))))
NTDSAPI
DWORD
WINAPI
DsGetDomainControllerInfoW(
    _In_ HANDLE                          hDs,            // in
    _In_ LPCWSTR                         DomainName,     // in
    _In_ DWORD                           InfoLevel,      // in
    _Out_ DWORD                          *pcOut,         // out
    _Outptr_ VOID                     **ppInfo);      // out

NTDSAPI
VOID
WINAPI
DsFreeDomainControllerInfoA(
    DWORD                           InfoLevel,      // in
    DWORD                           cInfo,          // in
    _In_reads_(cInfo) VOID                            *pInfo);        // in

NTDSAPI
VOID
WINAPI
DsFreeDomainControllerInfoW(
    DWORD                           InfoLevel,      // in
    DWORD                           cInfo,          // in
    _In_reads_(cInfo) VOID                            *pInfo);        // in

#ifdef UNICODE
#define DS_DOMAIN_CONTROLLER_INFO_1 DS_DOMAIN_CONTROLLER_INFO_1W
#define DS_DOMAIN_CONTROLLER_INFO_2 DS_DOMAIN_CONTROLLER_INFO_2W
#define DS_DOMAIN_CONTROLLER_INFO_3 DS_DOMAIN_CONTROLLER_INFO_3W
#define PDS_DOMAIN_CONTROLLER_INFO_1 PDS_DOMAIN_CONTROLLER_INFO_1W
#define PDS_DOMAIN_CONTROLLER_INFO_2 PDS_DOMAIN_CONTROLLER_INFO_2W
#define PDS_DOMAIN_CONTROLLER_INFO_3 PDS_DOMAIN_CONTROLLER_INFO_3W
#define DsGetDomainControllerInfo DsGetDomainControllerInfoW
#define DsFreeDomainControllerInfo DsFreeDomainControllerInfoW
#else
#define DS_DOMAIN_CONTROLLER_INFO_1 DS_DOMAIN_CONTROLLER_INFO_1A
#define DS_DOMAIN_CONTROLLER_INFO_2 DS_DOMAIN_CONTROLLER_INFO_2A
#define DS_DOMAIN_CONTROLLER_INFO_3 DS_DOMAIN_CONTROLLER_INFO_3A
#define PDS_DOMAIN_CONTROLLER_INFO_1 PDS_DOMAIN_CONTROLLER_INFO_1A
#define PDS_DOMAIN_CONTROLLER_INFO_2 PDS_DOMAIN_CONTROLLER_INFO_2A
#define PDS_DOMAIN_CONTROLLER_INFO_3 PDS_DOMAIN_CONTROLLER_INFO_3A
#define DsGetDomainControllerInfo DsGetDomainControllerInfoA
#define DsFreeDomainControllerInfo DsFreeDomainControllerInfoA
#endif

// Which task should be run?
typedef enum {
    DS_KCC_TASKID_UPDATE_TOPOLOGY = 0
} DS_KCC_TASKID;

// Don't wait for completion of the task; queue it and return.
#define DS_KCC_FLAG_ASYNC_OP    (1 << 0)

// Don't enqueue the task if another queued task will run soon.
#define DS_KCC_FLAG_DAMPED      (1 << 1)

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaConsistencyCheck(
    _In_ HANDLE          hDS,        // in
    DS_KCC_TASKID   TaskID,     // in
    DWORD           dwFlags);   // in

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaVerifyObjectsW(
    _In_ HANDLE          hDS,        // in
    _In_ LPCWSTR         NameContext,// in
    _In_ const UUID *    pUuidDsaSrc,// in
    ULONG           ulOptions);   // in

_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaVerifyObjectsA(
    _In_ HANDLE          hDS,        // in
    _In_ LPCSTR          NameContext,// in
    _In_ const UUID *    pUuidDsaSrc,// in
    ULONG           ulOptions);   // in

#ifdef UNICODE
#define DsReplicaVerifyObjects DsReplicaVerifyObjectsW
#else
#define DsReplicaVerifyObjects DsReplicaVerifyObjectsA
#endif

// Do not delete objects on DsReplicaVerifyObjects call
#define DS_EXIST_ADVISORY_MODE (0x1)

typedef enum _DS_REPL_INFO_TYPE {
    DS_REPL_INFO_NEIGHBORS        = 0,          // returns DS_REPL_NEIGHBORS *
    DS_REPL_INFO_CURSORS_FOR_NC   = 1,          // returns DS_REPL_CURSORS *
    DS_REPL_INFO_METADATA_FOR_OBJ = 2,          // returns DS_REPL_OBJECT_META_DATA *
    DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES = 3,  // both return
    DS_REPL_INFO_KCC_DSA_LINK_FAILURES = 4,     //    DS_REPL_KCC_DSA_FAILURES *
    DS_REPL_INFO_PENDING_OPS      = 5,          // returns DS_REPL_PENDING_OPS *

    ////////////////////////////////////////////////////////////////////////////
    //
    //  The following info types are not supported by Windows 2000.  Calling
    //  DsReplicaGetInfo() with one of the types on a Windows 2000 client or
    //  where hDS is bound to a Windows 2000 DC will fail with
    //  ERROR_NOT_SUPPORTED.
    //

    DS_REPL_INFO_METADATA_FOR_ATTR_VALUE = 6,   // returns DS_REPL_ATTR_VALUE_META_DATA *
    DS_REPL_INFO_CURSORS_2_FOR_NC = 7,          // returns DS_REPL_CURSORS_2 *
    DS_REPL_INFO_CURSORS_3_FOR_NC = 8,          // returns DS_REPL_CURSORS_3 *
    DS_REPL_INFO_METADATA_2_FOR_OBJ = 9,        // returns DS_REPL_OBJECT_META_DATA_2 *
    DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE = 10,// returns DS_REPL_ATTR_VALUE_META_DATA_2 *

    // <- insert new DS_REPL_INFO_* types here.
    DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE = 11,   // returns DS_REPL_ATTR_VALUE_META_DATA_EXT *
    DS_REPL_INFO_TYPE_MAX
} DS_REPL_INFO_TYPE;

// Bit values for flags argument to DsReplicaGetInfo2
#define DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS      (0x00000001)

// Bit values for the dwReplicaFlags field of the DS_REPL_NEIGHBOR structure.
// Also used for the ulReplicaFlags argument to DsReplicaModify
#define DS_REPL_NBR_WRITEABLE                       (0x00000010)
#define DS_REPL_NBR_SYNC_ON_STARTUP                 (0x00000020)
#define DS_REPL_NBR_DO_SCHEDULED_SYNCS              (0x00000040)
#define DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT   (0x00000080)
#define DS_REPL_NBR_TWO_WAY_SYNC                    (0x00000200)
#define DS_REPL_NBR_NONGC_RO_REPLICA                (0x00000400)
#define DS_REPL_NBR_RETURN_OBJECT_PARENTS           (0x00000800)
#define DS_REPL_NBR_SELECT_SECRETS                  (0x00001000)
#define DS_REPL_NBR_FULL_SYNC_IN_PROGRESS           (0x00010000)
#define DS_REPL_NBR_FULL_SYNC_NEXT_PACKET           (0x00020000)
#define DS_REPL_NBR_GCSPN                           (0x00100000)
#define DS_REPL_NBR_NEVER_SYNCED                    (0x00200000)
#define DS_REPL_NBR_PREEMPTED                       (0x01000000)
#define DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS     (0x04000000)
#define DS_REPL_NBR_DISABLE_SCHEDULED_SYNC          (0x08000000)
#define DS_REPL_NBR_COMPRESS_CHANGES                (0x10000000)
#define DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS         (0x20000000)
#define DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET           (0x40000000)

// This is the mask of replica flags that may be changed on the DsReplicaModify
// call using the ulReplicaFlags parameter. The other flags are protected
// system flags.  The previous values of the system flags must be read in
// advance and merged into the ulReplicaFlags parameter unchanged.
#define DS_REPL_NBR_MODIFIABLE_MASK \
        ( \
        DS_REPL_NBR_SYNC_ON_STARTUP | \
        DS_REPL_NBR_DO_SCHEDULED_SYNCS | \
        DS_REPL_NBR_TWO_WAY_SYNC | \
        DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS | \
        DS_REPL_NBR_DISABLE_SCHEDULED_SYNC | \
        DS_REPL_NBR_COMPRESS_CHANGES | \
        DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS \
        )

typedef struct _DS_REPL_NEIGHBORW {
    LPWSTR      pszNamingContext;
    LPWSTR      pszSourceDsaDN;
    LPWSTR      pszSourceDsaAddress;
    LPWSTR      pszAsyncIntersiteTransportDN;
    DWORD       dwReplicaFlags;
    DWORD       dwReserved;         // alignment

    UUID        uuidNamingContextObjGuid;
    UUID        uuidSourceDsaObjGuid;
    UUID        uuidSourceDsaInvocationID;
    UUID        uuidAsyncIntersiteTransportObjGuid;

    USN         usnLastObjChangeSynced;
    USN         usnAttributeFilter;

    FILETIME    ftimeLastSyncSuccess;
    FILETIME    ftimeLastSyncAttempt;

    DWORD       dwLastSyncResult;
    DWORD       cNumConsecutiveSyncFailures;
} DS_REPL_NEIGHBORW;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_NEIGHBORW_BLOB {
    DWORD       oszNamingContext;
    DWORD       oszSourceDsaDN;
    DWORD       oszSourceDsaAddress;
    DWORD       oszAsyncIntersiteTransportDN;
    DWORD       dwReplicaFlags;
    DWORD       dwReserved;

    UUID        uuidNamingContextObjGuid;
    UUID        uuidSourceDsaObjGuid;
    UUID        uuidSourceDsaInvocationID;
    UUID        uuidAsyncIntersiteTransportObjGuid;

    USN         usnLastObjChangeSynced;
    USN         usnAttributeFilter;

    FILETIME    ftimeLastSyncSuccess;
    FILETIME    ftimeLastSyncAttempt;

    DWORD       dwLastSyncResult;
    DWORD       cNumConsecutiveSyncFailures;
} DS_REPL_NEIGHBORW_BLOB;

typedef struct _DS_REPL_NEIGHBORSW {
    DWORD       cNumNeighbors;
    DWORD       dwReserved;             // alignment
#ifdef MIDL_PASS
    [size_is(cNumNeighbors)] DS_REPL_NEIGHBORW rgNeighbor[];
#else
    DS_REPL_NEIGHBORW rgNeighbor[1];
#endif
} DS_REPL_NEIGHBORSW;

typedef struct _DS_REPL_CURSOR {
    UUID        uuidSourceDsaInvocationID;
    USN         usnAttributeFilter;
} DS_REPL_CURSOR;

typedef struct _DS_REPL_CURSOR_2 {
    UUID        uuidSourceDsaInvocationID;
    USN         usnAttributeFilter;
    FILETIME    ftimeLastSyncSuccess;
} DS_REPL_CURSOR_2;

typedef struct _DS_REPL_CURSOR_3W {
    UUID        uuidSourceDsaInvocationID;
    USN         usnAttributeFilter;
    FILETIME    ftimeLastSyncSuccess;
    LPWSTR      pszSourceDsaDN;
} DS_REPL_CURSOR_3W;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_CURSOR_BLOB {
    UUID        uuidSourceDsaInvocationID;
    USN         usnAttributeFilter;
    FILETIME    ftimeLastSyncSuccess;
    DWORD       oszSourceDsaDN;
} DS_REPL_CURSOR_BLOB;

typedef struct _DS_REPL_CURSORS {
    DWORD       cNumCursors;
    DWORD       dwReserved;             // alignment
#ifdef MIDL_PASS
    [size_is(cNumCursors)] DS_REPL_CURSOR rgCursor[];
#else
    DS_REPL_CURSOR rgCursor[1];
#endif
} DS_REPL_CURSORS;

typedef struct _DS_REPL_CURSORS_2 {
    DWORD       cNumCursors;
    DWORD       dwEnumerationContext;
    // keep this 8 byte aligned
#ifdef MIDL_PASS
    [size_is(cNumCursors)] DS_REPL_CURSOR_2 rgCursor[];
#else
    DS_REPL_CURSOR_2 rgCursor[1];
#endif
} DS_REPL_CURSORS_2;

typedef struct _DS_REPL_CURSORS_3W {
    DWORD       cNumCursors;
    DWORD       dwEnumerationContext;
    // keep this 8 byte aligned
#ifdef MIDL_PASS
    [size_is(cNumCursors)] DS_REPL_CURSOR_3W rgCursor[];
#else
    DS_REPL_CURSOR_3W rgCursor[1];
#endif
} DS_REPL_CURSORS_3W;

typedef struct _DS_REPL_ATTR_META_DATA {
    LPWSTR      pszAttributeName;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
} DS_REPL_ATTR_META_DATA;

typedef struct _DS_REPL_ATTR_META_DATA_2 {
    LPWSTR      pszAttributeName;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
    LPWSTR      pszLastOriginatingDsaDN;
} DS_REPL_ATTR_META_DATA_2;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_ATTR_META_DATA_BLOB {
    DWORD       oszAttributeName;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
    DWORD       oszLastOriginatingDsaDN;
} DS_REPL_ATTR_META_DATA_BLOB;

typedef struct _DS_REPL_OBJ_META_DATA {
    DWORD       cNumEntries;
    DWORD       dwReserved;             // alignment
#ifdef MIDL_PASS
    [size_is(cNumEntries)] DS_REPL_ATTR_META_DATA rgMetaData[];
#else
    DS_REPL_ATTR_META_DATA rgMetaData[1];
#endif
} DS_REPL_OBJ_META_DATA;

typedef struct _DS_REPL_OBJ_META_DATA_2 {
    DWORD       cNumEntries;
    DWORD       dwReserved;             // alignment
#ifdef MIDL_PASS
    [size_is(cNumEntries)] DS_REPL_ATTR_META_DATA_2 rgMetaData[];
#else
    DS_REPL_ATTR_META_DATA_2 rgMetaData[1];
#endif
} DS_REPL_OBJ_META_DATA_2;

typedef struct _DS_REPL_KCC_DSA_FAILUREW {
    LPWSTR      pszDsaDN;
    UUID        uuidDsaObjGuid;
    FILETIME    ftimeFirstFailure;
    DWORD       cNumFailures;
    DWORD       dwLastResult;   // Win32 error code
} DS_REPL_KCC_DSA_FAILUREW;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_KCC_DSA_FAILUREW_BLOB {
    DWORD       oszDsaDN;
    UUID        uuidDsaObjGuid;
    FILETIME    ftimeFirstFailure;
    DWORD       cNumFailures;
    DWORD       dwLastResult;   // Win32 error code
} DS_REPL_KCC_DSA_FAILUREW_BLOB;

typedef struct _DS_REPL_KCC_DSA_FAILURESW {
    DWORD       cNumEntries;
    DWORD       dwReserved;             // alignment
#ifdef MIDL_PASS
    [size_is(cNumEntries)] DS_REPL_KCC_DSA_FAILUREW rgDsaFailure[];
#else
    DS_REPL_KCC_DSA_FAILUREW rgDsaFailure[1];
#endif
} DS_REPL_KCC_DSA_FAILURESW;

typedef enum _DS_REPL_OP_TYPE {
    DS_REPL_OP_TYPE_SYNC = 0,
    DS_REPL_OP_TYPE_ADD,
    DS_REPL_OP_TYPE_DELETE,
    DS_REPL_OP_TYPE_MODIFY,
    DS_REPL_OP_TYPE_UPDATE_REFS
} DS_REPL_OP_TYPE;

typedef struct _DS_REPL_OPW {
    FILETIME        ftimeEnqueued;  // time at which the operation was enqueued
    ULONG           ulSerialNumber; // ID of this sync; unique per machine per boot
    ULONG           ulPriority;     // > priority, > urgency
    DS_REPL_OP_TYPE OpType;

    ULONG           ulOptions;      // Zero or more bits specific to OpType; e.g.,
                                    //  DS_REPADD_* for DS_REPL_OP_TYPE_ADD,
                                    //  DS_REPSYNC_* for DS_REPL_OP_TYPE_SYNC, etc.
    LPWSTR          pszNamingContext;
    LPWSTR          pszDsaDN;
    LPWSTR          pszDsaAddress;

    UUID            uuidNamingContextObjGuid;
    UUID            uuidDsaObjGuid;
} DS_REPL_OPW;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_OPW_BLOB {
    FILETIME        ftimeEnqueued;  // time at which the operation was enqueued
    ULONG           ulSerialNumber; // ID of this sync; unique per machine per boot
    ULONG           ulPriority;     // > priority, > urgency
    DS_REPL_OP_TYPE OpType;

    ULONG           ulOptions;      // Zero or more bits specific to OpType; e.g.,
                                    //  DS_REPADD_* for DS_REPL_OP_TYPE_ADD,
                                    //  DS_REPSYNC_* for DS_REPL_OP_TYPE_SYNC, etc.
    DWORD           oszNamingContext;
    DWORD           oszDsaDN;
    DWORD           oszDsaAddress;

    UUID            uuidNamingContextObjGuid;
    UUID            uuidDsaObjGuid;
} DS_REPL_OPW_BLOB;

typedef struct _DS_REPL_PENDING_OPSW {
    FILETIME            ftimeCurrentOpStarted;
    DWORD               cNumPendingOps;
#ifdef MIDL_PASS
    [size_is(cNumPendingOps)] DS_REPL_OPW rgPendingOp[];
#else
    DS_REPL_OPW         rgPendingOp[1];
#endif
} DS_REPL_PENDING_OPSW;

typedef struct _DS_REPL_VALUE_META_DATA {
    LPWSTR      pszAttributeName;
    LPWSTR      pszObjectDn;
    DWORD       cbData;
#ifdef MIDL_PASS
    [size_is(cbData), ptr] BYTE        *pbData;
#else
    BYTE        *pbData;
#endif
    FILETIME    ftimeDeleted;
    FILETIME    ftimeCreated;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
} DS_REPL_VALUE_META_DATA;

typedef struct _DS_REPL_VALUE_META_DATA_2 {
    LPWSTR      pszAttributeName;
    LPWSTR      pszObjectDn;
    DWORD       cbData;
#ifdef MIDL_PASS
    [size_is(cbData), ptr] BYTE        *pbData;
#else
    BYTE        *pbData;
#endif
    FILETIME    ftimeDeleted;
    FILETIME    ftimeCreated;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
    LPWSTR      pszLastOriginatingDsaDN;
} DS_REPL_VALUE_META_DATA_2;

typedef struct _DS_REPL_VALUE_META_DATA_EXT {
    LPWSTR      pszAttributeName;
    LPWSTR      pszObjectDn;
    DWORD       cbData;
#ifdef MIDL_PASS
    [size_is(cbData), ptr] BYTE        *pbData;
#else
    BYTE        *pbData;
#endif
    FILETIME    ftimeDeleted;
    FILETIME    ftimeCreated;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
    LPWSTR      pszLastOriginatingDsaDN;
    DWORD       dwUserIdentifier;
    DWORD       dwPriorLinkState;
    DWORD       dwCurrentLinkState;
} DS_REPL_VALUE_META_DATA_EXT;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_VALUE_META_DATA_BLOB {
    DWORD       oszAttributeName;
    DWORD       oszObjectDn;
    DWORD       cbData;
    DWORD       obData;
    FILETIME    ftimeDeleted;
    FILETIME    ftimeCreated;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
    DWORD       oszLastOriginatingDsaDN;
} DS_REPL_VALUE_META_DATA_BLOB;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_VALUE_META_DATA_BLOB_EXT {
    DWORD       oszAttributeName;
    DWORD       oszObjectDn;
    DWORD       cbData;
    DWORD       obData;
    FILETIME    ftimeDeleted;
    FILETIME    ftimeCreated;
    DWORD       dwVersion;
    FILETIME    ftimeLastOriginatingChange;
    UUID        uuidLastOriginatingDsaInvocationID;
    USN         usnOriginatingChange;   // in the originating DSA's USN space
    USN         usnLocalChange;         // in the local DSA's USN space
    DWORD       oszLastOriginatingDsaDN;
    DWORD       dwUserIdentifier;
    DWORD       dwPriorLinkState;
    DWORD       dwCurrentLinkState;
} DS_REPL_VALUE_META_DATA_BLOB_EXT;

typedef struct _DS_REPL_ATTR_VALUE_META_DATA {
    DWORD       cNumEntries;
    DWORD       dwEnumerationContext;
#ifdef MIDL_PASS
    [size_is(cNumEntries)] DS_REPL_VALUE_META_DATA rgMetaData[];
#else
    DS_REPL_VALUE_META_DATA rgMetaData[1];
#endif
} DS_REPL_ATTR_VALUE_META_DATA;

typedef struct _DS_REPL_ATTR_VALUE_META_DATA_2 {
    DWORD       cNumEntries;
    DWORD       dwEnumerationContext;
#ifdef MIDL_PASS
    [size_is(cNumEntries)] DS_REPL_VALUE_META_DATA_2 rgMetaData[];
#else
    DS_REPL_VALUE_META_DATA_2 rgMetaData[1];
#endif
} DS_REPL_ATTR_VALUE_META_DATA_2;

typedef struct _DS_REPL_ATTR_VALUE_META_DATA_EXT {
    DWORD       cNumEntries;
    DWORD       dwEnumerationContext;
#ifdef MIDL_PASS
    [size_is(cNumEntries)] DS_REPL_VALUE_META_DATA_EXT rgMetaData[];
#else
    DS_REPL_VALUE_META_DATA_EXT rgMetaData[1];
#endif
} DS_REPL_ATTR_VALUE_META_DATA_EXT;

typedef struct _DS_REPL_QUEUE_STATISTICSW
{
    FILETIME ftimeCurrentOpStarted;
    DWORD cNumPendingOps;
    FILETIME ftimeOldestSync;
    FILETIME ftimeOldestAdd;
    FILETIME ftimeOldestMod;
    FILETIME ftimeOldestDel;
    FILETIME ftimeOldestUpdRefs;
} DS_REPL_QUEUE_STATISTICSW;

// Fields can be added only to the end of this structure.
typedef struct _DS_REPL_QUEUE_STATISTICSW DS_REPL_QUEUE_STATISTICSW_BLOB;


_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaGetInfoW(
    _In_ HANDLE              hDS,                        // in
    DS_REPL_INFO_TYPE   InfoType,                   // in
    _In_opt_ LPCWSTR             pszObject,                  // in
    _In_opt_ UUID *              puuidForSourceDsaObjGuid,   // in
    _Outptr_ VOID **             ppInfo);                    // out

// This API is not supported by Windows 2000 clients or Windows 2000 DCs.
_Check_return_
NTDSAPI
DWORD
WINAPI
DsReplicaGetInfo2W(
    _In_ HANDLE              hDS,                        // in
    DS_REPL_INFO_TYPE   InfoType,                   // in
    _In_opt_ LPCWSTR             pszObject,                  // in
    _In_opt_ UUID *              puuidForSourceDsaObjGuid,   // in
    _In_opt_ LPCWSTR             pszAttributeName,           // in
    _In_opt_ LPCWSTR             pszValue,                   // in
    DWORD               dwFlags,                    // in
    DWORD               dwEnumerationContext,       // in
    _Outptr_ VOID **             ppInfo);                    // out

NTDSAPI
void
WINAPI
DsReplicaFreeInfo(
    DS_REPL_INFO_TYPE   InfoType,   // in
    _In_ VOID *              pInfo);     // in


#ifdef UNICODE
#define DsReplicaGetInfo          DsReplicaGetInfoW
#define DsReplicaGetInfo2         DsReplicaGetInfo2W
#define DS_REPL_NEIGHBOR          DS_REPL_NEIGHBORW
#define DS_REPL_NEIGHBORS         DS_REPL_NEIGHBORSW
#define DS_REPL_CURSOR_3          DS_REPL_CURSOR_3W
#define DS_REPL_CURSORS_3         DS_REPL_CURSORS_3W
#define DS_REPL_KCC_DSA_FAILURES  DS_REPL_KCC_DSA_FAILURESW
#define DS_REPL_KCC_DSA_FAILURE   DS_REPL_KCC_DSA_FAILUREW
#define DS_REPL_OP                DS_REPL_OPW
#define DS_REPL_PENDING_OPS       DS_REPL_PENDING_OPSW
#else
// No ANSI equivalents currently supported.
#endif

_Check_return_
NTDSAPI
DWORD
WINAPI
DsAddSidHistoryW(
    _In_ HANDLE                  hDS,                    // in
    _Reserved_ DWORD                   Flags,                  // in - sbz for now
    _In_ LPCWSTR                 SrcDomain,              // in - DNS or NetBIOS
    _In_ LPCWSTR                 SrcPrincipal,           // in - SAM account name
    _In_opt_ LPCWSTR                 SrcDomainController,    // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE SrcDomainCreds,        // in - creds for src domain
    _In_ LPCWSTR                 DstDomain,              // in - DNS or NetBIOS
    _In_ LPCWSTR                 DstPrincipal);          // in - SAM account name

_Check_return_
NTDSAPI
DWORD
WINAPI
DsAddSidHistoryA(
    _In_ HANDLE                  hDS,                    // in
    _Reserved_ DWORD                   Flags,                  // in - sbz for now
    _In_ LPCSTR                  SrcDomain,              // in - DNS or NetBIOS
    _In_ LPCSTR                  SrcPrincipal,           // in - SAM account name
    _In_opt_ LPCSTR                  SrcDomainController,    // in, optional
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE SrcDomainCreds,        // in - creds for src domain
    _In_ LPCSTR                  DstDomain,              // in - DNS or NetBIOS
    _In_ LPCSTR                  DstPrincipal);          // in - SAM account name

#ifdef UNICODE
#define DsAddSidHistory DsAddSidHistoryW
#else
#define DsAddSidHistory DsAddSidHistoryA
#endif

// The DsInheritSecurityIdentity API adds the source principal's SID and
// SID history to the destination principal's SID history and then DELETES
// THE SOURCE PRINCIPAL.  Source and destination principal must be in the
// same domain.

_Check_return_
NTDSAPI
DWORD
WINAPI
DsInheritSecurityIdentityW(
    _In_ HANDLE                  hDS,                    // in
    _Reserved_ DWORD                   Flags,                  // in - sbz for now
    _In_ LPCWSTR                 SrcPrincipal,           // in - distinguished name
    _In_ LPCWSTR                 DstPrincipal);          // in - distinguished name

_Check_return_
NTDSAPI
DWORD
WINAPI
DsInheritSecurityIdentityA(
    _In_ HANDLE                  hDS,                    // in
    _Reserved_ DWORD                   Flags,                  // in - sbz for now
    _In_ LPCSTR                  SrcPrincipal,           // in - distinguished name
    _In_ LPCSTR                  DstPrincipal);          // in - distinguished name

#ifdef UNICODE
#define DsInheritSecurityIdentity DsInheritSecurityIdentityW
#else
#define DsInheritSecurityIdentity DsInheritSecurityIdentityA
#endif

#ifndef MIDL_PASS
// -------------------------------------------------------------------------
// strings used by ADAM for constructing keywords values for SCP publication
// -------------------------------------------------------------------------

// Site name, e.g. "site:Default-First-Site-Name"
#define ADAM_SCP_SITE_NAME_STRING        "site:"
#define ADAM_SCP_SITE_NAME_STRING_W     L"site:"

// Partition DN, e.g. "partition:O=MSFT,L=WA,C=US"
#define ADAM_SCP_PARTITION_STRING        "partition:"
#define ADAM_SCP_PARTITION_STRING_W     L"partition:"

// Instance name, e.g. "instance:instance1"
#define ADAM_SCP_INSTANCE_NAME_STRING    "instance:"
#define ADAM_SCP_INSTANCE_NAME_STRING_W L"instance:"

// FSMO, e.g. "fsmo:naming"
#define ADAM_SCP_FSMO_STRING             "fsmo:"
#define ADAM_SCP_FSMO_STRING_W          L"fsmo:"

// FSMO values, e.g. "fsmo:naming"
#define ADAM_SCP_FSMO_NAMING_STRING      "naming"
#define ADAM_SCP_FSMO_NAMING_STRING_W   L"naming"

#define ADAM_SCP_FSMO_SCHEMA_STRING      "schema"
#define ADAM_SCP_FSMO_SCHEMA_STRING_W   L"schema"

// -------------------------------------------------------------------------
// ADAM repl authenticaion mode:  Stored as an attribute on the config NC.
// -------------------------------------------------------------------------

// Negotiate with pass-through authentication. All instances must run using
// service accounts with the same name and password.
#define ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH                0

// Negotiate authentication. If Kerberos is available, it will be used.
// Otherwise, authentication will fall back to NTLM (unless machine policy
// forbids this).
#define ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE                             1

// ADAM will require Kerberos mutual authentication.
#define ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED                  2

// -------------------------------------------------------------------------
// Optional Feature values:
// -------------------------------------------------------------------------

// Flags for the msDS-OptionalFeatureFlags attribute of the FeatureConfiguration object.
#define FLAG_FOREST_OPTIONAL_FEATURE     (0x00000001) // The optional feature is a forest level feature.
#define FLAG_DOMAIN_OPTIONAL_FEATURE     (0x00000002) // The optional feature is a domain level feature.
#define FLAG_DISABLABLE_OPTIONAL_FEATURE (0x00000004) // The optional feature may be turned off.
#define FLAG_SERVER_OPTIONAL_FEATURE     (0x00000008) // The optional feature is a server level feature..

// GUID of the Recycle Bin optional feature
#define GUID_RECYCLE_BIN_OPTIONAL_FEATURE_A     "d8dc6d76d0ac5e44f3b9a7f9b6744f2a"
#define GUID_RECYCLE_BIN_OPTIONAL_FEATURE_W    L"d8dc6d76d0ac5e44f3b9a7f9b6744f2a"
#define GUID_RECYCLE_BIN_OPTIONAL_FEATURE_BYTE  "\xd8\xdc\x6d\x76\xd0\xac\x5e\x44\xf3\xb9\xa7\xf9\xb6\x74\x4f\x2a"

#define GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_A    "73e843ece8cc4046b4ab07ffe4ab5bcd"
#define GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_W   L"73e843ece8cc4046b4ab07ffe4ab5bcd"
#define GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_BYTE "\x73\xe8\x43\xec\xe8\xcc\x40\x46\xb4\xab\x07\xff\xe4\xab\x5b\xcd"

// GUID for Database 32k Pages Feature {52982AC6-1E73-754F-AE24-73AE2775AAB8}
#define GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_A       "c62a9852731e4f75ae2473ae2775aab8"
#define GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_W       L"c62a9852731e4f75ae2473ae2775aab8"
#define GUID_DATABASE_32K_PAGES_OPTIONAL_FEATURE_BYTE    "\xc6\x2a\x98\x52\x73\x1e\x4f\x75\xae\x24\x73\xae\x27\x75\xaa\xb8"

#ifdef __cplusplus
}
#endif
#endif // !MIDL_PASS


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // _NTDSAPI_H_

