/*++

  Microsoft Windows
  Copyright (C) Microsoft Corporation. All rights reserved.

Module Name:

    dlnaerror.mc

Abstract:

    Definitions for DLNA errors.

Author:


Revision History:

Notes:

    This file is used by the MC tool to generate the dlnaerror.h file

**************************** READ ME ******************************************

 Here are the commented error ranges for the Windows Media Technologies Group


 LEGACY RANGES

     0 -  1199 = NetShow errors (see nserror.mc)

  2000 - 13999 = Windows Media Player errors (see nserror.mc and asferr.mc)

 14000 - 42999 = Media Foundation errors (see mferror.mc)

 NEW RANGES

 50000 - 50099 = General DLNA errors

 50100 - 50199 = Digital Media Controller errors

 50200 - 50299 = Digital Media Renderer errors

 Max: 65535

**************************** READ ME ******************************************

--*/

#ifndef _DLNAERROR_H
#define _DLNAERROR_H


#define STATUS_SEVERITY(hr)  (((hr) >> 30) & 0x3)

#ifdef RC_INVOKED
#define _HRESULT_TYPEDEF_(_sc) _sc
#else // RC_INVOKED
#define _HRESULT_TYPEDEF_(_sc) ((HRESULT)_sc)
#endif // RC_INVOKED


/////////////////////////////////////////////////////////////////////////
//
// General DLNA Errors
//
/////////////////////////////////////////////////////////////////////////

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
#define FACILITY_NS                      0xD
#define FACILITY_NS_WIN32                0x7


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: DLNA_E_NOT_FOUND
//
// MessageText:
//
// The specified object or value does not exist.%0
//
#define DLNA_E_NOT_FOUND                 _HRESULT_TYPEDEF_(0xC00DC350L)

//
// MessageId: DLNA_E_NOT_AVAILABLE
//
// MessageText:
//
// The requested value is not available.%0
//
#define DLNA_E_NOT_AVAILABLE             _HRESULT_TYPEDEF_(0xC00DC351L)

//
// MessageId: DLNA_E_OPERATION_CANCELLED
//
// MessageText:
//
// The operation is cancelled.%0
//
#define DLNA_E_OPERATION_CANCELLED       _HRESULT_TYPEDEF_(0xC00DC352L)

//
// MessageId: DLNA_E_UNAUTHORIZED
//
// MessageText:
//
// This operation is not authorized.%0
//
#define DLNA_E_UNAUTHORIZED              _HRESULT_TYPEDEF_(0xC00DC353L)

//
// MessageId: DLNA_E_OUT_OF_RANGE
//
// MessageText:
//
// The value is not in the specified or valid range.%0
//
#define DLNA_E_OUT_OF_RANGE              _HRESULT_TYPEDEF_(0xC00DC354L)

//
// MessageId: DLNA_E_INVALID_DIDL_LITE_RESPONSE
//
// MessageText:
//
// The DIDL-Lite response provided by the server is not valid.
//
#define DLNA_E_INVALID_DIDL_LITE_RESPONSE _HRESULT_TYPEDEF_(0xC00DC355L)

//
// MessageId: DLNA_E_DIDL_VALUE_NOT_FOUND
//
// MessageText:
//
// A necessary element name was not found in the DIDL-Lite response provieded by the server.
//
#define DLNA_E_DIDL_VALUE_NOT_FOUND      _HRESULT_TYPEDEF_(0xC00DC356L)

//
// MessageId: DLNA_E_DIDL_VALUE_INVALID
//
// MessageText:
//
// A value in the DIDL-Lite response provided by the server was not in an accepted format.
//
#define DLNA_E_DIDL_VALUE_INVALID        _HRESULT_TYPEDEF_(0xC00DC357L)

//
// MessageId: DLNA_E_INVALID_DEVICE_RESPONSE
//
// MessageText:
//
// The device did not provide a valid response to the UPnP request.
//
#define DLNA_E_INVALID_DEVICE_RESPONSE   _HRESULT_TYPEDEF_(0xC00DC358L)

//
// MessageId: DLNA_E_INVALID_LAST_CHANGE_RESPONSE
//
// MessageText:
//
// The device did not advertise a valid LastChange response to the event subscriber.
//
#define DLNA_E_INVALID_LAST_CHANGE_RESPONSE _HRESULT_TYPEDEF_(0xC00DC359L)

//
// MessageId: DLNA_E_INVALID_PROTOCOL_INFO
//
// MessageText:
//
// The protocol info string returned from the server is invalid.
//
#define DLNA_E_INVALID_PROTOCOL_INFO     _HRESULT_TYPEDEF_(0xC00DC35AL)

//
// MessageId: DLNA_E_SERVICE_NOT_FOUND
//
// MessageText:
//
// The requested service was not found on the device.
//
#define DLNA_E_SERVICE_NOT_FOUND         _HRESULT_TYPEDEF_(0xC00DC35BL)

//
// MessageId: DLNA_E_NO_SERVICES_ON_DEVICE
//
// MessageText:
//
// No services were enumerated by the device.
//
#define DLNA_E_NO_SERVICES_ON_DEVICE     _HRESULT_TYPEDEF_(0xC00DC35CL)

//
// MessageId: DLNA_E_DEVICE_UNAVAILABLE
//
// MessageText:
//
// The device could not be contacted to fulfill the request.
//
#define DLNA_E_DEVICE_UNAVAILABLE        _HRESULT_TYPEDEF_(0xC00DC35DL)

//
// MessageId: DLNA_E_DEVICE_NOT_FOUND
//
// MessageText:
//
// The specified device was not found. The device might require pairing with the PC.%0
//
#define DLNA_E_DEVICE_NOT_FOUND          _HRESULT_TYPEDEF_(0xC00DC35EL)

//
// MessageId: DLNA_E_SERVER_NOT_FOUND
//
// MessageText:
//
// The specified Media Server was not found. The device might require pairing with the PC.%0
//
#define DLNA_E_SERVER_NOT_FOUND          _HRESULT_TYPEDEF_(0xC00DC35FL)

//
// MessageId: DLNA_E_RENDERER_NOT_FOUND
//
// MessageText:
//
// The specified Media Renderer was not found. The device might require pairing with the PC.%0
//
#define DLNA_E_RENDERER_NOT_FOUND        _HRESULT_TYPEDEF_(0xC00DC360L)


/////////////////////////////////////////////////////////////////////////
//
// Digital Media Controller Errors
//
/////////////////////////////////////////////////////////////////////////

//
// MessageId: DLNA_E_NO_COMPATIBLE_RES_ELEMENT_FOUND
//
// MessageText:
//
// The content is in a format that is not compatible with the Media Renderer device, and it is not possible to convert it to a compatible format.%0
//
#define DLNA_E_NO_COMPATIBLE_RES_ELEMENT_FOUND _HRESULT_TYPEDEF_(0xC00DC3B4L)

//
// MessageId: DLNA_E_UNSUPPORTED_FORMAT
//
// MessageText:
//
// The content cannot be sent to the Media Renderer because it is in a format that is not understood by Windows.%0
//
#define DLNA_E_UNSUPPORTED_FORMAT        _HRESULT_TYPEDEF_(0xC00DC3B5L)

//
// MessageId: DLNA_E_RENDERER_FAILED_TO_PLAY
//
// MessageText:
//
// The Media Renderer failed to play the content.%0
//
#define DLNA_E_RENDERER_FAILED_TO_PLAY   _HRESULT_TYPEDEF_(0xC00DC3B6L)

//
// MessageId: DLNA_E_DRM_PLAYBACK_FAILURE
//
// MessageText:
//
// The Media Renderer failed to play the content because it is protected by a Digital Rights Management system.%0
//
#define DLNA_E_DRM_PLAYBACK_FAILURE      _HRESULT_TYPEDEF_(0xC00DC3B7L)


/////////////////////////////////////////////////////////////////////////
//
// Digital Media Renderer Errors
//
/////////////////////////////////////////////////////////////////////////


#endif // _DLNAERROR_H

