/****************************************************************************
*
*  Copyright (c) Microsoft Corporation. All rights reserved.
*
*  File: wiadef.h
*
*  Version: 4.0
*
*  Description: WIA constant definitions
*
*****************************************************************************/

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later

#pragma once

#ifndef _WIADEF_
#define _WIADEF_

#include <winapifamily.h>

#include <pshpack8.h>
#ifndef _NO_COM
#include <objbase.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// WIA property ID and string constants
//

#define WIA_DIP_DEV_ID                                       2 // 0x2
#define WIA_DIP_DEV_ID_STR                                   L"Unique Device ID"

#define WIA_DIP_VEND_DESC                                    3 // 0x3
#define WIA_DIP_VEND_DESC_STR                                L"Manufacturer"

#define WIA_DIP_DEV_DESC                                     4 // 0x4
#define WIA_DIP_DEV_DESC_STR                                 L"Description"

#define WIA_DIP_DEV_TYPE                                     5 // 0x5
#define WIA_DIP_DEV_TYPE_STR                                 L"Type"

#define WIA_DIP_PORT_NAME                                    6 // 0x6
#define WIA_DIP_PORT_NAME_STR                                L"Port"

#define WIA_DIP_DEV_NAME                                     7 // 0x7
#define WIA_DIP_DEV_NAME_STR                                 L"Name"

#define WIA_DIP_SERVER_NAME                                  8 // 0x8
#define WIA_DIP_SERVER_NAME_STR                              L"Server"

#define WIA_DIP_REMOTE_DEV_ID                                9 // 0x9
#define WIA_DIP_REMOTE_DEV_ID_STR                            L"Remote Device ID"

#define WIA_DIP_UI_CLSID                                     10 // 0xa
#define WIA_DIP_UI_CLSID_STR                                 L"UI Class ID"

#define WIA_DIP_HW_CONFIG                                    11 // 0xb
#define WIA_DIP_HW_CONFIG_STR                                L"Hardware Configuration"

#define WIA_DIP_BAUDRATE                                     12 // 0xc
#define WIA_DIP_BAUDRATE_STR                                 L"BaudRate"

#define WIA_DIP_STI_GEN_CAPABILITIES                         13 // 0xd
#define WIA_DIP_STI_GEN_CAPABILITIES_STR                     L"STI Generic Capabilities"

#define WIA_DIP_WIA_VERSION                                  14 // 0xe
#define WIA_DIP_WIA_VERSION_STR                              L"WIA Version"

#define WIA_DIP_DRIVER_VERSION                               15 // 0xf
#define WIA_DIP_DRIVER_VERSION_STR                           L"Driver Version"

#define WIA_DIP_PNP_ID                                       16 // 0x10
#define WIA_DIP_PNP_ID_STR                                   L"PnP ID String"

#define WIA_DIP_STI_DRIVER_VERSION                           17 // 0x11
#define WIA_DIP_STI_DRIVER_VERSION_STR                       L"STI Driver Version"

#define WIA_DPA_FIRMWARE_VERSION                             1026 // 0x402
#define WIA_DPA_FIRMWARE_VERSION_STR                         L"Firmware Version"

#define WIA_DPA_CONNECT_STATUS                               1027 // 0x403
#define WIA_DPA_CONNECT_STATUS_STR                           L"Connect Status"

#define WIA_DPA_DEVICE_TIME                                  1028 // 0x404
#define WIA_DPA_DEVICE_TIME_STR                              L"Device Time"

#define WIA_DPC_PICTURES_TAKEN                               2050 // 0x802
#define WIA_DPC_PICTURES_TAKEN_STR                           L"Pictures Taken"

#define WIA_DPC_PICTURES_REMAINING                           2051 // 0x803
#define WIA_DPC_PICTURES_REMAINING_STR                       L"Pictures Remaining"

#define WIA_DPC_EXPOSURE_MODE                                2052 // 0x804
#define WIA_DPC_EXPOSURE_MODE_STR                            L"Exposure Mode"

#define WIA_DPC_EXPOSURE_COMP                                2053 // 0x805
#define WIA_DPC_EXPOSURE_COMP_STR                            L"Exposure Compensation"

#define WIA_DPC_EXPOSURE_TIME                                2054 // 0x806
#define WIA_DPC_EXPOSURE_TIME_STR                            L"Exposure Time"

#define WIA_DPC_FNUMBER                                      2055 // 0x807
#define WIA_DPC_FNUMBER_STR                                  L"F Number"

#define WIA_DPC_FLASH_MODE                                   2056 // 0x808
#define WIA_DPC_FLASH_MODE_STR                               L"Flash Mode"

#define WIA_DPC_FOCUS_MODE                                   2057 // 0x809
#define WIA_DPC_FOCUS_MODE_STR                               L"Focus Mode"

#define WIA_DPC_FOCUS_MANUAL_DIST                            2058 // 0x80a
#define WIA_DPC_FOCUS_MANUAL_DIST_STR                        L"Focus Manual Dist"

#define WIA_DPC_ZOOM_POSITION                                2059 // 0x80b
#define WIA_DPC_ZOOM_POSITION_STR                            L"Zoom Position"

#define WIA_DPC_PAN_POSITION                                 2060 // 0x80c
#define WIA_DPC_PAN_POSITION_STR                             L"Pan Position"

#define WIA_DPC_TILT_POSITION                                2061 // 0x80d
#define WIA_DPC_TILT_POSITION_STR                            L"Tilt Position"

#define WIA_DPC_TIMER_MODE                                   2062 // 0x80e
#define WIA_DPC_TIMER_MODE_STR                               L"Timer Mode"

#define WIA_DPC_TIMER_VALUE                                  2063 // 0x80f
#define WIA_DPC_TIMER_VALUE_STR                              L"Timer Value"

#define WIA_DPC_POWER_MODE                                   2064 // 0x810
#define WIA_DPC_POWER_MODE_STR                               L"Power Mode"

#define WIA_DPC_BATTERY_STATUS                               2065 // 0x811
#define WIA_DPC_BATTERY_STATUS_STR                           L"Battery Status"

#define WIA_DPC_THUMB_WIDTH                                  2066 // 0x812
#define WIA_DPC_THUMB_WIDTH_STR                              L"Thumbnail Width"

#define WIA_DPC_THUMB_HEIGHT                                 2067 // 0x813
#define WIA_DPC_THUMB_HEIGHT_STR                             L"Thumbnail Height"

#define WIA_DPC_PICT_WIDTH                                   2068 // 0x814
#define WIA_DPC_PICT_WIDTH_STR                               L"Picture Width"

#define WIA_DPC_PICT_HEIGHT                                  2069 // 0x815
#define WIA_DPC_PICT_HEIGHT_STR                              L"Picture Height"

#define WIA_DPC_DIMENSION                                    2070 // 0x816
#define WIA_DPC_DIMENSION_STR                                L"Dimension"

#define WIA_DPC_COMPRESSION_SETTING                          2071 // 0x817
#define WIA_DPC_COMPRESSION_SETTING_STR                      L"Compression Setting"

#define WIA_DPC_FOCUS_METERING                               2072 // 0x818
#define WIA_DPC_FOCUS_METERING_STR                           L"Focus Metering Mode"

#define WIA_DPC_TIMELAPSE_INTERVAL                           2073 // 0x819
#define WIA_DPC_TIMELAPSE_INTERVAL_STR                       L"Timelapse Interval"

#define WIA_DPC_TIMELAPSE_NUMBER                             2074 // 0x81a
#define WIA_DPC_TIMELAPSE_NUMBER_STR                         L"Timelapse Number"

#define WIA_DPC_BURST_INTERVAL                               2075 // 0x81b
#define WIA_DPC_BURST_INTERVAL_STR                           L"Burst Interval"

#define WIA_DPC_BURST_NUMBER                                 2076 // 0x81c
#define WIA_DPC_BURST_NUMBER_STR                             L"Burst Number"

#define WIA_DPC_EFFECT_MODE                                  2077 // 0x81d
#define WIA_DPC_EFFECT_MODE_STR                              L"Effect Mode"

#define WIA_DPC_DIGITAL_ZOOM                                 2078 // 0x81e
#define WIA_DPC_DIGITAL_ZOOM_STR                             L"Digital Zoom"

#define WIA_DPC_SHARPNESS                                    2079 // 0x81f
#define WIA_DPC_SHARPNESS_STR                                L"Sharpness"

#define WIA_DPC_CONTRAST                                     2080 // 0x820
#define WIA_DPC_CONTRAST_STR                                 L"Contrast"

#define WIA_DPC_CAPTURE_MODE                                 2081 // 0x821
#define WIA_DPC_CAPTURE_MODE_STR                             L"Capture Mode"

#define WIA_DPC_CAPTURE_DELAY                                2082 // 0x822
#define WIA_DPC_CAPTURE_DELAY_STR                            L"Capture Delay"

#define WIA_DPC_EXPOSURE_INDEX                               2083 // 0x823
#define WIA_DPC_EXPOSURE_INDEX_STR                           L"Exposure Index"

#define WIA_DPC_EXPOSURE_METERING_MODE                       2084 // 0x824
#define WIA_DPC_EXPOSURE_METERING_MODE_STR                   L"Exposure Metering Mode"

#define WIA_DPC_FOCUS_METERING_MODE                          2085 // 0x825
#define WIA_DPC_FOCUS_METERING_MODE_STR                      L"Focus Metering Mode"

#define WIA_DPC_FOCUS_DISTANCE                               2086 // 0x826
#define WIA_DPC_FOCUS_DISTANCE_STR                           L"Focus Distance"

#define WIA_DPC_FOCAL_LENGTH                                 2087 // 0x827
#define WIA_DPC_FOCAL_LENGTH_STR                             L"Focus Length"

#define WIA_DPC_RGB_GAIN                                     2088 // 0x828
#define WIA_DPC_RGB_GAIN_STR                                 L"RGB Gain"

#define WIA_DPC_WHITE_BALANCE                                2089 // 0x829
#define WIA_DPC_WHITE_BALANCE_STR                            L"White Balance"

#define WIA_DPC_UPLOAD_URL                                   2090 // 0x82a
#define WIA_DPC_UPLOAD_URL_STR                               L"Upload URL"

#define WIA_DPC_ARTIST                                       2091 // 0x82b
#define WIA_DPC_ARTIST_STR                                   L"Artist"

#define WIA_DPC_COPYRIGHT_INFO                               2092 // 0x82c
#define WIA_DPC_COPYRIGHT_INFO_STR                           L"Copyright Info"

#define WIA_DPS_HORIZONTAL_BED_SIZE                          3074 // 0xc02
#define WIA_DPS_HORIZONTAL_BED_SIZE_STR                      L"Horizontal Bed Size"

#define WIA_DPS_VERTICAL_BED_SIZE                            3075 // 0xc03
#define WIA_DPS_VERTICAL_BED_SIZE_STR                        L"Vertical Bed Size"

#define WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE                   3076 // 0xc04
#define WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE_STR               L"Horizontal Sheet Feed Size"

#define WIA_DPS_VERTICAL_SHEET_FEED_SIZE                     3077 // 0xc05
#define WIA_DPS_VERTICAL_SHEET_FEED_SIZE_STR                 L"Vertical Sheet Feed Size"

#define WIA_DPS_SHEET_FEEDER_REGISTRATION                    3078 // 0xc06
#define WIA_DPS_SHEET_FEEDER_REGISTRATION_STR                L"Sheet Feeder Registration"

#define WIA_DPS_HORIZONTAL_BED_REGISTRATION                  3079 // 0xc07
#define WIA_DPS_HORIZONTAL_BED_REGISTRATION_STR              L"Horizontal Bed Registration"

#define WIA_DPS_VERTICAL_BED_REGISTRATION                    3080 // 0xc08
#define WIA_DPS_VERTICAL_BED_REGISTRATION_STR                L"Vertical Bed Registration"

#define WIA_DPS_PLATEN_COLOR                                 3081 // 0xc09
#define WIA_DPS_PLATEN_COLOR_STR                             L"Platen Color"

#define WIA_DPS_PAD_COLOR                                    3082 // 0xc0a
#define WIA_DPS_PAD_COLOR_STR                                L"Pad Color"

#define WIA_DPS_FILTER_SELECT                                3083 // 0xc0b
#define WIA_DPS_FILTER_SELECT_STR                            L"Filter Select"

#define WIA_DPS_DITHER_SELECT                                3084 // 0xc0c
#define WIA_DPS_DITHER_SELECT_STR                            L"Dither Select"

#define WIA_DPS_DITHER_PATTERN_DATA                          3085 // 0xc0d
#define WIA_DPS_DITHER_PATTERN_DATA_STR                      L"Dither Pattern Data"

#define WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES               3086 // 0xc0e
#define WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES_STR           L"Document Handling Capabilities"

#define WIA_DPS_DOCUMENT_HANDLING_STATUS                     3087 // 0xc0f
#define WIA_DPS_DOCUMENT_HANDLING_STATUS_STR                 L"Document Handling Status"

#define WIA_DPS_DOCUMENT_HANDLING_SELECT                     3088 // 0xc10
#define WIA_DPS_DOCUMENT_HANDLING_SELECT_STR                 L"Document Handling Select"

#define WIA_DPS_DOCUMENT_HANDLING_CAPACITY                   3089 // 0xc11
#define WIA_DPS_DOCUMENT_HANDLING_CAPACITY_STR               L"Document Handling Capacity"

#define WIA_DPS_OPTICAL_XRES                                 3090 // 0xc12
#define WIA_DPS_OPTICAL_XRES_STR                             L"Horizontal Optical Resolution"

#define WIA_DPS_OPTICAL_YRES                                 3091 // 0xc13
#define WIA_DPS_OPTICAL_YRES_STR                             L"Vertical Optical Resolution"

#define WIA_DPS_ENDORSER_CHARACTERS                          3092 // 0xc14, superseded by WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS
#define WIA_DPS_ENDORSER_CHARACTERS_STR                      L"Endorser Characters"

#define WIA_DPS_ENDORSER_STRING                              3093 // 0xc15, superseded by WIA_IPS_PRINTER_ENDORSER_STRING
#define WIA_DPS_ENDORSER_STRING_STR                          L"Endorser String"

#define WIA_DPS_SCAN_AHEAD_PAGES                             3094 // 0xc16, superseded by WIA_IPS_SCAN_AHEAD
#define WIA_DPS_SCAN_AHEAD_PAGES_STR                         L"Scan Ahead Pages"

#define WIA_DPS_MAX_SCAN_TIME                                3095 // 0xc17
#define WIA_DPS_MAX_SCAN_TIME_STR                            L"Max Scan Time"

#define WIA_DPS_PAGES                                        3096 // 0xc18
#define WIA_DPS_PAGES_STR                                    L"Pages"

#define WIA_DPS_PAGE_SIZE                                    3097 // 0xc19
#define WIA_DPS_PAGE_SIZE_STR                                L"Page Size"

#define WIA_DPS_PAGE_WIDTH                                   3098 // 0xc1a
#define WIA_DPS_PAGE_WIDTH_STR                               L"Page Width"

#define WIA_DPS_PAGE_HEIGHT                                  3099 // 0xc1b
#define WIA_DPS_PAGE_HEIGHT_STR                              L"Page Height"

#define WIA_DPS_PREVIEW                                      3100 // 0xc1c
#define WIA_DPS_PREVIEW_STR                                  L"Preview"

#define WIA_DPS_TRANSPARENCY                                 3101 // 0xc1d
#define WIA_DPS_TRANSPARENCY_STR                             L"Transparency Adapter"

#define WIA_DPS_TRANSPARENCY_SELECT                          3102 // 0xc1e
#define WIA_DPS_TRANSPARENCY_SELECT_STR                      L"Transparency Adapter Select"

#define WIA_DPS_SHOW_PREVIEW_CONTROL                         3103 // 0xc1f
#define WIA_DPS_SHOW_PREVIEW_CONTROL_STR                     L"Show preview control"

#define WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE               3104 // 0xc20
#define WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE_STR           L"Minimum Horizontal Sheet Feed Size"

#define WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE                 3105 // 0xc21
#define WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE_STR             L"Minimum Vertical Sheet Feed Size"

#define WIA_DPS_TRANSPARENCY_CAPABILITIES                    3106 // 0xc22
#define WIA_DPS_TRANSPARENCY_CAPABILITIES_STR                L"Transparency Adapter Capabilities"

#define WIA_DPS_TRANSPARENCY_STATUS                          3107 // 0xc23
#define WIA_DPS_TRANSPARENCY_STATUS_STR                      L"Transparency Adapter Status"

#define WIA_DPF_MOUNT_POINT                                  3330 // 0xd02
#define WIA_DPF_MOUNT_POINT_STR                              L"Directory mount point"

#define WIA_DPV_LAST_PICTURE_TAKEN                           3586 // 0xe02
#define WIA_DPV_LAST_PICTURE_TAKEN_STR                       L"Last Picture Taken"

#define WIA_DPV_IMAGES_DIRECTORY                             3587 // 0xe03
#define WIA_DPV_IMAGES_DIRECTORY_STR                         L"Images Directory"

#define WIA_DPV_DSHOW_DEVICE_PATH                            3588 // 0xe04
#define WIA_DPV_DSHOW_DEVICE_PATH_STR                        L"Directshow Device Path"

#define WIA_IPA_ITEM_NAME                                    4098 // 0x1002
#define WIA_IPA_ITEM_NAME_STR                                L"Item Name"

#define WIA_IPA_FULL_ITEM_NAME                               4099 // 0x1003
#define WIA_IPA_FULL_ITEM_NAME_STR                           L"Full Item Name"

#define WIA_IPA_ITEM_TIME                                    4100 // 0x1004
#define WIA_IPA_ITEM_TIME_STR                                L"Item Time Stamp"

#define WIA_IPA_ITEM_FLAGS                                   4101 // 0x1005
#define WIA_IPA_ITEM_FLAGS_STR                               L"Item Flags"

#define WIA_IPA_ACCESS_RIGHTS                                4102 // 0x1006
#define WIA_IPA_ACCESS_RIGHTS_STR                            L"Access Rights"

#define WIA_IPA_DATATYPE                                     4103 // 0x1007
#define WIA_IPA_DATATYPE_STR                                 L"Data Type"

#define WIA_IPA_DEPTH                                        4104 // 0x1008
#define WIA_IPA_DEPTH_STR                                    L"Bits Per Pixel"

#define WIA_IPA_PREFERRED_FORMAT                             4105 // 0x1009
#define WIA_IPA_PREFERRED_FORMAT_STR                         L"Preferred Format"

#define WIA_IPA_FORMAT                                       4106 // 0x100a
#define WIA_IPA_FORMAT_STR                                   L"Format"

#define WIA_IPA_COMPRESSION                                  4107 // 0x100b
#define WIA_IPA_COMPRESSION_STR                              L"Compression"

#define WIA_IPA_TYMED                                        4108 // 0x100c
#define WIA_IPA_TYMED_STR                                    L"Media Type"

#define WIA_IPA_CHANNELS_PER_PIXEL                           4109 // 0x100d
#define WIA_IPA_CHANNELS_PER_PIXEL_STR                       L"Channels Per Pixel"

#define WIA_IPA_BITS_PER_CHANNEL                             4110 // 0x100e
#define WIA_IPA_BITS_PER_CHANNEL_STR                         L"Bits Per Channel"

#define WIA_IPA_PLANAR                                       4111 // 0x100f
#define WIA_IPA_PLANAR_STR                                   L"Planar"

#define WIA_IPA_PIXELS_PER_LINE                              4112 // 0x1010
#define WIA_IPA_PIXELS_PER_LINE_STR                          L"Pixels Per Line"

#define WIA_IPA_BYTES_PER_LINE                               4113 // 0x1011
#define WIA_IPA_BYTES_PER_LINE_STR                           L"Bytes Per Line"

#define WIA_IPA_NUMBER_OF_LINES                              4114 // 0x1012
#define WIA_IPA_NUMBER_OF_LINES_STR                          L"Number of Lines"

#define WIA_IPA_GAMMA_CURVES                                 4115 // 0x1013
#define WIA_IPA_GAMMA_CURVES_STR                             L"Gamma Curves"

#define WIA_IPA_ITEM_SIZE                                    4116 // 0x1014
#define WIA_IPA_ITEM_SIZE_STR                                L"Item Size"

#define WIA_IPA_COLOR_PROFILE                                4117 // 0x1015
#define WIA_IPA_COLOR_PROFILE_STR                            L"Color Profiles"

#define WIA_IPA_MIN_BUFFER_SIZE                              4118 // 0x1016
#define WIA_IPA_MIN_BUFFER_SIZE_STR                          L"Buffer Size"

#define WIA_IPA_BUFFER_SIZE                                  4118 // 0x1016
#define WIA_IPA_BUFFER_SIZE_STR                              L"Buffer Size"

#define WIA_IPA_REGION_TYPE                                  4119 // 0x1017
#define WIA_IPA_REGION_TYPE_STR                              L"Region Type"

#define WIA_IPA_ICM_PROFILE_NAME                             4120 // 0x1018
#define WIA_IPA_ICM_PROFILE_NAME_STR                         L"Color Profile Name"

#define WIA_IPA_APP_COLOR_MAPPING                            4121 // 0x1019
#define WIA_IPA_APP_COLOR_MAPPING_STR                        L"Application Applies Color Mapping"

#define WIA_IPA_PROP_STREAM_COMPAT_ID                        4122 // 0x101a
#define WIA_IPA_PROP_STREAM_COMPAT_ID_STR                    L"Stream Compatibility ID"

#define WIA_IPA_FILENAME_EXTENSION                           4123 // 0x101b
#define WIA_IPA_FILENAME_EXTENSION_STR                       L"Filename extension"

#define WIA_IPA_SUPPRESS_PROPERTY_PAGE                       4124 // 0x101c
#define WIA_IPA_SUPPRESS_PROPERTY_PAGE_STR                   L"Suppress a property page"

#define WIA_IPC_THUMBNAIL                                    5122 // 0x1402
#define WIA_IPC_THUMBNAIL_STR                                L"Thumbnail Data"

#define WIA_IPC_THUMB_WIDTH                                  5123 // 0x1403
#define WIA_IPC_THUMB_WIDTH_STR                              L"Thumbnail Width"

#define WIA_IPC_THUMB_HEIGHT                                 5124 // 0x1404
#define WIA_IPC_THUMB_HEIGHT_STR                             L"Thumbnail Height"

#define WIA_IPC_AUDIO_AVAILABLE                              5125 // 0x1405
#define WIA_IPC_AUDIO_AVAILABLE_STR                          L"Audio Available"

#define WIA_IPC_AUDIO_DATA_FORMAT                            5126 // 0x1406
#define WIA_IPC_AUDIO_DATA_FORMAT_STR                        L"Audio Format"

#define WIA_IPC_AUDIO_DATA                                   5127 // 0x1407
#define WIA_IPC_AUDIO_DATA_STR                               L"Audio Data"

#define WIA_IPC_NUM_PICT_PER_ROW                             5128 // 0x1408
#define WIA_IPC_NUM_PICT_PER_ROW_STR                         L"Pictures per Row"

#define WIA_IPC_SEQUENCE                                     5129 // 0x1409
#define WIA_IPC_SEQUENCE_STR                                 L"Sequence Number"

#define WIA_IPC_TIMEDELAY                                    5130 // 0x140a
#define WIA_IPC_TIMEDELAY_STR                                L"Time Delay"

#define WIA_IPS_CUR_INTENT                                   6146 // 0x1802
#define WIA_IPS_CUR_INTENT_STR                               L"Current Intent"

#define WIA_IPS_XRES                                         6147 // 0x1803
#define WIA_IPS_XRES_STR                                     L"Horizontal Resolution"

#define WIA_IPS_YRES                                         6148 // 0x1804
#define WIA_IPS_YRES_STR                                     L"Vertical Resolution"

#define WIA_IPS_XPOS                                         6149 // 0x1805
#define WIA_IPS_XPOS_STR                                     L"Horizontal Start Position"

#define WIA_IPS_YPOS                                         6150 // 0x1806
#define WIA_IPS_YPOS_STR                                     L"Vertical Start Position"

#define WIA_IPS_XEXTENT                                      6151 // 0x1807
#define WIA_IPS_XEXTENT_STR                                  L"Horizontal Extent"

#define WIA_IPS_YEXTENT                                      6152 // 0x1808
#define WIA_IPS_YEXTENT_STR                                  L"Vertical Extent"

#define WIA_IPS_PHOTOMETRIC_INTERP                           6153 // 0x1809
#define WIA_IPS_PHOTOMETRIC_INTERP_STR                       L"Photometric Interpretation"

#define WIA_IPS_BRIGHTNESS                                   6154 // 0x180a
#define WIA_IPS_BRIGHTNESS_STR                               L"Brightness"

#define WIA_IPS_CONTRAST                                     6155 // 0x180b
#define WIA_IPS_CONTRAST_STR                                 L"Contrast"

#define WIA_IPS_ORIENTATION                                  6156 // 0x180c
#define WIA_IPS_ORIENTATION_STR                              L"Orientation"

#define WIA_IPS_ROTATION                                     6157 // 0x180d
#define WIA_IPS_ROTATION_STR                                 L"Rotation"

#define WIA_IPS_MIRROR                                       6158 // 0x180e
#define WIA_IPS_MIRROR_STR                                   L"Mirror"

#define WIA_IPS_THRESHOLD                                    6159 // 0x180f
#define WIA_IPS_THRESHOLD_STR                                L"Threshold"

#define WIA_IPS_INVERT                                       6160 // 0x1810
#define WIA_IPS_INVERT_STR                                   L"Invert"

#define WIA_IPS_WARM_UP_TIME                                 6161 // 0x1811
#define WIA_IPS_WARM_UP_TIME_STR                             L"Lamp Warm up Time"

#if (_WIN32_WINNT >= 0x0600)

//
// New properties, property names and values specific to WIA 2.0 (introduced in Windows Vista):
//

#define WIA_DPS_USER_NAME                                    3112 // 0xc28
#define WIA_DPS_USER_NAME_STR                                L"User Name"

#define WIA_DPS_SERVICE_ID                                   3113 // 0xc29
#define WIA_DPS_SERVICE_ID_STR                               L"Service ID"

#define WIA_DPS_DEVICE_ID                                    3114 // 0xc2a
#define WIA_DPS_DEVICE_ID_STR                                L"Device ID"

#define WIA_DPS_GLOBAL_IDENTITY                              3115 // 0xc2b
#define WIA_DPS_GLOBAL_IDENTITY_STR                          L"Global Identity"

#define WIA_DPS_SCAN_AVAILABLE_ITEM                          3116 // 0xc2c
#define WIA_DPS_SCAN_AVAILABLE_ITEM_STR                      L"Scan Available Item"

#define WIA_IPS_DESKEW_X                                     6162 // 0x1812
#define WIA_IPS_DESKEW_X_STR                                 L"DeskewX"

#define WIA_IPS_DESKEW_Y                                     6163 // 0x1813
#define WIA_IPS_DESKEW_Y_STR                                 L"DeskewY"

#define WIA_IPS_SEGMENTATION                                 6164 // 0x1814
#define WIA_IPS_SEGMENTATION_STR                             L"Segmentation"

#define WIA_SEGMENTATION_FILTER_STR                          L"SegmentationFilter"
#define WIA_IMAGEPROC_FILTER_STR                             L"ImageProcessingFilter"

#define WIA_IPS_MAX_HORIZONTAL_SIZE                          6165 // 0x1815
#define WIA_IPS_MAX_HORIZONTAL_SIZE_STR                      L"Maximum Horizontal Scan Size"

#define WIA_IPS_MAX_VERTICAL_SIZE                            6166 // 0x1816
#define WIA_IPS_MAX_VERTICAL_SIZE_STR                        L"Maximum Vertical Scan Size"

#define WIA_IPS_MIN_HORIZONTAL_SIZE                          6167 // 0x1817
#define WIA_IPS_MIN_HORIZONTAL_SIZE_STR                      L"Minimum Horizontal Scan Size"

#define WIA_IPS_MIN_VERTICAL_SIZE                            6168 // 0x1818
#define WIA_IPS_MIN_VERTICAL_SIZE_STR                        L"Minimum Vertical Scan Size"

#define WIA_IPS_TRANSFER_CAPABILITIES                        6169 // 0x1819
#define WIA_IPS_TRANSFER_CAPABILITIES_STR                    L"Transfer Capabilities"

#define WIA_IPS_SHEET_FEEDER_REGISTRATION                    3078 // 0xc06
#define WIA_IPS_SHEET_FEEDER_REGISTRATION_STR                L"Sheet Feeder Registration"

#define WIA_IPS_DOCUMENT_HANDLING_SELECT                     3088 // 0xc10
#define WIA_IPS_DOCUMENT_HANDLING_SELECT_STR                 L"Document Handling Select"

#define WIA_IPS_OPTICAL_XRES                                 3090 // 0xc12
#define WIA_IPS_OPTICAL_XRES_STR                             L"Horizontal Optical Resolution"

#define WIA_IPS_OPTICAL_YRES                                 3091 // 0xc13
#define WIA_IPS_OPTICAL_YRES_STR                             L"Vertical Optical Resolution"

#define WIA_IPS_PAGES                                        3096 // 0xc18
#define WIA_IPS_PAGES_STR                                    L"Pages"

#define WIA_IPS_PAGE_SIZE                                    3097 // 0xc19
#define WIA_IPS_PAGE_SIZE_STR                                L"Page Size"

#define WIA_IPS_PAGE_WIDTH                                   3098 // 0xc1a
#define WIA_IPS_PAGE_WIDTH_STR                               L"Page Width"

#define WIA_IPS_PAGE_HEIGHT                                  3099 // 0xc1b
#define WIA_IPS_PAGE_HEIGHT_STR                              L"Page Height"

#define WIA_IPS_PREVIEW                                      3100 // 0xc1c
#define WIA_IPS_PREVIEW_STR                                  L"Preview"

#define WIA_IPS_SHOW_PREVIEW_CONTROL                         3103 // 0xc1f
#define WIA_IPS_SHOW_PREVIEW_CONTROL_STR                     L"Show preview control"

#define WIA_IPS_FILM_SCAN_MODE                               3104 // 0xc20
#define WIA_IPS_FILM_SCAN_MODE_STR                           L"Film Scan Mode"

#define WIA_IPS_LAMP                                         3105 // 0xc21
#define WIA_IPS_LAMP_STR                                     L"Lamp"

#define WIA_IPS_LAMP_AUTO_OFF                                3106 // 0xc22
#define WIA_IPS_LAMP_AUTO_OFF_STR                            L"Lamp Auto Off"

#define WIA_IPS_AUTO_DESKEW                                  3107 // 0xc23
#define WIA_IPS_AUTO_DESKEW_STR                              L"Automatic Deskew"

#define WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION                 3108 // 0xc24
#define WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION_STR             L"Supports Child Item Creation"

#define WIA_IPS_XSCALING                                     3109 // 0xc25
#define WIA_IPS_XSCALING_STR                                 L"Horizontal Scaling"

#define WIA_IPS_YSCALING                                     3110 // 0xc26
#define WIA_IPS_YSCALING_STR                                 L"Vertical Scaling"

#define WIA_IPS_PREVIEW_TYPE                                 3111 // 0xc27
#define WIA_IPS_PREVIEW_TYPE_STR                             L"Preview Type"

#define WIA_IPA_ITEM_CATEGORY                                4125 // 0x101d
#define WIA_IPA_ITEM_CATEGORY_STR                            L"Item Category"

#define WIA_IPA_UPLOAD_ITEM_SIZE                             4126 // 0x101e
#define WIA_IPA_UPLOAD_ITEM_SIZE_STR                         L"Upload Item Size"

#define WIA_IPA_ITEMS_STORED                                 4127 // 0x101f
#define WIA_IPA_ITEMS_STORED_STR                             L"Items Stored"

#define WIA_IPA_RAW_BITS_PER_CHANNEL                         4128 // 0x1020
#define WIA_IPA_RAW_BITS_PER_CHANNEL_STR                     L"Raw Bits Per Channel"

#define WIA_IPS_FILM_NODE_NAME                               4129 // 0x1021
#define WIA_IPS_FILM_NODE_NAME_STR                           L"Film Node Name"

#define WIA_IPS_PRINTER_ENDORSER                             4130 // 0x1022
#define WIA_IPS_PRINTER_ENDORSER_STR                         L"Printer/Endorser"

#define WIA_IPS_PRINTER_ENDORSER_ORDER                       4131 // 0x1023
#define WIA_IPS_PRINTER_ENDORSER_ORDER_STR                   L"Printer/Endorser Order"

#define WIA_IPS_PRINTER_ENDORSER_COUNTER                     4132 // 0x1024
#define WIA_IPS_PRINTER_ENDORSER_COUNTER_STR                 L"Printer/Endorser Counter"

#define WIA_IPS_PRINTER_ENDORSER_STEP                        4133 // 0x1025
#define WIA_IPS_PRINTER_ENDORSER_STEP_STR                    L"Printer/Endorser Step"

#define WIA_IPS_PRINTER_ENDORSER_XOFFSET                     4134 // 0x1026
#define WIA_IPS_PRINTER_ENDORSER_XOFFSET_STR                 L"Printer/Endorser Horizontal Offset"

#define WIA_IPS_PRINTER_ENDORSER_YOFFSET                     4135 // 0x1027
#define WIA_IPS_PRINTER_ENDORSER_YOFFSET_STR                 L"Printer/Endorser Vertical Offset"

#define WIA_IPS_PRINTER_ENDORSER_NUM_LINES                   4136 // 0x1028
#define WIA_IPS_PRINTER_ENDORSER_NUM_LINES_STR               L"Printer/Endorser Lines"

#define WIA_IPS_PRINTER_ENDORSER_STRING                      4137 // 0x1029
#define WIA_IPS_PRINTER_ENDORSER_STRING_STR                  L"Printer/Endorser String"

#define WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS            4138 // 0x102a
#define WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS_STR        L"Printer/Endorser Valid Characters"

#define WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS     4139 // 0x102b
#define WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS_STR L"Printer/Endorser Valid Format Specifiers"

#define WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD                 4140 // 0x102c
#define WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD_STR             L"Printer/Endorser Text Upload"

#define WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD               4141 // 0x102d
#define WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD_STR           L"Printer/Endorser Text Download"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS                    4142 // 0x102e
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_STR                L"Printer/Endorser Graphics"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION           4143 // 0x102f
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION_STR       L"Printer/Endorser Graphics Position"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH          4144 // 0x1030
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH_STR      L"Printer/Endorser Graphics Minimum Width"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH          4145 // 0x1031
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH_STR      L"Printer/Endorser Graphics Maximum Width"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT         4146 // 0x1032
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT_STR     L"Printer/Endorser Graphics Minimum Height"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT         4147 // 0x1033
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT_STR     L"Printer/Endorser Graphics Maximum Height"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD             4148 // 0x1034
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD_STR         L"Printer/Endorser Graphics Upload"

#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD           4149 // 0x1035
#define WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD_STR       L"Printer/Endorser Graphics Download"

#define WIA_IPS_BARCODE_READER                               4150 // 0x1036
#define WIA_IPS_BARCODE_READER_STR                           L"Barcode Reader"

#define WIA_IPS_MAXIMUM_BARCODES_PER_PAGE                    4151 // 0x1037
#define WIA_IPS_MAXIMUM_BARCODES_PER_PAGE_STR                L"Maximum Barcodes Per Page"

#define WIA_IPS_BARCODE_SEARCH_DIRECTION                     4152 // 0x1038
#define WIA_IPS_BARCODE_SEARCH_DIRECTION_STR                 L"Barcode Search Direction"

#define WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES               4153 // 0x1039
#define WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES_STR           L"Barcode Search Retries"

#define WIA_IPS_BARCODE_SEARCH_TIMEOUT                       4154 // 0x103a
#define WIA_IPS_BARCODE_SEARCH_TIMEOUT_STR                   L"Barcode Search Timeout"

#define WIA_IPS_SUPPORTED_BARCODE_TYPES                      4155 // 0x103b
#define WIA_IPS_SUPPORTED_BARCODE_TYPES_STR                  L"Supported Barcode Types"

#define WIA_IPS_ENABLED_BARCODE_TYPES                        4156 // 0x103c
#define WIA_IPS_ENABLED_BARCODE_TYPES_STR                    L"Enabled Barcode Types"

#define WIA_IPS_PATCH_CODE_READER                            4157 // 0x103d
#define WIA_IPS_PATCH_CODE_READER_STR                        L"Patch Code Reader"

#define WIA_IPS_SUPPORTED_PATCH_CODE_TYPES                   4162 // 0x1042
#define WIA_IPS_SUPPORTED_PATCH_CODE_TYPES_STR               L"Supported Patch Code Types"

#define WIA_IPS_ENABLED_PATCH_CODE_TYPES                     4163 // 0x1043
#define WIA_IPS_ENABLED_PATCH_CODE_TYPES_STR                 L"Enabled Path Code Types"

#define WIA_IPS_MICR_READER                                  4164 // 0x1044
#define WIA_IPS_MICR_READER_STR                              L"MICR Reader"

#define WIA_IPS_JOB_SEPARATORS                               4165 // 0x1045
#define WIA_IPS_JOB_SEPARATORS_STR                           L"Job Separators"

#define WIA_IPS_LONG_DOCUMENT                                4166 // 0x1046
#define WIA_IPS_LONG_DOCUMENT_STR                            L"Long Document"

#define WIA_IPS_BLANK_PAGES                                  4167 // 0x1047
#define WIA_IPS_BLANK_PAGES_STR                              L"Blank Pages"

#define WIA_IPS_MULTI_FEED                                   4168 // 0x1048
#define WIA_IPS_MULTI_FEED_STR                               L"Multi-Feed"

#define WIA_IPS_MULTI_FEED_SENSITIVITY                       4169 // 0x1049
#define WIA_IPS_MULTI_FEED_SENSITIVITY_STR                   L"Multi-Feed Sensitivity"

#define WIA_IPS_AUTO_CROP                                    4170 // 0x104a
#define WIA_IPS_AUTO_CROP_STR                                L"Auto-Crop"

#define WIA_IPS_OVER_SCAN                                    4171 // 0x104b
#define WIA_IPS_OVER_SCAN_STR                                L"Overscan"

#define WIA_IPS_OVER_SCAN_LEFT                               4172 // 0x104c
#define WIA_IPS_OVER_SCAN_LEFT_STR                           L"Overscan Left"

#define WIA_IPS_OVER_SCAN_RIGHT                              4173 // 0x104d
#define WIA_IPS_OVER_SCAN_RIGHT_STR                          L"Overscan Right"

#define WIA_IPS_OVER_SCAN_TOP                                4174 // 0x104e
#define WIA_IPS_OVER_SCAN_TOP_STR                            L"Overscan Top"

#define WIA_IPS_OVER_SCAN_BOTTOM                             4175 // 0x104f
#define WIA_IPS_OVER_SCAN_BOTTOM_STR                         L"Overscan Bottom"

#define WIA_IPS_COLOR_DROP                                   4176 // 0x1050
#define WIA_IPS_COLOR_DROP_STR                               L"Color Drop"

#define WIA_IPS_COLOR_DROP_RED                               4177 // 0x1051
#define WIA_IPS_COLOR_DROP_RED_STR                           L"Color Drop Red"

#define WIA_IPS_COLOR_DROP_GREEN                             4178 // 0x1052
#define WIA_IPS_COLOR_DROP_GREEN_STR                         L"Color Drop Green"

#define WIA_IPS_COLOR_DROP_BLUE                              4179 // 0x1053
#define WIA_IPS_COLOR_DROP_BLUE_STR                          L"Color Drop Blue"

#define WIA_IPS_SCAN_AHEAD                                   4180 // 0x1054
#define WIA_IPS_SCAN_AHEAD_STR                               L"Scan Ahead"

#define WIA_IPS_SCAN_AHEAD_CAPACITY                          4181 // 0x1055
#define WIA_IPS_SCAN_AHEAD_CAPACITY_STR                      L"Scan Ahead Capacity"

#define WIA_IPS_FEEDER_CONTROL                               4182 // 0x1056
#define WIA_IPS_FEEDER_CONTROL_STR                           L"Feeder Control"

#define WIA_IPS_PRINTER_ENDORSER_PADDING                     4183 // 0x1057
#define WIA_IPS_PRINTER_ENDORSER_PADDING_STR                 L"Printer/Endorser Padding"

#define WIA_IPS_PRINTER_ENDORSER_FONT_TYPE                   4184 // 0x1058
#define WIA_IPS_PRINTER_ENDORSER_FONT_TYPE_STR               L"Printer/Endorser Font Type"

#define WIA_IPS_ALARM                                        4185 // 0x1059
#define WIA_IPS_ALARM_STR                                    L"Alarm"

#define WIA_IPS_PRINTER_ENDORSER_INK                         4186 // 0x105A
#define WIA_IPS_PRINTER_ENDORSER_INK_STR                     L"Printer/Endorser Ink"

#define WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION          4187 // 0x105B
#define WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION_STR      L"Printer/Endorser Character Rotation"

#define WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS              4188 // 0x105C
#define WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS_STR          L"Printer/Endorser Maximum Characters"

#define WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS                4189 // 0x105D
#define WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS_STR            L"Printer/Endorser Maximum Graphics"

#define WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS              4190 // 0x105E
#define WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS_STR          L"Printer/Endorser Counter Digits"

#define WIA_IPS_COLOR_DROP_MULTI                             4191 // 0x105F
#define WIA_IPS_COLOR_DROP_MULTI_STR                         L"Color Drop Multiple"

#define WIA_IPS_BLANK_PAGES_SENSITIVITY                      4192 // 0x1060
#define WIA_IPS_BLANK_PAGES_SENSITIVITY_STR                  L"Blank Pages Sensitivity"

#define WIA_IPS_MULTI_FEED_DETECT_METHOD                     4193 // 0x1061
#define WIA_IPS_MULTI_FEED_DETECT_METHOD_STR                 L"Multi-Feed Detection Method"

//
// WIA_IPA_ITEM_CATEGORY constants
//

DEFINE_GUID(WIA_CATEGORY_FINISHED_FILE,     0xff2b77ca, 0xcf84, 0x432b, 0xa7, 0x35, 0x3a, 0x13, 0x0d, 0xde, 0x2a, 0x88);
DEFINE_GUID(WIA_CATEGORY_FLATBED,           0xfb607b1f, 0x43f3, 0x488b, 0x85, 0x5b, 0xfb, 0x70, 0x3e, 0xc3, 0x42, 0xa6);
DEFINE_GUID(WIA_CATEGORY_FEEDER,            0xfe131934, 0xf84c, 0x42ad, 0x8d, 0xa4, 0x61, 0x29, 0xcd, 0xdd, 0x72, 0x88);
DEFINE_GUID(WIA_CATEGORY_FILM,              0xfcf65be7, 0x3ce3, 0x4473, 0xaf, 0x85, 0xf5, 0xd3, 0x7d, 0x21, 0xb6, 0x8a);
DEFINE_GUID(WIA_CATEGORY_ROOT,              0xf193526f, 0x59b8, 0x4a26, 0x98, 0x88, 0xe1, 0x6e, 0x4f, 0x97, 0xce, 0x10);
DEFINE_GUID(WIA_CATEGORY_FOLDER,            0xc692a446, 0x6f5a, 0x481d, 0x85, 0xbb, 0x92, 0xe2, 0xe8, 0x6f, 0xd3, 0xa);
DEFINE_GUID(WIA_CATEGORY_FEEDER_FRONT,      0x4823175c, 0x3b28, 0x487b, 0xa7, 0xe6, 0xee, 0xbc, 0x17, 0x61, 0x4f, 0xd1);
DEFINE_GUID(WIA_CATEGORY_FEEDER_BACK,       0x61ca74d4, 0x39db, 0x42aa, 0x89, 0xb1, 0x8c, 0x19, 0xc9, 0xcd, 0x4c, 0x23);
DEFINE_GUID(WIA_CATEGORY_AUTO,              0xdefe5fd8, 0x6c97, 0x4dde, 0xb1, 0x1e, 0xcb, 0x50, 0x9b, 0x27, 0x0e, 0x11);
DEFINE_GUID(WIA_CATEGORY_IMPRINTER,         0xfc65016d, 0x9202, 0x43dd, 0x91, 0xa7, 0x64, 0xc2, 0x95, 0x4c, 0xfb, 0x8b);
DEFINE_GUID(WIA_CATEGORY_ENDORSER,          0x47102cc3, 0x127f, 0x4771, 0xad, 0xfc, 0x99, 0x1a, 0xb8, 0xee, 0x1e, 0x97);
DEFINE_GUID(WIA_CATEGORY_BARCODE_READER,    0x36e178a0, 0x473f, 0x494b, 0xaf, 0x8f, 0x6c, 0x3f, 0x6d, 0x74, 0x86, 0xfc);
DEFINE_GUID(WIA_CATEGORY_PATCH_CODE_READER, 0x8faa1a6d, 0x9c8a, 0x42cd, 0x98, 0xb3, 0xee, 0x97, 0x00, 0xcb, 0xc7, 0x4f);
DEFINE_GUID(WIA_CATEGORY_MICR_READER,       0x3b86c1ec, 0x71bc, 0x4645, 0xb4, 0xd5, 0x1b, 0x19, 0xda, 0x2b, 0xe9, 0x78);

//
// Default Segmentation Filter GUID
//

DEFINE_GUID(CLSID_WiaDefaultSegFilter, 0xD4F4D30B, 0x0B29, 0x4508, 0x89, 0x22, 0x0C, 0x57, 0x97, 0xD4, 0x27, 0x65);

//
// WIA_IPS_TRANSFER_CAPABILITIES flags:
//

#define WIA_TRANSFER_CHILDREN_SINGLE_SCAN              0x00000001

//
// WIA_IPS_SEGMENTATION_FILTER constants
//

#define WIA_USE_SEGMENTATION_FILTER                    0
#define WIA_DONT_USE_SEGMENTATION_FILTER               1

//
// WIA_IPS_FILM_SCAN_MODE constants
//

#define WIA_FILM_COLOR_SLIDE                           0
#define WIA_FILM_COLOR_NEGATIVE                        1
#define WIA_FILM_BW_NEGATIVE                           2

//
// WIA_IPS_LAMP constants
//

#define WIA_LAMP_ON                                    0
#define WIA_LAMP_OFF                                   1

//
// WIA_IPS_AUTO_DESKEW constants
//

#define WIA_AUTO_DESKEW_ON                             0
#define WIA_AUTO_DESKEW_OFF                            1

//
// WIA_IPS_PREVIEW_TYPE constants
//

#define WIA_ADVANCED_PREVIEW                           0
#define WIA_BASIC_PREVIEW                              1

//
// WIA_IPS_PRINTER_ENDORSER constants
//

#define WIA_PRINTER_ENDORSER_DISABLED                  0
#define WIA_PRINTER_ENDORSER_AUTO                      1
#define WIA_PRINTER_ENDORSER_FLATBED                   2
#define WIA_PRINTER_ENDORSER_FEEDER_FRONT              3
#define WIA_PRINTER_ENDORSER_FEEDER_BACK               4
#define WIA_PRINTER_ENDORSER_FEEDER_DUPLEX             5
#define WIA_PRINTER_ENDORSER_DIGITAL                   6

//
// WIA_IPS_PRINTER_ENDORSER_ORDER constants
//

#define WIA_PRINTER_ENDORSER_BEFORE_SCAN               0
#define WIA_PRINTER_ENDORSER_AFTER_SCAN                1

//
// WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS constants
//

#define WIA_PRINT_DATE                                 0 //L"$DATE$"
#define WIA_PRINT_YEAR                                 1 //L"$YEAR$"
#define WIA_PRINT_MONTH                                2 //L"$MONTH$"
#define WIA_PRINT_DAY                                  3 //L"$DAY$"
#define WIA_PRINT_WEEK_DAY                             4 //L"$WEEK_DAY$"
#define WIA_PRINT_TIME_24H                             5 //L"$TIME$"
#define WIA_PRINT_TIME_12H                             6 //L"$TIME_12H$"
#define WIA_PRINT_HOUR_24H                             7 //L"$HOUR_24H$"
#define WIA_PRINT_HOUR_12H                             8 //L"$HOUR_12H$"
#define WIA_PRINT_AM_PM                                9 //L"$AM_PM$"
#define WIA_PRINT_MINUTE                              10 //L"$MINUTE$"
#define WIA_PRINT_SECOND                              11 //L"$SECOND$"
#define WIA_PRINT_PAGE_COUNT                          12 //L"$PAGE_COUNT$"
#define WIA_PRINT_IMAGE                               13 //L"$IMAGE$"
#define WIA_PRINT_MILLISECOND                         14 //L"$MSECOND$"
#define WIA_PRINT_MONTH_NAME                          15 //L"$MONTH_NAME$"
#define WIA_PRINT_MONTH_SHORT                         16 //L"$MONTH_SHORT$"
#define WIA_PRINT_WEEK_DAY_SHORT                      17 //L"$WEEK_DAY_SHORT$"

//
// WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION constants
//

#define WIA_PRINTER_ENDORSER_GRAPHICS_LEFT             0
#define WIA_PRINTER_ENDORSER_GRAPHICS_RIGHT            1
#define WIA_PRINTER_ENDORSER_GRAPHICS_TOP              2
#define WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM           3
#define WIA_PRINTER_ENDORSER_GRAPHICS_TOP_LEFT         4
#define WIA_PRINTER_ENDORSER_GRAPHICS_TOP_RIGHT        5
#define WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_LEFT      6
#define WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_RIGHT     7
#define WIA_PRINTER_ENDORSER_GRAPHICS_BACKGROUND       8
#define WIA_PRINTER_ENDORSER_GRAPHICS_DEVICE_DEFAULT   9

//
// WIA_IPS_BARCODE_READER constants
//

#define WIA_BARCODE_READER_DISABLED                    0
#define WIA_BARCODE_READER_AUTO                        1
#define WIA_BARCODE_READER_FLATBED                     2
#define WIA_BARCODE_READER_FEEDER_FRONT                3
#define WIA_BARCODE_READER_FEEDER_BACK                 4
#define WIA_BARCODE_READER_FEEDER_DUPLEX               5

//
// The WIA_IPS_BARCODE_SEARCH_DIRECTION constants
//

#define WIA_BARCODE_HORIZONTAL_SEARCH                  0
#define WIA_BARCODE_VERTICAL_SEARCH                    1
#define WIA_BARCODE_HORIZONTAL_VERTICAL_SEARCH         2
#define WIA_BARCODE_VERTICAL_HORIZONTAL_SEARCH         3
#define WIA_BARCODE_AUTO_SEARCH                        4

//
// WIA_IPS_SUPPORTED_BARCODE_TYPES constants
//

#define WIA_BARCODE_UPCA                               0
#define WIA_BARCODE_UPCE                               1
#define WIA_BARCODE_CODABAR                            2
#define WIA_BARCODE_NONINTERLEAVED_2OF5                3
#define WIA_BARCODE_INTERLEAVED_2OF5                   4
#define WIA_BARCODE_CODE39                             5
#define WIA_BARCODE_CODE39_MOD43                       6
#define WIA_BARCODE_CODE39_FULLASCII                   7
#define WIA_BARCODE_CODE93                             8
#define WIA_BARCODE_CODE128                            9
#define WIA_BARCODE_CODE128A                          10
#define WIA_BARCODE_CODE128B                          11
#define WIA_BARCODE_CODE128C                          12
#define WIA_BARCODE_GS1128                            13
#define WIA_BARCODE_GS1DATABAR                        14
#define WIA_BARCODE_ITF14                             15
#define WIA_BARCODE_EAN8                              16
#define WIA_BARCODE_EAN13                             17
#define WIA_BARCODE_POSTNETA                          18
#define WIA_BARCODE_POSTNETB                          19
#define WIA_BARCODE_POSTNETC                          20
#define WIA_BARCODE_POSTNET_DPBC                      21
#define WIA_BARCODE_PLANET                            22
#define WIA_BARCODE_INTELLIGENT_MAIL                  23
#define WIA_BARCODE_POSTBAR                           24
#define WIA_BARCODE_RM4SCC                            25
#define WIA_BARCODE_HIGH_CAPACITY_COLOR               26
#define WIA_BARCODE_MAXICODE                          27
#define WIA_BARCODE_PDF417                            28
#define WIA_BARCODE_CPCBINARY                         29
#define WIA_BARCODE_FIM                               30
#define WIA_BARCODE_PHARMACODE                        31
#define WIA_BARCODE_PLESSEY                           32
#define WIA_BARCODE_MSI                               33
#define WIA_BARCODE_JAN                               34
#define WIA_BARCODE_TELEPEN                           35
#define WIA_BARCODE_AZTEC                             36
#define WIA_BARCODE_SMALLAZTEC                        37
#define WIA_BARCODE_DATAMATRIX                        38
#define WIA_BARCODE_DATASTRIP                         39
#define WIA_BARCODE_EZCODE                            40
#define WIA_BARCODE_QRCODE                            41
#define WIA_BARCODE_SHOTCODE                          42
#define WIA_BARCODE_SPARQCODE                         43
#define WIA_BARCODE_CUSTOMBASE                    0x8000

//
// WIA_IPS_PATH_CODE_READER constants
//

#define WIA_PATCH_CODE_READER_DISABLED                 0
#define WIA_PATCH_CODE_READER_AUTO                     1
#define WIA_PATCH_CODE_READER_FLATBED                  2
#define WIA_PATCH_CODE_READER_FEEDER_FRONT             3
#define WIA_PATCH_CODE_READER_FEEDER_BACK              4
#define WIA_PATCH_CODE_READER_FEEDER_DUPLEX            5

//
// WIA_IPS_SUPPORTED_PATCH_CODE_TYPES constants
//

#define WIA_PATCH_CODE_UNKNOWN                         0
#define WIA_PATCH_CODE_1                               1
#define WIA_PATCH_CODE_2                               2
#define WIA_PATCH_CODE_3                               3
#define WIA_PATCH_CODE_4                               4
#define WIA_PATCH_CODE_T                               5
#define WIA_PATCH_CODE_6                               6
#define WIA_PATCH_CODE_7                               7
#define WIA_PATCH_CODE_8                               8
#define WIA_PATCH_CODE_9                               9
#define WIA_PATCH_CODE_10                             10
#define WIA_PATCH_CODE_11                             11
#define WIA_PATCH_CODE_12                             12
#define WIA_PATCH_CODE_13                             13
#define WIA_PATCH_CODE_14                             14
#define WIA_PATCH_CODE_CUSTOM_BASE                0x8000

//
// WIA_IPS_MICR_READER constants
//

#define WIA_MICR_READER_DISABLED                       0
#define WIA_MICR_READER_AUTO                           1
#define WIA_MICR_READER_FLATBED                        2
#define WIA_MICR_READER_FEEDER_FRONT                   3
#define WIA_MICR_READER_FEEDER_BACK                    4
#define WIA_MICR_READER_FEEDER_DUPLEX                  5

//
// WIA_IPS_JOB_SEPARATORS constants
//

#define WIA_SEPARATOR_DISABLED                         0
#define WIA_SEPARATOR_DETECT_SCAN_CONTINUE             1
#define WIA_SEPARATOR_DETECT_SCAN_STOP                 2
#define WIA_SEPARATOR_DETECT_NOSCAN_CONTINUE           3
#define WIA_SEPARATOR_DETECT_NOSCAN_STOP               4

//
// WIA_IPS_LONG_DOCUMENT constants
//

#define WIA_LONG_DOCUMENT_DISABLED                     0
#define WIA_LONG_DOCUMENT_ENABLED                      1
#define WIA_LONG_DOCUMENT_SPLIT                        2

//
// WIA_IPS_BLANK_PAGES constants
//

#define WIA_BLANK_PAGE_DETECTION_DISABLED              0
#define WIA_BLANK_PAGE_DISCARD                         1
#define WIA_BLANK_PAGE_JOB_SEPARATOR                   2

//
// WIA_IPS_MULTI_FEED constants
//

#define WIA_MULTI_FEED_DETECT_DISABLED                 0
#define WIA_MULTI_FEED_DETECT_STOP_ERROR               1
#define WIA_MULTI_FEED_DETECT_STOP_SUCCESS             2
#define WIA_MULTI_FEED_DETECT_CONTINUE                 3

//
// WIA_IPS_MULTI_FEED_DETECT_METHOD constants
//

#define WIA_MULTI_FEED_DETECT_METHOD_LENGTH            0
#define WIA_MULTI_FEED_DETECT_METHOD_OVERLAP           1

//
// WIA_IPS_AUTO_CROP constants
//

#define WIA_AUTO_CROP_DISABLED                         0
#define WIA_AUTO_CROP_SINGLE                           1
#define WIA_AUTO_CROP_MULTI                            2

//
// WIA_IPS_OVER_SCAN constants
//

#define WIA_OVER_SCAN_DISABLED                         0
#define WIA_OVER_SCAN_TOP_BOTTOM                       1
#define WIA_OVER_SCAN_LEFT_RIGHT                       2
#define WIA_OVER_SCAN_ALL                              3

//
// WIA_IPS_COLOR_DROP constants
//

#define WIA_COLOR_DROP_DISABLED                        0
#define WIA_COLOR_DROP_RED                             1
#define WIA_COLOR_DROP_GREEN                           2
#define WIA_COLOR_DROP_BLUE                            3
#define WIA_COLOR_DROP_RGB                             4

//
// WIA_IPS_SCAN_AHEAD constants
//

#define WIA_SCAN_AHEAD_DISABLED                        0
#define WIA_SCAN_AHEAD_ENABLED                         1

//
// WIA_IPS_FEEDER_CONTROL constants
//

#define WIA_FEEDER_CONTROL_AUTO                        0
#define WIA_FEEDER_CONTROL_MANUAL                      1

//
// WIA_IPS_PRINTER_ENDORSER_PADDING constants
//

#define WIA_PRINT_PADDING_NONE                         0
#define WIA_PRINT_PADDING_ZERO                         1
#define WIA_PRINT_PADDING_BLANK                        2

//
// WIA_IPS_PRINTER_ENDORSER_FONT_TYPE constants
//

#define WIA_PRINT_FONT_NORMAL                          0
#define WIA_PRINT_FONT_BOLD                            1
#define WIA_PRINT_FONT_EXTRA_BOLD                      2
#define WIA_PRINT_FONT_ITALIC_BOLD                     3
#define WIA_PRINT_FONT_ITALIC_EXTRA_BOLD               4
#define WIA_PRINT_FONT_ITALIC                          5
#define WIA_PRINT_FONT_SMALL                           6
#define WIA_PRINT_FONT_SMALL_BOLD                      7
#define WIA_PRINT_FONT_SMALL_EXTRA_BOLD                8
#define WIA_PRINT_FONT_SMALL_ITALIC_BOLD               9
#define WIA_PRINT_FONT_SMALL_ITALIC_EXTRA_BOLD        10
#define WIA_PRINT_FONT_SMALL_ITALIC                   11
#define WIA_PRINT_FONT_LARGE                          12
#define WIA_PRINT_FONT_LARGE_BOLD                     13
#define WIA_PRINT_FONT_LARGE_EXTRA_BOLD               14
#define WIA_PRINT_FONT_LARGE_ITALIC_BOLD              15
#define WIA_PRINT_FONT_LARGE_ITALIC_EXTRA_BOLD        16
#define WIA_PRINT_FONT_LARGE_ITALIC                   17

//
// WIA_IPS_ALARM constants
//

#define WIA_ALARM_NONE                                 0
#define WIA_ALARM_BEEP1                                1
#define WIA_ALARM_BEEP2                                2
#define WIA_ALARM_BEEP3                                3
#define WIA_ALARM_BEEP4                                4
#define WIA_ALARM_BEEP5                                5
#define WIA_ALARM_BEEP6                                6
#define WIA_ALARM_BEEP7                                7
#define WIA_ALARM_BEEP8                                8
#define WIA_ALARM_BEEP9                                9
#define WIA_ALARM_BEEP10                              10

//
// WIA Raw Format header structure:
//

typedef struct _WIA_RAW_HEADER
{
    DWORD Tag;
    DWORD Version;
    DWORD HeaderSize;
    DWORD XRes;
    DWORD YRes;
    DWORD XExtent;
    DWORD YExtent;
    DWORD BytesPerLine;
    DWORD BitsPerPixel;
    DWORD ChannelsPerPixel;
    DWORD DataType;
    BYTE  BitsPerChannel[8];
    DWORD Compression;
    DWORD PhotometricInterp;
    DWORD LineOrder;
    DWORD RawDataOffset;
    DWORD RawDataSize;
    DWORD PaletteOffset;
    DWORD PaletteSize;
} WIA_RAW_HEADER;

typedef struct _WIA_RAW_HEADER *PWIA_RAW_HEADER;

//
// Barcode Metadata Raw Format structures:
//

typedef struct _WIA_BARCODE_INFO
{
    DWORD Size;
    DWORD Type;
    DWORD Page;
    DWORD Confidence;
    DWORD XOffset;
    DWORD YOffset;
    DWORD Rotation;
    DWORD Length;
    WCHAR Text[1];
} WIA_BARCODE_INFO;

typedef struct _WIA_BARCODES
{
    DWORD Tag;
    DWORD Version;
    DWORD Size;
    DWORD Count;
    WIA_BARCODE_INFO Barcodes[1];
} WIA_BARCODES;

//
// Patch Code Metadata Raw Format structures:
//

typedef struct _WIA_PATCH_CODE_INFO
{
    DWORD Type;
} WIA_PATCH_CODE_INFO;

typedef struct _WIA_PATCH_CODES
{
    DWORD Tag;
    DWORD Version;
    DWORD Size;
    DWORD Count;
    WIA_PATCH_CODE_INFO PatchCodes[1];
} WIA_PATCH_CODES;

//
// MICR Metadata Raw Format structures:
//

typedef struct _WIA_MICR_INFO
{
    DWORD Size;
    DWORD Page;
    DWORD Length;
    WCHAR Text[1];
}   WIA_MICR_INFO;

typedef struct _WIA_MICR
{
    DWORD Tag;
    DWORD Version;
    DWORD Size;
    WCHAR Placeholder;
    WORD  Reserved;
    DWORD Count;
    WIA_MICR_INFO Micr[1];
} WIA_MICR;


#endif //#if (_WIN32_WINNT >= 0x0600)

//
// Use the WIA property offsets to define private WIA properties.
//
// Example: (Creating a private WIA property)
//
// #define WIA_THE_PROP         (WIA_PRIVATE_DEVPROP + 1)
// #define WIA_THE_PROP_STR     L"Property Name")
//

//
// Private property offset constants
//

#define WIA_PRIVATE_DEVPROP      38914 // offset for private device (root) item properties
#define WIA_PRIVATE_ITEMPROP     71682 // offset for private item properties

//
// WIA image format constants
//

DEFINE_GUID(WiaImgFmt_UNDEFINED, 0xb96b3ca9, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_RAWRGB,    0xbca48b55, 0xf272, 0x4371, 0xb0, 0xf1, 0x4a, 0x15, 0x0d, 0x05, 0x7b, 0xb4);
DEFINE_GUID(WiaImgFmt_MEMORYBMP, 0xb96b3caa, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_BMP,       0xb96b3cab, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_EMF,       0xb96b3cac, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_WMF,       0xb96b3cad, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_JPEG,      0xb96b3cae, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_PNG,       0xb96b3caf, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_GIF,       0xb96b3cb0, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_TIFF,      0xb96b3cb1, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_EXIF,      0xb96b3cb2, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_PHOTOCD,   0xb96b3cb3, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_FLASHPIX,  0xb96b3cb4, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_ICO,       0xb96b3cb5, 0x0728, 0x11d3, 0x9d, 0x7b, 0x00, 0x00, 0xf8, 0x1e, 0xf3, 0x2e);
DEFINE_GUID(WiaImgFmt_CIFF,      0x9821a8ab, 0x3a7e, 0x4215, 0x94, 0xe0, 0xd2, 0x7a, 0x46, 0x0c, 0x03, 0xb2);
DEFINE_GUID(WiaImgFmt_PICT,      0xa6bc85d8, 0x6b3e, 0x40ee, 0xa9, 0x5c, 0x25, 0xd4, 0x82, 0xe4, 0x1a, 0xdc);
DEFINE_GUID(WiaImgFmt_JPEG2K,    0x344ee2b2, 0x39db, 0x4dde, 0x81, 0x73, 0xc4, 0xb7, 0x5f, 0x8f, 0x1e, 0x49);
DEFINE_GUID(WiaImgFmt_JPEG2KX,   0x43e14614, 0xc80a, 0x4850, 0xba, 0xf3, 0x4b, 0x15, 0x2d, 0xc8, 0xda, 0x27);
#if (_WIN32_WINNT >= 0x0600)
DEFINE_GUID(WiaImgFmt_RAW,       0x6f120719, 0xf1a8, 0x4e07, 0x9a, 0xde, 0x9b, 0x64, 0xc6, 0x3a, 0x3d, 0xcc);
DEFINE_GUID(WiaImgFmt_JBIG,      0x41e8dd92, 0x2f0a, 0x43d4, 0x86, 0x36, 0xf1, 0x61, 0x4b, 0xa1, 0x1e, 0x46);
DEFINE_GUID(WiaImgFmt_JBIG2,     0xbb8e7e67, 0x283c, 0x4235, 0x9e, 0x59, 0x0b, 0x9b, 0xf9, 0x4c, 0xa6, 0x87);
#endif //#if (_WIN32_WINNT >= 0x0600)

//
// WIA document format constants
//

DEFINE_GUID(WiaImgFmt_RTF,       0x573dd6a3, 0x4834, 0x432d, 0xa9, 0xb5, 0xe1, 0x98, 0xdd, 0x9e, 0x89, 0x0d);
DEFINE_GUID(WiaImgFmt_XML,       0xb9171457, 0xdac8, 0x4884, 0xb3, 0x93, 0x15, 0xb4, 0x71, 0xd5, 0xf0, 0x7e);
DEFINE_GUID(WiaImgFmt_HTML,      0xc99a4e62, 0x99de, 0x4a94, 0xac, 0xca, 0x71, 0x95, 0x6a, 0xc2, 0x97, 0x7d);
DEFINE_GUID(WiaImgFmt_TXT,       0xfafd4d82, 0x723f, 0x421f, 0x93, 0x18, 0x30, 0x50, 0x1a, 0xc4, 0x4b, 0x59);
#if (_WIN32_WINNT >= 0x0600)
DEFINE_GUID(WiaImgFmt_PDFA,      0x9980bd5b, 0x3463, 0x43c7, 0xbd, 0xca, 0x3c, 0xaa, 0x14, 0x6f, 0x22, 0x9f);
DEFINE_GUID(WiaImgFmt_XPS,       0x700b4a0f, 0x2011, 0x411c, 0xb4, 0x30, 0xd1, 0xe0, 0xb2, 0xe1, 0x0b, 0x28);
DEFINE_GUID(WiaImgFmt_OXPS,      0x2c7b1240, 0xc14d, 0x4109, 0x97, 0x55, 0x04, 0xb8, 0x90, 0x25, 0x15, 0x3a);
DEFINE_GUID(WiaImgFmt_CSV,       0x355bda24, 0x5a9f, 0x4494, 0x80, 0xdc, 0xbe, 0x75, 0x2c, 0xec, 0xbc, 0x8c);
#endif //#if (_WIN32_WINNT >= 0x0600)

//
// WIA video format constants
//

DEFINE_GUID(WiaImgFmt_MPG,       0xecd757e4, 0xd2ec, 0x4f57, 0x95, 0x5d, 0xbc, 0xf8, 0xa9, 0x7c, 0x4e, 0x52);
DEFINE_GUID(WiaImgFmt_AVI,       0x32f8ca14, 0x087c, 0x4908, 0xb7, 0xc4, 0x67, 0x57, 0xfe, 0x7e, 0x90, 0xab);

//
// WIA audio format constants
//

DEFINE_GUID(WiaAudFmt_WAV,       0xf818e146, 0x07af, 0x40ff, 0xae, 0x55, 0xbe, 0x8f, 0x2c, 0x06, 0x5d, 0xbe);
DEFINE_GUID(WiaAudFmt_MP3,       0x0fbc71fb, 0x43bf, 0x49f2, 0x91, 0x90, 0xe6, 0xfe, 0xcf, 0xf3, 0x7e, 0x54);
DEFINE_GUID(WiaAudFmt_AIFF,      0x66e2bf4f, 0xb6fc, 0x443f, 0x94, 0xc8, 0x2f, 0x33, 0xc8, 0xa6, 0x5a, 0xaf);
DEFINE_GUID(WiaAudFmt_WMA,       0xd61d6413, 0x8bc2, 0x438f, 0x93, 0xad, 0x21, 0xbd, 0x48, 0x4d, 0xb6, 0xa1);

//
// WIA misc format constants
//

DEFINE_GUID(WiaImgFmt_ASF,       0x8d948ee9, 0xd0aa, 0x4a12, 0x9d, 0x9a, 0x9c, 0xc5, 0xde, 0x36, 0x19, 0x9b);
DEFINE_GUID(WiaImgFmt_SCRIPT,    0xfe7d6c53, 0x2dac, 0x446a, 0xb0, 0xbd, 0xd7, 0x3e, 0x21, 0xe9, 0x24, 0xc9);
DEFINE_GUID(WiaImgFmt_EXEC,      0x485da097, 0x141e, 0x4aa5, 0xbb, 0x3b, 0xa5, 0x61, 0x8d, 0x95, 0xd0, 0x2b);
DEFINE_GUID(WiaImgFmt_UNICODE16, 0x1b7639b6, 0x6357, 0x47d1, 0x9a, 0x07, 0x12, 0x45, 0x2d, 0xc0, 0x73, 0xe9);
DEFINE_GUID(WiaImgFmt_DPOF,      0x369eeeab, 0xa0e8, 0x45ca, 0x86, 0xa6, 0xa8, 0x3c, 0xe5, 0x69, 0x7e, 0x28);

//
// WIA meta-data format constants
//

DEFINE_GUID(WiaImgFmt_XMLBAR,   0x6235701c, 0x3a98, 0x484c, 0xb2, 0xa8, 0xfd, 0xff, 0xd8, 0x7e, 0x6b, 0x16);
DEFINE_GUID(WiaImgFmt_RAWBAR,   0xda63f833, 0xd26e, 0x451e, 0x90, 0xd2, 0xea, 0x55, 0xa1, 0x36, 0x5d, 0x62);
DEFINE_GUID(WiaImgFmt_XMLPAT,   0xf8986f55, 0xf052, 0x460d, 0x95, 0x23, 0x3a, 0x7d, 0xfe, 0xdb, 0xb3, 0x3c);
DEFINE_GUID(WiaImgFmt_RAWPAT,   0x7760507c, 0x5064, 0x400c, 0x9a, 0x17, 0x57, 0x56, 0x24, 0xd8, 0x82, 0x4b);
DEFINE_GUID(WiaImgFmt_XMLMIC,   0x2d164c61, 0xb9ae, 0x4b23, 0x89, 0x73, 0xc7, 0x06, 0x7e, 0x1f, 0xbd, 0x31);
DEFINE_GUID(WiaImgFmt_RAWMIC,   0x22c4f058, 0x0d88, 0x409c, 0xac, 0x1c, 0xee, 0xc1, 0x2b, 0x0e, 0xa6, 0x80);

//
// WIA event constants
//

DEFINE_GUID(WIA_EVENT_DEVICE_DISCONNECTED, 0x143e4e83, 0x6497, 0x11d2, 0xa2, 0x31, 0x00, 0xc0, 0x4f, 0xa3, 0x18, 0x09);
DEFINE_GUID(WIA_EVENT_DEVICE_CONNECTED,    0xa28bbade, 0x64b6, 0x11d2, 0xa2, 0x31, 0x00, 0xc0, 0x4f, 0xa3, 0x18, 0x09);
DEFINE_GUID(WIA_EVENT_ITEM_DELETED,        0x1d22a559, 0xe14f, 0x11d2, 0xb3, 0x26, 0x00, 0xc0, 0x4f, 0x68, 0xce, 0x61);
DEFINE_GUID(WIA_EVENT_ITEM_CREATED,        0x4c8f4ef5, 0xe14f, 0x11d2, 0xb3, 0x26, 0x00, 0xc0, 0x4f, 0x68, 0xce, 0x61);
DEFINE_GUID(WIA_EVENT_TREE_UPDATED,        0xc9859b91, 0x4ab2, 0x4cd6, 0xa1, 0xfc, 0x58, 0x2e, 0xec, 0x55, 0xe5, 0x85);
DEFINE_GUID(WIA_EVENT_VOLUME_INSERT,       0x9638bbfd, 0xd1bd, 0x11d2, 0xb3, 0x1f, 0x00, 0xc0, 0x4f, 0x68, 0xce, 0x61);
DEFINE_GUID(WIA_EVENT_SCAN_IMAGE,          0xa6c5a715, 0x8c6e, 0x11d2, 0x97, 0x7a, 0x00, 0x00, 0xf8, 0x7a, 0x92, 0x6f);
DEFINE_GUID(WIA_EVENT_SCAN_PRINT_IMAGE,    0xb441f425, 0x8c6e, 0x11d2, 0x97, 0x7a, 0x00, 0x00, 0xf8, 0x7a, 0x92, 0x6f);
DEFINE_GUID(WIA_EVENT_SCAN_FAX_IMAGE,      0xc00eb793, 0x8c6e, 0x11d2, 0x97, 0x7a, 0x00, 0x00, 0xf8, 0x7a, 0x92, 0x6f);
DEFINE_GUID(WIA_EVENT_SCAN_OCR_IMAGE,      0x9d095b89, 0x37d6, 0x4877, 0xaf, 0xed, 0x62, 0xa2, 0x97, 0xdc, 0x6d, 0xbe);
DEFINE_GUID(WIA_EVENT_SCAN_EMAIL_IMAGE,    0xc686dcee, 0x54f2, 0x419e, 0x9a, 0x27, 0x2f, 0xc7, 0xf2, 0xe9, 0x8f, 0x9e);
DEFINE_GUID(WIA_EVENT_SCAN_FILM_IMAGE,     0x9b2b662c, 0x6185, 0x438c, 0xb6, 0x8b, 0xe3, 0x9e, 0xe2, 0x5e, 0x71, 0xcb);
DEFINE_GUID(WIA_EVENT_SCAN_IMAGE2,         0xfc4767c1, 0xc8b3, 0x48a2, 0x9c, 0xfa, 0x2e, 0x90, 0xcb, 0x3d, 0x35, 0x90);
DEFINE_GUID(WIA_EVENT_SCAN_IMAGE3,         0x154e27be, 0xb617, 0x4653, 0xac, 0xc5, 0x0f, 0xd7, 0xbd, 0x4c, 0x65, 0xce);
DEFINE_GUID(WIA_EVENT_SCAN_IMAGE4,         0xa65b704a, 0x7f3c, 0x4447, 0xa7, 0x5d, 0x8a, 0x26, 0xdf, 0xca, 0x1f, 0xdf);
DEFINE_GUID(WIA_EVENT_STORAGE_CREATED,     0x353308b2, 0xfe73, 0x46c8, 0x89, 0x5e, 0xfa, 0x45, 0x51, 0xcc, 0xc8, 0x5a);
DEFINE_GUID(WIA_EVENT_STORAGE_DELETED,     0x5e41e75e, 0x9390, 0x44c5, 0x9a, 0x51, 0xe4, 0x70, 0x19, 0xe3, 0x90, 0xcf);
DEFINE_GUID(WIA_EVENT_STI_PROXY,           0xd711f81f, 0x1f0d, 0x422d, 0x86, 0x41, 0x92, 0x7d, 0x1b, 0x93, 0xe5, 0xe5);
DEFINE_GUID(WIA_EVENT_CANCEL_IO,           0xc860f7b8, 0x9ccd, 0x41ea, 0xbb, 0xbf, 0x4d, 0xd0, 0x9c, 0x5b, 0x17, 0x95);

//
// Power management event GUIDs, sent by the WIA service to drivers
//

DEFINE_GUID(WIA_EVENT_POWER_SUSPEND,       0xa0922ff9, 0xc3b4, 0x411c, 0x9e, 0x29, 0x03, 0xa6, 0x69, 0x93, 0xd2, 0xbe);
DEFINE_GUID(WIA_EVENT_POWER_RESUME,        0x618f153e, 0xf686, 0x4350, 0x96, 0x34, 0x41, 0x15, 0xa3, 0x04, 0x83, 0x0c);

//
// No action handler and prompt handler
//

DEFINE_GUID(WIA_EVENT_HANDLER_NO_ACTION,   0xe0372b7d, 0xe115, 0x4525, 0xbc, 0x55, 0xb6, 0x29, 0xe6, 0x8c, 0x74, 0x5a);
DEFINE_GUID(WIA_EVENT_HANDLER_PROMPT,      0x5f4baad0, 0x4d59, 0x4fcd, 0xb2, 0x13, 0x78, 0x3c, 0xe7, 0xa9, 0x2f, 0x22);

//
// Device status change events (actionable)
//

DEFINE_GUID(WIA_EVENT_DEVICE_NOT_READY,    0xd8962d7e, 0xe4dc, 0x4b4d, 0xba, 0x29, 0x66, 0x8a, 0x87, 0xf4, 0x2e, 0x6f);
DEFINE_GUID(WIA_EVENT_DEVICE_READY,        0x7523ec6c, 0x988b, 0x419e, 0x9a, 0x0a, 0x42, 0x5a, 0xc3, 0x1b, 0x37, 0xdc);
DEFINE_GUID(WIA_EVENT_FLATBED_LID_OPEN,    0xba0a0623, 0x437d, 0x4f03, 0xa9, 0x7d, 0x77, 0x93, 0xb1, 0x23, 0x11, 0x3c);
DEFINE_GUID(WIA_EVENT_FLATBED_LID_CLOSED,  0xf879af0f, 0x9b29, 0x4283, 0xad, 0x95, 0xd4, 0x12, 0x16, 0x4d, 0x39, 0xa9);
DEFINE_GUID(WIA_EVENT_FEEDER_LOADED,       0xcc8d701e, 0x9aba, 0x481d, 0xbf, 0x74, 0x78, 0xf7, 0x63, 0xdc, 0x34, 0x2a);
DEFINE_GUID(WIA_EVENT_FEEDER_EMPTIED,      0xe70b4b82, 0x6dda, 0x46bb, 0x8f, 0xf9, 0x53, 0xce, 0xb1, 0xa0, 0x3e, 0x35);
DEFINE_GUID(WIA_EVENT_COVER_OPEN,          0x19a12136, 0xfa1c, 0x4f66, 0x90, 0x0f, 0x8f, 0x91, 0x4e, 0xc7, 0x4e, 0xc9);
DEFINE_GUID(WIA_EVENT_COVER_CLOSED,        0x6714a1e6, 0xe285, 0x468c, 0x9b, 0x8c, 0xda, 0x7d, 0xc4, 0xcb, 0xaa, 0x05);

//
// WIA command constants
//

DEFINE_GUID(WIA_CMD_SYNCHRONIZE,          0x9b26b7b2, 0xacad, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c);
DEFINE_GUID(WIA_CMD_TAKE_PICTURE,         0xaf933cac, 0xacad, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c);
DEFINE_GUID(WIA_CMD_DELETE_ALL_ITEMS,     0xe208c170, 0xacad, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c);
DEFINE_GUID(WIA_CMD_CHANGE_DOCUMENT,      0x04e725b0, 0xacae, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c);
DEFINE_GUID(WIA_CMD_UNLOAD_DOCUMENT,      0x1f3b3d8e, 0xacae, 0x11d2, 0xa0, 0x93, 0x00, 0xc0, 0x4f, 0x72, 0xdc, 0x3c);
DEFINE_GUID(WIA_CMD_DIAGNOSTIC,           0x10ff52f5, 0xde04, 0x4cf0, 0xa5, 0xad, 0x69, 0x1f, 0x8d, 0xce, 0x01, 0x41);
DEFINE_GUID(WIA_CMD_FORMAT,               0xc3a693aa, 0xf788, 0x4d34, 0xa5, 0xb0, 0xbe, 0x71, 0x90, 0x75, 0x9a, 0x24);

//
// WIA command constants used for debugging only
//

DEFINE_GUID(WIA_CMD_DELETE_DEVICE_TREE,   0x73815942, 0xdbea, 0x11d2, 0x84, 0x16, 0x00, 0xc0, 0x4f, 0xa3, 0x61, 0x45);
DEFINE_GUID(WIA_CMD_BUILD_DEVICE_TREE,    0x9cba5ce0, 0xdbea, 0x11d2, 0x84, 0x16, 0x00, 0xc0, 0x4f, 0xa3, 0x61, 0x45);

//
// WIA command constants for feeder motor control
//

#if (_WIN32_WINNT >= 0x0600)
DEFINE_GUID(WIA_CMD_START_FEEDER,         0x5a9df6c9, 0x5f2d, 0x4a39, 0x9d, 0x6c, 0x00, 0x45, 0x6d, 0x04, 0x7f, 0x00);
DEFINE_GUID(WIA_CMD_STOP_FEEDER,          0xd847b06d, 0x3905, 0x459c, 0x95, 0x09, 0x9b, 0x29, 0xcd, 0xb6, 0x91, 0xe7);
DEFINE_GUID(WIA_CMD_PAUSE_FEEDER,         0x50985e4d, 0xa5b2, 0x4b71, 0x9c, 0x95, 0x6d, 0x7d, 0x7c, 0x46, 0x9a, 0x43);
#endif //#if (_WIN32_WINNT >= 0x0600)

#define BASE_VAL_WIA_ERROR   0x00000000
#define BASE_VAL_WIA_SUCCESS 0x00000000

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#define WIA_ERROR_GENERAL_ERROR                     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 1)
#define WIA_ERROR_PAPER_JAM                         MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 2)
#define WIA_ERROR_PAPER_EMPTY                       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 3)
#define WIA_ERROR_PAPER_PROBLEM                     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 4)
#define WIA_ERROR_OFFLINE                           MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 5)
#define WIA_ERROR_BUSY                              MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 6)
#define WIA_ERROR_WARMING_UP                        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 7)
#define WIA_ERROR_USER_INTERVENTION                 MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 8)
#define WIA_ERROR_ITEM_DELETED                      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 9)
#define WIA_ERROR_DEVICE_COMMUNICATION              MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 10)
#define WIA_ERROR_INVALID_COMMAND                   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 11)
#define WIA_ERROR_INCORRECT_HARDWARE_SETTING        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 12)
#define WIA_ERROR_DEVICE_LOCKED                     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 13)
#define WIA_ERROR_EXCEPTION_IN_DRIVER               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 14)
#define WIA_ERROR_INVALID_DRIVER_RESPONSE           MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 15)
#define WIA_ERROR_COVER_OPEN                        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 16)
#define WIA_ERROR_LAMP_OFF                          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 17)
#define WIA_ERROR_DESTINATION                       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 18)
#define WIA_ERROR_NETWORK_RESERVATION_FAILED        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 19)
#if (_WIN32_WINNT >= 0x0600)
#define WIA_ERROR_MULTI_FEED                        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 20)
#define WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 21)
#endif //#if (_WIN32_WINNT >= 0x0600)
#define WIA_STATUS_END_OF_MEDIA                     MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 1)

//
// Definitions for errors and status codes passed to IWiaDataTransfer::BandedDataCallback as the lReason parameter.
// These codes are in addition to the errors defined above; in some cases the SEVERITY_SUCCESS version of
// an error is meant to replace the SEVERITY_ERROR version listed above.
//

#define WIA_STATUS_WARMING_UP                       MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 2)
#define WIA_STATUS_CALIBRATING                      MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 3)
#define WIA_STATUS_RESERVING_NETWORK_DEVICE         MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 6)
#define WIA_STATUS_NETWORK_DEVICE_RESERVED          MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 7)
#define WIA_STATUS_CLEAR                            MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 8)
#define WIA_STATUS_SKIP_ITEM                        MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 9)
#define WIA_STATUS_NOT_HANDLED                      MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 10)

//
// The value is returned by Scansetting.dll when the user chooses to change the scanner in scandialog
//

#define WIA_S_CHANGE_DEVICE                         MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_WIA, 11)

//
// SelectDeviceDlg and SelectDeviceDlgID status code when there are no devices available
//

#define WIA_S_NO_DEVICE_AVAILABLE                   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_WIA, 21)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// SelectDeviceDlg and GetImageDlg flag constants
//

#define WIA_SELECT_DEVICE_NODEFAULT          0x00000001

//
// DeviceDlg and GetImageDlg flags constants
//

#define WIA_DEVICE_DIALOG_SINGLE_IMAGE       0x00000002  // Only allow one image to be selected
#define WIA_DEVICE_DIALOG_USE_COMMON_UI      0x00000004  // Give preference to the system-provided UI, if available

//
// RegisterEventCallbackInterface and RegisterEventCallbackCLSID flag constants
//

#define  WIA_REGISTER_EVENT_CALLBACK         0x00000001
#define  WIA_UNREGISTER_EVENT_CALLBACK       0x00000002
#define  WIA_SET_DEFAULT_HANDLER             0x00000004

//
// WIA event type constants
//

#define  WIA_NOTIFICATION_EVENT              0x00000001
#define  WIA_ACTION_EVENT                    0x00000002

//
// Additional WIA raw format constants
//

#define  WIA_LINE_ORDER_TOP_TO_BOTTOM        0x00000001
#define  WIA_LINE_ORDER_BOTTOM_TO_TOP        0x00000002

//
// WIA event persistent handler flag constants
//

#define  WIA_IS_DEFAULT_HANDLER              0x00000001

//
// WIA connected and disconnected event description strings
//

#define WIA_EVENT_DEVICE_DISCONNECTED_STR L"Device Disconnected"
#define WIA_EVENT_DEVICE_CONNECTED_STR    L"Device Connected"

//
// WIA event and command icon resource identifier constants
//
// Events   : -1000 to -1499 (Standard), -1500 to -1999 (Custom)
// Commands : -2000 to -2499 (Standard), -2500 to -2999 (Custom)
//

#define WIA_ICON_DEVICE_DISCONNECTED (L"sti.dll,-1001")
#define WIA_ICON_DEVICE_CONNECTED    (L"sti.dll,-1001")
#define WIA_ICON_ITEM_DELETED        (L"sti.dll,-1001")
#define WIA_ICON_ITEM_CREATED        (L"sti.dll,-1001")
#define WIA_ICON_TREE_UPDATED        (L"sti.dll,-1001")
#define WIA_ICON_VOLUME_INSERT       (L"sti.dll,-1001")
#define WIA_ICON_SCAN_BUTTON_PRESS   (L"sti.dll,-1001")
#define WIA_ICON_DEVICE_NOT_READY    (L"sti.dll,-1001")
#define WIA_ICON_DEVICE_READY        (L"sti.dll,-1001")
#define WIA_ICON_FLATBED_LID_OPEN    (L"sti.dll,-1001")
#define WIA_ICON_FLATBED_LID_CLOSED  (L"sti.dll,-1001")
#define WIA_ICON_FEEDER_LOADED       (L"sti.dll,-1001")
#define WIA_ICON_FEEDER_EMPTIED      (L"sti.dll,-1001")
#define WIA_ICON_COVER_OPEN          (L"sti.dll,-1001")
#define WIA_ICON_COVER_CLOSED        (L"sti.dll,-1001")
#define WIA_ICON_START_FEEDER        (L"sti.dll,-1001")
#define WIA_ICON_STOP_FEEDER         (L"sti.dll,-1001")
#define WIA_ICON_SYNCHRONIZE         (L"sti.dll,-2000")
#define WIA_ICON_TAKE_PICTURE        (L"sti.dll,-2001")
#define WIA_ICON_DELETE_ALL_ITEMS    (L"sti.dll,-2002")
#define WIA_ICON_CHANGE_DOCUMENT     (L"sti.dll,-2003")
#define WIA_ICON_UNLOAD_DOCUMENT     (L"sti.dll,-2004")
#define WIA_ICON_DELETE_DEVICE_TREE  (L"sti.dll,-2005")
#define WIA_ICON_BUILD_DEVICE_TREE   (L"sti.dll,-2006")

//
// WIA TYMED constants
//

#define TYMED_CALLBACK                     128
#define TYMED_MULTIPAGE_FILE               256
#define TYMED_MULTIPAGE_CALLBACK           512

//
// IWiaDataCallback and IWiaMiniDrvCallBack message ID constants
//

#define IT_MSG_DATA_HEADER              0x0001
#define IT_MSG_DATA                     0x0002
#define IT_MSG_STATUS                   0x0003
#define IT_MSG_TERMINATION              0x0004
#define IT_MSG_NEW_PAGE                 0x0005
#define IT_MSG_FILE_PREVIEW_DATA        0x0006
#define IT_MSG_FILE_PREVIEW_DATA_HEADER 0x0007

//
// IWiaDataCallback and IWiaMiniDrvCallBack status flag constants
//

#define IT_STATUS_TRANSFER_FROM_DEVICE    0x0001
#define IT_STATUS_PROCESSING_DATA         0x0002
#define IT_STATUS_TRANSFER_TO_CLIENT      0x0004
#define IT_STATUS_MASK                    0x0007 // any status value that doesn't
                                                 // fit the mask is an HRESULT
//
// IWiaTransfer flags
//

#define WIA_TRANSFER_ACQUIRE_CHILDREN     0x0001

//
// IWiaTransferCallback Message types
//

#define WIA_TRANSFER_MSG_STATUS           0x00001
#define WIA_TRANSFER_MSG_END_OF_STREAM    0x00002
#define WIA_TRANSFER_MSG_END_OF_TRANSFER  0x00003
#define WIA_TRANSFER_MSG_DEVICE_STATUS    0x00005
#define WIA_TRANSFER_MSG_NEW_PAGE         0x00006

//
// IWiaEventCallback code constants
//

#define WIA_MAJOR_EVENT_DEVICE_CONNECT    0x01
#define WIA_MAJOR_EVENT_DEVICE_DISCONNECT 0x02
#define WIA_MAJOR_EVENT_PICTURE_TAKEN     0x03
#define WIA_MAJOR_EVENT_PICTURE_DELETED   0x04

//
// WIA device connection status constants
//

#define  WIA_DEVICE_NOT_CONNECTED         0
#define  WIA_DEVICE_CONNECTED             1

//
// EnumDeviceCapabilities and drvGetCapabilities flags
//

#define WIA_DEVICE_COMMANDS               1
#define WIA_DEVICE_EVENTS                 2

//
// EnumDeviceInfo Flags
//

#define WIA_DEVINFO_ENUM_ALL              0x0000000F
#define WIA_DEVINFO_ENUM_LOCAL            0x00000010


//
// WIA item type constants
//

#define WiaItemTypeFree                   0x00000000
#define WiaItemTypeImage                  0x00000001
#define WiaItemTypeFile                   0x00000002
#define WiaItemTypeFolder                 0x00000004
#define WiaItemTypeRoot                   0x00000008
#define WiaItemTypeAnalyze                0x00000010
#define WiaItemTypeAudio                  0x00000020
#define WiaItemTypeDevice                 0x00000040
#define WiaItemTypeDeleted                0x00000080
#define WiaItemTypeDisconnected           0x00000100
#define WiaItemTypeHPanorama              0x00000200
#define WiaItemTypeVPanorama              0x00000400
#define WiaItemTypeBurst                  0x00000800
#define WiaItemTypeStorage                0x00001000
#define WiaItemTypeTransfer               0x00002000
#define WiaItemTypeGenerated              0x00004000
#define WiaItemTypeHasAttachments         0x00008000
#define WiaItemTypeVideo                  0x00010000
#define WiaItemTypeRemoved                0x80000000
//
// 0x00020000 is reserved for the TWAIN-WIA Compatiblity Layer
//
#if (_WIN32_WINNT >= 0x0600)
#define WiaItemTypeDocument               0x00040000
#define WiaItemTypeProgrammableDataSource 0x00080000
#define WiaItemTypeMask                   0x800FFFFF
#else
#define WiaItemTypeMask                   0x8003FFFF
#endif

//
// Maximum device specific item context
//

#define WIA_MAX_CTX_SIZE                  0x01000000

//
// WIA property access flag constants
//

#define WIA_PROP_READ            0x01
#define WIA_PROP_WRITE           0x02
#define WIA_PROP_RW              (WIA_PROP_READ | WIA_PROP_WRITE)
#define WIA_PROP_SYNC_REQUIRED   0x04

#define WIA_PROP_NONE            0x08
#define WIA_PROP_RANGE           0x10
#define WIA_PROP_LIST            0x20
#define WIA_PROP_FLAG            0x40

#define WIA_PROP_CACHEABLE       0x10000

//
// IWiaItem2 CreateChildItem flag constants
//

#define COPY_PARENT_PROPERTY_VALUES       0x40000000

//
// WIA item access flag constants
//

#define WIA_ITEM_CAN_BE_DELETED  0x80
#define WIA_ITEM_READ            WIA_PROP_READ
#define WIA_ITEM_WRITE           WIA_PROP_WRITE
#define WIA_ITEM_RD              (WIA_ITEM_READ | WIA_ITEM_CAN_BE_DELETED)
#define WIA_ITEM_RWD             (WIA_ITEM_READ | WIA_ITEM_WRITE | WIA_ITEM_CAN_BE_DELETED)

//
// WIA property container constants
//

#define  WIA_RANGE_MIN                          0
#define  WIA_RANGE_NOM                          1
#define  WIA_RANGE_MAX                          2
#define  WIA_RANGE_STEP                         3
#define  WIA_RANGE_NUM_ELEMS                    4

#define  WIA_LIST_COUNT                         0
#define  WIA_LIST_NOM                           1
#define  WIA_LIST_VALUES                        2
#define  WIA_LIST_NUM_ELEMS                     2

#define  WIA_FLAG_NOM                           0
#define  WIA_FLAG_VALUES                        1
#define  WIA_FLAG_NUM_ELEMS                     2

//
// WIA property LIST container MACROS
//

#define WIA_PROP_LIST_COUNT(ppv) (((PROPVARIANT*)ppv)->cal.cElems - WIA_LIST_VALUES)

#define WIA_PROP_LIST_VALUE(ppv, index)                              \\
     ((index > ((PROPVARIANT*) ppv)->cal.cElems - WIA_LIST_VALUES) || (index < -WIA_LIST_NOM)) ?\\
     NULL :                                                          \\
     (((PROPVARIANT*) ppv)->vt == VT_UI1) ?                          \\
     ((PROPVARIANT*) ppv)->caub.pElems[WIA_LIST_VALUES + index] :    \\
     (((PROPVARIANT*) ppv)->vt == VT_UI2) ?                          \\
     ((PROPVARIANT*) ppv)->caui.pElems[WIA_LIST_VALUES + index] :    \\
     (((PROPVARIANT*) ppv)->vt == VT_UI4) ?                          \\
     ((PROPVARIANT*) ppv)->caul.pElems[WIA_LIST_VALUES + index] :    \\
     (((PROPVARIANT*) ppv)->vt == VT_I2) ?                           \\
     ((PROPVARIANT*) ppv)->cai.pElems[WIA_LIST_VALUES + index] :     \\
     (((PROPVARIANT*) ppv)->vt == VT_I4) ?                           \\
     ((PROPVARIANT*) ppv)->cal.pElems[WIA_LIST_VALUES + index] :     \\
     (((PROPVARIANT*) ppv)->vt == VT_R4) ?                           \\
     ((PROPVARIANT*) ppv)->caflt.pElems[WIA_LIST_VALUES + index] :   \\
     (((PROPVARIANT*) ppv)->vt == VT_R8) ?                           \\
     ((PROPVARIANT*) ppv)->cadbl.pElems[WIA_LIST_VALUES + index] :   \\
     (((PROPVARIANT*) ppv)->vt == VT_BSTR) ?                         \\
     (LONG)(((PROPVARIANT*) ppv)->cabstr.pElems[WIA_LIST_VALUES + index]) : \\
     NULL

//
// Microsoft defined WIA property offset constants
//

#define WIA_DIP_FIRST                        2
#define WIA_IPA_FIRST                     4098
#define WIA_DPF_FIRST                     3330
#define WIA_IPS_FIRST                     6146
#define WIA_DPS_FIRST                     3074
#define WIA_IPC_FIRST                     5122
#define WIA_NUM_IPC                          9
#define WIA_RESERVED_FOR_NEW_PROPS        1024

//
// WIA_DPC_WHITE_BALANCE constants
//

#define WHITEBALANCE_MANUAL            1
#define WHITEBALANCE_AUTO              2
#define WHITEBALANCE_ONEPUSH_AUTO      3
#define WHITEBALANCE_DAYLIGHT          4
#define WHITEBALANCE_FLORESCENT        5
#define WHITEBALANCE_TUNGSTEN          6
#define WHITEBALANCE_FLASH             7

//
// WIA_DPC_FOCUS_MODE constants
//

#define FOCUSMODE_MANUAL               1
#define FOCUSMODE_AUTO                 2
#define FOCUSMODE_MACROAUTO            3

//
// WIA_DPC_EXPOSURE_METERING_MODE constants
//

#define EXPOSUREMETERING_AVERAGE       1
#define EXPOSUREMETERING_CENTERWEIGHT  2
#define EXPOSUREMETERING_MULTISPOT     3
#define EXPOSUREMETERING_CENTERSPOT    4

//
// WIA_DPC_FLASH_MODE constants
//

#define FLASHMODE_AUTO                 1
#define FLASHMODE_OFF                  2
#define FLASHMODE_FILL                 3
#define FLASHMODE_REDEYE_AUTO          4
#define FLASHMODE_REDEYE_FILL          5
#define FLASHMODE_EXTERNALSYNC         6

//
// WIA_DPC_EXPOSURE_MODE constants
//

#define EXPOSUREMODE_MANUAL            1
#define EXPOSUREMODE_AUTO              2
#define EXPOSUREMODE_APERTURE_PRIORITY 3
#define EXPOSUREMODE_SHUTTER_PRIORITY  4
#define EXPOSUREMODE_PROGRAM_CREATIVE  5
#define EXPOSUREMODE_PROGRAM_ACTION    6
#define EXPOSUREMODE_PORTRAIT          7

//
// WIA_DPC_CAPTURE_MODE constants
//

#define CAPTUREMODE_NORMAL             1
#define CAPTUREMODE_BURST              2
#define CAPTUREMODE_TIMELAPSE          3

//
// WIA_DPC_EFFECT_MODE constants
//

#define EFFECTMODE_STANDARD            1
#define EFFECTMODE_BW                  2
#define EFFECTMODE_SEPIA               3

//
// WIA_DPC_FOCUS_METERING_MODE constants
//

#define FOCUSMETERING_CENTERSPOT       1
#define FOCUSMETERING_MULTISPOT        2

//
// WIA_DPC_POWER_MODE constants
//

#define POWERMODE_LINE                 1
#define POWERMODE_BATTERY              2

//
// WIA_DPS_SHEET_FEEDER_REGISTRATION and
// WIA_DPS_HORIZONTAL_BED_REGISTRATION constants
//

#define  LEFT_JUSTIFIED                0
#define  CENTERED                      1
#define  RIGHT_JUSTIFIED               2

//
// WIA_DPS_VERTICAL_BED_REGISTRATION constants
//

#define  TOP_JUSTIFIED                 0
#define  CENTERED                      1
#define  BOTTOM_JUSTIFIED              2

//
// WIA_IPS_ORIENTATION and WIA_IPS_ROTATION constants
//

#define  PORTRAIT                      0
#define  LANSCAPE                      1
#if (_WIN32_WINNT >= 0x0600)
#define  LANDSCAPE                     LANSCAPE
#endif
#define  ROT180                        2
#define  ROT270                        3


//
// WIA_IPS_MIRROR flags
//

#define  MIRRORED                      0x01

//
// WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES flags
//

#define  FEED                          0x01
#define  FLAT                          0x02
#define  DUP                           0x04
#define  DETECT_FLAT                   0x08
#define  DETECT_SCAN                   0x10
#define  DETECT_FEED                   0x20
#define  DETECT_DUP                    0x40
#define  DETECT_FEED_AVAIL             0x80
#define  DETECT_DUP_AVAIL              0x100
#if (_WIN32_WINNT >= 0x0600)
#define  FILM_TPA                      0x200
#define  DETECT_FILM_TPA               0x400
#define  STOR                          0x800
#define  DETECT_STOR                   0x1000
#define  ADVANCED_DUP                  0x2000
#define  AUTO_SOURCE                   0x8000
#define  IMPRINTER                     0x10000
#define  ENDORSER                      0x20000
#define  BARCODE_READER                0x40000
#define  PATCH_CODE_READER             0x80000
#define  MICR_READER                   0x100000
#endif

//
// WIA_DPS_DOCUMENT_HANDLING_STATUS flags
//

#define  FEED_READY                    0x01
#define  FLAT_READY                    0x02
#define  DUP_READY                     0x04
#define  FLAT_COVER_UP                 0x08
#define  PATH_COVER_UP                 0x10
#define  PAPER_JAM                     0x20
#if (_WIN32_WINNT >= 0x0600)
#define  FILM_TPA_READY                0x40
#define  STORAGE_READY                 0x80
#define  STORAGE_FULL                  0x100
#define  MULTIPLE_FEED                 0x200
#define  DEVICE_ATTENTION              0x400
#define  LAMP_ERR                      0x800
#define  IMPRINTER_READY               0x1000
#define  ENDORSER_READY                0x2000
#define  BARCODE_READER_READY          0x4000
#define  PATCH_CODE_READER_READY       0x8000
#define  MICR_READER_READY             0x10000
#endif

//
// WIA_DPS_DOCUMENT_HANDLING_SELECT flags
//

#define  FEEDER                        0x001
#define  FLATBED                       0x002
#define  DUPLEX                        0x004
#define  FRONT_FIRST                   0x008
#define  BACK_FIRST                    0x010
#define  FRONT_ONLY                    0x020
#define  BACK_ONLY                     0x040
#define  NEXT_PAGE                     0x080
#define  PREFEED                       0x100
#define  AUTO_ADVANCE                  0x200
#if (_WIN32_WINNT >= 0x0600)
//
// New WIA_IPS_DOCUMENT_HANDLING_SELECT flag
//
#define  ADVANCED_DUPLEX               0x400
#endif //#if (_WIN32_WINNT >= 0x0600)

//
// WIA_DPS_TRANSPARENCY / WIA_DPS_TRANSPARENCY_STATUS flags
//

#define  LIGHT_SOURCE_PRESENT_DETECT   0x01
#define  LIGHT_SOURCE_PRESENT          0x02
#define  LIGHT_SOURCE_DETECT_READY     0x04
#define  LIGHT_SOURCE_READY            0x08

//
// WIA_DPS_TRANSPARENCY_CAPABILITIES
//

#define TRANSPARENCY_DYNAMIC_FRAME_SUPPORT 0x01
#define TRANSPARENCY_STATIC_FRAME_SUPPORT  0x02

//
// WIA_DPS_TRANSPARENCY_SELECT flags
//

#define  LIGHT_SOURCE_SELECT           0x001 // currently not used
#define  LIGHT_SOURCE_POSITIVE         0x002
#define  LIGHT_SOURCE_NEGATIVE         0x004

//
// WIA_DPS_SCAN_AHEAD_PAGES constants
//
// WIA_DPS_SCAN_AHEAD_PAGES is superseded in WIA 2.0 by WIA_IPS_SCAN_AHEAD
//

#define  WIA_SCAN_AHEAD_ALL            0

//
// WIA_DPS_PAGES constants
//

#define  ALL_PAGES                     0

//
// WIA_DPS_PREVIEW constants
//

#define WIA_FINAL_SCAN                 0
#define WIA_PREVIEW_SCAN               1

//
// WIA_DPS_SHOW_PREVIEW_CONTROL constants
//

#define WIA_SHOW_PREVIEW_CONTROL       0
#define WIA_DONT_SHOW_PREVIEW_CONTROL  1

//
// Predefined strings for WIA_DPS_ENDORSER_STRING
//
// WIA_DPS_ENDORSER_STRING is superseded in WIA 2.0 by WIA_IPS_PRINTER_ENDORSER_STRING and these
// constant values are replaced by the WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS constants
//

#define WIA_ENDORSER_TOK_DATE          L"$DATE$"
#define WIA_ENDORSER_TOK_TIME          L"$TIME$"
#define WIA_ENDORSER_TOK_PAGE_COUNT    L"$PAGE_COUNT$"
#define WIA_ENDORSER_TOK_DAY           L"$DAY$"
#define WIA_ENDORSER_TOK_MONTH         L"$MONTH$"
#define WIA_ENDORSER_TOK_YEAR          L"$YEAR$"

//
// WIA_DPS_PAGE_SIZE/WIA_IPS_PAGE_SIZE constants
// Dimensions are defined as (WIDTH x HEIGHT) in 1/1000ths of an inch
//

#define WIA_PAGE_A4            0 //  8267 x 11692
#define WIA_PAGE_LETTER        1 //  8500 x 11000
#define WIA_PAGE_CUSTOM        2 // (current extent settings)

#define WIA_PAGE_USLEGAL       3 //  8500 x 14000
#define WIA_PAGE_USLETTER      WIA_PAGE_LETTER
#define WIA_PAGE_USLEDGER      4 // 11000 x 17000
#define WIA_PAGE_USSTATEMENT   5 //  5500 x  8500
#define WIA_PAGE_BUSINESSCARD  6 //  3543 x  2165

//
// ISO A page size constants
//

#define WIA_PAGE_ISO_A0        7 // 33110 x 46811
#define WIA_PAGE_ISO_A1        8 // 23385 x 33110
#define WIA_PAGE_ISO_A2        9 // 16535 x 23385
#define WIA_PAGE_ISO_A3       10 // 11692 x 16535
#define WIA_PAGE_ISO_A4       WIA_PAGE_A4
#define WIA_PAGE_ISO_A5       11 //  5826 x  8267
#define WIA_PAGE_ISO_A6       12 //  4133 x  5826
#define WIA_PAGE_ISO_A7       13 //  2913 x  4133
#define WIA_PAGE_ISO_A8       14 //  2047 x  2913
#define WIA_PAGE_ISO_A9       15 //  1456 x  2047
#define WIA_PAGE_ISO_A10      16 //  1023 x  1456

//
// ISO B page size constants
//

#define WIA_PAGE_ISO_B0       17 //  39370 x 55669
#define WIA_PAGE_ISO_B1       18 //  27834 x 39370
#define WIA_PAGE_ISO_B2       19 //  19685 x 27834
#define WIA_PAGE_ISO_B3       20 //  13897 x 19685
#define WIA_PAGE_ISO_B4       21 //   9842 x 13897
#define WIA_PAGE_ISO_B5       22 //   6929 x  9842
#define WIA_PAGE_ISO_B6       23 //   4921 x  6929
#define WIA_PAGE_ISO_B7       24 //   3464 x  4921
#define WIA_PAGE_ISO_B8       25 //   2440 x  3464
#define WIA_PAGE_ISO_B9       26 //   1732 x  2440
#define WIA_PAGE_ISO_B10      27 //   1220 x  1732

//
// ISO C page size constants
//

#define WIA_PAGE_ISO_C0       28 //  36102 x 51062
#define WIA_PAGE_ISO_C1       29 //  25511 x 36102
#define WIA_PAGE_ISO_C2       30 //  18031 x 25511
#define WIA_PAGE_ISO_C3       31 //  12755 x 18031
#define WIA_PAGE_ISO_C4       32 //   9015 x 12755 (unfolded)
#define WIA_PAGE_ISO_C5       33 //   6377 x  9015 (folded once)
#define WIA_PAGE_ISO_C6       34 //   4488 x  6377 (folded twice)
#define WIA_PAGE_ISO_C7       35 //   3188 x  4488
#define WIA_PAGE_ISO_C8       36 //   2244 x  3188
#define WIA_PAGE_ISO_C9       37 //   1574 x  2244
#define WIA_PAGE_ISO_C10      38 //   1102 x  1574

//
// JIS B page size constants
//

#define WIA_PAGE_JIS_B0       39 //  40551 x 57322
#define WIA_PAGE_JIS_B1       40 //  28661 x 40551
#define WIA_PAGE_JIS_B2       41 //  20275 x 28661
#define WIA_PAGE_JIS_B3       42 //  14330 x 20275
#define WIA_PAGE_JIS_B4       43 //  10118 x 14330
#define WIA_PAGE_JIS_B5       44 //   7165 x 10118
#define WIA_PAGE_JIS_B6       45 //   5039 x  7165
#define WIA_PAGE_JIS_B7       46 //   3582 x  5039
#define WIA_PAGE_JIS_B8       47 //   2519 x  3582
#define WIA_PAGE_JIS_B9       48 //   1771 x  2519
#define WIA_PAGE_JIS_B10      49 //   1259 x  1771

//
// JIS A page size constants
//

#define WIA_PAGE_JIS_2A       50 //  46811 x 66220
#define WIA_PAGE_JIS_4A       51 //  66220 x  93622

//
// DIN B page size constants
//

#define WIA_PAGE_DIN_2B       52 //  55669 x 78740
#define WIA_PAGE_DIN_4B       53 //  78740 x 111338

#if (_WIN32_WINNT >= 0x0600)
//
// Additional WIA_IPS_PAGE_SIZE constants:
//
#define WIA_PAGE_AUTO         100
#define WIA_PAGE_CUSTOM_BASE  0x8000
#endif //#if (_WIN32_WINNT >= 0x0600)


//
// WIA_IPA_COMPRESSION constants
//

#define WIA_COMPRESSION_NONE           0
#define WIA_COMPRESSION_BI_RLE4        1
#define WIA_COMPRESSION_BI_RLE8        2
#define WIA_COMPRESSION_G3             3
#define WIA_COMPRESSION_G4             4
#define WIA_COMPRESSION_JPEG           5
#if (_WIN32_WINNT >= 0x0600)
#define WIA_COMPRESSION_JBIG           6
#define WIA_COMPRESSION_JPEG2K         7
#define WIA_COMPRESSION_PNG            8
#define WIA_COMPRESSION_AUTO         100
#endif //#if (_WIN32_WINNT >= 0x0600)

//
// WIA_IPA_PLANAR constants
//

#define WIA_PACKED_PIXEL               0
#define WIA_PLANAR                     1

//
// WIA_IPA_DATATYPE constants
//

#define WIA_DATA_THRESHOLD             0
#define WIA_DATA_DITHER                1
#define WIA_DATA_GRAYSCALE             2
#define WIA_DATA_COLOR                 3
#define WIA_DATA_COLOR_THRESHOLD       4
#define WIA_DATA_COLOR_DITHER          5
#if (_WIN32_WINNT >= 0x0600)
#define WIA_DATA_RAW_RGB               6
#define WIA_DATA_RAW_BGR               7
#define WIA_DATA_RAW_YUV               8
#define WIA_DATA_RAW_YUVK              9
#define WIA_DATA_RAW_CMY              10
#define WIA_DATA_RAW_CMYK             11
#define WIA_DATA_AUTO                100
#endif //#if (_WIN32_WINNT >= 0x0600)

//
// WIA_IPA_DEPTH constant
//
#if (_WIN32_WINNT >= 0x0600)
#define WIA_DEPTH_AUTO                 0
#endif //#if (_WIN32_WINNT >= 0x0600)

//
// WIA_IPS_PHOTOMETRIC_INTERP constants
//

#define WIA_PHOTO_WHITE_1              0 // white is 1, black is 0
#define WIA_PHOTO_WHITE_0              1 // white is 0, black is 1

//
// WIA_IPA_SUPPRESS_PROPERTY_PAGE flags
//

#define WIA_PROPPAGE_SCANNER_ITEM_GENERAL 0x00000001
#define WIA_PROPPAGE_CAMERA_ITEM_GENERAL  0x00000002
#define WIA_PROPPAGE_DEVICE_GENERAL       0x00000004

//
// WIA_IPS_CUR_INTENT flags
//
#define WIA_INTENT_NONE                   0x00000000
#define WIA_INTENT_IMAGE_TYPE_COLOR       0x00000001
#define WIA_INTENT_IMAGE_TYPE_GRAYSCALE   0x00000002
#define WIA_INTENT_IMAGE_TYPE_TEXT        0x00000004
#define WIA_INTENT_IMAGE_TYPE_MASK        0x0000000F
#define WIA_INTENT_MINIMIZE_SIZE          0x00010000
#define WIA_INTENT_MAXIMIZE_QUALITY       0x00020000
#define WIA_INTENT_BEST_PREVIEW           0x00040000
#define WIA_INTENT_SIZE_MASK              0x000F0000

//
// Global WIA device information property arrays
//

#define WIA_NUM_DIP 16

#ifdef WIA_DECLARE_DEVINFO_PROP_ARRAY

const PROPSPEC g_psDeviceInfo[WIA_NUM_DIP] =
{
    {PRSPEC_PROPID, WIA_DIP_DEV_ID},
    {PRSPEC_PROPID, WIA_DIP_VEND_DESC},
    {PRSPEC_PROPID, WIA_DIP_DEV_DESC},
    {PRSPEC_PROPID, WIA_DIP_DEV_TYPE},
    {PRSPEC_PROPID, WIA_DIP_PORT_NAME},
    {PRSPEC_PROPID, WIA_DIP_DEV_NAME},
    {PRSPEC_PROPID, WIA_DIP_SERVER_NAME},
    {PRSPEC_PROPID, WIA_DIP_REMOTE_DEV_ID},
    {PRSPEC_PROPID, WIA_DIP_UI_CLSID},
    {PRSPEC_PROPID, WIA_DIP_HW_CONFIG},
    {PRSPEC_PROPID, WIA_DIP_BAUDRATE},
    {PRSPEC_PROPID, WIA_DIP_STI_GEN_CAPABILITIES},
    {PRSPEC_PROPID, WIA_DIP_WIA_VERSION},
    {PRSPEC_PROPID, WIA_DIP_DRIVER_VERSION},
    {PRSPEC_PROPID, WIA_DIP_PNP_ID},
    {PRSPEC_PROPID, WIA_DIP_STI_DRIVER_VERSION},
};

const PROPID g_piDeviceInfo[WIA_NUM_DIP] =
{
    WIA_DIP_DEV_ID,
    WIA_DIP_VEND_DESC,
    WIA_DIP_DEV_DESC,
    WIA_DIP_DEV_TYPE,
    WIA_DIP_PORT_NAME,
    WIA_DIP_DEV_NAME,
    WIA_DIP_SERVER_NAME,
    WIA_DIP_REMOTE_DEV_ID,
    WIA_DIP_UI_CLSID,
    WIA_DIP_HW_CONFIG,
    WIA_DIP_BAUDRATE,
    WIA_DIP_STI_GEN_CAPABILITIES,
    WIA_DIP_WIA_VERSION,
    WIA_DIP_DRIVER_VERSION,
    WIA_DIP_PNP_ID,
    WIA_DIP_STI_DRIVER_VERSION,
};

const PCWSTR g_pszDeviceInfo[WIA_NUM_DIP] =
{
    WIA_DIP_DEV_ID_STR,
    WIA_DIP_VEND_DESC_STR,
    WIA_DIP_DEV_DESC_STR,
    WIA_DIP_DEV_TYPE_STR,
    WIA_DIP_PORT_NAME_STR,
    WIA_DIP_DEV_NAME_STR,
    WIA_DIP_SERVER_NAME_STR,
    WIA_DIP_REMOTE_DEV_ID_STR,
    WIA_DIP_UI_CLSID_STR,
    WIA_DIP_HW_CONFIG_STR,
    WIA_DIP_BAUDRATE_STR,
    WIA_DIP_STI_GEN_CAPABILITIES_STR,
    WIA_DIP_WIA_VERSION_STR,
    WIA_DIP_DRIVER_VERSION_STR,
    WIA_DIP_PNP_ID_STR,
    WIA_DIP_STI_DRIVER_VERSION_STR,
};

#else

extern PROPSPEC g_psDeviceInfo[WIA_NUM_DIP];
extern PROPID   g_piDeviceInfo[WIA_NUM_DIP];
extern LPOLESTR g_pszDeviceInfo[WIA_NUM_DIP];

#endif

//
// Global WIA property ID to property name array
//

#ifdef DEFINE_WIA_PROPID_TO_NAME

const WIA_PROPID_TO_NAME g_wiaPropIdToName[] =
{
    {WIA_DIP_DEV_ID,                                   WIA_DIP_DEV_ID_STR},
    {WIA_DIP_VEND_DESC,                                WIA_DIP_VEND_DESC_STR},
    {WIA_DIP_DEV_DESC,                                 WIA_DIP_DEV_DESC_STR},
    {WIA_DIP_DEV_TYPE,                                 WIA_DIP_DEV_TYPE_STR},
    {WIA_DIP_PORT_NAME,                                WIA_DIP_PORT_NAME_STR},
    {WIA_DIP_DEV_NAME,                                 WIA_DIP_DEV_NAME_STR},
    {WIA_DIP_SERVER_NAME,                              WIA_DIP_SERVER_NAME_STR},
    {WIA_DIP_REMOTE_DEV_ID,                            WIA_DIP_REMOTE_DEV_ID_STR},
    {WIA_DIP_UI_CLSID,                                 WIA_DIP_UI_CLSID_STR},
    {WIA_DIP_HW_CONFIG,                                WIA_DIP_HW_CONFIG_STR},
    {WIA_DIP_BAUDRATE,                                 WIA_DIP_BAUDRATE_STR},
    {WIA_DIP_STI_GEN_CAPABILITIES,                     WIA_DIP_STI_GEN_CAPABILITIES_STR},
    {WIA_DIP_WIA_VERSION,                              WIA_DIP_WIA_VERSION_STR},
    {WIA_DIP_DRIVER_VERSION,                           WIA_DIP_DRIVER_VERSION_STR},
    {WIA_DIP_PNP_ID,                                   WIA_DIP_PNP_ID_STR},
    {WIA_DIP_STI_DRIVER_VERSION,                       WIA_DIP_STI_DRIVER_VERSION_STR},
    {WIA_DPA_FIRMWARE_VERSION,                         WIA_DPA_FIRMWARE_VERSION_STR},
    {WIA_DPA_CONNECT_STATUS,                           WIA_DPA_CONNECT_STATUS_STR},
    {WIA_DPA_DEVICE_TIME,                              WIA_DPA_DEVICE_TIME_STR},
    {WIA_DPC_PICTURES_TAKEN,                           WIA_DPC_PICTURES_TAKEN_STR},
    {WIA_DPC_PICTURES_REMAINING,                       WIA_DPC_PICTURES_REMAINING_STR},
    {WIA_DPC_EXPOSURE_MODE,                            WIA_DPC_EXPOSURE_MODE_STR},
    {WIA_DPC_EXPOSURE_COMP,                            WIA_DPC_EXPOSURE_COMP_STR},
    {WIA_DPC_EXPOSURE_TIME,                            WIA_DPC_EXPOSURE_TIME_STR},
    {WIA_DPC_FNUMBER,                                  WIA_DPC_FNUMBER_STR},
    {WIA_DPC_FLASH_MODE,                               WIA_DPC_FLASH_MODE_STR},
    {WIA_DPC_FOCUS_MODE,                               WIA_DPC_FOCUS_MODE_STR},
    {WIA_DPC_FOCUS_MANUAL_DIST,                        WIA_DPC_FOCUS_MANUAL_DIST_STR},
    {WIA_DPC_ZOOM_POSITION,                            WIA_DPC_ZOOM_POSITION_STR},
    {WIA_DPC_PAN_POSITION,                             WIA_DPC_PAN_POSITION_STR},
    {WIA_DPC_TILT_POSITION,                            WIA_DPC_TILT_POSITION_STR},
    {WIA_DPC_TIMER_MODE,                               WIA_DPC_TIMER_MODE_STR},
    {WIA_DPC_TIMER_VALUE,                              WIA_DPC_TIMER_VALUE_STR},
    {WIA_DPC_POWER_MODE,                               WIA_DPC_POWER_MODE_STR},
    {WIA_DPC_BATTERY_STATUS,                           WIA_DPC_BATTERY_STATUS_STR},
    {WIA_DPC_DIMENSION,                                WIA_DPC_DIMENSION_STR},
    {WIA_DPS_HORIZONTAL_BED_SIZE,                      WIA_DPS_HORIZONTAL_BED_SIZE_STR},
    {WIA_DPS_VERTICAL_BED_SIZE,                        WIA_DPS_VERTICAL_BED_SIZE_STR},
    {WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE,               WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE_STR},
    {WIA_DPS_VERTICAL_SHEET_FEED_SIZE,                 WIA_DPS_VERTICAL_SHEET_FEED_SIZE_STR},
    {WIA_DPS_SHEET_FEEDER_REGISTRATION,                WIA_DPS_SHEET_FEEDER_REGISTRATION_STR},
    {WIA_DPS_HORIZONTAL_BED_REGISTRATION,              WIA_DPS_HORIZONTAL_BED_REGISTRATION_STR},
    {WIA_DPS_VERTICAL_BED_REGISTRATION,                WIA_DPS_VERTICAL_BED_REGISTRATION_STR},
    {WIA_DPS_PLATEN_COLOR,                             WIA_DPS_PLATEN_COLOR_STR},
    {WIA_DPS_PAD_COLOR,                                WIA_DPS_PAD_COLOR_STR},
    {WIA_DPS_FILTER_SELECT,                            WIA_DPS_FILTER_SELECT_STR},
    {WIA_DPS_DITHER_SELECT,                            WIA_DPS_DITHER_SELECT_STR},
    {WIA_DPS_DITHER_PATTERN_DATA,                      WIA_DPS_DITHER_PATTERN_DATA_STR},
    {WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES,           WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES_STR},
    {WIA_DPS_DOCUMENT_HANDLING_STATUS,                 WIA_DPS_DOCUMENT_HANDLING_STATUS_STR},
    {WIA_DPS_DOCUMENT_HANDLING_SELECT,                 WIA_DPS_DOCUMENT_HANDLING_SELECT_STR},
    {WIA_DPS_DOCUMENT_HANDLING_CAPACITY,               WIA_DPS_DOCUMENT_HANDLING_CAPACITY_STR},
    {WIA_DPS_OPTICAL_XRES,                             WIA_DPS_OPTICAL_XRES_STR},
    {WIA_DPS_OPTICAL_YRES,                             WIA_DPS_OPTICAL_YRES_STR},
    {WIA_DPS_ENDORSER_CHARACTERS,                      WIA_DPS_ENDORSER_CHARACTERS_STR},
    {WIA_DPS_ENDORSER_STRING,                          WIA_DPS_ENDORSER_STRING_STR},
    {WIA_DPS_SCAN_AHEAD_PAGES,                         WIA_DPS_SCAN_AHEAD_PAGES_STR},
    {WIA_DPS_MAX_SCAN_TIME,                            WIA_DPS_MAX_SCAN_TIME_STR},
    {WIA_DPS_PAGES,                                    WIA_DPS_PAGES_STR},
    {WIA_DPS_PAGE_SIZE,                                WIA_DPS_PAGE_SIZE_STR},
    {WIA_DPS_PAGE_WIDTH,                               WIA_DPS_PAGE_WIDTH_STR},
    {WIA_DPS_PAGE_HEIGHT,                              WIA_DPS_PAGE_HEIGHT_STR},
    {WIA_DPS_PREVIEW,                                  WIA_DPS_PREVIEW_STR},
    {WIA_DPS_TRANSPARENCY,                             WIA_DPS_TRANSPARENCY_STR},
    {WIA_DPS_TRANSPARENCY_SELECT,                      WIA_DPS_TRANSPARENCY_SELECT_STR},
    {WIA_DPS_SHOW_PREVIEW_CONTROL,                     WIA_DPS_SHOW_PREVIEW_CONTROL_STR},
    {WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE,           WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE_STR},
    {WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE,             WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE_STR},
    {WIA_DPS_USER_NAME,                                WIA_DPS_USER_NAME_STR},
    {WIA_DPV_LAST_PICTURE_TAKEN,                       WIA_DPV_LAST_PICTURE_TAKEN_STR},
    {WIA_DPV_IMAGES_DIRECTORY,                         WIA_DPV_IMAGES_DIRECTORY_STR},
    {WIA_DPV_DSHOW_DEVICE_PATH,                        WIA_DPV_DSHOW_DEVICE_PATH_STR},
    {WIA_DPF_MOUNT_POINT,                              WIA_DPF_MOUNT_POINT_STR},
    {WIA_IPA_ITEM_NAME,                                WIA_IPA_ITEM_NAME_STR},
    {WIA_IPA_FULL_ITEM_NAME,                           WIA_IPA_FULL_ITEM_NAME_STR},
    {WIA_IPA_ITEM_TIME,                                WIA_IPA_ITEM_TIME_STR},
    {WIA_IPA_ITEM_FLAGS,                               WIA_IPA_ITEM_FLAGS_STR},
    {WIA_IPA_ACCESS_RIGHTS,                            WIA_IPA_ACCESS_RIGHTS_STR},
    {WIA_IPA_DATATYPE,                                 WIA_IPA_DATATYPE_STR},
    {WIA_IPA_DEPTH,                                    WIA_IPA_DEPTH_STR},
    {WIA_IPA_PREFERRED_FORMAT,                         WIA_IPA_PREFERRED_FORMAT_STR},
    {WIA_IPA_FORMAT,                                   WIA_IPA_FORMAT_STR},
    {WIA_IPA_COMPRESSION,                              WIA_IPA_COMPRESSION_STR},
    {WIA_IPA_TYMED,                                    WIA_IPA_TYMED_STR},
    {WIA_IPA_CHANNELS_PER_PIXEL,                       WIA_IPA_CHANNELS_PER_PIXEL_STR},
    {WIA_IPA_BITS_PER_CHANNEL,                         WIA_IPA_BITS_PER_CHANNEL_STR},
    {WIA_IPA_PLANAR,                                   WIA_IPA_PLANAR_STR},
    {WIA_IPA_PIXELS_PER_LINE,                          WIA_IPA_PIXELS_PER_LINE_STR},
    {WIA_IPA_BYTES_PER_LINE,                           WIA_IPA_BYTES_PER_LINE_STR},
    {WIA_IPA_NUMBER_OF_LINES,                          WIA_IPA_NUMBER_OF_LINES_STR},
    {WIA_IPA_GAMMA_CURVES,                             WIA_IPA_GAMMA_CURVES_STR},
    {WIA_IPA_ITEM_SIZE,                                WIA_IPA_ITEM_SIZE_STR},
    {WIA_IPA_COLOR_PROFILE,                            WIA_IPA_COLOR_PROFILE_STR},
    {WIA_IPA_MIN_BUFFER_SIZE,                          WIA_IPA_MIN_BUFFER_SIZE_STR},
    {WIA_IPA_REGION_TYPE,                              WIA_IPA_REGION_TYPE_STR},
    {WIA_IPA_ICM_PROFILE_NAME,                         WIA_IPA_ICM_PROFILE_NAME_STR},
    {WIA_IPA_APP_COLOR_MAPPING,                        WIA_IPA_APP_COLOR_MAPPING_STR},
    {WIA_IPA_PROP_STREAM_COMPAT_ID,                    WIA_IPA_PROP_STREAM_COMPAT_ID_STR},
    {WIA_IPA_FILENAME_EXTENSION,                       WIA_IPA_FILENAME_EXTENSION_STR},
    {WIA_IPA_SUPPRESS_PROPERTY_PAGE,                   WIA_IPA_SUPPRESS_PROPERTY_PAGE_STR},
    {WIA_IPC_THUMBNAIL,                                WIA_IPC_THUMBNAIL_STR},
    {WIA_IPC_THUMB_WIDTH,                              WIA_IPC_THUMB_WIDTH_STR},
    {WIA_IPC_THUMB_HEIGHT,                             WIA_IPC_THUMB_HEIGHT_STR},
    {WIA_IPC_AUDIO_AVAILABLE,                          WIA_IPC_AUDIO_AVAILABLE_STR},
    {WIA_IPC_AUDIO_DATA_FORMAT,                        WIA_IPC_AUDIO_DATA_FORMAT_STR},
    {WIA_IPC_AUDIO_DATA,                               WIA_IPC_AUDIO_DATA_STR},
    {WIA_IPC_NUM_PICT_PER_ROW,                         WIA_IPC_NUM_PICT_PER_ROW_STR},
    {WIA_IPC_SEQUENCE,                                 WIA_IPC_SEQUENCE_STR},
    {WIA_IPC_TIMEDELAY,                                WIA_IPC_TIMEDELAY_STR},
    {WIA_IPS_CUR_INTENT,                               WIA_IPS_CUR_INTENT_STR},
    {WIA_IPS_XRES,                                     WIA_IPS_XRES_STR},
    {WIA_IPS_YRES,                                     WIA_IPS_YRES_STR},
    {WIA_IPS_XPOS,                                     WIA_IPS_XPOS_STR},
    {WIA_IPS_YPOS,                                     WIA_IPS_YPOS_STR},
    {WIA_IPS_XEXTENT,                                  WIA_IPS_XEXTENT_STR},
    {WIA_IPS_YEXTENT,                                  WIA_IPS_YEXTENT_STR},
    {WIA_IPS_PHOTOMETRIC_INTERP,                       WIA_IPS_PHOTOMETRIC_INTERP_STR},
    {WIA_IPS_BRIGHTNESS,                               WIA_IPS_BRIGHTNESS_STR},
    {WIA_IPS_CONTRAST,                                 WIA_IPS_CONTRAST_STR},
    {WIA_IPS_ORIENTATION,                              WIA_IPS_ORIENTATION_STR},
    {WIA_IPS_ROTATION,                                 WIA_IPS_ROTATION_STR},
    {WIA_IPS_MIRROR,                                   WIA_IPS_MIRROR_STR},
    {WIA_IPS_THRESHOLD,                                WIA_IPS_THRESHOLD_STR},
    {WIA_IPS_INVERT,                                   WIA_IPS_INVERT_STR},
    {WIA_IPS_WARM_UP_TIME,                             WIA_IPS_WARM_UP_TIME_STR},
#if (_WIN32_WINNT >= 0x0600)
    {WIA_IPA_ITEM_CATEGORY,                            WIA_IPA_ITEM_CATEGORY_STR},
    {WIA_IPA_RAW_BITS_PER_CHANNEL,                     WIA_IPA_RAW_BITS_PER_CHANNEL_STR},
    {WIA_IPS_DESKEW_X,                                 WIA_IPS_DESKEW_X_STR},
    {WIA_IPS_DESKEW_Y,                                 WIA_IPS_DESKEW_Y_STR},
    {WIA_IPS_SEGMENTATION,                             WIA_IPS_SEGMENTATION_STR},
    {WIA_IPS_MAX_HORIZONTAL_SIZE,                      WIA_IPS_MAX_HORIZONTAL_SIZE_STR},
    {WIA_IPS_MAX_VERTICAL_SIZE,                        WIA_IPS_MAX_VERTICAL_SIZE_STR},
    {WIA_IPS_MIN_HORIZONTAL_SIZE,                      WIA_IPS_MIN_HORIZONTAL_SIZE_STR},
    {WIA_IPS_MIN_VERTICAL_SIZE,                        WIA_IPS_MIN_VERTICAL_SIZE_STR},
    {WIA_IPS_SHEET_FEEDER_REGISTRATION,                WIA_IPS_SHEET_FEEDER_REGISTRATION_STR},
    {WIA_IPS_DOCUMENT_HANDLING_SELECT,                 WIA_IPS_DOCUMENT_HANDLING_SELECT_STR},
    {WIA_IPS_OPTICAL_XRES,                             WIA_IPS_OPTICAL_XRES_STR},
    {WIA_IPS_OPTICAL_YRES,                             WIA_IPS_OPTICAL_YRES_STR},
    {WIA_IPS_PAGES,                                    WIA_IPS_PAGES_STR},
    {WIA_IPS_PAGE_SIZE,                                WIA_IPS_PAGE_SIZE_STR},
    {WIA_IPS_PAGE_WIDTH,                               WIA_IPS_PAGE_WIDTH_STR},
    {WIA_IPS_PAGE_HEIGHT,                              WIA_IPS_PAGE_HEIGHT_STR},
    {WIA_IPS_PREVIEW,                                  WIA_IPS_PREVIEW_STR},
    {WIA_IPS_SHOW_PREVIEW_CONTROL,                     WIA_IPS_SHOW_PREVIEW_CONTROL_STR},
    {WIA_IPS_TRANSFER_CAPABILITIES,                    WIA_IPS_TRANSFER_CAPABILITIES_STR},
    {WIA_IPS_FILM_SCAN_MODE,                           WIA_IPS_FILM_SCAN_MODE_STR},
    {WIA_IPS_LAMP,                                     WIA_IPS_LAMP_STR},
    {WIA_IPS_LAMP_AUTO_OFF,                            WIA_IPS_LAMP_AUTO_OFF_STR},
    {WIA_IPS_AUTO_DESKEW,                              WIA_IPS_AUTO_DESKEW_STR},
    {WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION,             WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION_STR},
    {WIA_IPS_PREVIEW_TYPE,                             WIA_IPS_PREVIEW_TYPE_STR},
    {WIA_IPS_XSCALING,                                 WIA_IPS_XSCALING_STR},
    {WIA_IPS_YSCALING,                                 WIA_IPS_YSCALING_STR},
    {WIA_IPA_UPLOAD_ITEM_SIZE,                         WIA_IPA_UPLOAD_ITEM_SIZE_STR},
    {WIA_IPA_ITEMS_STORED,                             WIA_IPA_ITEMS_STORED_STR},
    {WIA_IPS_PRINTER_ENDORSER,                         WIA_IPS_PRINTER_ENDORSER_STR},
    {WIA_IPS_PRINTER_ENDORSER_ORDER,                   WIA_IPS_PRINTER_ENDORSER_ORDER_STR},
    {WIA_IPS_PRINTER_ENDORSER_COUNTER,                 WIA_IPS_PRINTER_ENDORSER_COUNTER_STR},
    {WIA_IPS_PRINTER_ENDORSER_STEP,                    WIA_IPS_PRINTER_ENDORSER_STEP_STR},
    {WIA_IPS_PRINTER_ENDORSER_XOFFSET,                 WIA_IPS_PRINTER_ENDORSER_XOFFSET_STR},
    {WIA_IPS_PRINTER_ENDORSER_YOFFSET,                 WIA_IPS_PRINTER_ENDORSER_YOFFSET_STR},
    {WIA_IPS_PRINTER_ENDORSER_NUM_LINES,               WIA_IPS_PRINTER_ENDORSER_NUM_LINES_STR},
    {WIA_IPS_PRINTER_ENDORSER_STRING,                  WIA_IPS_PRINTER_ENDORSER_STRING_STR},
    {WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS,        WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS_STR},
    {WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS, WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS_STR},
    {WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD,             WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD_STR},
    {WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD,           WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS,                WIA_IPS_PRINTER_ENDORSER_GRAPHICS_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION,       WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH,      WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH,      WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT,     WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT,     WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD,         WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD_STR},
    {WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD,       WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD_STR},
    {WIA_IPS_BARCODE_READER,                           WIA_IPS_BARCODE_READER_STR},
    {WIA_IPS_MAXIMUM_BARCODES_PER_PAGE,                WIA_IPS_MAXIMUM_BARCODES_PER_PAGE_STR},
    {WIA_IPS_BARCODE_SEARCH_DIRECTION,                 WIA_IPS_BARCODE_SEARCH_DIRECTION_STR},
    {WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES,           WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES_STR},
    {WIA_IPS_BARCODE_SEARCH_TIMEOUT,                   WIA_IPS_BARCODE_SEARCH_TIMEOUT_STR},
    {WIA_IPS_SUPPORTED_BARCODE_TYPES,                  WIA_IPS_SUPPORTED_BARCODE_TYPES_STR},
    {WIA_IPS_ENABLED_BARCODE_TYPES,                    WIA_IPS_ENABLED_BARCODE_TYPES_STR},
    {WIA_IPS_PATCH_CODE_READER,                        WIA_IPS_PATCH_CODE_READER_STR},
    {WIA_IPS_SUPPORTED_PATCH_CODE_TYPES,               WIA_IPS_SUPPORTED_PATCH_CODE_TYPES_STR},
    {WIA_IPS_ENABLED_PATCH_CODE_TYPES,                 WIA_IPS_ENABLED_PATCH_CODE_TYPES_STR},
    {WIA_IPS_MICR_READER,                              WIA_IPS_MICR_READER_STR},
    {WIA_IPS_JOB_SEPARATORS,                           WIA_IPS_JOB_SEPARATORS_STR},
    {WIA_IPS_LONG_DOCUMENT,                            WIA_IPS_LONG_DOCUMENT_STR},
    {WIA_IPS_BLANK_PAGES,                              WIA_IPS_BLANK_PAGES_STR},
    {WIA_IPS_MULTI_FEED,                               WIA_IPS_MULTI_FEED_STR},
    {WIA_IPS_MULTI_FEED_SENSITIVITY,                   WIA_IPS_MULTI_FEED_SENSITIVITY_STR},
    {WIA_IPS_AUTO_CROP,                                WIA_IPS_AUTO_CROP_STR},
    {WIA_IPS_OVER_SCAN,                                WIA_IPS_OVER_SCAN_STR},
    {WIA_IPS_OVER_SCAN_LEFT,                           WIA_IPS_OVER_SCAN_LEFT_STR},
    {WIA_IPS_OVER_SCAN_RIGHT,                          WIA_IPS_OVER_SCAN_RIGHT_STR},
    {WIA_IPS_OVER_SCAN_TOP,                            WIA_IPS_OVER_SCAN_TOP_STR},
    {WIA_IPS_OVER_SCAN_BOTTOM,                         WIA_IPS_OVER_SCAN_BOTTOM_STR},
    {WIA_IPS_COLOR_DROP,                               WIA_IPS_COLOR_DROP_STR},
    {WIA_IPS_COLOR_DROP_RED,                           WIA_IPS_COLOR_DROP_RED_STR},
    {WIA_IPS_COLOR_DROP_GREEN,                         WIA_IPS_COLOR_DROP_GREEN_STR},
    {WIA_IPS_COLOR_DROP_BLUE,                          WIA_IPS_COLOR_DROP_BLUE_STR},
    {WIA_IPS_SCAN_AHEAD,                               WIA_IPS_SCAN_AHEAD_STR},
    {WIA_IPS_SCAN_AHEAD_CAPACITY,                      WIA_IPS_SCAN_AHEAD_CAPACITY_STR},
    {WIA_IPS_FEEDER_CONTROL,                           WIA_IPS_FEEDER_CONTROL_STR},
    {WIA_IPS_PRINTER_ENDORSER_PADDING,                 WIA_IPS_PRINTER_ENDORSER_PADDING_STR},
    {WIA_IPS_PRINTER_ENDORSER_FONT_TYPE,               WIA_IPS_PRINTER_ENDORSER_FONT_TYPE_STR},
    {WIA_IPS_ALARM,                                    WIA_IPS_ALARM_STR},
    {WIA_IPS_PRINTER_ENDORSER_INK,                     WIA_IPS_PRINTER_ENDORSER_INK_STR},
    {WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION,      WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION_STR},
    {WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS,          WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS_STR},
    {WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS,            WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS_STR},
    {WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS,          WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS_STR},
    {WIA_IPS_COLOR_DROP_MULTI,                         WIA_IPS_COLOR_DROP_MULTI_STR},
    {WIA_IPS_BLANK_PAGES_SENSITIVITY,                  WIA_IPS_BLANK_PAGES_SENSITIVITY_STR},
#endif
    {0,                                                L"Not a standard WIA property"}
};

#else

extern WIA_PROPID_TO_NAME g_wiaPropIdToName[];

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
};
#endif

#include <poppack.h>

#endif // _WIADEF_

#endif //#if (_WIN32_WINNT >= 0x0501)
