#ifndef _FILTERR_H_
#define _FILTERR_H_
#ifndef FACILITY_WINDOWS
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
#define FACILITY_ITF                     0x4
#define FACILITY_WINDOWS                 0x8


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_COERROR          0x2
#define STATUS_SEVERITY_COFAIL           0x3


//
// MessageId: NOT_AN_ERROR
//
// MessageText:
//
// NOTE:  This dummy error message is necessary to force MC to output
//        the above defines inside the FACILITY_WINDOWS guard instead
//        of leaving it empty.
//
#define NOT_AN_ERROR                     ((HRESULT)0x00080000L)

#endif // FACILITY_WINDOWS
//
// Codes 0x1700-0x172F are reserved for FILTER
//
//
// MessageId: FILTER_E_END_OF_CHUNKS
//
// MessageText:
//
// No more chunks of text available in object.
//
#define FILTER_E_END_OF_CHUNKS           ((HRESULT)0x80041700L)

//
// MessageId: FILTER_E_NO_MORE_TEXT
//
// MessageText:
//
// No more text available in chunk.
//
#define FILTER_E_NO_MORE_TEXT            ((HRESULT)0x80041701L)

//
// MessageId: FILTER_E_NO_MORE_VALUES
//
// MessageText:
//
// No more property values available in chunk.
//
#define FILTER_E_NO_MORE_VALUES          ((HRESULT)0x80041702L)

//
// MessageId: FILTER_E_ACCESS
//
// MessageText:
//
// Unable to access object.
//
#define FILTER_E_ACCESS                  ((HRESULT)0x80041703L)

//
// MessageId: FILTER_W_MONIKER_CLIPPED
//
// MessageText:
//
// Moniker doesn't cover entire region.
//
#define FILTER_W_MONIKER_CLIPPED         ((HRESULT)0x00041704L)

//
// MessageId: FILTER_E_NO_TEXT
//
// MessageText:
//
// No text in current chunk.
//
#define FILTER_E_NO_TEXT                 ((HRESULT)0x80041705L)

//
// MessageId: FILTER_E_NO_VALUES
//
// MessageText:
//
// No values in current chunk.
//
#define FILTER_E_NO_VALUES               ((HRESULT)0x80041706L)

//
// MessageId: FILTER_E_EMBEDDING_UNAVAILABLE
//
// MessageText:
//
// Unable to bind IFilter for embedded object.
//
#define FILTER_E_EMBEDDING_UNAVAILABLE   ((HRESULT)0x80041707L)

//
// MessageId: FILTER_E_LINK_UNAVAILABLE
//
// MessageText:
//
// Unable to bind IFilter for linked object.
//
#define FILTER_E_LINK_UNAVAILABLE        ((HRESULT)0x80041708L)

//
// MessageId: FILTER_S_LAST_TEXT
//
// MessageText:
//
// This is the last text in the current chunk.
//
#define FILTER_S_LAST_TEXT               ((HRESULT)0x00041709L)

//
// MessageId: FILTER_S_LAST_VALUES
//
// MessageText:
//
// This is the last value in the current chunk.
//
#define FILTER_S_LAST_VALUES             ((HRESULT)0x0004170AL)

//
// MessageId: FILTER_E_PASSWORD
//
// MessageText:
//
// File was not filtered due to password protection.
//
#define FILTER_E_PASSWORD                ((HRESULT)0x8004170BL)

//
// MessageId: FILTER_E_UNKNOWNFORMAT
//
// MessageText:
//
// The document format is not recognized by the flter.
//
#define FILTER_E_UNKNOWNFORMAT           ((HRESULT)0x8004170CL)

#endif // _FILTERR_H_
