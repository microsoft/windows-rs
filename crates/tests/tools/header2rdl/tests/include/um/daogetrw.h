/************************************************************************
**  D A O G E T R W . H                                                 *
**                                                                      *
**  GetRows interface                                                   *
**                                                                      *
**  Warning: This file is copied from cdaost.h. Changes must be made    *
**           in both files                                              *
**                                                                      *
**                                                                      *
*************************************************************************
** Copyright (C) 1996 by Microsoft Corporation                          *
**         All Rights Reserved                                          *
************************************************************************/

#if !defined (_DAOGETRW_H_)
#define _DAOGETRW_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
	Enumerations
*/
typedef enum
	{
	DAOCOLKIND_IND = 0,
	DAOCOLKIND_STR,
	DAOCOLKIND_WSTR
	} DAOCOLKIND;

typedef enum
	{
	DAO_I2 = 0,
	DAO_I4,
	DAO_R4,
	DAO_R8,
	DAO_CURRENCY,
	DAO_DATE,
	DAO_BOOL,
	DAO_BSTR,
	DAO_LPSTR,
	DAO_LPWSTR,
	DAO_BLOB,
	DAO_BYTES,
	DAO_CHAR,
	DAO_WCHAR,
	DAO_ANYVARIANT,
	DAO_BOOKMARK,
	DAO_BYTE,
	DAO_GUID,
	DAO_DATATYPEMAX
	} DAODATATYPE;

/*
	Macros
*/
#define DAO_NOINDICATOR 0xffffffff
#define DAO_NULL        0xffffffff
#define DAO_CANTCOERCE  0xfffffffc
#define DAO_NOMAXLENGTH 0x00000000

#define DAOROWFETCH_CALLEEALLOCATES     0x00000001
#define DAOROWFETCH_DONTADVANCE         0x00000002
#define DAOROWFETCH_FORCEREFRESH        0x00000004
#define DAOROWFETCH_BINDABSOLUTE        0x00000008
#define DAOROWFETCH_ODBCNEXT			0x00000010

#define DAOBINDING_DIRECT               0x00000001
#define DAOBINDING_VARIANT              0x00000002
#define DAOBINDING_CALLBACK             0x00000004

/*
	Structures
*/
typedef struct
	{
	DWORD           dwKind;
	union
		{
		LONG        ind;
		LPCSTR      lpstr;
		LPCWSTR		lpwstr;
		};
	} DAOCOLUMNID;
typedef DAOCOLUMNID *LPDAOCOLUMNID;

// Callback for binding
EXTERN_C typedef HRESULT (STDAPICALLTYPE *LPDAOBINDFUNC)(ULONG cb, DWORD dwUser, LPVOID *ppData);
#define DAOBINDINGFUNC(f)   STDAPI f (ULONG cb, DWORD dwUser, LPVOID *ppData)

typedef struct
	{
	DAOCOLUMNID     columnID;
	ULONG           cbDataOffset;
	ULONG           cbMaxLen;
	ULONG           cbInfoOffset;
	DWORD           dwBinding;
	DWORD           dwDataType;
	DWORD           dwUser;
	} DAOCOLUMNBINDING;
typedef DAOCOLUMNBINDING *LPDAOCOLUMNBINDING;

typedef struct
	{
	ULONG           cRowsRequested;
	DWORD           dwFlags;
	LPVOID          pData;
	LPVOID          pVarData;
	ULONG           cbVarData;
	ULONG           cRowsReturned;
	} DAOFETCHROWS;
typedef DAOFETCHROWS *LPDAOFETCHROWS;

/*
	New Errors

	**NOTE: OLE standard ids to be determined.
*/
#define S_BUFFERTOOSMALL    MAKE_SCODE(SEVERITY_SUCCESS,    FACILITY_ITF,   0x1000)
#define S_ENDOFCURSOR       MAKE_SCODE(SEVERITY_SUCCESS,    FACILITY_ITF,   0x1001)
#define S_SILENTCANCEL      MAKE_SCODE(SEVERITY_SUCCESS,    FACILITY_ITF,   0x1002)
#define S_RECORDDELETED     MAKE_SCODE(SEVERITY_SUCCESS,    FACILITY_ITF,   0x1003)

#define E_ROWTOOSHORT       MAKE_SCODE(SEVERITY_ERROR,      FACILITY_ITF,   0x1000)
#define E_BADBINDINFO       MAKE_SCODE(SEVERITY_ERROR,      FACILITY_ITF,   0x1001)
#define E_COLUMNUNAVAILABLE MAKE_SCODE(SEVERITY_ERROR,      FACILITY_ITF,   0x1002)


/*
	Interfaces
*/
#undef INTERFACE
#define INTERFACE ICDAORecordset
DECLARE_INTERFACE_(ICDAORecordset, IDispatch)
	{
	STDMETHOD(GetRows)          (THIS_ LONG cRowsToSkip, LONG cCol, LPDAOCOLUMNBINDING prgBndCol, ULONG cbRowLen, LPDAOFETCHROWS pFetchRows) PURE;
	STDMETHOD(SetNotify)		(THIS_ REFIID riid, BOOL fNotify);
	STDMETHOD(GetNotify)		(THIS_ REFIID riid, BOOL *fNotify);
	STDMETHOD(OnBeforeNotify)	(THIS_ REFIID riid, DWORD cat, DWORD rsn, VARIANT v1, VARIANT v2);
	STDMETHOD(OnAfterNotify)	(THIS_ REFIID riid, DWORD cat, DWORD rsn, VARIANT v1, VARIANT v2, HRESULT hr);
	STDMETHOD(PutLock)			(THIS_ BOOL f);
	STDMETHOD(GetLock)			(THIS_ BOOL *f);
	STDMETHOD(AddGetRowsErr)    (THIS_ HRESULT hr) PURE;
	};

//Get rows errors
#define errVtoFetchBuffTooSmall	-30028	//3640	The fetch buffer was too small for the amount of data you requested.
#define errVtoEOFDuringFetch	-30029	//3641	There are fewer records remaining in the recordset than you requested.
#define errVtoSilentCancel		-30030	//3642	A cancel was performed on the operation.
#define errVtoRecordDeleted		-30031	//3643	One of the records in the recordset was deleted by another process.
#define errVtoRowLenTooSmall	-30032	//3646	The specified row length is shorter than the sum of the column lengths.
#define errVtoBadBindInfo		-30033	//3645	One of the binding parameters is incorrect.
#define errVtoColumnMissing		-30034	//3647	A column requested is not being returned to the recordset.


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _DAOGETRW_H_
