//  Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.
/* -----------------------------------------------------------------
 * Microsoft Distributed Transaction Coordinator
 * Microsoft Corporation, 1995.
 *
 * File : xa.h 
 * 
 * Contents : This file is derived from xa.h as it appears in 
 * "Distributed Transaction Processing: The XA Specification", 
 * November 93, X/Open Company Limited.
 *
 * DO NOT REDISTRIBUTE
 */

/*
 * Start of xa.h header
 *
 * Define a symbol to prevent multiple inclusion of this header file
 */

#ifndef XA_H
#define XA_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * Transaction branch identification: XID and NULLXID:
 */
#define XIDDATASIZE		128			/* size in bytes */
#define MAXGTRIDSIZE    64  		/* maximum size in bytes of gtrid */
#define MAXBQUALSIZE    64  		/* maximum size in bytes of bqual */

#ifndef _XID_T_DEFINED
#define _XID_T_DEFINED
struct xid_t
{
	long formatID;					/* format identifier */
	long gtrid_length;				/* value not to exceed 64 */
	long bqual_length;				/* value not to exceed 64 */
	char data[XIDDATASIZE];
};
#endif

typedef struct xid_t XID;
/*
 * A value of -1 in formatID means that the XID is null.
 */
/*
 * Declarations of routines by which RMs call TMs:
 */

int __cdecl ax_reg(int, XID *, long);
int __cdecl ax_unreg(int, long);


/*
 * XA Switch Data Structure
 */
#define RMNAMESZ	32									/* length of resource manager name, */
														/* including the null terminator */
#define MAXINFOSIZE 256									/* maximum size in bytes of xa_info strings, */
														/* including the null terminator */

#ifndef _XA_SWITCH_T_DEFINED
#define _XA_SWITCH_T_DEFINED
struct xa_switch_t
{
  char name[RMNAMESZ];									/* name of resource manager */
  long flags;											/* resource manager specific options */
  long version;											/* must be 0 */
  int (__cdecl *xa_open_entry)(char *, int, long);		/* xa_open function pointer */
  int (__cdecl *xa_close_entry)(char *, int, long);		/* xa_close function pointer*/
  int (__cdecl *xa_start_entry)(XID *, int, long);		/* xa_start function pointer */
  int (__cdecl *xa_end_entry)(XID *, int, long);		/* xa_end function pointer */
  int (__cdecl *xa_rollback_entry)(XID *, int, long);	/* xa_rollback function pointer */
  int (__cdecl *xa_prepare_entry)(XID *, int, long);	/* xa_prepare function pointer */
  int (__cdecl *xa_commit_entry)(XID *, int, long);		/* xa_commit function pointer */
  int (__cdecl *xa_recover_entry)(XID *, long, int, long);
														/* xa_recover function pointer*/
  int (__cdecl *xa_forget_entry)(XID *, int, long);		/* xa_forget function pointer */
  int (__cdecl *xa_complete_entry)(int *, int *, int, long);
														/* xa_complete function pointer */
};

typedef struct xa_switch_t xa_switch_t;
#endif

/*
 * Flag definitions for the RM switch
 */
#define TMNOFLAGS		0x00000000L						/* no resource manager features selected */
#define TMREGISTER		0x00000001L						/* resource manager dynamically registers */
#define TMNOMIGRATE		0x00000002L						/* resource manager does not support association migration */
#define TMUSEASYNC		0x00000004L						/* resource manager supports asynchronous operations */
/*
 * Flag definitions for xa_ and ax_ routines
 */
/* use TMNOFLAGS, defined above, when not specifying other flags */
#define TMASYNC			0x80000000L						/* perform routine asynchronously */
#define TMONEPHASE		0x40000000L						/* caller is using one-phase commit optimisation */
#define TMFAIL			0x20000000L						/* dissociates caller and marks transaction branch rollback-only */
#define TMNOWAIT		0x10000000L						/* return if blocking condition exists */
#define TMRESUME		0x08000000L						/* caller is resuming association with suspended transaction branch */
#define TMSUCCESS		0x04000000L						/* dissociate caller from transaction branch */
#define TMSUSPEND		0x02000000L						/* caller is suspending, not ending, association */
#define TMSTARTRSCAN	0x01000000L						/* start a recovery scan */
#define TMENDRSCAN		0x00800000L						/* end a recovery scan */
#define TMMULTIPLE		0x00400000L						/* wait for any asynchronous operation */
#define TMJOIN			0x00200000L						/* caller is joining existing transaction branch */
#define TMMIGRATE		0x00100000L						/* caller intends to perform migration */
/*
 * ax_() return codes (transaction manager reports to resource manager)
 */
#define TM_JOIN			2								/* caller is joining existing transaction branch */
#define TM_RESUME		1								/* caller is resuming association with suspended transaction branch */
#define TM_OK			0								/* normal execution */
#define TMER_TMERR		(-1)							/* an error occurred in the transaction manager */
#define TMER_INVAL		(-2)							/* invalid arguments were given */
#define TMER_PROTO		(-3)							/* routine invoked in an improper context */
/*
 * xa_() return codes (resource manager reports to transaction manager)
 */
#define XA_RBBASE		100								/* The inclusive lower bound of the rollback codes */
#define XA_RBROLLBACK	XA_RBBASE						/* The rollback was caused by an unspecified reason */
#define XA_RBCOMMFAIL	XA_RBBASE+1						/* The rollback was caused by a communication failure */
#define XA_RBDEADLOCK	XA_RBBASE+2						/* A deadlock was detected */
#define XA_RBINTEGRITY	XA_RBBASE+3						/* A condition that violates the integrity of the resources was detected */
#define XA_RBOTHER		XA_RBBASE+4						/* The resource manager rolled back the transaction branch for a reason not on this list */
#define XA_RBPROTO		XA_RBBASE+5						/* A protocol error occurred in the resource manager */
#define XA_RBTIMEOUT	XA_RBBASE+6						/* A transaction branch took too long */
#define XA_RBTRANSIENT	XA_RBBASE+7						/* May retry the transaction branch */
#define XA_RBEND		XA_RBTRANSIENT					/* The inclusive upper bound of the rollback codes */

#define XA_NOMIGRATE	9								/* resumption must occur where suspension occurred */
#define XA_HEURHAZ		8								/* the transaction branch may have been heuristically completed */
#define XA_HEURCOM		7								/* the transaction branch has been heuristically committed */
#define XA_HEURRB		6								/* the transaction branch has been heuristically rolled back */
#define XA_HEURMIX		5								/* the transaction branch has been heuristically committed and rolled back */
#define XA_RETRY		4								/* routine returned with no effect and may be re-issued */
#define XA_RDONLY		3								/* the transaction branch was read-only and has been committed */
#define XA_OK			0								/* normal execution */
#define XAER_ASYNC		(-2)							/* asynchronous operation already outstanding */
#define XAER_RMERR		(-3)							/* a resource manager error occurred in the transaction branch */
#define XAER_NOTA		(-4)							/* the XID is not valid */
#define XAER_INVAL		(-5)							/* invalid arguments were given */
#define XAER_PROTO		(-6)							/* routine invoked in an improper context */
#define XAER_RMFAIL		(-7)							/* resource manager unavailable */
#define XAER_DUPID		(-8)							/* the XID already exists */
#define XAER_OUTSIDE	(-9)							/* resource manager doing work outside */
														/* global transaction */
/*
 * XA entry point type definitions:
 */

typedef int (__cdecl *XA_OPEN_EPT)(char *, int, long);	/* xa_open entry point */
typedef int (__cdecl *XA_CLOSE_EPT)(char *, int, long);	/* xa_close entry point*/
typedef int (__cdecl *XA_START_EPT)(XID *, int, long);	/* xa_start entry point */
typedef int (__cdecl *XA_END_EPT)(XID *, int, long);	/* xa_end entry point */
typedef int (__cdecl *XA_ROLLBACK_EPT)(XID *, int, long);
														/* xa_rollback entry point */
typedef int (__cdecl *XA_PREPARE_EPT)(XID *, int, long);/* xa_prepare entry point */
typedef int (__cdecl *XA_COMMIT_EPT)(XID *, int, long);	/* xa_commit entry point */
typedef int (__cdecl *XA_RECOVER_EPT)(XID *, long, int, long);
														/* xa_recover entry point*/
typedef int (__cdecl *XA_FORGET_EPT)(XID *, int, long);	/* xa_forget entry point */
typedef int (__cdecl *XA_COMPLETE_EPT)(int *, int *, int, long);
														/* xa_complete entry point */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* ifndef XA_H */
/*
 * End of xa.h header
 */

