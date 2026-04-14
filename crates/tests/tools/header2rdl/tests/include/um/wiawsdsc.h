/**************************************************************************
*
*  Copyright (c) Microsoft Corporation
*
*  File: wiawsdsc.h
*
*  Version: 2.0
*
*  Description: contains custom WIA definitions for the WSD scan class driver
*
***************************************************************************/

#ifndef _WIAWSDSC_
#define _WIAWSDSC_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef _WIADEF_
#include <wiadef.h>
#endif

//
// Custom WIA property IDs (see wiadef.h)
//
// These custom properties describe PnP-X device properties
// read at run time from Function Discovery, along with:
//
// WIA_DPS_SERVICE_ID
// WIA_DPS_DEVICE_ID
// WIA_DPS_GLOBAL_IDENTITY
// WIA_DPS_FIRMWARE_VERSION
//
// All are read-only Root item properties maintained by the driver.
//
// Property Type: VT_BSTR
// Valid Values:  WIA_PROP_NONE
// Access Rights: READONLY
//

#define WIA_WSD_MANUFACTURER             WIA_PRIVATE_DEVPROP
#define WIA_WSD_MANUFACTURER_STR         L"Device manufacturer"

#define WIA_WSD_MANUFACTURER_URL         (WIA_PRIVATE_DEVPROP + 1)
#define WIA_WSD_MANUFACTURER_URL_STR     L"Manufacurer URL"

#define WIA_WSD_MODEL_NAME               (WIA_PRIVATE_DEVPROP + 2)
#define WIA_WSD_MODEL_NAME_STR           L"Model name"

#define WIA_WSD_MODEL_NUMBER             (WIA_PRIVATE_DEVPROP + 3)
#define WIA_WSD_MODEL_NUMBER_STR         L"Model number"

#define WIA_WSD_MODEL_URL                (WIA_PRIVATE_DEVPROP + 4)
#define WIA_WSD_MODEL_URL_STR            L"Model URL"

#define WIA_WSD_PRESENTATION_URL         (WIA_PRIVATE_DEVPROP + 5)
#define WIA_WSD_PRESENTATION_URL_STR     L"Presentation URL"

#define WIA_WSD_FRIENDLY_NAME            (WIA_PRIVATE_DEVPROP + 6)
#define WIA_WSD_FRIENDLY_NAME_STR        L"Friendly name"

#define WIA_WSD_SERIAL_NUMBER            (WIA_PRIVATE_DEVPROP + 7)
#define WIA_WSD_SERIAL_NUMBER_STR        L"Serial number"

//
// Obsolete custom WIA property for automatic input-source selection
// during programmed push (device initiated) scanning, currently
// replaced by the standard WIA_DPS_SCAN_AVAILABLE_ITEM (defined
// in wiadef.h) and kept only for backwards compatibility.
// Use WIA_DPS_SCAN_AVAILABLE_ITEM in all new code:
//
#define WIA_WSD_SCAN_AVAILABLE_ITEM      (WIA_PRIVATE_DEVPROP + 8)
#define WIA_WSD_SCAN_AVAILABLE_ITEM_STR  WIA_DPS_SCAN_AVAILABLE_ITEM_STR


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_WIAWSDSC_
