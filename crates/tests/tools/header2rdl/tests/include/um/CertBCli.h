//+--------------------------------------------------------------------------
//
// Microsoft Windows
// Copyright (C) Microsoft Corporation, 1996 - 1999
//
// File:        certbcli.h
//
// Contents:    Cert Server backup client APIs
//
//---------------------------------------------------------------------------

#ifdef _CERTBCLI_TYPECHECK
#undef __CERTBCLI_H__	// allow redundant include
#endif

#ifndef __CERTBCLI_H__
#define __CERTBCLI_H__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#ifdef	MIDL_PASS
#define	RPC_STRING [string]
#else
#define	RPC_STRING
#endif

#define IN
#define OUT
#define OPTIONAL
#define VOID		void

#define CERTBCLI_CALL	__stdcall

#ifndef _CERTBCLI_DEFINED
#define CERTBCLI_API __declspec(dllimport) CERTBCLI_CALL
#else
#define CERTBCLI_API
#endif

#define szBACKUPANNOTATION   "Cert Server Backup Interface"
#define wszBACKUPANNOTATION  TEXT(szBACKUPANNOTATION)

#define szRESTOREANNOTATION  "Cert Server Restore Interface"
#define wszRESTOREANNOTATION TEXT(szRESTOREANNOTATION)


// Type of Backup passed to CertSrvBackupPrepare:
// CSBACKUP_TYPE_FULL: Requesting backup of the complete DB (DB & Log files)
// CSBACKUP_TYPE_LOGS_ONLY: Requesting backup of only the log files
// CSBACKUP_TYPE_INCREMENTAL: Requesting incremental backup

// CertSrvBackupPrepare flags:
#define CSBACKUP_TYPE_FULL		0x00000001
#define CSBACKUP_TYPE_LOGS_ONLY		0x00000002
//#define CSBACKUP_TYPE_INCREMENTAL	0x00000004	// not yet supported
#define CSBACKUP_TYPE_MASK		0x00000003	// valid flags

// Type of Restore passed to CertSrvRestorePrepare:
// CSRESTORE_TYPE_FULL: Requesting restore of the complete DB (DB & Log files)
// CSRESTORE_TYPE_ONLINE: Restoration is done when Cert Server is online.

#define CSRESTORE_TYPE_FULL		0x00000001	// else incremental
#define CSRESTORE_TYPE_ONLINE		0x00000002	// not yet supported
#define CSRESTORE_TYPE_CATCHUP		0x00000004	// not yet supported
#define CSRESTORE_TYPE_MASK		0x00000005	// valid flags


// Setting the current log # to this value would disable incremental backup
#define CSBACKUP_DISABLE_INCREMENTAL  0xffffffff


// BFT is the bit flag used to represent file types (directory/dit/logfile/etc.)
// We keep them as a character so that we can append/prepend them to the actual
// file path. The code in the Backup API's rely on the fact that values 0-256
// in 8 bit ascii map to the values 0-256 in unicode.

typedef WCHAR CSBFT;


// Bit flags:
//  CSBFT_DIRECTORY               - path specified is a directory
//  CSBFT_DATABASE_DIRECTORY      - that file goes into database directory
//  CSBFT_LOG_DIRECTORY           - that the file goes into log directory

#define	CSBFT_DIRECTORY		    0x80
#define CSBFT_DATABASE_DIRECTORY    0x40
#define	CSBFT_LOG_DIRECTORY	    0x20

// Following combinations are defined for easy use of the filetype and the
// directory into into which it goes

#define	CSBFT_LOG		  ((CSBFT) (TEXT('\x01') | CSBFT_LOG_DIRECTORY))
#define	CSBFT_LOG_DIR		  ((CSBFT) (TEXT('\x02') | CSBFT_DIRECTORY))
#define	CSBFT_CHECKPOINT_DIR	  ((CSBFT) (TEXT('\x03') | CSBFT_DIRECTORY))
#define	CSBFT_CERTSERVER_DATABASE ((CSBFT) (TEXT('\x04') | CSBFT_DATABASE_DIRECTORY))
#define	CSBFT_PATCH_FILE	  ((CSBFT) (TEXT('\x05') | CSBFT_LOG_DIRECTORY))
#define	CSBFT_UNKNOWN		  ((CSBFT) (TEXT('\x0f')))


// Backup Context Handle
typedef void *HCSBC;

#ifndef CSEDB_RSTMAP
typedef struct tagCSEDB_RSTMAPW {
    RPC_STRING WCHAR *pwszDatabaseName;
    RPC_STRING WCHAR *pwszNewDatabaseName;
} CSEDB_RSTMAPW;

#define CSEDB_RSTMAP CSEDB_RSTMAPW
#endif // CSEDB_RSTMAP


// For all the functions in this interface that have at least one string
// parameter, provide macros to invoke the appropriate version of the
// corresponding function.

#define CertSrvIsServerOnline		CertSrvIsServerOnlineW
#define CertSrvBackupGetDynamicFileList	CertSrvBackupGetDynamicFileListW
#define CertSrvBackupPrepare		CertSrvBackupPrepareW
#define CertSrvBackupGetDatabaseNames	CertSrvBackupGetDatabaseNamesW
#define CertSrvBackupOpenFile		CertSrvBackupOpenFileW
#define CertSrvBackupGetBackupLogs	CertSrvBackupGetBackupLogsW

#define CertSrvRestoreGetDatabaseLocations CertSrvRestoreGetDatabaseLocationsW
#define CertSrvRestorePrepare		CertSrvRestorePrepareW
#define CertSrvRestoreRegister		CertSrvRestoreRegisterW

#define CertSrvServerControl		CertSrvServerControlW


//+--------------------------------------------------------------------------
// CertSrvIsServerOnline -- check to see if the Cert Server is Online on the
//	given server. This call is guaranteed to return quickly.
//
// Parameters:
//	[in]  pwszServerName - name or config string of the server to check
//	[out] pfServerOnline - pointer to receive the bool result
//		(TRUE if Cert Server is online; FALSE, otherwise)
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//+--------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVISSERVERONLINEW)(
    IN  WCHAR const *pwszServerName,
    OUT BOOL *pfServerOnline);

HRESULT
CERTBCLI_API
CertSrvIsServerOnlineW(
    IN  WCHAR const *pwszServerName,
    OUT BOOL *pfServerOnline);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVISSERVERONLINEW *pfnCertSrvIsServerOnline = CertSrvIsServerOnline;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupGetDynamicFileList -- return the list of dynamic files that
//	need to be backed up in addition to database files.
//
// Parameters:
//	[in]  hbc - backup context handle
//	[out] ppwszzFileList - pointer to receive the pointer to the file list;
//		allocated memory should be freed using CertSrvBackupFree() API
//		by the caller when it is no longer needed; The file list info
//		is an array of null-terminated filenames and the list is
//		terminated by two L'\0's.
//	[out] pcbSize - will receive the number of bytes returned
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPGETDYNAMICFILELISTW)(
    IN  HCSBC hbc,
    OUT WCHAR **ppwszzFileList,
    OUT DWORD *pcbSize);

HRESULT
CERTBCLI_API
CertSrvBackupGetDynamicFileListW(
    _In_ IN  HCSBC hbc,
    _Outptr_ PWSTR *ppwszzFileList,
    _Out_ OUT DWORD *pcbSize);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPGETDYNAMICFILELISTW *pfnCertSrvBackupGetDynamicFileList = CertSrvBackupGetDynamicFileList;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupPrepare -- prepare the DB for the online backup and return a
//	Backup Context Handle to be used for subsequent calls to backup
//	functions.
//
// Parameters:
//	[in]  pwszServerName - name or config string of the server to check
//	[in]  grbitJet - flag to be passed to jet while backing up dbs
//	[in]  dwBackupFlags - CSBACKUP_TYPE_FULL or CSBACKUP_TYPE_LOGS_ONLY
//	[out] phbc - pointer that will receive the backup context handle
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPPREPAREW)(
    IN  WCHAR const *pwszServerName,
    IN  ULONG grbitJet,
    IN  ULONG dwBackupFlags,	// CSBACKUP_TYPE_*
    OUT HCSBC *phbc);

HRESULT
CERTBCLI_API
CertSrvBackupPrepareW(
    IN  WCHAR const *pwszServerName,
    IN  ULONG grbitJet,
    IN  ULONG dwBackupFlags,	// CSBACKUP_TYPE_*
    OUT HCSBC *phbc);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPPREPAREW *pfnCertSrvBackupPrepare = CertSrvBackupPrepare;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupGetDatabaseNames -- return the list of data bases that need to
//	be backed up for the given backup context
//
// Parameters:
//	[in]  hbc - backup context handle
//	[out] ppwszzAttachmentInformation - pointer to receive the pointer to
//		the attachment info; allocated memory should be freed using
//		CertSrvBackupFree() API by the caller when it is no longer
//		needed; Attachment info is an array of null-terminated
//		filenames and the list is terminated by two L'\0's.
//	[out] pcbSize - will receive the number of bytes returned
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPGETDATABASENAMESW)(
    IN  HCSBC hbc,
    OUT WCHAR **ppwszzAttachmentInformation,
    OUT DWORD *pcbSize);

HRESULT
CERTBCLI_API
CertSrvBackupGetDatabaseNamesW(
    _In_ IN  HCSBC hbc,
    _Outptr_ PWSTR *ppwszzAttachmentInformation,
    _Out_ OUT DWORD *pcbSize);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPGETDATABASENAMESW *pfnCertSrvBackupGetDatabaseNames = CertSrvBackupGetDatabaseNames;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupOpenFile -- open the given attachment for read.
//
// Parameters:
//	[in]  hbc - backup context handle
//	[in]  pwszAttachmentName - name of the attachment to be opened for read
//	[in]  cbReadHintSize - suggested size in bytes that might be used
//		during the subsequent reads on this attachment
//	[out] pliFileSize - pointer to a large integer that would receive the
//		size in bytes of the given attachment
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPOPENFILEW)(
    IN  HCSBC hbc,
    IN  WCHAR const *pwszAttachmentName,
    IN  DWORD cbReadHintSize,
    OUT LARGE_INTEGER *pliFileSize);

HRESULT
CERTBCLI_API
CertSrvBackupOpenFileW(
    IN  HCSBC hbc,
    IN  WCHAR const *pwszAttachmentName,
    IN  DWORD cbReadHintSize,
    OUT LARGE_INTEGER *pliFileSize);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPOPENFILEW *pfnCertSrvBackupOpenFile = CertSrvBackupOpenFile;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupRead -- read the currently open attachment bytes into the given
//	buffer.  The client application is expected to call this function
//	repeatedly until it gets the entire file (the application would have
//	received the file size through the CertSrvBackupOpenFile call before.
//
// Parameters:
//	[in]  hbc - backup context handle
//	[out] pvBuffer - pointer to the buffer that would receive the read data.
//	[in]  cbBuffer - specifies the size of the above buffer
//	[out] pcbRead - pointer to receive the actual number of bytes read.
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPREAD)(
    IN  HCSBC hbc,
    OUT VOID *pvBuffer,
    IN  DWORD cbBuffer,
    OUT DWORD *pcbRead);

HRESULT
CERTBCLI_API
CertSrvBackupRead(
    IN  HCSBC hbc,
    OUT VOID *pvBuffer,
    IN  DWORD cbBuffer,
    OUT DWORD *pcbRead);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPREAD *pfnCertSrvBackupRead = CertSrvBackupRead;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupClose -- called by the application after it completes reading all
//	the data in the currently opened attachement.
//
// Parameters:
//	[in] hbc - backup context handle
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPCLOSE)(
    IN HCSBC hbc);

HRESULT
CERTBCLI_API
CertSrvBackupClose(
    IN HCSBC hbc);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPCLOSE *pfnCertSrvBackupClose = CertSrvBackupClose;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupGetBackupLogs -- return the list of log files that need to be
//	backed up for the given backup context
//
// Parameters:
//	[in]  hbc - backup context handle
//	[out] pwszzBackupLogFiles - pointer that will receive the pointer to
//		the list of log files; allocated memory should be freed using
//		CertSrvBackupFree() API by the caller when it is no
//		longer needed; Log files are returned in an array of
//		null-terminated filenames and the list is terminated by two
//		L'\0's
//	[out] pcbSize - will receive the number of bytes returned
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPGETBACKUPLOGSW)(
    IN  HCSBC hbc,
    OUT WCHAR **ppwszzBackupLogFiles,
    OUT DWORD *pcbSize);

HRESULT
CERTBCLI_API
CertSrvBackupGetBackupLogsW(
    _In_ IN  HCSBC hbc,
    _Outptr_ PWSTR *ppwszzBackupLogFiles,
    _Out_ OUT DWORD *pcbSize);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPGETBACKUPLOGSW *pfnCertSrvBackupGetBackupLogs = CertSrvBackupGetBackupLogs;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupTruncateLogs -- called to truncate the already read backup logs.
//
// Parameters:
//	[in] hbc - backup context handle
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPTRUNCATELOGS)(
    IN HCSBC hbc);

HRESULT
CERTBCLI_API
CertSrvBackupTruncateLogs(
    IN HCSBC hbc);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPTRUNCATELOGS *pfnCertSrvBackupTruncateLogs = CertSrvBackupTruncateLogs;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupEnd -- called to end the current backup session.
//
// Parameters:
//	[in] hbc - backup context handle of the backup session
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVBACKUPEND)(
    IN HCSBC hbc);

HRESULT
CERTBCLI_API
CertSrvBackupEnd(
    IN HCSBC hbc);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPEND *pfnCertSrvBackupEnd = CertSrvBackupEnd;
#endif


//+--------------------------------------------------------------------------
// CertSrvBackupFree -- free any buffer allocated by certbcli.dll APIs.
//
// Parameters:
//	[in] pv - pointer to the buffer that is to be freed.
//
// Returns:
//	None.
//---------------------------------------------------------------------------

typedef VOID (CERTBCLI_CALL FNCERTSRVBACKUPFREE)(
    IN VOID *pv);

VOID
CERTBCLI_API
CertSrvBackupFree(
    IN VOID *pv);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVBACKUPFREE *pfnCertSrvBackupFree = CertSrvBackupFree;
#endif


//+--------------------------------------------------------------------------
// CertSrvRestoreGetDatabaseLocations -- called both at backup time as well as
//	at restorate time to get data base locations for different types of
//	files.
//
// Parameters:
//	[in]  hbc - backup context handle which would have been obtained
//		through CertSrvBackupPrepare in the backup case and through
//		CertSrvRestorePrepare in the restore case.
//	[out] ppwszzDatabaseLocationList - pointer that will receive the
//		pointer to the list of database locations; allocated memory
//		should be freed using CertSrvBackupFree() API by the caller
//		when it is no longer needed; locations are returned in an array
//		of null-terminated names and and the list is terminated by
//		two L'\0's.  The first character of each name is the BFT
//		character that indicates the type of the file and the rest of
//		the name tells gives the path into which that particular type
//		of file should be restored.
//	[out] pcbSize - will receive the number of bytes returned
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVRESTOREGETDATABASELOCATIONSW)(
    IN  HCSBC hbc,
    OUT WCHAR **ppwszzDatabaseLocationList,
    OUT DWORD *pcbSize);

HRESULT
CERTBCLI_API
CertSrvRestoreGetDatabaseLocationsW(
    _In_ IN  HCSBC hbc,
    _Outptr_ PWSTR *ppwszzDatabaseLocationList,
    _Out_ OUT DWORD *pcbSize);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVRESTOREGETDATABASELOCATIONSW *pfnCertSrvRestoreGetDatabaseLocations = CertSrvRestoreGetDatabaseLocations;
#endif


//+--------------------------------------------------------------------------
// CertSrvRestorePrepare -- indicate beginning of a restore session.
//
// Parameters:
//	[in]  pwszServerName - name or config string of the server into which
//		the restore operation is going to be performed.
//	[in]  dwRestoreFlags -  Or'ed combination of CSRESTORE_TYPE_* flags;
//		0 if no special flags are to be specified
//	[out] phbc - pointer to receive the backup context handle which is to
//		be passed to the subsequent restore APIs
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVRESTOREPREPAREW)(
    IN  WCHAR const *pwszServerName,
    IN  ULONG dwRestoreFlags,
    OUT HCSBC *phbc);

HRESULT
CERTBCLI_API
CertSrvRestorePrepareW(
    IN  WCHAR const *pwszServerName,
    IN  ULONG dwRestoreFlags,		// CSRESTORE_TYPE_*
    OUT HCSBC *phbc);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVRESTOREPREPAREW *pfnCertSrvRestorePrepare = CertSrvRestorePrepare;
#endif


//+--------------------------------------------------------------------------
// CertSrvRestoreRegister -- register a restore operation. It will interlock
//	all subsequent restore operations, and will prevent the restore target
//	from starting until the call to CertSrvRestoreRegisterComplete is made.
//
// Parameters:
//	[in] hbc - backup context handle for the restore session.
//	[in] pwszCheckPointFilePath - path to restore the check point files
//	[in] pwszLogPath - path where the log files are restored
//	[in] rgrstmap - restore map
//	[in] crstmap - tells if there is a new restore map
//	[in] pwszBackupLogPath - path where the backup logs are located
//	[in] genLow - Lowest log# that was restored in this restore session
//	[in] genHigh - Highest log# that was restored in this restore session
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVRESTOREREGISTERW)(
    OPTIONAL IN HCSBC hbc,
    OPTIONAL IN WCHAR const *pwszCheckPointFilePath,
    OPTIONAL IN WCHAR const *pwszLogPath,
    OPTIONAL IN CSEDB_RSTMAPW rgrstmap[],
    IN LONG crstmap,
    OPTIONAL IN WCHAR const *pwszBackupLogPath,
    IN ULONG genLow,
    IN ULONG genHigh);

HRESULT
CERTBCLI_API
CertSrvRestoreRegisterW(
    OPTIONAL IN HCSBC hbc,
    OPTIONAL IN WCHAR const *pwszCheckPointFilePath,
    OPTIONAL IN WCHAR const *pwszLogPath,
    OPTIONAL IN CSEDB_RSTMAPW rgrstmap[],
    IN LONG crstmap,
    OPTIONAL IN WCHAR const *pwszBackupLogPath,
    IN ULONG genLow,
    IN ULONG genHigh);

HRESULT
CERTBCLI_API
CertSrvRestoreRegisterThroughFile(
    IN HCSBC hbc,
    OPTIONAL IN WCHAR const *pwszCheckPointFilePath,
    OPTIONAL IN WCHAR const *pwszLogPath,
    OPTIONAL IN CSEDB_RSTMAPW rgrstmap[],
    IN LONG crstmap,
    OPTIONAL IN WCHAR const *pwszBackupLogPath,
    IN ULONG genLow,
    IN ULONG genHigh);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVRESTOREREGISTERW *pfnCertSrvRestoreRegister = CertSrvRestoreRegister;
#endif


//+--------------------------------------------------------------------------
// CertSrvRestoreRegisterComplete -- indicate that a previously registered
//	restore is complete.
//
// Parameters:
//	[in] hbc - backup context handle
//	[in] hrRestoreState - success code if the restore was successful
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVRESTOREREGISTERCOMPLETE)(
    OPTIONAL IN HCSBC hbc,
    IN HRESULT hrRestoreState);

HRESULT
CERTBCLI_API
CertSrvRestoreRegisterComplete(
    OPTIONAL IN HCSBC hbc,
    IN HRESULT hrRestoreState);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVRESTOREREGISTERCOMPLETE *pfnCertSrvRestoreRegisterComplete = CertSrvRestoreRegisterComplete;
#endif


//+--------------------------------------------------------------------------
// CertSrvRestoreEnd -- end a restore session
//
// Parameters:
//	[in] hbc - backup context handle
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

typedef HRESULT (CERTBCLI_CALL FNCERTSRVRESTOREEND)(
    IN HCSBC hbc);

HRESULT
CERTBCLI_API
CertSrvRestoreEnd(
    IN HCSBC hbc);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVRESTOREEND *pfnCertSrvRestoreEnd = CertSrvRestoreEnd;
#endif


//+--------------------------------------------------------------------------
// CertSrvServerControl -- send a control command to the cert server.
//
// Parameters:
//	[in]  pwszServerName - name or config string of the server to control
//	[in]  dwControlFlags - control command and flags
//	[out] pcbOut - pointer to receive the size of command output data
//	[out] ppbOut - pointer to receive command output data.  Use the
//		CertSrvBackupFree() API to free the buffer.
//
// Returns:
//	S_OK if the call executed successfully;
//	Failure code otherwise.
//---------------------------------------------------------------------------

#define CSCONTROL_SHUTDOWN	    0x000000001
#define CSCONTROL_SUSPEND	    0x000000002
#define CSCONTROL_RESTART	    0x000000003

typedef HRESULT (CERTBCLI_CALL FNCERTSRVSERVERCONTROLW)(
    IN WCHAR const *pwszServerName,
    IN DWORD dwControlFlags,
    OPTIONAL OUT DWORD *pcbOut,
    OPTIONAL OUT BYTE **ppbOut);

HRESULT
CERTBCLI_API
CertSrvServerControlW(
    IN WCHAR const *pwszServerName,
    IN DWORD dwControlFlags,
    OPTIONAL OUT DWORD *pcbOut,
    OPTIONAL OUT BYTE **ppbOut);

#ifdef _CERTBCLI_TYPECHECK
FNCERTSRVSERVERCONTROLW *pfnCertSrvServerControl = CertSrvServerControl;
#endif


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __CERTBCLI_H__
