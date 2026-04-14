//--------------------------------------------------------------------------
//    Copyright (c) Microsoft Corporation.
//
//    @File: synchronizationerrors.h
//
//    Purpose:  Error Messages for Microsoft Synchronization Platform
//
//
//---------------------------------------------------------------------------
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


//
// Define the severity codes
//


//
// MessageId: SYNC_E_ID_FORMAT_MISMATCH
//
// MessageText:
//
// Specified synchronization ID is not of the specified format for IDs of that type.
//
#define SYNC_E_ID_FORMAT_MISMATCH        0x80041000L

//
// MessageId: SYNC_E_INVALID_OPERATION
//
// MessageText:
//
// Operation is not valid due to the current state of the object.
//
#define SYNC_E_INVALID_OPERATION         0x80041001L

//
// MessageId: SYNC_E_REPLICA_NOT_FOUND
//
// MessageText:
//
// Replica with the specified key was not found.
//
#define SYNC_E_REPLICA_NOT_FOUND         0x80041002L

//
// MessageId: SYNC_E_CHANGE_COUNT_MISMATCH
//
// MessageText:
//
// Item changes provided were not of the expected quantity.
//
#define SYNC_E_CHANGE_COUNT_MISMATCH     0x80041003L

//
// MessageId: SYNC_E_CHANGE_UNIT_COUNT_MISMATCH
//
// MessageText:
//
// Change units provided were not of the expected quantity.
//
#define SYNC_E_CHANGE_UNIT_COUNT_MISMATCH 0x80041004L

//
// MessageId: SYNC_E_KNOWLEDGE_DECREASED
//
// MessageText:
//
// Knowledge has unexpectedly decreased.
//
#define SYNC_E_KNOWLEDGE_DECREASED       0x80041005L

//
// MessageId: SYNC_E_CHANGE_NOT_IN_KNOWLEDGE
//
// MessageText:
//
// Change version was not contained in knowledge as expected.
//
#define SYNC_E_CHANGE_NOT_IN_KNOWLEDGE   0x80041006L

//
// MessageId: SYNC_E_ITEM_MUST_EXIST
//
// MessageText:
//
// The item does not exist at the destination, but the creation version of the item is contained in the destination knowledge.
//
#define SYNC_E_ITEM_MUST_EXIST           0x80041007L

//
// MessageId: SYNC_E_HAS_NO_DATA
//
// MessageText:
//
// Item for which data was requested has no data either because it is a tombstone or it does not exist.
//
#define SYNC_E_HAS_NO_DATA               0x80041008L

//
// MessageId: SYNC_E_CHANGE_NEEDS_KNOWLEDGE
//
// MessageText:
//
// Change was provided that has no associated knowledge.
//
#define SYNC_E_CHANGE_NEEDS_KNOWLEDGE    0x80041009L

//
// MessageId: SYNC_E_RANGE_OUT_OF_ORDER
//
// MessageText:
//
// Range was provided out of expected order. Ranges should be provided in increasing order.
//
#define SYNC_E_RANGE_OUT_OF_ORDER        0x8004100AL

//
// MessageId: SYNC_E_NOT_EXPECTED_CHANGE
//
// MessageText:
//
// Change was provided that was not expected either because it is out-of-order or incorrect.
//
#define SYNC_E_NOT_EXPECTED_CHANGE       0x8004100BL

//
// MessageId: SYNC_E_DESERIALIZATION
//
// MessageText:
//
// Deserialization failed. This is typically due to serialization data that is not valid being supplied to the deserializer.
//
#define SYNC_E_DESERIALIZATION           0x8004100CL

//
// MessageId: SYNC_E_SINGLE_RANGE_ONLY
//
// MessageText:
//
// Only one and exactly one range may be specified per batch during forgotten knowledge recovery.
//
#define SYNC_E_SINGLE_RANGE_ONLY         0x8004100DL

//
// MessageId: SYNC_E_ITEM_HAS_CHANGE_UNITS
//
// MessageText:
//
// Operation is not valid as the specified item has change units.
//
#define SYNC_E_ITEM_HAS_CHANGE_UNITS     0x8004100EL

//
// MessageId: SYNC_E_ITEM_HAS_NO_CHANGE_UNITS
//
// MessageText:
//
// Operation is not valid as the specified item does not have change units.
//
#define SYNC_E_ITEM_HAS_NO_CHANGE_UNITS  0x8004100FL

//
// MessageId: SYNC_E_ITEM_HAS_NO_VERSION_DATA
//
// MessageText:
//
// Operation is not valid as the specified item has no version data.
//
#define SYNC_E_ITEM_HAS_NO_VERSION_DATA  0x80041010L

//
// MessageId: SYNC_E_OBJECT_NEEDS_STATE
//
// MessageText:
//
// Object was not correctly initialized. Object state info must be provided as the state could not be derived from sync session context.
//
#define SYNC_E_OBJECT_NEEDS_STATE        0x80041011L

//
// MessageId: SYNC_E_FEEDSYNC_INVALID_FEED
//
// MessageText:
//
// The specified feed is invalid.
//
#define SYNC_E_FEEDSYNC_INVALID_FEED     0x80041012L

//
// MessageId: SYNC_E_FEEDSYNC_ITEM_NOT_IN_METADATA
//
// MessageText:
//
// Item was not found in feed metadata as expected.
//
#define SYNC_E_FEEDSYNC_ITEM_NOT_IN_METADATA 0x80041013L

//
// MessageId: SYNC_E_FEEDSYNC_CALLBACK_EXPECTED
//
// MessageText:
//
// A feed item or item ID conversion result was unexpectedly not available.
//
#define SYNC_E_FEEDSYNC_CALLBACK_EXPECTED 0x80041014L

//
// MessageId: SYNC_E_INVALID_VERSION
//
// MessageText:
//
// Specified version is not supported.
//
#define SYNC_E_INVALID_VERSION           0x80041015L

//
// MessageId: SYNC_E_DUPLICATE_ITEM
//
// MessageText:
//
// Item already exists.
//
#define SYNC_E_DUPLICATE_ITEM            0x80041016L

//
// MessageId: SYNC_E_INVALID_ORDER_FOR_VECTOR_ELEMENTS
//
// MessageText:
//
// Clock vector elements are provided with the wrong order.
//
#define SYNC_E_INVALID_ORDER_FOR_VECTOR_ELEMENTS 0x80041017L

//
// MessageId: SYNC_E_INVALID_SYNC_TIME
//
// MessageText:
//
// SYNC_TIME value is not valid.
//
#define SYNC_E_INVALID_SYNC_TIME         0x80041018L

//
// MessageId: SYNC_E_INCOMPLETE_REPLICA_KEY_MAP
//
// MessageText:
//
// Incomplete replica key map.
//
#define SYNC_E_INCOMPLETE_REPLICA_KEY_MAP 0x80041019L

//
// MessageId: SYNC_E_INVALID_REPLICA_KEY
//
// MessageText:
//
// Clock vector element has a replica key not present in the replica key map.
//
#define SYNC_E_INVALID_REPLICA_KEY       0x8004101AL

//
// MessageId: SYNC_E_NEGATIVE_RANGE_EXCEPTION
//
// MessageText:
//
// Negative range exceptions are not supported.
//
#define SYNC_E_NEGATIVE_RANGE_EXCEPTION  0x8004101BL

//
// MessageId: SYNC_E_BATCH_NEEDS_KNOWLEDGE
//
// MessageText:
//
// Change batch provided unexpectedly did not contain knowledge.
//
#define SYNC_E_BATCH_NEEDS_KNOWLEDGE     0x8004101CL

//
// MessageId: SYNC_E_INTERNAL_ERROR
//
// MessageText:
//
// An internal error occurred in the synchronization runtime.
//
#define SYNC_E_INTERNAL_ERROR            0x8004101DL

//
// MessageId: SYNC_E_CHANGE_BATCH_IS_READ_ONLY
//
// MessageText:
//
// The operation could not be completed as the specified change batch has become read-only.
//
#define SYNC_E_CHANGE_BATCH_IS_READ_ONLY 0x8004101EL

//
// MessageId: SYNC_E_DATA_MODIFIED_CONCURRENTLY
//
// MessageText:
//
// The operation could not be completed as an item changed unexpectedly during synchronization.
//
#define SYNC_E_DATA_MODIFIED_CONCURRENTLY 0x8004101FL

//
// MessageId: SYNC_E_ON_CREATE_MUST_FAIL_ENTIRE_ITEM
//
// MessageText:
//
// A failure during creation may not be recorded per change unit. Item creation may only fail for the entire item.
//
#define SYNC_E_ON_CREATE_MUST_FAIL_ENTIRE_ITEM 0x80041020L

//
// MessageId: SYNC_E_FILTER_NOT_SUPPORTED
//
// MessageText:
//
// The specified filter is not supported.
//
#define SYNC_E_FILTER_NOT_SUPPORTED      0x80041021L

//
// MessageId: SYNC_E_LOAD_CONFLICT_DATA_FAILED
//
// MessageText:
//
// The requested data could not be loaded.  This occurs when a recoverable error is set while loading the data or if data conversion fails.  Possible resolutions include deferring the conflict or resolving the conflict without change data.
//
#define SYNC_E_LOAD_CONFLICT_DATA_FAILED 0x80041022L

//
// MessageId: SYNC_E_INVALID_SERIALIZATION_VERSION
//
// MessageText:
//
// Specified serialization version is not supported or cannot be used.
//
#define SYNC_E_INVALID_SERIALIZATION_VERSION 0x80041023L

//
// MessageId: SYNC_E_MARKER_MISMATCH
//
// MessageText:
//
// Marker type is not valid.
//
#define SYNC_E_MARKER_MISMATCH           0x80041024L

//
// MessageId: SYNC_E_FORGOTTEN_KNOWLEDGE_NOT_CONTAINED
//
// MessageText:
//
// The forgotten knowledge is not contained in the knowledge.
//
#define SYNC_E_FORGOTTEN_KNOWLEDGE_NOT_CONTAINED 0x80041025L

//
// MessageId: SYNC_E_ACTIVE_CHANGE_APPLICATION_CONTEXT
//
// MessageText:
//
// There is an active change application context being used.
//
#define SYNC_E_ACTIVE_CHANGE_APPLICATION_CONTEXT 0x80041026L

//
// MessageId: SYNC_E_ITEM_LIST_FILTERED_FULL_ENUMERATION_NOT_SUPPORTED
//
// MessageText:
//
// The item list filtered full enumeration synchronization is not supported.
//
#define SYNC_E_ITEM_LIST_FILTERED_FULL_ENUMERATION_NOT_SUPPORTED 0x80041027L

//
// MessageId: SYNC_E_FULL_ENUMERATION_MUST_BE_USED
//
// MessageText:
//
// Full enumeration synchronization must be used to apply full enumeration change.
//
#define SYNC_E_FULL_ENUMERATION_MUST_BE_USED 0x80041028L

//
// MessageId: SYNC_E_BATCH_NEEDS_FILTER_FORGOTTEN_KNOWLEDGE
//
// MessageText:
//
// One or more filter forgotten knowledges were not set on the change batch.
//
#define SYNC_E_BATCH_NEEDS_FILTER_FORGOTTEN_KNOWLEDGE 0x80041029L

//
// MessageId: SYNC_E_CONSTRAINT_CONFLICT_NOT_ALLOWED
//
// MessageText:
//
// A constraint conflict is not allowed for the current save action.
//
#define SYNC_E_CONSTRAINT_CONFLICT_NOT_ALLOWED 0x8004102AL

//
// MessageId: SYNC_E_SOURCE_DOES_NOT_TRACK_FILTER
//
// MessageText:
//
// Source provider does not track the specified filter.
//
#define SYNC_E_SOURCE_DOES_NOT_TRACK_FILTER 0x8004102BL

