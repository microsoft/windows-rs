/**@@@+++@@@@******************************************************************
**
** Microsoft (r) PlayReady (r)
** Copyright (c) Microsoft Corporation. All rights reserved.
**
***@@@---@@@@******************************************************************
*/

#ifndef __WINDOWS_MEDIA_PROTECTION_PLAYREADY_RESULTS_H_
#define __WINDOWS_MEDIA_PROTECTION_PLAYREADY_RESULTS_H_

/*
** This file contains all the DRM_RESULTS required for building Porting Kit
** applications.
**
** NOTE: DRM_RESULTS can be converted into Windows HRESULTS without any
**       changes. The FACILITY values below are carefully selected to allow
**       for this.
*/
#if MICROSOFT_PK_WINDOWS
/*
** NOTE: PC and Services error codes should ****NOT**** be added here,
**       but rather to the files discussed in the comments further down
**       regarding them.
**
*/
#endif /* MICROSOFT_PK_WINDOWS */



#define DRM_FAILED(Status) ((DRM_RESULT)(Status)<0)
#define DRM_SUCCEEDED(Status) ((DRM_RESULT)(Status) >= 0)

#define DRM_S_BASECODE                          0xC000
#define DRM_E_BASECODE                          0xC000
#define DRM_E_PK_BASECODE                       0xA000


/*
** !!!!!!  VERY IMPORTANT, PLEASE READ !!!!!
**
**  NOTE: The following ranges are reserved and should never be used to define PK error codes.
**  The range from 80040000 to 80040fff is reserved for Windows error codes.
**  The range from 80041000 to 80047fff is reserved for non-PK Microsoft error codes.
**  The range from 8004b000 to 8004bfff is reserved for Windows PC error codes.
**  The range from 8004e000 to 8004ffff is reserved for Windows PC error codes.
**  The range from 8004c600 to 8004c6ff is reserved for Server and Services error codes.
**  The range from 8004dc80 to 8004ddff is reserved for OEM-defined PK error codes.
**
**  NOTE: The following ranges are OK to be used for PK errors.
**  The range from 80048000 to 8004afff is reserved for PK error codes.
**  The range from 8004c000 to 8004c5ff is reserved for PK error codes.
**  The range from 8004c700 to 8004dc7f is reserved for PK error codes.
**  The range from 8004de00 to 8004dfff is reserved for PK error codes.
*/
#if MICROSOFT_PK_WINDOWS
/*
**  There's a range of error codes that is only applicable to the PC and should
**  not be included here. These errors are in common\include, files:
**  msprerr.h
**  drmndbridgeerr.h
**  errorservice.h
**  To prevent overlaps of PK and PC error codes, please assume that:
**  The range from 0x8004B000 to 0x8004BEFF is reserved for the PC (error codes).
**  The range from 0x0004B000 to 0x0004BEFF is reserved for the PC (success codes).
**  The range from 0x8004BE00 to 0x8004BFFF is reserved for the Modern SDK (error codes).
**  The range from 0x0004BE00 to 0x0004BFFF is reserved for the Modern SDK (success codes).
**  The following three defines represent the basecodes for the PC.
*/
#define DRM_E_NDBRIDGEONLY_BASECODE     0xB000
#define DRM_E_MSPRSDK_BASECODE          0xB800
#define DRM_E_MODERN_BASECODE           0xBE00
#endif /* MICROSOFT_PK_WINDOWS */

#define DRM_FACILITY_ITF DRM_FACILITY_CORE

#define MAKE_DRM_RESULT(sev,fac,code) \
    ((DRM_RESULT) (((unsigned long)(sev)<<30) | ((unsigned long)(fac)<<16) | ((unsigned long)(code))) )

/* ============================================================
**
** Standard Success values
**
** ============================================================
*/

/*
 *  Values are 32 bit values laid out as follows:
 *
 *   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
 *   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
 *  +-+-+-+-+-+---------------------+-------------------------------+
 *  |S|R|C|N|r|    Facility         |               Code            |
 *  +-+-+-+-+-+---------------------+-------------------------------+
 *
 *  where
 *
 *      S - Severity - indicates success/fail
 *
 *          0 - Success
 *          1 - Fail (COERROR)
 *
 *      R - reserved portion of the facility code, corresponds to NT's
 *              second severity bit.
 *
 *      C - reserved portion of the facility code, corresponds to NT's
 *              C field.
 *
 *      N - reserved portion of the facility code. Used to indicate a
 *              mapped NT status value.
 *
 *      r - reserved portion of the facility code. Reserved for internal
 *              use. Used to indicate HRESULT values that are not status
 *              values, but are instead message ids for display strings.
 *
 *      Facility - is the facility code
 *
 *      Code - is the facility's status code
 *
 *
 * Define the facility codes
 *
 */
#define DRM_FACILITY_STANDARD            0x0
#define DRM_FACILITY_COM_STORAGE         0x3
#define DRM_FACILITY_CORE                0x4
#define DRM_FACILITY_WIN32               0x7
#define DRM_FACILITY_NETSHOW             0xD


/*
 * Define the severity codes
 *
 */
#define DRM_SEVERITY_SUCCESS             0x0
#define DRM_SEVERITY_ERROR               0x2
#define DRM_SEVERITY_NETSHOW_ERROR       0x3


/*
 * MessageId: DRM_SUCCESS
 *
 * MessageText:
 *
 * Operation was successful.
 *
 */
#define DRM_SUCCESS                      ((DRM_RESULT)0x00000000L)

/*
 * MessageId: DRM_S_FALSE
 *
 * MessageText:
 *
 * Operation was successful, but returned a FALSE test condition.
 *
 */
#define DRM_S_FALSE                      ((DRM_RESULT)0x00000001L)

/*
 * MessageId: DRM_S_MORE_DATA
 *
 * MessageText:
 *
 * Operation was successful, but more data is available.
 *
 */
#define DRM_S_MORE_DATA                  ((DRM_RESULT)0x00000002L)



/* ============================================================
**
** Standard error messages (0x8000xxxx)
**
** ============================================================
*/

/*
 * MessageId: DRM_E_OUTOFMEMORY
 *
 * MessageText:
 *
 * Insufficient resources exist to complete the request.
 *
 */
#define DRM_E_OUTOFMEMORY                ((DRM_RESULT)0x80000002L)

/*
 * MessageId: DRM_E_NOTIMPL
 *
 * MessageText:
 *
 * The requested operation is not implemented.
 *
 */
#define DRM_E_NOTIMPL                    ((DRM_RESULT)0x80004001L)

/*
 * MessageId: DRM_E_POINTER
 *
 * MessageText:
 *
 * Invalid pointer.
 *
 */
#define DRM_E_POINTER                    ((DRM_RESULT)0x80004003L)

/*
 * MessageId: DRM_E_FAIL
 *
 * MessageText:
 *
 * The requested operation failed.
 *
 */
#define DRM_E_FAIL                       ((DRM_RESULT)0x80004005L)


/* ============================================================
**
** Error messages shared with Win32 (0x8007xxxx)
**
** ============================================================
*/

/*
 * MessageId: DRM_E_WIN32_FILE_NOT_FOUND
 *
 * MessageText:
 *
 * The system cannot find the file specified.
 *
 */
#define DRM_E_WIN32_FILE_NOT_FOUND       ((DRM_RESULT)0x80070002L)

/*
 * MessageId: DRM_E_HANDLE
 *
 * MessageText:
 *
 * Invalid handle.
 *
 */
#define DRM_E_HANDLE                     ((DRM_RESULT)0x80070006L)

/*
 * MessageId: DRM_E_WIN32_NO_MORE_FILES
 *
 * MessageText:
 *
 * There are no more files.
 *
 */
#define DRM_E_WIN32_NO_MORE_FILES        ((DRM_RESULT)0x80070012L)

/*
 * MessageId: DRM_E_INVALIDARG
 *
 * MessageText:
 *
 * The parameter is incorrect.
 *
 */
#define DRM_E_INVALIDARG                 ((DRM_RESULT)0x80070057L)

/*
 * MessageId: DRM_E_BUFFERTOOSMALL
 *
 * MessageText:
 *
 * The data area passed to a function is too small.
 *
 */
#define DRM_E_BUFFERTOOSMALL             ((DRM_RESULT)0x8007007AL)

/*
 * MessageId: DRM_E_NOMORE
 *
 * MessageText:
 *
 * No more data is available.
 *
 */
#define DRM_E_NOMORE                     ((DRM_RESULT)0x80070103L)

/*
 * MessageId: DRM_E_ARITHMETIC_OVERFLOW
 *
 * MessageText:
 *
 * Arithmetic result exceeded maximum value.
 *
 */
#define DRM_E_ARITHMETIC_OVERFLOW        ((DRM_RESULT)0x80070216L)

/*
 * MessageId: DRM_E_NOT_FOUND
 *
 * MessageText:
 *
 * Element not found.
 *
 */
#define DRM_E_NOT_FOUND                  ((DRM_RESULT)0x80070490L)

/*
 * MessageId: DRM_E_INVALID_COMMAND_LINE
 *
 * MessageText:
 *
 * Invalid command line argument.
 *
 */
#define DRM_E_INVALID_COMMAND_LINE       ((DRM_RESULT)0x80070667L)


/* ============================================================
**
** Error messages shared with COM Storage (mostly file errors)
** (0x8003xxxx)
**
** ============================================================
*/

/*
 * MessageId: DRM_E_FILENOTFOUND
 *
 * MessageText:
 *
 * A requested file could not be found.
 *
 */
#define DRM_E_FILENOTFOUND               ((DRM_RESULT)0x80030002L)

/*
 * MessageId: DRM_E_FILEOPEN
 *
 * MessageText:
 *
 * A request failed due to a file being open.
 *
 */
#define DRM_E_FILEOPEN                   ((DRM_RESULT)0x8003006EL)


/* ============================================================
**
** NetShow Errors from NsError.h (0xc00Dxxxx)
**
** ============================================================
*/

/*
 * MessageId: DRM_E_PARAMETERS_MISMATCHED
 *
 * MessageText:
 *
 * A problem has occurred in the Digital Rights Management component.
 *
 */
#define DRM_E_PARAMETERS_MISMATCHED      ((DRM_RESULT)0xC00D272FL)

/*
 * MessageId: DRM_E_FAILED_TO_STORE_LICENSE
 *
 * MessageText:
 *
 * License storage is not working.
 *
 */
#define DRM_E_FAILED_TO_STORE_LICENSE    ((DRM_RESULT)0xC00D2712L)

/*
 * MessageId: DRM_E_NOT_ALL_STORED
 *
 * MessageText:
 *
 * Some of the licenses could not be stored.
 *
 */
#define DRM_E_NOT_ALL_STORED             ((DRM_RESULT)0xC00D275FL)


/* ============================================================
**
** Vista crypto errors, 0x80040e80-0x80040e8f.
**
** ============================================================
*/

/*
 * MessageId: DRM_E_VERIFICATION_FAILURE
 *
 * MessageText:
 *
 * Validation of a Longhorn certificate failed.
 *
 */
#define DRM_E_VERIFICATION_FAILURE       ((DRM_RESULT)0x80040E80L)

/*
 * MessageId: DRM_E_RSA_SIGNATURE_ERROR
 *
 * MessageText:
 *
 * Error in RSA(PSS) signature.
 *
 */
#define DRM_E_RSA_SIGNATURE_ERROR        ((DRM_RESULT)0x80040E82L)

/*
 * MessageId: DRM_E_BAD_RSA_EXPONENT
 *
 * MessageText:
 *
 * An incorrect RSA exponent was supplied for a public key.
 *
 */
#define DRM_E_BAD_RSA_EXPONENT           ((DRM_RESULT)0x80040E86L)

/*
 * MessageId: DRM_E_P256_CONVERSION_FAILURE
 *
 * MessageText:
 *
 * An error occurred while converting between P256 types.
 *
 */
#define DRM_E_P256_CONVERSION_FAILURE    ((DRM_RESULT)0x80040E87L)

/*
 * MessageId: DRM_E_P256_PKCRYPTO_FAILURE
 *
 * MessageText:
 *
 * An error occurred in an asymmetric P256 cryptographic operation.
 *
 */
#define DRM_E_P256_PKCRYPTO_FAILURE      ((DRM_RESULT)0x80040E88L)

/*
 * MessageId: DRM_E_P256_PLAINTEXT_MAPPING_FAILURE
 *
 * MessageText:
 *
 * An error occurred while attempting to map a plaintext array to a EC Point: There is no conversion for this byte array to a EC Point.
 *
 */
#define DRM_E_P256_PLAINTEXT_MAPPING_FAILURE ((DRM_RESULT)0x80040E89L)

/*
 * MessageId: DRM_E_P256_INVALID_SIGNATURE
 *
 * MessageText:
 *
 * The ECDSA signature to be verified was not a valid signature format.
 *
 */
#define DRM_E_P256_INVALID_SIGNATURE     ((DRM_RESULT)0x80040E8AL)

/*
 * MessageId: DRM_E_P256_ECDSA_VERIFICATION_ERROR
 *
 * MessageText:
 *
 * The ECDSA verification algorithm encountered an unknown error.
 *
 */
#define DRM_E_P256_ECDSA_VERIFICATION_ERROR ((DRM_RESULT)0x80040E8BL)

/*
 * MessageId: DRM_E_P256_ECDSA_SIGNING_ERROR
 *
 * MessageText:
 *
 * The ECDSA signature algorithm encountered an unknown error.
 *
 */
#define DRM_E_P256_ECDSA_SIGNING_ERROR   ((DRM_RESULT)0x80040E8CL)

/*
 * MessageId: DRM_E_P256_HMAC_KEYGEN_FAILURE
 *
 * MessageText:
 *
 * Could not generate a valid HMAC key under constraint where CK || HMACK is a valid x coord on the EC (P256).
 *
 */
#define DRM_E_P256_HMAC_KEYGEN_FAILURE   ((DRM_RESULT)0x80040E8DL)


/* ============================================================
**
** IContentHeader errors: error codes from DRM_E_CH_BASECODE+0
** to DRM_E_CH_BASECODE+0x7F, 0x80041100-0x8004117f.
**
** ============================================================
*/

#define DRM_E_CH_BASECODE                ((DRM_RESULT)0x80041100L)

/*
 * MessageId: DRM_E_CH_VERSION_MISSING
 *
 * MessageText:
 *
 * Missing content header version.
 *
 */
#define DRM_E_CH_VERSION_MISSING         ((DRM_RESULT)0x80041103L)

/*
 * MessageId: DRM_E_CH_KID_MISSING
 *
 * MessageText:
 *
 * Missing KID attribute in content header.
 *
 */
#define DRM_E_CH_KID_MISSING             ((DRM_RESULT)0x80041104L)

/*
 * MessageId: DRM_E_CH_LAINFO_MISSING
 *
 * MessageText:
 *
 * Missing LAINFO attribute in content header.
 *
 */
#define DRM_E_CH_LAINFO_MISSING          ((DRM_RESULT)0x80041105L)

/*
 * MessageId: DRM_E_CH_CHECKSUM_MISSING
 *
 * MessageText:
 *
 * Missing content header checksum.
 *
 */
#define DRM_E_CH_CHECKSUM_MISSING        ((DRM_RESULT)0x80041106L)

/*
 * MessageId: DRM_E_CH_INVALID_HEADER
 *
 * MessageText:
 *
 * Invalid content header.
 *
 */
#define DRM_E_CH_INVALID_HEADER          ((DRM_RESULT)0x80041108L)

/*
 * MessageId: DRM_E_CH_INVALID_CHECKSUM
 *
 * MessageText:
 *
 * Invalid checksum in the header.
 *
 */
#define DRM_E_CH_INVALID_CHECKSUM        ((DRM_RESULT)0x80041109L)

/*
 * MessageId: DRM_E_CH_UNSUPPORTED_VERSION
 *
 * MessageText:
 *
 * Unsupported content header version.
 *
 */
#define DRM_E_CH_UNSUPPORTED_VERSION     ((DRM_RESULT)0x8004110BL)

/*
 * MessageId: DRM_E_CH_BAD_KEY
 *
 * MessageText:
 *
 * Invalid key.
 *
 */
#define DRM_E_CH_BAD_KEY                 ((DRM_RESULT)0x8004110EL)

/*
 * MessageId: DRM_E_CH_INCOMPATIBLE_HEADER_TYPE
 *
 * MessageText:
 *
 * Incompatible content header type.
 *
 */
#define DRM_E_CH_INCOMPATIBLE_HEADER_TYPE ((DRM_RESULT)0x8004110FL)

/*
 * MessageId: DRM_E_HEADER_ALREADY_SET
 *
 * MessageText:
 *
 * Content header type is already set. Reinitialize is required.
 *
 */
#define DRM_E_HEADER_ALREADY_SET         ((DRM_RESULT)0x80041110L)

/*
 * MessageId: DRM_E_CH_MULTIPLE_KIDS
 *
 * MessageText:
 *
 * Content header includes multiple KIDs.  The operation requested is unsupported.
 *
 */
#define DRM_E_CH_MULTIPLE_KIDS           ((DRM_RESULT)0x80041111L)


/* ============================================================
**
** CD Migration Tool errors: error codes from DRM_E_CH_BASECODE+0x80
** to DRM_E_CH_BASECODE+0xFF, 0x80041180-0x800411ff.
**
** ============================================================
*/

#define DRM_E_CDMIGRATIONTOOL_BASECODE                ((DRM_RESULT)0x80041180L)
#define DRM_E_CDMIGRATIONTOOL_MAXCODE                 ((DRM_RESULT)0x800411FFL)

/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_INVALID_FILE
 *
 * MessageText:
 *
 * File cannot be migrated because it is invalid.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_INVALID_FILE ((DRM_RESULT)0x80041180L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_FILE_IS_NOT_CD_RIPPED
 *
 * MessageText:
 *
 * File cannot be migrated because it was not ripped from CD.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_FILE_IS_NOT_CD_RIPPED ((DRM_RESULT)0x80041181L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_FILE_IS_NOT_PROTECTED
 *
 * MessageText:
 *
 * File cannot be migrated because it is not protected.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_FILE_IS_NOT_PROTECTED ((DRM_RESULT)0x80041182L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_LICENSE_KID_INVALID
 *
 * MessageText:
 *
 * File cannot be migrated because the server returned a license with an invalid KID.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_LICENSE_KID_INVALID ((DRM_RESULT)0x80041183L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_LICENSE_KID_MISMATCH
 *
 * MessageText:
 *
 * File cannot be migrated because the server returned a license with a KID that did not match the content.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_LICENSE_KID_MISMATCH ((DRM_RESULT)0x80041184L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_LICENSE_CONTENT_KEY_INVALID
 *
 * MessageText:
 *
 * File cannot be migrated because the server returned a license with an invalid content key.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_LICENSE_CONTENT_KEY_INVALID ((DRM_RESULT)0x80041185L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_INVALID_ASF_FORMAT
 *
 * MessageText:
 *
 * File cannot be migrated because the ASF is corrupt.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_INVALID_ASF_FORMAT ((DRM_RESULT)0x80041186L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_INVALID_ASF_PACKETS
 *
 * MessageText:
 *
 * File cannot be migrated because the ASF packets are corrupt.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_INVALID_ASF_PACKETS ((DRM_RESULT)0x80041187L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_CONTENT_KEY_CACHE_CORRUPT
 *
 * MessageText:
 *
 * File cannot be migrated because the content key obtained from the local cache is invalid.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_CONTENT_KEY_CACHE_CORRUPT ((DRM_RESULT)0x80041188L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_FILE_WRITE_ERROR
 *
 * MessageText:
 *
 * File cannot be migrated because the file could not be written.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_FILE_WRITE_ERROR ((DRM_RESULT)0x80041189L)


/*
 * MessageId: DRM_E_CDMIGRATIONTOOL_CANCELLED
 *
 * MessageText:
 *
 * File migration was cancelled.
 *
 */
#define DRM_E_CDMIGRATIONTOOL_CANCELLED  ((DRM_RESULT)0x8004118AL)


/* ============================================================
**
** License parsing results: error codes from 0x80041200-0x800412ff.
**
** ============================================================
*/

#define DRM_E_LIC_BASECODE           ((DRM_RESULT)0x80041200L)

/*
 * MessageId: DRM_E_LIC_UNSUPPORTED_VALUE
 *
 * MessageText:
 *
 * Unsupported value in license.
 *
 */
#define DRM_E_LIC_UNSUPPORTED_VALUE      ((DRM_RESULT)0x80041207L)


/* ============================================================
**
** CDMI: error codes from 0x80041300-0x8004137f.
**
** ============================================================
*/

#define DRM_E_CDMI_BASECODE          ((DRM_RESULT)0x80041300L)

/*
 * MessageId: DRM_E_CDMI_INVALID_INITIALIZATION_DATA
 *
 * MessageText:
 *
 * Invalid initialization data.
 *
 */
#define DRM_E_CDMI_INVALID_INITIALIZATION_DATA ((DRM_RESULT)0x80041301L)

/*
 * MessageId: DRM_E_CDMI_PERSISTENT_LICENSE_FOR_NON_PERSISTENT_LICENSE_SESSION
 *
 * MessageText:
 *
 * A persistent license was provided for a session that was not persistent-license.
 *
 */
#define DRM_E_CDMI_PERSISTENT_LICENSE_FOR_NON_PERSISTENT_LICENSE_SESSION ((DRM_RESULT)0x80041302L)

/*
 * MessageId: DRM_E_CDMI_SECURE_STOP_LICENSE_FOR_NON_PERSISTENT_USAGE_RECORD_SESSION
 *
 * MessageText:
 *
 * A secure stop license was provided for a session that was not persistent-usage-record.
 *
 */
#define DRM_E_CDMI_SECURE_STOP_LICENSE_FOR_NON_PERSISTENT_USAGE_RECORD_SESSION ((DRM_RESULT)0x80041303L)

/*
 * MessageId: DRM_E_CDMI_TEMPORARY_LICENSE_FOR_NON_TEMPORARY_SESSION
 *
 * MessageText:
 *
 * An in-memory-only license without secure-stop was provided for a session that was not temporary.
 *
 */
#define DRM_E_CDMI_TEMPORARY_LICENSE_FOR_NON_TEMPORARY_SESSION ((DRM_RESULT)0x80041304L)

/*
 * MessageId: DRM_E_CDMI_UNSUPPORTED_KEY_SYSTEM
 *
 * MessageText:
 *
 * The requested key system is not supported.
 *
 */
#define DRM_E_CDMI_UNSUPPORTED_KEY_SYSTEM ((DRM_RESULT)0x80041305L)

/*
 * MessageId: DRM_E_CDMI_UNSUPPORTED_INITIALIZATION_DATA_TYPES
 *
 * MessageText:
 *
 * None of the requested initialization data types are supported.
 *
 */
#define DRM_E_CDMI_UNSUPPORTED_INITIALIZATION_DATA_TYPES ((DRM_RESULT)0x80041306L)

/*
 * MessageId: DRM_E_CDMI_UNSUPPORTED_DISTINCTIVE_IDENTIFIER
 *
 * MessageText:
 *
 * The requested distinctive identifier setting is not supported.
 *
 */
#define DRM_E_CDMI_UNSUPPORTED_DISTINCTIVE_IDENTIFIER ((DRM_RESULT)0x80041307L)

/*
 * MessageId: DRM_E_CDMI_UNSUPPORTED_SESSION_TYPE
 *
 * MessageText:
 *
 * The requested session type is not supported.
 *
 */
#define DRM_E_CDMI_UNSUPPORTED_SESSION_TYPE ((DRM_RESULT)0x80041308L)

/*
 * MessageId: DRM_E_CDMI_UNSUPPORTED_INITIALIZATION_DATA
 *
 * MessageText:
 *
 * The provided initialization data is not supported.
 *
 */
#define DRM_E_CDMI_UNSUPPORTED_INITIALIZATION_DATA ((DRM_RESULT)0x80041309L)

/*
 * MessageId: DRM_E_CDMI_SESSION_ALREADY_USED
 *
 * MessageText:
 *
 * The session has already been used.
 *
 */
#define DRM_E_CDMI_SESSION_ALREADY_USED  ((DRM_RESULT)0x8004130AL)

/*
 * MessageId: DRM_E_CDMI_SESSION_UNINITIALIZED
 *
 * MessageText:
 *
 * The session is not yet initialized.
 *
 */
#define DRM_E_CDMI_SESSION_UNINITIALIZED ((DRM_RESULT)0x8004130BL)

/*
 * MessageId: DRM_E_CDMI_SESSION_CLOSED
 *
 * MessageText:
 *
 * The session is closed.
 *
 */
#define DRM_E_CDMI_SESSION_CLOSED        ((DRM_RESULT)0x8004130CL)

/*
 * MessageId: DRM_E_CDMI_SESSION_ID_NOT_FOUND
 *
 * MessageText:
 *
 * The given session ID could not be found.
 *
 */
#define DRM_E_CDMI_SESSION_ID_NOT_FOUND  ((DRM_RESULT)0x8004130DL)

/*
 * MessageId: DRM_E_CDMI_SESSION_TYPE_MISMATCH
 *
 * MessageText:
 *
 * The given session was initialized with a different session type than the session being loaded or Load/Remove was called on a temporary session.
 *
 */
#define DRM_E_CDMI_SESSION_TYPE_MISMATCH ((DRM_RESULT)0x8004130EL)

/* ============================================================
**
** Legacy errors: error codes from 0x80048000-0x800480ff.
**
** ============================================================
*/

#define DRM_E_LEGACY_BASECODE                ((DRM_RESULT)0x80048000L)

/*
 * MessageId: DRM_E_LIC_KEY_DECODE_FAILURE
 *
 * MessageText:
 *
 * Key decode failure.
 *
 */
#define DRM_E_LIC_KEY_DECODE_FAILURE     ((DRM_RESULT)0x80048007L)

/*
 * MessageId: DRM_E_KEY_MISMATCH
 *
 * MessageText:
 *
 * A public/private keypair is mismatched.
 *
 */
#define DRM_E_KEY_MISMATCH               ((DRM_RESULT)0x80048014L)

/*
 * MessageId: DRM_E_INVALID_SIGNATURE
 *
 * MessageText:
 *
 * License signature failure.
 *
 */
#define DRM_E_INVALID_SIGNATURE          ((DRM_RESULT)0x800480CFL)

/*
 * MessageId: DRM_E_CIPHER_NOT_INITIALIZED
 *
 * MessageText:
 *
 * The DRM Cipher routines were not correctly initialized before calling encryption/decryption routines.
 *
 */
#define DRM_E_CIPHER_NOT_INITIALIZED     ((DRM_RESULT)0x800480D2L)

/*
 * MessageId: DRM_E_DECRYPT_NOT_INITIALIZED
 *
 * MessageText:
 *
 * The DRM decrypt routines were not correctly initialized before trying to decrypt data.
 *
 */
#define DRM_E_DECRYPT_NOT_INITIALIZED    ((DRM_RESULT)0x800480D3L)

/*
 * MessageId: DRM_E_SECURESTORE_LOCK_NOT_OBTAINED
 *
 * MessageText:
 *
 * Before reading or writing data to securestore in raw mode, first the lock must be obtained using DRM_SST_OpenData.
 *
 */
#define DRM_E_SECURESTORE_LOCK_NOT_OBTAINED ((DRM_RESULT)0x800480D4L)

/*
 * MessageId: DRM_E_PKCRYPTO_FAILURE
 *
 * MessageText:
 *
 * An error occurred in an asymmetric cryptographic operation.
 *
 */
#define DRM_E_PKCRYPTO_FAILURE           ((DRM_RESULT)0x800480D5L)

/*
 * MessageId: DRM_E_INVALID_DST_SLOT_SIZE
 *
 * MessageText:
 *
 * Invalid DST slot size is specified.
 *
 */
#define DRM_E_INVALID_DST_SLOT_SIZE      ((DRM_RESULT)0x800480D6L)


/* ============================================================
**
** DRM utility results: error codes from 0x80049000-0x800490ff.
**
** ============================================================
*/

#define DRMUTIL_BASECODE       ((DRM_RESULT)0x80049000L)

/*
 * MessageId: DRM_E_UNSUPPORTED_VERSION
 *
 * MessageText:
 *
 * A version string is malformed.
 *
 */
#define DRM_E_UNSUPPORTED_VERSION        ((DRM_RESULT)0x80049005L)


/* ============================================================
**
** PK specific errors (from 0x8004a000 to 0x8004afff)
**
** ============================================================
*/

/*
 * MessageId: DRM_E_REVOCATION_GUID_NOT_RECOGNIZED
 *
 * MessageText:
 *
 * The revocation list type GUID was not recognized
 *
 */
#define DRM_E_REVOCATION_GUID_NOT_RECOGNIZED ((DRM_RESULT)0x8004A002L)

/*
 * MessageId: DRM_E_LIC_CHAIN_TOO_DEEP
 *
 * MessageText:
 *
 * The license chained deeper than this implementation can handle
 *
 */
#define DRM_E_LIC_CHAIN_TOO_DEEP         ((DRM_RESULT)0x8004A003L)

/*
 * MessageId: DRM_E_DEVICE_SECURITY_LEVEL_TOO_LOW
 *
 * MessageText:
 *
 * The security level of the remote device is too low to receive the license
 *
 */
#define DRM_E_DEVICE_SECURITY_LEVEL_TOO_LOW ((DRM_RESULT)0x8004A004L)

/*
 * MessageId: DRM_E_DST_BLOCK_CACHE_CORRUPT
 *
 * MessageText:
 *
 * The block header cache returned invalid data
 *
 */
#define DRM_E_DST_BLOCK_CACHE_CORRUPT    ((DRM_RESULT)0x8004A005L)

/*
 * MessageId: DRM_E_DST_BLOCK_CACHE_MISS
 *
 * MessageText:
 *
 * The block header cache didn't contain the requested block header
 *
 */
#define DRM_E_DST_BLOCK_CACHE_MISS       ((DRM_RESULT)0x8004A007L)

/*
 * MessageId: DRM_E_INVALID_METERRESPONSE_SIGNATURE
 *
 * MessageText:
 *
 * Invalid signature in meter response
 *
 */
#define DRM_E_INVALID_METERRESPONSE_SIGNATURE ((DRM_RESULT)0x8004A013L)

/*
 * MessageId: DRM_E_METERSTORE_DATA_NOT_FOUND
 *
 * MessageText:
 *
 * Metering data slot not found due to bad data in response file
 *
 */
#define DRM_E_METERSTORE_DATA_NOT_FOUND  ((DRM_RESULT)0x8004A016L)

/*
 * MessageId: DRM_E_INVALID_REVOCATION_LIST
 *
 * MessageText:
 *
 * The revocation list version does not match the current revocation version
 *
 */
#define DRM_E_INVALID_REVOCATION_LIST    ((DRM_RESULT)0x8004A018L)

/*
 * MessageId: DRM_E_ENVELOPE_CORRUPT
 *
 * MessageText:
 *
 * The envelope archive or file is corrupt
 *
 */
#define DRM_E_ENVELOPE_CORRUPT           ((DRM_RESULT)0x8004A019L)

/*
 * MessageId: DRM_E_ENVELOPE_FILE_NOT_COMPATIBLE
 *
 * MessageText:
 *
 * The envelope file is not compatible with this version of the porting kit
 *
 */
#define DRM_E_ENVELOPE_FILE_NOT_COMPATIBLE ((DRM_RESULT)0x8004A01AL)

/*
 * MessageId: DRM_E_EXTENDED_RESTRICTION_NOT_UNDERSTOOD
 *
 * MessageText:
 *
 * An extensible restriction was not understood by the app, and is mark as being required
 *
 */
#define DRM_E_EXTENDED_RESTRICTION_NOT_UNDERSTOOD ((DRM_RESULT)0x8004A01BL)

/*
 * MessageId: DRM_E_OUTDATED_REVOCATION_LIST
 *
 * MessageText:
 *
 * The revocation list is outdated. It is required for the revocation list to be refreshed at least every 90 days.
 *
 */
#define DRM_E_OUTDATED_REVOCATION_LIST   ((DRM_RESULT)0x8004A01EL)


/* ============================================================
**
** Drm Core errors (from 0x8004c000 to 0x8004dfff)
**
** ============================================================
*/

/*
 * MessageId: DRM_E_DEVICE_NOT_INITIALIZED
 *
 * MessageText:
 *
 * This device has not been initialized against a DRM init service
 *
 */
#define DRM_E_DEVICE_NOT_INITIALIZED     ((DRM_RESULT)0x8004C001L)

/*
 * MessageId: DRM_E_DRM_NOT_INITIALIZED
 *
 * MessageText:
 *
 * The app has not call DRM_Init properly
 *
 */
#define DRM_E_DRM_NOT_INITIALIZED        ((DRM_RESULT)0x8004C002L)

/*
 * MessageId: DRM_E_INVALID_LICENSE
 *
 * MessageText:
 *
 * The license is invalid
 *
 */
#define DRM_E_INVALID_LICENSE            ((DRM_RESULT)0x8004C006L)

/*
 * MessageId: DRM_E_LICENSE_EXPIRED
 *
 * MessageText:
 *
 * The license has expired either by depleting a play count or via an end time.
 *
 */
#define DRM_E_LICENSE_EXPIRED            ((DRM_RESULT)0x8004C009L)

/*
 * MessageId: DRM_E_RIGHTS_NOT_AVAILABLE
 *
 * MessageText:
 *
 * The rights the app has requested are not available in the license
 *
 */
#define DRM_E_RIGHTS_NOT_AVAILABLE       ((DRM_RESULT)0x8004C00BL)

/*
 * MessageId: DRM_E_WRONG_TOKEN_TYPE
 *
 * MessageText:
 *
 * The token parameter was of an incompatible type.
 *
 */
#define DRM_E_WRONG_TOKEN_TYPE           ((DRM_RESULT)0x8004C00DL)

/*
 * MessageId: DRM_E_LICENSE_NOT_BOUND
 *
 * MessageText:
 *
 * A license has not been bound to. Decrypt can not happen without a successful bind call
 *
 */
#define DRM_E_LICENSE_NOT_BOUND          ((DRM_RESULT)0x8004C00FL)

/*
 * MessageId: DRM_E_HASH_MISMATCH
 *
 * MessageText:
 *
 * A Keyed Hash check failed.
 *
 */
#define DRM_E_HASH_MISMATCH              ((DRM_RESULT)0x8004C010L)

/*
 * MessageId: DRM_E_LICENSE_NOT_FOUND
 *
 * MessageText:
 *
 * A license was not found in the license store.
 *
 */
#define DRM_E_LICENSE_NOT_FOUND          ((DRM_RESULT)0x8004C013L)

/*
 * MessageId: DRM_E_LICENSE_VERSION_NOT_SUPPORTED
 *
 * MessageText:
 *
 * The DRM license version is not supported by the DRM version on the device.
 *
 */
#define DRM_E_LICENSE_VERSION_NOT_SUPPORTED ((DRM_RESULT)0x8004C014L)

/*
 * MessageId: DRM_E_UNSUPPORTED_ALGORITHM
 *
 * MessageText:
 *
 * The encryption algorithm required for this operation is not supported.
 *
 */
#define DRM_E_UNSUPPORTED_ALGORITHM      ((DRM_RESULT)0x8004C016L)

/*
 * MessageId: DRM_E_INVALID_LICENSE_STORE
 *
 * MessageText:
 *
 * The license store version number is incorrect, or the store is invalid in some other way.
 *
 */
#define DRM_E_INVALID_LICENSE_STORE      ((DRM_RESULT)0x8004C019L)

/*
 * MessageId: DRM_E_FILE_READ_ERROR
 *
 * MessageText:
 *
 * There was an error reading a file.
 *
 */
#define DRM_E_FILE_READ_ERROR            ((DRM_RESULT)0x8004C01AL)

/*
 * MessageId: DRM_E_FILE_WRITE_ERROR
 *
 * MessageText:
 *
 * There was an error writing a file.
 *
 */
#define DRM_E_FILE_WRITE_ERROR           ((DRM_RESULT)0x8004C01BL)

/*
 * MessageId: DRM_E_DST_STORE_FULL
 *
 * MessageText:
 *
 * The data store is full.
 *
 */
#define DRM_E_DST_STORE_FULL             ((DRM_RESULT)0x8004C01DL)

/*
 * MessageId: DRM_E_NO_XML_OPEN_TAG
 *
 * MessageText:
 *
 * XML open tag not found
 *
 */
#define DRM_E_NO_XML_OPEN_TAG            ((DRM_RESULT)0x8004C01EL)

/*
 * MessageId: DRM_E_NO_XML_CLOSE_TAG
 *
 * MessageText:
 *
 * XML close tag not found
 *
 */
#define DRM_E_NO_XML_CLOSE_TAG           ((DRM_RESULT)0x8004C01FL)

/*
 * MessageId: DRM_E_INVALID_XML_TAG
 *
 * MessageText:
 *
 * Invalid XML tag
 *
 */
#define DRM_E_INVALID_XML_TAG            ((DRM_RESULT)0x8004C020L)

/*
 * MessageId: DRM_E_NO_XML_CDATA
 *
 * MessageText:
 *
 * No XML CDATA found
 *
 */
#define DRM_E_NO_XML_CDATA               ((DRM_RESULT)0x8004C021L)

/*
 * MessageId: DRM_E_DST_NAMESPACE_NOT_FOUND
 *
 * MessageText:
 *
 * No DST Namespace found
 *
 */
#define DRM_E_DST_NAMESPACE_NOT_FOUND    ((DRM_RESULT)0x8004C023L)

/*
 * MessageId: DRM_E_DST_SLOT_NOT_FOUND
 *
 * MessageText:
 *
 * DST Dataslot not found
 *
 */
#define DRM_E_DST_SLOT_NOT_FOUND         ((DRM_RESULT)0x8004C024L)

/*
 * MessageId: DRM_E_DST_SLOT_EXISTS
 *
 * MessageText:
 *
 * DST Dataslot already exists
 *
 */
#define DRM_E_DST_SLOT_EXISTS            ((DRM_RESULT)0x8004C025L)

/*
 * MessageId: DRM_E_DST_CORRUPTED
 *
 * MessageText:
 *
 * The data store is corrupted
 *
 */
#define DRM_E_DST_CORRUPTED              ((DRM_RESULT)0x8004C026L)

/*
 * MessageId: DRM_E_DST_SEEK_ERROR
 *
 * MessageText:
 *
 * There was an error attempting to seek in the Data Store
 *
 */
#define DRM_E_DST_SEEK_ERROR             ((DRM_RESULT)0x8004C027L)

/*
 * MessageId: DRM_E_INVALID_SECURESTORE_PASSWORD
 *
 * MessageText:
 *
 * The password used to open the secure store key was not able to validate the secure store hash.
 *
 */
#define DRM_E_INVALID_SECURESTORE_PASSWORD ((DRM_RESULT)0x8004C029L)

/*
 * MessageId: DRM_E_SECURESTORE_CORRUPT
 *
 * MessageText:
 *
 * The secure store is corrupt
 *
 */
#define DRM_E_SECURESTORE_CORRUPT        ((DRM_RESULT)0x8004C02AL)

/*
 * MessageId: DRM_E_SECURESTORE_FULL
 *
 * MessageText:
 *
 * The current secure store key is full. No more data can be added.
 *
 */
#define DRM_E_SECURESTORE_FULL           ((DRM_RESULT)0x8004C02BL)

/*
 * MessageId: DRM_E_DUPLICATED_HEADER_ATTRIBUTE
 *
 * MessageText:
 *
 * Duplicated attribute in Header
 *
 */
#define DRM_E_DUPLICATED_HEADER_ATTRIBUTE ((DRM_RESULT)0x8004C02DL)

/*
 * MessageId: DRM_E_NO_KID_IN_HEADER
 *
 * MessageText:
 *
 * No KID attribute in Header
 *
 */
#define DRM_E_NO_KID_IN_HEADER           ((DRM_RESULT)0x8004C02EL)

/*
 * MessageId: DRM_E_NO_LAINFO_IN_HEADER
 *
 * MessageText:
 *
 * No LAINFO attribute in Header
 *
 */
#define DRM_E_NO_LAINFO_IN_HEADER        ((DRM_RESULT)0x8004C02FL)

/*
 * MessageId: DRM_E_NO_CHECKSUM_IN_HEADER
 *
 * MessageText:
 *
 * No Checksum attribute in Header
 *
 */
#define DRM_E_NO_CHECKSUM_IN_HEADER      ((DRM_RESULT)0x8004C030L)

/*
 * MessageId: DRM_E_DST_BLOCK_MISMATCH
 *
 * MessageText:
 *
 * DST block mismatch
 *
 */
#define DRM_E_DST_BLOCK_MISMATCH         ((DRM_RESULT)0x8004C031L)

/*
 * MessageId: DRM_E_DST_EXISTS
 *
 * MessageText:
 *
 * A DST already exists in the specified location
 *
 */
#define DRM_E_DST_EXISTS                 ((DRM_RESULT)0x8004C034L)

/*
 * MessageId: DRM_E_INVALID_DEVICE_CERTIFICATE
 *
 * MessageText:
 *
 * The device certificate is invalid.
 *
 */
#define DRM_E_INVALID_DEVICE_CERTIFICATE ((DRM_RESULT)0x8004C035L)

/*
 * MessageId: DRM_E_DST_LOCK_FAILED
 *
 * MessageText:
 *
 * Locking a segment of the DST failed.
 *
 */
#define DRM_E_DST_LOCK_FAILED            ((DRM_RESULT)0x8004C036L)

/*
 * MessageId: DRM_E_FILE_SEEK_ERROR
 *
 * MessageText:
 *
 * File Seek Error
 *
 */
#define DRM_E_FILE_SEEK_ERROR            ((DRM_RESULT)0x8004C037L)

/*
 * MessageId: DRM_E_DST_NOT_LOCKED_EXCLUSIVE
 *
 * MessageText:
 *
 * Existing lock is not exclusive
 *
 */
#define DRM_E_DST_NOT_LOCKED_EXCLUSIVE   ((DRM_RESULT)0x8004C038L)

/*
 * MessageId: DRM_E_DST_EXCLUSIVE_LOCK_ONLY
 *
 * MessageText:
 *
 * Only exclusive lock is accepted
 *
 */
#define DRM_E_DST_EXCLUSIVE_LOCK_ONLY    ((DRM_RESULT)0x8004C039L)

/*
 * MessageId: DRM_E_HEADER_NOT_SET
 *
 * MessageText:
 *
 * Content header is not set
 *
 */
#define DRM_E_HEADER_NOT_SET             ((DRM_RESULT)0x8004C03CL)

/*
 * MessageId: DRM_E_MACHINE_ID_MISMATCH
 *
 * MessageText:
 *
 * The device has Machine Id different from that in devcert.
 *
 */
#define DRM_E_MACHINE_ID_MISMATCH        ((DRM_RESULT)0x8004C03EL)

/*
 * MessageId: DRM_E_CLK_INVALID_RESPONSE
 *
 * MessageText:
 *
 * The secure clock response is invalid.
 *
 */
#define DRM_E_CLK_INVALID_RESPONSE       ((DRM_RESULT)0x8004C03FL)

/*
 * MessageId: DRM_E_DEVCERT_EXCEEDS_SIZE_LIMIT
 *
 * MessageText:
 *
 * The device certificate exceeds max size
 *
 */
#define DRM_E_DEVCERT_EXCEEDS_SIZE_LIMIT ((DRM_RESULT)0x8004C043L)

/*
 * MessageId: DRM_E_PRIVKEY_READ_ERROR
 *
 * MessageText:
 *
 * Can't get device private key
 *
 */
#define DRM_E_PRIVKEY_READ_ERROR         ((DRM_RESULT)0x8004C047L)

/*
 * MessageId: DRM_E_DEVCERT_TEMPLATE_READ_ERROR
 *
 * MessageText:
 *
 * Can't get the device certificate template
 *
 */
#define DRM_E_DEVCERT_TEMPLATE_READ_ERROR ((DRM_RESULT)0x8004C049L)

/*
 * MessageId: DRM_E_CLK_NOT_SUPPORTED
 *
 * MessageText:
 *
 * The secure clock is not supported.
 *
 */
#define DRM_E_CLK_NOT_SUPPORTED          ((DRM_RESULT)0x8004C04AL)

/*
 * MessageId: DRM_E_METERING_NOT_SUPPORTED
 *
 * MessageText:
 *
 * The Metering is not supported.
 *
 */
#define DRM_E_METERING_NOT_SUPPORTED     ((DRM_RESULT)0x8004C04CL)

/*
 * MessageId: DRM_E_XMLNOTFOUND
 *
 * MessageText:
 *
 * a required XML tag was not found
 *
 */
#define DRM_E_XMLNOTFOUND                ((DRM_RESULT)0x8004C04FL)

/*
 * MessageId: DRM_E_METERING_WRONG_TID
 *
 * MessageText:
 *
 * wrong TID sent on metering response
 *
 */
#define DRM_E_METERING_WRONG_TID         ((DRM_RESULT)0x8004C050L)

/*
 * MessageId: DRM_E_METERING_STORE_CORRUPT
 *
 * MessageText:
 *
 * The metering store is corrupt
 *
 */
#define DRM_E_METERING_STORE_CORRUPT     ((DRM_RESULT)0x8004C052L)

/*
 * MessageId: DRM_E_CERTIFICATE_REVOKED
 *
 * MessageText:
 *
 * A certificate given to DRM was revoked.
 *
 */
#define DRM_E_CERTIFICATE_REVOKED        ((DRM_RESULT)0x8004C053L)

/*
 * MessageId: DRM_E_CRYPTO_FAILED
 *
 * MessageText:
 *
 * A cryptographic operation failed.
 *
 */
#define DRM_E_CRYPTO_FAILED              ((DRM_RESULT)0x8004C054L)

/*
 * MessageId: DRM_E_STACK_CORRUPT
 *
 * MessageText:
 *
 * The stack allocator context is corrupt. Likely a buffer overrun problem.
 *
 */
#define DRM_E_STACK_CORRUPT              ((DRM_RESULT)0x8004C055L)

/*
 * MessageId: DRM_E_V1_LICENSE_CHAIN_NOT_SUPPORTED
 *
 * MessageText:
 *
 * License chaining with V1 content is not supported.
 *
 */
#define DRM_E_V1_LICENSE_CHAIN_NOT_SUPPORTED ((DRM_RESULT)0x8004C057L)

/*
 * MessageId: DRM_E_CLK_NOT_SET
 *
 * MessageText:
 *
 * Time based licenses can not be used because the secure clock is not set on the device.
 *
 */
#define DRM_E_CLK_NOT_SET                ((DRM_RESULT)0x8004C05BL)

/*
 * MessageId: DRM_E_NO_CLK_SUPPORTED
 *
 * MessageText:
 *
 * Time based licenses can not be used because the device does not support any clock.
 *
 */
#define DRM_E_NO_CLK_SUPPORTED           ((DRM_RESULT)0x8004C05CL)

/*
 * MessageId: DRM_E_NO_URL
 *
 * MessageText:
 *
 * Can not find URL info.
 *
 */
#define DRM_E_NO_URL                     ((DRM_RESULT)0x8004C05DL)

/*
 * MessageId: DRM_E_UNKNOWN_DEVICE_PROPERTY
 *
 * MessageText:
 *
 * Unknown device property.
 *
 */
#define DRM_E_UNKNOWN_DEVICE_PROPERTY    ((DRM_RESULT)0x8004C05EL)

/*
 * MessageId: DRM_E_RIV_TOO_SMALL
 *
 * MessageText:
 *
 * RIV on the machine is too small.
 *
 */
#define DRM_E_RIV_TOO_SMALL              ((DRM_RESULT)0x8004C063L)

/*
 * MessageId: DRM_E_STACK_ALREADY_INITIALIZED
 *
 * MessageText:
 *
 * DRM_STK_Init called for initialized stack
 *
 */
#define DRM_E_STACK_ALREADY_INITIALIZED  ((DRM_RESULT)0x8004C064L)

/*
 * MessageId: DRM_E_DEVCERT_REVOKED
 *
 * MessageText:
 *
 * The device certificate given to DRM is revoked.
 *
 */
#define DRM_E_DEVCERT_REVOKED            ((DRM_RESULT)0x8004C065L)

/*
 * MessageId: DRM_E_OEM_RSA_DECRYPTION_ERROR
 *
 * MessageText:
 *
 * Error in OEM RSA Decryption.
 *
 */
#define DRM_E_OEM_RSA_DECRYPTION_ERROR   ((DRM_RESULT)0x8004C066L)

/*
 * MessageId: DRM_E_OEM_RSA_ENCRYPTION_ERROR
 *
 * MessageText:
 *
 * Error in OEM RSA Encryption process
 *
 */
#define DRM_E_OEM_RSA_ENCRYPTION_ERROR   ((DRM_RESULT)0x8004C069L)

/*
 * MessageId: DRM_E_DST_NAMESPACE_EXISTS
 *
 * MessageText:
 *
 * The DST Namespace already exists.
 *
 */
#define DRM_E_DST_NAMESPACE_EXISTS       ((DRM_RESULT)0x8004C06AL)

/*
 * MessageId: DRM_E_PERF_SCOPING_ERROR
 *
 * MessageText:
 *
 * Error in performance scope context
 *
 */
#define DRM_E_PERF_SCOPING_ERROR         ((DRM_RESULT)0x8004C06BL)

/*
 * MessageId: DRM_E_OEM_RSA_INVALID_PRIVATE_KEY
 *
 * MessageText:
 *
 * Invalid private key.
 *
 */
#define DRM_E_OEM_RSA_INVALID_PRIVATE_KEY ((DRM_RESULT)0x8004C06DL)

/*
 * MessageId: DRM_E_NO_OPL_CALLBACK
 *
 * MessageText:
 *
 * There is no callback function to process the output restrictions specified in the license
 *
 */
#define DRM_E_NO_OPL_CALLBACK            ((DRM_RESULT)0x8004C06EL)

/*
 * MessageId: DRM_E_INVALID_PLAYREADY_OBJECT
 *
 * MessageText:
 *
 * Structure of PlayReady object is invalid
 *
 */
#define DRM_E_INVALID_PLAYREADY_OBJECT   ((DRM_RESULT)0x8004C06FL)

/*
 * MessageId: DRM_E_DUPLICATE_LICENSE
 *
 * MessageText:
 *
 * There is already a license in the store with the same KID & LID
 *
 */
#define DRM_E_DUPLICATE_LICENSE          ((DRM_RESULT)0x8004C070L)

/*
 * MessageId: DRM_E_RECORD_NOT_FOUND
 *
 * MessageText:
 *
 * Record with requested type was not found in PlayReady object.
 *
 */
#define DRM_E_RECORD_NOT_FOUND           ((DRM_RESULT)0x8004C072L)

/*
 * MessageId: DRM_E_BUFFER_BOUNDS_EXCEEDED
 *
 * MessageText:
 *
 * An array is being referenced outside of it's bounds.
 *
 */
#define DRM_E_BUFFER_BOUNDS_EXCEEDED     ((DRM_RESULT)0x8004C073L)

/*
 * MessageId: DRM_E_INVALID_BASE64
 *
 * MessageText:
 *
 * An input string contains invalid Base64 characters.
 *
 */
#define DRM_E_INVALID_BASE64             ((DRM_RESULT)0x8004C074L)

/*
 * MessageId: DRM_E_PROTOCOL_VERSION_NOT_SUPPORTED
 *
 * MessageText:
 *
 * The protocol version is not supported.
 *
 */
#define DRM_E_PROTOCOL_VERSION_NOT_SUPPORTED ((DRM_RESULT)0x8004C075L)

/*
 * MessageId: DRM_E_INVALID_LICENSE_RESPONSE_SIGNATURE
 *
 * MessageText:
 *
 * Cannot verify license acquisition's response because signature is invalid.
 *
 */
#define DRM_E_INVALID_LICENSE_RESPONSE_SIGNATURE ((DRM_RESULT)0x8004C076L)

/*
 * MessageId: DRM_E_INVALID_LICENSE_RESPONSE_ID
 *
 * MessageText:
 *
 * Cannot verify license acquisition's response because response ID is invalid.
 *
 */
#define DRM_E_INVALID_LICENSE_RESPONSE_ID ((DRM_RESULT)0x8004C077L)

/*
 * MessageId: DRM_E_LICENSE_RESPONSE_SIGNATURE_MISSING
 *
 * MessageText:
 *
 * Cannot verify license acquisition's response because either response ID, license nonce or signature is missing.
 *
 */
#define DRM_E_LICENSE_RESPONSE_SIGNATURE_MISSING ((DRM_RESULT)0x8004C078L)

/*
 * MessageId: DRM_E_INVALID_DOMAIN_JOIN_RESPONSE_SIGNATURE
 *
 * MessageText:
 *
 * Cannot verify domain join response because signature is invalid.
 *
 */
#define DRM_E_INVALID_DOMAIN_JOIN_RESPONSE_SIGNATURE ((DRM_RESULT)0x8004C079L)

/*
 * MessageId: DRM_E_DOMAIN_JOIN_RESPONSE_SIGNATURE_MISSING
 *
 * MessageText:
 *
 * Cannot verify domain join response because either signing certificate chain or signature is missing.
 *
 */
#define DRM_E_DOMAIN_JOIN_RESPONSE_SIGNATURE_MISSING ((DRM_RESULT)0x8004C07AL)

/*
 * MessageId: DRM_E_ACTIVATION_REQUIRED
 *
 * MessageText:
 *
 * The device must be activated before initialization can succeed.
 *
 */
#define DRM_E_ACTIVATION_REQUIRED        ((DRM_RESULT)0x8004C07BL)

/*
 * MessageId: DRM_E_ACTIVATION_INTERNAL_ERROR
 *
 * MessageText:
 *
 * A server error occurred during device activation.
 *
 */
#define DRM_E_ACTIVATION_INTERNAL_ERROR  ((DRM_RESULT)0x8004C07CL)

/*
 * MessageId: DRM_E_ACTIVATION_GROUP_CERT_REVOKED_ERROR
 *
 * MessageText:
 *
 * The activation group cert has been revoked and the application must be updated with a new client lib.
 *
 */
#define DRM_E_ACTIVATION_GROUP_CERT_REVOKED_ERROR ((DRM_RESULT)0x8004C07DL)

/*
 * MessageId: DRM_E_ACTIVATION_NEW_CLIENT_LIB_REQUIRED_ERROR
 *
 * MessageText:
 *
 * The client lib used by the application is not supported and must be updated.
 *
 */
#define DRM_E_ACTIVATION_NEW_CLIENT_LIB_REQUIRED_ERROR ((DRM_RESULT)0x8004C07EL)

/*
 * MessageId: DRM_E_ACTIVATION_BAD_REQUEST
 *
 * MessageText:
 *
 * The activation request is invalid
 *
 */
#define DRM_E_ACTIVATION_BAD_REQUEST     ((DRM_RESULT)0x8004C07FL)

/*
 * MessageId: DRM_E_FILEIO_ERROR
 *
 * MessageText:
 *
 * Encountered a system error during file I/O.
 *
 */
#define DRM_E_FILEIO_ERROR               ((DRM_RESULT)0x8004C080L)

/*
 * MessageId: DRM_E_UPLINK_LICENSE_NOT_FOUND
 *
 * MessageText:
 *
 * A license was found in the license store but no license was found for its uplink ID.
 *
 */
#define DRM_E_UPLINK_LICENSE_NOT_FOUND   ((DRM_RESULT)0x8004C082L)

/*
 * MessageId: DRM_E_ACTIVATION_CLIENT_ALREADY_CURRENT
 *
 * MessageText:
 *
 * The activation client already has the lastest verion.
 *
 */
#define DRM_E_ACTIVATION_CLIENT_ALREADY_CURRENT ((DRM_RESULT)0x8004C083L)

/*
 * MessageId: DRM_E_LICENSE_REALTIME_EXPIRED
 *
 * MessageText:
 *
 * The license has expired during decryption due to the RealTimeExpiration Restriction.
 *
 */
#define DRM_E_LICENSE_REALTIME_EXPIRED   ((DRM_RESULT)0x8004C084L)

/*
 * MessageId: DRM_E_DECRYPTOR_CANNOT_CLONE
 *
 * MessageText:
 *
 * The decryptor cannot be cloned due to restrictions in the corresponding license.
 *
 */
#define DRM_E_DECRYPTOR_CANNOT_CLONE     ((DRM_RESULT)0x8004C085L)

/*
 * MessageId: DRM_E_ACTIVATION_REQUIRED_REACTIVATION_POSSIBLE
 *
 * MessageText:
 *
 * The device must be activated or reactivated before initialization can succeed.
 *
 */
#define DRM_E_ACTIVATION_REQUIRED_REACTIVATION_POSSIBLE ((DRM_RESULT)0x8004C086L)


/* ------------------------------------------------------------
**
** License evaluator errors: error codes from DRM_E_BASECODE+0xC0 to
** DRM_E_BASECODE+0xDF, 0x8004c0c0-0x8004c0df.
**
** ------------------------------------------------------------
*/

#define DRM_E_LICEVAL_BASECODE                  DRM_E_BASECODE+0xC0

/*
 * MessageId: DRM_E_LICEVAL_LICENSE_NOT_SUPPLIED
 *
 * MessageText:
 *
 * License not supplied in the liceval context
 *
 */
#define DRM_E_LICEVAL_LICENSE_NOT_SUPPLIED ((DRM_RESULT)0x8004C0C0L)

/*
 * MessageId: DRM_E_LICEVAL_KID_MISMATCH
 *
 * MessageText:
 *
 * Mismatch between KID from header and the one inside license
 *
 */
#define DRM_E_LICEVAL_KID_MISMATCH       ((DRM_RESULT)0x8004C0C1L)

/*
 * MessageId: DRM_E_LICEVAL_REQUIRED_REVOCATION_LIST_NOT_AVAILABLE
 *
 * MessageText:
 *
 * Failed to update content revocation
 *
 */
#define DRM_E_LICEVAL_REQUIRED_REVOCATION_LIST_NOT_AVAILABLE ((DRM_RESULT)0x8004C0C4L)


/* ------------------------------------------------------------
**
** XMR parser and builder errors: error codes from DRM_E_BASECODE+0xE0 to
** DRM_E_BASECODE+0xFF, 0x8004c0e0-0x8004c0ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_XMR_BASECODE                      DRM_E_BASECODE+0xE0

/*
 * MessageId: DRM_E_XMR_REQUIRED_OBJECT_MISSING
 *
 * MessageText:
 *
 * XMR license doesn't have one or more required objects.
 *
 */
#define DRM_E_XMR_REQUIRED_OBJECT_MISSING ((DRM_RESULT)0x8004C0E2L)

/*
 * MessageId: DRM_E_XMR_LICENSE_NOT_BINDABLE
 *
 * MessageText:
 *
 * XMR license cannot be bound to because of the Cannot Bind right
 *
 */
#define DRM_E_XMR_LICENSE_NOT_BINDABLE   ((DRM_RESULT)0x8004C0E5L)


/* ------------------------------------------------------------
**
** Device certificate errors: error codes from DRM_E_BASECODE+0x200 to
** DRM_E_BASECODE+0x2FF, 0x8004c200-0x8004c2ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_CERT_BASECODE                  DRM_E_BASECODE+0x200

/*
 * MessageId: DRM_E_INVALID_DEVCERT_ATTRIBUTE
 *
 * MessageText:
 *
 * The attributes in the Device certificate are invalid
 *
 */
#define DRM_E_INVALID_DEVCERT_ATTRIBUTE  ((DRM_RESULT)0x8004C200L)


/* ------------------------------------------------------------
**
** Test errors: error codes from DRM_E_BASECODE+0x300 to
** DRM_E_BASECODE+0x3E7, 0x8004c300-0x8004c3e7.
**
** ------------------------------------------------------------
*/

#define DRM_E_TEST_BASECODE        DRM_E_BASECODE+0x300
#define DRM_S_TEST_BASECODE        DRM_S_BASECODE+0x300

/*
 * MessageId: DRM_E_TEST_ENCRYPT_ERROR
 *
 * MessageText:
 *
 * Error in encryption of cipher text.
 *
 */
#define DRM_E_TEST_ENCRYPT_ERROR         ((DRM_RESULT)0x8004C302L)

/*
 * MessageId: DRM_E_TEST_DECRYPT_ERROR
 *
 * MessageText:
 *
 * Error in cipher text decryption.
 *
 */
#define DRM_E_TEST_DECRYPT_ERROR         ((DRM_RESULT)0x8004C304L)

/*
 * MessageId: DRM_E_TEST_INVALIDARG
 *
 * MessageText:
 *
 * Error in the number of arguments or argument data in Test files.
 *
 */
#define DRM_E_TEST_INVALIDARG            ((DRM_RESULT)0x8004C308L)

/*
 * MessageId: DRM_E_TEST_UNEXPECTED_REVINFO_RESULT
 *
 * MessageText:
 *
 * Revocation cache result was not as expected.
 *
 */
#define DRM_E_TEST_UNEXPECTED_REVINFO_RESULT ((DRM_RESULT)0x8004C30CL)

/*
 * MessageId: DRM_E_TEST_RIV_MISMATCH
 *
 * MessageText:
 *
 * Revocation Info Version(RIV) mismatch.
 *
 */
#define DRM_E_TEST_RIV_MISMATCH          ((DRM_RESULT)0x8004C30DL)

/*
 * MessageId: DRM_E_TEST_URL_ERROR
 *
 * MessageText:
 *
 * There is an error in the URL from the challenge generated.
 *
 */
#define DRM_E_TEST_URL_ERROR             ((DRM_RESULT)0x8004C310L)

/*
 * MessageId: DRM_E_TEST_MID_MISMATCH
 *
 * MessageText:
 *
 * The MIDs returned from the DRM_MANAGER_CONTEXT does not match the test input.
 *
 */
#define DRM_E_TEST_MID_MISMATCH          ((DRM_RESULT)0x8004C311L)

/*
 * MessageId: DRM_E_TEST_METER_CERTIFICATE_MISMATCH
 *
 * MessageText:
 *
 * The input data does not match with the Metering certificate returned from the license.
 *
 */
#define DRM_E_TEST_METER_CERTIFICATE_MISMATCH ((DRM_RESULT)0x8004C312L)

/*
 * MessageId: DRM_E_TEST_SOURCE_ID_MISMATCH
 *
 * MessageText:
 *
 * The input data and license state returned from the license do not match.
 *
 */
#define DRM_E_TEST_SOURCE_ID_MISMATCH    ((DRM_RESULT)0x8004C316L)

/*
 * MessageId: DRM_E_TEST_UNEXPECTED_LICENSE_COUNT
 *
 * MessageText:
 *
 * The input data and the number of license from the KID do not match.
 *
 */
#define DRM_E_TEST_UNEXPECTED_LICENSE_COUNT ((DRM_RESULT)0x8004C317L)

/*
 * MessageId: DRM_E_TEST_UNEXPECTED_DEVICE_PROPERTY
 *
 * MessageText:
 *
 * Unknown device property.
 *
 */
#define DRM_E_TEST_UNEXPECTED_DEVICE_PROPERTY ((DRM_RESULT)0x8004C318L)

/*
 * MessageId: DRM_E_TEST_DRMMANAGER_MISALIGNED_BYTES
 *
 * MessageText:
 *
 * Error due to misalignment of bytes.
 *
 */
#define DRM_E_TEST_DRMMANAGER_MISALIGNED_BYTES ((DRM_RESULT)0x8004C319L)

/*
 * MessageId: DRM_E_TEST_OPL_MISMATCH
 *
 * MessageText:
 *
 * The minimum levels of the compressed/uncompressed Digital and Analog Video do not match the OPL.
 *
 */
#define DRM_E_TEST_OPL_MISMATCH          ((DRM_RESULT)0x8004C31BL)

/*
 * MessageId: DRM_E_TEST_INVALID_OPL_CALLBACK
 *
 * MessageText:
 *
 * The callback type supplied is not valid.
 *
 */
#define DRM_E_TEST_INVALID_OPL_CALLBACK  ((DRM_RESULT)0x8004C31CL)

/*
 * MessageId: DRM_E_TEST_INCOMPLETE
 *
 * MessageText:
 *
 * The test function failed to complete.
 *
 */
#define DRM_E_TEST_INCOMPLETE            ((DRM_RESULT)0x8004C31DL)

/*
 * MessageId: DRM_E_TEST_UNEXPECTED_OUTPUT
 *
 * MessageText:
 *
 * The output of the function being tested does not match the expected output.
 *
 */
#define DRM_E_TEST_UNEXPECTED_OUTPUT     ((DRM_RESULT)0x8004C31EL)

/*
 * MessageId: DRM_E_TEST_TOO_SLOW
 *
 * MessageText:
 *
 * The performance test failed because DRM took longer than its maximum time.
 *
 */
#define DRM_E_TEST_TOO_SLOW              ((DRM_RESULT)0x8004C322L)

/*
 * MessageId: DRM_E_TEST_LICENSESTORE_NOT_OPEN
 *
 * MessageText:
 *
 * The License Store contexts in the App Manager context are not open.
 *
 */
#define DRM_E_TEST_LICENSESTORE_NOT_OPEN ((DRM_RESULT)0x8004C323L)

/*
 * MessageId: DRM_E_TEST_VARIABLE_NOT_SET
 *
 * MessageText:
 *
 * A global variable needed for test execution has not been set correctly.
 *
 */
#define DRM_E_TEST_VARIABLE_NOT_SET      ((DRM_RESULT)0x8004C325L)

/*
 * MessageId: DRM_E_TEST_NOMORE
 *
 * MessageText:
 *
 * The same as DRM_E_NOMORE, only explicitly used in test code.
 *
 */
#define DRM_E_TEST_NOMORE                ((DRM_RESULT)0x8004C326L)

/*
 * MessageId: DRM_E_TEST_FILE_LOAD_ERROR
 *
 * MessageText:
 *
 * There was an error loading a test data file.
 *
 */
#define DRM_E_TEST_FILE_LOAD_ERROR       ((DRM_RESULT)0x8004C327L)

/*
 * MessageId: DRM_E_TEST_UNSUPPORTED_FILE_FORMAT
 *
 * MessageText:
 *
 * A file format is being used which is not supported by the test function.
 *
 */
#define DRM_E_TEST_UNSUPPORTED_FILE_FORMAT ((DRM_RESULT)0x8004C329L)

/*
 * MessageId: DRM_E_TEST_PARSING_ERROR
 *
 * MessageText:
 *
 * There was an error parsing input parameter.
 *
 */
#define DRM_E_TEST_PARSING_ERROR         ((DRM_RESULT)0x8004C32AL)

/*
 * MessageId: DRM_E_TEST_NOTIMPL
 *
 * MessageText:
 *
 * The specified test API is not implemented.
 *
 */
#define DRM_E_TEST_NOTIMPL               ((DRM_RESULT)0x8004C32BL)

/*
 * MessageId: DRM_E_TEST_VARIABLE_NOTFOUND
 *
 * MessageText:
 *
 * The specified test varaible was not found in the shared variable table.
 *
 */
#define DRM_E_TEST_VARIABLE_NOTFOUND     ((DRM_RESULT)0x8004C32CL)

/*
 * MessageId: DRM_E_TEST_VARIABLE_LISTFULL
 *
 * MessageText:
 *
 * The shared test variable table is full.
 *
 */
#define DRM_E_TEST_VARIABLE_LISTFULL     ((DRM_RESULT)0x8004C32DL)

/*
 * MessageId: DRM_E_TEST_UNEXPECTED_CONTENT_PROPERTY
 *
 * MessageText:
 *
 * Unknown content property.
 *
 */
#define DRM_E_TEST_UNEXPECTED_CONTENT_PROPERTY ((DRM_RESULT)0x8004C32EL)

/*
 * MessageId: DRM_E_TEST_PRO_HEADER_NOT_SET
 *
 * MessageText:
 *
 * PlayReady Object Header not set.
 *
 */
#define DRM_E_TEST_PRO_HEADER_NOT_SET    ((DRM_RESULT)0x8004C32FL)

/*
 * MessageId: DRM_E_TEST_NON_PRO_HEADER_TYPE
 *
 * MessageText:
 *
 * Incompatible header - PlayReady Object Header expected.
 *
 */
#define DRM_E_TEST_NON_PRO_HEADER_TYPE   ((DRM_RESULT)0x8004C330L)

/*
 * MessageId: DRM_E_TEST_INVALID_FILE
 *
 * MessageText:
 *
 * The data file given was invalid.
 *
 */
#define DRM_E_TEST_INVALID_FILE          ((DRM_RESULT)0x8004C334L)

/*
 * MessageId: DRM_E_TEST_METERING_DATA_INCORRECT
 *
 * MessageText:
 *
 * The metering data reported is incorrect.
 *
 */
#define DRM_E_TEST_METERING_DATA_INCORRECT ((DRM_RESULT)0x8004C336L)

/*
 * MessageId: DRM_E_TEST_FILE_NOT_OPEN
 *
 * MessageText:
 *
 * The handle variable for a test file is NULL. This indicates that a file was not opened.
 *
 */
#define DRM_E_TEST_FILE_NOT_OPEN         ((DRM_RESULT)0x8004C338L)

/*
 * MessageId: DRM_E_TEST_KEYFILE_VERIFICATION_FAILURE
 *
 * MessageText:
 *
 * Verification of the Keyfile context failed.
 *
 */
#define DRM_E_TEST_KEYFILE_VERIFICATION_FAILURE ((DRM_RESULT)0x8004C33CL)

/*
 * MessageId: DRM_E_TEST_DATA_VERIFICATION_FAILURE
 *
 * MessageText:
 *
 * Data does not match expected value and failed verification.
 *
 */
#define DRM_E_TEST_DATA_VERIFICATION_FAILURE ((DRM_RESULT)0x8004C33DL)

/*
 * MessageId: DRM_E_TEST_NET_FAIL
 *
 * MessageText:
 *
 * The Test failed to perform Network I/O.
 *
 */
#define DRM_E_TEST_NET_FAIL              ((DRM_RESULT)0x8004C33EL)

/*
 * MessageId: DRM_E_TEST_CLEANUP_FAIL
 *
 * MessageText:
 *
 * A failure occurred during the test case cleanup phase.
 *
 */
#define DRM_E_TEST_CLEANUP_FAIL          ((DRM_RESULT)0x8004C33FL)

/*
 * MessageId: DRM_E_TEST_LICGEN_UNSUPPORTED_VALUE
 *
 * MessageText:
 *
 * A property used during license generation is not supported.
 *
 */
#define DRM_E_TEST_LICGEN_UNSUPPORTED_VALUE ((DRM_RESULT)0x8004C340L)


/* ------------------------------------------------------------
**
** Errors of the range 0x8004c3e8-0x8004c3f8 (range is where
** *decimal* +1000 starts.
**
** ------------------------------------------------------------
*/

/*
 * MessageId: DRM_E_LOGICERR
 *
 * MessageText:
 *
 * DRM code has a logic error in it.  This result should never be returned.  There is an unhandled code path if it is returned.
 *
 */
#define DRM_E_LOGICERR                   ((DRM_RESULT)0x8004C3E8L)

/*
 * MessageId: DRM_E_INVALID_REV_INFO
 *
 * MessageText:
 *
 * The rev info blob is invalid.
 *
 */
#define DRM_E_INVALID_REV_INFO           ((DRM_RESULT)0x8004C3E9L)

/*
 * MessageId: DRM_E_REVOCATION_BUFFER_TOO_SMALL
 *
 * MessageText:
 *
 * The revocation buffer is too small.
 *
 */
#define DRM_E_REVOCATION_BUFFER_TOO_SMALL ((DRM_RESULT)0x8004C3EBL)

/*
 * MessageId: DRM_E_DST_NOT_COMPATIBLE
 *
 * MessageText:
 *
 * The data store version is incompatible with this version of DRM.
 *
 */
#define DRM_E_DST_NOT_COMPATIBLE         ((DRM_RESULT)0x8004C3EDL)

/*
 * MessageId: DRM_E_RSA_DECRYPTION_ERROR
 *
 * MessageText:
 *
 * The data block/Encoded message used in OAEP decoding is incorrect.
 *
 */
#define DRM_E_RSA_DECRYPTION_ERROR       ((DRM_RESULT)0x8004C3F0L)

/*
 * MessageId: DRM_E_OEM_RSA_MESSAGE_TOO_BIG
 *
 * MessageText:
 *
 * The base message buffer is larger than the given modulus.
 *
 */
#define DRM_E_OEM_RSA_MESSAGE_TOO_BIG    ((DRM_RESULT)0x8004C3F1L)

/*
 * MessageId: DRM_E_METERCERT_NOT_FOUND
 *
 * MessageText:
 *
 * The metering certificate was not found in the store.
 *
 */
#define DRM_E_METERCERT_NOT_FOUND        ((DRM_RESULT)0x8004C3F2L)

/*
 * MessageId: DRM_E_MODULAR_ARITHMETIC_FAILURE
 *
 * MessageText:
 *
 * A failure occurred in bignum modular arithmetic.
 *
 */
#define DRM_E_MODULAR_ARITHMETIC_FAILURE ((DRM_RESULT)0x8004C3F3L)

/*
 * MessageId: DRM_E_REVOCATION_INVALID_PACKAGE
 *
 * MessageText:
 *
 * The revocation package is invalid
 *
 */
#define DRM_E_REVOCATION_INVALID_PACKAGE ((DRM_RESULT)0x8004C3F5L)

/*
 * MessageId: DRM_E_VAR_NOT_INITIALIZED
 *
 * MessageText:
 *
 * Variable was not initialized.
 *
 */
#define DRM_E_VAR_NOT_INITIALIZED        ((DRM_RESULT)0x8004C3F7L)


/* ------------------------------------------------------------
**
** Domain errors: error codes from DRM_E_BASECODE+0x500 to
** DRM_E_BASECODE+0x57F, 0x8004c500-0x8004c57f.
**
** ------------------------------------------------------------
*/

#define DRM_E_DOMAIN_BASECODE       DRM_E_BASECODE + 0x500

/*
 * MessageId: DRM_E_DOMAIN_INVALID_GUID
 *
 * MessageText:
 *
 * Not a correct GUID.
 *
 */
#define DRM_E_DOMAIN_INVALID_GUID        ((DRM_RESULT)0x8004C500L)

/*
 * MessageId: DRM_E_DOMAIN_INVALID_CUSTOM_DATA_TYPE
 *
 * MessageText:
 *
 * Not a valid custom data type.
 *
 */
#define DRM_E_DOMAIN_INVALID_CUSTOM_DATA_TYPE ((DRM_RESULT)0x8004C501L)

/*
 * MessageId: DRM_E_DOMAIN_STORE_ADD_DATA
 *
 * MessageText:
 *
 * Failed to add data into the domain store.
 *
 */
#define DRM_E_DOMAIN_STORE_ADD_DATA      ((DRM_RESULT)0x8004C502L)

/*
 * MessageId: DRM_E_DOMAIN_STORE_GET_DATA
 *
 * MessageText:
 *
 * Failed to retrieve data from the domain store.
 *
 */
#define DRM_E_DOMAIN_STORE_GET_DATA      ((DRM_RESULT)0x8004C503L)

/*
 * MessageId: DRM_E_DOMAIN_STORE_DELETE_DATA
 *
 * MessageText:
 *
 * Failed to delete data from the domain store.
 *
 */
#define DRM_E_DOMAIN_STORE_DELETE_DATA   ((DRM_RESULT)0x8004C504L)

/*
 * MessageId: DRM_E_DOMAIN_STORE_OPEN_STORE
 *
 * MessageText:
 *
 * Failed to open the domain store.
 *
 */
#define DRM_E_DOMAIN_STORE_OPEN_STORE    ((DRM_RESULT)0x8004C505L)

/*
 * MessageId: DRM_E_DOMAIN_STORE_CLOSE_STORE
 *
 * MessageText:
 *
 * Failed to close the domain store.
 *
 */
#define DRM_E_DOMAIN_STORE_CLOSE_STORE   ((DRM_RESULT)0x8004C506L)

/*
 * MessageId: DRM_E_DOMAIN_BIND_LICENSE
 *
 * MessageText:
 *
 * Failed to bind to the domain license.
 *
 */
#define DRM_E_DOMAIN_BIND_LICENSE        ((DRM_RESULT)0x8004C507L)

/*
 * MessageId: DRM_E_DOMAIN_INVALID_CUSTOM_DATA
 *
 * MessageText:
 *
 * Not a valid custom data.
 *
 */
#define DRM_E_DOMAIN_INVALID_CUSTOM_DATA ((DRM_RESULT)0x8004C508L)

/*
 * MessageId: DRM_E_DOMAIN_NOT_FOUND
 *
 * MessageText:
 *
 * No domain information is found.
 *
 */
#define DRM_E_DOMAIN_NOT_FOUND           ((DRM_RESULT)0x8004C509L)

/*
 * MessageId: DRM_E_DOMAIN_INVALID_DOMKEYXMR_DATA
 *
 * MessageText:
 *
 * The domain join response contains invalid domain privkey XMR data.
 *
 */
#define DRM_E_DOMAIN_INVALID_DOMKEYXMR_DATA ((DRM_RESULT)0x8004C50AL)

/*
 * MessageId: DRM_E_DOMAIN_STORE_INVALID_KEY_RECORD
 *
 * MessageText:
 *
 * Invalid format of domain private key record read from the domain store.
 *
 */
#define DRM_E_DOMAIN_STORE_INVALID_KEY_RECORD ((DRM_RESULT)0x8004C50BL)

/*
 * MessageId: DRM_E_DOMAIN_JOIN_TOO_MANY_KEYS
 *
 * MessageText:
 *
 * The server returned too many domain keys for the client to handle.
 *
 */
#define DRM_E_DOMAIN_JOIN_TOO_MANY_KEYS  ((DRM_RESULT)0x8004C50CL)


/* ------------------------------------------------------------
**
** PC errors returned by core logic: error codes from DRM_E_BASECODE+0x580 to
** DRM_E_BASECODE+0x5FF, 0x8004c580-0x8004c5ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_PC_BASECODE           DRM_E_BASECODE + 0x580

/*
 * MessageId: DRM_E_DEVICE_DOMAIN_JOIN_REQUIRED
 *
 * MessageText:
 *
 * This error code communicates to the application that the device is not a member of a domain. The app can uses this error code in turn to decide whether it needs to join the domain or not
 *
 */
#define DRM_E_DEVICE_DOMAIN_JOIN_REQUIRED ((DRM_RESULT)0x8004C580L)


/* ------------------------------------------------------------
**
** Server errors returned by core logic: error codes from DRM_E_BASECODE+0x600
** to DRM_E_BASECODE+0x6FF, 0x8004c600-0x8004c6ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_SERVER_BASECODE       DRM_E_BASECODE + 0x600

/*
 * MessageId: DRM_E_SERVER_INTERNAL_ERROR
 *
 * MessageText:
 *
 * An internal server error occurred.
 *
 */
#define DRM_E_SERVER_INTERNAL_ERROR      ((DRM_RESULT)0x8004C600L)

/*
 * MessageId: DRM_E_SERVER_INVALID_MESSAGE
 *
 * MessageText:
 *
 * The message sent to the server was invalid.
 *
 */
#define DRM_E_SERVER_INVALID_MESSAGE     ((DRM_RESULT)0x8004C601L)

/*
 * MessageId: DRM_E_SERVER_DEVICE_LIMIT_REACHED
 *
 * MessageText:
 *
 * The device limit for the domain has been reached.
 *
 */
#define DRM_E_SERVER_DEVICE_LIMIT_REACHED ((DRM_RESULT)0x8004C602L)

/*
 * MessageId: DRM_E_SERVER_INDIV_REQUIRED
 *
 * MessageText:
 *
 * Individualization of the client is required.
 *
 */
#define DRM_E_SERVER_INDIV_REQUIRED      ((DRM_RESULT)0x8004C603L)

/*
 * MessageId: DRM_E_SERVER_SERVICE_SPECIFIC
 *
 * MessageText:
 *
 * An error specific to the service has occurred.
 *
 */
#define DRM_E_SERVER_SERVICE_SPECIFIC    ((DRM_RESULT)0x8004C604L)

/*
 * MessageId: DRM_E_SERVER_DOMAIN_REQUIRED
 *
 * MessageText:
 *
 * A Domain certificate is required.
 *
 */
#define DRM_E_SERVER_DOMAIN_REQUIRED     ((DRM_RESULT)0x8004C605L)

/*
 * MessageId: DRM_E_SERVER_RENEW_DOMAIN
 *
 * MessageText:
 *
 * The Domain certificate needs to be renewed.
 *
 */
#define DRM_E_SERVER_RENEW_DOMAIN        ((DRM_RESULT)0x8004C606L)

/*
 * MessageId: DRM_E_SERVER_UNKNOWN_METERINGID
 *
 * MessageText:
 *
 * The metering identifier is unknown.
 *
 */
#define DRM_E_SERVER_UNKNOWN_METERINGID  ((DRM_RESULT)0x8004C607L)

/*
 * MessageId: DRM_E_SERVER_COMPUTER_LIMIT_REACHED
 *
 * MessageText:
 *
 * The computer limit for the domain has been reached.
 *
 */
#define DRM_E_SERVER_COMPUTER_LIMIT_REACHED ((DRM_RESULT)0x8004C608L)

/*
 * MessageId: DRM_E_SERVER_PROTOCOL_FALLBACK
 *
 * MessageText:
 *
 * The client should fallback to the V2 license acquisition protocol.
 *
 */
#define DRM_E_SERVER_PROTOCOL_FALLBACK   ((DRM_RESULT)0x8004C609L)

/*
 * MessageId: DRM_E_SERVER_NOT_A_MEMBER
 *
 * MessageText:
 *
 * The client was removed from the domain in an offline fashion and thus still has a domain cert, but not a valid domain membership.
 *
 */
#define DRM_E_SERVER_NOT_A_MEMBER        ((DRM_RESULT)0x8004C60AL)

/*
 * MessageId: DRM_E_SERVER_PROTOCOL_VERSION_MISMATCH
 *
 * MessageText:
 *
 * The protocol version specified was not supported by the server.
 *
 */
#define DRM_E_SERVER_PROTOCOL_VERSION_MISMATCH ((DRM_RESULT)0x8004C60BL)

/*
 * MessageId: DRM_E_SERVER_UNKNOWN_ACCOUNTID
 *
 * MessageText:
 *
 * The account identifier is unknown.
 *
 */
#define DRM_E_SERVER_UNKNOWN_ACCOUNTID   ((DRM_RESULT)0x8004C60CL)

/*
 * MessageId: DRM_E_SERVER_PROTOCOL_REDIRECT
 *
 * MessageText:
 *
 * The protocol has a redirect.
 *
 */
#define DRM_E_SERVER_PROTOCOL_REDIRECT   ((DRM_RESULT)0x8004C60DL)

/*
 * MessageId: DRM_E_SERVER_UNKNOWN_TRANSACTIONID
 *
 * MessageText:
 *
 * The transaction identifier is unknown.
 *
 */
#define DRM_E_SERVER_UNKNOWN_TRANSACTIONID ((DRM_RESULT)0x8004C610L)

/*
 * MessageId: DRM_E_SERVER_INVALID_LICENSEID
 *
 * MessageText:
 *
 * The license identifier is invalid.
 *
 */
#define DRM_E_SERVER_INVALID_LICENSEID   ((DRM_RESULT)0x8004C611L)

/*
 * MessageId: DRM_E_SERVER_MAXIMUM_LICENSEID_EXCEEDED
 *
 * MessageText:
 *
 * The maximum number of license identifiers in the request was exceeded.
 *
 */
#define DRM_E_SERVER_MAXIMUM_LICENSEID_EXCEEDED ((DRM_RESULT)0x8004C612L)


/* ------------------------------------------------------------
** DRM_E_BASECODE + 0x680 - DRM_E_BASECODE + 0x6ff (0x8004c680-0x8004c6ff)
** are reserved for DRM Services.
**
** See source\common\services\inc\svcerrors.h for Services error codes.
**
** ------------------------------------------------------------
*/

#define DRM_E_SERVICES_BASECODE     (DRM_E_BASECODE + 0x680)

/* ------------------------------------------------------------
**
** License acquisition protocol errors: error codes from DRM_E_BASECODE+0x700
** to DRM_E_BASECODE+0x77F, 0x8004c700-0x8004c77f.
**
** ------------------------------------------------------------
*/

#define DRM_E_LICACQ_BASECODE       DRM_E_BASECODE + 0x700

/*
 * MessageId: DRM_E_LICACQ_TOO_MANY_LICENSES
 *
 * MessageText:
 *
 * There are too many licenses in the license response.
 *
 */
#define DRM_E_LICACQ_TOO_MANY_LICENSES   ((DRM_RESULT)0x8004C700L)

/*
 * MessageId: DRM_E_LICACQ_ACK_TRANSACTION_ID_TOO_BIG
 *
 * MessageText:
 *
 * The Transaction ID specified by the server exceeds the allocated buffer.
 *
 */
#define DRM_E_LICACQ_ACK_TRANSACTION_ID_TOO_BIG ((DRM_RESULT)0x8004C701L)


/* ------------------------------------------------------------
**
** Binary certificate errors: error codes from DRM_E_BASECODE+0x800
** to DRM_E_BASECODE+0x8FF, 0x8004c800-0x8004c8ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_BCERT_BASECODE     DRM_E_BASECODE + 0x800

/*
 * MessageId: DRM_E_BCERT_INVALID_SIGNATURE_TYPE
 *
 * MessageText:
 *
 * An invalid signature type was encountered
 *
 */
#define DRM_E_BCERT_INVALID_SIGNATURE_TYPE ((DRM_RESULT)0x8004C800L)

/*
 * MessageId: DRM_E_BCERT_CHAIN_TOO_DEEP
 *
 * MessageText:
 *
 * There are, or there would be, too many certificates in the certificate chain
 *
 */
#define DRM_E_BCERT_CHAIN_TOO_DEEP       ((DRM_RESULT)0x8004C801L)

/*
 * MessageId: DRM_E_BCERT_INVALID_CERT_TYPE
 *
 * MessageText:
 *
 * An invalid certificate type was encountered
 *
 */
#define DRM_E_BCERT_INVALID_CERT_TYPE    ((DRM_RESULT)0x8004C802L)

/*
 * MessageId: DRM_E_BCERT_INVALID_FEATURE
 *
 * MessageText:
 *
 * An invalid feature entry was encountered OR the porting kit was linked with mutually incompatible features or features incompatible with the certificate
 *
 */
#define DRM_E_BCERT_INVALID_FEATURE      ((DRM_RESULT)0x8004C803L)

/*
 * MessageId: DRM_E_BCERT_INVALID_KEY_USAGE
 *
 * MessageText:
 *
 * An invalid public key usage was encountered
 *
 */
#define DRM_E_BCERT_INVALID_KEY_USAGE    ((DRM_RESULT)0x8004C804L)

/*
 * MessageId: DRM_E_BCERT_INVALID_SECURITY_VERSION
 *
 * MessageText:
 *
 * An invalid Indiv Box security version was encountered
 *
 */
#define DRM_E_BCERT_INVALID_SECURITY_VERSION ((DRM_RESULT)0x8004C805L)

/*
 * MessageId: DRM_E_BCERT_INVALID_KEY_TYPE
 *
 * MessageText:
 *
 * An invalid public key type was encountered
 *
 */
#define DRM_E_BCERT_INVALID_KEY_TYPE     ((DRM_RESULT)0x8004C806L)

/*
 * MessageId: DRM_E_BCERT_INVALID_KEY_LENGTH
 *
 * MessageText:
 *
 * An invalid public key length was encountered
 *
 */
#define DRM_E_BCERT_INVALID_KEY_LENGTH   ((DRM_RESULT)0x8004C807L)

/*
 * MessageId: DRM_E_BCERT_INVALID_MAX_LICENSE_CHAIN_DEPTH
 *
 * MessageText:
 *
 * An invalid maximum license chain depth was encountered
 *
 */
#define DRM_E_BCERT_INVALID_MAX_LICENSE_CHAIN_DEPTH ((DRM_RESULT)0x8004C80AL)

/*
 * MessageId: DRM_E_BCERT_INVALID_SECURITY_LEVEL
 *
 * MessageText:
 *
 * An invalid security level was encountered
 *
 */
#define DRM_E_BCERT_INVALID_SECURITY_LEVEL ((DRM_RESULT)0x8004C80BL)

/*
 * MessageId: DRM_E_BCERT_PRIVATE_KEY_NOT_SPECIFIED
 *
 * MessageText:
 *
 * A private key for signing the certificate was not provided to the builder
 *
 */
#define DRM_E_BCERT_PRIVATE_KEY_NOT_SPECIFIED ((DRM_RESULT)0x8004C80CL)

/*
 * MessageId: DRM_E_BCERT_ISSUER_KEY_NOT_SPECIFIED
 *
 * MessageText:
 *
 * An issuer key was not provided to the builder
 *
 */
#define DRM_E_BCERT_ISSUER_KEY_NOT_SPECIFIED ((DRM_RESULT)0x8004C80DL)

/*
 * MessageId: DRM_E_BCERT_ACCOUNT_ID_NOT_SPECIFIED
 *
 * MessageText:
 *
 * An account ID was not provided to the builder
 *
 */
#define DRM_E_BCERT_ACCOUNT_ID_NOT_SPECIFIED ((DRM_RESULT)0x8004C80EL)

/*
 * MessageId: DRM_E_BCERT_SERVICE_ID_NOT_SPECIFIED
 *
 * MessageText:
 *
 * A service provider ID was not provided to the builder
 *
 */
#define DRM_E_BCERT_SERVICE_ID_NOT_SPECIFIED ((DRM_RESULT)0x8004C80FL)

/*
 * MessageId: DRM_E_BCERT_DOMAIN_URL_NOT_SPECIFIED
 *
 * MessageText:
 *
 * A domain URL was not provided to the builder
 *
 */
#define DRM_E_BCERT_DOMAIN_URL_NOT_SPECIFIED ((DRM_RESULT)0x8004C811L)

/*
 * MessageId: DRM_E_BCERT_DOMAIN_URL_TOO_LONG
 *
 * MessageText:
 *
 * The domain URL contains too many ASCII characters
 *
 */
#define DRM_E_BCERT_DOMAIN_URL_TOO_LONG  ((DRM_RESULT)0x8004C812L)

/*
 * MessageId: DRM_E_BCERT_CERT_ID_NOT_SPECIFIED
 *
 * MessageText:
 *
 * A certificate ID was not provided to the builder
 *
 */
#define DRM_E_BCERT_CERT_ID_NOT_SPECIFIED ((DRM_RESULT)0x8004C816L)

/*
 * MessageId: DRM_E_BCERT_PUBLIC_KEY_NOT_SPECIFIED
 *
 * MessageText:
 *
 * A public key for the certificate was not provided to the builder or not found by the parser
 *
 */
#define DRM_E_BCERT_PUBLIC_KEY_NOT_SPECIFIED ((DRM_RESULT)0x8004C817L)

/*
 * MessageId: DRM_E_BCERT_KEY_USAGES_NOT_SPECIFIED
 *
 * MessageText:
 *
 * The public key usage information was not provided to the builder or not found by the parser
 *
 */
#define DRM_E_BCERT_KEY_USAGES_NOT_SPECIFIED ((DRM_RESULT)0x8004C818L)

/*
 * MessageId: DRM_E_BCERT_STRING_NOT_NULL_TERMINATED
 *
 * MessageText:
 *
 * Data string is not null-teminated
 *
 */
#define DRM_E_BCERT_STRING_NOT_NULL_TERMINATED ((DRM_RESULT)0x8004C819L)

/*
 * MessageId: DRM_E_BCERT_BASICINFO_CERT_EXPIRED
 *
 * MessageText:
 *
 * Certificate is expired
 *
 */
#define DRM_E_BCERT_BASICINFO_CERT_EXPIRED ((DRM_RESULT)0x8004C81CL)

/*
 * MessageId: DRM_E_BCERT_ISSUERKEY_KEYINFO_MISMATCH
 *
 * MessageText:
 *
 * The cert's Issuer Key does not match key info in the next cert
 *
 */
#define DRM_E_BCERT_ISSUERKEY_KEYINFO_MISMATCH ((DRM_RESULT)0x8004C81EL)

/*
 * MessageId: DRM_E_BCERT_INVALID_CHAIN_HEADER_TAG
 *
 * MessageText:
 *
 * Cert chain header tag is invalid
 *
 */
#define DRM_E_BCERT_INVALID_CHAIN_HEADER_TAG ((DRM_RESULT)0x8004C821L)

/*
 * MessageId: DRM_E_BCERT_INVALID_CHAIN_VERSION
 *
 * MessageText:
 *
 * Cert chain version is invalid
 *
 */
#define DRM_E_BCERT_INVALID_CHAIN_VERSION ((DRM_RESULT)0x8004C822L)

/*
 * MessageId: DRM_E_BCERT_INVALID_CHAIN_LENGTH
 *
 * MessageText:
 *
 * Cert chain length value is invalid
 *
 */
#define DRM_E_BCERT_INVALID_CHAIN_LENGTH ((DRM_RESULT)0x8004C823L)

/*
 * MessageId: DRM_E_BCERT_INVALID_CERT_VERSION
 *
 * MessageText:
 *
 * Cert version is invalid
 *
 */
#define DRM_E_BCERT_INVALID_CERT_VERSION ((DRM_RESULT)0x8004C825L)

/*
 * MessageId: DRM_E_BCERT_INVALID_SIGNEDCERT_LENGTH
 *
 * MessageText:
 *
 * Length of signed portion of certificate is invalid
 *
 */
#define DRM_E_BCERT_INVALID_SIGNEDCERT_LENGTH ((DRM_RESULT)0x8004C827L)

/*
 * MessageId: DRM_E_BCERT_INVALID_PLATFORM_IDENTIFIER
 *
 * MessageText:
 *
 * An invalid Platform Identifier was specified
 *
 */
#define DRM_E_BCERT_INVALID_PLATFORM_IDENTIFIER ((DRM_RESULT)0x8004C828L)

/*
 * MessageId: DRM_E_BCERT_INVALID_EXTDATARECORD
 *
 * MessageText:
 *
 * An invalid extended data record
 *
 */
#define DRM_E_BCERT_INVALID_EXTDATARECORD ((DRM_RESULT)0x8004C82AL)

/*
 * MessageId: DRM_E_BCERT_EXTDATA_PRIVKEY_MUST_PRESENT
 *
 * MessageText:
 *
 * Extended data record length must be present.
 *
 */
#define DRM_E_BCERT_EXTDATA_PRIVKEY_MUST_PRESENT ((DRM_RESULT)0x8004C82CL)

/*
 * MessageId: DRM_E_BCERT_HWIDINFO_IS_MISSING
 *
 * MessageText:
 *
 * The PC certificate is correct but is not ready to use because has no HWID information
 *
 */
#define DRM_E_BCERT_HWIDINFO_IS_MISSING  ((DRM_RESULT)0x8004C82FL)

/*
 * MessageId: DRM_E_BCERT_METERING_ID_NOT_SPECIFIED
 *
 * MessageText:
 *
 * An metering ID was not provided to the builder
 *
 */
#define DRM_E_BCERT_METERING_ID_NOT_SPECIFIED ((DRM_RESULT)0x8004C833L)

/*
 * MessageId: DRM_E_BCERT_METERING_URL_NOT_SPECIFIED
 *
 * MessageText:
 *
 * A metering URL was not provided to the builder
 *
 */
#define DRM_E_BCERT_METERING_URL_NOT_SPECIFIED ((DRM_RESULT)0x8004C834L)

/*
 * MessageId: DRM_E_BCERT_METERING_URL_TOO_LONG
 *
 * MessageText:
 *
 * The metering URL contains too many ASCII characters
 *
 */
#define DRM_E_BCERT_METERING_URL_TOO_LONG ((DRM_RESULT)0x8004C835L)

/*
 * MessageId: DRM_E_BCERT_VERIFICATION_ERRORS
 *
 * MessageText:
 *
 * Verification errors are found while parsing cert chain
 *
 */
#define DRM_E_BCERT_VERIFICATION_ERRORS  ((DRM_RESULT)0x8004C836L)

/*
 * MessageId: DRM_E_BCERT_REQUIRED_KEYUSAGE_MISSING
 *
 * MessageText:
 *
 * Required key usage is missing
 *
 */
#define DRM_E_BCERT_REQUIRED_KEYUSAGE_MISSING ((DRM_RESULT)0x8004C837L)

/*
 * MessageId: DRM_E_BCERT_NO_PUBKEY_WITH_REQUESTED_KEYUSAGE
 *
 * MessageText:
 *
 * The certificate does not contain a public key with the requested key usage
 *
 */
#define DRM_E_BCERT_NO_PUBKEY_WITH_REQUESTED_KEYUSAGE ((DRM_RESULT)0x8004C838L)

/*
 * MessageId: DRM_E_BCERT_MANUFACTURER_STRING_TOO_LONG
 *
 * MessageText:
 *
 * The manufacturer string is too long
 *
 */
#define DRM_E_BCERT_MANUFACTURER_STRING_TOO_LONG ((DRM_RESULT)0x8004C839L)

/*
 * MessageId: DRM_E_BCERT_TOO_MANY_PUBLIC_KEYS
 *
 * MessageText:
 *
 * There are too many public keys in the certificate
 *
 */
#define DRM_E_BCERT_TOO_MANY_PUBLIC_KEYS ((DRM_RESULT)0x8004C83AL)

/*
 * MessageId: DRM_E_BCERT_INVALID_WARNING_DAYS
 *
 * MessageText:
 *
 * An invalid server certificate expiration warning days. Warning days must be greater than zero.
 *
 */
#define DRM_E_BCERT_INVALID_WARNING_DAYS ((DRM_RESULT)0x8004C83CL)

/*
 * MessageId: DRM_E_BCERT_INVALID_DIGEST
 *
 * MessageText:
 *
 * The certificate digest is invalid.
 *
 */
#define DRM_E_BCERT_INVALID_DIGEST       ((DRM_RESULT)0x8004C83DL)

/*
 * MessageId: DRM_E_BCERT_MANUFACTURING_INFO_REQUIRED
 *
 * MessageText:
 *
 * This certificate type requires Manufacturer Name, Model Name, and Model Number to be set.
 *
 */
#define DRM_E_BCERT_MANUFACTURING_INFO_REQUIRED ((DRM_RESULT)0x8004C83EL)


/* ------------------------------------------------------------
**
** XML Signature/Encryption errors: error codes from DRM_E_BASECODE+0x900
** to DRM_E_BASECODE+0x9FF, 0x8004c900-0x8004c9ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_XMLSIG_BASECODE       DRM_E_BASECODE + 0x900

/*
 * MessageId: DRM_E_XMLSIG_ECDSA_VERIFY_FAILURE
 *
 * MessageText:
 *
 * Error in ECDSA signature verification.
 *
 */
#define DRM_E_XMLSIG_ECDSA_VERIFY_FAILURE ((DRM_RESULT)0x8004C900L)

/*
 * MessageId: DRM_E_XMLSIG_SHA_VERIFY_FAILURE
 *
 * MessageText:
 *
 * Error in SHA verification.
 *
 */
#define DRM_E_XMLSIG_SHA_VERIFY_FAILURE  ((DRM_RESULT)0x8004C901L)

/*
 * MessageId: DRM_E_XMLSIG_FORMAT
 *
 * MessageText:
 *
 * The format of XML signature or encryption segment is incorrect.
 *
 */
#define DRM_E_XMLSIG_FORMAT              ((DRM_RESULT)0x8004C902L)

/*
 * MessageId: DRM_E_XMLSIG_PUBLIC_KEY_ID
 *
 * MessageText:
 *
 * Invalud pre-shared public key ID.
 *
 */
#define DRM_E_XMLSIG_PUBLIC_KEY_ID       ((DRM_RESULT)0x8004C903L)

/*
 * MessageId: DRM_E_XMLSIG_INVALID_KEY_FORMAT
 *
 * MessageText:
 *
 * Invalid type of public/private key format.
 *
 */
#define DRM_E_XMLSIG_INVALID_KEY_FORMAT  ((DRM_RESULT)0x8004C904L)

/*
 * MessageId: DRM_E_XMLSIG_SHA_HASH_SIZE
 *
 * MessageText:
 *
 * Size of hash is unexpected.
 *
 */
#define DRM_E_XMLSIG_SHA_HASH_SIZE       ((DRM_RESULT)0x8004C905L)

/*
 * MessageId: DRM_E_XMLSIG_ECDSA_SIGNATURE_SIZE
 *
 * MessageText:
 *
 * Size of ECDSA signature is unexpected.
 *
 */
#define DRM_E_XMLSIG_ECDSA_SIGNATURE_SIZE ((DRM_RESULT)0x8004C906L)


/* ------------------------------------------------------------
**
** UTF8 encoding errors: error codes from DRM_E_BASECODE+0xA00
** to DRM_E_BASECODE+0xAFF, 0x8004ca00-0x8004caff.
**
** ------------------------------------------------------------
*/

#define DRM_E_UTF_BASECODE       DRM_E_BASECODE + 0xa00

/*
 * MessageId: DRM_E_UTF_UNEXPECTED_END
 *
 * MessageText:
 *
 * Unexpected end of data in the middle of multibyte character.
 *
 */
#define DRM_E_UTF_UNEXPECTED_END         ((DRM_RESULT)0x8004CA00L)

/*
 * MessageId: DRM_E_UTF_INVALID_CODE
 *
 * MessageText:
 *
 * UTF character maps into a code with invalid value.
 *
 */
#define DRM_E_UTF_INVALID_CODE           ((DRM_RESULT)0x8004CA01L)


/* ------------------------------------------------------------
**
** XML SOAP errors: error codes from DRM_E_BASECODE+0xB00
** to DRM_E_BASECODE+0xBFF, 0x8004cb00-0x8004cbff.
**
** ------------------------------------------------------------
*/

#define DRM_E_SOAPXML_BASECODE       DRM_E_BASECODE + 0xb00

/*
 * MessageId: DRM_E_SOAPXML_INVALID_STATUS_CODE
 *
 * MessageText:
 *
 * Status code contained in the server error response is invalid.
 *
 */
#define DRM_E_SOAPXML_INVALID_STATUS_CODE ((DRM_RESULT)0x8004CB00L)

/*
 * MessageId: DRM_E_SOAPXML_XML_FORMAT
 *
 * MessageText:
 *
 * Cannot parse out expected XML node.
 *
 */
#define DRM_E_SOAPXML_XML_FORMAT         ((DRM_RESULT)0x8004CB01L)

/*
 * MessageId: DRM_E_SOAPXML_WRONG_MESSAGE_TYPE
 *
 * MessageText:
 *
 * The message type associated with the soap message is wrong.
 *
 */
#define DRM_E_SOAPXML_WRONG_MESSAGE_TYPE ((DRM_RESULT)0x8004CB02L)

/*
 * MessageId: DRM_E_SOAPXML_SIGNATURE_MISSING
 *
 * MessageText:
 *
 * The message did not have a signature and needed one
 *
 */
#define DRM_E_SOAPXML_SIGNATURE_MISSING  ((DRM_RESULT)0x8004CB03L)

/*
 * MessageId: DRM_E_SOAPXML_PROTOCOL_NOT_SUPPORTED
 *
 * MessageText:
 *
 * The requested protocol is not supported by the DRM SOAP parser.
 *
 */
#define DRM_E_SOAPXML_PROTOCOL_NOT_SUPPORTED ((DRM_RESULT)0x8004CB04L)

/*
 * MessageId: DRM_E_SOAPXML_DATA_NOT_FOUND
 *
 * MessageText:
 *
 * The requested data is not found in the response.
 *
 */
#define DRM_E_SOAPXML_DATA_NOT_FOUND     ((DRM_RESULT)0x8004CB05L)


/* ------------------------------------------------------------
**
** Generic crypto errors: error codes from DRM_E_BASECODE+0xC00
** to DRM_E_BASECODE+0xCFF, 0x8004cc00-0x8004ccff.
**
** ------------------------------------------------------------
*/

#define DRM_E_CRYPTO_BASECODE       DRM_E_BASECODE + 0xc00

/*
 * MessageId: DRM_E_CRYPTO_PUBLIC_KEY_NOT_MATCH
 *
 * MessageText:
 *
 * The public key associated with an encrypted domain private from the server does not match any public key on the device.
 *
 */
#define DRM_E_CRYPTO_PUBLIC_KEY_NOT_MATCH ((DRM_RESULT)0x8004CC00L)

/*
 * MessageId: DRM_E_UNABLE_TO_RESOLVE_LOCATION_TREE
 *
 * MessageText:
 *
 * Unable to derive the key.  May be due to blackout or no rights to the service, etc.
 *
 */
#define DRM_E_UNABLE_TO_RESOLVE_LOCATION_TREE ((DRM_RESULT)0x8004CC01L)


/* ------------------------------------------------------------
**
** TEE errors: error codes from DRM_E_BASECODE+0xD10
** to DRM_E_BASECODE+0xDFF, 0x8004cd10-0x8004cdff.
**
** ------------------------------------------------------------
*/

#define DRM_E_TEE_BASECODE       DRM_E_BASECODE + 0xd10

/*
 * MessageId: DRM_E_TEE_INVALID_KEY_DATA
 *
 * MessageText:
 *
 * The key data given to the TEE was invalid.
 *
 */
#define DRM_E_TEE_INVALID_KEY_DATA       ((DRM_RESULT)0x8004CD10L)

/*
 * MessageId: DRM_E_TEE_PROVISIONING_REQUIRED
 *
 * MessageText:
 *
 * Provisioning is required.
 *
 */
#define DRM_E_TEE_PROVISIONING_REQUIRED  ((DRM_RESULT)0x8004CD11L)

/*
 * MessageId: DRM_E_TEE_INVALID_HWDRM_STATE
 *
 * MessageText:
 *
 * The HWDRM state is invalid, e.g. the TEE context is invalid.  Reinitialization is required.
 *
 */
#define DRM_E_TEE_INVALID_HWDRM_STATE    ((DRM_RESULT)0x8004CD12L)

/*
 * MessageId: DRM_E_TEE_PROVISIONING_REQUEST_EXPIRED
 *
 * MessageText:
 *
 * Provisioning request expired.
 *
 */
#define DRM_E_TEE_PROVISIONING_REQUEST_EXPIRED ((DRM_RESULT)0x8004CD13L)

/*
 * MessageId: DRM_E_TEE_CLOCK_NOT_SET
 *
 * MessageText:
 *
 * The TEE secure clock needs to be reset.
 *
 */
#define DRM_E_TEE_CLOCK_NOT_SET          ((DRM_RESULT)0x8004CD14L)

/*
 * MessageId: DRM_E_TEE_BLOB_ACCESS_DENIED
 *
 * MessageText:
 *
 * The blob data is protected and cannot be transfered outside of the TEE.
 *
 */
#define DRM_E_TEE_BLOB_ACCESS_DENIED     ((DRM_RESULT)0x8004CD15L)

/*
 * MessageId: DRM_E_TEE_PROVISIONING_BAD_NONCE
 *
 * MessageText:
 *
 * Malformed nonce
 *
 */
#define DRM_E_TEE_PROVISIONING_BAD_NONCE ((DRM_RESULT)0x8004CD16L)

/*
 * MessageId: DRM_E_TEE_PROVISIONING_NONCE_MISMATCH
 *
 * MessageText:
 *
 * Nonce mismatch. Possibly another request has happened in parallel.
 *
 */
#define DRM_E_TEE_PROVISIONING_NONCE_MISMATCH ((DRM_RESULT)0x8004CD17L)

/*
 * MessageId: DRM_E_TEE_ROOT_KEY_CHANGED
 *
 * MessageText:
 *
 * The root-most TEE key has changed without maintaining key history.  All TEE-bound data is now invalid.
 *
 */
#define DRM_E_TEE_ROOT_KEY_CHANGED       ((DRM_RESULT)0x8004CD18L)

/*
 * MessageId: DRM_E_TEE_PROVISIONING_INVALID_RESPONSE
 *
 * MessageText:
 *
 * Invalid provisioning response.
 *
 */
#define DRM_E_TEE_PROVISIONING_INVALID_RESPONSE ((DRM_RESULT)0x8004CD19L)

/*
 * MessageId: DRM_E_TEE_PROXY_INVALID_SERIALIZATION_MESSAGE
 *
 * MessageText:
 *
 * Invalid TEE proxy serialization message.
 *
 */
#define DRM_E_TEE_PROXY_INVALID_SERIALIZATION_MESSAGE ((DRM_RESULT)0x8004CD1AL)

/*
 * MessageId: DRM_E_TEE_PROXY_INVALID_SERIALIZATION_TYPE
 *
 * MessageText:
 *
 * Invalid TEE proxy serialization type.
 *
 */
#define DRM_E_TEE_PROXY_INVALID_SERIALIZATION_TYPE ((DRM_RESULT)0x8004CD1BL)

/*
 * MessageId: DRM_E_TEE_LAYER_UNINITIALIZED
 *
 * MessageText:
 *
 * TEE Layer is not initialized.
 *
 */
#define DRM_E_TEE_LAYER_UNINITIALIZED    ((DRM_RESULT)0x8004CD1CL)

/*
 * MessageId: DRM_E_TEE_INVALID_HEADER_FOOTER_SIZE
 *
 * MessageText:
 *
 * The OEM defined TEE message header/footer size was not a multiple of 8 bytes.
 *
 */
#define DRM_E_TEE_INVALID_HEADER_FOOTER_SIZE ((DRM_RESULT)0x8004CD1DL)

/*
 * MessageId: DRM_E_TEE_MESSAGE_TOO_LARGE
 *
 * MessageText:
 *
 * TEE method invocation message is too large.
 *
 */
#define DRM_E_TEE_MESSAGE_TOO_LARGE      ((DRM_RESULT)0x8004CD1EL)

/*
 * MessageId: DRM_E_TEE_CLOCK_DRIFTED
 *
 * MessageText:
 *
 * TEE clock drift detected.
 *
 */
#define DRM_E_TEE_CLOCK_DRIFTED          ((DRM_RESULT)0x8004CD1FL)

/*
 * MessageId: DRM_E_TEE_PROXY_INVALID_BUFFER_ALIGNMENT
 *
 * MessageText:
 *
 * The TEE serialization buffer is incorrectly aligned.  It requires 8-byte alignment.
 *
 */
#define DRM_E_TEE_PROXY_INVALID_BUFFER_ALIGNMENT ((DRM_RESULT)0x8004CD20L)

/*
 * MessageId: DRM_E_TEE_PROXY_INVALID_ALIGNMENT
 *
 * MessageText:
 *
 * The TEE serialization buffer has parameters that are not properly aligned.
 *
 */
#define DRM_E_TEE_PROXY_INVALID_ALIGNMENT ((DRM_RESULT)0x8004CD21L)

/*
 * MessageId: DRM_E_TEE_OUTPUT_PROTECTION_REQUIREMENTS_NOT_MET
 *
 * MessageText:
 *
 * The TEE has detected that certain output requirements are not being satisfied. Most commonly HDCP is required but not enabled on all available outputs.
 *
 */
#define DRM_E_TEE_OUTPUT_PROTECTION_REQUIREMENTS_NOT_MET ((DRM_RESULT)0x8004CD22L)


/* ------------------------------------------------------------
**
** Nonce store errors: error codes from DRM_E_BASECODE+0x1000
** to DRM_E_BASECODE+0x10FF, 0x8004d000-0x8004d0ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_NONCE_STORE_BASECODE       DRM_E_BASECODE + 0x1000

/*
 * MessageId: DRM_E_NONCE_STORE_TOKEN_NOT_FOUND
 *
 * MessageText:
 *
 * The matching nonce store token is not found.
 *
 */
#define DRM_E_NONCE_STORE_TOKEN_NOT_FOUND ((DRM_RESULT)0x8004D000L)

/*
 * MessageId: DRM_E_NONCE_STORE_OPEN_STORE
 *
 * MessageText:
 *
 * Fail to open nonce store.
 *
 */
#define DRM_E_NONCE_STORE_OPEN_STORE     ((DRM_RESULT)0x8004D001L)

/*
 * MessageId: DRM_E_NONCE_STORE_CLOSE_STORE
 *
 * MessageText:
 *
 * Fail to close nonce store.
 *
 */
#define DRM_E_NONCE_STORE_CLOSE_STORE    ((DRM_RESULT)0x8004D002L)

/*
 * MessageId: DRM_E_NONCE_STORE_ADD_LICENSE
 *
 * MessageText:
 *
 * There is already a license associated with the nonce store token.
 *
 */
#define DRM_E_NONCE_STORE_ADD_LICENSE    ((DRM_RESULT)0x8004D003L)


/* ------------------------------------------------------------
**
** Policy State errors: error codes from DRM_E_BASECODE+0x1200 to
** DRM_E_BASECODE+0x12FF, 0x8004d200-0x8004d2ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_POLICYSTATE_BASECODE              DRM_E_BASECODE+0x1200

/*
 * MessageId: DRM_E_POLICYSTATE_NOT_FOUND
 *
 * MessageText:
 *
 * The policy state is not found in the secure store.
 *
 */
#define DRM_E_POLICYSTATE_NOT_FOUND      ((DRM_RESULT)0x8004D200L)

/*
 * MessageId: DRM_E_POLICYSTATE_CORRUPTED
 *
 * MessageText:
 *
 * The policy state is not stored as a valid internal format in the secure store.
 *
 */
#define DRM_E_POLICYSTATE_CORRUPTED      ((DRM_RESULT)0x8004D201L)


/* ------------------------------------------------------------
**
** Extensible Binary errors: error codes from DRM_E_BASECODE+0x1400 to
** DRM_E_BASECODE+0x141F, 0x8004d400-0x8004d41f.
**
** ------------------------------------------------------------
*/

#define DRM_E_XB_BASECODE              DRM_E_BASECODE+0x1400
#define DRM_E_XB_MAXCODE               DRM_E_BASECODE+0x141F
#define DRM_IS_XB_ERROR( __dr )                                                                        \
    ( ( ( __dr ) >= MAKE_DRM_RESULT( DRM_SEVERITY_ERROR, DRM_FACILITY_CORE, DRM_E_XB_BASECODE ) )      \
   && ( ( __dr ) <= MAKE_DRM_RESULT( DRM_SEVERITY_ERROR, DRM_FACILITY_CORE, DRM_E_XB_MAXCODE ) ) )

/*
 * MessageId: DRM_E_XB_OBJECT_NOTFOUND
 *
 * MessageText:
 *
 * The extensible binary object was not found.
 *
 */
#define DRM_E_XB_OBJECT_NOTFOUND         ((DRM_RESULT)0x8004D400L)

/*
 * MessageId: DRM_E_XB_INVALID_OBJECT
 *
 * MessageText:
 *
 * The extensible binary object format was invalid.
 *
 */
#define DRM_E_XB_INVALID_OBJECT          ((DRM_RESULT)0x8004D401L)

/*
 * MessageId: DRM_E_XB_OBJECT_ALREADY_EXISTS
 *
 * MessageText:
 *
 * A single instance extensible binary object was encountered more than once.
 *
 */
#define DRM_E_XB_OBJECT_ALREADY_EXISTS   ((DRM_RESULT)0x8004D402L)

/*
 * MessageId: DRM_E_XB_REQUIRED_OBJECT_MISSING
 *
 * MessageText:
 *
 * A required extensible binary object was not found during building.
 *
 */
#define DRM_E_XB_REQUIRED_OBJECT_MISSING ((DRM_RESULT)0x8004D403L)

/*
 * MessageId: DRM_E_XB_UNKNOWN_ELEMENT_TYPE
 *
 * MessageText:
 *
 * An extensible binary object description contained an element of an unknown type.
 *
 */
#define DRM_E_XB_UNKNOWN_ELEMENT_TYPE    ((DRM_RESULT)0x8004D404L)

/*
 * MessageId: DRM_E_XB_INVALID_VERSION
 *
 * MessageText:
 *
 * The serialized object version could not be found in the extensible binary object description.
 *
 */
#define DRM_E_XB_INVALID_VERSION         ((DRM_RESULT)0x8004D405L)

/*
 * MessageId: DRM_E_XB_MAX_UNKNOWN_CONTAINER_DEPTH
 *
 * MessageText:
 *
 * The maximum unknown container depth was reached.
 *
 */
#define DRM_E_XB_MAX_UNKNOWN_CONTAINER_DEPTH ((DRM_RESULT)0x8004D406L)

/*
 * MessageId: DRM_E_XB_INVALID_ALIGNMENT
 *
 * MessageText:
 *
 * The serialized message buffer is not properly aligned according to the XBinary format description.
 *
 */
#define DRM_E_XB_INVALID_ALIGNMENT       ((DRM_RESULT)0x8004D407L)

/*
 * MessageId: DRM_E_XB_OBJECT_OUT_OF_RANGE
 *
 * MessageText:
 *
 * An extensible binary object size or count is out of the range specified by the attributes 'MinSize' and 'MaxSize'.
 *
 */
#define DRM_E_XB_OBJECT_OUT_OF_RANGE     ((DRM_RESULT)0x8004D408L)

/* ------------------------------------------------------------
**
** Available range 0x8004d420-0x8004d4ff.
**
** ------------------------------------------------------------
*/

/* ------------------------------------------------------------
** DRM_E_BASECODE + 0x1600 - DRM_E_BASECODE + 0x16ff (0x8004d600-0x8004d6ff)
** are reserved for additional DRM Services error codes.
**
** See source\common\services\inc\svcerrors.h for Services error codes.
**
** ------------------------------------------------------------
*/

#define DRM_E_SERVICES_BASECODE_EX     (DRM_E_BASECODE + 0x1600)


/* ------------------------------------------------------------
**
** LicGen errors: error codes from DRM_E_BASECODE + 0x1900 to
** DRM_E_BASECODE + 0x19ff, 0x8004d900-0x8004d9ff.
**
** ------------------------------------------------------------
*/

#define DRM_E_LICGEN_BASECODE          (DRM_E_BASECODE + 0x1900)

/*
 * MessageId: DRM_E_LICGEN_CANNOT_PERSIST_LICENSE
 *
 * MessageText:
 *
 * A non-persistent license cannot be stored in the license store.
 *
 */
#define DRM_E_LICGEN_CANNOT_PERSIST_LICENSE ((DRM_RESULT)0x8004D901L)

/*
 * MessageId: DRM_E_LICGEN_ROOT_LICENSE_CANNOT_ENCRYPT
 *
 * MessageText:
 *
 * A root license should not be used to encrypt content.
 *
 */
#define DRM_E_LICGEN_ROOT_LICENSE_CANNOT_ENCRYPT ((DRM_RESULT)0x8004D904L)

/*
 * MessageId: DRM_E_LICGEN_EMBED_LOCAL_LICENSE
 *
 * MessageText:
 *
 * A local bound license cannot be embedded.
 *
 */
#define DRM_E_LICGEN_EMBED_LOCAL_LICENSE ((DRM_RESULT)0x8004D905L)

/*
 * MessageId: DRM_E_LICGEN_DUPLICATE_PLAY_ENABLER
 *
 * MessageText:
 *
 * A license descriptor contains a duplicate play enabler.
 *
 */
#define DRM_E_LICGEN_DUPLICATE_PLAY_ENABLER ((DRM_RESULT)0x8004D908L)

/*
 * MessageId: DRM_E_LICGEN_CHILD_SECURITY_LEVEL_TOO_LOW
 *
 * MessageText:
 *
 * The security level of the chained license is too low.
 *
 */
#define DRM_E_LICGEN_CHILD_SECURITY_LEVEL_TOO_LOW ((DRM_RESULT)0x8004D909L)

/* ------------------------------------------------------------
**
** H264 errors: error codes from DRM_E_BASECODE + 0x1A00 to
** DRM_E_BASECODE + 0x1Aff, 0x8004da00-0x8004daff.
**
** ------------------------------------------------------------
*/
#define DRM_E_H264_BASECODE      (DRM_E_BASECODE + 0x1A00)
#define DRM_E_H264_FINALCODE     (DRM_E_BASECODE + 0x1AFF)

/*
 * MessageId: DRM_E_H264_PARSING_FAILED
 *
 * MessageText:
 *
 * The H264 was unable to be parsed.
 *
 */
#define DRM_E_H264_PARSING_FAILED        ((DRM_RESULT)0x8004DA00L)

/*
 * MessageId: DRM_E_H264_SPS_PROFILE
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_PROFILE           ((DRM_RESULT)0x8004DA01L)


#define DRM_E_H264_MINIMUM DRM_E_H264_SPS_PROFILE

/*
 * MessageId: DRM_E_H264_SPS_IDC
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_IDC               ((DRM_RESULT)0x8004DA02L)

/*
 * MessageId: DRM_E_H264_SPS_SPSID
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_SPSID             ((DRM_RESULT)0x8004DA03L)

/*
 * MessageId: DRM_E_H264_SPS_FRAMENUM
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_FRAMENUM          ((DRM_RESULT)0x8004DA04L)

/*
 * MessageId: DRM_E_H264_SPS_POCTYPE
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_POCTYPE           ((DRM_RESULT)0x8004DA05L)

/*
 * MessageId: DRM_E_H264_SPS_POCLSB
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_POCLSB            ((DRM_RESULT)0x8004DA06L)

/*
 * MessageId: DRM_E_H264_SPS_POCCYCLE
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_POCCYCLE          ((DRM_RESULT)0x8004DA07L)

/*
 * MessageId: DRM_E_H264_SPS_NUMREFFRAMES
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_NUMREFFRAMES      ((DRM_RESULT)0x8004DA08L)

/*
 * MessageId: DRM_E_H264_SPS_CHROMATOP
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_CHROMATOP         ((DRM_RESULT)0x8004DA09L)

/*
 * MessageId: DRM_E_H264_SPS_CHROMABOTTOM
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_CHROMABOTTOM      ((DRM_RESULT)0x8004DA0AL)

/*
 * MessageId: DRM_E_H264_SPS_NALHRD
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_NALHRD            ((DRM_RESULT)0x8004DA0BL)

/*
 * MessageId: DRM_E_H264_SPS_VLDHRD
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VLDHRD            ((DRM_RESULT)0x8004DA0CL)

/*
 * MessageId: DRM_E_H264_SPS_VUIBPPD
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VUIBPPD           ((DRM_RESULT)0x8004DA0DL)

/*
 * MessageId: DRM_E_H264_SPS_VUIBPMD
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VUIBPMD           ((DRM_RESULT)0x8004DA0EL)

/*
 * MessageId: DRM_E_H264_SPS_VUIMMLH
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VUIMMLH           ((DRM_RESULT)0x8004DA0FL)

/*
 * MessageId: DRM_E_H264_SPS_VUIMMLV
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VUIMMLV           ((DRM_RESULT)0x8004DA10L)

/*
 * MessageId: DRM_E_H264_SPS_VUINRF
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VUINRF            ((DRM_RESULT)0x8004DA11L)

/*
 * MessageId: DRM_E_H264_SPS_VUIMDFB
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_VUIMDFB           ((DRM_RESULT)0x8004DA12L)

/*
 * MessageId: DRM_E_H264_SPS_WIDTH_HEIGHT
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_WIDTH_HEIGHT      ((DRM_RESULT)0x8004DA13L)

/*
 * MessageId: DRM_E_H264_SPS_AREA
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_AREA              ((DRM_RESULT)0x8004DA14L)

/*
 * MessageId: DRM_E_H264_SPS_MINHEIGHT2
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_MINHEIGHT2        ((DRM_RESULT)0x8004DA15L)

/*
 * MessageId: DRM_E_H264_SPS_MINHEIGHT3
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_MINHEIGHT3        ((DRM_RESULT)0x8004DA16L)

/*
 * MessageId: DRM_E_H264_SPS_CROPWIDTH
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_CROPWIDTH         ((DRM_RESULT)0x8004DA17L)

/*
 * MessageId: DRM_E_H264_SPS_CROPHEIGHT
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_CROPHEIGHT        ((DRM_RESULT)0x8004DA18L)

/*
 * MessageId: DRM_E_H264_SPS_MORE_RBSP
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_MORE_RBSP         ((DRM_RESULT)0x8004DA19L)

/*
 * MessageId: DRM_E_H264_SPS_CHROMA_IDC
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_CHROMA_IDC        ((DRM_RESULT)0x8004DA1AL)

/*
 * MessageId: DRM_E_H264_SPS_BITDEPTHLUMA
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_BITDEPTHLUMA      ((DRM_RESULT)0x8004DA1BL)

/*
 * MessageId: DRM_E_H264_SPS_BITDEPTHCHROMA
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_BITDEPTHCHROMA    ((DRM_RESULT)0x8004DA1CL)

/*
 * MessageId: DRM_E_H264_SPS_DELTASCALE1
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_DELTASCALE1       ((DRM_RESULT)0x8004DA1DL)

/*
 * MessageId: DRM_E_H264_SPS_DELTASCALE2
 *
 * MessageText:
 *
 * SPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_SPS_DELTASCALE2       ((DRM_RESULT)0x8004DA1EL)

/*
 * MessageId: DRM_E_H264_BITSTREAM_TOOMANY
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_TOOMANY     ((DRM_RESULT)0x8004DA30L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_TOOSHORT1
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_TOOSHORT1   ((DRM_RESULT)0x8004DA31L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_TOOSHORT2
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_TOOSHORT2   ((DRM_RESULT)0x8004DA32L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_TOOSHORT3
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_TOOSHORT3   ((DRM_RESULT)0x8004DA33L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_TOOSHORT4
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_TOOSHORT4   ((DRM_RESULT)0x8004DA34L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_TOOSHORT5
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_TOOSHORT5   ((DRM_RESULT)0x8004DA35L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_EXGOLOBMTOOLONG1
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_EXGOLOBMTOOLONG1 ((DRM_RESULT)0x8004DA36L)

/*
 * MessageId: DRM_E_H264_BITSTREAM_EXGOLOBMTOOLONG2
 *
 * MessageText:
 *
 * Bitstream-specific H264 parsing error
 *
 */
#define DRM_E_H264_BITSTREAM_EXGOLOBMTOOLONG2 ((DRM_RESULT)0x8004DA37L)

/*
 * MessageId: DRM_E_H264_NALU_NO_START_CODE
 *
 * MessageText:
 *
 * Nalu-specific H264 parsing error
 *
 */
#define DRM_E_H264_NALU_NO_START_CODE    ((DRM_RESULT)0x8004DA40L)

/*
 * MessageId: DRM_E_H264_NALU_ALL_ZERO
 *
 * MessageText:
 *
 * Nalu-specific H264 parsing error
 *
 */
#define DRM_E_H264_NALU_ALL_ZERO         ((DRM_RESULT)0x8004DA41L)

/*
 * MessageId: DRM_E_H264_NALU_EMULATION
 *
 * MessageText:
 *
 * Nalu-specific H264 parsing error
 *
 */
#define DRM_E_H264_NALU_EMULATION        ((DRM_RESULT)0x8004DA42L)

/*
 * MessageId: DRM_E_H264_PPS_PPSID
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_PPSID             ((DRM_RESULT)0x8004DA50L)

/*
 * MessageId: DRM_E_H264_PPS_SPSID
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SPSID             ((DRM_RESULT)0x8004DA51L)

/*
 * MessageId: DRM_E_H264_PPS_SPS_NOT_FOUND
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SPS_NOT_FOUND     ((DRM_RESULT)0x8004DA52L)

/*
 * MessageId: DRM_E_H264_PPS_NUM_SLICE_GROUPS
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_NUM_SLICE_GROUPS  ((DRM_RESULT)0x8004DA53L)

/*
 * MessageId: DRM_E_H264_PPS_SLICE_GROUP_MAX
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SLICE_GROUP_MAX   ((DRM_RESULT)0x8004DA54L)

/*
 * MessageId: DRM_E_H264_PPS_RUN_LENGTH
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_RUN_LENGTH        ((DRM_RESULT)0x8004DA55L)

/*
 * MessageId: DRM_E_H264_PPS_TOP_LEFT
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_TOP_LEFT          ((DRM_RESULT)0x8004DA56L)

/*
 * MessageId: DRM_E_H264_PPS_SLICE_GROUP_RATE
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SLICE_GROUP_RATE  ((DRM_RESULT)0x8004DA57L)

/*
 * MessageId: DRM_E_H264_PPS_SLICE_GROUP_MAP
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SLICE_GROUP_MAP   ((DRM_RESULT)0x8004DA58L)

/*
 * MessageId: DRM_E_H264_PPS_SLICE_GROUP_ID
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SLICE_GROUP_ID    ((DRM_RESULT)0x8004DA59L)

/*
 * MessageId: DRM_E_H264_PPS_REF_IDX_L0
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_REF_IDX_L0        ((DRM_RESULT)0x8004DA5AL)

/*
 * MessageId: DRM_E_H264_PPS_REF_IDX_L1
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_REF_IDX_L1        ((DRM_RESULT)0x8004DA5BL)

/*
 * MessageId: DRM_E_H264_PPS_WEIGHTED_BIPRED
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_WEIGHTED_BIPRED   ((DRM_RESULT)0x8004DA5CL)

/*
 * MessageId: DRM_E_H264_PPS_PIC_INIT_QP
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_PIC_INIT_QP       ((DRM_RESULT)0x8004DA5DL)

/*
 * MessageId: DRM_E_H264_PPS_PIC_INIT_QS
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_PIC_INIT_QS       ((DRM_RESULT)0x8004DA5EL)

/*
 * MessageId: DRM_E_H264_PPS_PIC_CHROMA_QP
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_PIC_CHROMA_QP     ((DRM_RESULT)0x8004DA5FL)

/*
 * MessageId: DRM_E_H264_PPS_REDUN_PIC_COUNT
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_REDUN_PIC_COUNT   ((DRM_RESULT)0x8004DA61L)

/*
 * MessageId: DRM_E_H264_PPS_DELTA_SCALE1
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_DELTA_SCALE1      ((DRM_RESULT)0x8004DA62L)

/*
 * MessageId: DRM_E_H264_PPS_DELTA_SCALE2
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_DELTA_SCALE2      ((DRM_RESULT)0x8004DA63L)

/*
 * MessageId: DRM_E_H264_PPS_SECOND_CHROMA_QP
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_SECOND_CHROMA_QP  ((DRM_RESULT)0x8004DA64L)

/*
 * MessageId: DRM_E_H264_PPS_MORE_RBSP
 *
 * MessageText:
 *
 * PPS-specific H264 parsing error
 *
 */
#define DRM_E_H264_PPS_MORE_RBSP         ((DRM_RESULT)0x8004DA65L)

/*
 * MessageId: DRM_E_H264_SH_SLICE_TYPE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_SLICE_TYPE         ((DRM_RESULT)0x8004DA70L)

/*
 * MessageId: DRM_E_H264_SH_SLICE_TYPE_UNSUPPORTED
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_SLICE_TYPE_UNSUPPORTED ((DRM_RESULT)0x8004DA71L)

/*
 * MessageId: DRM_E_H264_SH_PPSID
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_PPSID              ((DRM_RESULT)0x8004DA72L)

/*
 * MessageId: DRM_E_H264_SH_PPS_NOT_FOUND
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_PPS_NOT_FOUND      ((DRM_RESULT)0x8004DA73L)

/*
 * MessageId: DRM_E_H264_SH_SPS_NOT_FOUND
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_SPS_NOT_FOUND      ((DRM_RESULT)0x8004DA74L)

/*
 * MessageId: DRM_E_H264_SH_SLICE_TYPE_PROFILE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_SLICE_TYPE_PROFILE ((DRM_RESULT)0x8004DA75L)

/*
 * MessageId: DRM_E_H264_SH_IDR_FRAME_NUM
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_IDR_FRAME_NUM      ((DRM_RESULT)0x8004DA76L)

/*
 * MessageId: DRM_E_H264_SH_FIRST_MB_IN_SLICE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_FIRST_MB_IN_SLICE  ((DRM_RESULT)0x8004DA77L)

/*
 * MessageId: DRM_E_H264_SH_IDR_PIC_ID
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_IDR_PIC_ID         ((DRM_RESULT)0x8004DA78L)

/*
 * MessageId: DRM_E_H264_SH_REDUN_PIC_COUNT
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_REDUN_PIC_COUNT    ((DRM_RESULT)0x8004DA79L)

/*
 * MessageId: DRM_E_H264_SH_NUM_REF_IDX_LX0
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_NUM_REF_IDX_LX0    ((DRM_RESULT)0x8004DA7AL)

/*
 * MessageId: DRM_E_H264_SH_NUM_REF_IDX_LX1
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_NUM_REF_IDX_LX1    ((DRM_RESULT)0x8004DA7BL)

/*
 * MessageId: DRM_E_H264_SH_REF_PIC_LIST_REORDER0
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_REF_PIC_LIST_REORDER0 ((DRM_RESULT)0x8004DA7CL)

/*
 * MessageId: DRM_E_H264_SH_REF_PIC_LIST_REORDER1
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_REF_PIC_LIST_REORDER1 ((DRM_RESULT)0x8004DA7DL)

/*
 * MessageId: DRM_E_H264_SH_LUMA_WEIGHT_DENOM
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_LUMA_WEIGHT_DENOM  ((DRM_RESULT)0x8004DA7EL)

/*
 * MessageId: DRM_E_H264_SH_CHROMA_WEIGHT_DENOM
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_CHROMA_WEIGHT_DENOM ((DRM_RESULT)0x8004DA7FL)

/*
 * MessageId: DRM_E_H264_SH_WP_WEIGHT_LUMA0
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_WEIGHT_LUMA0    ((DRM_RESULT)0x8004DA80L)

/*
 * MessageId: DRM_E_H264_SH_WP_OFFSET_LUMA0
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_OFFSET_LUMA0    ((DRM_RESULT)0x8004DA81L)

/*
 * MessageId: DRM_E_H264_SH_WP_WEIGHT_CHROMA0
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_WEIGHT_CHROMA0  ((DRM_RESULT)0x8004DA82L)

/*
 * MessageId: DRM_E_H264_SH_WP_OFFSET_CHROMA0
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_OFFSET_CHROMA0  ((DRM_RESULT)0x8004DA83L)

/*
 * MessageId: DRM_E_H264_SH_WP_WEIGHT_LUMA1
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_WEIGHT_LUMA1    ((DRM_RESULT)0x8004DA84L)

/*
 * MessageId: DRM_E_H264_SH_WP_OFFSET_LUMA1
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_OFFSET_LUMA1    ((DRM_RESULT)0x8004DA85L)

/*
 * MessageId: DRM_E_H264_SH_WP_WEIGHT_CHROMA1
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_WEIGHT_CHROMA1  ((DRM_RESULT)0x8004DA86L)

/*
 * MessageId: DRM_E_H264_SH_WP_OFFSET_CHROMA1
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_WP_OFFSET_CHROMA1  ((DRM_RESULT)0x8004DA87L)

/*
 * MessageId: DRM_E_H264_SH_NUM_REF_PIC_MARKING
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_NUM_REF_PIC_MARKING ((DRM_RESULT)0x8004DA88L)

/*
 * MessageId: DRM_E_H264_SH_MMCO4_DUPLICATE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MMCO4_DUPLICATE    ((DRM_RESULT)0x8004DA89L)

/*
 * MessageId: DRM_E_H264_SH_MMCO4_MAX_LONG_TERM_FRAME
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MMCO4_MAX_LONG_TERM_FRAME ((DRM_RESULT)0x8004DA8AL)

/*
 * MessageId: DRM_E_H264_SH_MMCO5_DUPLICATE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MMCO5_DUPLICATE    ((DRM_RESULT)0x8004DA8BL)

/*
 * MessageId: DRM_E_H264_SH_MMCO5_FOLLOWS_MMC06
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MMCO5_FOLLOWS_MMC06 ((DRM_RESULT)0x8004DA8CL)

/*
 * MessageId: DRM_E_H264_SH_MMCO5_COEXIST_MMCO_1_OR_3
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MMCO5_COEXIST_MMCO_1_OR_3 ((DRM_RESULT)0x8004DA8DL)

/*
 * MessageId: DRM_E_H264_SH_MMCO6_DUPLICATE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MMCO6_DUPLICATE    ((DRM_RESULT)0x8004DA8EL)

/*
 * MessageId: DRM_E_H264_SH_MODEL_NUMBER
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_MODEL_NUMBER       ((DRM_RESULT)0x8004DA8FL)

/*
 * MessageId: DRM_E_H264_SH_SLICE_QP
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_SLICE_QP           ((DRM_RESULT)0x8004DA90L)

/*
 * MessageId: DRM_E_H264_SH_LF_ALPHA_C0_OFFSET
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_LF_ALPHA_C0_OFFSET ((DRM_RESULT)0x8004DA91L)

/*
 * MessageId: DRM_E_H264_SH_LF_BETA_OFFSET
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_LF_BETA_OFFSET     ((DRM_RESULT)0x8004DA92L)

/*
 * MessageId: DRM_E_H264_SH_SLICE_GROUP_CHANGE
 *
 * MessageText:
 *
 * Slice-Header-specific H264 parsing error
 *
 */
#define DRM_E_H264_SH_SLICE_GROUP_CHANGE ((DRM_RESULT)0x8004DA93L)


#define DRM_E_H264_MAXIMUM DRM_E_H264_SH_SLICE_GROUP_CHANGE

/* ------------------------------------------------------------
**
** Provisioning errors: error codes from DRM_E_BASECODE + 0x1B00 to
** DRM_E_BASECODE + 0x1Bff, 0x8004db00-0x8004dbff.
**
** ------------------------------------------------------------
*/
#define DRM_E_RPROV_BASECODE      (DRM_E_BASECODE + 0x1B00)
#define DRM_E_RPROV_FINALCODE     (DRM_E_BASECODE + 0x1BFF)

/*
 * MessageId: DRM_E_RPROV_INVALID_REQUEST
 *
 * MessageText:
 *
 * Invalid Remote provisioning request received.
 *
 */
#define DRM_E_RPROV_INVALID_REQUEST      ((DRM_RESULT)0x8004DB00L)

/*
 * MessageId: DRM_E_RPROV_VERSION_MISSMATCH
 *
 * MessageText:
 *
 * Invalid Remote provisioning version received.
 *
 */
#define DRM_E_RPROV_VERSION_MISSMATCH    ((DRM_RESULT)0x8004DB01L)

/*
 * MessageId: DRM_E_RPROV_INVALID_RESPONSE
 *
 * MessageText:
 *
 * Invalid response received.
 *
 */
#define DRM_E_RPROV_INVALID_RESPONSE     ((DRM_RESULT)0x8004DB02L)

/*
 * MessageId: DRM_E_RPROV_BOOTSTRAP_FAILURE
 *
 * MessageText:
 *
 * Remote provisioning bootstrap failed.
 *
 */
#define DRM_E_RPROV_BOOTSTRAP_FAILURE    ((DRM_RESULT)0x8004DB03L)

/*
 * MessageId: DRM_E_FIRMWARE_REVOKED
 *
 * MessageText:
 *
 * TEE Firmware is revoked; firmware update necessary.
 *
 */
#define DRM_E_FIRMWARE_REVOKED           ((DRM_RESULT)0x8004DB04L)

/*
 * MessageId: DRM_E_RPROV_SKIP_BOOTSTRAP
 *
 * MessageText:
 *
 * Remote provisioning does not need bootstrap.
 *
 */
#define DRM_E_RPROV_SKIP_BOOTSTRAP       ((DRM_RESULT)0x8004DB05L)


/* ============================================================
**
** Secure stop errors: error codes from DRM_E_BASECODE + 0x1C00 to
** DRM_E_BASECODE + 0x1CFF, 0x8004dc00-0x8004dc7f.
**
** ============================================================
*/

#define DRM_E_SECURESTOP_BASECODE  (DRM_E_BASECODE + 0x1C00)
#define DRM_E_SECURESTOP_FINALCODE (DRM_E_BASECODE + 0x1C7F)

/*
 * MessageId: DRM_E_SECURESTOP_STORE_CORRUPT
 *
 * MessageText:
 *
 * The secure stop store is corrupted.
 *
 */
#define DRM_E_SECURESTOP_STORE_CORRUPT   ((DRM_RESULT)0x8004DC00L)

/*
 * MessageId: DRM_E_SECURESTOP_SESSION_LOCKED
 *
 * MessageText:
 *
 * The secure stop session is locked and may not be modified.
 *
 */
#define DRM_E_SECURESTOP_SESSION_LOCKED  ((DRM_RESULT)0x8004DC02L)

/*
 * MessageId: DRM_E_SECURESTOP_SESSION_CORRUPT
 *
 * MessageText:
 *
 * The secure stop session data is corrupted.
 *
 */
#define DRM_E_SECURESTOP_SESSION_CORRUPT ((DRM_RESULT)0x8004DC03L)

/*
 * MessageId: DRM_E_SECURESTOP_SESSION_ACTIVE
 *
 * MessageText:
 *
 * The secure stop session is active and cannot be locked.
 *
 */
#define DRM_E_SECURESTOP_SESSION_ACTIVE  ((DRM_RESULT)0x8004DC04L)

/*
 * MessageId: DRM_E_SECURESTOP_SESSION_NOT_FOUND
 *
 * MessageText:
 *
 * The secure stop session could not be found in the data store.
 *
 */
#define DRM_E_SECURESTOP_SESSION_NOT_FOUND ((DRM_RESULT)0x8004DC05L)

/*
 * MessageId: DRM_E_SECURESTOP_INVALID_RESPONSE
 *
 * MessageText:
 *
 * The secure stop response is invalid.
 *
 */
#define DRM_E_SECURESTOP_INVALID_RESPONSE ((DRM_RESULT)0x8004DC06L)

/*
 * MessageId: DRM_E_SECURESTOP_SESSION_STOPPED
 *
 * MessageText:
 *
 * The secure stop session is stopped and may not be used for decryption.
 *
 */
#define DRM_E_SECURESTOP_SESSION_STOPPED ((DRM_RESULT)0x8004DC07L)

/*
 * MessageId: DRM_E_SECURESTOP_INVALID_PUBLISHER_ID
 *
 * MessageText:
 *
 * Trying to generate a challenge with a publisher ID that doesn't match the one associated with the session.
 *
 */
#define DRM_E_SECURESTOP_INVALID_PUBLISHER_ID ((DRM_RESULT)0x8004DC08L)

/*
 * MessageId: DRM_E_SECURESTOP_PUBLISHER_ID_INCONSISTENT
 *
 * MessageText:
 *
 * Licenses acquired within the same session don't have the same secure stop publisher ID.
 *
 */
#define DRM_E_SECURESTOP_PUBLISHER_ID_INCONSISTENT ((DRM_RESULT)0x8004DC09L)

/*
 * MessageId: DRM_E_SECURESTOP_INCONSISTENT
 *
 * MessageText:
 *
 * Some licenses acquired within the same session have secure stop while others don't.
 *
 */
#define DRM_E_SECURESTOP_INCONSISTENT    ((DRM_RESULT)0x8004DC0AL)

/* ============================================================
**
** 0x8004dc80 to 0x8004ddff are reserved for OEM-defined errors
**
** ============================================================
*/

/* Nothing should be added here - Reserved for OEM error codes. */


/* ============================================================
**
** Secure time errors: error codes from DRM_E_BASECODE + 0x1E00 to
** DRM_E_BASECODE + 0x1EFF, 0x8004de00-0x8004deff.
**
** ============================================================
*/

#define DRM_E_SECURETIME_BASECODE  (DRM_E_BASECODE + 0x1E00)
#define DRM_E_SECURETIME_FINALCODE (DRM_E_BASECODE + 0x1EFF)

/*
 * MessageId: DRM_E_SECURETIME_INVALID_REQUEST_DATA
 *
 * MessageText:
 *
 * The secure time client request data is invalid.
 *
 */
#define DRM_E_SECURETIME_INVALID_REQUEST_DATA ((DRM_RESULT)0x8004DE00L)

/*
 * MessageId: DRM_E_SECURETIME_CLOCK_NOT_SET
 *
 * MessageText:
 *
 * The secure time clock has not been set.
 *
 */
#define DRM_E_SECURETIME_CLOCK_NOT_SET   ((DRM_RESULT)0x8004DE01L)

/*
 * MessageId: DRM_E_SECURETIME_RESPONSE_TIMEOUT
 *
 * MessageText:
 *
 * The secure time server response timed out.
 *
 */
#define DRM_E_SECURETIME_RESPONSE_TIMEOUT ((DRM_RESULT)0x8004DE02L)

/*
 * MessageId: DRM_E_SECURETIME_SERVER_SECURITY_LEVEL_TOO_LOW
 *
 * MessageText:
 *
 * The secure time server's security level is too low for the client.
 *
 */
#define DRM_E_SECURETIME_SERVER_SECURITY_LEVEL_TOO_LOW ((DRM_RESULT)0x8004DE03L)

/*
 * MessageId: DRM_E_LICENSESERVERTIME_MUST_REACQUIRE_LICENSE
 *
 * MessageText:
 *
 * This license was acquired before the LicenseServerTime feature was enabled.  It must be reacquired.
 *
 */
#define DRM_E_LICENSESERVERTIME_MUST_REACQUIRE_LICENSE ((DRM_RESULT)0x8004DE04L)


/* ============================================================
**
** LSRD errors: error codes from DRM_E_BASECODE + 0x1F00 to
** DRM_E_BASECODE + 0x1F0F, 0x8004df00-0x8004df0f.
**
** ============================================================
*/

#define DRM_E_LSRD_BASECODE  (DRM_E_BASECODE + 0x1F00)
#define DRM_E_LSRD_FINALCODE (DRM_E_BASECODE + 0x1F0F)

/*
 * MessageId: DRM_E_LSRD_DETECTED
 *
 * MessageText:
 *
 * HDS file rollback is detected.
 *
 */
#define DRM_E_LSRD_DETECTED              ((DRM_RESULT)0x8004DF00L)

/*
 * MessageId: DRM_E_LSRD_INVALID_ACL
 *
 * MessageText:
 *
 * The ACL of the HDS Registry Subkey is invalid.
 *
 */
#define DRM_E_LSRD_INVALID_ACL           ((DRM_RESULT)0x8004DF01L)

/*
 * MessageId: DRM_E_LSRD_DETECTION_IN_PROGRESS
 *
 * MessageText:
 *
 * The client is currently processing LSRD check operation. Concurrent operations are not allowed.
 *
 */
#define DRM_E_LSRD_DETECTION_IN_PROGRESS ((DRM_RESULT)0x8004DF02L)

/*
 * MessageId: DRM_E_LSRD_ACL_NOT_PRESENT
 *
 * MessageText:
 *
 * The security descriptor does not contain an ACL.
 *
 */
#define DRM_E_LSRD_ACL_NOT_PRESENT       ((DRM_RESULT)0x8004DF03L)

/*
 * MessageId: DRM_E_LSRD_INVALID_COMMAND
 *
 * MessageText:
 *
 * The Process received an invalid command.
 *
 */
#define DRM_E_LSRD_INVALID_COMMAND       ((DRM_RESULT)0x8004DF04L)

/*
 * MessageId: DRM_E_LSRD_SEQUENCE_NUMBER_IS_AT_MAX_LIMIT
 *
 * MessageText:
 *
 * The LSRD sequence number has reached its maximum limit.
 *
 */
#define DRM_E_LSRD_SEQUENCE_NUMBER_IS_AT_MAX_LIMIT ((DRM_RESULT)0x8004DF05L)


/* ------------------------------------------------------------
**
** Available range 0x8004df10-0x8004df9f.
**
** ------------------------------------------------------------
*/

/* ============================================================
**
** Secure Delete errors: error codes from DRM_E_BASECODE + 0x1FA0 to
** DRM_E_BASECODE + 0x1FAF, 0x8004dfa0-0x8004dfaf.
**
** ============================================================
*/

#define DRM_E_SECUREDELETE_BASECODE  (DRM_E_BASECODE + 0x1FA0)
#define DRM_E_SECUREDELETE_FINALCODE (DRM_E_BASECODE + 0x1FAF)

/*
 * MessageId: DRM_E_SECUREDELETE_INVALID_RESPONSE
 *
 * MessageText:
 *
 * The secure delete response is invalid.
 *
 */
#define DRM_E_SECUREDELETE_INVALID_RESPONSE ((DRM_RESULT)0x8004DFA0L)


/* ============================================================
**
** Provenance errors: error codes from DRM_E_BASECODE + 0x1FB0 to
** DRM_E_BASECODE + 0x1FCF, 0x8004dfb0-0x8004dfcf.
**
** ============================================================
*/

#define DRM_E_PROVENANCE_BASECODE  (DRM_E_BASECODE + 0x1FB0)
#define DRM_E_PROVENANCE_FINALCODE (DRM_E_BASECODE + 0x1FCF)

/*
 * MessageId: DRM_E_PROVENANCE_VALIDATION_FAILED
 *
 * MessageText:
 *
 * The provenance validation failed. The media file has been tampered with.
 *
 */
#define DRM_E_PROVENANCE_VALIDATION_FAILED ((DRM_RESULT)0x8004DFB0L)

/*
 * MessageId: DRM_E_INVALID_PROVENANCE_MANIFEST
 *
 * MessageText:
 *
 * The provenance manifest is invalid.
 *
 */
#define DRM_E_INVALID_PROVENANCE_MANIFEST ((DRM_RESULT)0x8004DFB1L)

/*
 * MessageId: DRM_E_INVALID_PROVENANCE_CERTIFICATE_CHAIN
 *
 * MessageText:
 *
 * The provenance certificate chain stored in the manifest is invalid or a valid certificate chain could not be established.
 *
 */
#define DRM_E_INVALID_PROVENANCE_CERTIFICATE_CHAIN ((DRM_RESULT)0x8004DFB2L)

/*
 * MessageId: DRM_E_PROVENANCE_UNTRUSTED_ROOT_CERTIFICATE
 *
 * MessageText:
 *
 * The provenance certificate chain could not be validated because no root certificates were provided in the trusted list.
 *
 */
#define DRM_E_PROVENANCE_UNTRUSTED_ROOT_CERTIFICATE ((DRM_RESULT)0x8004DFB3L)

/*
 * MessageId: DRM_E_MP4_EXCEEDED_NUM_CHUNKS
 *
 * MessageText:
 *
 * A query was made to the MP4 parser to get chunk information for a chunk that does not exist.
 *
 */
#define DRM_E_MP4_EXCEEDED_NUM_CHUNKS    ((DRM_RESULT)0x8004DFB4L)

/*
 * MessageId: DRM_E_MP4_NULL_FILE_STREAM
 *
 * MessageText:
 *
 * A file stream pointer was unexpectedly null.
 *
 */
#define DRM_E_MP4_NULL_FILE_STREAM       ((DRM_RESULT)0x8004DFB5L)

/*
 * MessageId: DRM_E_MP4_INVALID_MP4_FILE
 *
 * MessageText:
 *
 * The MP4 file is malformed.
 *
 */
#define DRM_E_MP4_INVALID_MP4_FILE       ((DRM_RESULT)0x8004DFB6L)

/*
 * MessageId: DRM_E_MP4_PARSING_ABORTED
 *
 * MessageText:
 *
 * MP4 parsing was aborted by the caller.
 *
 */
#define DRM_E_MP4_PARSING_ABORTED        ((DRM_RESULT)0x8004DFB7L)

/*
 * MessageId: DRM_E_MP4_EXCEEDED_BOX_SIZE
 *
 * MessageText:
 *
 * The MP4 file has a box that references data beyond the end of the box.
 *
 */
#define DRM_E_MP4_EXCEEDED_BOX_SIZE      ((DRM_RESULT)0x8004DFB8L)

/*
 * MessageId: DRM_E_MP4_INVALID_BOX_SIZE
 *
 * MessageText:
 *
 * The MP4 file has an invalid box size.
 *
 */
#define DRM_E_MP4_INVALID_BOX_SIZE       ((DRM_RESULT)0x8004DFB9L)

/*
 * MessageId: DRM_E_MP4_INVALID_PARSING_STATE
 *
 * MessageText:
 *
 * MP4 parser functions were invoked in an invalid sequence.
 *
 */
#define DRM_E_MP4_INVALID_PARSING_STATE  ((DRM_RESULT)0x8004DFBAL)

/*
 * MessageId: DRM_E_MP4_INVALID_BOX_ATTRIBUTE
 *
 * MessageText:
 *
 * An MP4 box has a malformed attribute.
 *
 */
#define DRM_E_MP4_INVALID_BOX_ATTRIBUTE  ((DRM_RESULT)0x8004DFBBL)

/*
 * MessageId: DRM_E_MP4_INVALID_BOX_VERSION
 *
 * MessageText:
 *
 * An MP4 box had an unrecognized version.
 *
 */
#define DRM_E_MP4_INVALID_BOX_VERSION    ((DRM_RESULT)0x8004DFBCL)

/*
 * MessageId: DRM_E_MP4_INVALID_STTS_CONTAINS_ENTRIES
 *
 * MessageText:
 *
 * The 'stts' box should not contain entries in purely fragmented Mp4 files.
 *
 */
#define DRM_E_MP4_INVALID_STTS_CONTAINS_ENTRIES ((DRM_RESULT)0x8004DFBDL)

/*
 * MessageId: DRM_E_C2PA_FTYP_NOT_SET
 *
 * MessageText:
 *
 * The 'ftyp' box lacks the 'c2pa' compatible_brands attribute.
 *
 */
#define DRM_E_C2PA_FTYP_NOT_SET          ((DRM_RESULT)0x8004DFBEL)

/*
 * MessageId: DRM_E_MP4_BOX_LARGER_THAN_4GB
 *
 * MessageText:
 *
 * The MP4 file has a box that is larger than 4 GB in size which is not supported by this parsre.
 *
 */
#define DRM_E_MP4_BOX_LARGER_THAN_4GB    ((DRM_RESULT)0x8004DFBFL)

/*
 * MessageId: DRM_E_MP4_C2PA_BOX_ALREADY_PRESENT
 *
 * MessageText:
 *
 * The MP4 file already contains an unexpected C2PA Box.
 *
 */
#define DRM_E_MP4_C2PA_BOX_ALREADY_PRESENT ((DRM_RESULT)0x8004DFC0L)

/*
 * MessageId: DRM_E_C2PA_MANIFEST_BOX_NOT_PRESENT
 *
 * MessageText:
 *
 * The MP4 file lacks the expected c2pa Manifest Box.
 *
 */
#define DRM_E_C2PA_MANIFEST_BOX_NOT_PRESENT ((DRM_RESULT)0x8004DFC2L)

/*
 * MessageId: DRM_E_C2PA_MERKLE_BOX_NOT_PRESENT
 *
 * MessageText:
 *
 * The MP4 file lacks the expected c2pa Merkle Box.
 *
 */
#define DRM_E_C2PA_MERKLE_BOX_NOT_PRESENT ((DRM_RESULT)0x8004DFC3L)

/*
 * MessageId: DRM_E_MP4_INVALID_C2PA_MANIFEST_BOX
 *
 * MessageText:
 *
 * The MP4 file contains an invalid c2pa box with type Manifest.
 *
 */
#define DRM_E_MP4_INVALID_C2PA_MANIFEST_BOX ((DRM_RESULT)0x8004DFC4L)

/*
 * MessageId: DRM_E_MP4_INVALID_C2PA_MERKLE_BOX
 *
 * MessageText:
 *
 * The MP4 file contains an invalid c2pa box with type Merkle.
 *
 */
#define DRM_E_MP4_INVALID_C2PA_MERKLE_BOX ((DRM_RESULT)0x8004DFC5L)

/*
 * MessageId: DRM_E_MP4_INVALID_PARSING_TYPE
 *
 * MessageText:
 *
 * The MP4 parser was initialized with an invalid type.
 *
 */
#define DRM_E_MP4_INVALID_PARSING_TYPE   ((DRM_RESULT)0x8004DFC6L)

/*
 * MessageId: DRM_E_MP4_FILE_LACKS_MOOV_BOX
 *
 * MessageText:
 *
 * The MP4 file does not have a 'moov' box.
 *
 */
#define DRM_E_MP4_FILE_LACKS_MOOV_BOX    ((DRM_RESULT)0x8004DFC7L)

/*
 * MessageId: DRM_E_MP4_EXCEEDED_NUM_TRACK_IDS
 *
 * MessageText:
 *
 * A query was made to the MP4 parser to get track information for a track that does not exist.
 *
 */
#define DRM_E_MP4_EXCEEDED_NUM_TRACK_IDS ((DRM_RESULT)0x8004DFC8L)

/*
 * MessageId: DRM_E_MP4_INVALID_EXCLUSION_RULE
 *
 * MessageText:
 *
 * An Exclusion Rule was passed into the MP4 Parser that was incorrectly formatted.
 *
 */
#define DRM_E_MP4_INVALID_EXCLUSION_RULE ((DRM_RESULT)0x8004DFC9L)


/* ------------------------------------------------------------
**
** Available range 0x8004dfd0-0x8004dfff.
**
** ------------------------------------------------------------
*/

/* ============================================================
**
** Windows PC specific errors (from 0x8004e000 to 0x8004ffff)
**
** ============================================================
*/

/* Nothing should be added here - Windows PC error codes are not here. */


/* ============================================================
**
** Deprecated errors.  Returned in previous versions of the PK but not in the current version.
** Note: No new error codes should be defined with the same values as these.
**
** ============================================================
*/

#define DRM_E_DEPRECATED_CH_ATTR_MISSING                                           ((DRM_RESULT)0x80041107L)
#define DRM_E_DEPRECATED_CH_UNABLE_TO_VERIFY                                       ((DRM_RESULT)0x8004110AL)
#define DRM_E_DEPRECATED_CH_UNSUPPORTED_HASH_ALGORITHM                             ((DRM_RESULT)0x8004110CL)
#define DRM_E_DEPRECATED_CH_UNSUPPORTED_SIGN_ALGORITHM                             ((DRM_RESULT)0x8004110DL)
#define DRM_E_DEPRECATED_CH_NOT_SIGNED                                             ((DRM_RESULT)0x80041113L)
#define DRM_E_DEPRECATED_CH_UNKNOWN_ERROR                                          ((DRM_RESULT)0x80041116L)
#define DRM_E_DEPRECATED_LIC_INIT_FAILURE                                          ((DRM_RESULT)0x80041201L)
#define DRM_E_DEPRECATED_LIC_LICENSE_NOTSET                                        ((DRM_RESULT)0x80041202L)
#define DRM_E_DEPRECATED_LIC_PARAM_NOT_OPTIONAL                                    ((DRM_RESULT)0x80041203L)
#define DRM_E_DEPRECATED_LIC_MEMORY_ALLOCATION_ERROR                               ((DRM_RESULT)0x80041204L)
#define DRM_E_DEPRECATED_LIC_INVALID_LICENSE                                       ((DRM_RESULT)0x80041205L)
#define DRM_E_DEPRECATED_LIC_FIELD_MISSING                                         ((DRM_RESULT)0x80041206L)
#define DRM_E_DEPRECATED_LIC_UNKNOWN_ERROR                                         ((DRM_RESULT)0x80041208L)
#define DRM_E_DEPRECATED_LIC_INVALID_REVLIST                                       ((DRM_RESULT)0x80041209L)
#define DRM_E_DEPRECATED_LIC_EXPIRED_CERT                                          ((DRM_RESULT)0x8004120AL)
#define DRM_E_DEPRECATED_CPRMEXP_NOERROR                                           ((DRM_RESULT)0x80041400L)
#define DRM_E_DEPRECATED_CPRMEXP_PARAM_NOT_OPTIONAL                                ((DRM_RESULT)0x80041401L)
#define DRM_E_DEPRECATED_CPRMEXP_MEMORY_ALLOCATION_ERROR                           ((DRM_RESULT)0x80041402L)
#define DRM_E_DEPRECATED_CPRMEXP_NO_OPERANDS_IN_EXPRESSION                         ((DRM_RESULT)0x80041403L)
#define DRM_E_DEPRECATED_CPRMEXP_INVALID_TOKEN                                     ((DRM_RESULT)0x80041404L)
#define DRM_E_DEPRECATED_CPRMEXP_INVALID_CONSTANT                                  ((DRM_RESULT)0x80041405L)
#define DRM_E_DEPRECATED_CPRMEXP_INVALID_VARIABLE                                  ((DRM_RESULT)0x80041406L)
#define DRM_E_DEPRECATED_CPRMEXP_INVALID_FUNCTION                                  ((DRM_RESULT)0x80041407L)
#define DRM_E_DEPRECATED_CPRMEXP_INVALID_ARGUMENT                                  ((DRM_RESULT)0x80041408L)
#define DRM_E_DEPRECATED_CPRMEXP_INVALID_CONTEXT                                   ((DRM_RESULT)0x80041409L)
#define DRM_E_DEPRECATED_CPRMEXP_ENDOFBUFFER                                       ((DRM_RESULT)0x8004140AL)
#define DRM_E_DEPRECATED_CPRMEXP_MISSING_OPERAND                                   ((DRM_RESULT)0x8004140BL)
#define DRM_E_DEPRECATED_CPRMEXP_OVERFLOW                                          ((DRM_RESULT)0x8004140CL)
#define DRM_E_DEPRECATED_CPRMEXP_UNDERFLOW                                         ((DRM_RESULT)0x8004140DL)
#define DRM_E_DEPRECATED_CPRMEXP_INCORRECT_NUM_ARGS                                ((DRM_RESULT)0x8004140EL)
#define DRM_E_DEPRECATED_CPRMEXP_VARIABLE_EXPECTED                                 ((DRM_RESULT)0x8004140FL)
#define DRM_E_DEPRECATED_CPRMEXP_RETRIEVAL_FAILURE                                 ((DRM_RESULT)0x80041410L)
#define DRM_E_DEPRECATED_CPRMEXP_UPDATE_FAILURE                                    ((DRM_RESULT)0x80041411L)
#define DRM_E_DEPRECATED_CPRMEXP_STRING_UNTERMINATED                               ((DRM_RESULT)0x80041412L)
#define DRM_E_DEPRECATED_CPRMEXP_UPDATE_UNSUPPORTED                                ((DRM_RESULT)0x80041413L)
#define DRM_E_DEPRECATED_CPRMEXP_ISOLATED_OPERAND_OR_OPERATOR                      ((DRM_RESULT)0x80041414L)
#define DRM_E_DEPRECATED_CPRMEXP_UNMATCHED                                         ((DRM_RESULT)0x80041415L)
#define DRM_E_DEPRECATED_CPRMEXP_WRONG_TYPE_OPERAND                                ((DRM_RESULT)0x80041416L)
#define DRM_E_DEPRECATED_CPRMEXP_TOO_MANY_OPERANDS                                 ((DRM_RESULT)0x80041417L)
#define DRM_E_DEPRECATED_CPRMEXP_UNKNOWN_PARSE_ERROR                               ((DRM_RESULT)0x80041418L)
#define DRM_E_DEPRECATED_CPRMEXP_UNSUPPORTED_FUNCTION                              ((DRM_RESULT)0x80041419L)
#define DRM_E_DEPRECATED_CPRMEXP_CLOCK_REQUIRED                                    ((DRM_RESULT)0x8004141AL)
#define DRM_E_DEPRECATED_LIC_SIGNATURE_FAILURE                                     ((DRM_RESULT)0x80048008L)
#define DRM_E_DEPRECATED_LIC_KEY_AND_CERT_MISMATCH                                 ((DRM_RESULT)0x80048013L)
#define DRM_E_DEPRECATED_SYNC_ENTRY_NOT_FOUND                                      ((DRM_RESULT)0x800480D0L)
#define DRM_E_DEPRECATED_STACK_TOO_SMALL                                           ((DRM_RESULT)0x800480D1L)
#define DRM_E_DEPRECATED_EXPIRED_CERT                                              ((DRM_RESULT)0x80049006L)
#define DRM_E_DEPRECATED_DRMUTIL_INVALID_CERT                                      ((DRM_RESULT)0x80049007L)
#define DRM_E_DEPRECATED_DEVICE_NOT_REGISTERED                                     ((DRM_RESULT)0x8004A000L)
#define DRM_E_DEPRECATED_TOO_MANY_INCLUSION_GUIDS                                  ((DRM_RESULT)0x8004A001L)
#define DRM_E_DEPRECATED_CONTRACT_FAILED                                           ((DRM_RESULT)0x8004A006L)
#define DRM_E_DEPRECATED_INVALID_LICENSE_REVOCATION_LIST_SIGNATURE                 ((DRM_RESULT)0x8004A014L)
#define DRM_E_DEPRECATED_INVALID_METERCERT_SIGNATURE                               ((DRM_RESULT)0x8004A015L)
#define DRM_E_DEPRECATED_NO_LICENSES_TO_SYNC                                       ((DRM_RESULT)0x8004A017L)
#define DRM_E_DEPRECATED_INVALID_SLK                                               ((DRM_RESULT)0x8004A01CL)
#define DRM_E_DEPRECATED_DEVCERT_MODEL_MISMATCH                                    ((DRM_RESULT)0x8004A01DL)
#define DRM_E_DEPRECATED_DSTR_NOT_FOUND                                            ((DRM_RESULT)0x8004A01FL)
#define DRM_E_DEPRECATED_INVALID_RIGHT                                             ((DRM_RESULT)0x8004C003L)
#define DRM_E_DEPRECATED_INCOMPATABLE_LICENSE_SIZE                                 ((DRM_RESULT)0x8004C004L)
#define DRM_E_DEPRECATED_INVALID_LICENSE_FLAGS                                     ((DRM_RESULT)0x8004C005L)
#define DRM_E_DEPRECATED_CONDITION_FAIL                                            ((DRM_RESULT)0x8004C007L)
#define DRM_E_DEPRECATED_CONDITION_NOT_SUPPORTED                                   ((DRM_RESULT)0x8004C008L)
#define DRM_E_DEPRECATED_LICENSE_NOT_YET_VALID                                     ((DRM_RESULT)0x8004C00AL)
#define DRM_E_DEPRECATED_LICENSE_MISMATCH                                          ((DRM_RESULT)0x8004C00CL)
#define DRM_E_DEPRECATED_NO_RIGHTS_REQUESTED                                       ((DRM_RESULT)0x8004C00EL)
#define DRM_E_DEPRECATED_INVALID_TIME                                              ((DRM_RESULT)0x8004C011L)
#define DRM_E_DEPRECATED_LICENSESTORE_NOT_FOUND                                    ((DRM_RESULT)0x8004C012L)
#define DRM_E_DEPRECATED_INVALID_BIND_ID                                           ((DRM_RESULT)0x8004C015L)
#define DRM_E_DEPRECATED_ALGORITHM_NOT_SET                                         ((DRM_RESULT)0x8004C017L)
#define DRM_E_DEPRECATED_LICENSE_SERVER_NEEDS_KEY                                  ((DRM_RESULT)0x8004C018L)
#define DRM_E_DEPRECATED_CLIENT_TIME_INVALID                                       ((DRM_RESULT)0x8004C01CL)
#define DRM_E_DEPRECATED_DST_NAMESPACE_FULL                                        ((DRM_RESULT)0x8004C022L)
#define DRM_E_DEPRECATED_DST_NAMESPACE_IN_USE                                      ((DRM_RESULT)0x8004C028L)
#define DRM_E_DEPRECATED_NO_ACTION_IN_LICENSE_REQUEST                              ((DRM_RESULT)0x8004C02CL)
#define DRM_E_DEPRECATED_BACKUP_EXISTS                                             ((DRM_RESULT)0x8004C032L)
#define DRM_E_DEPRECATED_LICENSE_TOO_LONG                                          ((DRM_RESULT)0x8004C033L)
#define DRM_E_DEPRECATED_DST_RESERVED_KEY_DETECTED                                 ((DRM_RESULT)0x8004C03AL)
#define DRM_E_DEPRECATED_V1_NOT_SUPPORTED                                          ((DRM_RESULT)0x8004C03BL)
#define DRM_E_DEPRECATED_NEED_DEVCERT_INDIV                                        ((DRM_RESULT)0x8004C03DL)
#define DRM_E_DEPRECATED_CLK_INVALID_DATE                                          ((DRM_RESULT)0x8004C040L)
#define DRM_E_DEPRECATED_CLK_UNSUPPORTED_VALUE                                     ((DRM_RESULT)0x8004C041L)
#define DRM_E_DEPRECATED_INVALID_DEVCERT_TEMPLATE                                  ((DRM_RESULT)0x8004C042L)
#define DRM_E_DEPRECATED_DEVCERT_TEMPLATE_EXCEEDS_SIZE_LIMIT                       ((DRM_RESULT)0x8004C044L)
#define DRM_E_DEPRECATED_DEVCERT_READ_ERROR                                        ((DRM_RESULT)0x8004C045L)
#define DRM_E_DEPRECATED_DEVCERT_WRITE_ERROR                                       ((DRM_RESULT)0x8004C046L)
#define DRM_E_DEPRECATED_PRIVKEY_WRITE_ERROR                                       ((DRM_RESULT)0x8004C048L)
#define DRM_E_DEPRECATED_DEVCERT_INDIV_NOT_SUPPORTED                               ((DRM_RESULT)0x8004C04BL)
#define DRM_E_DEPRECATED_CLK_RESET_STATE_READ_ERROR                                ((DRM_RESULT)0x8004C04DL)
#define DRM_E_DEPRECATED_CLK_RESET_STATE_WRITE_ERROR                               ((DRM_RESULT)0x8004C04EL)
#define DRM_E_DEPRECATED_METERING_INVALID_COMMAND                                  ((DRM_RESULT)0x8004C051L)
#define DRM_E_DEPRECATED_UNKNOWN_BINDING_KEY                                       ((DRM_RESULT)0x8004C056L)
#define DRM_E_DEPRECATED_WRONG_TOKEN_TYPE                                          ((DRM_RESULT)0x8004C058L)
#define DRM_E_DEPRECATED_POLICY_METERING_DISABLED                                  ((DRM_RESULT)0x8004C059L)
#define DRM_E_DEPRECATED_POLICY_ONLINE_DISABLED                                    ((DRM_RESULT)0x8004C05AL)
#define DRM_E_DEPRECATED_METERING_MID_MISMATCH                                     ((DRM_RESULT)0x8004C05FL)
#define DRM_E_DEPRECATED_METERING_RESPONSE_DECRYPT_FAILED                          ((DRM_RESULT)0x8004C060L)
#define DRM_E_DEPRECATED_INVALID_DEVSTORE_ATTRIBUTE                                ((DRM_RESULT)0x8004C067L)
#define DRM_E_DEPRECATED_INVALID_DEVSTORE_ENTRY                                    ((DRM_RESULT)0x8004C068L)
#define DRM_E_DEPRECATED_PRECISION_ARITHMETIC_FAIL                                 ((DRM_RESULT)0x8004C06CL)
#define DRM_E_DEPRECATED_REVOCATION_NOT_SUPPORTED                                  ((DRM_RESULT)0x8004C071L)
#define DRM_E_DEPRECATED_DISK_SPACE_ERROR                                          ((DRM_RESULT)0x8004C081L)
#define DRM_E_DEPRECATED_LRB_NO_LGPUBKEY                                           ((DRM_RESULT)0x8004C0A0L)
#define DRM_E_DEPRECATED_LRB_INVALID_SIGNATURE                                     ((DRM_RESULT)0x8004C0A1L)
#define DRM_E_DEPRECATED_LRB_LGPUBKEY_MISMATCH                                     ((DRM_RESULT)0x8004C0A2L)
#define DRM_E_DEPRECATED_LRB_INVALID_LICENSE_DATA                                  ((DRM_RESULT)0x8004C0A3L)
#define DRM_E_DEPRECATED_LICEVAL_LICENSE_REVOKED                                   ((DRM_RESULT)0x8004C0C2L)
#define DRM_E_DEPRECATED_LICEVAL_UPDATE_FAILURE                                    ((DRM_RESULT)0x8004C0C3L)
#define DRM_E_DEPRECATED_LICEVAL_INVALID_PRND_LICENSE                              ((DRM_RESULT)0x8004C0C5L)
#define DRM_E_DEPRECATED_XMR_OBJECT_ALREADY_EXISTS                                 ((DRM_RESULT)0x8004C0E0L)
#define DRM_E_DEPRECATED_XMR_OBJECT_NOT_FOUND                                      ((DRM_RESULT)0x8004C0E1L)
#define DRM_E_DEPRECATED_XMR_INVALID_UNKNOWN_OBJECT                                ((DRM_RESULT)0x8004C0E3L)
#define DRM_E_DEPRECATED_XMR_LICENSE_BINDABLE                                      ((DRM_RESULT)0x8004C0E4L)
#define DRM_E_DEPRECATED_XMR_UNSUPPORTED_XMR_VERSION                               ((DRM_RESULT)0x8004C0E6L)
#define DRM_E_DEPRECATED_NOT_CRL_BLOB                                              ((DRM_RESULT)0x8004C100L)
#define DRM_E_DEPRECATED_BAD_CRL_BLOB                                              ((DRM_RESULT)0x8004C101L)
#define DRM_E_DEPRECATED_TEST_PKCRYPTO_FAILURE                                     ((DRM_RESULT)0x8004C300L)
#define DRM_E_DEPRECATED_TEST_PKSIGN_VERIFY_ERROR                                  ((DRM_RESULT)0x8004C301L)
#define DRM_E_DEPRECATED_TEST_RC4KEY_FAILED                                        ((DRM_RESULT)0x8004C303L)
#define DRM_E_DEPRECATED_TEST_DESKEY_FAILED                                        ((DRM_RESULT)0x8004C305L)
#define DRM_E_DEPRECATED_TEST_CBC_INVERSEMAC_FAILURE                               ((DRM_RESULT)0x8004C306L)
#define DRM_E_DEPRECATED_TEST_HMAC_FAILURE                                         ((DRM_RESULT)0x8004C307L)
#define DRM_E_DEPRECATED_TEST_DEVICE_PRIVATE_KEY_INCORRECTLY_STORED                ((DRM_RESULT)0x8004C30AL)
#define DRM_E_DEPRECATED_TEST_DRMMANAGER_CONTEXT_NULL                              ((DRM_RESULT)0x8004C30BL)
#define DRM_E_DEPRECATED_TEST_LICENSE_STATE_MISMATCH                               ((DRM_RESULT)0x8004C313L)
#define DRM_E_DEPRECATED_TEST_LICENSE_RESPONSE_ERROR                               ((DRM_RESULT)0x8004C31AL)
#define DRM_E_DEPRECATED_TEST_DLA_NO_CONTENT_HEADER                                ((DRM_RESULT)0x8004C31FL)
#define DRM_E_DEPRECATED_TEST_DLA_CONTENT_HEADER_FOUND                             ((DRM_RESULT)0x8004C320L)
#define DRM_E_DEPRECATED_TEST_SYNC_LSD_INCORRECT                                   ((DRM_RESULT)0x8004C321L)
#define DRM_E_DEPRECATED_TEST_DEVICE_NOT_INITED                                    ((DRM_RESULT)0x8004C324L)
#define DRM_E_DEPRECATED_TEST_LICENSE_ACQ_FAILED                                   ((DRM_RESULT)0x8004C328L)
#define DRM_E_DEPRECATED_TEST_INVALID_DEVICE_WRAPPER                               ((DRM_RESULT)0x8004C331L)
#define DRM_E_DEPRECATED_TEST_INVALID_WMDM_WRAPPER                                 ((DRM_RESULT)0x8004C332L)
#define DRM_E_DEPRECATED_TEST_INVALID_WPD_WRAPPER                                  ((DRM_RESULT)0x8004C333L)
#define DRM_E_DEPRECATED_TEST_PROPERTY_NOT_FOUND                                   ((DRM_RESULT)0x8004C335L)
#define DRM_E_DEPRECATED_TEST_FILE_ALREADY_OPEN                                    ((DRM_RESULT)0x8004C337L)
#define DRM_E_DEPRECATED_TEST_PICT_COLUMN_TOO_WIDE                                 ((DRM_RESULT)0x8004C339L)
#define DRM_E_DEPRECATED_TEST_PICT_COLUMN_MISMATCH                                 ((DRM_RESULT)0x8004C33AL)
#define DRM_E_DEPRECATED_TEST_TUX_TEST_SKIPPED                                     ((DRM_RESULT)0x8004C33BL)
#define DRM_E_DEPRECATED_SYNCLIST_NOT_SUPPORTED                                    ((DRM_RESULT)0x8004C3EAL)
#define DRM_E_DEPRECATED_DEVICE_ALREADY_REGISTERED                                 ((DRM_RESULT)0x8004C3ECL)
#define DRM_E_DEPRECATED_FEATURE_NOT_SUPPORTED                                     ((DRM_RESULT)0x8004C3F4L)
#define DRM_E_DEPRECATED_HWID_ERROR                                                ((DRM_RESULT)0x8004C3F6L)
#define DRM_E_DEPRECATED_LICACQ_ACK_MESSAGE_NOT_CREATED                            ((DRM_RESULT)0x8004C702L)
#define DRM_E_DEPRECATED_INITIATORS_UNKNOWN_TYPE                                   ((DRM_RESULT)0x8004C780L)
#define DRM_E_DEPRECATED_INITIATORS_INVALID_SERVICEID                              ((DRM_RESULT)0x8004C781L)
#define DRM_E_DEPRECATED_INITIATORS_INVALID_ACCOUNTID                              ((DRM_RESULT)0x8004C782L)
#define DRM_E_DEPRECATED_INITIATORS_INVALID_MID                                    ((DRM_RESULT)0x8004C783L)
#define DRM_E_DEPRECATED_INITIATORS_MISSING_DC_URL                                 ((DRM_RESULT)0x8004C784L)
#define DRM_E_DEPRECATED_INITIATORS_MISSING_CONTENT_HEADER                         ((DRM_RESULT)0x8004C785L)
#define DRM_E_DEPRECATED_INITIATORS_MISSING_LAURL_IN_CONTENT_HEADER                ((DRM_RESULT)0x8004C786L)
#define DRM_E_DEPRECATED_INITIATORS_MISSING_METERCERT_URL                          ((DRM_RESULT)0x8004C787L)
#define DRM_E_DEPRECATED_BCERT_INVALID_MAX_LICENSE_SIZE                            ((DRM_RESULT)0x8004C808L)
#define DRM_E_DEPRECATED_BCERT_INVALID_MAX_HEADER_SIZE                             ((DRM_RESULT)0x8004C809L)
#define DRM_E_DEPRECATED_BCERT_CLIENT_ID_NOT_SPECIFIED                             ((DRM_RESULT)0x8004C810L)
#define DRM_E_DEPRECATED_BCERT_HARDWARE_ID_NOT_SPECIFIED                           ((DRM_RESULT)0x8004C813L)
#define DRM_E_DEPRECATED_BCERT_HARDWARE_ID_TOO_LONG                                ((DRM_RESULT)0x8004C814L)
#define DRM_E_DEPRECATED_BCERT_SERIAL_NUM_NOT_SPECIFIED                            ((DRM_RESULT)0x8004C815L)
#define DRM_E_DEPRECATED_BCERT_OBJECTHEADER_LEN_TOO_BIG                            ((DRM_RESULT)0x8004C81AL)
#define DRM_E_DEPRECATED_BCERT_INVALID_ISSUERKEY_LENGTH                            ((DRM_RESULT)0x8004C81BL)
#define DRM_E_DEPRECATED_BCERT_UNEXPECTED_OBJECT_HEADER                            ((DRM_RESULT)0x8004C81DL)
#define DRM_E_DEPRECATED_BCERT_INVALID_MAX_KEY_USAGES                              ((DRM_RESULT)0x8004C81FL)
#define DRM_E_DEPRECATED_BCERT_INVALID_MAX_FEATURES                                ((DRM_RESULT)0x8004C820L)
#define DRM_E_DEPRECATED_BCERT_INVALID_CERT_HEADER_TAG                             ((DRM_RESULT)0x8004C824L)
#define DRM_E_DEPRECATED_BCERT_INVALID_CERT_LENGTH                                 ((DRM_RESULT)0x8004C826L)
#define DRM_E_DEPRECATED_BCERT_INVALID_NUMBER_EXTDATARECORDS                       ((DRM_RESULT)0x8004C829L)
#define DRM_E_DEPRECATED_BCERT_EXTDATA_LENGTH_MUST_PRESENT                         ((DRM_RESULT)0x8004C82BL)
#define DRM_E_DEPRECATED_BCERT_INVALID_EXTDATA_LENGTH                              ((DRM_RESULT)0x8004C82DL)
#define DRM_E_DEPRECATED_BCERT_EXTDATA_IS_NOT_PROVIDED                             ((DRM_RESULT)0x8004C82EL)
#define DRM_E_DEPRECATED_BCERT_INVALID_EXTDATA_SIGNED_LENGTH                       ((DRM_RESULT)0x8004C830L)
#define DRM_E_DEPRECATED_BCERT_INVALID_EXTDATA_RECORD_TYPE                         ((DRM_RESULT)0x8004C831L)
#define DRM_E_DEPRECATED_BCERT_EXTDATAFLAG_CERT_TYPE_MISMATCH                      ((DRM_RESULT)0x8004C832L)
#define DRM_E_DEPRECATED_BCERT_OBJECTHEADER_LEN_TOO_SMALL                          ((DRM_RESULT)0x8004C83BL)
#define DRM_E_DEPRECATED_SECURE_TRACE_BAD_GLOBAL_DATA_POINTER                      ((DRM_RESULT)0x8004CD00L)
#define DRM_E_DEPRECATED_SECURE_TRACE_INVALID_GLOBAL_DATA                          ((DRM_RESULT)0x8004CD01L)
#define DRM_E_DEPRECATED_SECURE_TRACE_FORMATTING_ERROR                             ((DRM_RESULT)0x8004CD02L)
#define DRM_E_DEPRECATED_SECURE_TRACE_BAD_SCHEME_DATA_POINTER                      ((DRM_RESULT)0x8004CD03L)
#define DRM_E_DEPRECATED_SECURE_TRACE_BAD_PER_THREAD_AES_DATA_POINTER              ((DRM_RESULT)0x8004CD04L)
#define DRM_E_DEPRECATED_SECURE_TRACE_BAD_PER_THREAD_AES_BUFFER_POINTER            ((DRM_RESULT)0x8004CD05L)
#define DRM_E_DEPRECATED_SECURE_TRACE_AES_INSUFFICIENT_BUFFER                      ((DRM_RESULT)0x8004CD06L)
#define DRM_E_DEPRECATED_SECURE_TRACE_VERSION_MISMATCH                             ((DRM_RESULT)0x8004CD07L)
#define DRM_E_DEPRECATED_SECURE_TRACE_UNEXPECTED_ERROR                             ((DRM_RESULT)0x8004CD08L)
#define DRM_E_DEPRECATED_ND_MUST_REVALIDATE                                        ((DRM_RESULT)0x8004CE00L)
#define DRM_E_DEPRECATED_ND_INVALID_MESSAGE                                        ((DRM_RESULT)0x8004CE01L)
#define DRM_E_DEPRECATED_ND_INVALID_MESSAGE_TYPE                                   ((DRM_RESULT)0x8004CE02L)
#define DRM_E_DEPRECATED_ND_INVALID_MESSAGE_VERSION                                ((DRM_RESULT)0x8004CE03L)
#define DRM_E_DEPRECATED_ND_INVALID_SESSION                                        ((DRM_RESULT)0x8004CE04L)
#define DRM_E_DEPRECATED_ND_MEDIA_SESSION_LIMIT_REACHED                            ((DRM_RESULT)0x8004CE05L)
#define DRM_E_DEPRECATED_ND_UNABLE_TO_VERIFY_PROXIMITY                             ((DRM_RESULT)0x8004CE06L)
#define DRM_E_DEPRECATED_ND_INVALID_PROXIMITY_RESPONSE                             ((DRM_RESULT)0x8004CE07L)
#define DRM_E_DEPRECATED_ND_DEVICE_LIMIT_REACHED                                   ((DRM_RESULT)0x8004CE08L)
#define DRM_E_DEPRECATED_ND_BAD_REQUEST                                            ((DRM_RESULT)0x8004CE09L)
#define DRM_E_DEPRECATED_ND_FAILED_SEEK                                            ((DRM_RESULT)0x8004CE0AL)
#define DRM_E_DEPRECATED_ND_INVALID_CONTEXT                                        ((DRM_RESULT)0x8004CE0BL)
#define DRM_E_DEPRECATED_ASF_BAD_ASF_HEADER                                        ((DRM_RESULT)0x8004CF00L)
#define DRM_E_DEPRECATED_ASF_BAD_PACKET_HEADER                                     ((DRM_RESULT)0x8004CF01L)
#define DRM_E_DEPRECATED_ASF_BAD_PAYLOAD_HEADER                                    ((DRM_RESULT)0x8004CF02L)
#define DRM_E_DEPRECATED_ASF_BAD_DATA_HEADER                                       ((DRM_RESULT)0x8004CF03L)
#define DRM_E_DEPRECATED_ASF_INVALID_OPERATION                                     ((DRM_RESULT)0x8004CF04L)
#define DRM_E_DEPRECATED_ASF_AES_PAYLOAD_FOUND                                     ((DRM_RESULT)0x8004CF05L)
#define DRM_E_DEPRECATED_ASF_EXTENDED_STREAM_PROPERTIES_OBJ_NOT_FOUND              ((DRM_RESULT)0x8004CF06L)
#define DRM_E_DEPRECATED_ASF_INVALID_DATA                                          ((DRM_RESULT)0x8004CF20L)
#define DRM_E_DEPRECATED_ASF_TOO_MANY_PAYLOADS                                     ((DRM_RESULT)0x8004CF21L)
#define DRM_E_DEPRECATED_ASF_BANDWIDTH_OVERRUN                                     ((DRM_RESULT)0x8004CF22L)
#define DRM_E_DEPRECATED_ASF_INVALID_STREAM_NUMBER                                 ((DRM_RESULT)0x8004CF23L)
#define DRM_E_DEPRECATED_ASF_LATE_SAMPLE                                           ((DRM_RESULT)0x8004CF24L)
#define DRM_E_DEPRECATED_ASF_NOT_ACCEPTING                                         ((DRM_RESULT)0x8004CF25L)
#define DRM_E_DEPRECATED_ASF_UNEXPECTED                                            ((DRM_RESULT)0x8004CF26L)
#define DRM_E_DEPRECATED_LICGEN_POLICY_NOT_SUPPORTED                               ((DRM_RESULT)0x8004D100L)
#define DRM_E_DEPRECATED_MOVE_DENIED                                               ((DRM_RESULT)0x8004D300L)
#define DRM_E_DEPRECATED_INVALID_MOVE_RESPONSE                                     ((DRM_RESULT)0x8004D301L)
#define DRM_E_DEPRECATED_MOVE_NONCE_MISMATCH                                       ((DRM_RESULT)0x8004D302L)
#define DRM_E_DEPRECATED_MOVE_TXID_MISMATCH                                        ((DRM_RESULT)0x8004D303L)
#define DRM_E_DEPRECATED_MOVE_STORE_OPEN_STORE                                     ((DRM_RESULT)0x8004D304L)
#define DRM_E_DEPRECATED_MOVE_STORE_CLOSE_STORE                                    ((DRM_RESULT)0x8004D305L)
#define DRM_E_DEPRECATED_MOVE_STORE_ADD_DATA                                       ((DRM_RESULT)0x8004D306L)
#define DRM_E_DEPRECATED_MOVE_STORE_GET_DATA                                       ((DRM_RESULT)0x8004D307L)
#define DRM_E_DEPRECATED_MOVE_FORMAT_INVALID                                       ((DRM_RESULT)0x8004D308L)
#define DRM_E_DEPRECATED_MOVE_SIGNATURE_INVALID                                    ((DRM_RESULT)0x8004D309L)
#define DRM_E_DEPRECATED_COPY_DENIED                                               ((DRM_RESULT)0x8004D30AL)
#define DRM_E_DEPRECATED_KEYFILE_INVALID_PLATFORM                                  ((DRM_RESULT)0x8004D500L)
#define DRM_E_DEPRECATED_KEYFILE_TOO_LARGE                                         ((DRM_RESULT)0x8004D501L)
#define DRM_E_DEPRECATED_KEYFILE_PRIVATE_KEY_NOT_FOUND                             ((DRM_RESULT)0x8004D502L)
#define DRM_E_DEPRECATED_KEYFILE_CERTIFICATE_CHAIN_NOT_FOUND                       ((DRM_RESULT)0x8004D503L)
#define DRM_E_DEPRECATED_KEYFILE_KEY_NOT_FOUND                                     ((DRM_RESULT)0x8004D504L)
#define DRM_E_DEPRECATED_KEYFILE_UNKNOWN_DECRYPTION_METHOD                         ((DRM_RESULT)0x8004D505L)
#define DRM_E_DEPRECATED_KEYFILE_INVALID_SIGNATURE                                 ((DRM_RESULT)0x8004D506L)
#define DRM_E_DEPRECATED_KEYFILE_INTERNAL_DECRYPTION_BUFFER_TOO_SMALL              ((DRM_RESULT)0x8004D507L)
#define DRM_E_DEPRECATED_KEYFILE_PLATFORMID_MISMATCH                               ((DRM_RESULT)0x8004D508L)
#define DRM_E_DEPRECATED_KEYFILE_CERTIFICATE_ISSUER_KEY_MISMATCH                   ((DRM_RESULT)0x8004D509L)
#define DRM_E_DEPRECATED_KEYFILE_ROBUSTNESSVERSION_MISMATCH                        ((DRM_RESULT)0x8004D50AL)
#define DRM_E_DEPRECATED_KEYFILE_FILE_NOT_CLOSED                                   ((DRM_RESULT)0x8004D50BL)
#define DRM_E_DEPRECATED_KEYFILE_NOT_INITED                                        ((DRM_RESULT)0x8004D50CL)
#define DRM_E_DEPRECATED_KEYFILE_FORMAT_INVALID                                    ((DRM_RESULT)0x8004D50DL)
#define DRM_E_DEPRECATED_KEYFILE_UPDATE_NOT_ALLOWED                                ((DRM_RESULT)0x8004D50EL)
#define DRM_E_DEPRECATED_PRND_MESSAGE_VERSION_INVALID                              ((DRM_RESULT)0x8004D700L)
#define DRM_E_DEPRECATED_PRND_MESSAGE_WRONG_TYPE                                   ((DRM_RESULT)0x8004D701L)
#define DRM_E_DEPRECATED_PRND_MESSAGE_INVALID                                      ((DRM_RESULT)0x8004D702L)
#define DRM_E_DEPRECATED_PRND_SESSION_ID_INVALID                                   ((DRM_RESULT)0x8004D703L)
#define DRM_E_DEPRECATED_PRND_PROXIMITY_DETECTION_REQUEST_CHANNEL_TYPE_UNSUPPORTED ((DRM_RESULT)0x8004D704L)
#define DRM_E_DEPRECATED_PRND_PROXIMITY_DETECTION_RESPONSE_INVALID                 ((DRM_RESULT)0x8004D705L)
#define DRM_E_DEPRECATED_PRND_PROXIMITY_DETECTION_RESPONSE_TIMEOUT                 ((DRM_RESULT)0x8004D706L)
#define DRM_E_DEPRECATED_PRND_LICENSE_REQUEST_CID_CALLBACK_REQUIRED                ((DRM_RESULT)0x8004D707L)
#define DRM_E_DEPRECATED_PRND_LICENSE_RESPONSE_CLMID_INVALID                       ((DRM_RESULT)0x8004D708L)
#define DRM_E_DEPRECATED_PRND_CERTIFICATE_NOT_RECEIVER                             ((DRM_RESULT)0x8004D709L)
#define DRM_E_DEPRECATED_PRND_CANNOT_RENEW_USING_NEW_SESSION                       ((DRM_RESULT)0x8004D70AL)
#define DRM_E_DEPRECATED_PRND_INVALID_CUSTOM_DATA_TYPE                             ((DRM_RESULT)0x8004D70BL)
#define DRM_E_DEPRECATED_PRND_CLOCK_OUT_OF_SYNC                                    ((DRM_RESULT)0x8004D70CL)
#define DRM_E_DEPRECATED_PRND_CANNOT_REBIND_PRND_RECEIVED_LICENSE                  ((DRM_RESULT)0x8004D70DL)
#define DRM_E_DEPRECATED_PRND_CANNOT_REGISTER_USING_EXISTING_SESSION               ((DRM_RESULT)0x8004D70EL)
#define DRM_E_DEPRECATED_PRND_BUSY_PERFORMING_RENEWAL                              ((DRM_RESULT)0x8004D70FL)
#define DRM_E_DEPRECATED_PRND_LICENSE_REQUEST_INVALID_ACTION                       ((DRM_RESULT)0x8004D710L)
#define DRM_E_DEPRECATED_PRND_TRANSMITTER_UNAUTHORIZED                             ((DRM_RESULT)0x8004D711L)
#define DRM_E_DEPRECATED_PRND_TX_SESSION_EXPIRED                                   ((DRM_RESULT)0x8004D712L)
#define DRM_E_DEPRECATED_PRND_INCOMPLETE_PROXIMITY_DETECTION                       ((DRM_RESULT)0x8004D713L)
#define DRM_E_DEPRECATED_PRND_INVALID_CERT_DIGEST                                  ((DRM_RESULT)0x8004D714L)
#define DRM_E_DEPRECATED_OEMHAL_NOT_INITIALIZED                                    ((DRM_RESULT)0x8004D780L)
#define DRM_E_DEPRECATED_OEMHAL_OUT_OF_KEY_REGISTERS                               ((DRM_RESULT)0x8004D781L)
#define DRM_E_DEPRECATED_OEMHAL_KEYS_IN_USE                                        ((DRM_RESULT)0x8004D782L)
#define DRM_E_DEPRECATED_OEMHAL_NO_KEY                                             ((DRM_RESULT)0x8004D783L)
#define DRM_E_DEPRECATED_OEMHAL_UNSUPPORTED_KEY_TYPE                               ((DRM_RESULT)0x8004D784L)
#define DRM_E_DEPRECATED_OEMHAL_UNSUPPORTED_KEY_WRAPPING_FORMAT                    ((DRM_RESULT)0x8004D785L)
#define DRM_E_DEPRECATED_OEMHAL_UNSUPPORTED_KEY_LENGTH                             ((DRM_RESULT)0x8004D786L)
#define DRM_E_DEPRECATED_OEMHAL_UNSUPPORTED_HASH_TYPE                              ((DRM_RESULT)0x8004D787L)
#define DRM_E_DEPRECATED_OEMHAL_UNSUPPORTED_SIGNATURE_SCHEME                       ((DRM_RESULT)0x8004D788L)
#define DRM_E_DEPRECATED_OEMHAL_BUFFER_TOO_LARGE                                   ((DRM_RESULT)0x8004D789L)
#define DRM_E_DEPRECATED_OEMHAL_SAMPLE_ENCRYPTION_MODE_NOT_PERMITTED               ((DRM_RESULT)0x8004D78AL)
#define DRM_E_DEPRECATED_M2TS_PAT_PID_IS_NOT_ZERO                                  ((DRM_RESULT)0x8004D800L)
#define DRM_E_DEPRECATED_M2TS_PTS_NOT_EXIST                                        ((DRM_RESULT)0x8004D801L)
#define DRM_E_DEPRECATED_M2TS_PES_PACKET_LENGTH_NOT_SPECIFIED                      ((DRM_RESULT)0x8004D802L)
#define DRM_E_DEPRECATED_M2TS_OUTPUT_BUFFER_FULL                                   ((DRM_RESULT)0x8004D803L)
#define DRM_E_DEPRECATED_M2TS_CONTEXT_NOT_INITIALIZED                              ((DRM_RESULT)0x8004D804L)
#define DRM_E_DEPRECATED_M2TS_NEED_KEY_DATA                                        ((DRM_RESULT)0x8004D805L)
#define DRM_E_DEPRECATED_M2TS_DDPLUS_FORMAT_INVALID                                ((DRM_RESULT)0x8004D806L)
#define DRM_E_DEPRECATED_M2TS_NOT_UNIT_START_PACKET                                ((DRM_RESULT)0x8004D807L)
#define DRM_E_DEPRECATED_M2TS_TOO_MANY_SUBSAMPLES                                  ((DRM_RESULT)0x8004D808L)
#define DRM_E_DEPRECATED_M2TS_TABLE_ID_INVALID                                     ((DRM_RESULT)0x8004D809L)
#define DRM_E_DEPRECATED_M2TS_PACKET_SYNC_BYTE_INVALID                             ((DRM_RESULT)0x8004D80AL)
#define DRM_E_DEPRECATED_M2TS_ADAPTATION_LENGTH_INVALID                            ((DRM_RESULT)0x8004D80BL)
#define DRM_E_DEPRECATED_M2TS_PAT_HEADER_INVALID                                   ((DRM_RESULT)0x8004D80CL)
#define DRM_E_DEPRECATED_M2TS_PMT_HEADER_INVALID                                   ((DRM_RESULT)0x8004D80DL)
#define DRM_E_DEPRECATED_M2TS_PES_START_CODE_NOT_FOUND                             ((DRM_RESULT)0x8004D80EL)
#define DRM_E_DEPRECATED_M2TS_STREAM_OR_PACKET_TYPE_CHANGED                        ((DRM_RESULT)0x8004D80FL)
#define DRM_E_DEPRECATED_M2TS_INTERNAL_ERROR                                       ((DRM_RESULT)0x8004D810L)
#define DRM_E_DEPRECATED_M2TS_ADTS_FORMAT_INVALID                                  ((DRM_RESULT)0x8004D811L)
#define DRM_E_DEPRECATED_M2TS_MPEGA_FORMAT_INVALID                                 ((DRM_RESULT)0x8004D812L)
#define DRM_E_DEPRECATED_M2TS_CA_DESCRIPTOR_LENGTH_INVALID                         ((DRM_RESULT)0x8004D813L)
#define DRM_E_DEPRECATED_M2TS_CRC_FIELD_INVALID                                    ((DRM_RESULT)0x8004D814L)
#define DRM_E_DEPRECATED_M2TS_INCOMPLETE_SECTION_HEADER                            ((DRM_RESULT)0x8004D815L)
#define DRM_E_DEPRECATED_M2TS_INVALID_UNALIGNED_DATA                               ((DRM_RESULT)0x8004D816L)
#define DRM_E_DEPRECATED_M2TS_GET_ENCRYPTED_DATA_FIRST                             ((DRM_RESULT)0x8004D817L)
#define DRM_E_DEPRECATED_M2TS_CANNOT_CHANGE_PARAMETER                              ((DRM_RESULT)0x8004D818L)
#define DRM_E_DEPRECATED_M2TS_UNKNOWN_PACKET                                       ((DRM_RESULT)0x8004D819L)
#define DRM_E_DEPRECATED_M2TS_DROP_PACKET                                          ((DRM_RESULT)0x8004D820L)
#define DRM_E_DEPRECATED_M2TS_DROP_PES                                             ((DRM_RESULT)0x8004D821L)
#define DRM_E_DEPRECATED_M2TS_INCOMPLETE_PES                                       ((DRM_RESULT)0x8004D822L)
#define DRM_E_DEPRECATED_M2TS_WAITED_TOO_LONG                                      ((DRM_RESULT)0x8004D823L)
#define DRM_E_DEPRECATED_M2TS_SECTION_LENGTH_INVALID                               ((DRM_RESULT)0x8004D824L)
#define DRM_E_DEPRECATED_M2TS_PROGRAM_INFO_LENGTH_INVALID                          ((DRM_RESULT)0x8004D825L)
#define DRM_E_DEPRECATED_M2TS_PES_HEADER_INVALID                                   ((DRM_RESULT)0x8004D826L)
#define DRM_E_DEPRECATED_M2TS_ECM_PAYLOAD_OVER_LIMIT                               ((DRM_RESULT)0x8004D827L)
#define DRM_E_DEPRECATED_M2TS_SET_CA_PID_FAILED                                    ((DRM_RESULT)0x8004D828L)
#define DRM_E_DEPRECATED_LICGEN_PERSISTENT_REMOTE_LICENSE                          ((DRM_RESULT)0x8004D902L)
#define DRM_E_DEPRECATED_LICGEN_EXPIRE_AFTER_FIRST_PLAY_REMOTE_LICENSE             ((DRM_RESULT)0x8004D903L)
#define DRM_E_DEPRECATED_LICGEN_LOCAL_LICENSE_WITH_REMOTE_CERTIFICATE              ((DRM_RESULT)0x8004D906L)
#define DRM_E_DEPRECATED_LICGEN_PLAY_ENABLER_REMOTE_LICENSE                        ((DRM_RESULT)0x8004D907L)

#define DRM_S_DEPRECATED_TEST_SKIP_FILE                                            ((DRM_RESULT)0x0004C300L)
#define DRM_S_DEPRECATED_TEST_CONVERTED_FILE                                       ((DRM_RESULT)0x0004C301L)
#endif /*__WINDOWS_MEDIA_PROTECTION_PLAYREADY_RESULTS_H_ */

