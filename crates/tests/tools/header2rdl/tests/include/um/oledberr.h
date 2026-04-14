 /********************************************************
 *                                                       *
 *   Copyright (C) Microsoft. All rights reserved.       *
 *                                                       *
 ********************************************************/
//-----------------------------------------------------------------------------
// File:			OledbErr.mc
//
// Contents: 		
//
// Comments: 		
//
//
//-----------------------------------------------------------------------------
#ifndef _MSADERR_H_
#define _MSADERR_H_
#ifndef FACILITY_WINDOWS				  	
//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +-+-+-+-+-+---------------------+-------------------------------+
//  |S|R|C|N|r|    Facility         |               Code            |
//  +-+-+-+-+-+---------------------+-------------------------------+
//
//  where
//
//      S - Severity - indicates success/fail
//
//          0 - Success
//          1 - Fail (COERROR)
//
//      R - reserved portion of the facility code, corresponds to NT's
//              second severity bit.
//
//      C - reserved portion of the facility code, corresponds to NT's
//              C field.
//
//      N - reserved portion of the facility code. Used to indicate a
//              mapped NT status value.
//
//      r - reserved portion of the facility code. Reserved for internal
//              use. Used to indicate HRESULT values that are not status
//              values, but are instead message ids for display strings.
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
#define FACILITY_STORAGE                 0x3


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_COERROR          0x2


//
// MessageId: DB_E_BOGUS
//
// MessageText:
//
// Dummy error - need this error so that mc puts the above defines
// inside the FACILITY_WINDOWS guard, instead of leaving it empty
//
#define DB_E_BOGUS                       ((HRESULT)0x80040EFFL)

#endif // FACILITY_WINDOWS

//
// Codes 0x0e00-0x0eff are reserved for the OLE DB group of
// interfaces.
//
// Free codes are:
//
//		Error:
//						
//
//		Success:
//			0x0eea
//			
//


//
// OLEDBVER
//	OLE DB version number (0x0270); this can be overridden with an older
// version number if necessary
//

// If OLEDBVER is not defined, assume version 2.7
#ifndef OLEDBVER
#define OLEDBVER 0x0270
#endif

//
// MessageId: DB_E_BADACCESSORHANDLE
//
// MessageText:
//
// Accessor is invalid.
//
#define DB_E_BADACCESSORHANDLE           ((HRESULT)0x80040E00L)

//
// MessageId: DB_E_ROWLIMITEXCEEDED
//
// MessageText:
//
// Row could not be inserted into the rowset without exceeding provider's maximum number of active rows.
//
#define DB_E_ROWLIMITEXCEEDED            ((HRESULT)0x80040E01L)

//
// MessageId: DB_E_READONLYACCESSOR
//
// MessageText:
//
// Accessor is read-only. Operation failed.
//
#define DB_E_READONLYACCESSOR            ((HRESULT)0x80040E02L)

//
// MessageId: DB_E_SCHEMAVIOLATION
//
// MessageText:
//
// Values violate the database schema.
//
#define DB_E_SCHEMAVIOLATION             ((HRESULT)0x80040E03L)

//
// MessageId: DB_E_BADROWHANDLE
//
// MessageText:
//
// Row handle is invalid.
//
#define DB_E_BADROWHANDLE                ((HRESULT)0x80040E04L)

//
// MessageId: DB_E_OBJECTOPEN
//
// MessageText:
//
// Object was open.
//
#define DB_E_OBJECTOPEN                  ((HRESULT)0x80040E05L)

//@@@+ V1.5
#if( OLEDBVER >= 0x0150 )
//
// MessageId: DB_E_BADCHAPTER
//
// MessageText:
//
// Chapter is invalid.
//
#define DB_E_BADCHAPTER                  ((HRESULT)0x80040E06L)

#endif // OLEDBVER >= 0x0150
//@@@- V1.5

//
// MessageId: DB_E_CANTCONVERTVALUE
//
// MessageText:
//
// Data or literal value could not be converted to the type of the column in the data source, and the provider was unable to determine which columns could not be converted.  Data overflow or sign mismatch was not the cause.
//
#define DB_E_CANTCONVERTVALUE            ((HRESULT)0x80040E07L)

//
// MessageId: DB_E_BADBINDINFO
//
// MessageText:
//
// Binding information is invalid.
//
#define DB_E_BADBINDINFO                 ((HRESULT)0x80040E08L)

//
// MessageId: DB_SEC_E_PERMISSIONDENIED
//
// MessageText:
//
// Permission denied.
//
#define DB_SEC_E_PERMISSIONDENIED        ((HRESULT)0x80040E09L)

//
// MessageId: DB_E_NOTAREFERENCECOLUMN
//
// MessageText:
//
// Column does not contain bookmarks or chapters.
//
#define DB_E_NOTAREFERENCECOLUMN         ((HRESULT)0x80040E0AL)

//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_E_LIMITREJECTED
//
// MessageText:
//
// Cost limits were rejected.
//
#define DB_E_LIMITREJECTED               ((HRESULT)0x80040E0BL)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//
// MessageId: DB_E_NOCOMMAND
//
// MessageText:
//
// Command text was not set for the command object.
//
#define DB_E_NOCOMMAND                   ((HRESULT)0x80040E0CL)

//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_E_COSTLIMIT
//
// MessageText:
//
// Query plan within the cost limit cannot be found.
//
#define DB_E_COSTLIMIT                   ((HRESULT)0x80040E0DL)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//
// MessageId: DB_E_BADBOOKMARK
//
// MessageText:
//
// Bookmark is invalid.
//
#define DB_E_BADBOOKMARK                 ((HRESULT)0x80040E0EL)

//
// MessageId: DB_E_BADLOCKMODE
//
// MessageText:
//
// Lock mode is invalid.
//
#define DB_E_BADLOCKMODE                 ((HRESULT)0x80040E0FL)

//
// MessageId: DB_E_PARAMNOTOPTIONAL
//
// MessageText:
//
// No value given for one or more required parameters.
//
#define DB_E_PARAMNOTOPTIONAL            ((HRESULT)0x80040E10L)

//
// MessageId: DB_E_BADCOLUMNID
//
// MessageText:
//
// Column ID is invalid.
//
#define DB_E_BADCOLUMNID                 ((HRESULT)0x80040E11L)

//
// MessageId: DB_E_BADRATIO
//
// MessageText:
//
// Numerator was greater than denominator. Values must express ratio between zero and 1.
//
#define DB_E_BADRATIO                    ((HRESULT)0x80040E12L)

//@@@+ V2.0
#if( OLEDBVER >= 0x0200 )
//
// MessageId: DB_E_BADVALUES
//
// MessageText:
//
// Value is invalid.
//
#define DB_E_BADVALUES                   ((HRESULT)0x80040E13L)

#endif // OLEDBVER >= 0x0200
//@@@- V2.0

//
// MessageId: DB_E_ERRORSINCOMMAND
//
// MessageText:
//
// One or more errors occurred during processing of command.
//
#define DB_E_ERRORSINCOMMAND             ((HRESULT)0x80040E14L)

//
// MessageId: DB_E_CANTCANCEL
//
// MessageText:
//
// Command cannot be canceled.
//
#define DB_E_CANTCANCEL                  ((HRESULT)0x80040E15L)

//
// MessageId: DB_E_DIALECTNOTSUPPORTED
//
// MessageText:
//
// Command dialect is not supported by this provider.
//
#define DB_E_DIALECTNOTSUPPORTED         ((HRESULT)0x80040E16L)

//
// MessageId: DB_E_DUPLICATEDATASOURCE
//
// MessageText:
//
// Data source object could not be created because the named data source already exists.
//
#define DB_E_DUPLICATEDATASOURCE         ((HRESULT)0x80040E17L)

//
// MessageId: DB_E_CANNOTRESTART
//
// MessageText:
//
// Rowset position cannot be restarted.
//
#define DB_E_CANNOTRESTART               ((HRESULT)0x80040E18L)

//
// MessageId: DB_E_NOTFOUND
//
// MessageText:
//
// Object or data matching the name, range, or selection criteria was not found within the scope of this operation.
//
#define DB_E_NOTFOUND                    ((HRESULT)0x80040E19L)

//
// MessageId: DB_E_NEWLYINSERTED
//
// MessageText:
//
// Identity cannot be determined for newly inserted rows.
//
#define DB_E_NEWLYINSERTED               ((HRESULT)0x80040E1BL)

//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_E_CANNOTFREE
//
// MessageText:
//
// Provider has ownership of this tree.
//
#define DB_E_CANNOTFREE                  ((HRESULT)0x80040E1AL)

//
// MessageId: DB_E_GOALREJECTED
//
// MessageText:
//
// Goal was rejected because no nonzero weights were specified for any goals supported. Current goal was not changed.
//
#define DB_E_GOALREJECTED                ((HRESULT)0x80040E1CL)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//
// MessageId: DB_E_UNSUPPORTEDCONVERSION
//
// MessageText:
//
// Requested conversion is not supported.
//
#define DB_E_UNSUPPORTEDCONVERSION       ((HRESULT)0x80040E1DL)

//
// MessageId: DB_E_BADSTARTPOSITION
//
// MessageText:
//
// No rows were returned because the offset value moves the position before the beginning or after the end of the rowset.
//
#define DB_E_BADSTARTPOSITION            ((HRESULT)0x80040E1EL)

//@@@+ V2.0
#if( OLEDBVER >= 0x0200 )
//
// MessageId: DB_E_NOQUERY
//
// MessageText:
//
// Information was requested for a query and the query was not set.
//
#define DB_E_NOQUERY                     ((HRESULT)0x80040E1FL)

#endif // OLEDBVER >= 0x0200
//@@@- V2.0

//
// MessageId: DB_E_NOTREENTRANT
//
// MessageText:
//
// Consumer's event handler called a non-reentrant method in the provider.
//
#define DB_E_NOTREENTRANT                ((HRESULT)0x80040E20L)

//
// MessageId: DB_E_ERRORSOCCURRED
//
// MessageText:
//
// Multiple-step OLE DB operation generated errors. Check each OLE DB status value, if available. No work was done.
//
#define DB_E_ERRORSOCCURRED              ((HRESULT)0x80040E21L)

//
// MessageId: DB_E_NOAGGREGATION
//
// MessageText:
//
// Non-NULL controlling IUnknown was specified, and either the requested interface was not 
// IUnknown, or the provider does not support COM aggregation.
//
#define DB_E_NOAGGREGATION               ((HRESULT)0x80040E22L)

//
// MessageId: DB_E_DELETEDROW
//
// MessageText:
//
// Row handle referred to a deleted row or a row marked for deletion.
//
#define DB_E_DELETEDROW                  ((HRESULT)0x80040E23L)

//
// MessageId: DB_E_CANTFETCHBACKWARDS
//
// MessageText:
//
// Rowset does not support fetching backward.
//
#define DB_E_CANTFETCHBACKWARDS          ((HRESULT)0x80040E24L)

//
// MessageId: DB_E_ROWSNOTRELEASED
//
// MessageText:
//
// Row handles must all be released before new ones can be obtained.
//
#define DB_E_ROWSNOTRELEASED             ((HRESULT)0x80040E25L)

//
// MessageId: DB_E_BADSTORAGEFLAG
//
// MessageText:
//
// One or more storage flags are not supported.
//
#define DB_E_BADSTORAGEFLAG              ((HRESULT)0x80040E26L)

//@@@+ V1.5
#if( OLEDBVER >= 0x0150 )
//
// MessageId: DB_E_BADCOMPAREOP
//
// MessageText:
//
// Comparison operator is invalid.
//
#define DB_E_BADCOMPAREOP                ((HRESULT)0x80040E27L)

#endif // OLEDBVER >= 0x0150
//@@@- V1.5

//
// MessageId: DB_E_BADSTATUSVALUE
//
// MessageText:
//
// Status flag was neither DBCOLUMNSTATUS_OK nor
// DBCOLUMNSTATUS_ISNULL.
//
#define DB_E_BADSTATUSVALUE              ((HRESULT)0x80040E28L)

//
// MessageId: DB_E_CANTSCROLLBACKWARDS
//
// MessageText:
//
// Rowset does not support scrolling backward.
//
#define DB_E_CANTSCROLLBACKWARDS         ((HRESULT)0x80040E29L)

//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_E_BADREGIONHANDLE
//
// MessageText:
//
// Region handle is invalid.
//
#define DB_E_BADREGIONHANDLE             ((HRESULT)0x80040E2AL)

//
// MessageId: DB_E_NONCONTIGUOUSRANGE
//
// MessageText:
//
// Set of rows is not contiguous to, or does not overlap, the rows in the watch region.
//
#define DB_E_NONCONTIGUOUSRANGE          ((HRESULT)0x80040E2BL)

//
// MessageId: DB_E_INVALIDTRANSITION
//
// MessageText:
//
// Transition from ALL* to MOVE* or EXTEND* was specified.
//
#define DB_E_INVALIDTRANSITION           ((HRESULT)0x80040E2CL)

//
// MessageId: DB_E_NOTASUBREGION
//
// MessageText:
//
// Region is not a proper subregion of the region identified by the watch region handle.
//
#define DB_E_NOTASUBREGION               ((HRESULT)0x80040E2DL)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//
// MessageId: DB_E_MULTIPLESTATEMENTS
//
// MessageText:
//
// Multiple-statement commands are not supported by this provider.
//
#define DB_E_MULTIPLESTATEMENTS          ((HRESULT)0x80040E2EL)

//
// MessageId: DB_E_INTEGRITYVIOLATION
//
// MessageText:
//
// Value violated the integrity constraints for a column or table.
//
#define DB_E_INTEGRITYVIOLATION          ((HRESULT)0x80040E2FL)

//
// MessageId: DB_E_BADTYPENAME
//
// MessageText:
//
// Type name is invalid.
//
#define DB_E_BADTYPENAME                 ((HRESULT)0x80040E30L)

//
// MessageId: DB_E_ABORTLIMITREACHED
//
// MessageText:
//
// Execution stopped because a resource limit was reached. No results were returned.
//
#define DB_E_ABORTLIMITREACHED           ((HRESULT)0x80040E31L)

//@@@+ V2.0
#if( OLEDBVER >= 0x0200 )
//
// MessageId: DB_E_ROWSETINCOMMAND
//
// MessageText:
//
// Command object whose command tree contains a rowset or rowsets cannot be cloned.
//
#define DB_E_ROWSETINCOMMAND             ((HRESULT)0x80040E32L)

//
// MessageId: DB_E_CANTTRANSLATE
//
// MessageText:
//
// Current tree cannot be represented as text.
//
#define DB_E_CANTTRANSLATE               ((HRESULT)0x80040E33L)

#endif // OLEDBVER >= 0x0200
//@@@- V2.0

//
// MessageId: DB_E_DUPLICATEINDEXID
//
// MessageText:
//
// Index already exists.
//
#define DB_E_DUPLICATEINDEXID            ((HRESULT)0x80040E34L)

//
// MessageId: DB_E_NOINDEX
//
// MessageText:
//
// Index does not exist.
//
#define DB_E_NOINDEX                     ((HRESULT)0x80040E35L)

//
// MessageId: DB_E_INDEXINUSE
//
// MessageText:
//
// Index is in use.
//
#define DB_E_INDEXINUSE                  ((HRESULT)0x80040E36L)

//
// MessageId: DB_E_NOTABLE
//
// MessageText:
//
// Table does not exist.
//
#define DB_E_NOTABLE                     ((HRESULT)0x80040E37L)

//
// MessageId: DB_E_CONCURRENCYVIOLATION
//
// MessageText:
//
// Rowset used optimistic concurrency and the value of a column has changed since it was last read.
//
#define DB_E_CONCURRENCYVIOLATION        ((HRESULT)0x80040E38L)

//
// MessageId: DB_E_BADCOPY
//
// MessageText:
//
// Errors detected during the copy.
//
#define DB_E_BADCOPY                     ((HRESULT)0x80040E39L)

//
// MessageId: DB_E_BADPRECISION
//
// MessageText:
//
// Precision is invalid.
//
#define DB_E_BADPRECISION                ((HRESULT)0x80040E3AL)

//
// MessageId: DB_E_BADSCALE
//
// MessageText:
//
// Scale is invalid.
//
#define DB_E_BADSCALE                    ((HRESULT)0x80040E3BL)

//
// MessageId: DB_E_BADTABLEID
//
// MessageText:
//
// Table ID is invalid.
//
#define DB_E_BADTABLEID                  ((HRESULT)0x80040E3CL)

// DB_E_BADID is deprecated; use DB_E_BADTABLEID instead
#define DB_E_BADID DB_E_BADTABLEID

//
// MessageId: DB_E_BADTYPE
//
// MessageText:
//
// Type is invalid.
//
#define DB_E_BADTYPE                     ((HRESULT)0x80040E3DL)

//
// MessageId: DB_E_DUPLICATECOLUMNID
//
// MessageText:
//
// Column ID already exists or occurred more than once in the array of columns.
//
#define DB_E_DUPLICATECOLUMNID           ((HRESULT)0x80040E3EL)

//
// MessageId: DB_E_DUPLICATETABLEID
//
// MessageText:
//
// Table already exists.
//
#define DB_E_DUPLICATETABLEID            ((HRESULT)0x80040E3FL)

//
// MessageId: DB_E_TABLEINUSE
//
// MessageText:
//
// Table is in use.
//
#define DB_E_TABLEINUSE                  ((HRESULT)0x80040E40L)

//
// MessageId: DB_E_NOLOCALE
//
// MessageText:
//
// Locale ID is not supported.
//
#define DB_E_NOLOCALE                    ((HRESULT)0x80040E41L)

//
// MessageId: DB_E_BADRECORDNUM
//
// MessageText:
//
// Record number is invalid.
//
#define DB_E_BADRECORDNUM                ((HRESULT)0x80040E42L)

//
// MessageId: DB_E_BOOKMARKSKIPPED
//
// MessageText:
//
// Form of bookmark is valid, but no row was found to match it.
//
#define DB_E_BOOKMARKSKIPPED             ((HRESULT)0x80040E43L)

//
// MessageId: DB_E_BADPROPERTYVALUE
//
// MessageText:
//
// Property value is invalid.
//
#define DB_E_BADPROPERTYVALUE            ((HRESULT)0x80040E44L)

//
// MessageId: DB_E_INVALID
//
// MessageText:
//
// Rowset is not chaptered.
//
#define DB_E_INVALID                     ((HRESULT)0x80040E45L)

//
// MessageId: DB_E_BADACCESSORFLAGS
//
// MessageText:
//
// One or more accessor flags were invalid.
//
#define DB_E_BADACCESSORFLAGS            ((HRESULT)0x80040E46L)

//
// MessageId: DB_E_BADSTORAGEFLAGS
//
// MessageText:
//
// One or more storage flags are invalid.
//
#define DB_E_BADSTORAGEFLAGS             ((HRESULT)0x80040E47L)

//
// MessageId: DB_E_BYREFACCESSORNOTSUPPORTED
//
// MessageText:
//
// Reference accessors are not supported by this provider.
//
#define DB_E_BYREFACCESSORNOTSUPPORTED   ((HRESULT)0x80040E48L)

//
// MessageId: DB_E_NULLACCESSORNOTSUPPORTED
//
// MessageText:
//
// Null accessors are not supported by this provider.
//
#define DB_E_NULLACCESSORNOTSUPPORTED    ((HRESULT)0x80040E49L)

//
// MessageId: DB_E_NOTPREPARED
//
// MessageText:
//
// Command was not prepared.
//
#define DB_E_NOTPREPARED                 ((HRESULT)0x80040E4AL)

//
// MessageId: DB_E_BADACCESSORTYPE
//
// MessageText:
//
// Accessor is not a parameter accessor.
//
#define DB_E_BADACCESSORTYPE             ((HRESULT)0x80040E4BL)

//
// MessageId: DB_E_WRITEONLYACCESSOR
//
// MessageText:
//
// Accessor is write-only.
//
#define DB_E_WRITEONLYACCESSOR           ((HRESULT)0x80040E4CL)

//
// MessageId: DB_SEC_E_AUTH_FAILED
//
// MessageText:
//
// Authentication failed.
//
#define DB_SEC_E_AUTH_FAILED             ((HRESULT)0x80040E4DL)

//
// MessageId: DB_E_CANCELED
//
// MessageText:
//
// Operation was canceled.
//
#define DB_E_CANCELED                    ((HRESULT)0x80040E4EL)

//@@@+ V2.0
#if( OLEDBVER >= 0x0200 )
//
// MessageId: DB_E_CHAPTERNOTRELEASED
//
// MessageText:
//
// Rowset is single-chaptered. The chapter was not released.
//
#define DB_E_CHAPTERNOTRELEASED          ((HRESULT)0x80040E4FL)

#endif // OLEDBVER >= 0x0200
//@@@- V2.0

//
// MessageId: DB_E_BADSOURCEHANDLE
//
// MessageText:
//
// Source handle is invalid.
//
#define DB_E_BADSOURCEHANDLE             ((HRESULT)0x80040E50L)

//
// MessageId: DB_E_PARAMUNAVAILABLE
//
// MessageText:
//
// Provider cannot derive parameter information and SetParameterInfo has not been called.
//
#define DB_E_PARAMUNAVAILABLE            ((HRESULT)0x80040E51L)

//
// MessageId: DB_E_ALREADYINITIALIZED
//
// MessageText:
//
// Data source object is already initialized.
//
#define DB_E_ALREADYINITIALIZED          ((HRESULT)0x80040E52L)

//
// MessageId: DB_E_NOTSUPPORTED
//
// MessageText:
//
// Method is not supported by this provider.
//
#define DB_E_NOTSUPPORTED                ((HRESULT)0x80040E53L)

//
// MessageId: DB_E_MAXPENDCHANGESEXCEEDED
//
// MessageText:
//
// Number of rows with pending changes exceeded the limit.
//
#define DB_E_MAXPENDCHANGESEXCEEDED      ((HRESULT)0x80040E54L)

//
// MessageId: DB_E_BADORDINAL
//
// MessageText:
//
// Column does not exist.
//
#define DB_E_BADORDINAL                  ((HRESULT)0x80040E55L)

//
// MessageId: DB_E_PENDINGCHANGES
//
// MessageText:
//
// Pending changes exist on a row with a reference count of zero.
//
#define DB_E_PENDINGCHANGES              ((HRESULT)0x80040E56L)

//
// MessageId: DB_E_DATAOVERFLOW
//
// MessageText:
//
// Literal value in the command exceeded the range of the type of the associated column.
//
#define DB_E_DATAOVERFLOW                ((HRESULT)0x80040E57L)

//
// MessageId: DB_E_BADHRESULT
//
// MessageText:
//
// HRESULT is invalid.
//
#define DB_E_BADHRESULT                  ((HRESULT)0x80040E58L)

//
// MessageId: DB_E_BADLOOKUPID
//
// MessageText:
//
// Lookup ID is invalid.
//
#define DB_E_BADLOOKUPID                 ((HRESULT)0x80040E59L)

//
// MessageId: DB_E_BADDYNAMICERRORID
//
// MessageText:
//
// DynamicError ID is invalid.
//
#define DB_E_BADDYNAMICERRORID           ((HRESULT)0x80040E5AL)

//
// MessageId: DB_E_PENDINGINSERT
//
// MessageText:
//
// Most recent data for a newly inserted row could not be retrieved because the insert is pending.
//
#define DB_E_PENDINGINSERT               ((HRESULT)0x80040E5BL)

//
// MessageId: DB_E_BADCONVERTFLAG
//
// MessageText:
//
// Conversion flag is invalid.
//
#define DB_E_BADCONVERTFLAG              ((HRESULT)0x80040E5CL)

//
// MessageId: DB_E_BADPARAMETERNAME
//
// MessageText:
//
// Parameter name is unrecognized.
//
#define DB_E_BADPARAMETERNAME            ((HRESULT)0x80040E5DL)

//
// MessageId: DB_E_MULTIPLESTORAGE
//
// MessageText:
//
// Multiple storage objects cannot be open simultaneously.
//
#define DB_E_MULTIPLESTORAGE             ((HRESULT)0x80040E5EL)

//
// MessageId: DB_E_CANTFILTER
//
// MessageText:
//
// Filter cannot be opened.
//
#define DB_E_CANTFILTER                  ((HRESULT)0x80040E5FL)

//
// MessageId: DB_E_CANTORDER
//
// MessageText:
//
// Order cannot be opened.
//
#define DB_E_CANTORDER                   ((HRESULT)0x80040E60L)

//@@@+ V2.0
#if( OLEDBVER >= 0x0200 )
//
// MessageId: MD_E_BADTUPLE
//
// MessageText:
//
// Tuple is invalid.
//
#define MD_E_BADTUPLE                    ((HRESULT)0x80040E61L)

//
// MessageId: MD_E_BADCOORDINATE
//
// MessageText:
//
// Coordinate is invalid.
//
#define MD_E_BADCOORDINATE               ((HRESULT)0x80040E62L)

//
// MessageId: MD_E_INVALIDAXIS
//
// MessageText:
//
// Axis is invalid.
//
#define MD_E_INVALIDAXIS                 ((HRESULT)0x80040E63L)

//
// MessageId: MD_E_INVALIDCELLRANGE
//
// MessageText:
//
// One or more cell ordinals is invalid.
//
#define MD_E_INVALIDCELLRANGE            ((HRESULT)0x80040E64L)

//
// MessageId: DB_E_NOCOLUMN
//
// MessageText:
//
// Column ID is invalid.
//
#define DB_E_NOCOLUMN                    ((HRESULT)0x80040E65L)

//
// MessageId: DB_E_COMMANDNOTPERSISTED
//
// MessageText:
//
// Command does not have a DBID.
//
#define DB_E_COMMANDNOTPERSISTED         ((HRESULT)0x80040E67L)

//
// MessageId: DB_E_DUPLICATEID
//
// MessageText:
//
// DBID already exists.
//
#define DB_E_DUPLICATEID                 ((HRESULT)0x80040E68L)

//
// MessageId: DB_E_OBJECTCREATIONLIMITREACHED
//
// MessageText:
//
// Session cannot be created because maximum number of active sessions was already reached. Consumer must release one or more sessions before creating a new session object. 
//
#define DB_E_OBJECTCREATIONLIMITREACHED  ((HRESULT)0x80040E69L)

//
// MessageId: DB_E_BADINDEXID
//
// MessageText:
//
// Index ID is invalid.
//
#define DB_E_BADINDEXID                  ((HRESULT)0x80040E72L)

//
// MessageId: DB_E_BADINITSTRING
//
// MessageText:
//
// Format of the initialization string does not conform to the OLE DB specification.
//
#define DB_E_BADINITSTRING               ((HRESULT)0x80040E73L)

//
// MessageId: DB_E_NOPROVIDERSREGISTERED
//
// MessageText:
//
// No OLE DB providers of this source type are registered.
//
#define DB_E_NOPROVIDERSREGISTERED       ((HRESULT)0x80040E74L)

//
// MessageId: DB_E_MISMATCHEDPROVIDER
//
// MessageText:
//
// Initialization string specifies a provider that does not match the active provider.
//
#define DB_E_MISMATCHEDPROVIDER          ((HRESULT)0x80040E75L)

//
// MessageId: DB_E_BADCOMMANDID
//
// MessageText:
//
// DBID is invalid.
//
#define DB_E_BADCOMMANDID                ((HRESULT)0x80040E76L)

#endif // OLEDBVER >= 0x0200
//@@@- V2.0
//@@@+ V2.1
#if( OLEDBVER >= 0x0210 )
#define SEC_E_PERMISSIONDENIED DB_SEC_E_PERMISSIONDENIED
//
// MessageId: SEC_E_BADTRUSTEEID
//
// MessageText:
//
// Trustee is invalid.
//
#define SEC_E_BADTRUSTEEID               ((HRESULT)0x80040E6AL)

//
// MessageId: SEC_E_NOTRUSTEEID
//
// MessageText:
//
// Trustee was not recognized for this data source.
//
#define SEC_E_NOTRUSTEEID                ((HRESULT)0x80040E6BL)

//
// MessageId: SEC_E_NOMEMBERSHIPSUPPORT
//
// MessageText:
//
// Trustee does not support memberships or collections.
//
#define SEC_E_NOMEMBERSHIPSUPPORT        ((HRESULT)0x80040E6CL)

//
// MessageId: SEC_E_INVALIDOBJECT
//
// MessageText:
//
// Object is invalid or unknown to the provider.
//
#define SEC_E_INVALIDOBJECT              ((HRESULT)0x80040E6DL)

//
// MessageId: SEC_E_NOOWNER
//
// MessageText:
//
// Object does not have an owner.
//
#define SEC_E_NOOWNER                    ((HRESULT)0x80040E6EL)

//
// MessageId: SEC_E_INVALIDACCESSENTRYLIST
//
// MessageText:
//
// Access entry list is invalid.
//
#define SEC_E_INVALIDACCESSENTRYLIST     ((HRESULT)0x80040E6FL)

//
// MessageId: SEC_E_INVALIDOWNER
//
// MessageText:
//
// Trustee supplied as owner is invalid or unknown to the provider.
//
#define SEC_E_INVALIDOWNER               ((HRESULT)0x80040E70L)

//
// MessageId: SEC_E_INVALIDACCESSENTRY
//
// MessageText:
//
// Permission in the access entry list is invalid.
//
#define SEC_E_INVALIDACCESSENTRY         ((HRESULT)0x80040E71L)

//
// MessageId: DB_E_BADCONSTRAINTTYPE
//
// MessageText:
//
// ConstraintType is invalid or not supported by the provider.
//
#define DB_E_BADCONSTRAINTTYPE           ((HRESULT)0x80040E77L)

//
// MessageId: DB_E_BADCONSTRAINTFORM
//
// MessageText:
//
// ConstraintType is not DBCONSTRAINTTYPE_FOREIGNKEY and cForeignKeyColumns is not zero.
//
#define DB_E_BADCONSTRAINTFORM           ((HRESULT)0x80040E78L)

//
// MessageId: DB_E_BADDEFERRABILITY
//
// MessageText:
//
// Specified deferrability flag is invalid or not supported by the provider.
//
#define DB_E_BADDEFERRABILITY            ((HRESULT)0x80040E79L)

//
// MessageId: DB_E_BADMATCHTYPE
//
// MessageText:
//
// MatchType is invalid or the value is not supported by the provider.
//
#define DB_E_BADMATCHTYPE                ((HRESULT)0x80040E80L)

//
// MessageId: DB_E_BADUPDATEDELETERULE
//
// MessageText:
//
// Constraint update rule or delete rule is invalid.
//
#define DB_E_BADUPDATEDELETERULE         ((HRESULT)0x80040E8AL)

//
// MessageId: DB_E_BADCONSTRAINTID
//
// MessageText:
//
// Constraint ID is invalid.
//
#define DB_E_BADCONSTRAINTID             ((HRESULT)0x80040E8BL)

//
// MessageId: DB_E_BADCOMMANDFLAGS
//
// MessageText:
//
// Command persistence flag is invalid.
//
#define DB_E_BADCOMMANDFLAGS             ((HRESULT)0x80040E8CL)

//
// MessageId: DB_E_OBJECTMISMATCH
//
// MessageText:
//
// rguidColumnType points to a GUID that does not match the object type of this column, or this column was not set.
//
#define DB_E_OBJECTMISMATCH              ((HRESULT)0x80040E8DL)

//
// MessageId: DB_E_NOSOURCEOBJECT
//
// MessageText:
//
// Source row does not exist.
//
#define DB_E_NOSOURCEOBJECT              ((HRESULT)0x80040E91L)

//
// MessageId: DB_E_RESOURCELOCKED
//
// MessageText:
//
// OLE DB object represented by this URL is locked by one or more other processes.
//
#define DB_E_RESOURCELOCKED              ((HRESULT)0x80040E92L)

//
// MessageId: DB_E_NOTCOLLECTION
//
// MessageText:
//
// Client requested an object type that is valid only for a collection. 
//
#define DB_E_NOTCOLLECTION               ((HRESULT)0x80040E93L)

//
// MessageId: DB_E_READONLY
//
// MessageText:
//
// Caller requested write access to a read-only object.
//
#define DB_E_READONLY                    ((HRESULT)0x80040E94L)

//
// MessageId: DB_E_ASYNCNOTSUPPORTED
//
// MessageText:
//
// Asynchronous binding is not supported by this provider.
//
#define DB_E_ASYNCNOTSUPPORTED           ((HRESULT)0x80040E95L)

//
// MessageId: DB_E_CANNOTCONNECT
//
// MessageText:
//
// Connection to the server for this URL cannot be established.
//
#define DB_E_CANNOTCONNECT               ((HRESULT)0x80040E96L)

//
// MessageId: DB_E_TIMEOUT
//
// MessageText:
//
// Timeout occurred when attempting to bind to the object.
//
#define DB_E_TIMEOUT                     ((HRESULT)0x80040E97L)

//
// MessageId: DB_E_RESOURCEEXISTS
//
// MessageText:
//
// Object cannot be created at this URL because an object named by this URL already exists.
//
#define DB_E_RESOURCEEXISTS              ((HRESULT)0x80040E98L)

//
// MessageId: DB_E_RESOURCEOUTOFSCOPE
//
// MessageText:
//
// URL is outside of scope.
//
#define DB_E_RESOURCEOUTOFSCOPE          ((HRESULT)0x80040E8EL)

//
// MessageId: DB_E_DROPRESTRICTED
//
// MessageText:
//
// Column or constraint could not be dropped because it is referenced by a dependent view or constraint.
//
#define DB_E_DROPRESTRICTED              ((HRESULT)0x80040E90L)

//
// MessageId: DB_E_DUPLICATECONSTRAINTID
//
// MessageText:
//
// Constraint already exists.
//
#define DB_E_DUPLICATECONSTRAINTID       ((HRESULT)0x80040E99L)

//
// MessageId: DB_E_OUTOFSPACE
//
// MessageText:
//
// Object cannot be created at this URL because the server is out of physical storage.
//
#define DB_E_OUTOFSPACE                  ((HRESULT)0x80040E9AL)

#define SEC_E_PERMISSIONDENIED DB_SEC_E_PERMISSIONDENIED
#endif // OLEDBVER >= 0x0210
//@@@- V2.1
//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_SEC_E_SAFEMODE_DENIED
//
// MessageText:
//
// Safety settings on this computer prohibit accessing a data source on another domain.
//
#define DB_SEC_E_SAFEMODE_DENIED         ((HRESULT)0x80040E9BL)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//@@@+ V2.6
#if( OLEDBVER >= 0x0260 )
//
// MessageId: DB_E_NOSTATISTIC
//
// MessageText:
//
// The specified statistic does not exist in the current data source or did not apply to the specified table or it does not support a histogram. 
//
#define DB_E_NOSTATISTIC                 ((HRESULT)0x80040E9CL)

//
// MessageId: DB_E_ALTERRESTRICTED
//
// MessageText:
//
// Column or table could not be altered because it is referenced by a constraint.
//
#define DB_E_ALTERRESTRICTED             ((HRESULT)0x80040E9DL)

//
// MessageId: DB_E_RESOURCENOTSUPPORTED
//
// MessageText:
//
// Requested object type is not supported by the provider.
//
#define DB_E_RESOURCENOTSUPPORTED        ((HRESULT)0x80040E9EL)

//
// MessageId: DB_E_NOCONSTRAINT
//
// MessageText:
//
// Constraint does not exist.
//
#define DB_E_NOCONSTRAINT                ((HRESULT)0x80040E9FL)

//
// MessageId: DB_E_COLUMNUNAVAILABLE
//
// MessageText:
//
// Requested column is valid, but could not be retrieved. This could be due to a forward only cursor attempting to go backwards in a row.
//
#define DB_E_COLUMNUNAVAILABLE           ((HRESULT)0x80040EA0L)

#endif // OLEDBVER >= 0x0260
//@@@- V2.6
//
// MessageId: DB_S_ROWLIMITEXCEEDED
//
// MessageText:
//
// Fetching requested number of rows will exceed total number of active rows supported by the rowset.
//
#define DB_S_ROWLIMITEXCEEDED            ((HRESULT)0x00040EC0L)

//
// MessageId: DB_S_COLUMNTYPEMISMATCH
//
// MessageText:
//
// One or more column types are incompatible. Conversion errors will occur during copying.
//
#define DB_S_COLUMNTYPEMISMATCH          ((HRESULT)0x00040EC1L)

//
// MessageId: DB_S_TYPEINFOOVERRIDDEN
//
// MessageText:
//
// Parameter type information was overridden by caller.
//
#define DB_S_TYPEINFOOVERRIDDEN          ((HRESULT)0x00040EC2L)

//
// MessageId: DB_S_BOOKMARKSKIPPED
//
// MessageText:
//
// Bookmark was skipped for deleted or nonmember row.
//
#define DB_S_BOOKMARKSKIPPED             ((HRESULT)0x00040EC3L)

//@@@+ V2.0
#if( OLEDBVER >= 0x0200 )
//
// MessageId: DB_S_NONEXTROWSET
//
// MessageText:
//
// No more rowsets.
//
#define DB_S_NONEXTROWSET                ((HRESULT)0x00040EC5L)

#endif // OLEDBVER >= 0x0200
//@@@- V2.0

//
// MessageId: DB_S_ENDOFROWSET
//
// MessageText:
//
// Start or end of rowset or chapter was reached.
//
#define DB_S_ENDOFROWSET                 ((HRESULT)0x00040EC6L)

//
// MessageId: DB_S_COMMANDREEXECUTED
//
// MessageText:
//
// Command was reexecuted.
//
#define DB_S_COMMANDREEXECUTED           ((HRESULT)0x00040EC7L)

//
// MessageId: DB_S_BUFFERFULL
//
// MessageText:
//
// Operation succeeded, but status array or string buffer could not be allocated. 
//
#define DB_S_BUFFERFULL                  ((HRESULT)0x00040EC8L)

//
// MessageId: DB_S_NORESULT
//
// MessageText:
//
// No more results.
//
#define DB_S_NORESULT                    ((HRESULT)0x00040EC9L)

//
// MessageId: DB_S_CANTRELEASE
//
// MessageText:
//
// Server cannot release or downgrade a lock until the end of the transaction.
//
#define DB_S_CANTRELEASE                 ((HRESULT)0x00040ECAL)

//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_S_GOALCHANGED
//
// MessageText:
//
// Weight is not supported or exceeded the supported limit, and was set to 0 or the supported limit.
//
#define DB_S_GOALCHANGED                 ((HRESULT)0x00040ECBL)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//@@@+ V1.5
#if( OLEDBVER >= 0x0150 )
//
// MessageId: DB_S_UNWANTEDOPERATION
//
// MessageText:
//
// Consumer does not want to receive further notification calls for this operation. 
//
#define DB_S_UNWANTEDOPERATION           ((HRESULT)0x00040ECCL)

#endif // OLEDBVER >= 0x0150
//@@@- V1.5

//
// MessageId: DB_S_DIALECTIGNORED
//
// MessageText:
//
// Input dialect was ignored and command was processed using default dialect.
//
#define DB_S_DIALECTIGNORED              ((HRESULT)0x00040ECDL)

//
// MessageId: DB_S_UNWANTEDPHASE
//
// MessageText:
//
// Consumer does not want to receive further notification calls for this phase.
//
#define DB_S_UNWANTEDPHASE               ((HRESULT)0x00040ECEL)

//
// MessageId: DB_S_UNWANTEDREASON
//
// MessageText:
//
// Consumer does not want to receive further notification calls for this reason.
//
#define DB_S_UNWANTEDREASON              ((HRESULT)0x00040ECFL)

//@@@+ V1.5
#if( OLEDBVER >= 0x0150 )
//
// MessageId: DB_S_ASYNCHRONOUS
//
// MessageText:
//
// Operation is being processed asynchronously.
//
#define DB_S_ASYNCHRONOUS                ((HRESULT)0x00040ED0L)

#endif // OLEDBVER >= 0x0150
//@@@- V1.5

//
// MessageId: DB_S_COLUMNSCHANGED
//
// MessageText:
//
// Command was executed to reposition to the start of the rowset. Either the order of the columns changed, or columns were added to or removed from the rowset.
//
#define DB_S_COLUMNSCHANGED              ((HRESULT)0x00040ED1L)

//
// MessageId: DB_S_ERRORSRETURNED
//
// MessageText:
//
// Method had some errors, which were returned in the error array.
//
#define DB_S_ERRORSRETURNED              ((HRESULT)0x00040ED2L)

//
// MessageId: DB_S_BADROWHANDLE
//
// MessageText:
//
// Row handle is invalid.
//
#define DB_S_BADROWHANDLE                ((HRESULT)0x00040ED3L)

//
// MessageId: DB_S_DELETEDROW
//
// MessageText:
//
// Row handle referred to a deleted row.
//
#define DB_S_DELETEDROW                  ((HRESULT)0x00040ED4L)

//@@@+ V2.5
#if( OLEDBVER >= 0x0250 )
//
// MessageId: DB_S_TOOMANYCHANGES
//
// MessageText:
//
// Provider cannot keep track of all the changes. Client must refetch the data associated with the watch region by using another method.
//
#define DB_S_TOOMANYCHANGES              ((HRESULT)0x00040ED5L)

#endif // OLEDBVER >= 0x0250
//@@@- V2.5

//
// MessageId: DB_S_STOPLIMITREACHED
//
// MessageText:
//
// Execution stopped because a resource limit was reached. Results obtained so far were returned, but execution cannot resume.
//
#define DB_S_STOPLIMITREACHED            ((HRESULT)0x00040ED6L)

//
// MessageId: DB_S_LOCKUPGRADED
//
// MessageText:
//
// Lock was upgraded from the value specified.
//
#define DB_S_LOCKUPGRADED                ((HRESULT)0x00040ED8L)

//
// MessageId: DB_S_PROPERTIESCHANGED
//
// MessageText:
//
// One or more properties were changed as allowed by provider.
//
#define DB_S_PROPERTIESCHANGED           ((HRESULT)0x00040ED9L)

//
// MessageId: DB_S_ERRORSOCCURRED
//
// MessageText:
//
// Multiple-step operation completed with one or more errors. Check each status value.
//
#define DB_S_ERRORSOCCURRED              ((HRESULT)0x00040EDAL)

//
// MessageId: DB_S_PARAMUNAVAILABLE
//
// MessageText:
//
// Parameter is invalid.
//
#define DB_S_PARAMUNAVAILABLE            ((HRESULT)0x00040EDBL)

//
// MessageId: DB_S_MULTIPLECHANGES
//
// MessageText:
//
// Updating a row caused more than one row to be updated in the data source.
//
#define DB_S_MULTIPLECHANGES             ((HRESULT)0x00040EDCL)

//@@@+ V2.1
#if( OLEDBVER >= 0x0210 )
//
// MessageId: DB_S_NOTSINGLETON
//
// MessageText:
//
// Row object was requested on a non-singleton result. First row was returned.
//
#define DB_S_NOTSINGLETON                ((HRESULT)0x00040ED7L)

//
// MessageId: DB_S_NOROWSPECIFICCOLUMNS
//
// MessageText:
//
// Row has no row-specific columns.
//
#define DB_S_NOROWSPECIFICCOLUMNS        ((HRESULT)0x00040EDDL)

#endif // OLEDBVER >= 0x0210
//@@@- V2.1
// To help DSL display more meaningful error message, we need to overwrite system error message
// in the following two cases.
#ifdef MESSAGESANDHEADERS
//(0x80030002L)STG_E_FILENOTFOUND
//
// MessageId: STG_E_FILENOTFOUND
//
// MessageText:
//
// The file could not be found.
//
#define STG_E_FILENOTFOUND               ((HRESULT)0x80030002L)

//(0x80030003L)STG_E_PATHNOTFOUND
//
// MessageId: STG_E_PATHNOTFOUND
//
// MessageText:
//
// The path could not be found.
//
#define STG_E_PATHNOTFOUND               ((HRESULT)0x80030003L)

//(0x80030050L)STG_E_FILEALREADYEXISTS
//
// MessageId: STG_E_FILEALREADYEXISTS
//
// MessageText:
//
// The file already exists.
//
#define STG_E_FILEALREADYEXISTS          ((HRESULT)0x80030050L)

//(0x800300fbL)STG_E_INVALIDHEADER
//
// MessageId: STG_E_INVALIDHEADER
//
// MessageText:
//
// The file is not a valid compound file.
//
#define STG_E_INVALIDHEADER              ((HRESULT)0x800300FBL)

//(0x800300fcL)STG_E_INVALIDNAME
//
// MessageId: STG_E_INVALIDNAME
//
// MessageText:
//
// The name is not valid.
//
#define STG_E_INVALIDNAME                ((HRESULT)0x800300FCL)

//(0x80030104L)STG_E_OLDFORMAT
//
// MessageId: STG_E_OLDFORMAT
//
// MessageText:
//
// The compound file was produced with an incompatible version of storage.
//
#define STG_E_OLDFORMAT                  ((HRESULT)0x80030104L)

//(0x80030105L)STG_E_OLDDLL
//
// MessageId: STG_E_OLDDLL
//
// MessageText:
//
// The compound file was produced with an incompatible version of storage.
//
#define STG_E_OLDDLL                     ((HRESULT)0x80030105L)

#endif //MESSAGESANDHEADERS
#endif // _OLEDBERR_H_
