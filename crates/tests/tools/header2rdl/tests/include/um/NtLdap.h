/*++

Copyright (c) 1998-1999  Microsoft Corporation

Module Name:

    ntldap.h

Abstract:

   This is the header that defines NT specific server LDAP extensions.

Environments :

    Win32 user mode

--*/

#ifndef NT_LDAP_H
#define NT_LDAP_H

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
//
// Server controls section
//

//
// Force Update Control.  No Data.
//

#define LDAP_SERVER_FORCE_UPDATE_OID        "1.2.840.113556.1.4.1974"
#define LDAP_SERVER_FORCE_UPDATE_OID_W     L"1.2.840.113556.1.4.1974"


//
// Permissive Modify Control.  No Data.
//

#define LDAP_SERVER_PERMISSIVE_MODIFY_OID        "1.2.840.113556.1.4.1413"
#define LDAP_SERVER_PERMISSIVE_MODIFY_OID_W     L"1.2.840.113556.1.4.1413"


//
// Show Deleted Control.  No Data.
//

#define LDAP_SERVER_SHOW_DELETED_OID            "1.2.840.113556.1.4.417"
#define LDAP_SERVER_SHOW_DELETED_OID_W         L"1.2.840.113556.1.4.417"


//
// Show Recycled Control.  No Data.
//

#define LDAP_SERVER_SHOW_RECYCLED_OID           "1.2.840.113556.1.4.2064"
#define LDAP_SERVER_SHOW_RECYCLED_OID_W        L"1.2.840.113556.1.4.2064"

//
// Expected Entry Count control.
//
//      SEQUENCE {
//          MinEntries INTEGER
//          MaxEntries INTEGER
//          }
//      }
#define LDAP_SERVER_EXPECTED_ENTRY_COUNT_OID    "1.2.840.113556.1.4.2211"
#define LDAP_SERVER_EXPECTED_ENTRY_COUNT_OID_W L"1.2.840.113556.1.4.2211"

//
// Server Search Hints Control Request.
//
// SEQUENCE OF SEQUENCE {
//               hintId          OCTET STRING,
//               hintValue       OCTET STRING
//      }

#define LDAP_SERVER_SEARCH_HINTS_OID            "1.2.840.113556.1.4.2206"
#define LDAP_SERVER_SEARCH_HINTS_OID_W          L"1.2.840.113556.1.4.2206"

//
// Specific hintId values
//
#define LDAP_SEARCH_HINT_INDEX_ONLY_OID         "1.2.840.113556.1.4.2207"
#define LDAP_SEARCH_HINT_INDEX_ONLY_OID_W      L"1.2.840.113556.1.4.2207"
#define LDAP_SEARCH_HINT_SOFT_SIZE_LIMIT_OID    "1.2.840.113556.1.4.2210"
#define LDAP_SEARCH_HINT_SOFT_SIZE_LIMIT_OID_W L"1.2.840.113556.1.4.2210"
#define LDAP_SEARCH_HINT_REQUIRED_INDEX_OID     "1.2.840.113556.1.4.2306"
#define LDAP_SEARCH_HINT_REQUIRED_INDEX_OID_W  L"1.2.840.113556.1.4.2306"


//
// Server Update Stats Control Request.
//     No Data.
//
// Server Update Stats Control Response:
//     SEQUENCE OF SEQUENCE {
//               statId          LDAPOID
//               statValue       OCTET STRING
//      }

#define LDAP_SERVER_UPDATE_STATS_OID            "1.2.840.113556.1.4.2205"
#define LDAP_SERVER_UPDATE_STATS_OID_W         L"1.2.840.113556.1.4.2205"

//
// Specific statId values
//
#define LDAP_UPDATE_STATS_USN_OID              "1.2.840.113556.1.4.2208"
#define LDAP_UPDATE_STATS_USN_OID_W           L"1.2.840.113556.1.4.2208"
#define LDAP_UPDATE_STATS_INVOCATIONID_OID     "1.2.840.113556.1.4.2209"
#define LDAP_UPDATE_STATS_INVOCATIONID_OID_W  L"1.2.840.113556.1.4.2209"

//
// Server Search Stats Control Request
//    No Data
//
// Server Search Stats Control Response
//
//      SEQUENCE {
//          threadCountTag        INTEGER
//          threadCount           INTEGER
//          callTimeTag           INTEGER
//          callTime              INTEGER
//          entriesReturnedTag    INTEGER
//          entriesReturned       INTEGER
//          entriesVisitedTag     INTEGER
//          entriesVisited        INTEGER
//          filterTag             INTEGER
//          filter                OCTET STRING
//          indexTag              INTEGER
//          index                 OCTET STRING
//       }
//  
#define LDAP_SERVER_GET_STATS_OID                  "1.2.840.113556.1.4.970"
#define LDAP_SERVER_GET_STATS_OID_W                L"1.2.840.113556.1.4.970"

//
// Show Deactivated Links Control.  No Data.
//

#define LDAP_SERVER_SHOW_DEACTIVATED_LINK_OID     "1.2.840.113556.1.4.2065"
#define LDAP_SERVER_SHOW_DEACTIVATED_LINK_OID_W  L"1.2.840.113556.1.4.2065"

// Behavior hint control. Allows the client to pass in flags to control
// various behaviours. Data as follows
//      SEQUENCE {
//          Flags   INTEGER
//      }

//
//  N.B.:   Clients must use LDAP_SERVER_POLICY_HINTS_OID if the DC supports
//  it (check the supportedControl attribute), but may fall back to using 
//  LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID otherwise.  The behavior of the two
//  controls is identical.   Support for LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID
//  will eventually be phased out in a future release.
//

#define LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID     "1.2.840.113556.1.4.2066"
#define LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID_W  L"1.2.840.113556.1.4.2066"

#define LDAP_SERVER_POLICY_HINTS_OID     "1.2.840.113556.1.4.2239"
#define LDAP_SERVER_POLICY_HINTS_OID_W  L"1.2.840.113556.1.4.2239"

// 
// Server Supports Ranged Retrieval.
//
// This OID is not used in a request or a reply, but is returned as a value
// of the supportedControl Root DSE attribute
#define LDAP_SERVER_RANGE_OPTION_OID                "1.2.840.113556.1.4.802"
#define LDAP_SERVER_RANGE_OPTION_OID_W              L"1.2.840.113556.1.4.802"

//
// Cross Domain Move Control. Data as follows
//      SEQUENCE {
//          Name OCTET STRING
//      }
//

#define LDAP_SERVER_CROSSDOM_MOVE_TARGET_OID    "1.2.840.113556.1.4.521"
#define LDAP_SERVER_CROSSDOM_MOVE_TARGET_OID_W L"1.2.840.113556.1.4.521"

//
// Notification. No Data.
//

#define LDAP_SERVER_NOTIFICATION_OID            "1.2.840.113556.1.4.528"
#define LDAP_SERVER_NOTIFICATION_OID_W         L"1.2.840.113556.1.4.528"

//
// Shutdown Notification. No Data.
//

#define LDAP_SERVER_SHUTDOWN_NOTIFY_OID          "1.2.840.113556.1.4.1907"
#define LDAP_SERVER_SHUTDOWN_NOTIFY_OID_W       L"1.2.840.113556.1.4.1907"
    
//
// Lazy Commit. No Data.
//

#define LDAP_SERVER_LAZY_COMMIT_OID             "1.2.840.113556.1.4.619"
#define LDAP_SERVER_LAZY_COMMIT_OID_W          L"1.2.840.113556.1.4.619"

//
// Security Descriptor Flag. Data as follows
//      SEQUENCE {
//          Flags INTEGER
//      }
//

#define LDAP_SERVER_SD_FLAGS_OID                "1.2.840.113556.1.4.801"
#define LDAP_SERVER_SD_FLAGS_OID_W             L"1.2.840.113556.1.4.801"

//
// Extended Tree Delete Request:
//      SEQUENCE {
//          MaxObjectsToDelete   INTEGER
//      }

#define LDAP_SERVER_TREE_DELETE_EX_OID          "1.2.840.113556.1.4.2204"
#define LDAP_SERVER_TREE_DELETE_EX_OID_W       L"1.2.840.113556.1.4.2204"


//
// Tree Delete. No Data.
//

#define LDAP_SERVER_TREE_DELETE_OID             "1.2.840.113556.1.4.805"
#define LDAP_SERVER_TREE_DELETE_OID_W          L"1.2.840.113556.1.4.805"


//
// Attribute Scoped Query Request:
// SEQUENCE {
//        controlType   1.2.840.113556.1.4.1504 
//        controlValue  string
//        criticality   TRUE
// }
//
// Attribute Scoped Query Response:
// SEQUENCE {
//      result   ENUMERATED {
//           success (0),
//           invalidAttributeSyntax  (21),
//           unwillingToPerform      (53),
//           affectsMultipleDSAs     (71), 
//      }
// }
//

#define LDAP_SERVER_ASQ_OID                     "1.2.840.113556.1.4.1504"
#define LDAP_SERVER_ASQ_OID_W                  L"1.2.840.113556.1.4.1504"



//
// DirSync operation. Data as follows
//      SEQUENCE {
//          Flags   INTEGER
//          Size    INTEGER
//          Cookie  OCTET STRING
//      }
//
// Flags are listed below.
//

#define LDAP_SERVER_DIRSYNC_OID                 "1.2.840.113556.1.4.841"
#define LDAP_SERVER_DIRSYNC_OID_W              L"1.2.840.113556.1.4.841"

//
// Extended DirSync operation. Data as follows
//      SEQUENCE {
//          Flags   INTEGER
//          Size    INTEGER
//          Cookie  OCTET STRING
//      }
//
// Flags are listed below.
//

#define LDAP_SERVER_DIRSYNC_EX_OID                 "1.2.840.113556.1.4.2090"
#define LDAP_SERVER_DIRSYNC_EX_OID_W              L"1.2.840.113556.1.4.2090"

//
// Return extended DNs according to the requested format.  Optional data as
// follows
//      SEQUENCE {
//          Option  INTEGER
//      }
//
//  Option values:
//      0:  DN preceded by GUID and SID (if any) in hex string format
//      1:  DN preceded by GUID and SID (if any) in standard string format
//
//  If no data is provided then option 0 is selected for backwards compat.
//

#define LDAP_SERVER_EXTENDED_DN_OID             "1.2.840.113556.1.4.529"
#define LDAP_SERVER_EXTENDED_DN_OID_W          L"1.2.840.113556.1.4.529"

//
// Tell DC which server to verify with that a DN exist. Data as follows
//      SEQUENCE {
//          Flags   INTEGER,
//          ServerName OCTET STRING     // unicode server string
//      }
//

#define LDAP_SERVER_VERIFY_NAME_OID             "1.2.840.113556.1.4.1338"
#define LDAP_SERVER_VERIFY_NAME_OID_W          L"1.2.840.113556.1.4.1338"

//
// Tells server not to generate referrals
//

#define LDAP_SERVER_DOMAIN_SCOPE_OID            "1.2.840.113556.1.4.1339"
#define LDAP_SERVER_DOMAIN_SCOPE_OID_W         L"1.2.840.113556.1.4.1339"

//
// Server Search Options. Allows the client to pass in flags to control
// various search behaviours. Data as follows
//      SEQUENCE {
//          Flags   INTEGER
//      }
//

#define LDAP_SERVER_SEARCH_OPTIONS_OID          "1.2.840.113556.1.4.1340"
#define LDAP_SERVER_SEARCH_OPTIONS_OID_W       L"1.2.840.113556.1.4.1340"

//
// search option flags
//

#define SERVER_SEARCH_FLAG_DOMAIN_SCOPE         0x1 // no referrals generated
#define SERVER_SEARCH_FLAG_PHANTOM_ROOT         0x2 // search all NCs subordinate
                                                    // to search base

//
// This control is used to pass the sid of a security principle
// who's quota is being queried
//      SEQUENCE {
//          QuerySid OCTET STRING     // sid of security principle
//      }
//

#define LDAP_SERVER_QUOTA_CONTROL_OID           "1.2.840.113556.1.4.1852"
#define LDAP_SERVER_QUOTA_CONTROL_OID_W        L"1.2.840.113556.1.4.1852"

//
// Range retrieval no error
// No control data

#define LDAP_SERVER_RANGE_RETRIEVAL_NOERR_OID       "1.2.840.113556.1.4.1948"
#define LDAP_SERVER_RANGE_RETRIEVAL_NOERR_OID_W    L"1.2.840.113556.1.4.1948"


//
// DN input control: This control is used to pass a DN as an input
//                          for a LDAP operation
//      SEQUENCE {
//          InputDN OCTET STRING     //
//      }
//
#define LDAP_SERVER_DN_INPUT_OID       "1.2.840.113556.1.4.2026"
#define LDAP_SERVER_DN_INPUT_OID_W    L"1.2.840.113556.1.4.2026"

//
// Set Owner LDAP Control
// Request:
//      SEQUENCE {
//          SID OCTET STRING    // Unicode String SID of the Owner
//      }
//

#define LDAP_SERVER_SET_OWNER_OID      "1.2.840.113556.1.4.2255"
#define LDAP_SERVER_SET_OWNER_OID_W    L"1.2.840.113556.1.4.2255"


//
// Bypass Quota LDAP Control
//

#define LDAP_SERVER_BYPASS_QUOTA_OID      "1.2.840.113556.1.4.2256"
#define LDAP_SERVER_BYPASS_QUOTA_OID_W    L"1.2.840.113556.1.4.2256"


//
// Show Time To Live Control.  No Data.
//

#define LDAP_SERVER_LINK_TTL_OID                 "1.2.840.113556.1.4.2309"
#define LDAP_SERVER_LINK_TTL_OID_W               L"1.2.840.113556.1.4.2309"


//
// End of Server controls
//

//
//
// Operational Attributes
//

#define LDAP_OPATT_BECOME_DOM_MASTER            "becomeDomainMaster"
#define LDAP_OPATT_BECOME_DOM_MASTER_W          L"becomeDomainMaster"

#define LDAP_OPATT_BECOME_RID_MASTER            "becomeRidMaster"
#define LDAP_OPATT_BECOME_RID_MASTER_W          L"becomeRidMaster"

#define LDAP_OPATT_BECOME_SCHEMA_MASTER         "becomeSchemaMaster"
#define LDAP_OPATT_BECOME_SCHEMA_MASTER_W       L"becomeSchemaMaster"

#define LDAP_OPATT_RECALC_HIERARCHY             "recalcHierarchy"
#define LDAP_OPATT_RECALC_HIERARCHY_W           L"recalcHierarchy"

#define LDAP_OPATT_SCHEMA_UPDATE_NOW            "schemaUpdateNow"
#define LDAP_OPATT_SCHEMA_UPDATE_NOW_W          L"schemaUpdateNow"

#define LDAP_OPATT_BECOME_PDC                   "becomePdc"
#define LDAP_OPATT_BECOME_PDC_W                 L"becomePdc"

#define LDAP_OPATT_FIXUP_INHERITANCE            "fixupInheritance"
#define LDAP_OPATT_FIXUP_INHERITANCE_W          L"fixupInheritance"

#define LDAP_OPATT_INVALIDATE_RID_POOL          "invalidateRidPool"
#define LDAP_OPATT_INVALIDATE_RID_POOL_W        L"invalidateRidPool"

#define LDAP_OPATT_ABANDON_REPL                 "abandonReplication"
#define LDAP_OPATT_ABANDON_REPL_W               L"abandonReplication"

#define LDAP_OPATT_DO_GARBAGE_COLLECTION        "doGarbageCollection"
#define LDAP_OPATT_DO_GARBAGE_COLLECTION_W      L"doGarbageCollection"

//
//  Root DSE Attributes
//

#define LDAP_OPATT_SUBSCHEMA_SUBENTRY           "subschemaSubentry"
#define LDAP_OPATT_SUBSCHEMA_SUBENTRY_W         L"subschemaSubentry"

#define LDAP_OPATT_CURRENT_TIME                 "currentTime"
#define LDAP_OPATT_CURRENT_TIME_W               L"currentTime"

#define LDAP_OPATT_SERVER_NAME                  "serverName"
#define LDAP_OPATT_SERVER_NAME_W                L"serverName"

#define LDAP_OPATT_NAMING_CONTEXTS              "namingContexts"
#define LDAP_OPATT_NAMING_CONTEXTS_W            L"namingContexts"

#define LDAP_OPATT_DEFAULT_NAMING_CONTEXT       "defaultNamingContext"
#define LDAP_OPATT_DEFAULT_NAMING_CONTEXT_W     L"defaultNamingContext"

#define LDAP_OPATT_SUPPORTED_CONTROL            "supportedControl"
#define LDAP_OPATT_SUPPORTED_CONTROL_W          L"supportedControl"

#define LDAP_OPATT_HIGHEST_COMMITTED_USN        "highestCommitedUSN"
#define LDAP_OPATT_HIGHEST_COMMITTED_USN_W      L"highestCommitedUSN"

#define LDAP_OPATT_SUPPORTED_LDAP_VERSION       "supportedLDAPVersion"
#define LDAP_OPATT_SUPPORTED_LDAP_VERSION_W     L"supportedLDAPVersion"

#define LDAP_OPATT_SUPPORTED_LDAP_POLICIES      "supportedLDAPPolicies"
#define LDAP_OPATT_SUPPORTED_LDAP_POLICIES_W    L"supportedLDAPPolicies"

#define LDAP_OPATT_SCHEMA_NAMING_CONTEXT        "schemaNamingContext"
#define LDAP_OPATT_SCHEMA_NAMING_CONTEXT_W      L"schemaNamingContext"

#define LDAP_OPATT_CONFIG_NAMING_CONTEXT        "configurationNamingContext"
#define LDAP_OPATT_CONFIG_NAMING_CONTEXT_W      L"configurationNamingContext"

#define LDAP_OPATT_ROOT_DOMAIN_NAMING_CONTEXT   "rootDomainNamingContext"
#define LDAP_OPATT_ROOT_DOMAIN_NAMING_CONTEXT_W L"rootDomainNamingContext"

#define LDAP_OPATT_SUPPORTED_SASL_MECHANISM     "supportedSASLMechanisms"
#define LDAP_OPATT_SUPPORTED_SASL_MECHANISM_W   L"supportedSASLMechanisms"

#define LDAP_OPATT_DNS_HOST_NAME                "dnsHostName"
#define LDAP_OPATT_DNS_HOST_NAME_W              L"dnsHostName"

#define LDAP_OPATT_LDAP_SERVICE_NAME            "ldapServiceName"
#define LDAP_OPATT_LDAP_SERVICE_NAME_W          L"ldapServiceName"

#define LDAP_OPATT_DS_SERVICE_NAME              "dsServiceName"
#define LDAP_OPATT_DS_SERVICE_NAME_W            L"dsServiceName"

#define LDAP_OPATT_SUPPORTED_CAPABILITIES       "supportedCapabilities"
#define LDAP_OPATT_SUPPORTED_CAPABILITIES_W     L"supportedCapabilities"

//
// End of Operational attributes
//



//
//
// Server Capabilities
//

//
// NT5 Active Directory
//

#define LDAP_CAP_ACTIVE_DIRECTORY_OID          "1.2.840.113556.1.4.800"
#define LDAP_CAP_ACTIVE_DIRECTORY_OID_W        L"1.2.840.113556.1.4.800"

#define LDAP_CAP_ACTIVE_DIRECTORY_V51_OID      "1.2.840.113556.1.4.1670"
#define LDAP_CAP_ACTIVE_DIRECTORY_V51_OID_W    L"1.2.840.113556.1.4.1670"

#define LDAP_CAP_ACTIVE_DIRECTORY_LDAP_INTEG_OID   "1.2.840.113556.1.4.1791"
#define LDAP_CAP_ACTIVE_DIRECTORY_LDAP_INTEG_OID_W L"1.2.840.113556.1.4.1791"

#define LDAP_CAP_ACTIVE_DIRECTORY_ADAM_OID   "1.2.840.113556.1.4.1851"
#define LDAP_CAP_ACTIVE_DIRECTORY_ADAM_OID_W L"1.2.840.113556.1.4.1851"

#define LDAP_CAP_ACTIVE_DIRECTORY_PARTIAL_SECRETS_OID "1.2.840.113556.1.4.1920"
#define LDAP_CAP_ACTIVE_DIRECTORY_PARTIAL_SECRETS_OID_W L"1.2.840.113556.1.4.1920"

#define LDAP_CAP_ACTIVE_DIRECTORY_V60_OID    "1.2.840.113556.1.4.1935"
#define LDAP_CAP_ACTIVE_DIRECTORY_V60_OID_W L"1.2.840.113556.1.4.1935"

#define LDAP_CAP_ACTIVE_DIRECTORY_V61_OID    LDAP_CAP_ACTIVE_DIRECTORY_V60_OID
#define LDAP_CAP_ACTIVE_DIRECTORY_V61_OID_W  LDAP_CAP_ACTIVE_DIRECTORY_V60_OID_W 

#define LDAP_CAP_ACTIVE_DIRECTORY_V61_R2_OID    "1.2.840.113556.1.4.2080"
#define LDAP_CAP_ACTIVE_DIRECTORY_V61_R2_OID_W L"1.2.840.113556.1.4.2080"

#define LDAP_CAP_ACTIVE_DIRECTORY_W8_OID    "1.2.840.113556.1.4.2237"
#define LDAP_CAP_ACTIVE_DIRECTORY_W8_OID_W L"1.2.840.113556.1.4.2237"


//
//  End of capabilities
//


//
//
// Matching Rules
//

//
// BIT AND
//

#define LDAP_MATCHING_RULE_BIT_AND              "1.2.840.113556.1.4.803"
#define LDAP_MATCHING_RULE_BIT_AND_W            L"1.2.840.113556.1.4.803"

//
// BIT OR
//

#define LDAP_MATCHING_RULE_BIT_OR               "1.2.840.113556.1.4.804"
#define LDAP_MATCHING_RULE_BIT_OR_W             L"1.2.840.113556.1.4.804"

//
// TRANSITIVE EVALUATION
//
#define LDAP_MATCHING_RULE_TRANSITIVE_EVALUATION   "1.2.840.113556.1.4.1941"
#define LDAP_MATCHING_RULE_TRANSITIVE_EVALUATION_W L"1.2.840.113556.1.4.1941"

//
// COMPLEX DN-STRING OR DN-BINARY MATCH
//
#define LDAP_MATCHING_RULE_DN_BINARY_COMPLEX  "1.2.840.113556.1.4.2253"
#define LDAP_MATCHING_RULE_DN_BINARY_COMPLEX_W  L"1.2.840.113556.1.4.2253"

//
//
// Extended Requests
//

//
// Fast bind mode.
//

#define LDAP_SERVER_FAST_BIND_OID               "1.2.840.113556.1.4.1781"
#define LDAP_SERVER_FAST_BIND_OID_W             L"1.2.840.113556.1.4.1781"

//
// WhoAmI extended request
//

#define LDAP_SERVER_WHO_AM_I_OID                "1.3.6.1.4.1.4203.1.11.3"
#define LDAP_SERVER_WHO_AM_I_OID_W              L"1.3.6.1.4.1.4203.1.11.3"


// 
// Batched operation request
#define LDAP_SERVER_BATCH_REQUEST_OID                  "1.2.840.113556.1.4.2212"
#define LDAP_SERVER_BATCH_REQUEST_OID_W                L"1.2.840.113556.1.4.2212"


//
// DirSync flags
//

// Without this flag, the caller must have the replicate changes right. With this flag,
// the caller needs no rights, but is only allowed to see objects and attributes
// which are accessible to them.
#define LDAP_DIRSYNC_OBJECT_SECURITY             (0x1)

// Return parents before children, when parents would otherwise appear later
// in the replication stream.
#define LDAP_DIRSYNC_ANCESTORS_FIRST_ORDER    (0x0800)

// Do not return secret data. Always on by default.
#define LDAP_DIRSYNC_PUBLIC_DATA_ONLY         (0x2000)

// Without this flag, all the values (up to a limit) in a multi-valued attribute are
// returned when any value changes. With this flag, only the changed values are returned.
#define LDAP_DIRSYNC_INCREMENTAL_VALUES   (0x80000000)

// Do not return filtered attributes.  On by default, unless the caller asks
// for these attributes specifically.
#define LDAP_DIRSYNC_ROPAS_DATA_ONLY         (0x40000000)

//
// Policy hint flags
//

// When defined, this flag tells the server to apply full pwd policy 
// on password-set operations.
#define LDAP_POLICYHINT_APPLY_FULLPWDPOLICY             (0x1)



#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif  // NT_LDAP_H

