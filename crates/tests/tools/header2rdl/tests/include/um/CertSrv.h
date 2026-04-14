//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992 - 1999
//
//  File:       CertSrv.h
//  Contents:   Main Certificate Server header
//              Also includes .h files for the COM interfaces
//
//----------------------------------------------------------------------------

#if !defined( _CERTSRV_H_ )
#define _CERTSRV_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <certadm.h>
#include <certbcli.h>
#include <certcli.h>
#include <certenc.h>
#include <certexit.h>
#include <certif.h>
#include <certpol.h>
#include <certmod.h>
#include <certview.h>

#ifndef DBG_CERTSRV
# if defined(_DEBUG)
#  define DBG_CERTSRV     1
# elif defined(DBG)
#  define DBG_CERTSRV     DBG
# else
#  define DBG_CERTSRV     0
# endif
#endif

#define wszSERVICE_NAME		TEXT("CertSvc")

#define wszREGKEYNOSYSTEMCERTSVCPATH \
				TEXT("CurrentControlSet\\Services\\") \
				wszSERVICE_NAME

#define wszREGKEYCERTSVCPATH	TEXT("SYSTEM\\") wszREGKEYNOSYSTEMCERTSVCPATH
#define wszREGKEYBASE		wszREGKEYCERTSVCPATH	// obsolete definition

//======================================================================
// Full path to "CertSvc\Configuration\":
#define wszREGKEYCONFIGPATH	wszREGKEYCERTSVCPATH TEXT("\\") wszREGKEYCONFIG
#define wszREGKEYCONFIGPATH_BS	wszREGKEYCONFIGPATH TEXT("\\")
#define wszREGKEYCONFIGCANAME	wszREGKEYCONFIGPATH_BS	// obsolete definition

//======================================================================
// Full path to "CertSvc\Configuration\RestoreInProgress":
#define wszREGKEYCONFIGRESTORE wszREGKEYCONFIGPATH_BS wszREGKEYRESTOREINPROGRESS

//======================================================================
// Key Under "CertSvc":
#define wszREGKEYCONFIG		TEXT("Configuration")

//======================================================================
// Values Under "CertSvc\Configuration":
#define wszREGACTIVE		      TEXT("Active")
#define wszREGDIRECTORY		      TEXT("ConfigurationDirectory")
#define wszREGDBDIRECTORY             TEXT("DBDirectory")
#define wszREGDBLOGDIRECTORY          TEXT("DBLogDirectory")
#define wszREGDBSYSDIRECTORY          TEXT("DBSystemDirectory")
#define wszREGDBTEMPDIRECTORY         TEXT("DBTempDirectory")
#define wszREGDBSESSIONCOUNT	      TEXT("DBSessionCount")
#define wszREGDBMAXREADSESSIONCOUNT   TEXT("DBMaxReadSessionCount")
#define wszREGDBFLAGS		      TEXT("DBFlags")
#define wszREGDBLASTFULLBACKUP	      TEXT("DBLastFullBackup")
#define wszREGDBLASTINCREMENTALBACKUP TEXT("DBLastIncrementalBackup")
#define wszREGDBLASTRECOVERY	      TEXT("DBLastRecovery")
#define wszREGWEBCLIENTCAMACHINE      TEXT("WebClientCAMachine")
#define wszREGVERSION		      TEXT("Version")
#define wszREGWEBCLIENTCANAME         TEXT("WebClientCAName")
#define wszREGWEBCLIENTCATYPE         TEXT("WebClientCAType")
#define wszREGLDAPFLAGS               TEXT("LDAPFlags")
#define wszREGCERTSRVDEBUG	      TEXT("Debug")

// Default value for wszREGDBSESSIONCOUNT
#define DBSESSIONCOUNTDEFAULT	     100

//==================================
// Values for wszREGDBFLAGS

#define DBFLAGS_READONLY		0x00000001	// ignored in registry
#define DBFLAGS_CREATEIFNEEDED		0x00000002
#define DBFLAGS_CIRCULARLOGGING		0x00000004
#define DBFLAGS_LAZYFLUSH		0x00000008
#define DBFLAGS_MAXCACHESIZEX100	0x00000010
#define DBFLAGS_CHECKPOINTDEPTH60MB	0x00000020
#define DBFLAGS_LOGBUFFERSLARGE		0x00000040
#define DBFLAGS_LOGBUFFERSHUGE		0x00000080
#define DBFLAGS_LOGFILESIZE16MB		0x00000100
#define DBFLAGS_MULTITHREADTRANSACTIONS	0x00000200
#define DBFLAGS_DISABLESNAPSHOTBACKUP	0x00000400	// ignored in registry
#define DBFLAGS_ENABLEVOLATILEREQUESTS  0X00000800 // enables the use of CCertDBMem

#define DBFLAGS_DEFAULT		(DBFLAGS_MAXCACHESIZEX100 | \
				 DBFLAGS_CHECKPOINTDEPTH60MB | \
				 DBFLAGS_LOGBUFFERSHUGE)


//==================================
// Values for wszREGLDAPSSLFLAGS
// Default is zero (same as a missing registry value):
//    0: Don't use SSL, but sign all LDAP traffic.
//    3: Use SSL but don't sign LDAP traffic.
#define LDAPF_SSLENABLE		0x00000001	// use SSL for LDAP traffic
#define LDAPF_SIGNDISABLE	0x00000002	// disable signing LDAP traffic

// Default value for wszREGMAXINCOMINGMESSAGESIZE
#define MAXINCOMINGMESSAGESIZEDEFAULT		(64 * 1024)

// Default value for wszREGMAXINCOMINGALLOCSIZE
#define MAXINCOMINGALLOCSIZEDEFAULT		(64 * 1024)

// Value for wszREGVERSION:

// win2k version
#define CSVER_MAJOR_WIN2K            1 // actually this wasn't define in win2k
#define CSVER_MINOR_WIN2K            1

// whistler version
#define CSVER_MAJOR_WHISTLER         2
#define CSVER_MINOR_WHISTLER_BETA2   1
#define CSVER_MINOR_WHISTLER_BETA3   2

// longhorn version
#define CSVER_MAJOR_LONGHORN         3
#define CSVER_MINOR_LONGHORN_BETA1   1

// win7 version
#define CSVER_MAJOR_WIN7 4
#define CSVER_MINOR_WIN7 1

// win8 version
#define CSVER_MAJOR_WIN8 5
#define CSVER_MINOR_WIN8 1

// winblue version
#define CSVER_MAJOR_WINBLUE 6
#define CSVER_MINOR_WINBLUE 1

// threshold version
#define CSVER_MAJOR_THRESHOLD 7
#define CSVER_MINOR_THRESHOLD 1

// current version
#define CSVER_MAJOR		     CSVER_MAJOR_THRESHOLD	// high 16 bits
#define CSVER_MINOR		     CSVER_MINOR_THRESHOLD	// low 16 bits

// version manipulation
#define CSVER_EXTRACT_MAJOR(version) ((version)>>16)
#define CSVER_EXTRACT_MINOR(version) ((version)&0xffff)
#define CSVER_BUILD_VERSION(major, minor) (((major)<<16)|(minor))

// propertyId manipulation to pass partition index and key index
#define CRL_BUILD_PROPID(partIndex, keyIndex) ((partIndex << 16) | keyIndex)
#define CRL_EXTRACT_KEY_INDEX(propId) ((propId) & 0xFFFF)
#define CRL_EXTRACT_PARTITION_INDEX(propId) ((propId) >> 16)

// Keys Under "CertSvc\Configuration":
#define wszREGKEYRESTOREINPROGRESS   TEXT("RestoreInProgress")
#define wszREGKEYDBPARAMETERS	     TEXT("DBParameters")

//======================================================================
// Values Under "CertSvc\Configuration\<CAName>":
#define wszREGCADESCRIPTION          TEXT("CADescription")
#define wszREGCACERTHASH	     TEXT("CACertHash")
#define wszREGCASERIALNUMBER	     TEXT("CACertSerialNumber")
#define wszREGCAXCHGCERTHASH	     TEXT("CAXchgCertHash")
#define wszREGKRACERTHASH	     TEXT("KRACertHash")
#define wszREGKRACERTCOUNT	     TEXT("KRACertCount")
#define wszREGKRAFLAGS		     TEXT("KRAFlags")
#define wszREGCATYPE		     TEXT("CAType")
#define wszREGCERTENROLLCOMPATIBLE   TEXT("CertEnrollCompatible")
#define wszREGENFORCEX500NAMELENGTHS TEXT("EnforceX500NameLengths")
#define wszREGCOMMONNAME	     TEXT("CommonName")
#define wszREGCLOCKSKEWMINUTES	     TEXT("ClockSkewMinutes")

#define wszREGCRLNEXTPUBLISH         TEXT("CRLNextPublish")
#define wszREGCRLPERIODSTRING	     TEXT("CRLPeriod")
#define wszREGCRLPERIODCOUNT	     TEXT("CRLPeriodUnits")
#define wszREGCRLOVERLAPPERIODSTRING TEXT("CRLOverlapPeriod")
#define wszREGCRLOVERLAPPERIODCOUNT  TEXT("CRLOverlapUnits")

#define wszREGCRLDELTANEXTPUBLISH    TEXT("CRLDeltaNextPublish")
#define wszREGCRLDELTAPERIODSTRING   TEXT("CRLDeltaPeriod")
#define wszREGCRLDELTAPERIODCOUNT    TEXT("CRLDeltaPeriodUnits")
#define wszREGCRLDELTAOVERLAPPERIODSTRING TEXT("CRLDeltaOverlapPeriod")
#define wszREGCRLDELTAOVERLAPPERIODCOUNT  TEXT("CRLDeltaOverlapUnits")

#define wszREGCRLPUBLICATIONURLS     TEXT("CRLPublicationURLs")
#define wszREGCACERTPUBLICATIONURLS  TEXT("CACertPublicationURLs")

#define wszREGCRLMAXPARTITIONS       TEXT("CRLMaxPartitions")
#define wszREGCRLSUSPENDEDPARTITIONS TEXT("CRLSuspendedPartitions")
#define wszREGCRLCURRENTPARTITION    TEXT("CRLCurrentPartition")

#define wszREGCAXCHGVALIDITYPERIODSTRING  TEXT("CAXchgValidityPeriod")
#define wszREGCAXCHGVALIDITYPERIODCOUNT   TEXT("CAXchgValidityPeriodUnits")
#define wszREGCAXCHGOVERLAPPERIODSTRING   TEXT("CAXchgOverlapPeriod")
#define wszREGCAXCHGOVERLAPPERIODCOUNT    TEXT("CAXchgOverlapPeriodUnits")

#define wszREGCRLPATH_OLD            TEXT("CRLPath")
#define wszREGCRLEDITFLAGS	     TEXT("CRLEditFlags")
#define wszREGCRLFLAGS		     TEXT("CRLFlags")
#define wszREGCRLATTEMPTREPUBLISH    TEXT("CRLAttemptRepublish")
#define wszREGENABLED		     TEXT("Enabled")
#define wszREGFORCETELETEX           TEXT("ForceTeletex")
#define wszREGLOGLEVEL		     TEXT("LogLevel")
#define wszREGHIGHSERIAL	     TEXT("HighSerial")
#define wszREGPOLICYFLAGS	     TEXT("PolicyFlags")
#define wszREGNAMESEPARATOR          TEXT("SubjectNameSeparator")
#define wszREGSUBJECTTEMPLATE	     TEXT("SubjectTemplate")
#define wszREGCAUSEDS		     TEXT("UseDS")
#define wszREGVALIDITYPERIODSTRING   TEXT("ValidityPeriod")
#define wszREGVALIDITYPERIODCOUNT    TEXT("ValidityPeriodUnits")
#define wszREGPARENTCAMACHINE        TEXT("ParentCAMachine")
#define wszREGPARENTCANAME           TEXT("ParentCAName")
#define wszREGREQUESTFILENAME        TEXT("RequestFileName")
#define wszREGREQUESTID              TEXT("RequestId")
#define wszREGREQUESTKEYCONTAINER    TEXT("RequestKeyContainer")
#define wszREGREQUESTKEYINDEX        TEXT("RequestKeyIndex")
#define wszREGCASERVERNAME           TEXT("CAServerName")
#define wszREGCACERTFILENAME         TEXT("CACertFileName")
#define wszREGCASECURITY             TEXT("Security")
#define wszREGAUDITFILTER            TEXT("AuditFilter")
#define wszREGOFFICERRIGHTS          TEXT("OfficerRights")
#define wszENROLLMENTAGENTRIGHTS     TEXT("EnrollmentAgentRights")
#define wszREGMAXINCOMINGMESSAGESIZE TEXT("MaxIncomingMessageSize")
#define wszREGMAXINCOMINGALLOCSIZE   TEXT("MaxIncomingAllocSize")
#define wszREGROLESEPARATIONENABLED  TEXT("RoleSeparationEnabled")
#define wszREGALTERNATEPUBLISHDOMAINS TEXT("AlternatePublishDomains")

#define wszREGSETUPSTATUS            TEXT("SetupStatus")
#define wszREGINTERFACEFLAGS         TEXT("InterfaceFlags")    
#define wszREGDSCONFIGDN	     TEXT("DSConfigDN")    
#define wszREGDSDOMAINDN	     TEXT("DSDomainDN")    
#define wszREGVIEWAGEMINUTES	     TEXT("ViewAgeMinutes")
#define wszREGVIEWIDLEMINUTES	     TEXT("ViewIdleMinutes")

// This contains a list of UNC or local directories, each containing a selection of EK public keys
// that will be explicitly trusted for TPM attestation
#define wszREGEKPUBLISTDIRECTORIES  TEXT("EndorsementKeyListDirectories")

// This configures the CA to accept Certificate Transparency requests
#define wszCERTIFICATETRANSPARENCYFLAGS  TEXT("CertificateTransparencyFlags")

// This customizes the maximum size of the CT SCTList accepted by the CA
#define wszREGMAXSCTLISTSIZE  TEXT("MaxSCTListSize")

// This sets an OID to replace szOID_CT_CERT_SCTLIST in the issued certificate
#define wszREGCERTIFICATETRANSPARENCYINFOOID TEXT("CTInformationExtensionOid")

// This extends optional CA behavior with simple processing flags
#define wszREGPROCESSINGFLAGS TEXT("ProcessingFlags")

// This REG_DWORD value determines if the CA accepts OCSP Signing certificate 
// enrollment and renewal requests for replaced CA keys. 
// * If the UseDefinedCACertInRequest does not exist, then the default behavior
//   is used (do not accept certificate requests on replaced CA keys)
// * Setting the value to 0 is the default setting
// * Setting the value to a non-zero value enables the CA for certificate 
//   requests for replaced CA keys
#define wszREGUSEDEFINEDCACERTINREQ  TEXT("UseDefinedCACertInRequest")

// This REG_MULTISZ value allows administrators to customize which OIDs can be
// present in OCSP requesting certificates in order to enroll or renew against
// replaced CA keys. 
// * If the EnabledEKUForDefinedCACert multi-string entry does not exist, 
//   then the default behavior is used (enforce OCSP EKU in resultant cert)
// * If a value is present, a basic ASN1 compliance check must be performed on
//   CA start-up. If compliance of OID fails validation, the CA must log an 
//   error and default to enforce OCSP EKU in resultant cert.
#define wszREGENABLEDEKUFORDEFINEDCACERT TEXT("EnabledEKUForDefinedCACert")

// This REG_MULTISZ value allows administrators to customize which OIDs can be
// present in certificates that are being revoked and need to be present in CRL
// regardless of the certificates validity period.
// * If the EKUOIDsForPublishExpiredCertInCRL multi-string entry does not exist, 
//   then the default behavior is used (default OIDS defined below).
// * If a value is present, a basic ASN1 compliance check must be performed on
//   CA start-up. If compliance of OID fails validation, the CA must log an 
//   error and default OIDs (defined below) would be used.
#define wszREGEKUOIDSFORPUBLISHEXPIREDCERTINCRL TEXT("EKUOIDsForPublishExpiredCertInCRL")


#define wszzDEFAULTEKUOIDSFORPUBLISHEXPIREDCERTINCRL \
    TEXT(szOID_PKIX_KP_CODE_SIGNING) L"\0" \
    TEXT(szOID_KP_KERNEL_MODE_CODE_SIGNING) L"\0"

#define wszCRTFILENAMEEXT	     TEXT(".crt")
#define wszPFXFILENAMEEXT	     TEXT(".p12")
#define wszDATFILENAMEEXT	     TEXT(".dat")
#define wszLOGFILENAMEEXT	     TEXT(".log")
#define wszDBFILENAMEEXT	     TEXT(".edb")
#define szDBBASENAMEPARM	     "edb"
#define wszDBBASENAMEPARM	     TEXT(szDBBASENAMEPARM)
#define wszLOGPATH		     TEXT("CertLog")
#define wszDBBACKUPSUBDIR	     TEXT("DataBase")
#define wszDBBACKUPCERTBACKDAT	     TEXT("certbkxp.dat")

#ifndef __ENUM_CATYPES__
#define __ENUM_CATYPES__

//==================================
// Values for wszREGCATYPE:
typedef enum {
    ENUM_ENTERPRISE_ROOTCA = 0,
    ENUM_ENTERPRISE_SUBCA = 1,
    //ENUM_UNUSED2 = 2,
    ENUM_STANDALONE_ROOTCA = 3,
    ENUM_STANDALONE_SUBCA = 4,
    ENUM_UNKNOWN_CA = 5,
} ENUM_CATYPES;

typedef struct _CAINFO
{
    DWORD   cbSize;
    ENUM_CATYPES CAType;
    DWORD   cCASignatureCerts;
    DWORD   cCAExchangeCerts;
    DWORD   cExitModules;
    LONG    lPropIdMax;
    LONG    lRoleSeparationEnabled;
    DWORD   cKRACertUsedCount;
    DWORD   cKRACertCount;
    DWORD   fAdvancedServer;
} CAINFO;

#endif // __ENUM_CATYPES__

// Default value for wszREGCLOCKSKEWMINUTES
#define CCLOCKSKEWMINUTESDEFAULT	      10


// Default value for wszREGVIEWAGEMINUTES, wszREGVIEWIDLEMINUTES
#define CVIEWAGEMINUTESDEFAULT			16
#define CVIEWIDLEMINUTESDEFAULT			(CVIEWAGEMINUTESDEFAULT / 2)

// Default validity period for ROOT CA certs:
#define dwVALIDITYPERIODCOUNTDEFAULT_ROOT	5

// Default validity periods for certs issued by a CA:
#define dwVALIDITYPERIODCOUNTDEFAULT_ENTERPRISE	2
#define dwVALIDITYPERIODCOUNTDEFAULT_STANDALONE	1
#define dwVALIDITYPERIODENUMDEFAULT	      ENUM_PERIOD_YEARS
#define wszVALIDITYPERIODSTRINGDEFAULT	      wszPERIODYEARS

#define dwCAXCHGVALIDITYPERIODCOUNTDEFAULT    1
#define dwCAXCHGVALIDITYPERIODENUMDEFAULT     ENUM_PERIOD_WEEKS
#define wszCAXCHGVALIDITYPERIODSTRINGDEFAULT  wszPERIODWEEKS

#define dwCAXCHGOVERLAPPERIODCOUNTDEFAULT     1
#define dwCAXCHGOVERLAPPERIODENUMDEFAULT      ENUM_PERIOD_DAYS
#define wszCAXCHGOVERLAPPERIODSTRINGDEFAULT   wszPERIODDAYS

#define dwCRLPERIODCOUNTDEFAULT		      1
#define wszCRLPERIODSTRINGDEFAULT	      wszPERIODWEEKS

#define dwCRLOVERLAPPERIODCOUNTDEFAULT	      0		// 0 --> disabled
#define wszCRLOVERLAPPERIODSTRINGDEFAULT      wszPERIODHOURS

#define dwCRLDELTAPERIODCOUNTDEFAULT          1
#define wszCRLDELTAPERIODSTRINGDEFAULT        wszPERIODDAYS

#define dwCRLDELTAOVERLAPPERIODCOUNTDEFAULT   0		// 0 --> disabled
#define wszCRLDELTAOVERLAPPERIODSTRINGDEFAULT wszPERIODMINUTES


//==================================
// Values for wszREGLOGLEVEL:
#define CERTLOG_MINIMAL		(DWORD) 0
#define CERTLOG_TERSE		(DWORD) 1
#define CERTLOG_ERROR		(DWORD) 2
#define CERTLOG_WARNING		(DWORD) 3
#define CERTLOG_VERBOSE		(DWORD) 4
#define CERTLOG_EXHAUSTIVE	(DWORD) 5


//==================================
// Values for wszREGSETUPSTATUS:
#define SETUP_SERVER_FLAG		     0x00000001	// server installed
#define SETUP_CLIENT_FLAG		     0x00000002	// client installed
#define SETUP_SUSPEND_FLAG		     0x00000004	// incomplete install
#define SETUP_REQUEST_FLAG		     0x00000008	// new cert requested
#define SETUP_ONLINE_FLAG		     0x00000010	// requested online
#define SETUP_DENIED_FLAG		     0x00000020	// request denied
#define SETUP_CREATEDB_FLAG		     0x00000040	// create new DB
#define SETUP_ATTEMPT_VROOT_CREATE	     0x00000080	// try to create vroots
#define SETUP_FORCECRL_FLAG		     0x00000100	// force new CRL(s)

// add server type to CA DS object "flags" attr:
#define SETUP_UPDATE_CAOBJECT_SVRTYPE	     0x00000200

#define SETUP_SERVER_UPGRADED_FLAG	     0x00000400	// server was upgraded

// still need to upgrade security:
#define SETUP_W2K_SECURITY_NOT_UPGRADED_FLAG 0x00000800

// permissons changed while CA was down, certsrv will need to update DS &
// service when it restarts:
#define SETUP_SECURITY_CHANGED		     0x00001000

// win2k3 SP1 - global DCOM security has been fixed:
#define SETUP_DCOM_SECURITY_UPDATED_FLAG     0x00002000

// Indicates that the server is up to date:
#define SETUP_SERVER_IS_UP_TO_DATE_FLAG      0x00004000


//==================================
// Values for wszREGCRLFLAGS:
#define CRLF_DELTA_USE_OLDEST_UNEXPIRED_BASE	0x00000001 // use oldest base:
// else use newest base CRL that satisfies base CRL propagation delay

#define CRLF_DELETE_EXPIRED_CRLS                0x00000002
#define CRLF_CRLNUMBER_CRITICAL                 0x00000004
#define CRLF_REVCHECK_IGNORE_OFFLINE            0x00000008
#define CRLF_IGNORE_INVALID_POLICIES            0x00000010
#define CRLF_REBUILD_MODIFIED_SUBJECT_ONLY      0x00000020
#define CRLF_SAVE_FAILED_CERTS                  0x00000040
#define CRLF_IGNORE_UNKNOWN_CMC_ATTRIBUTES      0x00000080
#define CRLF_IGNORE_CROSS_CERT_TRUST_ERROR      0x00000100
#define CRLF_PUBLISH_EXPIRED_CERT_CRLS          0x00000200
#define CRLF_ENFORCE_ENROLLMENT_AGENT           0x00000400
#define CRLF_DISABLE_RDN_REORDER                0x00000800
#define CRLF_DISABLE_ROOT_CROSS_CERTS           0x00001000
#define CRLF_LOG_FULL_RESPONSE                  0x00002000 // hex dump response to console
#define CRLF_USE_XCHG_CERT_TEMPLATE             0x00004000 // enforce xchg template access
#define CRLF_USE_CROSS_CERT_TEMPLATE            0x00008000 // enforce cross template access
#define CRLF_ALLOW_REQUEST_ATTRIBUTE_SUBJECT    0x00010000
#define CRLF_REVCHECK_IGNORE_NOREVCHECK         0x00020000
#define CRLF_PRESERVE_EXPIRED_CA_CERTS          0x00040000
#define CRLF_PRESERVE_REVOKED_CA_CERTS          0x00080000
#define CRLF_DISABLE_CHAIN_VERIFICATION         0x00100000
#define CRLF_BUILD_ROOTCA_CRLENTRIES_BASEDONKEY 0x00200000
// Flag to enable CRL partition feature
#define CRLF_ENABLE_CRL_PARTITION               0x00400000
// Flag to make the partition zero CRLs exclusive.
// When the flag is set, the CRLs of partition zero will exclusively contain
// entries related to certificates assigned specifically to partition zero. 
#define CRLF_PARTITION_ZERO_EXCLUSIVE           0x00800000
#define CRLF_CONTAINS_ONLY_CACERTS              0x01000000
#define CRLF_CONTAINS_ONLY_USERCERTS            0x02000000

//==================================
// Values for wszREGKRAFLAGS:
#define KRAF_ENABLEFOREIGN	0x00000001 // allow foreign cert, key archival
#define KRAF_SAVEBADREQUESTKEY	0x00000002 // save failed request w/archived key
#define KRAF_ENABLEARCHIVEALL	0x00000004
#define KRAF_DISABLEUSEDEFAULTPROVIDER    0x00000008

//==================================
// Values for wszREGINTERFACEFLAGS:
#define IF_LOCKICERTREQUEST             0x00000001
#define IF_NOREMOTEICERTREQUEST         0x00000002
#define IF_NOLOCALICERTREQUEST          0x00000004
#define IF_NORPCICERTREQUEST            0x00000008
#define IF_NOREMOTEICERTADMIN           0x00000010
#define IF_NOLOCALICERTADMIN            0x00000020
#define IF_NOREMOTEICERTADMINBACKUP     0x00000040
#define IF_NOLOCALICERTADMINBACKUP      0x00000080
#define IF_NOSNAPSHOTBACKUP             0x00000100
#define IF_ENFORCEENCRYPTICERTREQUEST   0x00000200
#define IF_ENFORCEENCRYPTICERTADMIN     0x00000400
#define IF_ENABLEEXITKEYRETRIEVAL       0x00000800
#define IF_ENABLEADMINASAUDITOR         0x00001000
#define IF_ENABLEPRESIGNSUPPORT         0x00002000

#define IF_DEFAULT                      (IF_NOREMOTEICERTADMINBACKUP | \
                                         IF_LOCKICERTREQUEST | \
                                         IF_ENFORCEENCRYPTICERTREQUEST | \
                                         IF_ENFORCEENCRYPTICERTADMIN)


// =================================
// Values for wszREGPROCESSINGFLAGS:
#define PROCFLG_NONE                0x00
#define PROCFLG_ENFORCEGOODKEYS     0x01

//==================================
// Values for numeric prefixes for
// wszREGCRLPUBLICATIONURLS and wszREGCACERTPUBLICATIONURLS:
//
// URL publication template Flags values, encoded as a decimal prefix for URL
// publication templates in the registry:
//   "1:c:\winnt\System32\CertSrv\CertEnroll\MyCA.crl"
//   "2:http:\//MyServer.MyDomain.com/CertEnroll\MyCA.crl"

#define CSURL_SERVERPUBLISH	 0x00000001
#define CSURL_ADDTOCERTCDP	 0x00000002
#define CSURL_ADDTOFRESHESTCRL	 0x00000004
#define CSURL_ADDTOCRLCDP	 0x00000008
#define CSURL_PUBLISHRETRY	 0x00000010
#define CSURL_ADDTOCERTOCSP	 0x00000020
#define CSURL_SERVERPUBLISHDELTA 0x00000040
#define CSURL_ADDTOIDP		 0x00000080  
//======================================================================
// Keys Under "CertSvc\Configuration\<CAName>":
#define wszREGKEYCSP			TEXT("CSP")
#define wszREGKEYENCRYPTIONCSP		TEXT("EncryptionCSP")
#define wszREGKEYEXITMODULES		TEXT("ExitModules")
#define wszREGKEYPOLICYMODULES	        TEXT("PolicyModules")
#define wszSECUREDATTRIBUTES		TEXT("SignedAttributes")

#define wszzDEFAULTSIGNEDATTRIBUTES     TEXT("RequesterName\0")

//======================================================================
// Values Under "CertSvc\Configuration\RestoreInProgress":
#define wszREGBACKUPLOGDIRECTORY	TEXT("BackupLogDirectory")
#define wszREGCHECKPOINTFILE		TEXT("CheckPointFile")
#define wszREGHIGHLOGNUMBER		TEXT("HighLogNumber")
#define wszREGLOWLOGNUMBER		TEXT("LowLogNumber")
#define wszREGLOGPATH			TEXT("LogPath")
#define wszREGRESTOREMAPCOUNT		TEXT("RestoreMapCount")
#define wszREGRESTOREMAP		TEXT("RestoreMap")
#define wszREGDATABASERECOVERED		TEXT("DatabaseRecovered")
#define wszREGRESTORESTATUS		TEXT("RestoreStatus")

// values under \Configuration\PolicyModules in nt5 beta 2
#define wszREGB2ICERTMANAGEMODULE   TEXT("ICertManageModule")
// values under \Configuration in nt4 sp4
#define wszREGSP4DEFAULTCONFIGURATION  TEXT("DefaultConfiguration")
// values under ca in nt4 sp4
#define wszREGSP4KEYSETNAME            TEXT("KeySetName")
#define wszREGSP4SUBJECTNAMESEPARATOR  TEXT("SubjectNameSeparator")
#define wszREGSP4NAMES                 TEXT("Names")
#define wszREGSP4QUERIES               TEXT("Queries")
// both nt4 sp4 and nt5 beta 2
#define wszREGNETSCAPECERTTYPE         TEXT("NetscapeCertType")
#define wszNETSCAPEREVOCATIONTYPE      TEXT("Netscape")


//======================================================================
// Values Under "CertSvc\Configuration\<CAName>\CSP":
// and "CertSvc\Configuration\<CAName>\EncryptionCSP":
#define wszREGPROVIDERTYPE     TEXT("ProviderType")
#define wszREGPROVIDER         TEXT("Provider")
#define wszHASHALGORITHM       TEXT("HashAlgorithm")
#define wszENCRYPTIONALGORITHM TEXT("EncryptionAlgorithm")
#define wszMACHINEKEYSET       TEXT("MachineKeyset")
#define wszREGKEYSIZE	       TEXT("KeySize")
#define wszREGSYMMETRICKEYSIZE TEXT("SymmetricKeySize")
#define wszCNGPUBLICKEYALGORITHM TEXT("CNGPublicKeyAlgorithm")
#define wszCNGHASHALGORITHM	TEXT("CNGHashAlgorithm")
#define wszCNGENCRYPTIONALGORITHM TEXT("CNGEncryptionAlgorithm")
#define wszREGALTERNATESIGNATUREALGORITHM TEXT("AlternateSignatureAlgorithm")


//======================================================================
// Value strings for "CertSvc\Configuration\<CAName>\SubjectNameSeparator":
#define szNAMESEPARATORDEFAULT   "\n"
#define wszNAMESEPARATORDEFAULT   TEXT(szNAMESEPARATORDEFAULT)


//======================================================================
// Value strings for "CertSvc\Configuration\<CAName>\ValidityPeriod", etc.:
#define wszPERIODYEARS		TEXT("Years")
#define wszPERIODMONTHS		TEXT("Months")
#define wszPERIODWEEKS		TEXT("Weeks")
#define wszPERIODDAYS		TEXT("Days")
#define wszPERIODHOURS		TEXT("Hours")
#define wszPERIODMINUTES	TEXT("Minutes")
#define wszPERIODSECONDS	TEXT("Seconds")

//======================================================================
// Values Under "CertSvc\Configuration\<CAName>\PolicyModules\<ProgId>":
#define wszREGISSUERCERTURLFLAGS    TEXT("IssuerCertURLFlags")
#define wszREGEDITFLAGS		    TEXT("EditFlags")
#define wszREGUPNMAP		    TEXT("UPNMap")
#define wszREGSUBJECTALTNAME	    TEXT("SubjectAltName")
#define wszREGSUBJECTALTNAME2	    TEXT("SubjectAltName2")
#define wszREGREQUESTDISPOSITION    TEXT("RequestDisposition")
#define wszREGCAPATHLENGTH	    TEXT("CAPathLength")
#define wszREGREVOCATIONTYPE	    TEXT("RevocationType")

#define wszREGLDAPREVOCATIONCRLURL_OLD	TEXT("LDAPRevocationCRLURL")
#define wszREGREVOCATIONCRLURL_OLD	TEXT("RevocationCRLURL")
#define wszREGFTPREVOCATIONCRLURL_OLD	TEXT("FTPRevocationCRLURL")
#define wszREGFILEREVOCATIONCRLURL_OLD	TEXT("FileRevocationCRLURL")

#define wszREGREVOCATIONURL		TEXT("RevocationURL")

#define wszREGLDAPISSUERCERTURL_OLD	TEXT("LDAPIssuerCertURL")
#define wszREGISSUERCERTURL_OLD		TEXT("IssuerCertURL")
#define wszREGFTPISSUERCERTURL_OLD	TEXT("FTPIssuerCertURL")
#define wszREGFILEISSUERCERTURL_OLD	TEXT("FileIssuerCertURL")

#define wszREGENABLEREQUESTEXTENSIONLIST  TEXT("EnableRequestExtensionList")
#define wszREGENABLEENROLLEEREQUESTEXTENSIONLIST  TEXT("EnableEnrolleeRequestExtensionList")
#define wszREGDISABLEEXTENSIONLIST  TEXT("DisableExtensionList")

// This REG_MULTISZ value allows administrators to control
// which certificate requests get written to the database.
// the EnableVolatileRequests registry value must be enabled for this to work
// * If a value is present, a basic ASN1 compliance check must be performed on
//   policy module start-up. If the compliance of OIDs fail validation, the policy module must fail to initialize.
#define wszREGEKUOIDSFORVOLATILEREQUESTS TEXT("EKUOIDsforVolatileRequests")

// Following values would be used to configure the ldap session during setup of connection to DC from
// policy module
// CertSvc\Configuration\<CAName>\PolicyModules\<ProgId>\LdapSessionOptions\
//                                        OptionId1\
//                                                  LDAPSessionOptionValue 
//                                        OptionId2\
//                                                  LDAPSessionOptionValue 
//                                        OptionId3\
//                                                  LDAPSessionOptionValue 
//                                        ...
// OptionId* should be the decimal equivalent of ldap session option identifier
// (see ldap_set_option for the id and allowed values). 
// LDAPSessionOptionValue should be REG_DWORD (meaning only options whose value
// is of integer type can be set by configuring this registry key
// 
#define wszREGLDAPSESSIONOPTIONS    TEXT("LDAPSessionOptions")
#define wszLDAPSESSIONOPTIONVALUE   TEXT("LDAPSessionOptionValue")

#define wszREGDEFAULTSMIME		TEXT("DefaultSMIME")

// wszREGCAPATHLENGTH Values:
#define CAPATHLENGTH_INFINITE		0xffffffff

// wszREGREQUESTDISPOSITION Values:
#define REQDISP_PENDING			0x00000000
#define REQDISP_ISSUE			0x00000001
#define REQDISP_DENY			0x00000002
#define REQDISP_USEREQUESTATTRIBUTE	0x00000003
#define REQDISP_MASK			0x000000ff
#define REQDISP_PENDINGFIRST		0x00000100
#define REQDISP_DEFAULT_STANDALONE	(REQDISP_PENDINGFIRST | REQDISP_ISSUE)
#define REQDISP_DEFAULT_ENTERPRISE	(REQDISP_ISSUE)

// wszREGREVOCATIONTYPE Values:
#define REVEXT_CDPLDAPURL_OLD		0x00000001
#define REVEXT_CDPHTTPURL_OLD		0x00000002
#define REVEXT_CDPFTPURL_OLD		0x00000004
#define REVEXT_CDPFILEURL_OLD		0x00000008
#define REVEXT_CDPURLMASK_OLD		0x000000ff
#define REVEXT_CDPENABLE		0x00000100
#define REVEXT_ASPENABLE		0x00000200

#define REVEXT_DEFAULT_NODS		(REVEXT_CDPENABLE)
#define REVEXT_DEFAULT_DS		(REVEXT_CDPENABLE)

// wszREGISSUERCERTURLFLAGS Values:
#define ISSCERT_LDAPURL_OLD		0x00000001
#define ISSCERT_HTTPURL_OLD		0x00000002
#define ISSCERT_FTPURL_OLD		0x00000004
#define ISSCERT_FILEURL_OLD		0x00000008
#define ISSCERT_URLMASK_OLD		0x000000ff
#define ISSCERT_ENABLE			0x00000100

#define ISSCERT_DEFAULT_NODS		(ISSCERT_ENABLE)
#define ISSCERT_DEFAULT_DS		(ISSCERT_ENABLE)

// wszREGEDITFLAGS Values:				   Defaults:
// Under CA key: wszREGCRLEDITFLAGS Values (EDITF_ENABLEAKI* only):
#define EDITF_ENABLEREQUESTEXTENSIONS	0x00000001	// neither
#define EDITF_REQUESTEXTENSIONLIST	0x00000002	// both
#define EDITF_DISABLEEXTENSIONLIST	0x00000004	// both
#define EDITF_ADDOLDKEYUSAGE		0x00000008	// both
#define EDITF_ADDOLDCERTTYPE		0x00000010	// neither
#define EDITF_ATTRIBUTEENDDATE		0x00000020	// Standalone
#define EDITF_BASICCONSTRAINTSCRITICAL	0x00000040	// both
#define EDITF_BASICCONSTRAINTSCA	0x00000080	// Standalone
#define EDITF_ENABLEAKIKEYID		0x00000100	// both
#define EDITF_ATTRIBUTECA		0x00000200	// Standalone
#define EDITF_IGNOREREQUESTERGROUP      0x00000400	// neither
#define EDITF_ENABLEAKIISSUERNAME	0x00000800	// neither
#define EDITF_ENABLEAKIISSUERSERIAL	0x00001000	// neither
#define EDITF_ENABLEAKICRITICAL		0x00002000	// neither
#define EDITF_SERVERUPGRADED		0x00004000	// neither
#define EDITF_ATTRIBUTEEKU		0x00008000	// Standalone
#define EDITF_ENABLEDEFAULTSMIME	0x00010000	// Enterprise
#define EDITF_EMAILOPTIONAL		0x00020000	// neither
#define EDITF_ATTRIBUTESUBJECTALTNAME2	0x00040000	// neither
#define EDITF_ENABLELDAPREFERRALS	0x00080000	// neither
#define EDITF_ENABLECHASECLIENTDC	0x00100000	// Enterprise
#define EDITF_AUDITCERTTEMPLATELOAD	0x00200000	// neither
#define EDITF_DISABLEOLDOSCNUPN         0x00400000	// neither
#define EDITF_DISABLELDAPPACKAGELIST	0x00800000	// neither
#define EDITF_ENABLEUPNMAP		0x01000000	// neither
#define EDITF_ENABLEOCSPREVNOCHECK	0x02000000	// neither
#define EDITF_ENABLERENEWONBEHALFOF	0x04000000	// Enterprise
#define EDITF_ENABLEKEYENCIPHERMENTCACERT 0x08000000    // neither

#define EDITF_DEFAULT_STANDALONE	(EDITF_REQUESTEXTENSIONLIST | \
					 EDITF_DISABLEEXTENSIONLIST | \
					 EDITF_ADDOLDKEYUSAGE | \
					 EDITF_ATTRIBUTEENDDATE | \
					 EDITF_BASICCONSTRAINTSCRITICAL | \
					 EDITF_BASICCONSTRAINTSCA | \
					 EDITF_ENABLEAKIKEYID | \
					 EDITF_ATTRIBUTECA | \
					 EDITF_ATTRIBUTEEKU)

#define EDITF_DEFAULT_ENTERPRISE	(EDITF_REQUESTEXTENSIONLIST | \
					 EDITF_DISABLEEXTENSIONLIST | \
					 EDITF_ADDOLDKEYUSAGE | \
                                         EDITF_BASICCONSTRAINTSCRITICAL | \
                                         EDITF_ENABLEAKIKEYID | \
					 EDITF_ENABLEDEFAULTSMIME | \
					 EDITF_ENABLECHASECLIENTDC)


//======================================================================
// Values Under "CertSvc\Configuration\<CAName>\ExitModules\<ProgId>":

// LDAP based CRL and URL issuance
#define wszREGLDAPREVOCATIONDN_OLD	   TEXT("LDAPRevocationDN")
#define wszREGLDAPREVOCATIONDNTEMPLATE_OLD TEXT("LDAPRevocationDNTemplate")
#define wszCRLPUBLISHRETRYCOUNT    TEXT("CRLPublishRetryCount")
#define wszREGCERTPUBLISHFLAGS     TEXT("PublishCertFlags")

// wszREGCERTPUBLISHFLAGS Values:
#define EXITPUB_FILE			0x00000001
#define EXITPUB_ACTIVEDIRECTORY		0x00000002
#define EXITPUB_REMOVEOLDCERTS		0x00000010

#define EXITPUB_DEFAULT_ENTERPRISE	EXITPUB_ACTIVEDIRECTORY

#define EXITPUB_DEFAULT_STANDALONE	EXITPUB_FILE


#define wszCLASS_CERTADMIN	  TEXT("CertificateAuthority.Admin")
#define wszCLASS_CERTCONFIG	  TEXT("CertificateAuthority.Config")
#define wszCLASS_CERTGETCONFIG	  TEXT("CertificateAuthority.GetConfig")

#define wszCLASS_CERTENCODE	  TEXT("CertificateAuthority.Encode")
#define wszCLASS_CERTDBMEM    TEXT("CertificateAuthority.DBMem") // no_certs    
#define wszCLASS_CERTREQUEST	  TEXT("CertificateAuthority.Request")
#define wszCLASS_CERTSERVEREXIT   TEXT("CertificateAuthority.ServerExit")
#define wszCLASS_CERTSERVERPOLICY TEXT("CertificateAuthority.ServerPolicy")
#define wszCLASS_CERTVIEW	  TEXT("CertificateAuthority.View")

// class name templates
#define wszMICROSOFTCERTMODULE_PREFIX  TEXT("CertificateAuthority_MicrosoftDefault") 
#define wszCERTMANAGE_SUFFIX TEXT("Manage")
#define wszCERTEXITMODULE_POSTFIX	TEXT(".Exit")
#define wszCERTMANAGEEXIT_POSTFIX	wszCERTEXITMODULE_POSTFIX wszCERTMANAGE_SUFFIX
#define wszCERTPOLICYMODULE_POSTFIX	TEXT(".Policy")
#define wszCERTMANAGEPOLICY_POSTFIX	wszCERTPOLICYMODULE_POSTFIX wszCERTMANAGE_SUFFIX


// actual policy/exit manage class names
#define wszCLASS_CERTMANAGEEXITMODULE   wszMICROSOFTCERTMODULE_PREFIX wszCERTMANAGEEXIT_POSTFIX 

#define wszCLASS_CERTMANAGEPOLICYMODULE wszMICROSOFTCERTMODULE_PREFIX wszCERTMANAGEPOLICY_POSTFIX 

// actual policy/exit class names
#define wszCLASS_CERTEXIT	wszMICROSOFTCERTMODULE_PREFIX wszCERTEXITMODULE_POSTFIX

#define wszCLASS_CERTPOLICY	wszMICROSOFTCERTMODULE_PREFIX wszCERTPOLICYMODULE_POSTFIX


#define wszCAPOLICYFILE			L"CAPolicy.inf"

#define wszINFSECTION_CDP		L"CRLDistributionPoint"
#define wszINFSECTION_AIA		L"AuthorityInformationAccess"
#define wszINFSECTION_EKU		L"EnhancedKeyUsageExtension"
#define wszINFSECTION_CCDP		L"CrossCertificateDistributionPointsExtension"

#define wszINFSECTION_CERTSERVER	L"certsrv_server"
#define wszINFKEY_RENEWALKEYLENGTH	L"RenewalKeyLength"
#define wszINFKEY_RENEWALVALIDITYPERIODSTRING	L"RenewalValidityPeriod"
#define wszINFKEY_RENEWALVALIDITYPERIODCOUNT	L"RenewalValidityPeriodUnits"
#define wszINFKEY_UTF8			L"UTF8"
#define wszINFKEY_CRLPERIODSTRING	wszREGCRLPERIODSTRING
#define wszINFKEY_CRLPERIODCOUNT	wszREGCRLPERIODCOUNT
#define wszINFKEY_CRLDELTAPERIODSTRING	wszREGCRLDELTAPERIODSTRING
#define wszINFKEY_CRLDELTAPERIODCOUNT	wszREGCRLDELTAPERIODCOUNT
#define wszINFKEY_LOADDEFAULTTEMPLATES  L"LoadDefaultTemplates"
#define wszINFKEY_ENABLEKEYCOUNTING     L"EnableKeyCounting"
#define wszINFKEY_FORCEUTF8		L"ForceUTF8"
#define wszINFKEY_ALTERNATESIGNATUREALGORITHM wszREGALTERNATESIGNATUREALGORITHM
#define wszINFKEY_SHOWALLCSPS           L"ShowAllCSPs"

#define wszINFKEY_CRITICAL		L"Critical"
#define wszINFKEY_EMPTY			L"Empty"

#define wszINFKEY_CCDPSYNCDELTATIME	L"SyncDeltaTime"

#define wszINFSECTION_CAPOLICY		L"CAPolicy"
#define wszINFSECTION_POLICYSTATEMENT	L"PolicyStatementExtension"
#define wszINFSECTION_APPLICATIONPOLICYSTATEMENT	L"ApplicationPolicyStatementExtension"
#define wszINFKEY_POLICIES		L"Policies"
#define wszINFKEY_OID			L"OID"
#define wszINFKEY_NOTICE		L"Notice"
#define wszINFKEY_FLAGS			L"Flags"

#define wszINFSECTION_REQUESTATTRIBUTES	L"RequestAttributes"

#define wszINFSECTION_NAMECONSTRAINTS	L"NameConstraintsExtension"
#define wszINFKEY_INCLUDE		L"Include"
#define wszINFKEY_EXCLUDE		L"Exclude"

// for [Extensions] section Name Constraints processing:
#define wszINFKEY_SUBTREE		L"SubTree"

#define wszINFKEY_UPN			L"UPN"
#define wszINFKEY_EMAIL			L"EMail"
#define wszINFKEY_DNS			L"DNS"
#define wszINFKEY_DIRECTORYNAME		L"DirectoryName"
#define wszINFKEY_URL			L"URL"
#define wszINFKEY_IPADDRESS		L"IPAddress"
#define wszINFKEY_REGISTEREDID		L"RegisteredId"
#define wszINFKEY_OTHERNAME		L"OtherName"

#define wszINFSECTION_POLICYMAPPINGS	L"PolicyMappingsExtension"
#define wszINFSECTION_APPLICATIONPOLICYMAPPINGS	L"ApplicationPolicyMappingsExtension"

#define wszINFSECTION_POLICYCONSTRAINTS	L"PolicyConstraintsExtension"
#define wszINFSECTION_APPLICATIONPOLICYCONSTRAINTS	L"ApplicationPolicyConstraintsExtension"
#define wszINFKEY_REQUIREEXPLICITPOLICY	L"RequireExplicitPolicy"
#define wszINFKEY_INHIBITPOLICYMAPPING	L"InhibitPolicyMapping"

#define wszINFSECTION_BASICCONSTRAINTS	L"BasicConstraintsExtension"
#define wszINFKEY_PATHLENGTH		L"PathLength"

#define wszINFSECTION_EXTENSIONS	L"Extensions"
#define wszINFSECTION_PROPERTIES	L"Properties"

#define wszINFKEY_CONTINUE		L"_continue_"


#define wszINFSECTION_NEWREQUEST	L"NewRequest"
#define wszINFKEY_SUBJECT		L"Subject"
#define wszINFKEY_SUBJECTNAMEFLAGS	L"SubjectNameFlags"
#define wszINFKEY_X500NAMEFLAGS		L"X500NameFlags"
#define wszINFKEY_EXPORTABLE		L"Exportable"
#define wszINFKEY_EXPORTABLEENCRYPTED	L"ExportableEncrypted"
#define wszINFKEY_HASHALGORITHM		L"HashAlgorithm"
#define wszINFKEY_KEYALGORITHM		L"KeyAlgorithm"
#define wszINFKEY_KEYALGORITHMPARMETERS	L"KeyAlgorithmParameters"
#define wszINFKEY_KEYCONTAINER		L"KeyContainer"
#define wszINFKEY_READERNAME		L"ReaderName"
#define wszINFKEY_KEYLENGTH		L"KeyLength"
#define wszINFKEY_LEGACYKEYSPEC		L"KeySpec"
#define wszINFKEY_KEYUSAGEEXTENSION	L"KeyUsage"
#define wszINFKEY_KEYUSAGEPROPERTY	L"KeyUsageProperty"
#define wszINFKEY_MACHINEKEYSET		L"MachineKeySet"
#define wszINFKEY_PRIVATEKEYARCHIVE	L"PrivateKeyArchive"
#define wszINFKEY_ENCRYPTIONALGORITHM	L"EncryptionAlgorithm"
#define wszINFKEY_ENCRYPTIONLENGTH	L"EncryptionLength"
#define wszINFKEY_PROVIDERNAME		L"ProviderName"
#define wszINFKEY_PROVIDERTYPE		L"ProviderType"
#define wszINFKEY_READERNAME		L"ReaderName"
#define wszINFKEY_RENEWALCERT		L"RenewalCert"
#define wszINFKEY_REQUESTERNAME         wszPROPREQUESTERNAME
#define wszINFKEY_REQUESTTYPE		L"RequestType"
#define wszINFKEY_SECURITYDESCRIPTOR	L"SecurityDescriptor"
//#define wszINFKEY_ALTERNATESIGNATUREALGORITHM wszREGALTERNATESIGNATUREALGORITHM
#define wszINFKEY_SILENT		L"Silent"
#define wszINFKEY_SMIME			L"SMIME"
#define wszINFKEY_SUPPRESSDEFAULTS	L"SuppressDefaults"
#define wszINFKEY_USEEXISTINGKEY	L"UseExistingKeySet"
#define wszINFKEY_USERPROTECTED		L"UserProtected"	// deprecated
#define wszINFKEY_KEYPROTECTION		L"KeyProtection"
#define wszINFKEY_UICONTEXTMESSAGE	L"UIContextMessage"
#define wszINFKEY_FRIENDLYNAME		L"FriendlyName"
#define wszINFKEY_NOTBEFORE		L"NotBefore"
#define wszINFKEY_NOTAFTER		L"NotAfter"
#define wszINFKEY_ATTESTPRIVATEKEY	L"AttestPrivateKey"
#define wszINFKEY_PUBLICKEY		L"PublicKey"
#define wszINFKEY_PUBLICKEYPARAMETERS	L"PublicKeyParameters"
#define wszINFKEY_ECCKEYPARAMETERS	L"EccKeyParameters"
#define wszINFKEY_ECCKEYPARAMETERS_P	L"EccKeyParameters_P"
#define wszINFKEY_ECCKEYPARAMETERS_A	L"EccKeyParameters_A"
#define wszINFKEY_ECCKEYPARAMETERS_B	L"EccKeyParameters_B"
#define wszINFKEY_ECCKEYPARAMETERS_SEED	L"EccKeyParameters_Seed"
#define wszINFKEY_ECCKEYPARAMETERS_BASE	L"EccKeyParameters_Base"
#define wszINFKEY_ECCKEYPARAMETERS_ORDER L"EccKeyParameters_Order"
#define wszINFKEY_ECCKEYPARAMETERS_COFACTOR L"EccKeyParameters_Cofactor"
#define wszINFKEY_ECCKEYPARAMETERSTYPE	L"EccKeyParametersType"
#define wszINFKEY_SERIALNUMBER		L"SerialNumber"
#define wszINFKEY_CATHUMBPRINT		L"CAThumbprint"
#define wszINFKEY_CACERTS		L"CACerts"
#define wszINFKEY_CACAPABILITIES	L"CACapabilities"
#define wszINFKEY_CHALLENGEPASSWORD	L"ChallengePassword"

#define wszINFVALUE_REQUESTTYPE_PKCS10	L"PKCS10"
#define wszINFVALUE_REQUESTTYPE_PKCS7	L"PKCS7"
#define wszINFVALUE_REQUESTTYPE_CMC	L"CMC"
#define wszINFVALUE_REQUESTTYPE_CERT	L"Cert"
#define wszINFVALUE_REQUESTTYPE_SCEP	L"SCEP"

#define wszINFVALUE_ENDORSEMENTKEY	L"EndorsementKey"



//======================================================================
// Values Under "CertSvc\Configuration\<CAName>\ExitModules\CertificateAuthority_MicrosoftDefault.Exit\SMTP":
//
// exit module mail support
//
#define wszREGEXITSMTPKEY	    	L"SMTP"
#define wszREGEXITSMTPTEMPLATES		L"Templates"
#define wszREGEXITSMTPEVENTFILTER	L"EventFilter"
#define wszREGEXITSMTPSERVER		L"SMTPServer"
#define wszREGEXITSMTPAUTHENTICATE	L"SMTPAuthenticate"

// Subkeys:
#define wszREGEXITDENIEDKEY		L"Denied"
#define wszREGEXITISSUEDKEY		L"Issued"
#define wszREGEXITPENDINGKEY		L"Pending"
#define wszREGEXITREVOKEDKEY		L"Revoked"
#define wszREGEXITCRLISSUEDKEY		L"CRLIssued"
#define wszREGEXITSHUTDOWNKEY		L"Shutdown"
#define wszREGEXITSTARTUPKEY		L"Startup"
#define wszREGEXITIMPORTEDKEY		L"Imported"

//======================================================================
// Values Under 
// "CertSvc\Configuration\<CAName>\ExitModules\CertificateAuthority_MicrosoftDefault.Exit\SMTP\Issued| 
// Pending|Denied|Revoked|CRLIssued|Shutdown":
#define wszREGEXITSMTPFROM		L"From"
#define wszREGEXITSMTPTO		L"To"
#define wszREGEXITSMTPCC		L"Cc"
#define wszREGEXITTITLEFORMAT		L"TitleFormat"
#define wszREGEXITTITLEARG		L"TitleArg"
#define wszREGEXITBODYFORMAT		L"BodyFormat"
#define wszREGEXITBODYARG		L"BodyArg"

#define wszREGEXITPROPNOTFOUND		L"???"

//======================================================================
// Full path to HKLM or HKCU "AutoEnrollment" key:
#define wszREGKEYENROLLMENT	L"Software\\Microsoft\\Cryptography\\AutoEnrollment"
#define wszREGKEYGROUPPOLICYENROLLMENT	L"Software\\Policies\\Microsoft\\Cryptography\\AutoEnrollment"

// Values Under "...\Cryptography\AutoEnrollment"

#define wszREGMAXPENDINGREQUESTDAYS	TEXT("MaxPendingRequestDays")
#define wszREGAELOGLEVEL_OLD		TEXT("AEEventLogLevel")	// obsolete
// #define wszREGLOGLEVEL		TEXT("LogLevel")	// new
// #define wszREGCERTSRVDEBUG		TEXT("Debug")
// #define AUTO_ENROLLMENT_FLAG		TEXT("AEFlags")	 	// autoenr.h
#define wszREGENROLLFLAGS		TEXT("EnrollFlags")
#define wszREGVERIFYFLAGS		TEXT("VerifyFlags")
#define wszREGUNICODE			TEXT("Unicode")
#define wszREGAIKCLOUDCAURL		TEXT("AIKCloudCAURL")
#define wszREGAIKKEYALGORITHM		TEXT("AIKKeyAlgorithm")
#define wszREGAIKKEYLENGTH		TEXT("AIKKeyLength")
#define wszREGPRESERVESCEPDUMMYCERTS	TEXT("PreserveSCEPDummyCerts")

// Keys Under "...\Cryptography\AutoEnrollment"
#define wszREGKEYDISALLOWEDSCEPALGS	wszREGKEYENROLLMENT L"\\DisallowedSCEPAlgorithms"
#define wszREGKEYTEMPLATEPOLICY		wszREGKEYENROLLMENT L"\\TemplatePolicy"

// Values Under "...\Cryptography\AutoEnrollment\DisallowedSCEPAlgorithms"
#define wszREGALLPROVIDERS		L"All"
// Or KSP name(s)

// Values Under "...\Cryptography\AutoEnrollment\TemplatePolicy"
// Set low bit to 1 to force machine GP lookup for user enrollment
// Set low bit to 0 (or if reg value is missing) to use default behavior: user context GP lookup
// Only applies to InitializeFromTemplateName methods using default policy & only from user context.
// REG_DWORD <Internal template Name>
#define TP_MACHINEPOLICY	0x00000001

// Set low bit to 0 to disable machine key repair (missing reg value enables)
// Set second bit to 0 to disable user key repair (missing reg value enables)
#define wszREGKEYREPAIR			TEXT("KeyRepair")
#define KR_ENABLE_MACHINE	0x00000001
#define KR_ENABLE_USER		0x00000002

//==========================================================================================
// (Un)Installation of each ADCS role create a sub key with names (CertificateAuthority, 
// WebEnrollment, CEP, CES, NDES) and then adds the "ConfigurationStatus" reg value 
// to indicate whether a particular role has been configured (2), not configured (1) 
// or failed (0). 
// If customers require a different EP setting than default value, they should create
// "EPTokenCheckValue" reg value directly under the ""..\\ADCS" parent key before starting 
// configuration of the given role. This should be created on the machine where the given 
// role is being configured and not on the machine where CA role is configured.

#define CONFIGURATION_STATUS_PARENT_REG_PATH    TEXT("Software\\Microsoft\\ADCS")
#define CONFIGURATION_STATUS_REG_VALUE_NAME     TEXT("ConfigurationStatus")

#define CONFIGURATION_REG_EPTOKENCHECKVALUE     TEXT("EPTokenCheckValue")
#define EP_TOKENCHECK_DEFAULT_VALUE             2  // Set it to "Always" by default

// Reg value to disable https only fix. Adding this config option mainly for 
// test code purpose. I expect lot of test code relying on http end point for
// certsrv and mscep_admin interfaces.
#define CONFIGURATION_REG_DISABLE_HTTPSONLY     TEXT("DisableHTTPSOnly")

//+--------------------------------------------------------------------------
// Name properties:

#define wszPROPDISTINGUISHEDNAME   TEXT("DistinguishedName")
#define wszPROPRAWNAME             TEXT("RawName")

#define wszPROPCOUNTRY             TEXT("Country")
#define wszPROPORGANIZATION        TEXT("Organization")
#define wszPROPORGUNIT             TEXT("OrgUnit")
#define wszPROPCOMMONNAME          TEXT("CommonName")
#define wszPROPLOCALITY            TEXT("Locality")
#define wszPROPSTATE               TEXT("State")
#define wszPROPTITLE               TEXT("Title")
#define wszPROPGIVENNAME           TEXT("GivenName")
#define wszPROPINITIALS            TEXT("Initials")
#define wszPROPSURNAME             TEXT("SurName")
#define wszPROPDOMAINCOMPONENT     TEXT("DomainComponent")
#define wszPROPEMAIL               TEXT("EMail")
#define wszPROPSTREETADDRESS       TEXT("StreetAddress")
#define wszPROPUNSTRUCTUREDNAME    TEXT("UnstructuredName")
#define wszPROPUNSTRUCTUREDADDRESS TEXT("UnstructuredAddress")
#define wszPROPDEVICESERIALNUMBER  TEXT("DeviceSerialNumber")

//+--------------------------------------------------------------------------
// Subject Name properties:

#define wszPROPSUBJECTDOT	    TEXT("Subject.")
#define wszPROPSUBJECTDISTINGUISHEDNAME \
				    wszPROPSUBJECTDOT wszPROPDISTINGUISHEDNAME
#define wszPROPSUBJECTRAWNAME       wszPROPSUBJECTDOT wszPROPRAWNAME

#define wszPROPSUBJECTCOUNTRY       wszPROPSUBJECTDOT wszPROPCOUNTRY
#define wszPROPSUBJECTORGANIZATION  wszPROPSUBJECTDOT wszPROPORGANIZATION
#define wszPROPSUBJECTORGUNIT       wszPROPSUBJECTDOT wszPROPORGUNIT
#define wszPROPSUBJECTCOMMONNAME    wszPROPSUBJECTDOT wszPROPCOMMONNAME
#define wszPROPSUBJECTLOCALITY      wszPROPSUBJECTDOT wszPROPLOCALITY
#define wszPROPSUBJECTSTATE         wszPROPSUBJECTDOT wszPROPSTATE
#define wszPROPSUBJECTTITLE	    wszPROPSUBJECTDOT wszPROPTITLE
#define wszPROPSUBJECTGIVENNAME	    wszPROPSUBJECTDOT wszPROPGIVENNAME
#define wszPROPSUBJECTINITIALS	    wszPROPSUBJECTDOT wszPROPINITIALS
#define wszPROPSUBJECTSURNAME	    wszPROPSUBJECTDOT wszPROPSURNAME
#define wszPROPSUBJECTDOMAINCOMPONENT wszPROPSUBJECTDOT wszPROPDOMAINCOMPONENT
#define wszPROPSUBJECTEMAIL	    wszPROPSUBJECTDOT wszPROPEMAIL
#define wszPROPSUBJECTSTREETADDRESS wszPROPSUBJECTDOT wszPROPSTREETADDRESS
#define wszPROPSUBJECTUNSTRUCTUREDNAME wszPROPSUBJECTDOT wszPROPUNSTRUCTUREDNAME
#define wszPROPSUBJECTUNSTRUCTUREDADDRESS wszPROPSUBJECTDOT wszPROPUNSTRUCTUREDADDRESS
#define wszPROPSUBJECTDEVICESERIALNUMBER wszPROPSUBJECTDOT wszPROPDEVICESERIALNUMBER


//+--------------------------------------------------------------------------
// Request properties:
#define wszPROPREQUESTDOT                       TEXT("Request.")

#define wszPROPREQUESTREQUESTID                 TEXT("RequestID")
#define wszPROPREQUESTRAWREQUEST                TEXT("RawRequest")
#define wszPROPREQUESTRAWARCHIVEDKEY            TEXT("RawArchivedKey")
#define wszPROPREQUESTARCHIVEDKEY               TEXT("ArchivedKey")    // constructed
#define wszPROPREQUESTKEYRECOVERYHASHES         TEXT("KeyRecoveryHashes")
#define wszPROPREQUESTRAWOLDCERTIFICATE         TEXT("RawOldCertificate")
#define wszPROPREQUESTATTRIBUTES                TEXT("RequestAttributes")
#define wszPROPREQUESTTYPE                      TEXT("RequestType")
#define wszPROPREQUESTFLAGS                     TEXT("RequestFlags")
#define wszPROPREQUESTSTATUSCODE                TEXT("StatusCode")
#define wszPROPREQUESTDISPOSITION               TEXT("Disposition")
#define wszPROPREQUESTDISPOSITIONMESSAGE        TEXT("DispositionMessage")
#define wszPROPREQUESTSUBMITTEDWHEN             TEXT("SubmittedWhen")
#define wszPROPREQUESTRESOLVEDWHEN              TEXT("ResolvedWhen")
#define wszPROPREQUESTREVOKEDWHEN               TEXT("RevokedWhen")
#define wszPROPREQUESTREVOKEDEFFECTIVEWHEN      TEXT("RevokedEffectiveWhen")
#define wszPROPREQUESTREVOKEDREASON             TEXT("RevokedReason")
#define wszPROPREQUESTERNAME                    TEXT("RequesterName")
#define wszPROPCALLERNAME                       TEXT("CallerName")
#define wszPROPSIGNERPOLICIES                   TEXT("SignerPolicies")
#define wszPROPSIGNERAPPLICATIONPOLICIES        TEXT("SignerApplicationPolicies")
#define wszPROPOFFICER                          TEXT("Officer")
#define wszPROPPUBLISHEXPIREDCERTINCRL          TEXT("PublishExpiredCertInCRL")
#define wszPROPREQUESTERNAMEFROMOLDCERTIFICATE  TEXT("RequesterNameFromOldCertificate")
#define wszPROPATTESTATIONCHALLENGE             TEXT("AttestationChallenge")
#define wszPROPENDORSEMENTKEYHASH               TEXT("EndorsementKeyHash")
#define wszPROPENDORSEMENTCERTIFICATEHASH       TEXT("EndorsementCertificateHash")
#define wszPROPRAWPRECERTIFICATE                TEXT("RawPrecertificate")
#define wszPROPCRLPARTITIONINDEX                TEXT("CRLPartitionIndex")
#define wszPROPLINTERCERTIFICATE                TEXT("LinterCertificate")

//+--------------------------------------------------------------------------
// Request attribute properties:

#define wszPROPCHALLENGE		TEXT("Challenge")
#define wszPROPEXPECTEDCHALLENGE	TEXT("ExpectedChallenge")

#define wszPROPDISPOSITION		TEXT("Disposition")
#define wszPROPDISPOSITIONDENY		TEXT("Deny")
#define wszPROPDISPOSITIONPENDING	TEXT("Pending")

#define wszPROPVALIDITYPERIODSTRING	TEXT("ValidityPeriod")
#define wszPROPVALIDITYPERIODCOUNT	TEXT("ValidityPeriodUnits")
#define wszPROPEXPIRATIONDATE           TEXT("ExpirationDate")

#define wszPROPCERTTYPE			TEXT("CertType")
#define wszPROPCERTTEMPLATE		TEXT("CertificateTemplate")
#define wszPROPCERTUSAGE		TEXT("CertificateUsage")

#define wszPROPREQUESTOSVERSION		TEXT("RequestOSVersion")
#define wszPROPREQUESTCSPPROVIDER       TEXT("RequestCSPProvider")

#define wszPROPEXITCERTFILE		TEXT("CertFile")
#define wszPROPCLIENTBROWSERMACHINE	TEXT("cbm")
#define wszPROPCERTCLIENTMACHINE	TEXT("ccm")
#define wszPROPCLIENTDCDNS		L"cdc"
#define wszPROPREQUESTMACHINEDNS	L"rmd"
#define wszPROPSUBJECTALTNAME2		TEXT("san")
#define wszPROPDNS			TEXT("dns")
#define wszPROPDN			TEXT("dn")
#define wszPROPURL			TEXT("url")
#define wszPROPIPADDRESS		TEXT("ipaddress")
#define wszPROPGUID			TEXT("guid")
#define wszPROPOID			TEXT("oid")
#define wszPROPUPN			TEXT("upn")
#define wszPROPUPN			TEXT("upn")

#define szPROPASNTAG			"{asn}"

#define wszPROPCRITICALTAG		TEXT("{critical}")
#define wszPROPUTF8TAG			TEXT("{utf8}")
#define wszPROPOCTETTAG			TEXT("{octet}")
#define wszPROPHEXTAG			TEXT("{hex}")
#define wszPROPASNTAG			TEXT(szPROPASNTAG)
#define wszPROPTEXTTAG			TEXT("{text}")
#define wszPROPDECIMALTAG		TEXT("{decimal}")
#define wszPROPFILETAG			TEXT("{file}")

#define wszAT_EKCERTINF			TEXT("@EKCert")
#define wszAT_TESTROOT			TEXT("@TestRoot")

#define wszPROPLINTCERTIFICATE	TEXT("LintCertificate")

//+--------------------------------------------------------------------------
// "System" properties
// ".#" means ".0", ".1", ".2" ... may be appended to the property name to
// collect context specific values.  For some properties, the suffix selects
// the CA certificate context.  For others, it selects the the CA CRL context.

#define wszPROPCATYPE                   TEXT("CAType")
#define wszPROPSANITIZEDCANAME          TEXT("SanitizedCAName")
#define wszPROPSANITIZEDSHORTNAME       TEXT("SanitizedShortName")
#define wszPROPMACHINEDNSNAME           TEXT("MachineDNSName")
#define wszPROPMODULEREGLOC             TEXT("ModuleRegistryLocation")
#define wszPROPUSEDS                    TEXT("fUseDS")
#define wszPROPDELTACRLSDISABLED        TEXT("fDeltaCRLsDisabled")
#define wszPROPSERVERUPGRADED           TEXT("fServerUpgraded")
#define wszPROPCONFIGDN			TEXT("ConfigDN")
#define wszPROPDOMAINDN			TEXT("DomainDN")
#define wszPROPLOGLEVEL			TEXT("LogLevel")
#define wszPROPSESSIONCOUNT		TEXT("SessionCount")
#define wszPROPTEMPLATECHANGESEQUENCENUMBER TEXT("TemplateChangeSequenceNumber")
#define wszPROPVOLATILEMODE		TEXT("VolatileMode")

// ".#" suffix for ENUM_PERIOD_SECONDS, ... ENUM_PERIOD_YEARS
#define wszLOCALIZEDTIMEPERIODUNITS	TEXT("LocalizedTimePeriodUnits")

// Request Context properties:

#define wszPROPREQUESTERCAACCESS	TEXT("RequesterCAAccess")
#define wszPROPUSERDN			TEXT("UserDN")
#define wszPROPKEYARCHIVED		TEXT("KeyArchived")


// CA Certificate properties: (all ".#" extensible except wszPROPCERTCOUNT)

#define wszPROPCERTCOUNT                TEXT("CertCount")
#define wszPROPRAWCACERTIFICATE         TEXT("RawCACertificate")
#define wszPROPCERTSTATE                TEXT("CertState")
#define wszPROPCERTSUFFIX               TEXT("CertSuffix")

// CA CRL properties: (all ".#" extensible)

#define wszPROPRAWCRL                   TEXT("RawCRL")
#define wszPROPRAWDELTACRL              TEXT("RawDeltaCRL")
#define wszPROPCRLINDEX                 TEXT("CRLIndex")
#define wszPROPCRLSTATE                 TEXT("CRLState")
#define wszPROPCRLSUFFIX                TEXT("CRLSuffix")

// Values for wszPROPCERTSTATE (see certadm.h):
//   CA_DISP_REVOKED    // This Cert has been revoked.
//   CA_DISP_VALID      // This Cert is still valid
//   CA_DISP_INVALID    // This Cert has expired.
//   CA_DISP_ERROR      // Cert unavailable (placehholder in registry?)

// Values for wszPROPCRLSTATE (see certadm.h):
//   CA_DISP_REVOKED	// All unexpired certs using this Cert's CRL have been
//			// revoked.
//   CA_DISP_VALID	// This Cert is still publishing CRLs as needed.
//   CA_DISP_INVALID    // All certs using this Cert's CRL are expired.
//   CA_DISP_ERROR      // This Cert's CRL is managed by another Cert.

// "Settable" system properties:
#define wszPROPEVENTLOGTERSE		TEXT("EventLogTerse")
#define wszPROPEVENTLOGERROR		TEXT("EventLogError")
#define wszPROPEVENTLOGWARNING		TEXT("EventLogWarning")
#define wszPROPEVENTLOGVERBOSE		TEXT("EventLogVerbose")
#define wszPROPEVENTLOGEXHAUSTIVE	TEXT("EventLogExhaustive")
#define wszPROPDCNAME			TEXT("DCName")
#define wszPROPCROSSFOREST              TEXT("CrossForest")
#define wszPROPREQUESTERSAMNAME         TEXT("RequesterSAMName")
#define wszPROPREQUESTERUPN             TEXT("RequesterUPN")
#define wszPROPREQUESTERDN              TEXT("RequesterDN")

// "Settable" system properties (".#" extensible)
#define wszPROPSEAUDITID		TEXT("SEAuditId")

// "Fetchable" system properties
#define wszPROPSEAUDITFILTER		TEXT("SEAuditFilter")

//+--------------------------------------------------------------------------
// Certificate properties:

#define wszPROPCERTIFICATEREQUESTID	       TEXT("RequestID")
#define wszPROPRAWCERTIFICATE		       TEXT("RawCertificate")
#define wszPROPCERTIFICATEHASH		       TEXT("CertificateHash")
#define wszPROPCERTIFICATETEMPLATE	       TEXT("CertificateTemplate")
#define wszPROPCERTIFICATEENROLLMENTFLAGS      TEXT("EnrollmentFlags")
#define wszPROPCERTIFICATEGENERALFLAGS         TEXT("GeneralFlags")
#define wszPROPCERTIFICATEPRIVATEKEYFLAGS      TEXT("PrivatekeyFlags")
#define wszPROPCERTIFICATESERIALNUMBER	       TEXT("SerialNumber")
#define wszPROPCERTIFICATENOTBEFOREDATE	       TEXT("NotBefore")
#define wszPROPCERTIFICATENOTAFTERDATE	       TEXT("NotAfter")
#define wszPROPCERTIFICATESUBJECTKEYIDENTIFIER TEXT("SubjectKeyIdentifier")
#define wszPROPCERTIFICATERAWPUBLICKEY	       TEXT("RawPublicKey")
#define wszPROPCERTIFICATEPUBLICKEYLENGTH      TEXT("PublicKeyLength")
#define wszPROPCERTIFICATEPUBLICKEYALGORITHM   TEXT("PublicKeyAlgorithm")
#define wszPROPCERTIFICATERAWPUBLICKEYALGORITHMPARAMETERS \
    TEXT("RawPublicKeyAlgorithmParameters")
#define wszPROPCERTIFICATEUPN		       TEXT("UPN")


// Obsolete:
#define wszPROPCERTIFICATETYPE		       TEXT("CertificateType")
#define wszPROPCERTIFICATERAWSMIMECAPABILITIES TEXT("RawSMIMECapabilities")
#define wszPROPNAMETYPE			       TEXT("NameType")

//+--------------------------------------------------------------------------
// Certificate extension properties:

#define EXTENSION_CRITICAL_FLAG	      0x00000001
#define EXTENSION_DISABLE_FLAG	      0x00000002
#define EXTENSION_DELETE_FLAG         0x00000004
#define EXTENSION_POLICY_MASK	      0x0000ffff // Settable by admin+policy

#define EXTENSION_ORIGIN_REQUEST      0x00010000
#define EXTENSION_ORIGIN_POLICY	      0x00020000
#define EXTENSION_ORIGIN_ADMIN	      0x00030000
#define EXTENSION_ORIGIN_SERVER	      0x00040000
#define EXTENSION_ORIGIN_RENEWALCERT  0x00050000
#define EXTENSION_ORIGIN_IMPORTEDCERT 0x00060000
#define EXTENSION_ORIGIN_PKCS7	      0x00070000
#define EXTENSION_ORIGIN_CMC	      0x00080000
#define EXTENSION_ORIGIN_CACERT       0x00090000
#define EXTENSION_ORIGIN_MASK	      0x000f0000

//+--------------------------------------------------------------------------
// Extension properties:

#define wszPROPEXTREQUESTID		TEXT("ExtensionRequestId")
#define wszPROPEXTNAME			TEXT("ExtensionName")
#define wszPROPEXTFLAGS			TEXT("ExtensionFlags")
#define wszPROPEXTRAWVALUE		TEXT("ExtensionRawValue")

//+--------------------------------------------------------------------------
// Attribute properties:

#define wszPROPATTRIBREQUESTID		TEXT("AttributeRequestId")
#define wszPROPATTRIBNAME		TEXT("AttributeName")
#define wszPROPATTRIBVALUE		TEXT("AttributeValue")

//+--------------------------------------------------------------------------
// CRL properties:

#define wszPROPCRLROWID			TEXT("CRLRowId")
#define wszPROPCRLNUMBER		TEXT("CRLNumber")
#define wszPROPCRLMINBASE		TEXT("CRLMinBase") // Delta CRLs only
#define wszPROPCRLNAMEID		TEXT("CRLNameId")
#define wszPROPCRLCOUNT			TEXT("CRLCount")
#define wszPROPCRLTHISUPDATE		TEXT("CRLThisUpdate")
#define wszPROPCRLNEXTUPDATE		TEXT("CRLNextUpdate")
#define wszPROPCRLTHISPUBLISH		TEXT("CRLThisPublish")
#define wszPROPCRLNEXTPUBLISH		TEXT("CRLNextPublish")
#define wszPROPCRLEFFECTIVE		TEXT("CRLEffective")
#define wszPROPCRLPROPAGATIONCOMPLETE	TEXT("CRLPropagationComplete")
#define wszPROPCRLLASTPUBLISHED		TEXT("CRLLastPublished")
#define wszPROPCRLPUBLISHATTEMPTS	TEXT("CRLPublishAttempts")
#define wszPROPCRLPUBLISHFLAGS		TEXT("CRLPublishFlags")
#define wszPROPCRLPUBLISHSTATUSCODE	TEXT("CRLPublishStatusCode")
#define wszPROPCRLPUBLISHERROR		TEXT("CRLPublishError")
#define wszPROPCRLRAWCRL		TEXT("CRLRawCRL")

//+--------------------------------------------------------------------------
// CRL Published Flags:

#define CPF_BASE		0x00000001
#define CPF_DELTA		0x00000002
#define CPF_COMPLETE		0x00000004
#define CPF_SHADOW		0x00000008
#define CPF_CASTORE_ERROR	0x00000010
#define CPF_BADURL_ERROR	0x00000020
#define CPF_MANUAL		0x00000040
#define CPF_SIGNATURE_ERROR	0x00000080
#define CPF_LDAP_ERROR		0x00000100
#define CPF_FILE_ERROR		0x00000200
#define CPF_FTP_ERROR		0x00000400
#define CPF_HTTP_ERROR		0x00000800
#define CPF_POSTPONED_BASE_LDAP_ERROR	0x00001000
#define CPF_POSTPONED_BASE_FILE_ERROR	0x00002000

//+--------------------------------------------------------------------------
// GetProperty/SetProperty Flags:
//
// Choose one Type

#define PROPTYPE_LONG		 0x00000001	// Signed long
#define PROPTYPE_DATE		 0x00000002	// Date+Time
#define PROPTYPE_BINARY		 0x00000003	// Binary data
#define PROPTYPE_STRING		 0x00000004	// Unicode String
#define PROPTYPE_MASK		 0x000000ff

// Choose one Caller:

#define PROPCALLER_SERVER	 0x00000100
#define PROPCALLER_POLICY	 0x00000200
#define PROPCALLER_EXIT		 0x00000300
#define PROPCALLER_ADMIN	 0x00000400
#define PROPCALLER_REQUEST	 0x00000500
#define PROPCALLER_MASK		 0x00000f00
#define PROPFLAGS_INDEXED	 0x00010000	

// RequestFlags definitions:

#define CR_FLG_FORCETELETEX             0x00000001
#define CR_FLG_RENEWAL                  0x00000002
#define CR_FLG_FORCEUTF8                0x00000004
#define CR_FLG_CAXCHGCERT               0x00000008
#define CR_FLG_ENROLLONBEHALFOF         0x00000010
#define CR_FLG_SUBJECTUNMODIFIED        0x00000020
#define CR_FLG_VALIDENCRYPTEDKEYHASH    0x00000040
#define CR_FLG_CACROSSCERT              0x00000080
#define CR_FLG_ENFORCEUTF8              0x00000100
#define CR_FLG_DEFINEDCACERT            0x00000200
#define CR_FLG_CHALLENGEPENDING         0x00000400
#define CR_FLG_CHALLENGESATISFIED       0x00000800
#define CR_FLG_TRUSTONUSE               0x00001000
#define CR_FLG_TRUSTEKCERT              0x00002000
#define CR_FLG_TRUSTEKKEY               0x00004000
#define CR_FLG_PUBLISHERROR             0x80000000

// Disposition property values:

// Disposition values for requests in the queue:
#define DB_DISP_ACTIVE	        8	// being processed
#define DB_DISP_PENDING		9	// taken under submission
#define DB_DISP_QUEUE_MAX	9	// max disposition value for queue view

#define DB_DISP_FOREIGN		12	// archived foreign cert

#define DB_DISP_CA_CERT		15	// CA cert
#define DB_DISP_CA_CERT_CHAIN	16	// CA cert chain
#define DB_DISP_KRA_CERT	17	// KRA cert

// Disposition values for requests in the log:
#define DB_DISP_LOG_MIN		20	// min disposition value for log view
#define DB_DISP_ISSUED		20	// cert issued
#define DB_DISP_REVOKED	        21	// issued and revoked

// Disposition values for failed requests in the log:
#define DB_DISP_LOG_FAILED_MIN	30	// min disposition value for log view
#define DB_DISP_ERROR		30	// request failed
#define DB_DISP_DENIED		31	// request denied


// VerifyRequest() return values

#define VR_PENDING	0	 // request will be accepted or denied later
#define VR_INSTANT_OK	1	 // request was accepted
#define VR_INSTANT_BAD	2	 // request was rejected


//+--------------------------------------------------------------------------
// Known request Attribute names and Value strings

// RequestType attribute name:
#define wszCERT_TYPE		L"RequestType"	// attribute name

// RequestType attribute values:
// Not specified: 				// Non-specific certificate
#define wszCERT_TYPE_CLIENT	L"Client"	// Client authentication cert
#define wszCERT_TYPE_SERVER	L"Server"	// Server authentication cert
#define wszCERT_TYPE_CODESIGN	L"CodeSign"	// Code signing certificate
#define wszCERT_TYPE_CUSTOMER	L"SetCustomer"	// SET Customer certificate
#define wszCERT_TYPE_MERCHANT	L"SetMerchant"	// SET Merchant certificate
#define wszCERT_TYPE_PAYMENT	L"SetPayment"	// SET Payment certificate


// Version attribute name:
#define wszCERT_VERSION		L"Version"	// attribute name

// Version attribute values:
// Not specified: 				// Whetever is current
#define wszCERT_VERSION_1	L"1"		// Version one certificate
#define wszCERT_VERSION_2	L"2"		// Version two certificate
#define wszCERT_VERSION_3	L"3"		// Version three certificate


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _CERTSRV_H_
