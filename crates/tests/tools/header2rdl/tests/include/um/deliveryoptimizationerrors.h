//+-------------------------------------------------------------------------
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
//+-------------------------------------------------------------------------
#ifndef __DELIVERYOPTIMIZATION_ERROR_H__
#define __DELIVERYOPTIMIZATION_ERROR_H__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

// DeliveryOptimization error codes are HRESULTs using FACILITY_DELIVERY_OPTIMIZATION = 208 (0xD0).
//
// See winerror.h for more info on the layout of HRESULT values.

#define DO_ZONE_MASK 0xF000
#define DO_TRANSIENT_ZONE_MASK 0xF800
#define DO_TRANSIENT_ZONE 0x3800

#ifdef FACILITY_DELIVERY_OPTIMIZATION
FORCEINLINE bool IS_DO_TRANSIENT_ERROR(HRESULT hr)
{
    return (
        FAILED(hr) && (HRESULT_FACILITY(hr) == FACILITY_DELIVERY_OPTIMIZATION) &&
        ((HRESULT_CODE(hr) & DO_TRANSIENT_ZONE_MASK) == DO_TRANSIENT_ZONE));
}
#endif

#ifndef _HRESULT_TYPEDEF_
#ifdef RC_INVOKED
#define _HRESULT_TYPEDEF_(_sc) _sc
#else // RC_INVOKED
#define _HRESULT_TYPEDEF_(_sc) ((HRESULT)_sc)
#endif // RC_INVOKED
#endif // _HRESULT_TYPEDEF_

// clang-format off

#define DO_E_NO_SERVICE                             _HRESULT_TYPEDEF_(0x80D01001L) // Delivery Optimization was unable to provide the service

// Download job codes

#define DO_E_DOWNLOAD_NO_PROGRESS                   _HRESULT_TYPEDEF_(0x80D02002L) // Download of a file saw no progress within the defined period
#define DO_E_JOB_NOT_FOUND                          _HRESULT_TYPEDEF_(0x80D02003L) // Job was not found
#define DO_E_JOB_EMPTY                              _HRESULT_TYPEDEF_(0x80D02004L) // There were no files in the job
#define DO_E_NO_DOWNLOADS                           _HRESULT_TYPEDEF_(0x80D02005L) // No downloads currently exist
#define DO_E_MEMORYSTREAM_DOWNLOAD_NOT_SUPPORTED    _HRESULT_TYPEDEF_(0x80D0200BL) // Memory stream transfer is not supported for this download
#define DO_E_JOB_TOO_OLD                            _HRESULT_TYPEDEF_(0x80D0200CL) // Job has neither completed nor has it been cancelled prior to reaching the maximum age threshold
#define DO_E_LOCALPATH_NOT_SET                      _HRESULT_TYPEDEF_(0x80D0200DL) // There is no local file path specified for this download
#define DO_E_FILE_NOT_AVAILABLE                     _HRESULT_TYPEDEF_(0x80D02010L) // No file is available because no URL generated an error.
#define DO_E_UNKNOWN_PROPERTY_ID                    _HRESULT_TYPEDEF_(0x80D02011L) // SetProperty() or GetProperty() called with an unknown property ID
#define DO_E_READ_ONLY_PROPERTY                     _HRESULT_TYPEDEF_(0x80D02012L) // Unable to call SetProperty() on a read-only property
#define DO_E_INVALID_STATE                          _HRESULT_TYPEDEF_(0x80D02013L) // The requested action is not allowed in the current job state. The job might have been canceled or completed transferring. It is in a read-only state now.
#define DO_E_ERROR_INFORMATION_UNAVAILABLE          _HRESULT_TYPEDEF_(0x80D02014L) // No errors have occurred
#define DO_E_WRITE_ONLY_PROPERTY                    _HRESULT_TYPEDEF_(0x80D02015L) // Unable to call GetProperty() on a write-only property
#define DO_E_INTEGRITYCHECKINFO_UNSPECIFIED         _HRESULT_TYPEDEF_(0x80D02016L) // Download job is marked as requiring integrity checking but integrity checking info was not specified.
#define DO_E_INTEGRITYCHECKINFO_UNAVAILABLE         _HRESULT_TYPEDEF_(0x80D02017L) // Download job is marked as requiring integrity checking but integrity checking info could not be retrieved
#define DO_E_FILE_DOWNLOADSINK_UNSPECIFIED          _HRESULT_TYPEDEF_(0x80D02018L) // Unable to start a download because no download sink (either local file or stream interface) was specified
#define DO_E_FILE_DOWNLOADSINK_ALREADY_SET          _HRESULT_TYPEDEF_(0x80D02019L) // An attempt to set a download sink failed because another type of sink is already set
#define DO_E_FILE_SIZE_UNKNOWN_HTTP_200             _HRESULT_TYPEDEF_(0x80D0201AL) // Unable to determine file size from HTTP 200 status code
#define DO_E_FILE_ENCRYPTION_EXPECTED               _HRESULT_TYPEDEF_(0x80D0201BL) // Decryption key was provided but file on CDN does not appear to be encrypted
#define DO_E_FILE_SIZE_UNKNOWN_HTTP_206             _HRESULT_TYPEDEF_(0x80D0201CL) // Unable to determine file size from HTTP 206 status code
#define DO_E_FILE_SIZE_UNKNOWN_HTTP_2XX             _HRESULT_TYPEDEF_(0x80D0201DL) // Unable to determine file size from an unexpected HTTP 2xx status code
#define DO_E_NETWORK_ACCESS_CONSENT_NEEDED          _HRESULT_TYPEDEF_(0x80D0201EL) // User consent to access the network is required to proceed

// IDODownload interface

#define DO_E_DOWNLOAD_NO_URI                        _HRESULT_TYPEDEF_(0x80D02200L) // The download was started without providing a URI
#define DO_E_DOWNLOAD_NO_CONTENT_ID                 _HRESULT_TYPEDEF_(0x80D02201L) // The download was started without providing a content ID
#define DO_E_INVALID_CONTENT_ID                     _HRESULT_TYPEDEF_(0x80D02202L) // The specified content ID is invalid
#define DO_E_DOWNLOAD_RANGES_UNEXPECTED             _HRESULT_TYPEDEF_(0x80D02203L) // Ranges are unexpected for the current download
#define DO_E_DOWNLOAD_RANGES_EXPECTED               _HRESULT_TYPEDEF_(0x80D02204L) // Ranges are expected for the current download

#define DO_E_NO_DOWNLOAD_PARTICIPATION              _HRESULT_TYPEDEF_(0x80D03001L) // Download job not allowed due to participation throttling
#define DO_E_NO_DOWNLOAD_EXTSETTINGS                _HRESULT_TYPEDEF_(0x80D03002L) // Download job not allowed due to user/admin settings

// Transient conditions

#define DO_E_BLOCKED_BY_COST_TRANSFER_POLICY        _HRESULT_TYPEDEF_(0x80D03801L) // DO core paused the job due to cost policy restrictions
#define DO_E_DOWNLOADMODE_SET_BY_CP                 _HRESULT_TYPEDEF_(0x80D03802L) // DO job download mode restricted by content policy
#define DO_E_BLOCKED_BY_CELLULAR_POLICY             _HRESULT_TYPEDEF_(0x80D03803L) // DO core paused the job due to detection of cellular network and policy restrictions
#define DO_E_BLOCKED_BY_POWER_STATE                 _HRESULT_TYPEDEF_(0x80D03804L) // DO core paused the job due to detection of power state change into non-AC mode
#define DO_E_BLOCKED_BY_NO_NETWORK                  _HRESULT_TYPEDEF_(0x80D03805L) // DO core paused the job due to loss of network connectivity
#define DO_E_DOWNLOADMODE_SET_BY_POLICY             _HRESULT_TYPEDEF_(0x80D03806L) // DO job download mode restricted by policy
#define DO_E_BLOCKED_BY_VPN_POLICY                  _HRESULT_TYPEDEF_(0x80D03807L) // DO core paused the completed job due to detection of VPN network
#define DO_E_BLOCKED_BY_CRITICAL_MEMORY_USAGE       _HRESULT_TYPEDEF_(0x80D03808L) // DO core paused the completed job due to detection of critical memory usage on the system
#define DO_E_DOWNLOADMODE_SET_BY_CACHE_ABSENCE      _HRESULT_TYPEDEF_(0x80D03809L) // DO job download mode restricted due to absence of the cache folder
#define DO_E_CLOUD_SERVICES_UNREACHABLE             _HRESULT_TYPEDEF_(0x80D0380AL) // Unable to contact one or more DO cloud services
#define DO_E_DOWNLOADMODE_SET_FOR_UNREG_CALLER      _HRESULT_TYPEDEF_(0x80D0380BL) // DO job download mode restricted for unregistered caller
#define DO_E_DOWNLOADMODE_SET_FOR_SIMPLE_RANGES     _HRESULT_TYPEDEF_(0x80D0380CL) // DO job is using the simple ranges download in simple mode
#define DO_E_UNEXPECTED_HTTP_STATUS_2XX             _HRESULT_TYPEDEF_(0x80D0380DL) // DO job paused due to unexpected HTTP response codes (e.g. 204)

// HTTP

#define DO_E_HTTP_BLOCKSIZE_MISMATCH                _HRESULT_TYPEDEF_(0x80D05001L) // HTTP server returned a response with data size not equal to what was requested
#define DO_E_HTTP_CERT_VALIDATION                   _HRESULT_TYPEDEF_(0x80D05002L) // The Http server certificate validation has failed
#define DO_E_INVALID_RANGE                          _HRESULT_TYPEDEF_(0x80D05010L) // The specified byte range is invalid
#define DO_E_INSUFFICIENT_RANGE_SUPPORT             _HRESULT_TYPEDEF_(0x80D05011L) // The server does not support the necessary HTTP protocol. Delivery Optimization (DO) requires that the server support the Range protocol header.
#define DO_E_OVERLAPPING_RANGES                     _HRESULT_TYPEDEF_(0x80D05012L) // The list of byte ranges contains some overlapping ranges, which are not supported

// General core codes

#define DO_E_UPLOADING_BAD_PIECES                   _HRESULT_TYPEDEF_(0x80D06800L) // Too many bad pieces found during upload
#define DO_E_FATAL_CORE_ERROR                       _HRESULT_TYPEDEF_(0x80D06802L) // Fatal error encountered in core
#define DO_E_SERVICES_RESPONSE_EMPTY                _HRESULT_TYPEDEF_(0x80D06803L) // Services response was an empty JSON content
#define DO_E_BAD_INCOMING_DATA                      _HRESULT_TYPEDEF_(0x80D06804L) // Received bad or incomplete data for a content piece
#define DO_E_BAD_PIECE_HASH                         _HRESULT_TYPEDEF_(0x80D06805L) // Content piece hash check failed
#define DO_E_BAD_PIECE_HASH_NO_BAN                  _HRESULT_TYPEDEF_(0x80D06806L) // Content piece hash check failed but source is not banned yet
#define DO_E_ALREADY_HAVE_PIECE                     _HRESULT_TYPEDEF_(0x80D06807L) // The piece was rejected because it already exists in the cache
#define DO_E_MISSING_PIECE                          _HRESULT_TYPEDEF_(0x80D06808L) // The piece requested is no longer available in the cache
#define DO_E_METAINFO_CONTENT                       _HRESULT_TYPEDEF_(0x80D06809L) // Failed to parse JSON from input buffer
#define DO_E_METAINFO_VERSION                       _HRESULT_TYPEDEF_(0x80D0680AL) // Invalid metainfo version
#define DO_E_SWARM_NOT_RUNNING                      _HRESULT_TYPEDEF_(0x80D0680BL) // The swarm isn't running
#define DO_E_UNRECOGNIZED_CONN                      _HRESULT_TYPEDEF_(0x80D0680CL) // The peer was not recognized by the connection manager
#define DO_E_PEER_BANNED                            _HRESULT_TYPEDEF_(0x80D0680DL) // The peer is banned
#define DO_E_CONNECT_TO_SELF                        _HRESULT_TYPEDEF_(0x80D0680EL) // The client is trying to connect to itself
#define DO_E_ALREADY_CONNECTED                      _HRESULT_TYPEDEF_(0x80D0680FL) // The socket or peer is already connected
#define DO_E_NO_MORE_CONNECTIONS                    _HRESULT_TYPEDEF_(0x80D06810L) // The maximum number of connections has been reached
#define DO_E_LOST_CONNECTION                        _HRESULT_TYPEDEF_(0x80D06811L) // The connection was lost
#define DO_E_UNRECOGNIZED_SWARM                     _HRESULT_TYPEDEF_(0x80D06812L) // The swarm ID is not recognized
#define DO_E_INVALID_HANDSHAKE_LEN                  _HRESULT_TYPEDEF_(0x80D06813L) // The handshake length is invalid
#define DO_E_SOCKET_CLOSED                          _HRESULT_TYPEDEF_(0x80D06814L) // The socket has been closed
#define DO_E_PEER_MSG_TOO_LONG                      _HRESULT_TYPEDEF_(0x80D06815L) // The message is too long
#define DO_E_INVALID_PEER_MSG                       _HRESULT_TYPEDEF_(0x80D06816L) // The message is invalid
#define DO_E_PEER_IS_UPLOAD                         _HRESULT_TYPEDEF_(0x80D06817L) // The peer is an upload
#define DO_E_PIN_FAIL_NO_PEERING                    _HRESULT_TYPEDEF_(0x80D06818L) // Cannot pin a swarm because it's not in peering mode
#define DO_E_DELETE_PINNED_SWARM                    _HRESULT_TYPEDEF_(0x80D06819L) // Cannot delete a pinned swarm without using the "force" flag

#define DO_E_METAINFO_DIGEST_MATCH                  _HRESULT_TYPEDEF_(0x80D0681AL) // Hash digest of the input buffer did not match
#define DO_E_METAINFO_HOH_MATCH                     _HRESULT_TYPEDEF_(0x80D0681BL) // Hash-of-hashes preliminary match failed
#define DO_E_METAINFO_HOH_FINAL_MATCH               _HRESULT_TYPEDEF_(0x80D0681CL) // Computed hash-of-hashes match failed
#define DO_E_METAINFO_CONTENT_LENGTH                _HRESULT_TYPEDEF_(0x80D0681DL) // Content length is invalid
#define DO_E_METAINFO_PIECE_SIZE                    _HRESULT_TYPEDEF_(0x80D0681EL) // Piece size is invalid
#define DO_E_METAINFO_PARSE_HOH                     _HRESULT_TYPEDEF_(0x80D0681FL) // Failed to parse hash-of-hashes value
#define DO_E_METAINFO_PARSE_PIECEHASHES             _HRESULT_TYPEDEF_(0x80D06820L) // Failed to parse array of piece hashes
#define DO_E_METAINFO_PARSE_ONE_PIECEHASH           _HRESULT_TYPEDEF_(0x80D06821L) // Failed to parse an individual piece hash digest
#define DO_E_METAINFO_FILE_SIZE                     _HRESULT_TYPEDEF_(0x80D06822L) // Failed to get PHF file's size or size is bad

// clang-format on

#endif // __DELIVERYOPTIMIZATION_ERROR_H__
