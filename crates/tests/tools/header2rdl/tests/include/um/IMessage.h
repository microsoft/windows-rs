/*
 *	I M E S S A G E . H
 *
 *	External definitions for MAPI's IMessage-on-IStorage facility
 *
 *  Copyright 1986-1999 Microsoft Corporation. All Rights Reserved.
 */

#ifndef _IMESSAGE_H_
#define _IMESSAGE_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"
{
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

typedef struct _MSGSESS		FAR * LPMSGSESS;

/*	Typedef of optional callback routine to be called on last release of
 *	top-level messages opened with OpenIMsgOnIStg
 */
typedef void (STDAPICALLTYPE MSGCALLRELEASE)(
	ULONG 		ulCallerData,
	LPMESSAGE	lpMessage );

/* DLL Entry Points (found in mapiu.dll) */

/* OpenIMsgSession
 * CloseIMsgSession
 *
 * These entry points allow the caller to "wrap" the creation of messages
 * inside a session, so that when the session is closed, all messages
 * created within that session are closed as well. Use of IMSG sessions
 * is optional. If OpenIMsgOnIStg is called with a NULL for the lpmsgsess
 * parameter, the message is created independent of any session, and has
 * no way to be shutdown. If the caller forgets to release the message, or
 * to release open tables within the message, the memory will be leaked until
 * the external application terminates.
 */

STDAPI_(SCODE) OpenIMsgSession(
	LPMALLOC		lpMalloc,			/* -> Co malloc object			*/
	ULONG			ulFlags,			/* reserved. Must be zero.		*/
	LPMSGSESS FAR	*lppMsgSess );		/* <- message session object	*/

STDAPI_(void) CloseIMsgSession(
	LPMSGSESS		lpMsgSess );		/* -> message session object	*/

/*	OpenIMsgOnIStg - Main entry point
 *
 *	NOTE 1:  The IStg must be opened with STGM_TRANSACTED if STGM_READWRITE
 *	is specified.  Since messages don't support a write only mode, IMessage
 *	doesn't allow a storage object opened in write only mode. If the storage
 *	is opened STGM_READ, then STGM_TRANSACTED is NOT required.
 *
 *	NOTE 2:  The lpMapiSup parameter is optional.  If supplied then IMessage
 *	will support the MAPI_DIALOG and ATTACH_DIALOG flags (by calling
 *	support method: DoMCDialog) on CopyTo and DeleteAttach methods.
 *	If lpMapiSup is not supplied (i.e. passed 0) then dialog flags will be
 *	ignored.  If supplied then ModifyRecipients will attempt to convert
 *	short term entryids to long term entryids (by calling support method
 *	OpenAddressBook and calls on the returned object).  If not supplied
 *	then short term entryid's will be stored without conversion.
 *
 *	NOTE 3:  The lpfMsgCallRelease parameter is optional.  If supplied then
 *	IMessage will call the routine when the last release on (the toplevel only)
 *	message is called.  It is intended to allow the callee to free the IStorage
 *	that contains the message.  IMessage will not use the IStorage object after
 *	making this call.
 *
 *	NOTE 4:  Behavior of multiple opens of sub-objects (Attachments, Streams,
 *	Storages, Messages, etc.) within a message is deliberately undefined in
 *	MAPI.  This implementation allows them, but will do it by AddRef'ing the
 *	existing open and returning it to the caller of OpenAttachment or
 *	OpenProperty.  This means that whatever access mode the first open on a
 *	specific Attachment or Property had is what all others will get regardless
 *	of what the subsequent opens asked for.
 *
 *	NOTE 5:  There is currently one flag defined for use with the ulFlags
 *	parameter. The IMSG_NO_ISTG_COMMIT flag controls whether the commit
 *	method of IStorage is called when the client calls SaveChanges on the
 *	IMessage object. Some clients of IMessage may wish to commit the IStorage
 *	themselves after writing additional data to the storage (beyond what
 *	IMessage itself writes). To aid in this, the IMessage implementation
 *	guarantees to name all sub-storages starting with "__". Therefore,
 *	if the client keeps its names out of that namespace, there will be no
 *	accidental collisions.
 *
 *	WARNING:	
 *
 *	This implementation of IMessage will support OpenProperty w/MAPI_CREATE
 *	where the source interface is IID_IStorage if the property id is
 *	'PR_ATTACH_DATA'.  Once this has been done, the caller has an IStorage
 *	interface on this property.  This is ok and should allow for
 *	easier implementation of OLE 2.0 Server functionality.  However, if you
 *	pass in the new IStorage ptr (to the attachment data) through the
 *	OpenIMsgOnIStg entry point and then proceed to release things in the
 *	wrong order we will make no attempt to behave in a predictable fashion.
 *	Keep in mind that the correct method for placing a message into an
 *	attachment is to call OpenProperty where the source interface is
 *	IID_IMessage.  The IStorage interface is supported to allow an easy way
 *	to stick a WWord doc. into an attachment w/o converting to/from IStream.
 *
 */
STDAPI_(SCODE) OpenIMsgOnIStg(
	LPMSGSESS		lpMsgSess,			/* -> message session obj (optional) */
	LPALLOCATEBUFFER lpAllocateBuffer,	/* -> AllocateBuffer memory routine  */
	LPALLOCATEMORE 	lpAllocateMore, 	/* -> AllocateMore memory routine    */
	LPFREEBUFFER	lpFreeBuffer, 		/* -> FreeBuffer memory routine      */
	LPMALLOC		lpMalloc,			/* -> Co malloc object				 */
	LPVOID			lpMapiSup,			/* -> MAPI Support Obj (optional)    */
	LPSTORAGE 		lpStg, 				/* -> open IStorage containing msg   */
	MSGCALLRELEASE FAR *lpfMsgCallRelease,	/* -> release callback rtn (opt) */
	ULONG			ulCallerData,		/* caller data returned in callback  */
	ULONG			ulFlags,			/* -> flags (controls istg commit)   */
	LPMESSAGE		FAR *lppMsg ) WIN_NOEXCEPT;	/* <- open message object			 */

#define IMSG_NO_ISTG_COMMIT		((ULONG) 0x00000001)


/* NOTE: Property Attributes are specific to this IMessage on IStorage 		*/
/* implementation and are not a part of standard MAPI 1.0 property methods 	*/

/* Property Attributes */

#define PROPATTR_MANDATORY		((ULONG) 0x00000001)
#define PROPATTR_READABLE		((ULONG) 0x00000002)
#define PROPATTR_WRITEABLE		((ULONG) 0x00000004)

#define PROPATTR_NOT_PRESENT	((ULONG) 0x00000008)

/* Attribute Array */

typedef struct _SPropAttrArray
{
	ULONG	cValues;							
	ULONG	aPropAttr[MAPI_DIM];
} SPropAttrArray, FAR * LPSPropAttrArray;

#define CbNewSPropAttrArray(_cattr) \
	(offsetof(SPropAttrArray,aPropAttr) + (_cattr)*sizeof(ULONG))
#define CbSPropAttrArray(_lparray) \
	(offsetof(SPropAttrArray,aPropAttr) + \
	(UINT)((_lparray)->cValues)*sizeof(ULONG))

#define SizedSPropAttrArray(_cattr, _name) \
struct _SPropAttrArray_ ## _name \
{ \
	ULONG	cValues; \
	ULONG	aPropAttr[_cattr]; \
} _name



/*	GetAttribIMsgOnIStg - To get attributes on properties
 *
 *	This call is provided because there is no method of IMAPIPropSet to allow
 *	getting attributes.
 */
_Check_return_
STDAPI GetAttribIMsgOnIStg(
	LPVOID					lpObject,
	LPSPropTagArray			lpPropTagArray,
	LPSPropAttrArray FAR 	*lppPropAttrArray ) WIN_NOEXCEPT;

/*	SetAttribIMsgOnIStg - To set attributes on properties
 *
 *	This call is provided because there is no method of IMAPIPropSet to allow
 *	setting of attributes.
 */
_Check_return_
STDAPI SetAttribIMsgOnIStg(
	LPVOID					lpObject,
	LPSPropTagArray			lpPropTags,
	LPSPropAttrArray		lpPropAttrs,
	LPSPropProblemArray FAR	*lppPropProblems ) WIN_NOEXCEPT;

/*	MapStorageSCode - To map an IStorage hResult to a MAPI sCode value
 *
 *	This call is provided for the internal use of PDK components that base
 *	their message implementations on IMessage.  Since these components must
 *	open the storage themselves, there is a common need to map OLE 2.0
 *	Storage error returns to MAPI sCodes.
 *
 *	WARNING:	There is no guarantee that this entry point will exist in
 *	shipped versions of mapiu.dll.
 */
STDAPI_(SCODE) MapStorageSCode( SCODE StgSCode ) WIN_NOEXCEPT;


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif	/* _IMESSAGE_H_ */

