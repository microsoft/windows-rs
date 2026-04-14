/*++

Copyright (c) 1992-1999 Microsoft Corporation

Module Name:

    VDDSVC.H

Abstract:

    Include file contains VDM services provided for installable VDDs.


--*/


/**
 * This file contains VDM services prototype defintions only; their
 * related structures and macros are defined in NT_VDD.H.
 * If we have not included the file yet, include it and set a signal
 * to tell anybody the fact.
**/

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef _NT_VDD
#include "nt_vdd.h"
#define _NT_VDD
#endif


/** Memory Accessing services **/

#define GetVDMAddress(usSeg, usOff) (((ULONG)usSeg << 4) + (ULONG)usOff)

#define GetVDMPointer(Address, Size, Mode) Sim32GetVDMPointer(\
						Address, Size, Mode)

#define FlushVDMPointer(Address, Size, Buffer, Mode) Sim32FlushVDMPointer(\
					   Address, Size, Buffer, Mode)

#define FreeVDMPointer(Address, Size, Buffer, Mode) Sim32FreeVDMPointer(\
					   Address, Size, Buffer, Mode)

/** interrupt simualtion services **/

#define ICA_MASTER 0
#define ICA_SLAVE  1 // Can't PC that one since it has been published forever, but use next define
#define ICA_SUBORDINATE 1

#define VDDSimulateInterrupt(ms, line, count) (call_ica_hw_interrupt)(\
						   ms, line, 1)


/** Register manipulation services **/

#ifndef i386

#define	getEAX()	(ULONG)c_getAX()
#define	getAX() 	c_getAX()
#define	getAL() 	c_getAL()
#define getAH()         c_getAH()
#define	getEBX()	(ULONG)c_getBX()
#define	getBX() 	c_getBX()
#define	getBL() 	c_getBL()
#define	getBH() 	c_getBH()
#define	getECX()	(ULONG)c_getCX()
#define	getCX() 	c_getCX()
#define	getCL() 	c_getCL()
#define	getCH() 	c_getCH()
#define	getEDX()	(ULONG)c_getDX()
#define	getDX() 	c_getDX()
#define	getDL() 	c_getDL()
#define	getDH() 	c_getDH()
#define	getESP()	(ULONG)c_getSP()
#define	getSP() 	c_getSP()
#define	getEBP()	(ULONG)c_getBP()
#define	getBP() 	c_getBP()
#define	getESI()	(ULONG)c_getSI()
#define	getSI() 	c_getSI()
#define	getEDI()	(ULONG)c_getDI()
#define	getDI() 	c_getDI()
#define	getEIP()	(ULONG)c_getIP()
#define	getIP() 	c_getIP()
#define	getCS() 	c_getCS()
#define	getSS() 	c_getSS()
#define	getDS() 	c_getDS()
#define	getES() 	c_getES()
#define	getCF() 	c_getCF()
#define	getPF() 	c_getPF()
#define	getAF() 	c_getAF()
#define	getZF() 	c_getZF()
#define	getSF() 	c_getSF()
#define	getIF() 	c_getIF()
#define	getDF() 	c_getDF()
#define	getOF() 	c_getOF()
#define	getMSW()	c_getMSW()

#define	setEAX(value)	c_setAX((WORD)value)
#define	setAX(value)	c_setAX(value)
#define	setAH(value)	c_setAH(value)
#define	setAL(value)	c_setAL(value)
#define	setEBX(value)	c_setBX((WORD)value)
#define	setBX(value)	c_setBX(value)
#define	setBH(value)	c_setBH(value)
#define	setBL(value)	c_setBL(value)
#define	setECX(value)	c_setCX((WORD)value)
#define	setCX(value)	c_setCX(value)
#define	setCH(value)	c_setCH(value)
#define	setCL(value)	c_setCL(value)
#define	setEDX(value)	c_setDX((WORD)value)
#define	setDX(value)	c_setDX(value)
#define	setDH(value)	c_setDH(value)
#define	setDL(value)	c_setDL(value)
#define	setESP(value)	c_setSP((WORD)value)
#define	setSP(value)	c_setSP(value)
#define	setEBP(value)	c_setBP((WORD)value)
#define	setBP(value)	c_setBP(value)
#define	setESI(value)	c_setSI((WORD)value)
#define	setSI(value)	c_setSI(value)
#define	setEDI(value)	c_setDI((WORD)value)
#define	setDI(value)	c_setDI(value)
#define	setEIP(value)	c_setIP((WORD)value)
#define	setIP(value)	c_setIP(value)
#define	setCS(value)	c_setCS(value)
#define	setSS(value)	c_setSS(value)
#define	setDS(value)	c_setDS(value)
#define	setES(value)	c_setES(value)
#define	setCF(value)	c_setCF(value)
#define	setPF(value)	c_setPF(value)
#define	setAF(value)	c_setAF(value)
#define	setZF(value)	c_setZF(value)
#define	setSF(value)	c_setSF(value)
#define	setIF(value)	c_setIF(value)
#define	setDF(value)	c_setDF(value)
#define	setOF(value)	c_setOF(value)
#define	setMSW(value)	c_setMSW(value)

#endif


/** Real function prototype declarations **/


/** interrupt simulation functions **/

VOID
call_ica_hw_interrupt (
 int ms,
 BYTE line,
 int count
 );

#define Sim32FreeVDMPointer(address, size, buffer, mode) TRUE

/** memory address manipulation functions **/

#ifdef i386

#define Sim32GetVDMPointer(address, size, mode) MGetVdmPointer(address,\
                                                               size, mode)
#define Sim32FlushVDMPointer(addess, size, buffer, mode) TRUE

PBYTE
MGetVdmPointer(
 ULONG	 Address,
 ULONG	 Size,
 UCHAR   ProtectedMode
);

#else

PBYTE
Sim32GetVDMPointer(
 ULONG	 Address,
 ULONG   Size,
 UCHAR   ProtectedMode
);

BOOLEAN
Sim32FlushVDMPointer(
 ULONG	 Address,
 USHORT	 Size,
 PBYTE	 Buffer,
 BOOLEAN ProtectedMode
);

#endif


/** Register manipulation functions **/

#ifdef i386
ULONG   getEAX(VOID);
USHORT  getAX(VOID);
UCHAR   getAL(VOID);
UCHAR   getAH(VOID);
ULONG   getEBX(VOID);
USHORT  getBX(VOID);
UCHAR   getBL(VOID);
UCHAR   getBH(VOID);
ULONG   getECX(VOID);
USHORT  getCX(VOID);
UCHAR   getCL(VOID);
UCHAR   getCH(VOID);
ULONG   getEDX(VOID);
USHORT  getDX(VOID);
UCHAR   getDL(VOID);
UCHAR   getDH(VOID);
ULONG   getESP(VOID);
USHORT  getSP(VOID);
ULONG   getEBP(VOID);
USHORT  getBP(VOID);
ULONG   getESI(VOID);
USHORT  getSI(VOID);
ULONG   getEDI(VOID);
USHORT  getDI(VOID);
ULONG   getEIP(VOID);
USHORT  getIP(VOID);
USHORT  getCS(VOID);
USHORT  getSS(VOID);
USHORT  getDS(VOID);
USHORT  getES(VOID);
USHORT  getFS(VOID);
USHORT  getGS(VOID);
ULONG   getCF(VOID);
ULONG   getPF(VOID);
ULONG   getAF(VOID);
ULONG   getZF(VOID);
ULONG   getSF(VOID);
ULONG   getIF(VOID);
ULONG   getDF(VOID);
ULONG   getOF(VOID);
USHORT	getMSW(VOID);

VOID    setEAX(ULONG);
VOID    setAX(USHORT);
VOID    setAH(UCHAR);
VOID    setAL(UCHAR);
VOID    setEBX(ULONG);
VOID    setBX(USHORT);
VOID    setBH(UCHAR);
VOID    setBL(UCHAR);
VOID    setECX(ULONG);
VOID    setCX(USHORT);
VOID    setCH(UCHAR);
VOID    setCL(UCHAR);
VOID    setEDX(ULONG);
VOID    setDX(USHORT);
VOID    setDH(UCHAR);
VOID    setDL(UCHAR);
VOID    setESP(ULONG);
VOID    setSP(USHORT);
VOID    setEBP(ULONG);
VOID    setBP(USHORT);
VOID    setESI(ULONG);
VOID    setSI(USHORT);
VOID    setEDI(ULONG);
VOID    setDI(USHORT);
VOID    setEIP(ULONG);
VOID    setIP(USHORT);
VOID    setCS(USHORT);
VOID    setSS(USHORT);
VOID    setDS(USHORT);
VOID    setES(USHORT);
VOID    setFS(USHORT);
VOID    setGS(USHORT);
VOID    setCF(ULONG);
VOID    setPF(ULONG);
VOID    setAF(ULONG);
VOID    setZF(ULONG);
VOID    setSF(ULONG);
VOID    setIF(ULONG);
VOID    setDF(ULONG);
VOID    setOF(ULONG);
VOID    setMSW(USHORT);


#else

UCHAR c_getAL(VOID);
UCHAR c_getCL(VOID);
UCHAR c_getDL(VOID);
UCHAR c_getBL(VOID);
UCHAR c_getAH(VOID);
UCHAR c_getCH(VOID);
UCHAR c_getDH(VOID);
UCHAR c_getBH(VOID);

USHORT c_getAX(VOID);
USHORT c_getCX(VOID);
USHORT c_getDX(VOID);
USHORT c_getBX(VOID);
USHORT c_getSP(VOID);
USHORT c_getBP(VOID);
USHORT c_getSI(VOID);
USHORT c_getDI(VOID);
USHORT c_getIP(VOID);
USHORT c_getES(VOID);
USHORT c_getCS(VOID);
USHORT c_getSS(VOID);
USHORT c_getDS(VOID);

USHORT c_getMSW(VOID);

ULONG c_getAF(VOID);
ULONG c_getCF(VOID);
ULONG c_getDF(VOID);
ULONG c_getIF(VOID);
ULONG c_getOF(VOID);
ULONG c_getPF(VOID);
ULONG c_getSF(VOID);
ULONG c_getZF(VOID);

VOID c_setAL(UCHAR val);
VOID c_setCL(UCHAR val);
VOID c_setDL(UCHAR val);
VOID c_setBL(UCHAR val);
VOID c_setAH(UCHAR val);
VOID c_setCH(UCHAR val);
VOID c_setDH(UCHAR val);
VOID c_setBH(UCHAR val);

VOID c_setAX(USHORT val);
VOID c_setCX(USHORT val);
VOID c_setDX(USHORT val);
VOID c_setBX(USHORT val);
VOID c_setSP(USHORT val);
VOID c_setBP(USHORT val);
VOID c_setSI(USHORT val);
VOID c_setDI(USHORT val);
VOID c_setIP(USHORT val);

VOID c_setES(USHORT val);
VOID c_setCS(USHORT val);
VOID c_setSS(USHORT val);
VOID c_setDS(USHORT val);

VOID c_setMSW(USHORT val);

VOID c_setAF(ULONG val);
VOID c_setCF(ULONG val);
VOID c_setDF(ULONG val);
VOID c_setIF(ULONG val);
VOID c_setOF(ULONG val);
VOID c_setPF(ULONG val);
VOID c_setSF(ULONG val);
VOID c_setZF(ULONG val);

#endif


/* end of VDDSVC.H */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

