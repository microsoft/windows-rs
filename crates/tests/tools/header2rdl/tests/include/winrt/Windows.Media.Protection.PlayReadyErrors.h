/**@@@+++@@@@******************************************************************
**
** Microsoft (r) PlayReady (r)
** Copyright (c) Microsoft Corporation. All rights reserved.
**
***@@@---@@@@******************************************************************
*/

#ifndef __WINDOWS_MEDIA_PROTECTION_PLAYREADY_ERRORS_H_
#define __WINDOWS_MEDIA_PROTECTION_PLAYREADY_ERRORS_H_

#ifndef DRM_RESULT_DEFINED
#define DRM_RESULT_DEFINED
typedef HRESULT DRM_RESULT;
#endif /*DRM_RESULT_DEFINED*/

#include <Windows.Media.Protection.PlayReadyResults.h>

/*
*  !!!!!!  VERY IMPORTANT, PLEASE READ !!!!!
*
*   The range of error codes reserved for the MSPRSDK is
*   0x8004B800 to 0x8004BEFF
*   The range of success codes reserved for the MSPRSDK is
*   0x0004B800 to 0x0004BEFF
*/

/*
*    MessageId:    MSPR_S_NETWORK_ACTIVITY_SUCCEEDED
*    Message Meaning:
*        The network operation completed successfully.
*    Hex Value:    0x0004B801
*/
#define MSPR_S_NETWORK_ACTIVITY_SUCCEEDED                           MAKE_DRM_RESULT(DRM_SEVERITY_SUCCESS,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0001)

/*
*    MessageId:    MSPR_S_HTTP_INDIVIDUALIZION_ALREADY_OCCURRED
*    Message Meaning:
*        Individualization was completed while this application
*        was attempting to individualize.  This application must
*        abandon its attempt to individualize.
*    Hex Value:    0x0004B802
*/
#define MSPR_S_HTTP_INDIVIDUALIZION_ALREADY_OCCURRED                MAKE_DRM_RESULT(DRM_SEVERITY_SUCCESS,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0002)

/*
*    MessageId:    MSPR_S_IN_MEMORY_LICENSE_DISCARDED
*    Message Meaning:
*        The license response included an in-memory license,
*        but the application did not request an enumerator
*        from the process license response method.
*        The license was therefore discarded.
*    Hex Value:    0x0004B803
*/
#define MSPR_S_IN_MEMORY_LICENSE_DISCARDED                          MAKE_DRM_RESULT(DRM_SEVERITY_SUCCESS,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0003)

/*
*    MessageId:    MSPR_E_INDIVBOX_HAS_BAD_SIGNATURE
*    Message Meaning:
*        The Individualized binary's signature has failed verification.
*        Confirm that the file is signed properly.
*        If the file is test signed, verify that the calling code trusts test signed binaries.
*    Hex Value:    0x8004B801
*/
#define MSPR_E_INDIVBOX_HAS_BAD_SIGNATURE                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0001)

/*
*    MessageId:    MSPR_E_PRO_HEADER_KID_NOT_SET
*    Message Meaning:
*        The PlayReady object header does not have a KID set.
*    Hex Value:    0x8004B804
*/
#define MSPR_E_PRO_HEADER_KID_NOT_SET                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0004)

/*
*    MessageId:    MSPR_E_PRO_HEADER_BYTES_NOT_SET
*    Message Meaning:
*        The PlayReady object header does not have bytes set.
*    Hex Value:    0x8004B805
*/
#define MSPR_E_PRO_HEADER_BYTES_NOT_SET                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0005)

/*
*    MessageId:    MSPR_E_PRO_HEADER_ALREADY_COMMITTED
*    Message Meaning:
*        The PlayReady object header is already committed to either
*        a KID or a content header.
*    Hex Value:    0x8004B806
*/
#define MSPR_E_PRO_HEADER_ALREADY_COMMITTED                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0006)

/*
*    MessageId:    MSPR_E_PRO_HEADER_NOT_YET_COMMITTED
*    Message Meaning:
*        The PlayReady object header has not had either
*        a KID or a content header set.
*    Hex Value:    0x8004B807
*/
#define MSPR_E_PRO_HEADER_NOT_YET_COMMITTED                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0007)

/*
*    MessageId:    MSPR_E_UNPROTECTED_PROTECTED_OBJECT_MISMATCH
*    Message Meaning:
*        The application attempted to pass a non-stublib-protected
*        object to a method on a stublib-protected object or vice-versa.
*    Hex Value:    0x8004B808
*/
#define MSPR_E_UNPROTECTED_PROTECTED_OBJECT_MISMATCH                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0008)

/*
*    MessageId:    MSPR_E_LEAF_LICENSE_NOT_DOMAINBOUND
*    Message Meaning:
*        The application requested the domain ID of a leaf license.
*        Leaf licenses are never domain bound, which results in this failure.
*    Hex Value:    0x8004B809
*/
#define MSPR_E_LEAF_LICENSE_NOT_DOMAINBOUND                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0009)

/*
*    MessageId:    MSPR_E_FILTER_HEADER_NOT_SET
*    Message Meaning:
*        The License filter does not have a Header object.
*    Hex Value:    0x8004B80A
*/
#define MSPR_E_FILTER_HEADER_NOT_SET                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x000A)

/*
*    MessageId:    MSPR_E_ALREADY_STUBLIB_PROTECTED
*    Message Meaning:
*        The object is already protected with a stublib.
*    Hex Value:    0x8004B80B
*/
#define MSPR_E_ALREADY_STUBLIB_PROTECTED                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x000B)

/*
*    MessageId:    MSPR_E_UNUSABLE_LICENSES_CAN_NOT_BE_EVALUATED
*    Message Meaning:
*        It is invalid to create a license filter with both
*        an Action and with the flag
*        MSPR_LICENSE_FILTER_FLAG_RETURN_UNUSABLE_LICENSES
*        because unusable licenses can not be evaluated.
*    Hex Value:    0x8004B80C
*/
#define MSPR_E_UNUSABLE_LICENSES_CAN_NOT_BE_EVALUATED               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x000C)

/*
*    MessageId:    MSPR_E_INVALID_ASF_FILE
*    Message Meaning:
*        The file is not a valid ASF file.
*    Hex Value:    0x8004B80D
*/
#define MSPR_E_INVALID_ASF_FILE                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x000D)

/*
*    MessageId:    MSPR_E_INVALID_PLAYREADY_FORMAT_FILE
*    Message Meaning:
*        The file is not a valid PlayReady format file.
*    Hex Value:    0x8004B80E
*/
#define MSPR_E_INVALID_PLAYREADY_FORMAT_FILE                        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x000E)

/*
*    MessageId:    MSPR_E_DEVICE_NOT_PLAYREADY
*    Message Meaning:
*        The device connected does not support PlayReady.
*    Hex Value:    0x8004B80F
*/
#define MSPR_E_DEVICE_NOT_PLAYREADY                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x000F)

/*
*    MessageId:    MSPR_E_INVALID_KEY_FILE
*    Message Meaning:
*        The current key file is invalid.
*    Hex Value:    0x8004B811
*/
#define MSPR_E_INVALID_KEY_FILE                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0011)

/*
*    MessageId:    MSPR_E_UNEXPECTED_CERTIFICATE_TYPE
*    Message Meaning:
*        The certificate used in an operation was of the wrong type.
*    Hex Value:    0x8004B812
*/
#define MSPR_E_UNEXPECTED_CERTIFICATE_TYPE                          MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0012)

/*
*    MessageId:    MSPR_E_NO_MATCHING_KEY
*    Message Meaning:
*        No key was found in the key history for decrypting this request.
*    Hex Value:    0x8004B814
*/
#define MSPR_E_NO_MATCHING_KEY                                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0014)

/*
*    MessageId:    MSPR_E_INDIV_INDIVIDUALIZING
*    Message Meaning:
*        The client is currently individualizing.
*        Concurrent individualization requests are not allowed.
*    Hex Value:    0x8004B817
*/
#define MSPR_E_INDIV_INDIVIDUALIZING                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0017)

/*
*    MessageId:    MSPR_E_INDIV_CANCELLED
*    Message Meaning:
*        Individualization was cancelled by the calling application.
*    Hex Value:    0x8004B818
*/
#define MSPR_E_INDIV_CANCELLED                                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0018)

/*
*    MessageId:    MSPR_E_INDIV_EXPECTED_KEY_FILE
*    Message Meaning:
*        Individualization expected a key file to exist
*        but was not able to load it from disk.
*    Hex Value:    0x8004B819
*/
#define MSPR_E_INDIV_EXPECTED_KEY_FILE                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0019)

/*
*    MessageId:    MSPR_E_INDIV_UNEXPECTED_KEY_FILE
*    Message Meaning:
*        Individualization did not expect a key file to exist
*        but one was found on disk.
*    Hex Value:    0x8004B81A
*/
#define MSPR_E_INDIV_UNEXPECTED_KEY_FILE                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x001A)

/*
*    MessageId:    MSPR_E_INDIV_BAD_REQUEST
*    Message Meaning:
*        The individualization server responded with a "bad request" message.
*        The client request was malformed.
*    Hex Value:    0x8004B81B
*/
#define MSPR_E_INDIV_BAD_REQUEST                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x001B)

/*
*    MessageId:    MSPR_E_INDIV_SERVER_ERROR
*    Message Meaning:
*        The individualization server responded with an "internal server error" message.
*        A more specific error code is not available.
*    Hex Value:    0x8004B81C
*/
#define MSPR_E_INDIV_SERVER_ERROR                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x001C)

/*
*    MessageId:    MSPR_E_INDIV_CLIENT_CURRENT
*    Message Meaning:
*        The individualization server responded that the client's security version
*        matches that of the current available IBX.
*    Hex Value:    0x8004B81D
*/
#define MSPR_E_INDIV_CLIENT_CURRENT                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x001D)

/*
*    MessageId:    MSPR_E_INVALID_SECURITY_VERSION
*    Message Meaning:
*        The security version stored in an XML certificate could not
*        be parsed correctly.
*    Hex Value:    0x8004B81E
*/
#define MSPR_E_INVALID_SECURITY_VERSION                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x001E)

/*
*    MessageId:    MSPR_E_NETWORK_ACTIVITY_FAILED
*    Message Meaning:
*        The network operation was not completed successfully due to unknown reasons.
*        Contact your content provider for further assistance.
*    Hex Value:    0x8004B820
*/
#define MSPR_E_NETWORK_ACTIVITY_FAILED                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0020)

/*
*    MessageId:    MSPR_E_CERTIFICATE_SECURITY_LEVEL_TOO_LOW
*    Message Meaning:
*        The given certificate has a security level lower than the level required by the license.
*    Hex Value:    0x8004B821
*/
#define MSPR_E_CERTIFICATE_SECURITY_LEVEL_TOO_LOW                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0021)

/*
*    MessageId:    MSPR_E_NEEDS_INDIVIDUALIZATION
*    Message Meaning:
*        The client application must individualize to the latest version.
*    Hex Value:    0x8004B822
*/
#define MSPR_E_NEEDS_INDIVIDUALIZATION                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0022)

/*
*    MessageId:    MSPR_E_HARDWAREID_MISMATCH
*    Message Meaning:
*        The requested action cannot be performed because a
*        hardware configuration change has been detected by
*        the Microsoft PlayReady components on your computer.
*    Hex Value:    0x8004B823
*/
#define MSPR_E_HARDWAREID_MISMATCH                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0023)

/*
*    MessageId:    MSPR_E_PARAMETERS_MISMATCHED
*    Message Meaning:
*        A problem has occurred in the Microsoft PlayReady component.
*        Contact Microsoft product support.
*    Hex Value:    0x8004B824
*/
#define MSPR_E_PARAMETERS_MISMATCHED                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0024)

/*
*    MessageId:    MSPR_E_STUBLIB_REQUIRED
*    Message Meaning:
*        A Microsoft issued stub library is required to access the requested functionality.
*    Hex Value:    0x8004B825
*/
#define MSPR_E_STUBLIB_REQUIRED                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0025)

/*
*    MessageId:    MSPR_E_DEBUGGING_NOT_ALLOWED
*    Message Meaning:
*        Running this process under a debugger while using protected content is not allowed.
*    Hex Value:    0x8004B82A
*/
#define MSPR_E_DEBUGGING_NOT_ALLOWED                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x002A)

/*
*    MessageId:    MSPR_E_NO_RIGHTS
*    Message Meaning:
*        The requested operation cannot be performed on this file.
*    Hex Value:    0x8004B82B
*/
#define MSPR_E_NO_RIGHTS                                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x002B)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_PROPERTY
*    Message Meaning:
*        The given property is not supported.
*    Hex Value:    0x8004B82C
*/
#define MSPR_E_UNSUPPORTED_PROPERTY                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x002C)

/*
*    MessageId:    MSPR_E_INTERNAL_SERVER_ERROR
*    Message Meaning:
*        The server encountered an unexpected condition which prevented it from fulfilling the request.
*    Hex Value:    0x8004B82D
*/
#define MSPR_E_INTERNAL_SERVER_ERROR                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x002D)

/*
*    MessageId:    MSPR_E_UNABLE_TO_OPEN_DATA_STORE
*    Message Meaning:
*        A problem has occurred in opening the PlayReady data storage file.
*        Contact Microsoft product support.
*    Hex Value:    0x8004B82E
*/
#define MSPR_E_UNABLE_TO_OPEN_DATA_STORE                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x002E)

/*
*    MessageId:    MSPR_E_CERTIFICATE_REVOKED
*    Message Meaning:
*        The client certificate has been revoked.
*    Hex Value:    0x8004B82F
*/
#define MSPR_E_CERTIFICATE_REVOKED                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x002F)

/*
*    MessageId:    MSPR_E_INVALID_DATA
*    Message Meaning:
*        Invalid or corrupt data was encountered.
*    Hex Value:    0x8004B830
*/
#define MSPR_E_INVALID_DATA                                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0030)

/*
*    MessageId:    MSPR_E_INDIV_SERVER_UNKNOWN_ERROR
*    Message Meaning:
*        An unrecognized error response was returned from the individualization server.
*    Hex Value:    0x8004B831
*/
#define MSPR_E_INDIV_SERVER_UNKNOWN_ERROR                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0031)

/*
*    MessageId:    MSPR_E_INSUFFICIENT_DATA
*    Message Meaning:
*        Insufficient data was found.
*    Hex Value:    0x8004B832
*/
#define MSPR_E_INSUFFICIENT_DATA                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0032)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_ACTION
*    Message Meaning:
*        The requested action is not supported.
*    Hex Value:    0x8004B833
*/
#define MSPR_E_UNSUPPORTED_ACTION                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0033)

/*
*    MessageId:    MSPR_E_LIC_NEEDS_DEVICE_CLOCK_SET
*    Message Meaning:
*        The file could not be transferred because the device clock is not set.
*    Hex Value:    0x8004B834
*/
#define MSPR_E_LIC_NEEDS_DEVICE_CLOCK_SET                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0034)

/*
*    MessageId:    MSPR_E_LICENSE_EXPIRED
*    Message Meaning:
*        The license for this file has expired and is no longer valid.
*        Contact your content provider for further assistance.
*    Hex Value:    0x8004B835
*/
#define MSPR_E_LICENSE_EXPIRED                                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0035)

/*
*    MessageId:    MSPR_E_INVALID_APPLICATION
*    Message Meaning:
*        A problem has occurred in the PlayReady component.
*    Hex Value:    0x8004B836
*/
#define MSPR_E_INVALID_APPLICATION                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0036)

/*
*    MessageId:    MSPR_E_PROTOCOL_FORCEFUL_TERMINATION_ON_PETITION
*    Message Meaning:
*        The client application has been forcefully terminated during a PlayReady petition.
*    Hex Value:    0x8004B837
*/
#define MSPR_E_PROTOCOL_FORCEFUL_TERMINATION_ON_PETITION            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0037)

/*
*    MessageId:    MSPR_E_PROTOCOL_FORCEFUL_TERMINATION_ON_CHALLENGE
*    Message Meaning:
*        The client application has been forcefully terminated during a PlayReady challenge.
*    Hex Value:    0x8004B838
*/
#define MSPR_E_PROTOCOL_FORCEFUL_TERMINATION_ON_CHALLENGE           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0038)

/*
*    MessageId:    MSPR_E_REDIRECT
*    Message Meaning:
*        The client has been redirected to another server.
*    Hex Value:    0x8004B839
*/
#define MSPR_E_REDIRECT                                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0039)

/*
*    MessageId:    MSPR_E_SERVER_UNAVAILABLE
*    Message Meaning:
*        The server is currently unable to handle the request due
*        to a temporary overload or maintenance of the server.
*    Hex Value:    0x8004B83A
*/
#define MSPR_E_SERVER_UNAVAILABLE                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x003A)

/*
*    MessageId:    MSPR_E_ERROR_FROM_PROXY
*    Message Meaning:
*        The proxy experienced an error while attempting to contact the media server.
*    Hex Value:    0x8004B83B
*/
#define MSPR_E_ERROR_FROM_PROXY                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x003B)

/*
*    MessageId:    MSPR_E_PROXY_TIMEOUT
*    Message Meaning:
*        The proxy did not receive a timely response while attempting to contact the media server.
*    Hex Value:    0x8004B83C
*/
#define MSPR_E_PROXY_TIMEOUT                                        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x003C)

/*
*    MessageId:    MSPR_E_SERVER_ACCESSDENIED
*    Message Meaning:
*        The http server is denying access.  The username and/or password might be incorrect.
*    Hex Value:    0x8004B83D
*/
#define MSPR_E_SERVER_ACCESSDENIED                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x003D)

/*
*    MessageId:    MSPR_E_PROXY_ACCESSDENIED
*    Message Meaning:
*        The proxy server is denying access.  The username and/or password might be incorrect.
*    Hex Value:    0x8004B83E
*/
#define MSPR_E_PROXY_ACCESSDENIED                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x003E)

/*
*    MessageId:    MSPR_E_OFFLINE_MODE
*    Message Meaning:
*        The requested URL is not available in offline mode.
*    Hex Value:    0x8004B83F
*/
#define MSPR_E_OFFLINE_MODE                                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x003F)

/*
*    MessageId:    MSPR_E_CURL_INVALIDHOSTNAME
*    Message Meaning:
*        The URL contains a host name that is not valid.
*    Hex Value:    0x8004B840
*/
#define MSPR_E_CURL_INVALIDHOSTNAME                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0040)

/*
*    MessageId:    MSPR_E_ERROR_BAD_NET_RESP
*    Message Meaning:
*        The specified server cannot perform the requested operation.
*    Hex Value:    0x8004B841
*/
#define MSPR_E_ERROR_BAD_NET_RESP                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0041)

/*
*    MessageId:    MSPR_E_BAD_SERVER_REQUEST
*    Message Meaning:
*        The request could not be understood by the server.
*    Hex Value:    0x8004B842
*/
#define MSPR_E_BAD_SERVER_REQUEST                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0042)

/*
*    MessageId:    MSPR_E_UNABLE_TO_VERIFY_PROXIMITY
*    Message Meaning:
*        The proximity detection procedure could not confirm that the
*        receiver is near the transmitter in the network.
*    Hex Value:    0x8004B844
*/
#define MSPR_E_UNABLE_TO_VERIFY_PROXIMITY                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0044)

/*
*    MessageId:    MSPR_E_INVALID_PROXIMITY_RESPONSE
*    Message Meaning:
*        The response to the proximity detection challenge is invalid.
*    Hex Value:    0x8004B845
*/
#define MSPR_E_INVALID_PROXIMITY_RESPONSE                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0045)

/*
*    MessageId:    MSPR_E_INVALID_SESSION
*    Message Meaning:
*        The requested session is invalid.
*    Hex Value:    0x8004B846
*/
#define MSPR_E_INVALID_SESSION                                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0046)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_PROTOCOL_VERSION
*    Message Meaning:
*        The protocol version is unsupported.
*    Hex Value:    0x8004B847
*/
#define MSPR_E_UNSUPPORTED_PROTOCOL_VERSION                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0047)

/*
*    MessageId:    MSPR_E_CERTIFICATE_NOT_SAMPLE_PROTECTION
*    Message Meaning:
*        The given certificate is not a sample protection certificate.
*    Hex Value:    0x8004B848
*/
#define MSPR_E_CERTIFICATE_NOT_SAMPLE_PROTECTION                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0048)

/*
*    MessageId:    MSPR_E_UNABLE_TO_SET_SECURE_CLOCK
*    Message Meaning:
*        A problem has occurred in setting the device's secure clock.
*        Contact Microsoft product support.
*    Hex Value:    0x8004B84A
*/
#define MSPR_E_UNABLE_TO_SET_SECURE_CLOCK                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x004A)

/*
*    MessageId:    MSPR_E_BAD_REQUEST
*    Message Meaning:
*        The message format is invalid.
*    Hex Value:    0x8004B84C
*/
#define MSPR_E_BAD_REQUEST                                          MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x004C)

/*
*    MessageId:    MSPR_E_LICENSE_NOTENABLED
*    Message Meaning:
*        The license for this file is not valid yet but will be at a future date.
*    Hex Value:    0x8004B84D
*/
#define MSPR_E_LICENSE_NOTENABLED                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x004D)

/*
*    MessageId:    MSPR_E_LICENSE_APPSECLOW
*    Message Meaning:
*        The license for this file requires a higher level of security
*        than the player you are currently using has.
*        Try using a different player or download a newer version of your current player.
*    Hex Value:    0x8004B84E
*/
#define MSPR_E_LICENSE_APPSECLOW                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x004E)

/*
*    MessageId:    MSPR_E_RESTRICTED_SOURCE
*    Message Meaning:
*        The license for this file requires your application
*        to have special licensing terms with Microsoft.
*        Contact your content provider for support.
*    Hex Value:    0x8004B84F
*/
#define MSPR_E_RESTRICTED_SOURCE                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x004F)

/*
*    MessageId:    MSPR_E_HWID_REGISTRY_OVERRIDE_NOT_PRESENT
*    Message Meaning:
*        There is no registry override available for the HWID.
*    Hex Value:    0x8004B850
*/
#define MSPR_E_HWID_REGISTRY_OVERRIDE_NOT_PRESENT                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0050)

/*
*    MessageId:    MSPR_E_POLICY_CONVERSION_FAILURE
*    Message Meaning:
*        The current license policy could not be converted.
*    Hex Value:    0x8004B853
*/
#define MSPR_E_POLICY_CONVERSION_FAILURE                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0053)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_OPL
*    Message Meaning:
*        The Output Protection Level value specified in the license is not supported.
*    Hex Value:    0x8004B854
*/
#define MSPR_E_UNSUPPORTED_OPL                                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0054)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_CONFIG_DATA
*    Message Meaning:
*        The configuration data associate with a protection type is not supported.
*    Hex Value:    0x8004B855
*/
#define MSPR_E_UNSUPPORTED_CONFIG_DATA                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0055)

/*
*    MessageId:    MSPR_E_KEY_FILE_NOTFOUND
*    Message Meaning:
*        Overriding the individualized binary file requires a
*        key file to exist, but one was not found.
*    Hex Value:    0x8004B856
*/
#define MSPR_E_KEY_FILE_NOTFOUND                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0056)

/*
*    MessageId:    MSPR_E_DEVICE_ERROR_INVALID
*    Message Meaning:
*        The device returned a PROPVARIANT of VT_ERROR but the scode value
*        was not recognized as a failure HRESULT.
*    Hex Value:    0x8004B858
*/
#define MSPR_E_DEVICE_ERROR_INVALID                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0058)

/*
*    MessageId:    MSPR_E_DEVICE_EMPTY_INVALID
*    Message Meaning:
*        The device returned a PROPVARIANT of VT_EMPTY which was not expected.
*    Hex Value:    0x8004B859
*/
#define MSPR_E_DEVICE_EMPTY_INVALID                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0059)

/*
*    MessageId:    MSPR_E_DEVICE_RETURN_TYPE_INVALID
*    Message Meaning:
*        The device returned a PROPVARIANT of an unexpected type
*        which was neither VT_ERROR nor VT_EMPTY.
*    Hex Value:    0x8004B85A
*/
#define MSPR_E_DEVICE_RETURN_TYPE_INVALID                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x005A)

/*
*    MessageId:    MSPR_E_DEVICE_MTP_ERROR_INVALID
*    Message Meaning:
*        The device returned MTP_RESPONSECODE_J_FAIL but the first
*        MTP out parameter was not recognized as an error code.
*    Hex Value:    0x8004B85B
*/
#define MSPR_E_DEVICE_MTP_ERROR_INVALID                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x005B)

/*
*    MessageId:    MSPR_E_DEVICE_MTP_UNRECOGNIZED_RESPONSE_CODE
*    Message Meaning:
*        The device returned a response code other than
*        MTP_RESPONSECODE_OK and MTP_RESPONSECODE_J_FAIL and that
*        response code could not be converted to an error code.
*    Hex Value:    0x8004B85C
*/
#define MSPR_E_DEVICE_MTP_UNRECOGNIZED_RESPONSE_CODE                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x005C)

/*
*    MessageId:    MSPR_E_DEVICE_MTP_RESPONSE_NO_OUT_PARAMS
*    Message Meaning:
*        The device returned response code MTP_RESPONSECODE_OK
*        or MTP_RESPONSECODE_J_FAIL but did not return at least
*        one out parameter for a success or error code.
*    Hex Value:    0x8004B85D
*/
#define MSPR_E_DEVICE_MTP_RESPONSE_NO_OUT_PARAMS                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x005D)

/*
*    MessageId:    MSPR_E_DEVICE_MTP_RESPONSE_INCOMPLETE
*    Message Meaning:
*        The device returned MTP_RESPONSECODE_OK but did not
*        return enough out params for the MTP extension being used.
*    Hex Value:    0x8004B85E
*/
#define MSPR_E_DEVICE_MTP_RESPONSE_INCOMPLETE                       MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x005E)

/*
*    MessageId:    MSPR_E_DEVICE_MTP_VENDOR_EXTENSION_MALFORMED
*    Message Meaning:
*        The device returned the vendor extension in an unrecognized format.
*    Hex Value:    0x8004B85F
*/
#define MSPR_E_DEVICE_MTP_VENDOR_EXTENSION_MALFORMED                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x005F)

/*
*    MessageId:    MSPR_E_DEVICE_MTP_PROPERTY_EMPTY
*    Message Meaning:
*        The device returned a sized MTP device property with
*        a valid size but with no data.
*    Hex Value:    0x8004B860
*/
#define MSPR_E_DEVICE_MTP_PROPERTY_EMPTY                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0060)

/*
*    MessageId:    MSPR_E_DEVICEAPP_ALREADY_INITIALIZED
*    Message Meaning:
*        The interface has already been initialized with a device pointer.
*    Hex Value:    0x8004B861
*/
#define MSPR_E_DEVICEAPP_ALREADY_INITIALIZED                        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0061)

/*
*    MessageId:    MSPR_E_DEVICEAPP_NOT_INITIALIZED
*    Message Meaning:
*        The interface has not been initialized with a device pointer.
*    Hex Value:    0x8004B862
*/
#define MSPR_E_DEVICEAPP_NOT_INITIALIZED                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0062)

/*
*    MessageId:    MSPR_E_DEVICE_CERTIFICATE_NOT_SET
*    Message Meaning:
*        The property requested requires the device certificate to be set.
*    Hex Value:    0x8004B863
*/
#define MSPR_E_DEVICE_CERTIFICATE_NOT_SET                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0063)

/*
*    MessageId:    MSPR_E_DIRECTORY_UNEXPECTED
*    Message Meaning:
*        A directory was encountered instead of an expected file.
*    Hex Value:    0x8004B864
*/
#define MSPR_E_DIRECTORY_UNEXPECTED                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0064)

/*
*    MessageId:    MSPR_E_NO_STUBLIB_INTERFACE
*    Message Meaning:
*        The specified interface does not utilize a stublib.
*    Hex Value:    0x8004B865
*/
#define MSPR_E_NO_STUBLIB_INTERFACE                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0065)

/*
*    MessageId:    MSPR_E_SECURE_FILE_COPY_ERROR
*    Message Meaning:
*        There was an error creating the a secured copy of a file.
*    Hex Value:    0x8004B866
*/
#define MSPR_E_SECURE_FILE_COPY_ERROR                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0066)

/*
*    MessageId:    MSPR_E_ROOT_LICENSE_DECRYPTOR_UNSUPPORTED
*    Message Meaning:
*        The application attempted to create a decryptor for a root license.
*        This is not a supported scenario because root licenses do not encrypt content.
*    Hex Value:    0x8004B867
*/
#define MSPR_E_ROOT_LICENSE_DECRYPTOR_UNSUPPORTED                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0067)

/*
*    MessageId:    MSPR_E_SDK_UPDATE_REQUIRED
*    Message Meaning:
*        The client SDK (msprsdk.dll) is out of date and must be updated.
*    Hex Value:    0x8004B868
*/
#define MSPR_E_SDK_UPDATE_REQUIRED                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0068)

/*
*    MessageId:    MSPR_E_CHKDR_DEBUG_SETUP_ERROR
*    Message Meaning:
*        The ChkDR() debug function shim encountered errors while setting up its error debug data.
*    Hex Value:    0x8004B869
*/
#define MSPR_E_CHKDR_DEBUG_SETUP_ERROR                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0069)


/*
*    MessageId:    MSPR_E_CHKDR_DEBUG_REGISTRY_ERROR
*    Message Meaning:
*        The ChkDR() debug function shim encountered errors while reading the registry.
*    Hex Value:    0x8004B86A
*/
#define MSPR_E_CHKDR_DEBUG_REGISTRY_ERROR                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x006A)

/*
*    MessageId:    MSPR_E_LICENSE_REQUIRES_NEWER_DEVICE_FIRMWARE
*    Message Meaning:
*        A license would have been able to be transferred to the device,
*        but the device is a legacy PlayReady device and does not support
*        the necessary functionality to use the available license.
*        Contact your device manufacturer for assistance.
*    Hex Value:    0x8004B86B
*/
#define MSPR_E_LICENSE_REQUIRES_NEWER_DEVICE_FIRMWARE               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x006B)

/*
*    MessageId:    MSPR_E_INDIVBOX_NOT_LOADED
*    Message Meaning:
*        The IBX referenced by the runtime is no longer loaded in memory.
*    Hex Value:    0x8004B86C
*/
#define MSPR_E_INDIVBOX_NOT_LOADED                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x006C)

/*
*    MessageId:    MSPR_E_INDIVBOX_MODULE_OBJECT_MISMATCH
*    Message Meaning:
*        When calling a function on an SDK object created by a given IBX, an object passed
*        in as a parameter was detected as not having been created by the same IBX.
*    Hex Value:    0x8004B86D
*/
#define MSPR_E_INDIVBOX_MODULE_OBJECT_MISMATCH                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x006D)

/*
*    MessageId:    MSPR_E_HTTP_RESPONSE_TOO_LARGE
*    Message Meaning:
*        The HTTP response received is too large.
*    Hex Value:    0x8004B86E
*/
#define MSPR_E_HTTP_RESPONSE_TOO_LARGE                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x006E)

/*
*    MessageId:    MSPR_E_CANNOT_PERSIST_INVALID
*    Message Meaning:
*        The license is marked CANNOT_PERSIST but it is not a temporary license.
*    Hex Value:    0x8004B86F
*/
#define MSPR_E_CANNOT_PERSIST_INVALID                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x006F)

/*
*    MessageId:    MSPR_E_FILTER_UNPROTECTED
*    Message Meaning:
*        The license filter used to enumerate the license was unprotected.
*    Hex Value:    0x8004B870
*/
#define MSPR_E_FILTER_UNPROTECTED                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0070)

/*
*    MessageId:    MSPR_E_HTTP_MESSAGE_MISMATCH
*    Message Meaning:
*        The http messages given have inconsistent types,
*        e.g. indiv petition challenge with indiv response.
*    Hex Value:    0x8004B871
*/
#define MSPR_E_HTTP_MESSAGE_MISMATCH                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0071)

/*
*    MessageId:    MSPR_E_CONTENT_KEY_NOT_SET
*    Message Meaning:
*        The content key was not set in the content key object.
*    Hex Value:    0x8004B872
*/
#define MSPR_E_CONTENT_KEY_NOT_SET                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0072)

/*
*    MessageId:    MSPR_E_CONTENT_KEY_SEED_NOT_SET
*    Message Meaning:
*        The Seed property was not set in the content key object because Seed was not used to generate the content key.
*    Hex Value:    0x8004B873
*/
#define MSPR_E_CONTENT_KEY_SEED_NOT_SET                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0073)

/*
*    MessageId:    MSPR_E_SERVER_AUTHORIZATION_REQUIRED
*    Message Meaning:
*        An attempt to use a license with a right that requires Microsoft service authorization was made, without first
*        completing the authorization.
*    Hex Value:    0x8004B874
*/
#define MSPR_E_SERVER_AUTHORIZATION_REQUIRED                        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0074)

/*
*    MessageId:    MSPR_E_PRO_HEADER_UNRECOGNIZED_ENCRYPTION_TYPE
*    Message Meaning:
*        The PlayReady object header has an unrecognized encryption type.
*    Hex Value:    0x8004B875
*/
#define MSPR_E_PRO_HEADER_UNRECOGNIZED_ENCRYPTION_TYPE              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0075)

/*
*    MessageId:    MSPR_E_SECURE_FILE_RECOVERY_ERROR
*    Message Meaning:
*        There was an error trying to recover from a secure file copy error.
*    Hex Value:    0x8004B876
*/
#define MSPR_E_SECURE_FILE_RECOVERY_ERROR                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0076)

/*
*    MessageId:    MSPR_E_CLOCK_ROLLBACK_POLICY_ENFORCED
*    Message Meaning:
*        Policy related to clock rollback detection is being enforced.  This could result in existing licenses not being usable.
*    Hex Value:    0x8004B878
*/
#define MSPR_E_CLOCK_ROLLBACK_POLICY_ENFORCED                       MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0078)

/*
*    MessageId:    MSPR_E_MOVE_LIST_REFRESH_REQUIRED
*    Message Meaning:
*        The client move list needs to be refreshed in order to use licenses with move enablers.
*    Hex Value:    0x8004B87A
*/
#define MSPR_E_MOVE_LIST_REFRESH_REQUIRED                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x007A)

/*
*    MessageId:    MSPR_E_UNABLE_TO_STORE_CANNOT_PERSIST_LICENSE
*    Message Meaning:
*        A license with the CANNOT PERSIST flag can not be stored in permanant or temporary storage.
*    Hex Value:    0x8004B87B
*/
#define MSPR_E_UNABLE_TO_STORE_CANNOT_PERSIST_LICENSE               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x007B)

/*
*    MessageId:    MSPR_E_LICENSE_AND_STORE_TYPE_MISMATCH
*    Message Meaning:
*        Properties of the license do not match the requested store type.  For example, this can be returned if a temporary license is placed in permanent storage.
*    Hex Value:    0x8004B87C
*/
#define MSPR_E_LICENSE_AND_STORE_TYPE_MISMATCH                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x007C)

/*
*    MessageId:    MSPR_E_HWID_OBJECT_PROPERTY_NOT_FOUND
*    Message Meaning:
*        Object property required for the hardware ID calculation not found or is incorrect.
*    Hex Value:    0x8004B87D
*/
#define MSPR_E_HWID_OBJECT_PROPERTY_NOT_FOUND                       MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x007D)

/*
*    MessageId:    MSPR_E_HWID_HDD_UNKNOWN_TYPE
*    Message Meaning:
*        Cannot collect HDD ID information because HDD type is unknown.
*    Hex Value:    0x8004B87E
*/
#define MSPR_E_HWID_HDD_UNKNOWN_TYPE                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x007E)

/*
*    MessageId:    MSPR_E_HWID_HDD_INVALID_BSD_NAME
*    Message Meaning:
*        Cannot collect HDD ID information because HDD BSD name appears to be invalid.
*    Hex Value:    0x8004B87F
*/
#define MSPR_E_HWID_HDD_INVALID_BSD_NAME                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x007F)

/*
*    MessageId:    MSPR_E_HWID_HDD_NO_MATCHING_SERVICE
*    Message Meaning:
*        Cannot collect HDD ID information because matching service object cannot be found for HDD BSD name.
*    Hex Value:    0x8004B880
*/
#define MSPR_E_HWID_HDD_NO_MATCHING_SERVICE                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0080)

/*
*    MessageId:    MSPR_E_HWID_HDD_ERROR_ACCESS_IOREGISTRY
*    Message Meaning:
*        Cannot collect HDD ID information because of an error happened while accessing IO Registry.
*    Hex Value:    0x8004B881
*/
#define MSPR_E_HWID_HDD_ERROR_ACCESS_IOREGISTRY                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0081)

/*
*    MessageId:    MSPR_E_HWID_INVALID_HWID_VERSION
*    Message Meaning:
*        Cannot verify hardware ID because of incompatible version of existing data. Please clean DRM and try Individialization again.
*    Hex Value:    0x8004B882
*/
#define MSPR_E_HWID_INVALID_HWID_VERSION                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0082)

/*
*    MessageId:    MSPR_E_PERSISTENT_ACLS_NOT_SUPPORTED
*    Message Meaning:
*        Persistent ACLS are not supported on the target partition
*    Hex Value:    0x8004B883
*/
#define MSPR_E_PERSISTENT_ACLS_NOT_SUPPORTED                        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0083)

/*
*    MessageId:     MSPR_E_LICENSE_RESPONSE_SIGNATURE_INVALID
*    Message Meaning:
*        Cannot verify license acquisition's response because signature is invalid.
*    Hex Value:    0x8004B884
*/
#define MSPR_E_LICENSE_RESPONSE_SIGNATURE_INVALID                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0084)

/*
*    MessageId:     MSPR_E_LICENSE_RESPONSE_RESPONSEID_INVALID
*    Message Meaning:
*        Cannot verify license acquisition's response because response ID is invalid.
*    Hex Value:    0x8004B885
*/
#define MSPR_E_LICENSE_RESPONSE_RESPONSEID_INVALID                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0085)

/*
*    MessageId:     MSPR_E_LICENSE_RESPONSE_SIGNATURE_MISSING
*    Message Meaning:
*        Cannot verify license acquisition's response because either response ID, license nonce or signature is missing.
*    Hex Value:    0x8004B886
*/
#define MSPR_E_LICENSE_RESPONSE_SIGNATURE_MISSING                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0086)

/*
*    MessageId:    MSPR_E_MUST_UNDERSTAND_RESTRICTION_UNPROCESSED
*    Message Meaning:
*        The license had a must understand restriction that was not processed.
*    Hex Value:    0x8004B887
*/
#define MSPR_E_MUST_UNDERSTAND_RESTRICTION_UNPROCESSED              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0087)

/*
*    MessageId:    MSPR_E_NO_RESTRICTION
*    Message Meaning:
*        No restriction corresponding to the restriction type or index specified was found.
*    Hex Value:    0x8004B888
*/
#define MSPR_E_NO_RESTRICTION                                       MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0088)

/*
*    MessageId:     MSPR_E_DOMAIN_JOIN_RESPONSE_SIGNATURE_INVALID
*    Message Meaning:
*        Cannot verify domain join's response because signature is invalid.
*    Hex Value:    0x8004B889
*/
#define MSPR_E_DOMAIN_JOIN_RESPONSE_SIGNATURE_INVALID               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0089)

/*
*    MessageId:     MSPR_E_DOMAIN_JOIN_RESPONSE_SIGNATURE_MISSING
*    Message Meaning:
*        Cannot verify domain join's response because either signature or certificate chain is missing.
*    Hex Value:    0x8004B88A
*/
#define MSPR_E_DOMAIN_JOIN_RESPONSE_SIGNATURE_MISSING               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x008A)

/*
*    MessageId:    MSPR_E_NOT_YET_COMMITTED
*    Message Meaning:
*        The desired action cannot be performed because an associated object has not been committed yet.
*    Hex Value:    0x8004B88B
*/
#define MSPR_E_NOT_YET_COMMITTED                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x008B)

/*
*    MessageId:    MSPR_E_ALREADY_COMMITTED
*    Message Meaning:
*        The desired action cannot be performed on a committed object
*    Hex Value:    0x8004B88C
*/
#define MSPR_E_ALREADY_COMMITTED                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x008C)

/*
*    MessageId:    MSPR_E_REQUIRED_PROPERTY_NOT_SET
*    Message Meaning:
*        The desired action cannot be performed because a required property
*        has not yet been set.
*    Hex Value:    0x8004B88D
*/
#define MSPR_E_REQUIRED_PROPERTY_NOT_SET                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x008D)

/*
*    MessageId:    MSPR_E_INVALID_PROPERTY_VALUE
*    Message Meaning:
*        The value for a property of the object is invalid
*    Hex Value:    0x8004B88E
*/
#define MSPR_E_INVALID_PROPERTY_VALUE                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x008E)

/*
*    MessageId:     MSPR_E_DERIVED_KEY_LICENSE
*    Message Meaning:
*        The license or license chain doesn't have a valid AuxKey and/or UnlinkX XMR object to derive the key.
*    Hex Value:     0x8004B88F
*/
#define MSPR_E_DERIVED_KEY_LICENSE                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x008F)

/*
*    MessageId:    MSPR_E_MULTIPLE_STORE_CONTEXTS_FOR_TYPE
*    Message Meaning:
*        Multiple store contexts exist for the specified type
*    Hex Value:    0x8004B891
*/
#define MSPR_E_MULTIPLE_STORE_CONTEXTS_FOR_TYPE                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0091)

/*
*    MessageId:    MSPR_E_LICENSE_STORE_SET_FOR_CHAIN_DEPTH
*    Message Meaning:
*        The license stores are set for chain depths (not all depths)
*    Hex Value:    0x8004B892
*/
#define MSPR_E_LICENSE_STORE_SET_FOR_CHAIN_DEPTH                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0092)

/*
*    MessageId:    MSPR_E_SCALABLE_LICENSE_NOT_ALLOWED_FOR_WMDRMND
*    Message Meaning:
*        Scalable license is not allowed for ND-Streaming
*    Hex Value:    0x8004B893
*/
#define MSPR_E_SCALABLE_LICENSE_NOT_ALLOWED_FOR_WMDRMND             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0093)

/*
*    MessageId:    MSPR_E_MODULE_INIT_FAILURE
*    Message Meaning:
*        PlayReady module initialization failed.
*    Hex Value:    0x8004B894
*/
#define MSPR_E_MODULE_INIT_FAILURE                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0094)

/*
*    MessageId:    MSPR_E_CONTENT_ENABLING_ACTION_REQUIRED
*    Message Meaning:
*        A new content enabling action is required before the protected content can be used.
*    Hex Value:    0x8004B895
*/
#define MSPR_E_CONTENT_ENABLING_ACTION_REQUIRED                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0095)

/*
*    MessageId:    MSPR_E_NO_DECRYPTOR_AVAILABLE
*    Message Meaning:
*        Failure occurred when attempting to find a correct PlayReady decryptor object - no decryptor was found.
*    Hex Value:    0x8004B896
*/
#define MSPR_E_NO_DECRYPTOR_AVAILABLE                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0096)

/*
*    MessageId:    MSPR_E_MAX_CE_ATTEMPTS_EXCEEDED
*    Message Meaning:
*        Too many repeat content enabling operations attempted. PlayReady license server may be improperly configured.
*    Hex Value:    0x8004B897
*/
#define MSPR_E_MAX_CE_ATTEMPTS_EXCEEDED                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0097)

/*
*    MessageId:    MSPR_E_DOMAIN_JOIN_REQUIRED
*    Message Meaning:
*        The desired action requires a domain join operation.
*    Hex Value:    0x8004B898
*/
#define MSPR_E_DOMAIN_JOIN_REQUIRED                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0098)

/*
*    MessageId:    MSPR_E_NO_LICENSE_DERIVATION_PATH
*    Message Meaning:
*        The root license does not contain a derivation path for the desired action.
*    Hex Value:    0x8004B899
*/
#define MSPR_E_NO_LICENSE_DERIVATION_PATH                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x0099)

/*
*    MessageId:    MSPR_E_NO_ROOT_LICENSE_RIGHTS
*    Message Meaning:
*        No root license available for the desired action.
*    Hex Value:    0x8004B89A
*/
#define MSPR_E_NO_ROOT_LICENSE_RIGHTS                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x009A)

/*
*    MessageId:    MSPR_E_REDIRECT_FAILURE
*    Message Meaning:
*        DRM challege was redirected by server too many times.
*    Hex Value:    0x8004B89B
*/
#define MSPR_E_REDIRECT_FAILURE                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x009B)

/*
*    MessageId:    MSPR_E_NO_METERING_DATA_AVAILABLE
*    Message Meaning:
*        A metering challenge was requested but there is no data associated with the given MID to report.
*    Hex Value:    0x8004B89C
*/
#define MSPR_E_NO_METERING_DATA_AVAILABLE                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x009C)

/*
*    MessageId:    MSPR_E_PREC_SERVER_RETURNED_ERROR
*    Message Meaning:
*        Provisioning server returned an error.
*    Hex Value:    0x8004B89D
*/
#define MSPR_E_PREC_SERVER_RETURNED_ERROR                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x009D)

/*
*    MessageId:    MSPR_E_PREC_INVALID_RESPONSE_MESSAGE
*    Message Meaning:
*        Provisioning response message contains invalid data.
*    Hex Value:    0x8004B89E
*/
#define MSPR_E_PREC_INVALID_RESPONSE_MESSAGE                        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x009E)

/*
*    MessageId:    MSPR_E_PREC_NO_REQUEST_CREATED
*    Message Meaning:
*        We tried to use a PREC response, but no request was created.
*    Hex Value:    0x8004B89F
*/
#define MSPR_E_PREC_NO_REQUEST_CREATED                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x009F)

/*
*    MessageId:    MSPR_E_PREC_RESPONSE_EXPIRED
*    Message Meaning:
*       PREC response received has already expired.
*    Hex Value:    0x8004B8A0
*/
#define MSPR_E_PREC_RESPONSE_EXPIRED                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A0)

/*
*    MessageId:    MSPR_E_PREC_CERT_EXPIRED
*    Message Meaning:
*        PREC is expired.
*    Hex Value:    0x8004B8A1
*/
#define MSPR_E_PREC_CERT_EXPIRED                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A1)

/*
*    MessageId:    MSPR_E_PREC_NONCE_MISMATCH
*    Message Meaning:
*        Bad PREC response was presented.
*    Hex Value:    0x8004B8A2
*/
#define MSPR_E_PREC_NONCE_MISMATCH                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A2)

/*
*    MessageId:    MSPR_E_PREC_BAD_RESPONSE_VERSION
*    Message Meaning:
*        PREC response of unexpected version presented.
*    Hex Value:    0x8004B8A3
*/
#define MSPR_E_PREC_BAD_RESPONSE_VERSION                            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A3)

/*
*    MessageId:    MSPR_E_PREC_BAD_RESPONSE
*    Message Meaning:
*        Malformed PREC response.
*    Hex Value:    0x8004B8A4
*/
#define MSPR_E_PREC_BAD_RESPONSE                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A4)

/*
*    MessageId:    MSPR_E_PREC_CRYPTO_ERROR
*    Message Meaning:
*        An unexpected cryptographic failure.
*    Hex Value:    0x8004B8A5
*/
#define MSPR_E_PREC_CRYPTO_ERROR                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A5)

/*
*    MessageId:    MSPR_E_PREC_PROVISIONING_REQUIRED
*    Message Meaning:
*        Provisioning has not been done on the machine.
*    Hex Value:    0x8004B8A6
*/
#define MSPR_E_PREC_PROVISIONING_REQUIRED                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A6)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_KEYSYSTEM
*    Message Meaning:
*        The Key System ID is not supported by PlayReady.
*    Hex Value:    0x8004B8A7
*/
#define MSPR_E_UNSUPPORTED_KEYSYSTEM                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A7)

/*
*    MessageId:    MSPR_E_UNSUPPORTED_CDMDATA_TYPE
*    Message Meaning:
*        The CDMData type is not supported by PlayReady. The CDMData type should be LicenseAcquisition.
*    Hex Value:    0x8004B8A8
*/
#define MSPR_E_UNSUPPORTED_CDMDATA_TYPE                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A8)

/*
*    MessageId:    MSPR_E_INVALID_CDMDATA_KEYID
*    Message Meaning:
*        The CDMData contains a KeyID that's already been set by initData, or no keyId was provided at all.
*    Hex Value:    0x8004B8A9
*/
#define MSPR_E_INVALID_CDMDATA_KEYID                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00A9)

/*
*    MessageId:    MSPR_E_DOMAIN_NOT_SUPPORTED
*    Message Meaning:
*       Domain is not supported.
*    Hex Value:    0x8004B8AA
*/
#define MSPR_E_DOMAIN_NOT_SUPPORTED                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00AA)

/*
*    MessageId:    MSPR_E_PREC_CERT_NOT_AVAILABLE
*    Message Meaning:
*        PREC is not available.
*    Hex Value:    0x8004B8AB
*/
#define MSPR_E_PREC_CERT_NOT_AVAILABLE                              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00AB)

/*
*    MessageId:    MSPR_E_INVALID_PROACTIVE_LICENSE_ACQUISITION
*    Message Meaning:
*       Proactive License Acquisition is not allowed when Content Decryption Module is attached to a media source.
*    Hex Value:    0x8004B8AC
*/
#define MSPR_E_INVALID_PROACTIVE_LICENSE_ACQUISITION                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00AC)

/*
*    MessageId:    MSPR_E_INVALID_CDMSESSION
*    Message Meaning:
*       Only one reactive CDM Session can be attached to a playback session at a time.
*    Hex Value:    0x8004B8AD
*/
#define MSPR_E_INVALID_CDMSESSION                                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00AD)

/*
*    MessageId:    MSPR_E_PRND_PROTOCOL_NOT_SUPPORTED
*    Message Meaning:
*        PRND transmitter doesn't support requested protocol.
*    Hex Value:    0x8004B8AE
*/
#define MSPR_E_PRND_PROTOCOL_NOT_SUPPORTED                          MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00AE)

/*
*    MessageId:    MSPR_E_PRND_SOCKET_NOT_READY
*    Message Meaning:
*        The total number of socket handles that are ready and contained in the fd_set structures is zero (i.e. the time limit expired)
*    Hex Value:    0x8004B8AF
*/
#define MSPR_E_PRND_SOCKET_NOT_READY                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00AF)

/*
*    MessageId:    MSPR_E_PRND_START_NOT_ALLOWED
*    Message Meaning:
*        Stop the client first before calling the StartAsync.
*    Hex Value:    0x8004B8B0
*/
#define MSPR_E_PRND_START_NOT_ALLOWED                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B0)

/*
*    MessageId:    MSPR_E_PRND_HOST_CERTIFICATE_REJECTED
*    Message Meaning:
*        The PlayReady ND remote host's certificate was rejected (by the application)
*    Hex Value:    0x8004B8B1
*/
#define MSPR_E_PRND_HOST_CERTIFICATE_REJECTED                       MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B1)

/*
*    MessageId:    MSPR_E_FILTER_EMBEDDED_LICENSE_NOT_SET
*    Message Meaning:
*        The License filter does not have a license object.
*    Hex Value:    0x8004B8B2
*/
#define MSPR_E_FILTER_EMBEDDED_LICENSE_NOT_SET                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B2)

/*
*    MessageId:    MSPR_E_PRND_SESSION_NOT_READY
*    Message Meaning:
*        The PlayReady ND session is not ready yet.
*    Hex Value:    0x8004B8B3
*/
#define MSPR_E_PRND_SESSION_NOT_READY                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B3)

/*
*    MessageId:    MSPR_E_IBX_DATAPATH_REQUIRED
*    Message Meaning:
*        PlayReady requires an IBX data store path.
*    Hex Value:    0x8004B8B4
*/
#define MSPR_E_IBX_DATAPATH_REQUIRED                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B4)

/*
*    MessageId:    MSPR_E_INVALID_MODE_SWITCH
*    Message Meaning:
*        Invalid switch between hardware and software mode. Make sure all interfaces are released before switching.
*    Hex Value:    0x8004B8B5
*/
#define MSPR_E_INVALID_MODE_SWITCH                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B5)


/*
*    MessageId:    MSPR_E_H264_FORMAT_ERROR
*    Message Meaning:
*        There is error with H264 content's format.
*    Hex Value:    0x8004B8B6
*/
#define MSPR_E_H264_FORMAT_ERROR                                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B6)

/*
*    MessageId:    MSPR_E_PEAUTH_FAILED_HIGH_SECURITY_CONTENT_DENIED
*    Message Meaning:
*        High security content is disallowed when PEAUTH failed.
*    Hex Value:    0x8004B8B7
*/
#define MSPR_E_PEAUTH_FAILED_HIGH_SECURITY_CONTENT_DENIED           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B7)

/*
*    MessageId:    MSPR_E_SECURE_STOP_CERT_SIGNATURE_INVALID
*    Message Meaning:
*        Cannot verify publisher certificate because signature is invalid.
*    Hex Value:    0x8004B8B8
*/
#define MSPR_E_SECURE_STOP_CERT_SIGNATURE_INVALID                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B8)

/*
*    MessageId:    MSPR_E_SECURE_STOP_ID_INVALID
*    Message Meaning:
*        The license acquisition response contains an invalid secure stop ID.
*    Hex Value:    0x8004B8B9
*/
#define MSPR_E_SECURE_STOP_ID_INVALID                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00B9)


/*
*    MessageId:    MSPR_E_MISCONFIGURED_SERVER
*    Message Meaning:
*       PR server insists on client being revoked, but provisioning server insists it is not.
*    Hex Value:    0x8004B8BA
*/
#define MSPR_E_MISCONFIGURED_SERVER                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00BA)

/*
*    MessageId:    MSPR_E_INVALID_H264_CONTENT
*    Message Meaning:
*       The H264 content is invalid and playback cannot continue.
*    Hex Value:    0x8004B8BB
*/
#define MSPR_E_INVALID_H264_CONTENT                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00BB)

/*
*    MessageId:    MSPR_E_INDIV_RETRY_REQUIRED
*    Message Meaning:
*        Another individualization attempt was started while this individualization attempt was in progress.
*        This is not supported.  Retrying the scenario after a delay for the other individualization attempt to complete may resolve this issue.
*        Do not immediately retry individualization, as this may cause each individualization attempt to infinitely loop due to the other
*        attempt causing it to return this error code.  Once individualization has completed successfully, this error will no longer be returned.
*    Hex Value:    0x8004B8BC
*/
#define MSPR_E_INDIV_RETRY_REQUIRED                                 MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00BC)

/*
*    MessageId:    MSPR_E_INVALID_ON_DEMAND_HEADER
*    Message Meaning:
*       This license challenge uses an on-demand header is not allowed.
*    Hex Value:    0x8004B8BD
*/
#define MSPR_E_INVALID_ON_DEMAND_HEADER                             MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00BD)

/*
*    MessageId:    MSPR_E_MISSING_KEY_FILE
*    Message Meaning:
*        The current key file is missing but the data store is present.
*    Hex Value:    0x8004B8BE
*/
#define MSPR_E_MISSING_KEY_FILE                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00BE)

/*
*    MessageId:    MSPR_E_FAILED_RECOVERY_ON_INDIV_CURRENT
*    Message Meaning:
*        The individualizion server indicated that the client was current
*        but the client was unable to recover from this error.
*    Hex Value:    0x8004B8BF
*/
#define MSPR_E_FAILED_RECOVERY_ON_INDIV_CURRENT                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00BF)

/*
*    MessageId:    MSPR_E_STRUCT_NOT_FOUND
*    Message Meaning:
*        Internal structure data was not found.
*    Hex Value:    0x8004B8C0
*/
#define MSPR_E_STRUCT_NOT_FOUND                                     MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C0)

/*
*    MessageId:    MSPR_E_INVALID_CONTENT
*    Message Meaning:
*       The content is invalid and playback cannot continue.
*    Hex Value:    0x8004B8C1
*/
#define MSPR_E_INVALID_CONTENT                                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C1)

/*
*    MessageId:    MSPR_E_INVALID_H264_CONTENT
*    Message Meaning:
*       The H264 content's slice headers are invalid and playback cannot continue.
*    Hex Value:    0x8004B8C2
*/
#define MSPR_E_INVALID_H264_SLICE_HEADERS                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C2)

/*
*    MessageId:    MSPR_E_CDM_PERSISTENT_LICENSE_NOT_ALLOWED
*    Message Meaning:
*       The app is trying to acquire a persistent license with the wrong session type.
*    Hex Value:    0x8004B8C3
*/
#define MSPR_E_CDM_PERSISTENT_LICENSE_NOT_ALLOWED                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C3)

/*
*    MessageId:    MSPR_E_CDM_TEMPORARY_LICENSE_NOT_ALLOWED
*    Message Meaning:
*       The app is trying to acquire a temporary license with the wrong session type.
*    Hex Value:    0x8004B8C4
*/
#define MSPR_E_CDM_TEMPORARY_LICENSE_NOT_ALLOWED                    MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C4)

/*
*    MessageId:    MSPR_E_CMD_PERSISTENT_USAGE_RECORD_NOT_ALLOWED
*    Message Meaning:
*       The app is trying to acquire a temporary securestop license with the wrong session type.
*    Hex Value:    0x8004B8C5
*/
#define MSPR_E_CDM_PERSISTENT_USAGE_RECORD_NOT_ALLOWED              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C5)

/*
*    MessageId:    MSPR_E_LICENSE_UNSUPPORTED_IN_SOFTWARE
*    Message Meaning:
*       The app received a SWDRM license which is only supported when running in HWDRM mode.
*    Hex Value:    0x8004B8C6
*/
#define MSPR_E_LICENSE_UNSUPPORTED_IN_SOFTWARE                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C6)

/*
*    MessageId:    MSPR_E_LICENSE_UNSUPPORTED_IN_CURRENT_FIRMWARE
*    Message Meaning:
*       The app received a HWDRM license which the current firmware does not support.
*    Hex Value:    0x8004B8C7
*/
#define MSPR_E_LICENSE_UNSUPPORTED_IN_CURRENT_FIRMWARE              MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C7)

/*
*    MessageId:    MSPR_E_LSRD_GET_SECURITY_INFO
*    Message Meaning:
*       Getting the security information of the LSRD Registry failed.
*    Hex Value:    0x8004B8C8
*/
#define MSPR_E_LSRD_GET_SECURITY_INFO                               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C8)

/*
*    MessageId:    MSPR_E_LSRD_CONVERT_SECURITYDESCRIPTOR_TO_STRING
*    Message Meaning:
*       Converting the security descriptor of the LSRD Registry to string failed.
*    Hex Value:    0x8004B8C9
*/
#define MSPR_E_LSRD_CONVERT_SECURITYDESCRIPTOR_TO_STRING            MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00C9)

/*
*    MessageId:    MSPR_E_LSRD_SECURITYDESCRIPTOR_SIZE
*    Message Meaning:
*       The size of the security descriptor of the LSRD Registry is incorrect.
*    Hex Value:    0x8004B8CA
*/
#define MSPR_E_LSRD_SECURITYDESCRIPTOR_SIZE                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00CA)

/*
*    MessageId:    MSPR_E_LSRD_LICENSE_DELETION_REQUIRED
*    Message Meaning:
*       LSRD is detected, and license deletion is required.
*    Hex Value:    0x8004B8CB
*/
#define MSPR_E_LSRD_LICENSE_DELETION_REQUIRED                       MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00CB)

/*
*    MessageId:    MSPR_E_PRO_HEADER_UNSPECIFIED_ENCRYPTION_TYPE
*    Message Meaning:
*        The PlayReady object header has an unspecified encryption type for a header version less than 4.3
*    Hex Value:    0x8004B8CC
*/
#define MSPR_E_PRO_HEADER_UNSPECIFIED_ENCRYPTION_TYPE               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00CC)

/*
*    MessageId:    MSPR_E_PRO_HEADER_INVALID_CBC_ENCRYPTION_TYPE
*    Message Meaning:
*        The PlayReady object header has a CBC encryption type for a header version less than 4.3
*    Hex Value:    0x8004B8CD
*/
#define MSPR_E_PRO_HEADER_INVALID_CBC_ENCRYPTION_TYPE               MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00CD)

/*
*    MessageId:    MSPR_E_HWDRM_NOT_SUPPORTED
*    Message Meaning:
*       HWDRM is present on the system but is not supported.
*    Hex Value:    0x8004B8CE
*/
#define MSPR_E_HWDRM_NOT_SUPPORTED                                  MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00CE)

/*
*    MessageId:    MSPR_E_HWDRM_SUPPORTED_BUT_NO_PATHS
*    Message Meaning:
*       Unable to initialize underlying DRM system without data store path.
*    Hex Value:    0x8004B8CF
*/
#define MSPR_E_HWDRM_SUPPORTED_BUT_NO_PATHS                         MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00CF)

/*
*    MessageId:    MSPR_E_AES_CBC_NOT_SUPPORTED
*    Message Meaning:
*       The content uses the AES CBC algorithm (CBCS or CBC1), but this is not supported.
*    Hex Value:    0x8004B8D0
*/
#define MSPR_E_AES_CBC_NOT_SUPPORTED                                MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00D0)

/*
*    MessageId:    MSPR_E_AES_STRIPING_NOT_SUPPORTED
*    Message Meaning:
*       The content includes AES striping (CBCS or CENS), but this is not supported.
*    Hex Value:    0x8004B8D1
*/
#define MSPR_E_AES_STRIPING_NOT_SUPPORTED                           MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00D1)

/*
*    MessageId:    MSPR_E_AES_SIXTEEN_BYTE_IVS_NOT_SUPPORTED
*    Message Meaning:
*       The content uses AES with sixteen byte IVs, but this is not supported.
*    Hex Value:    0x8004B8D2
*/
#define MSPR_E_AES_SIXTEEN_BYTE_IVS_NOT_SUPPORTED                   MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00D2)

/*
*    MessageId:    MSPR_E_SAMPLE_ENCRYPTION_TYPE_DOES_NOT_MATCH_LICENSE
*    Message Meaning:
*       The sample encryption type does not match the encryption type in the license.
*    Hex Value:    0x8004B8D3
*/
#define MSPR_E_SAMPLE_ENCRYPTION_TYPE_DOES_NOT_MATCH_LICENSE        MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00D3)

/*
*    MessageId:    MSPR_E_SET_SERVER_CERTIFICATE_REQUIRED
*    Message Meaning:
*       A call to the MediaKeys.setServerCertificate method in EME is required before this operation can be performed.
*    Hex Value:    0x8004B8D4
*/
#define MSPR_E_SET_SERVER_CERTIFICATE_REQUIRED                      MAKE_DRM_RESULT(DRM_SEVERITY_ERROR,  DRM_FACILITY_ITF, DRM_E_MSPRSDK_BASECODE+0x00D4)

#endif /* __WINDOWS_MEDIA_PROTECTION_PLAYREADY_ERRORS_H_ */

