//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 2002.
//
//  File: TPCError.h
//      Microsoft Tablet PC API Error Code definitions
//
//--------------------------------------------------------------------------

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _WINERROR_
#include <winerror.h>
#endif

#define FACILITY_INK      40
#define MAKE_INK_ERROR(err)        MAKE_HRESULT(SEVERITY_ERROR,FACILITY_INK,err)

/*** E_INK_EXCEPTION                                   0x80280001    -2144862207
*   An internal exception occurred while executing the method or property.
*/
#define E_INK_EXCEPTION                                MAKE_INK_ERROR(0x001)

/*** E_INK_MISMATCHED_INK_OBJECT                       0x80280002    -2144862206
*   The object is already associated with an ink object and cannot be reassociated.
*/
#define E_INK_MISMATCHED_INK_OBJECT                    MAKE_INK_ERROR(0x002)

/*** E_INK_COLLECTOR_BUSY                              0x80280003    -2144862205
*   The operation cannot be performed while the user is actively inking.
*/
#define E_INK_COLLECTOR_BUSY                           MAKE_INK_ERROR(0x003)

/*** E_INK_INCOMPATIBLE_OBJECT                          0x80280004    -2144862204
*   The interface pointer points to an object that is incompatible with the Ink API
*/
#define E_INK_INCOMPATIBLE_OBJECT                      MAKE_INK_ERROR(0x004)

/*** E_INK_WINDOW_NOT_SET                              0x80280005    -2144862203
*   The window handle must be set before ink collection can occur.
*/
#define E_INK_WINDOW_NOT_SET                           MAKE_INK_ERROR(0x005)

/*** E_INK_INVALID_MODE                                0x80280006    -2144862202
*   The InkCollector must be gesture mode for gesture features,
            and single tablet mode for single tablet features.
*/
#define E_INK_INVALID_MODE                             MAKE_INK_ERROR(0x006)

/*** E_INK_COLLECTOR_ENABLED                           0x80280007    -2144862201
*   The operation cannot be performed while the InkCollector is enabled.
*/
#define E_INK_COLLECTOR_ENABLED                        MAKE_INK_ERROR(0x007)

/*** E_INK_NO_STROKES_TO_RECOGNIZE                     0x80280008    -2144862200
*   There are no strokes for the recognizer to process.
*/
#define E_INK_NO_STROKES_TO_RECOGNIZE                  MAKE_INK_ERROR(0x008)

/*** E_INK_EMPTY_RECOGNITION_RESULT                    0x80280009    -2144862199
*   There are no strokes for the recognizer to process.
*/
#define E_INK_EMPTY_RECOGNITION_RESULT                 MAKE_INK_ERROR(0x009)

/*** E_INK_OVERLAPPING_INPUT_RECT                    0x80280010    -2144862192
*   "The window input rectangle overlaps with an enabled InkCollector's window input rectangle."
*/
#define E_INK_OVERLAPPING_INPUT_RECT                   MAKE_INK_ERROR(0x010)


#ifndef TPC_E_INVALID_PROPERTY
// Temporary fix until winerror.h gets published properly
#define TPC_E_INVALID_PROPERTY                0x80040241
#define TPC_E_NO_DEFAULT_TABLET               0x80040212
#define TPC_E_UNKNOWN_PROPERTY                0x8004021b
#define TPC_E_INVALID_INPUT_RECT              0x80040219
#define TPC_E_INVALID_STROKE                  0x80040222
#define TPC_E_INITIALIZE_FAIL                 0x80040223
#define TPC_E_NOT_RELEVANT                    0x80040232
#define TPC_E_RECOGNIZER_NOT_REGISTERED       0x80040235
#define TPC_E_INVALID_RIGHTS                  0x80040236
#define TPC_E_OUT_OF_ORDER_CALL               0x80040237
#define TPC_E_QUEUE_FULL                      0x80040238
#define TPC_E_INVALID_CONFIGURATION           0x80040239
#define TPC_E_INVALID_DATA_FROM_RECOGNIZER    0x8004023A
#define TPC_E_INVALID_PACKET_DESCRIPTION      0x80040233
#define TPC_S_TRUNCATED                       0x00040252
#define TPC_S_INTERRUPTED                     0x00040253
#define TPC_S_NO_DATA_TO_PROCESS              0x00040254
#endif

// IErrorInfo helper for objects that support error info (CLSID_IFoo && IID_IFoo)
#define MAKE_OBJ_ERROR_INFO( ID, hr, helpid, helpfile )     \
            AtlReportError( CLSID_##ID , IDS_##hr,          \
                            helpid, helpfile,               \
                            IID_I##ID, hr,                  \
                            _Module.GetModuleInstance())

// IErrorInfo helper for interfaces that support error info, but are not cocreatable
//      (e.g. IID_IFoo, but NOT CLSID_IFoo)
#define MAKE_INT_ERROR_INFO( ID, hr, helpid, helpfile )     \
            AtlReportError( GUID_NULL , IDS_##hr,           \
                            helpid, helpfile,               \
                            IID_I##ID, hr,                  \
                            _Module.GetModuleInstance())


#define TPC_E_INSUFFICIENT_BUFFER             __HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

