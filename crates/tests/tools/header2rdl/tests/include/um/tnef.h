/*
 *	T N E F . H
 *
 *
 *	This file contains structure and function definitions for the
 *	MAPI implementation of the Transport Neutral Encapsilation Format
 *	used by MAPI providers for the neutral serialization of a MAPI
 *	message.  This implementation sits on top of the IStream object as
 *	documented in the OLE 2 Specs.
 *
 *  Copyright 1986-1999 Microsoft Corporation. All Rights Reserved.
 */

#ifndef TNEF_H
#define TNEF_H

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#ifndef WIN_NOEXCEPT
	#ifdef __cplusplus
		#if _MSC_VER >= 1900
			#define WIN_NOEXCEPT noexcept
		#else
			#define WIN_NOEXCEPT throw()
		#endif
	#else
		#define WIN_NOEXCEPT
	#endif
#endif


#ifndef BEGIN_INTERFACE
#define BEGIN_INTERFACE
#endif

/* ------------------------------------ */
/* TNEF Problem and TNEF Problem Arrays */
/* ------------------------------------ */

typedef struct _STnefProblem
{
	ULONG	ulComponent;
	ULONG	ulAttribute;
	ULONG	ulPropTag;
	SCODE	scode;
} STnefProblem;

typedef struct _STnefProblemArray
{
	ULONG			cProblem;
	STnefProblem	aProblem[MAPI_DIM];
} STnefProblemArray, FAR * LPSTnefProblemArray;

#define CbNewSTnefProblemArray(_cprob) \
	(offsetof(STnefProblemArray,aProblem) + (_cprob)*sizeof(STnefProblem))
#define CbSTnefProblemArray(_lparray) \
	(offsetof(STnefProblemArray,aProblem) + \
	(UINT) ((_lparray)->cProblem*sizeof(STnefProblem)))

/* Pointers to TNEF Interface ---------------------------------------- */

DECLARE_MAPI_INTERFACE_PTR(ITnef, LPITNEF);

/*	OpenTNEFStream */

#define	TNEF_DECODE					((ULONG) 0)
#define TNEF_ENCODE					((ULONG) 2)

#define TNEF_PURE					((ULONG) 0x00010000)
#define TNEF_COMPATIBILITY			((ULONG) 0x00020000)
#define TNEF_BEST_DATA				((ULONG) 0x00040000)
#define TNEF_COMPONENT_ENCODING		((ULONG) 0x80000000)

/*	AddProps, ExtractProps */

#define TNEF_PROP_INCLUDE			((ULONG) 0x00000001)
#define TNEF_PROP_EXCLUDE			((ULONG) 0x00000002)
#define	TNEF_PROP_CONTAINED			((ULONG) 0x00000004)
#define	TNEF_PROP_MESSAGE_ONLY		((ULONG) 0x00000008)
#define TNEF_PROP_ATTACHMENTS_ONLY	((ULONG) 0x00000010)
#define	TNEF_PROP_CONTAINED_TNEF	((ULONG) 0x00000040)

/*	FinishComponent */

#define TNEF_COMPONENT_MESSAGE		((ULONG) 0x00001000)
#define TNEF_COMPONENT_ATTACHMENT	((ULONG) 0x00002000)

#define MAPI_ITNEF_METHODS(IPURE)										\
	MAPIMETHOD(AddProps)												\
		(THIS_	ULONG						ulFlags,					\
				ULONG						ulElemID,					\
				LPVOID						lpvData,					\
				LPSPropTagArray				lpPropList) IPURE;			\
	MAPIMETHOD(ExtractProps)											\
		(THIS_	ULONG						ulFlags,					\
				LPSPropTagArray				lpPropList,					\
				LPSTnefProblemArray FAR *	lpProblems) IPURE;			\
	MAPIMETHOD(Finish)													\
		(THIS_	ULONG						ulFlags,					\
				WORD FAR *					lpKey,						\
				LPSTnefProblemArray FAR *	lpProblems) IPURE;			\
	MAPIMETHOD(OpenTaggedBody)											\
		(THIS_	LPMESSAGE					lpMessage,					\
				ULONG						ulFlags,					\
				LPSTREAM FAR *				lppStream) IPURE;			\
	MAPIMETHOD(SetProps)												\
		(THIS_	ULONG						ulFlags,					\
				ULONG						ulElemID,					\
				ULONG						cValues,					\
				LPSPropValue				lpProps) IPURE;				\
	MAPIMETHOD(EncodeRecips)											\
		(THIS_	ULONG						ulFlags,					\
				LPMAPITABLE					lpRecipientTable) IPURE;	\
	MAPIMETHOD(FinishComponent)											\
		(THIS_	ULONG						ulFlags,					\
				ULONG						ulComponentID,				\
				LPSPropTagArray				lpCustomPropList,			\
				LPSPropValue				lpCustomProps,				\
				LPSPropTagArray				lpPropList,					\
				LPSTnefProblemArray FAR *	lpProblems) IPURE;			\

#undef		 INTERFACE
#define		 INTERFACE  ITnef
DECLARE_MAPI_INTERFACE_(ITnef, IUnknown)
{
	BEGIN_INTERFACE
	MAPI_IUNKNOWN_METHODS(PURE)
	MAPI_ITNEF_METHODS(PURE)
};

_Check_return_
STDMETHODIMP OpenTnefStream(
	LPVOID				lpvSupport,
	LPSTREAM			lpStream,
	_In_ LPTSTR			lpszStreamName,
	ULONG				ulFlags,
	LPMESSAGE			lpMessage,
	WORD				wKeyVal,
	LPITNEF FAR *		lppTNEF) WIN_NOEXCEPT;

typedef 
_Check_return_
HRESULT (STDMETHODCALLTYPE FAR * LPOPENTNEFSTREAM) (
	LPVOID				lpvSupport,
	LPSTREAM			lpStream,
	_In_ LPTSTR			lpszStreamName,
	ULONG				ulFlags,
	LPMESSAGE			lpMessage,
	WORD				wKeyVal,
	LPITNEF FAR *		lppTNEF);

_Check_return_
STDMETHODIMP OpenTnefStreamEx(
	LPVOID				lpvSupport,
	LPSTREAM			lpStream,
	_In_ LPTSTR			lpszStreamName,
	ULONG				ulFlags,
	LPMESSAGE			lpMessage,
	WORD				wKeyVal,
	LPADRBOOK			lpAdressBook,
	LPITNEF FAR *		lppTNEF) WIN_NOEXCEPT;

typedef HRESULT (STDMETHODCALLTYPE FAR * LPOPENTNEFSTREAMEX) (
	LPVOID				lpvSupport,
	LPSTREAM			lpStream,
	_In_ LPTSTR			lpszStreamName,
	ULONG				ulFlags,
	LPMESSAGE			lpMessage,
	WORD				wKeyVal,
	LPADRBOOK			lpAdressBook,
	LPITNEF FAR *		lppTNEF);

STDMETHODIMP GetTnefStreamCodepage (
	LPSTREAM			lpStream,
	ULONG FAR *			lpulCodepage,
	ULONG FAR *			lpulSubCodepage) WIN_NOEXCEPT;

typedef HRESULT (STDMETHODCALLTYPE FAR * LPGETTNEFSTREAMCODEPAGE) (
	LPSTREAM			lpStream,
	ULONG FAR *			lpulCodepage,
	ULONG FAR *			lpulSubCodepage);

#define OPENTNEFSTREAM "OpenTnefStream"
#define OPENTNEFSTREAMEX "OpenTnefStreamEx"
#define GETTNEFSTREAMCODEPAGE "GetTnefStreamCodePage"

/* -------------------------- */
/* TNEF Signature and Version */
/* -------------------------- */

#define MAKE_TNEF_VERSION(_mj,_mn)	(((ULONG)(0x0000FFFF & _mj) << 16) | (ULONG)(0x0000FFFF & _mn))
#define TNEF_SIGNATURE	((ULONG) 0x223E9F78)
#define TNEF_VERSION	((ULONG) MAKE_TNEF_VERSION(1,0))


/* ------------------------------------------- */
/* TNEF Down-level Attachment Types/Structures */
/* ------------------------------------------- */

typedef WORD ATYP;
enum { atypNull, atypFile, atypOle, atypPicture, atypMax };

#define MAC_BINARY	((DWORD) 0x00000001)

#include <pshpack1.h>
typedef struct _renddata
{
	ATYP	atyp;
	ULONG	ulPosition;
	WORD	dxWidth;
	WORD	dyHeight;
	DWORD	dwFlags;

} RENDDATA, *PRENDDATA;
#include <poppack.h>


/* ----------------------------------- */
/* TNEF Down-level Date/Time Structure */
/* ----------------------------------- */

#include <pshpack1.h>
typedef struct _dtr
{
	WORD	wYear;
	WORD	wMonth;
	WORD	wDay;
	WORD	wHour;
	WORD	wMinute;
	WORD	wSecond;
	WORD	wDayOfWeek;

} DTR;
#include <poppack.h>


/* ----------------------------- */
/* TNEF Down-level Message Flags */
/* ----------------------------- */

#define fmsNull			((BYTE) 0x00)
#define fmsModified		((BYTE) 0x01)
#define fmsLocal		((BYTE) 0x02)
#define fmsSubmitted	((BYTE) 0x04)
#define fmsRead			((BYTE) 0x20)
#define fmsHasAttach	((BYTE) 0x80)


/* ----------------------------------------- */
/* TNEF Down-level Triple Address Structures */
/* ----------------------------------------- */

#define	trpidNull					((WORD) 0x0000)
#define	trpidUnresolved				((WORD) 0x0001)
#define	trpidResolvedNSID			((WORD) 0x0002)
#define	trpidResolvedAddress		((WORD) 0x0003)
#define	trpidOneOff					((WORD) 0x0004)
#define	trpidGroupNSID				((WORD) 0x0005)
#define	trpidOffline				((WORD) 0x0006)
#define	trpidIgnore					((WORD) 0x0007)
#define	trpidClassEntry				((WORD) 0x0008)
#define	trpidResolvedGroupAddress	((WORD) 0x0009)
typedef struct _trp
{
	WORD	trpid;
	WORD	cbgrtrp;
	WORD	cch;
	WORD	cbRgb;

} TRP, *PTRP, *PGRTRP, FAR * LPTRP;
#define CbOfTrp(_p)		(sizeof(TRP) + (_p)->cch + (_p)->cbRgb)
#define LpszOfTrp(_p)	((LPSTR)(((LPTRP) (_p)) + 1))
#define LpbOfTrp(_p)	(((LPBYTE)(((LPTRP)(_p)) + 1)) + (_p)->cch)
#define LptrpNext(_p)	((LPTRP)((LPBYTE)(_p) + CbOfTrp(_p)))

typedef DWORD XTYPE;
#define xtypeUnknown	((XTYPE) 0)
#define xtypeInternet	((XTYPE) 6)

#define cbDisplayName	41
#define cbEmailName		11
#define cbSeverName		12
typedef struct _ADDR_ALIAS
{
	char	rgchName[cbDisplayName];
	char	rgchEName[cbEmailName];
	char	rgchSrvr[cbSeverName];
	ULONG	dibDetail;
	WORD	type;

} ADDRALIAS, FAR * LPADDRALIAS;
#define cbALIAS sizeof(ALIAS)

#define cbTYPE				16
#define cbMaxIdData			200
typedef struct _NSID
{
	DWORD	dwSize;
	unsigned char	uchType[cbTYPE];
	XTYPE	xtype;
	LONG	lTime;

	union
	{
		ADDRALIAS	alias;
		char		rgchInterNet[1];

	} address;

} NSID, * LPNSID;
#define cbNSID sizeof(NSID)


/* -------------------------- */
/* TNEF Down-level Priorities */
/* -------------------------- */

#define prioLow		3
#define prioNorm	2
#define prioHigh	1


/* ------------------------------------- */
/* TNEF Down-level Attributes/Properties */
/* ------------------------------------- */

#define atpTriples		((WORD) 0x0000)
#define	atpString		((WORD) 0x0001)
#define	atpText			((WORD) 0x0002)
#define	atpDate			((WORD) 0x0003)
#define	atpShort		((WORD) 0x0004)
#define	atpLong			((WORD) 0x0005)
#define	atpByte			((WORD) 0x0006)
#define	atpWord			((WORD) 0x0007)
#define	atpDword		((WORD) 0x0008)
#define atpMax			((WORD) 0x0009)

#define LVL_MESSAGE		((BYTE) 0x01)
#define LVL_ATTACHMENT	((BYTE) 0x02)

#define ATT_ID(_att)				((WORD) ((_att) & 0x0000FFFF))
#define ATT_TYPE(_att)				((WORD) (((_att) >> 16) & 0x0000FFFF))
#define ATT(_atp, _id)				((((DWORD) (_atp)) << 16) | ((WORD) (_id)))

#define attNull						ATT( 0,				0x0000)
#define attFrom						ATT( atpTriples,	0x8000)	/* PR_ORIGINATOR_RETURN_ADDRESS */
#define attSubject					ATT( atpString,		0x8004) /* PR_SUBJECT */
#define attDateSent					ATT( atpDate,		0x8005) /* PR_CLIENT_SUBMIT_TIME */
#define attDateRecd					ATT( atpDate,		0x8006)	/* PR_MESSAGE_DELIVERY_TIME */
#define attMessageStatus			ATT( atpByte,		0x8007) /* PR_MESSAGE_FLAGS */
#define attMessageClass				ATT( atpWord,		0x8008) /* PR_MESSAGE_CLASS */
#define attMessageID				ATT( atpString,		0x8009) /* PR_MESSAGE_ID */
#define attParentID					ATT( atpString,		0x800A)	/* PR_PARENT_ID */
#define attConversationID			ATT( atpString,		0x800B) /* PR_CONVERSATION_ID */
#define attBody						ATT( atpText,		0x800C) /* PR_BODY */
#define attPriority					ATT( atpShort,		0x800D)	/* PR_IMPORTANCE */
#define attAttachData				ATT( atpByte,		0x800F)	/* PR_ATTACH_DATA_xxx */
#define attAttachTitle				ATT( atpString,		0x8010) /* PR_ATTACH_FILENAME */
#define attAttachMetaFile			ATT( atpByte,		0x8011)	/* PR_ATTACH_RENDERING */
#define attAttachCreateDate			ATT( atpDate,		0x8012) /* PR_CREATION_TIME */
#define attAttachModifyDate			ATT( atpDate,		0x8013) /* PR_LAST_MODIFICATION_TIME */
#define attDateModified				ATT( atpDate,		0x8020) /* PR_LAST_MODIFICATION_TIME */
#define attAttachTransportFilename	ATT( atpByte,		0x9001) /* PR_ATTACH_TRANSPORT_NAME */
#define attAttachRenddata			ATT( atpByte,		0x9002)
#define attMAPIProps				ATT( atpByte,		0x9003)
#define attRecipTable				ATT( atpByte,		0x9004) /* PR_MESSAGE_RECIPIENTS */
#define attAttachment				ATT( atpByte,		0x9005)
#define attTnefVersion				ATT( atpDword,		0x9006)
#define attOemCodepage				ATT( atpByte,		0x9007)
#define attOriginalMessageClass		ATT( atpWord,		0x0006) /* PR_ORIG_MESSAGE_CLASS */

#define attOwner					ATT( atpByte,		0x0000) /* PR_RCVD_REPRESENTING_xxx  or
														           PR_SENT_REPRESENTING_xxx */
#define attSentFor					ATT( atpByte,		0x0001) /* PR_SENT_REPRESENTING_xxx */
#define attDelegate					ATT( atpByte,		0x0002)	/* PR_RCVD_REPRESENTING_xxx */
#define attDateStart				ATT( atpDate,		0x0006) /* PR_DATE_START */
#define attDateEnd					ATT( atpDate,		0x0007) /* PR_DATE_END */
#define attAidOwner					ATT( atpLong,		0x0008) /* PR_OWNER_APPT_ID */
#define attRequestRes				ATT( atpShort,		0x0009) /* PR_RESPONSE_REQUESTED */

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif	/*	defined TNEF_H */
