/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:        adogpool.h
//
// Contents:    ADO GUIDs definition
//
// Comments:
//
//-----------------------------------------------------------------------------


// Breaking Change Note:
// 
// The ADO interfaces were originally platform-dependent and not script friendly. ADO interface IID's are reguided 
// in this header file. Therefore, newly-compiled applications may not run on downlevel OSes, since the new interface 
// IID's are not available on downlevel OSes. To avoid this compatibility issue, customers can either:
// (1) install the KB983246 on all downlevel OSes 
// (2) Change your target platform to Win7 or below. This will use back the IID defined in Win7 SDK and 
//     they will be compatible for downlevel OSes


#if !defined(NTDDI_VERSION) || (NTDDI_VERSION < NTDDI_WIN8)

// Application is targetting below Windows 8
#include "adogpool_Backcompat.h"

#else

// Application is targetting Windows 8 or above
#ifndef ADO_SUPPRESS_MESSAGE
#ifdef _MSC_VER
#pragma message( "Change your target platform to Windows 7 or below if your application requires backward compatibility for the ADO interface." )
#endif // _MSC_VER
#endif // ADO_SUPPRESS_MESSAGE

#ifndef INCLUDING_ADOGUIDS
#error Incorrect usage of this include file - cannot be used directly
#endif


// How to assign new GUID:
//
// 1) In this file search for macro RESERVED_GUIDS_BEYOND_THIS_POINT
// 2) Take first reserved GUID in this section and move #ifdef RESERVED_GUIDS_BEYOND_THIS_POINT
//    beyond the guid being newly assigned
// 3) rename all instances of pattern ADO_Reserved_xxx (there are 5 of them including the comment)
//    with the desired GUID name
// 4) for midl, usage is typically:  uuid(ADO_Reserved_1)
//    for C++: ADO_Reserved_1
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Guids from adoid.h follow:


#define  MAXAVAILABLEGUID    0x00000570
#define  MAXAVAILABLEGUIDALL 0x0000057F


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Version-independent Typelib ID:

#define  LIBID_ADO      LIBID_ADO60
#define  LIBID_ADOR     LIBID_ADOR20
#define  LIBID_CADO10   LIBID_ADO20
#define  LIBID_CADOR10  LIBID_ADOR20

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Version specific Typelib ID:

// LIBID_ADO20
#define  LIBID_ADO20 GUID_BUILDER(LIBID_ADO20,00000200,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO20;
#undef LIBID_ADO20
#endif // IMMEDIATE_GUID_USE

// LIBID_ADO21
#define  LIBID_ADO21 GUID_BUILDER(LIBID_ADO21,00000201,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO21;
#undef LIBID_ADO21
#endif // IMMEDIATE_GUID_USE

// LIBID_ADO25
#define  LIBID_ADO25 GUID_BUILDER(LIBID_ADO25,00000205,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO25;
#undef LIBID_ADO25
#endif // IMMEDIATE_GUID_USE

// LIBID_ADO26
#define  LIBID_ADO26 GUID_BUILDER(LIBID_ADO26,00000206,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO26;
#undef LIBID_ADO26
#endif // IMMEDIATE_GUID_USE

// LIBID_ADO27
#define LIBID_ADO27 GUID_BUILDER(LIBID_ADO27,EF53050B,882E,4776,B6,43,ED,A4,72,E8,E3,F2)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO27;
#undef  LIBID_ADO27
#endif // IMMEDIATE_GUID_USE

// LIBID_ADO28 this is reserved GUID 11 we have to get it from there as 9.0 sterted using GUIDs in parallel with 28
#define LIBID_ADO28 GUID_BUILDER(LIBID_ADO28,2A75196C,D9EB,4129,B8,03,93,13,27,F7,2D,5C)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO28;
#undef  LIBID_ADO28
#endif // IMMEDIATE_GUID_USE

// LIBID_ADO60
#define LIBID_ADO60 GUID_BUILDER(LIBID_ADO60,B691E011,1797,432E,90,7A,4D,8C,69,33,91,29)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADO60;
#undef  LIBID_ADO60
#endif // IMMEDIATE_GUID_USE

// LIBID_ADOR20
#define  LIBID_ADOR20 GUID_BUILDER(LIBID_ADOR20,00000300,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADOR20;
#undef LIBID_ADOR20
#endif // IMMEDIATE_GUID_USE

// LIBID_ADOR25
#define  LIBID_ADOR25 GUID_BUILDER(LIBID_ADOR25,00000305,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
LIBID_ADOR25;
#undef LIBID_ADOR25
#endif // IMMEDIATE_GUID_USE

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Interface & Class ID (version-independent)

// CLSID_CADOError
#define  CLSID_CADOError GUID_BUILDER(CLSID_CADOError,00000541,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOError;
#undef CLSID_CADOError
#endif // IMMEDIATE_GUID_USE

// IID_IADOError
#define  IID_IADOError GUID_BUILDER(IID_IADOError,00000500,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOError;
#undef IID_IADOError
#endif // IMMEDIATE_GUID_USE

// IID_IADOErrors
#define  IID_IADOErrors GUID_BUILDER(IID_IADOErrors,00000501,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOErrors;
#undef IID_IADOErrors
#endif // IMMEDIATE_GUID_USE

// IID_IADOProperty
#define  IID_IADOProperty GUID_BUILDER(IID_IADOProperty,00000503,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOProperty;
#undef IID_IADOProperty
#endif // IMMEDIATE_GUID_USE

// IID_IADOProperties
#define  IID_IADOProperties GUID_BUILDER(IID_IADOProperties,00000504,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOProperties;
#undef IID_IADOProperties
#endif // IMMEDIATE_GUID_USE

// CLSID_CADOField
#define  CLSID_CADOField GUID_BUILDER(CLSID_CADOField,0000053A,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOField;
#undef CLSID_CADOField
#endif // IMMEDIATE_GUID_USE

// IID_IADOField
#define  IID_IADOField GUID_BUILDER(IID_IADOField,00001569,0000,0010,80,00,00,AA,00,6D,2E,A4)
#define  IID_IADOField25 GUID_BUILDER(IID_IADOField25,00001569,0000,0010,80,00,00,AA,00,6D,2E,A4) // alias of IID_IADOField
#ifdef IMMEDIATE_GUID_USE
IID_IADOField;
IID_IADOField25;
#undef IID_IADOField
#undef IID_IADOField25
#endif // IMMEDIATE_GUID_USE

// IID_IADOFields
#define  IID_IADOFields GUID_BUILDER(IID_IADOFields,00001564,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOFields;
#undef IID_IADOFields
#endif // IMMEDIATE_GUID_USE

// CLSID_CADOCommand
#define  CLSID_CADOCommand GUID_BUILDER(CLSID_CADOCommand,00000507,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOCommand;
#undef CLSID_CADOCommand
#endif // IMMEDIATE_GUID_USE

// IID_IADOCommand
#define IID_IADOCommand GUID_BUILDER(IID_IADOCommand,986761E8,7269,4890,AA,65,AD,7C,03,69,7A,6D)
#ifdef IMMEDIATE_GUID_USE
IID_IADOCommand;
#undef  IID_IADOCommand
#endif // IMMEDIATE_GUID_USE

// IID_IADOCommands
#define  IID_IADOCommands GUID_BUILDER(IID_IADOCommands,00000509,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOCommands;
#undef IID_IADOCommands
#endif // IMMEDIATE_GUID_USE

// IID_IADOCommandConstruction
#define  IID_IADOCommandConstruction GUID_BUILDER(IID_IADOCommandConstruction,00000517,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOCommandConstruction;
#undef IID_IADOCommandConstruction
#endif // IMMEDIATE_GUID_USE

// CLSID_CADOParameter
#define  CLSID_CADOParameter GUID_BUILDER(CLSID_CADOParameter,0000050B,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOParameter;
#undef CLSID_CADOParameter
#endif // IMMEDIATE_GUID_USE

// IID_IADOParameter
#define  IID_IADOParameter GUID_BUILDER(IID_IADOParameter,0000150C,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOParameter;
#undef IID_IADOParameter
#endif // IMMEDIATE_GUID_USE

// IID_IADOParameters
#define  IID_IADOParameters GUID_BUILDER(IID_IADOParameters,0000150D,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOParameters;
#undef IID_IADOParameters
#endif // IMMEDIATE_GUID_USE

// CLSID_CADORecordset
#define  CLSID_CADORecordset GUID_BUILDER(CLSID_CADORecordset,00000535,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADORecordset;
#undef CLSID_CADORecordset
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordset
#define  IID_IADORecordset GUID_BUILDER(IID_IADORecordset,00001556,0000,0010,80,00,00,AA,00,6D,2E,A4)
#define  IID_IADORecordset25 GUID_BUILDER(IID_IADORecordset25,00001556,0000,0010,80,00,00,AA,00,6D,2E,A4)   // alias of IID_IADORecordset
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordset;
IID_IADORecordset25;
#undef IID_IADORecordset
#undef IID_IADORecordset25
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordsets
#define  IID_IADORecordsets GUID_BUILDER(IID_IADORecordsets,0000050F,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordsets;
#undef IID_IADORecordsets
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordsetConstruction
#define  IID_IADORecordsetConstruction GUID_BUILDER(IID_IADORecordsetConstruction,00001283,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordsetConstruction;
#undef IID_IADORecordsetConstruction
#endif // IMMEDIATE_GUID_USE

// IID_IADOCollection
#define  IID_IADOCollection GUID_BUILDER(IID_IADOCollection,00000512,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOCollection;
#undef IID_IADOCollection
#endif // IMMEDIATE_GUID_USE

// IID_IADODynaCollection
#define  IID_IADODynaCollection GUID_BUILDER(IID_IADODynaCollection,00000513,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADODynaCollection;
#undef IID_IADODynaCollection
#endif // IMMEDIATE_GUID_USE

// CLSID_CADOConnection
#define  CLSID_CADOConnection GUID_BUILDER(CLSID_CADOConnection,00000514,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOConnection;
#undef CLSID_CADOConnection
#endif // IMMEDIATE_GUID_USE

// IID_IADOConnection
#define  IID_IADOConnection GUID_BUILDER(IID_IADOConnection,00001550,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnection;
#undef IID_IADOConnection
#endif // IMMEDIATE_GUID_USE

// IID_IADOConnections
#define  IID_IADOConnections GUID_BUILDER(IID_IADOConnections,00000518,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnections;
#undef IID_IADOConnections
#endif // IMMEDIATE_GUID_USE

// IID_IADOConnectionConstruction
#define  IID_IADOConnectionConstruction GUID_BUILDER(IID_IADOConnectionConstruction,00000551,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnectionConstruction;
#undef IID_IADOConnectionConstruction
#endif // IMMEDIATE_GUID_USE

// CLSID_CADORecord
#define  CLSID_CADORecord GUID_BUILDER(CLSID_CADORecord,00000560,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADORecord;
#undef CLSID_CADORecord
#endif // IMMEDIATE_GUID_USE

// CLSID_CADORecField
#define  CLSID_CADORecField GUID_BUILDER(CLSID_CADORecField,00000561,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADORecField;
#undef CLSID_CADORecField
#endif // IMMEDIATE_GUID_USE

// IID_IADORecord
#define  IID_IADORecord GUID_BUILDER(IID_IADORecord,00001562,0000,0010,80,00,00,AA,00,6D,2E,A4)
#define  IID_IADORecord25 GUID_BUILDER(IID_IADORecord25,00001562,0000,0010,80,00,00,AA,00,6D,2E,A4)   // alias of IID_IADORecord
#ifdef IMMEDIATE_GUID_USE
IID_IADORecord;
IID_IADORecord25;
#undef IID_IADORecord
#undef IID_IADORecord25
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordConstruction
#define  IID_IADORecordConstruction GUID_BUILDER(IID_IADORecordConstruction,00000567,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordConstruction;
#undef IID_IADORecordConstruction
#endif // IMMEDIATE_GUID_USE

// CLSID_CADOStream
#define  CLSID_CADOStream GUID_BUILDER(CLSID_CADOStream,00000566,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOStream;
#undef CLSID_CADOStream
#endif // IMMEDIATE_GUID_USE

// IID_IADOStream
#define  IID_IADOStream GUID_BUILDER(IID_IADOStream,00001565,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOStream;
#undef IID_IADOStream
#endif // IMMEDIATE_GUID_USE

// IID_IADOStreamConstruction
#define  IID_IADOStreamConstruction GUID_BUILDER(IID_IADOStreamConstruction,00000568,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOStreamConstruction;
#undef IID_IADOStreamConstruction
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordsetEvents
#define  IID_IADORecordsetEvents GUID_BUILDER(IID_IADORecordsetEvents,00001266,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordsetEvents;
#undef IID_IADORecordsetEvents
#endif // IMMEDIATE_GUID_USE

// IID_IADOConnectionEvents
#define  IID_IADOConnectionEvents GUID_BUILDER(IID_IADOConnectionEvents,00001400,0000,0010,80,00,00,AA,00,6D,2E,A4)
#define  IID_ConnectionEvents GUID_BUILDER(IID_ConnectionEvents,00001400,0000,0010,80,00,00,AA,00,6D,2E,A4)     // alias of IID_IADOConnectionEvents
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnectionEvents;
IID_ConnectionEvents;
#undef IID_IADOConnectionEvents
#undef IID_ConnectionEvents
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordsetEventsVt
#define  IID_IADORecordsetEventsVt GUID_BUILDER(IID_IADORecordsetEventsVt,00001403,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordsetEventsVt;
#undef IID_IADORecordsetEventsVt
#endif // IMMEDIATE_GUID_USE

// IID_IADOConnectionEventsVt
#define  IID_IADOConnectionEventsVt GUID_BUILDER(IID_IADOConnectionEventsVt,00001402,0000,0010,80,00,00,AA,00,6D,2E,A4)
#define  IID_ConnectionEventsVt GUID_BUILDER(IID_ConnectionEventsVt,00001402,0000,0010,80,00,00,AA,00,6D,2E,A4) // alias of IID_IADOConnectionEventsVt 
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnectionEventsVt;
IID_ConnectionEventsVt;
#undef IID_IADOConnectionEventsVt
#undef IID_ConnectionEventsVt
#endif // IMMEDIATE_GUID_USE

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Interface ID (version-dependent)

// IID_IADOField15
#define  IID_IADOField15 GUID_BUILDER(IID_IADOField15,00001505,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOField15;
#undef IID_IADOField15
#endif // IMMEDIATE_GUID_USE

// IID_IADOField20
#define  IID_IADOField20 GUID_BUILDER(IID_IADOField20,0000154C,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOField20;
#undef IID_IADOField20
#endif // IMMEDIATE_GUID_USE

// IID_IADOField25 was defined as IID_IADOField (see IID_IADOField)

#ifdef _LOCKBYTESUPPORT_
// IID_IADOField26
#define  IID_IADOField26 GUID_BUILDER(IID_IADOField26,00001557,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOField26;
#undef IID_IADOField26
#endif // IMMEDIATE_GUID_USE
#endif // _LOCKBYTESUPPORT_



// IID_IADOFields15
#define  IID_IADOFields15 GUID_BUILDER(IID_IADOFields15,00001506,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOFields15;
#undef IID_IADOFields15
#endif // IMMEDIATE_GUID_USE

// IID_IADOFields20
#define  IID_IADOFields20 GUID_BUILDER(IID_IADOFields20,0000154D,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOFields20;
#undef IID_IADOFields20
#endif // IMMEDIATE_GUID_USE



// IID_IADOCommand15
#define  IID_IADOCommand15 GUID_BUILDER(IID_IADOCommand15,00001508,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOCommand15;
#undef IID_IADOCommand15
#endif // IMMEDIATE_GUID_USE

// IID_IADOCommand25
#define  IID_IADOCommand25 GUID_BUILDER(IID_IADOCommand25,0000154E,0000,0010,80,00,00,AA,00,6D,2E,A4)

#ifdef IMMEDIATE_GUID_USE
IID_IADOCommand25;
#undef IID_IADOCommand25
#endif // IMMEDIATE_GUID_USE



// IID_IADORecordset15
#define  IID_IADORecordset15 GUID_BUILDER(IID_IADORecordset15,0000150E,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordset15;
#undef IID_IADORecordset15
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordset20
#define  IID_IADORecordset20 GUID_BUILDER(IID_IADORecordset20,0000154F,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordset20;
#undef IID_IADORecordset20
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordset21
#define  IID_IADORecordset21 GUID_BUILDER(IID_IADORecordset21,00001555,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordset21;
#undef IID_IADORecordset21
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordset25 was defined as IID_IADORecordset (see IID_IADORecordset above)



// IID_IADOConnection15
#define  IID_IADOConnection15 GUID_BUILDER(IID_IADOConnection15,00001515,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnection15;
#undef IID_IADOConnection15
#endif // IMMEDIATE_GUID_USE



// IID_IADOConnectionConstruction15
#define  IID_IADOConnectionConstruction15 GUID_BUILDER(IID_IADOConnectionConstruction15,00000516,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOConnectionConstruction15;
#undef IID_IADOConnectionConstruction15
#endif // IMMEDIATE_GUID_USE



//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Guids from adoidall.h follow:

// IID_EnumCursorType
#define  IID_EnumCursorType GUID_BUILDER(IID_EnumCursorType,0000051B,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCursorType;
#undef IID_EnumCursorType
#endif // IMMEDIATE_GUID_USE

// IID_EnumCursorOption
#define  IID_EnumCursorOption GUID_BUILDER(IID_EnumCursorOption,0000051C,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCursorOption;
#undef IID_EnumCursorOption
#endif // IMMEDIATE_GUID_USE

// IID_EnumLockType
#define  IID_EnumLockType GUID_BUILDER(IID_EnumLockType,0000051D,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumLockType;
#undef IID_EnumLockType
#endif // IMMEDIATE_GUID_USE

// IID_EnumExecuteOption
#define  IID_EnumExecuteOption GUID_BUILDER(IID_EnumExecuteOption,0000051E,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumExecuteOption;
#undef IID_EnumExecuteOption
#endif // IMMEDIATE_GUID_USE

// IID_EnumDataType
#define  IID_EnumDataType GUID_BUILDER(IID_EnumDataType,0000051F,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumDataType;
#undef IID_EnumDataType
#endif // IMMEDIATE_GUID_USE

// IID_EnumConnectPrompt
#define  IID_EnumConnectPrompt GUID_BUILDER(IID_EnumConnectPrompt,00000520,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumConnectPrompt;
#undef IID_EnumConnectPrompt
#endif // IMMEDIATE_GUID_USE

// IID_EnumConnectMode
#define  IID_EnumConnectMode GUID_BUILDER(IID_EnumConnectMode,00000521,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumConnectMode;
#undef IID_EnumConnectMode
#endif // IMMEDIATE_GUID_USE

// IID_EnumPrepareOption
#define  IID_EnumPrepareOption GUID_BUILDER(IID_EnumPrepareOption,00000522,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumPrepareOption;
#undef IID_EnumPrepareOption
#endif // IMMEDIATE_GUID_USE

// IID_EnumIsolationLevel
#define  IID_EnumIsolationLevel GUID_BUILDER(IID_EnumIsolationLevel,00000523,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumIsolationLevel;
#undef IID_EnumIsolationLevel
#endif // IMMEDIATE_GUID_USE

// IID_EnumXactAttribute
#define  IID_EnumXactAttribute GUID_BUILDER(IID_EnumXactAttribute,00000524,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumXactAttribute;
#undef IID_EnumXactAttribute
#endif // IMMEDIATE_GUID_USE

// IID_EnumFieldAttribute
#define  IID_EnumFieldAttribute GUID_BUILDER(IID_EnumFieldAttribute,00000525,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumFieldAttribute;
#undef IID_EnumFieldAttribute
#endif // IMMEDIATE_GUID_USE

// IID_EnumEditMode
#define  IID_EnumEditMode GUID_BUILDER(IID_EnumEditMode,00000526,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumEditMode;
#undef IID_EnumEditMode
#endif // IMMEDIATE_GUID_USE

// IID_EnumRecordStatus
#define  IID_EnumRecordStatus GUID_BUILDER(IID_EnumRecordStatus,00000527,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRecordStatus;
#undef IID_EnumRecordStatus
#endif // IMMEDIATE_GUID_USE

// IID_EnumPosition
#define  IID_EnumPosition GUID_BUILDER(IID_EnumPosition,00000528,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumPosition;
#undef IID_EnumPosition
#endif // IMMEDIATE_GUID_USE

// IID_EnumPropertyAttributes
#define  IID_EnumPropertyAttributes GUID_BUILDER(IID_EnumPropertyAttributes,00000529,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumPropertyAttributes;
#undef IID_EnumPropertyAttributes
#endif // IMMEDIATE_GUID_USE

// IID_EnumErrorValue
#define  IID_EnumErrorValue GUID_BUILDER(IID_EnumErrorValue,0000052A,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumErrorValue;
#undef IID_EnumErrorValue
#endif // IMMEDIATE_GUID_USE

// IID_EnumParameterAttributes
#define  IID_EnumParameterAttributes GUID_BUILDER(IID_EnumParameterAttributes,0000052B,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumParameterAttributes;
#undef IID_EnumParameterAttributes
#endif // IMMEDIATE_GUID_USE

// IID_EnumParameterDirection
#define  IID_EnumParameterDirection GUID_BUILDER(IID_EnumParameterDirection,0000052C,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumParameterDirection;
#undef IID_EnumParameterDirection
#endif // IMMEDIATE_GUID_USE

// IID_EnumFilterCriteria
#define  IID_EnumFilterCriteria GUID_BUILDER(IID_EnumFilterCriteria,0000052D,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumFilterCriteria;
#undef IID_EnumFilterCriteria
#endif // IMMEDIATE_GUID_USE

// IID_EnumCommandType
#define  IID_EnumCommandType GUID_BUILDER(IID_EnumCommandType,0000052E,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCommandType;
#undef IID_EnumCommandType
#endif // IMMEDIATE_GUID_USE

// IID_EnumCursorLocation
#define  IID_EnumCursorLocation GUID_BUILDER(IID_EnumCursorLocation,0000052F,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCursorLocation;
#undef IID_EnumCursorLocation
#endif // IMMEDIATE_GUID_USE

// IID_EnumEventStatus
#define  IID_EnumEventStatus GUID_BUILDER(IID_EnumEventStatus,00000530,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumEventStatus;
#undef IID_EnumEventStatus
#endif // IMMEDIATE_GUID_USE

// IID_EnumEventReason
#define  IID_EnumEventReason GUID_BUILDER(IID_EnumEventReason,00000531,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumEventReason;
#undef IID_EnumEventReason
#endif // IMMEDIATE_GUID_USE

// IID_EnumObjectState
#define  IID_EnumObjectState GUID_BUILDER(IID_EnumObjectState,00000532,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumObjectState;
#undef IID_EnumObjectState
#endif // IMMEDIATE_GUID_USE

// IID_EnumSchema
#define  IID_EnumSchema GUID_BUILDER(IID_EnumSchema,00000533,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumSchema;
#undef IID_EnumSchema
#endif // IMMEDIATE_GUID_USE

// IID_EnumMarshalOptions
#define  IID_EnumMarshalOptions GUID_BUILDER(IID_EnumMarshalOptions,00000540,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumMarshalOptions;
#undef IID_EnumMarshalOptions
#endif // IMMEDIATE_GUID_USE

// IID_EnumConnectOption
#define  IID_EnumConnectOption GUID_BUILDER(IID_EnumConnectOption,00000541,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumConnectOption;
#undef IID_EnumConnectOption
#endif // IMMEDIATE_GUID_USE

// IID_EnumGetRowsOption
#define  IID_EnumGetRowsOption GUID_BUILDER(IID_EnumGetRowsOption,00000542,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumGetRowsOption;
#undef IID_EnumGetRowsOption
#endif // IMMEDIATE_GUID_USE

// IID_EnumAffect
#define  IID_EnumAffect GUID_BUILDER(IID_EnumAffect,00000543,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumAffect;
#undef IID_EnumAffect
#endif // IMMEDIATE_GUID_USE

// IID_EnumResync
#define  IID_EnumResync GUID_BUILDER(IID_EnumResync,00000544,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumResync;
#undef IID_EnumResync
#endif // IMMEDIATE_GUID_USE

// IID_EnumCompare
#define  IID_EnumCompare GUID_BUILDER(IID_EnumCompare,00000545,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCompare;
#undef IID_EnumCompare
#endif // IMMEDIATE_GUID_USE

// IID_EnumFilterGroup
#define  IID_EnumFilterGroup GUID_BUILDER(IID_EnumFilterGroup,00000546,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumFilterGroup;
#undef IID_EnumFilterGroup
#endif // IMMEDIATE_GUID_USE

// IID_EnumSearchDirection
#define  IID_EnumSearchDirection GUID_BUILDER(IID_EnumSearchDirection,00000547,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumSearchDirection;
#undef IID_EnumSearchDirection
#endif // IMMEDIATE_GUID_USE

// IID_EnumPersistFormat
#define  IID_EnumPersistFormat GUID_BUILDER(IID_EnumPersistFormat,00000548,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumPersistFormat;
#undef IID_EnumPersistFormat
#endif // IMMEDIATE_GUID_USE

// IID_EnumStringFormat
#define  IID_EnumStringFormat GUID_BUILDER(IID_EnumStringFormat,00000549,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumStringFormat;
#undef IID_EnumStringFormat
#endif // IMMEDIATE_GUID_USE

// IID_EnumRDSUpdateCriteria
#define  IID_EnumRDSUpdateCriteria GUID_BUILDER(IID_EnumRDSUpdateCriteria,0000054A,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRDSUpdateCriteria;
#undef IID_EnumRDSUpdateCriteria
#endif // IMMEDIATE_GUID_USE

// IID_EnumRDSAsyncThreadPriority
#define  IID_EnumRDSAsyncThreadPriority GUID_BUILDER(IID_EnumRDSAsyncThreadPriority,0000054B,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRDSAsyncThreadPriority;
#undef IID_EnumRDSAsyncThreadPriority
#endif // IMMEDIATE_GUID_USE

// IID_EnumCEResync
#define  IID_EnumCEResync GUID_BUILDER(IID_EnumCEResync,00000553,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCEResync;
#undef IID_EnumCEResync
#endif // IMMEDIATE_GUID_USE

// IID_EnumRDSAutoRecalc
#define  IID_EnumRDSAutoRecalc GUID_BUILDER(IID_EnumRDSAutoRecalc,00000554,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRDSAutoRecalc;
#undef IID_EnumRDSAutoRecalc
#endif // IMMEDIATE_GUID_USE

// IID_EnumSeek
#define  IID_EnumSeek GUID_BUILDER(IID_EnumSeek,00000552,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumSeek;
#undef IID_EnumSeek
#endif // IMMEDIATE_GUID_USE

// IID_IADORecordGroup
#define  IID_IADORecordGroup GUID_BUILDER(IID_IADORecordGroup,00000511,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADORecordGroup;
#undef IID_IADORecordGroup
#endif // IMMEDIATE_GUID_USE

// IID_IADOCustomError
#define  IID_IADOCustomError GUID_BUILDER(IID_IADOCustomError,00000519,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOCustomError;
#undef IID_IADOCustomError
#endif // IMMEDIATE_GUID_USE

// IID_IPrivErrors
#define  IID_IPrivErrors GUID_BUILDER(IID_IPrivErrors,00000502,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IPrivErrors;
#undef IID_IPrivErrors
#endif // IMMEDIATE_GUID_USE

// CLSID_CADOErrorLookup
#define  CLSID_CADOErrorLookup GUID_BUILDER(CLSID_CADOErrorLookup,00000542,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_CADOErrorLookup;
#undef CLSID_CADOErrorLookup
#endif // IMMEDIATE_GUID_USE

// CLSID_ADO
#define  CLSID_ADO GUID_BUILDER(CLSID_ADO,0000051A,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
CLSID_ADO;
#undef CLSID_ADO
#endif // IMMEDIATE_GUID_USE

// IID__ADO
#define  IID__ADO GUID_BUILDER(IID__ADO,00000534,0000,0010,80,00,00,AA,00,6D,2E,A4)
#define  IID_IADO10StdObject GUID_BUILDER(IID_IADO10StdObject,00000534,0000,0010,80,00,00,AA,00,6D,2E,A4)  // alias of IID__ADO
#ifdef IMMEDIATE_GUID_USE
IID__ADO;
IID_IADO10StdObject;
#undef IID__ADO
#undef IID_IADO10StdObject
#endif // IMMEDIATE_GUID_USE

// IID_IADOClass
#define  IID_IADOClass GUID_BUILDER(IID_IADOClass,00000560,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_IADOClass;
#undef IID_IADOClass
#endif // IMMEDIATE_GUID_USE

// IID_EnumRecordCreateOptions
#define  IID_EnumRecordCreateOptions GUID_BUILDER(IID_EnumRecordCreateOptions,00000570,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRecordCreateOptions;
#undef IID_EnumRecordCreateOptions
#endif // IMMEDIATE_GUID_USE

// IID_EnumRecordOpenOptions
#define  IID_EnumRecordOpenOptions GUID_BUILDER(IID_EnumRecordOpenOptions,00000571,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRecordOpenOptions;
#undef IID_EnumRecordOpenOptions
#endif // IMMEDIATE_GUID_USE


// IID_EnumMoveRecordOptions
#define  IID_EnumMoveRecordOptions GUID_BUILDER(IID_EnumMoveRecordOptions,00000573,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumMoveRecordOptions;
#undef IID_EnumMoveRecordOptions
#endif // IMMEDIATE_GUID_USE

// IID_EnumCopyRecordOptions
#define  IID_EnumCopyRecordOptions GUID_BUILDER(IID_EnumCopyRecordOptions,00000574,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumCopyRecordOptions;
#undef IID_EnumCopyRecordOptions
#endif // IMMEDIATE_GUID_USE

// IID_EnumMode
#define  IID_EnumMode GUID_BUILDER(IID_EnumMode,00000575,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumMode;
#undef IID_EnumMode
#endif // IMMEDIATE_GUID_USE

// IID_EnumStreamType
#define  IID_EnumStreamType GUID_BUILDER(IID_EnumStreamType,00000576,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumStreamType;
#undef IID_EnumStreamType
#endif // IMMEDIATE_GUID_USE

// IID_EnumLineSeparator
#define  IID_EnumLineSeparator GUID_BUILDER(IID_EnumLineSeparator,00000577,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumLineSeparator;
#undef IID_EnumLineSeparator
#endif // IMMEDIATE_GUID_USE

// IID_EnumStreamOpenOptions
#define  IID_EnumStreamOpenOptions GUID_BUILDER(IID_EnumStreamOpenOptions,0000057A,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumStreamOpenOptions;
#undef IID_EnumStreamOpenOptions
#endif // IMMEDIATE_GUID_USE

// IID_EnumStreamWrite
#define  IID_EnumStreamWrite GUID_BUILDER(IID_EnumStreamWrite,0000057B,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumStreamWrite;
#undef IID_EnumStreamWrite
#endif // IMMEDIATE_GUID_USE

// IID_EnumSaveOptions
#define  IID_EnumSaveOptions GUID_BUILDER(IID_EnumSaveOptions,0000057C,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumSaveOptions;
#undef IID_EnumSaveOptions
#endif // IMMEDIATE_GUID_USE

// IID_EnumRecordType
#define  IID_EnumRecordType GUID_BUILDER(IID_EnumRecordType,0000057D,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumRecordType;
#undef IID_EnumRecordType
#endif // IMMEDIATE_GUID_USE

// IID_EnumFieldStatus
#define  IID_EnumFieldStatus GUID_BUILDER(IID_EnumFieldStatus,0000057E,0000,0010,80,00,00,AA,00,6D,2E,A4)
#ifdef IMMEDIATE_GUID_USE
IID_EnumFieldStatus;
#undef IID_EnumFieldStatus
#endif // IMMEDIATE_GUID_USE

// IID_ICMemStreamProperties
#define IID_ICMemStreamProperties GUID_BUILDER(IID_ICMemStreamProperties,FF184014,B5D3,4310,AB,F0,9B,70,45,A2,CF,17)
#ifdef IMMEDIATE_GUID_USE
IID_ICMemStreamProperties;
#undef  IID_ICMemStreamProperties
#endif // IMMEDIATE_GUID_USE

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// End of old guid pool. Newly generated ADO GUID pool follows.

#ifdef RESERVED_GUIDS_BEYOND_THIS_POINT

// ADO_Reserved_4
#define ADO_Reserved_4	GUID_BUILDER(ADO_Reserved_4,567747F1,658B,4906,82,C4,E9,CD,A1,46,26,15)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_4;
#undef	ADO_Reserved_4
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_6
#define ADO_Reserved_6	GUID_BUILDER(ADO_Reserved_6,ED5A4589,7A9D,41DF,89,86,CC,A9,25,01,A5,DA)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_6;
#undef	ADO_Reserved_6
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_7
#define ADO_Reserved_7  GUID_BUILDER(ADO_Reserved_7,C029178A,F16B,4A06,82,93,A8,08,B7,F8,78,92)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_7;
#undef  ADO_Reserved_7
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_8
#define ADO_Reserved_8  GUID_BUILDER(ADO_Reserved_8,FD6974FD,21FB,409C,96,56,A5,68,FE,C0,AC,01)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_8;
#undef  ADO_Reserved_8
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_9
#define ADO_Reserved_9	GUID_BUILDER(ADO_Reserved_9,F23FCB5E,7159,4CBA,A3,41,0E,7A,A5,15,18,70)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_9;
#undef	ADO_Reserved_9
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_10
#define ADO_Reserved_10 GUID_BUILDER(ADO_Reserved_10,E724D5C9,327C,43F7,86,4C,68,2F,FF,5C,99,93)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_10;
#undef  ADO_Reserved_10
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_11 do not use this
//#define ADO_Reserved_11   GUID_BUILDER(ADO_Reserved_11,2A75196C,D9EB,4129,B8,03,93,13,27,F7,2D,5C)
//#ifdef IMMEDIATE_GUID_USE do not use this used for 28 tlb
//ADO_Reserved_11;
//#undef    ADO_Reserved_11
//#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_12
#define ADO_Reserved_12 GUID_BUILDER(ADO_Reserved_12,8831EBB5,2C09,4DDD,9A,7A,AC,13,6D,58,D7,21)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_12;
#undef  ADO_Reserved_12
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_13
#define ADO_Reserved_13 GUID_BUILDER(ADO_Reserved_13,447B1221,64FA,44E9,B1,46,B1,1F,16,E3,14,B2)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_13;
#undef  ADO_Reserved_13
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_14
#define ADO_Reserved_14 GUID_BUILDER(ADO_Reserved_14,FC528DC2,A992,44D3,97,9F,07,F7,F4,45,5F,23)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_14;
#undef  ADO_Reserved_14
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_15
#define ADO_Reserved_15 GUID_BUILDER(ADO_Reserved_15,C2CC7BC0,9F8B,46C8,83,6B,BC,46,70,28,F4,54)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_15;
#undef  ADO_Reserved_15
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_16
#define ADO_Reserved_16 GUID_BUILDER(ADO_Reserved_16,4687EE6C,12CE,4A31,97,E9,E6,49,6D,E7,2C,71)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_16;
#undef  ADO_Reserved_16
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_17
#define ADO_Reserved_17 GUID_BUILDER(ADO_Reserved_17,4B56FC5D,992F,4339,95,81,C5,40,7A,B2,BF,FD)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_17;
#undef  ADO_Reserved_17
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_18
#define ADO_Reserved_18 GUID_BUILDER(ADO_Reserved_18,1F13BFB3,8BA8,46CA,91,78,74,28,EF,9A,85,C0)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_18;
#undef  ADO_Reserved_18
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_19
#define ADO_Reserved_19 GUID_BUILDER(ADO_Reserved_19,0B410060,4D75,4F77,96,A1,68,4C,38,15,E1,B1)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_19;
#undef  ADO_Reserved_19
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_20
#define ADO_Reserved_20 GUID_BUILDER(ADO_Reserved_20,5593F2E0,436B,40B8,81,A8,1B,7E,F4,E6,25,2C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_20;
#undef  ADO_Reserved_20
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_21
#define ADO_Reserved_21 GUID_BUILDER(ADO_Reserved_21,88447B2F,E1C9,413E,BE,E7,A7,D2,B9,0E,D1,96)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_21;
#undef  ADO_Reserved_21
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_22
#define ADO_Reserved_22 GUID_BUILDER(ADO_Reserved_22,89BFEE1B,8CB5,4A90,89,AF,E8,29,93,4E,6C,48)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_22;
#undef  ADO_Reserved_22
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_23
#define ADO_Reserved_23 GUID_BUILDER(ADO_Reserved_23,28D7F9FC,F485,4BDB,9C,C4,6F,AE,44,F9,9F,D9)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_23;
#undef  ADO_Reserved_23
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_24
#define ADO_Reserved_24 GUID_BUILDER(ADO_Reserved_24,1BB4223F,B0E8,4540,96,FD,B8,FE,D9,A7,C0,8B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_24;
#undef  ADO_Reserved_24
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_25
#define ADO_Reserved_25 GUID_BUILDER(ADO_Reserved_25,AD1A1568,8B4A,403F,84,76,D8,F6,33,4D,BD,9F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_25;
#undef  ADO_Reserved_25
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_26
#define ADO_Reserved_26 GUID_BUILDER(ADO_Reserved_26,1326B4D8,EE0B,4054,8F,4C,86,35,9F,00,24,AD)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_26;
#undef  ADO_Reserved_26
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_27
#define ADO_Reserved_27 GUID_BUILDER(ADO_Reserved_27,98B7EB70,7AED,401A,AF,6D,A6,B8,DB,A0,AF,A6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_27;
#undef  ADO_Reserved_27
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_28
#define ADO_Reserved_28 GUID_BUILDER(ADO_Reserved_28,FD46F2C2,7FDA,4DC9,A2,DB,D9,BE,4F,59,98,C2)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_28;
#undef  ADO_Reserved_28
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_29
#define ADO_Reserved_29 GUID_BUILDER(ADO_Reserved_29,FAA37542,B471,4183,A6,56,99,C8,FD,80,FF,73)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_29;
#undef  ADO_Reserved_29
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_30
#define ADO_Reserved_30 GUID_BUILDER(ADO_Reserved_30,56CE86F1,3116,4104,A5,28,17,D1,1E,DC,68,2A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_30;
#undef  ADO_Reserved_30
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_31
#define ADO_Reserved_31 GUID_BUILDER(ADO_Reserved_31,83E8CF0E,176F,4908,86,3A,2A,77,4D,76,9B,EF)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_31;
#undef  ADO_Reserved_31
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_32
#define ADO_Reserved_32 GUID_BUILDER(ADO_Reserved_32,0494D18D,98F7,4A38,80,DF,35,F8,80,98,BD,DF)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_32;
#undef  ADO_Reserved_32
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_33
#define ADO_Reserved_33 GUID_BUILDER(ADO_Reserved_33,00C61F59,4E7F,4093,BF,FD,03,53,B4,5D,E5,8B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_33;
#undef  ADO_Reserved_33
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_34
#define ADO_Reserved_34 GUID_BUILDER(ADO_Reserved_34,732A172F,384D,4C4A,A6,AF,D2,28,20,D3,34,26)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_34;
#undef  ADO_Reserved_34
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_35
#define ADO_Reserved_35 GUID_BUILDER(ADO_Reserved_35,104E1F7E,8993,455C,B7,D8,58,CD,88,74,80,75)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_35;
#undef  ADO_Reserved_35
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_36
#define ADO_Reserved_36 GUID_BUILDER(ADO_Reserved_36,C12B8DFD,42F7,408E,AE,FB,A7,C2,FB,43,49,A7)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_36;
#undef  ADO_Reserved_36
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_37
#define ADO_Reserved_37 GUID_BUILDER(ADO_Reserved_37,EE881FC9,6C2F,45A2,BA,17,24,95,BC,72,4E,55)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_37;
#undef  ADO_Reserved_37
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_38
#define ADO_Reserved_38 GUID_BUILDER(ADO_Reserved_38,7381C764,646B,4F11,A6,73,13,50,98,9D,62,3A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_38;
#undef  ADO_Reserved_38
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_39
#define ADO_Reserved_39 GUID_BUILDER(ADO_Reserved_39,D8E4965C,F571,4771,8A,74,63,95,05,16,B0,88)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_39;
#undef  ADO_Reserved_39
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_40
#define ADO_Reserved_40 GUID_BUILDER(ADO_Reserved_40,2BE262E5,3A8C,4B07,A3,C3,3B,B7,40,EF,40,95)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_40;
#undef  ADO_Reserved_40
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_41
#define ADO_Reserved_41 GUID_BUILDER(ADO_Reserved_41,3E90A199,4F86,445C,84,8E,A6,17,86,B9,67,D1)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_41;
#undef  ADO_Reserved_41
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_42
#define ADO_Reserved_42 GUID_BUILDER(ADO_Reserved_42,DCD025E0,DA44,47E4,82,65,E4,A7,6B,85,29,0C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_42;
#undef  ADO_Reserved_42
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_43
#define ADO_Reserved_43 GUID_BUILDER(ADO_Reserved_43,31EFF562,FB6B,41D6,81,AD,30,1B,B0,53,9C,61)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_43;
#undef  ADO_Reserved_43
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_44
#define ADO_Reserved_44 GUID_BUILDER(ADO_Reserved_44,BD3ECD6B,F4A7,42FC,90,F1,75,D5,37,2A,F2,8F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_44;
#undef  ADO_Reserved_44
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_45
#define ADO_Reserved_45 GUID_BUILDER(ADO_Reserved_45,6EFBC56F,67E4,4F7D,BE,59,C5,D6,FA,21,B7,77)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_45;
#undef  ADO_Reserved_45
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_46
#define ADO_Reserved_46 GUID_BUILDER(ADO_Reserved_46,3BF5E1FC,B960,4564,86,54,07,B0,7A,AF,6E,4F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_46;
#undef  ADO_Reserved_46
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_47
#define ADO_Reserved_47 GUID_BUILDER(ADO_Reserved_47,2430F883,1462,4899,9A,DE,F7,24,27,FD,5E,E4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_47;
#undef  ADO_Reserved_47
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_48
#define ADO_Reserved_48 GUID_BUILDER(ADO_Reserved_48,AB663F07,BA4D,42CC,93,C6,F2,EA,9F,C8,BA,74)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_48;
#undef  ADO_Reserved_48
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_49
#define ADO_Reserved_49 GUID_BUILDER(ADO_Reserved_49,D808C6F7,36C0,4302,80,EE,C4,B7,00,F8,D2,38)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_49;
#undef  ADO_Reserved_49
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_50
#define ADO_Reserved_50 GUID_BUILDER(ADO_Reserved_50,AB146E06,E493,4DF0,A1,CD,07,D4,B0,74,46,C3)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_50;
#undef  ADO_Reserved_50
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_51
#define ADO_Reserved_51 GUID_BUILDER(ADO_Reserved_51,74F1FD51,9CB8,4186,8C,3D,DD,F3,55,2A,99,9B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_51;
#undef  ADO_Reserved_51
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_52
#define ADO_Reserved_52 GUID_BUILDER(ADO_Reserved_52,71701A97,5386,43B0,95,8D,3C,EE,40,57,B1,99)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_52;
#undef  ADO_Reserved_52
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_53
#define ADO_Reserved_53 GUID_BUILDER(ADO_Reserved_53,63CC6087,A6C6,4CCF,8E,D4,17,5B,91,A6,32,C5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_53;
#undef  ADO_Reserved_53
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_54
#define ADO_Reserved_54 GUID_BUILDER(ADO_Reserved_54,7323FD37,B7D8,4F8A,80,F4,E8,3D,0B,2A,73,B5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_54;
#undef  ADO_Reserved_54
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_55
#define ADO_Reserved_55 GUID_BUILDER(ADO_Reserved_55,5C666403,2A0A,4B12,8E,1D,41,19,88,DD,E0,0A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_55;
#undef  ADO_Reserved_55
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_56
#define ADO_Reserved_56 GUID_BUILDER(ADO_Reserved_56,ECA4C14C,5529,49DF,B1,3C,17,F0,22,DB,1B,A6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_56;
#undef  ADO_Reserved_56
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_57
#define ADO_Reserved_57 GUID_BUILDER(ADO_Reserved_57,304ADE1D,4458,4A6A,93,48,1F,7C,2E,64,D6,FA)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_57;
#undef  ADO_Reserved_57
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_58
#define ADO_Reserved_58 GUID_BUILDER(ADO_Reserved_58,D87A7AF2,FB3C,49BC,B2,69,F3,57,36,E7,23,2E)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_58;
#undef  ADO_Reserved_58
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_59
#define ADO_Reserved_59 GUID_BUILDER(ADO_Reserved_59,542D6D77,AECB,4AFF,B1,C6,54,EF,79,8F,61,ED)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_59;
#undef  ADO_Reserved_59
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_60
#define ADO_Reserved_60 GUID_BUILDER(ADO_Reserved_60,46359618,34AE,410E,AE,20,F3,D4,E1,BD,A6,BE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_60;
#undef  ADO_Reserved_60
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_61
#define ADO_Reserved_61 GUID_BUILDER(ADO_Reserved_61,F98DF79B,2935,464B,AA,08,CC,EF,F1,5F,71,32)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_61;
#undef  ADO_Reserved_61
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_62
#define ADO_Reserved_62 GUID_BUILDER(ADO_Reserved_62,214887FB,4867,4DD8,83,9D,4C,F0,BB,83,E1,95)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_62;
#undef  ADO_Reserved_62
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_63
#define ADO_Reserved_63 GUID_BUILDER(ADO_Reserved_63,C9B68C08,F663,4386,8F,5B,FA,BA,E0,27,43,6D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_63;
#undef  ADO_Reserved_63
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_64
#define ADO_Reserved_64 GUID_BUILDER(ADO_Reserved_64,F46511DD,10B6,49CF,AA,75,5E,E2,7C,FD,9E,A4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_64;
#undef  ADO_Reserved_64
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_65
#define ADO_Reserved_65 GUID_BUILDER(ADO_Reserved_65,C057EF87,F3A8,4890,A9,56,57,8C,07,CD,2E,F8)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_65;
#undef  ADO_Reserved_65
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_66
#define ADO_Reserved_66 GUID_BUILDER(ADO_Reserved_66,1C9E0666,1405,4DC5,BD,A7,65,F4,B4,16,1D,7B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_66;
#undef  ADO_Reserved_66
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_67
#define ADO_Reserved_67 GUID_BUILDER(ADO_Reserved_67,B91484C2,5E48,438C,91,CD,B9,D6,99,32,30,E4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_67;
#undef  ADO_Reserved_67
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_68
#define ADO_Reserved_68 GUID_BUILDER(ADO_Reserved_68,17D12BFE,6C9F,4229,87,95,60,20,6F,D1,45,35)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_68;
#undef  ADO_Reserved_68
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_69
#define ADO_Reserved_69 GUID_BUILDER(ADO_Reserved_69,5A816EA3,EE82,4F65,BC,76,74,07,E9,E5,43,58)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_69;
#undef  ADO_Reserved_69
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_70
#define ADO_Reserved_70 GUID_BUILDER(ADO_Reserved_70,3AD0DE2B,AA3E,4508,BE,9E,1E,AA,DF,1C,4D,8B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_70;
#undef  ADO_Reserved_70
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_71
#define ADO_Reserved_71 GUID_BUILDER(ADO_Reserved_71,54DC8B80,7869,4D90,AB,5C,8C,54,1A,74,EE,F8)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_71;
#undef  ADO_Reserved_71
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_72
#define ADO_Reserved_72 GUID_BUILDER(ADO_Reserved_72,80A200B0,5783,48E7,81,25,B9,E4,BF,59,F7,22)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_72;
#undef  ADO_Reserved_72
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_73
#define ADO_Reserved_73 GUID_BUILDER(ADO_Reserved_73,1502CB61,8C42,4C4B,B9,0C,3A,9E,4E,46,D1,BE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_73;
#undef  ADO_Reserved_73
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_74
#define ADO_Reserved_74 GUID_BUILDER(ADO_Reserved_74,70EB3F53,91A0,42F5,BE,50,F1,02,DE,C8,92,27)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_74;
#undef  ADO_Reserved_74
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_75
#define ADO_Reserved_75 GUID_BUILDER(ADO_Reserved_75,4680AA81,B27C,4A8F,83,F9,6F,B7,E1,8E,D2,3C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_75;
#undef  ADO_Reserved_75
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_76
#define ADO_Reserved_76 GUID_BUILDER(ADO_Reserved_76,EF31F9EB,4541,4FCB,8D,67,59,2C,85,50,93,05)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_76;
#undef  ADO_Reserved_76
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_77
#define ADO_Reserved_77 GUID_BUILDER(ADO_Reserved_77,88B77D15,997E,4E3A,83,20,3B,37,83,52,86,D5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_77;
#undef  ADO_Reserved_77
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_78
#define ADO_Reserved_78 GUID_BUILDER(ADO_Reserved_78,D03A3AA8,1AAC,4867,93,C9,5F,51,D8,7D,6A,74)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_78;
#undef  ADO_Reserved_78
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_79
#define ADO_Reserved_79 GUID_BUILDER(ADO_Reserved_79,47022458,17E7,4BD7,90,81,85,B4,0B,03,6D,5B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_79;
#undef  ADO_Reserved_79
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_80
#define ADO_Reserved_80 GUID_BUILDER(ADO_Reserved_80,9E5BEE82,F410,44C7,9D,6D,3F,7D,D2,8B,A7,CC)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_80;
#undef  ADO_Reserved_80
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_81
#define ADO_Reserved_81 GUID_BUILDER(ADO_Reserved_81,278A1C47,3C39,41C7,A3,FB,7C,2E,62,0B,E4,44)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_81;
#undef  ADO_Reserved_81
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_82
#define ADO_Reserved_82 GUID_BUILDER(ADO_Reserved_82,964CBF05,8084,4C15,9C,F5,8C,4B,81,41,B4,AE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_82;
#undef  ADO_Reserved_82
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_83
#define ADO_Reserved_83 GUID_BUILDER(ADO_Reserved_83,A86296A0,F272,4ACD,83,06,FF,CA,FF,89,14,A9)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_83;
#undef  ADO_Reserved_83
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_84
#define ADO_Reserved_84 GUID_BUILDER(ADO_Reserved_84,F805FC7C,7C4A,43A1,B0,14,71,EA,0E,EB,EA,5F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_84;
#undef  ADO_Reserved_84
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_85
#define ADO_Reserved_85 GUID_BUILDER(ADO_Reserved_85,33E6E9B6,0BEA,4549,90,CB,3B,64,12,DB,8C,F5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_85;
#undef  ADO_Reserved_85
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_86
#define ADO_Reserved_86 GUID_BUILDER(ADO_Reserved_86,7337E3DC,219F,4D9E,82,5B,0A,2C,18,4E,C0,DE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_86;
#undef  ADO_Reserved_86
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_87
#define ADO_Reserved_87 GUID_BUILDER(ADO_Reserved_87,7397BAFC,354E,4F18,9F,76,C3,3A,4E,EF,6D,20)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_87;
#undef  ADO_Reserved_87
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_88
#define ADO_Reserved_88 GUID_BUILDER(ADO_Reserved_88,5EC2D163,E671,4186,BE,72,BF,FF,72,D5,7A,5C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_88;
#undef  ADO_Reserved_88
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_89
#define ADO_Reserved_89 GUID_BUILDER(ADO_Reserved_89,8B37B801,0A35,4F97,A3,43,82,57,B3,E7,6C,79)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_89;
#undef  ADO_Reserved_89
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_90
#define ADO_Reserved_90 GUID_BUILDER(ADO_Reserved_90,FAD396B6,EE4E,4F70,85,54,E8,23,9E,47,05,29)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_90;
#undef  ADO_Reserved_90
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_91
#define ADO_Reserved_91 GUID_BUILDER(ADO_Reserved_91,6063972C,395B,4FEF,A0,04,ED,95,E7,D8,72,0D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_91;
#undef  ADO_Reserved_91
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_92
#define ADO_Reserved_92 GUID_BUILDER(ADO_Reserved_92,85AEED72,A1F8,4597,82,32,F8,40,EF,C9,21,09)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_92;
#undef  ADO_Reserved_92
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_93
#define ADO_Reserved_93 GUID_BUILDER(ADO_Reserved_93,CE4FD8FF,553A,4424,B1,EA,3E,DF,11,42,AD,8B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_93;
#undef  ADO_Reserved_93
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_94
#define ADO_Reserved_94 GUID_BUILDER(ADO_Reserved_94,1A856A0F,0844,4DE4,AC,7B,75,30,62,56,39,86)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_94;
#undef  ADO_Reserved_94
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_95
#define ADO_Reserved_95 GUID_BUILDER(ADO_Reserved_95,09A742A1,19ED,43BB,85,E9,99,23,DE,C4,17,F7)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_95;
#undef  ADO_Reserved_95
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_96
#define ADO_Reserved_96 GUID_BUILDER(ADO_Reserved_96,3695BD0C,9DE6,4895,84,E6,B2,4C,E7,55,47,02)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_96;
#undef  ADO_Reserved_96
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_97
#define ADO_Reserved_97 GUID_BUILDER(ADO_Reserved_97,8802531F,6EA8,4A55,8A,18,05,97,86,3C,DA,38)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_97;
#undef  ADO_Reserved_97
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_98
#define ADO_Reserved_98 GUID_BUILDER(ADO_Reserved_98,498E70F0,B13F,4804,AD,D5,72,E8,0E,28,05,E7)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_98;
#undef  ADO_Reserved_98
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_99
#define ADO_Reserved_99 GUID_BUILDER(ADO_Reserved_99,50D0E90F,E3A4,4A93,8B,48,71,21,66,E8,87,CD)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_99;
#undef  ADO_Reserved_99
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_100
#define ADO_Reserved_100    GUID_BUILDER(ADO_Reserved_100,F1D30550,8515,4F8B,93,E1,1E,F0,12,1B,4B,D0)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_100;
#undef  ADO_Reserved_100
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_101
#define ADO_Reserved_101    GUID_BUILDER(ADO_Reserved_101,901CDA31,8CDB,4A5B,91,6B,63,EA,90,1D,8C,E0)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_101;
#undef  ADO_Reserved_101
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_102
#define ADO_Reserved_102    GUID_BUILDER(ADO_Reserved_102,00BDA239,1094,4AEF,93,AD,7C,E2,73,6C,42,25)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_102;
#undef  ADO_Reserved_102
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_103
#define ADO_Reserved_103    GUID_BUILDER(ADO_Reserved_103,DCA4E51E,250E,4AB3,B4,90,F2,CB,9E,8F,6C,C4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_103;
#undef  ADO_Reserved_103
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_104
#define ADO_Reserved_104    GUID_BUILDER(ADO_Reserved_104,24679EBD,8535,4494,A9,1C,18,91,F0,75,5B,6F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_104;
#undef  ADO_Reserved_104
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_105
#define ADO_Reserved_105    GUID_BUILDER(ADO_Reserved_105,F041739E,F37E,4925,94,25,FB,51,5E,56,0F,54)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_105;
#undef  ADO_Reserved_105
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_106
#define ADO_Reserved_106    GUID_BUILDER(ADO_Reserved_106,FECACBBF,A73C,4616,84,2F,FE,F5,72,85,70,AB)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_106;
#undef  ADO_Reserved_106
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_107
#define ADO_Reserved_107    GUID_BUILDER(ADO_Reserved_107,DBAD7368,1DED,4A77,B8,0A,1A,EB,12,99,BD,B3)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_107;
#undef  ADO_Reserved_107
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_108
#define ADO_Reserved_108    GUID_BUILDER(ADO_Reserved_108,CFDE81B8,66EF,4503,84,A8,7E,8F,C8,AB,0B,31)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_108;
#undef  ADO_Reserved_108
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_109
#define ADO_Reserved_109    GUID_BUILDER(ADO_Reserved_109,9B7484FA,023A,4FFB,A2,94,11,A6,E5,97,AB,35)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_109;
#undef  ADO_Reserved_109
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_110
#define ADO_Reserved_110    GUID_BUILDER(ADO_Reserved_110,54F0F09C,1201,49A9,B4,65,6B,02,9B,5F,E3,12)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_110;
#undef  ADO_Reserved_110
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_111
#define ADO_Reserved_111    GUID_BUILDER(ADO_Reserved_111,BFFA01F8,EAE7,4FA1,BF,74,37,73,3F,BF,36,4C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_111;
#undef  ADO_Reserved_111
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_112
#define ADO_Reserved_112    GUID_BUILDER(ADO_Reserved_112,12FAD291,4AAB,4038,9D,D1,04,E4,E7,A9,E0,F4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_112;
#undef  ADO_Reserved_112
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_113
#define ADO_Reserved_113    GUID_BUILDER(ADO_Reserved_113,8D2AF964,C489,4D77,A8,17,A0,4D,B1,DB,26,A5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_113;
#undef  ADO_Reserved_113
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_114
#define ADO_Reserved_114    GUID_BUILDER(ADO_Reserved_114,79F89DD7,BE86,4B36,BE,9B,FA,75,24,18,55,68)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_114;
#undef  ADO_Reserved_114
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_115
#define ADO_Reserved_115    GUID_BUILDER(ADO_Reserved_115,4387D7FA,7A52,4F67,BF,B6,7E,7D,7A,B7,C9,DE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_115;
#undef  ADO_Reserved_115
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_116
#define ADO_Reserved_116    GUID_BUILDER(ADO_Reserved_116,7571252F,0E49,4F4B,A3,87,9E,D9,70,54,68,D8)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_116;
#undef  ADO_Reserved_116
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_117
#define ADO_Reserved_117    GUID_BUILDER(ADO_Reserved_117,0DAB016B,6BA4,470F,98,1A,2B,A7,65,D4,60,4B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_117;
#undef  ADO_Reserved_117
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_118
#define ADO_Reserved_118    GUID_BUILDER(ADO_Reserved_118,E97D87A3,8A95,4080,8C,A9,ED,9F,05,1A,B7,B2)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_118;
#undef  ADO_Reserved_118
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_119
#define ADO_Reserved_119    GUID_BUILDER(ADO_Reserved_119,C9EA1598,2D23,4978,9B,33,3D,2C,C4,0A,B7,A1)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_119;
#undef  ADO_Reserved_119
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_120
#define ADO_Reserved_120    GUID_BUILDER(ADO_Reserved_120,E41CA9FC,7FC9,4831,90,CE,F5,33,96,CE,42,C3)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_120;
#undef  ADO_Reserved_120
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_121
#define ADO_Reserved_121    GUID_BUILDER(ADO_Reserved_121,15DF0905,4ACC,44F7,A0,1E,0F,EF,56,3C,C4,E5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_121;
#undef  ADO_Reserved_121
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_122
#define ADO_Reserved_122    GUID_BUILDER(ADO_Reserved_122,D2879A0E,D0B3,42A2,A1,16,D1,5E,13,C7,51,77)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_122;
#undef  ADO_Reserved_122
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_123
#define ADO_Reserved_123    GUID_BUILDER(ADO_Reserved_123,A999A8D2,5E83,4C0E,83,97,18,33,19,32,79,CD)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_123;
#undef  ADO_Reserved_123
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_124
#define ADO_Reserved_124    GUID_BUILDER(ADO_Reserved_124,C6AFAE72,B3FF,48AB,B1,EE,F5,EE,F9,05,DF,47)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_124;
#undef  ADO_Reserved_124
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_125
#define ADO_Reserved_125    GUID_BUILDER(ADO_Reserved_125,0DEADF50,0940,4F0E,AC,3B,94,80,B7,32,2B,1B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_125;
#undef  ADO_Reserved_125
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_126
#define ADO_Reserved_126    GUID_BUILDER(ADO_Reserved_126,61278818,2FE6,4892,8B,95,A7,5C,AC,6E,21,BB)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_126;
#undef  ADO_Reserved_126
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_127
#define ADO_Reserved_127    GUID_BUILDER(ADO_Reserved_127,3AC2BED7,1111,4E55,B2,06,1F,54,18,94,4C,BA)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_127;
#undef  ADO_Reserved_127
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_128
#define ADO_Reserved_128    GUID_BUILDER(ADO_Reserved_128,3D4751E2,04B8,4593,A0,0D,3A,4B,94,67,4B,E9)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_128;
#undef  ADO_Reserved_128
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_129
#define ADO_Reserved_129    GUID_BUILDER(ADO_Reserved_129,69BC6751,FE10,4B3F,89,35,40,2F,A5,FD,04,82)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_129;
#undef  ADO_Reserved_129
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_130
#define ADO_Reserved_130    GUID_BUILDER(ADO_Reserved_130,5867AF81,995A,4686,8B,CB,13,B6,8B,10,26,8A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_130;
#undef  ADO_Reserved_130
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_131
#define ADO_Reserved_131    GUID_BUILDER(ADO_Reserved_131,DA46C62F,BDCD,4745,A3,CA,4E,C9,FA,AB,E1,10)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_131;
#undef  ADO_Reserved_131
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_132
#define ADO_Reserved_132    GUID_BUILDER(ADO_Reserved_132,93028AA6,EECC,482F,B3,A4,2F,D4,13,04,96,5E)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_132;
#undef  ADO_Reserved_132
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_133
#define ADO_Reserved_133    GUID_BUILDER(ADO_Reserved_133,AB14F604,D05E,4E50,A4,5B,A8,10,48,E3,A4,75)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_133;
#undef  ADO_Reserved_133
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_134
#define ADO_Reserved_134    GUID_BUILDER(ADO_Reserved_134,35267875,8420,4226,87,C0,25,00,58,56,0F,D2)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_134;
#undef  ADO_Reserved_134
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_135
#define ADO_Reserved_135    GUID_BUILDER(ADO_Reserved_135,16E34932,EEFA,440E,A7,86,6A,36,D2,C6,21,69)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_135;
#undef  ADO_Reserved_135
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_136
#define ADO_Reserved_136    GUID_BUILDER(ADO_Reserved_136,2710A15A,B2B0,46EC,BD,EC,E2,2E,A8,A6,28,FA)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_136;
#undef  ADO_Reserved_136
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_137
#define ADO_Reserved_137    GUID_BUILDER(ADO_Reserved_137,2777696F,CB34,4CC4,A0,A9,02,EA,15,16,63,DD)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_137;
#undef  ADO_Reserved_137
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_138
#define ADO_Reserved_138    GUID_BUILDER(ADO_Reserved_138,D11CA1A0,A261,4BA2,81,68,46,52,32,9A,60,77)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_138;
#undef  ADO_Reserved_138
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_139
#define ADO_Reserved_139    GUID_BUILDER(ADO_Reserved_139,C33509A8,883F,4BEA,AF,B5,35,26,CF,0B,8B,E1)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_139;
#undef  ADO_Reserved_139
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_140
#define ADO_Reserved_140    GUID_BUILDER(ADO_Reserved_140,DEBDC8E1,4F02,43E1,8C,88,0B,A8,E1,50,6B,F5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_140;
#undef  ADO_Reserved_140
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_141
#define ADO_Reserved_141    GUID_BUILDER(ADO_Reserved_141,552F8531,3F79,4DB3,87,7B,8E,54,C3,5B,38,54)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_141;
#undef  ADO_Reserved_141
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_142
#define ADO_Reserved_142    GUID_BUILDER(ADO_Reserved_142,1E6A2BF4,241C,48A1,90,66,C6,E1,E5,2B,0A,4B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_142;
#undef  ADO_Reserved_142
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_143
#define ADO_Reserved_143    GUID_BUILDER(ADO_Reserved_143,8E5B2A8D,1F0D,429D,94,95,16,F8,E9,58,06,80)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_143;
#undef  ADO_Reserved_143
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_144
#define ADO_Reserved_144    GUID_BUILDER(ADO_Reserved_144,57FAEC9D,5CDE,4EBE,84,A1,5A,CB,75,7C,D4,51)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_144;
#undef  ADO_Reserved_144
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_145
#define ADO_Reserved_145    GUID_BUILDER(ADO_Reserved_145,707B03C3,A3B0,4F00,81,61,6E,3F,02,7F,F0,B3)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_145;
#undef  ADO_Reserved_145
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_146
#define ADO_Reserved_146    GUID_BUILDER(ADO_Reserved_146,5DD552F4,0718,4BDD,82,6C,7C,C3,5C,DA,1D,93)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_146;
#undef  ADO_Reserved_146
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_147
#define ADO_Reserved_147    GUID_BUILDER(ADO_Reserved_147,F3247F33,E377,4A44,A9,37,AC,E6,36,F6,58,1F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_147;
#undef  ADO_Reserved_147
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_148
#define ADO_Reserved_148    GUID_BUILDER(ADO_Reserved_148,E7C324C4,38A5,42A8,99,FF,34,5D,AD,8C,D2,29)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_148;
#undef  ADO_Reserved_148
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_149
#define ADO_Reserved_149    GUID_BUILDER(ADO_Reserved_149,D14FCA70,390D,4158,B5,C3,9A,02,D1,F7,85,87)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_149;
#undef  ADO_Reserved_149
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_150
#define ADO_Reserved_150    GUID_BUILDER(ADO_Reserved_150,58D30B5F,92A5,4EF4,8E,45,A0,24,A9,CD,F9,FE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_150;
#undef  ADO_Reserved_150
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_151
#define ADO_Reserved_151    GUID_BUILDER(ADO_Reserved_151,9673DF76,73E4,4C66,89,14,7F,A4,17,43,6C,4A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_151;
#undef  ADO_Reserved_151
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_152
#define ADO_Reserved_152    GUID_BUILDER(ADO_Reserved_152,9FA8A7E1,CF3C,4A61,BE,10,1D,85,5F,A0,D5,08)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_152;
#undef  ADO_Reserved_152
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_153
#define ADO_Reserved_153    GUID_BUILDER(ADO_Reserved_153,B657729F,6CC7,4392,BD,56,DC,ED,6E,53,F6,4C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_153;
#undef  ADO_Reserved_153
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_154
#define ADO_Reserved_154    GUID_BUILDER(ADO_Reserved_154,06E5224B,8C27,4F41,8F,B7,C6,41,E4,C5,04,2D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_154;
#undef  ADO_Reserved_154
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_155
#define ADO_Reserved_155    GUID_BUILDER(ADO_Reserved_155,2268A619,CC1D,4F72,B8,3F,79,63,C0,13,B1,3D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_155;
#undef  ADO_Reserved_155
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_156
#define ADO_Reserved_156    GUID_BUILDER(ADO_Reserved_156,FB4810F3,3A65,4C33,B3,99,B5,C9,33,11,11,D7)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_156;
#undef  ADO_Reserved_156
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_157
#define ADO_Reserved_157    GUID_BUILDER(ADO_Reserved_157,9011BE74,6C9D,44F7,BE,2C,8A,2A,BB,62,51,AC)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_157;
#undef  ADO_Reserved_157
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_158
#define ADO_Reserved_158    GUID_BUILDER(ADO_Reserved_158,3145C182,82C6,4082,BB,E7,79,1A,2F,49,6C,B1)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_158;
#undef  ADO_Reserved_158
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_159
#define ADO_Reserved_159    GUID_BUILDER(ADO_Reserved_159,D8865377,8799,4C08,97,E5,D6,7E,88,6F,F5,49)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_159;
#undef  ADO_Reserved_159
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_160
#define ADO_Reserved_160    GUID_BUILDER(ADO_Reserved_160,8993232E,8AFA,4552,A7,8C,C6,6C,9D,3A,E6,D0)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_160;
#undef  ADO_Reserved_160
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_161
#define ADO_Reserved_161    GUID_BUILDER(ADO_Reserved_161,40AF1931,8721,427B,83,5E,50,87,79,BD,1E,B2)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_161;
#undef  ADO_Reserved_161
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_162
#define ADO_Reserved_162    GUID_BUILDER(ADO_Reserved_162,9C6E2B26,4468,427C,8C,F5,01,14,7D,B8,DF,22)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_162;
#undef  ADO_Reserved_162
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_163
#define ADO_Reserved_163    GUID_BUILDER(ADO_Reserved_163,3537FA93,7E92,4CE0,80,96,EF,DC,1A,80,A8,95)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_163;
#undef  ADO_Reserved_163
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_164
#define ADO_Reserved_164    GUID_BUILDER(ADO_Reserved_164,36992492,3E17,47C1,AB,98,5F,0C,49,B4,6A,25)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_164;
#undef  ADO_Reserved_164
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_165
#define ADO_Reserved_165    GUID_BUILDER(ADO_Reserved_165,01662EDB,CE23,4215,AE,9D,51,51,E6,DA,36,3C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_165;
#undef  ADO_Reserved_165
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_166
#define ADO_Reserved_166    GUID_BUILDER(ADO_Reserved_166,80B4A97B,5256,4397,89,CD,4E,F9,91,0F,1D,E6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_166;
#undef  ADO_Reserved_166
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_167
#define ADO_Reserved_167    GUID_BUILDER(ADO_Reserved_167,C2341A38,2C6B,414E,96,A8,8B,5E,47,F8,14,DA)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_167;
#undef  ADO_Reserved_167
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_168
#define ADO_Reserved_168    GUID_BUILDER(ADO_Reserved_168,5C2B7578,53FA,4ACE,8E,6C,39,18,2F,68,D2,67)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_168;
#undef  ADO_Reserved_168
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_169
#define ADO_Reserved_169    GUID_BUILDER(ADO_Reserved_169,B80C1E36,611B,49D4,97,19,4E,0C,59,0E,2E,E1)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_169;
#undef  ADO_Reserved_169
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_170
#define ADO_Reserved_170    GUID_BUILDER(ADO_Reserved_170,BA269EB4,B741,4FB2,A9,C9,52,4C,9D,BE,7C,16)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_170;
#undef  ADO_Reserved_170
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_171
#define ADO_Reserved_171    GUID_BUILDER(ADO_Reserved_171,EE49769D,1028,4429,A9,66,2F,A8,1D,70,EE,19)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_171;
#undef  ADO_Reserved_171
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_172
#define ADO_Reserved_172    GUID_BUILDER(ADO_Reserved_172,541FC621,D6E6,4CC4,B4,98,8E,4F,AA,A0,65,BF)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_172;
#undef  ADO_Reserved_172
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_173
#define ADO_Reserved_173    GUID_BUILDER(ADO_Reserved_173,AA8B544C,4067,4E00,96,09,95,EE,21,68,AF,CE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_173;
#undef  ADO_Reserved_173
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_174
#define ADO_Reserved_174    GUID_BUILDER(ADO_Reserved_174,5B161B2B,D02C,4300,A1,54,CF,DC,25,3B,13,0D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_174;
#undef  ADO_Reserved_174
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_175
#define ADO_Reserved_175    GUID_BUILDER(ADO_Reserved_175,81F62203,182E,42DE,B1,B7,63,5F,C6,6F,6E,9E)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_175;
#undef  ADO_Reserved_175
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_176
#define ADO_Reserved_176    GUID_BUILDER(ADO_Reserved_176,04934BDD,A530,48EC,91,CE,11,83,42,5B,DB,61)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_176;
#undef  ADO_Reserved_176
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_177
#define ADO_Reserved_177    GUID_BUILDER(ADO_Reserved_177,F6997841,9A99,48AA,B0,56,8C,75,17,06,41,7F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_177;
#undef  ADO_Reserved_177
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_178
#define ADO_Reserved_178    GUID_BUILDER(ADO_Reserved_178,353FE3F1,DE50,45EE,91,E9,3E,62,E3,C7,86,04)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_178;
#undef  ADO_Reserved_178
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_179
#define ADO_Reserved_179    GUID_BUILDER(ADO_Reserved_179,F142C8C6,9E24,422E,81,BD,D2,94,7F,93,94,D4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_179;
#undef  ADO_Reserved_179
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_180
#define ADO_Reserved_180    GUID_BUILDER(ADO_Reserved_180,95951773,9566,46C9,86,B0,40,ED,25,46,02,93)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_180;
#undef  ADO_Reserved_180
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_181
#define ADO_Reserved_181    GUID_BUILDER(ADO_Reserved_181,54140563,0F25,4F56,9D,8F,B6,DE,CB,96,DC,E4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_181;
#undef  ADO_Reserved_181
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_182
#define ADO_Reserved_182    GUID_BUILDER(ADO_Reserved_182,91A48243,AE16,48CF,82,29,00,00,F8,3C,5E,FC)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_182;
#undef  ADO_Reserved_182
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_183
#define ADO_Reserved_183    GUID_BUILDER(ADO_Reserved_183,E0FA1A1F,3967,4392,AB,1A,E2,8B,85,04,68,CA)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_183;
#undef  ADO_Reserved_183
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_184
#define ADO_Reserved_184    GUID_BUILDER(ADO_Reserved_184,5582D772,ABAC,4A85,A0,B3,2E,65,E1,71,10,53)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_184;
#undef  ADO_Reserved_184
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_185
#define ADO_Reserved_185    GUID_BUILDER(ADO_Reserved_185,1CD1F347,8FB4,49A2,B5,65,A6,74,A0,C1,45,0E)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_185;
#undef  ADO_Reserved_185
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_186
#define ADO_Reserved_186    GUID_BUILDER(ADO_Reserved_186,0EC3AA4E,FEF5,4A5C,BD,0A,E9,CD,B7,6A,5F,30)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_186;
#undef  ADO_Reserved_186
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_187
#define ADO_Reserved_187    GUID_BUILDER(ADO_Reserved_187,4118414D,4A21,46DA,88,C1,EF,A7,01,8C,45,27)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_187;
#undef  ADO_Reserved_187
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_188
#define ADO_Reserved_188    GUID_BUILDER(ADO_Reserved_188,D5C1CC0D,E38E,4CB6,89,D9,99,27,7F,12,D1,9E)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_188;
#undef  ADO_Reserved_188
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_189
#define ADO_Reserved_189    GUID_BUILDER(ADO_Reserved_189,0956B82A,94A7,474E,A5,05,1A,76,26,36,AF,08)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_189;
#undef  ADO_Reserved_189
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_190
#define ADO_Reserved_190    GUID_BUILDER(ADO_Reserved_190,2CBF62AB,B8E4,48D0,B5,01,69,CF,63,3C,AA,E6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_190;
#undef  ADO_Reserved_190
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_191
#define ADO_Reserved_191    GUID_BUILDER(ADO_Reserved_191,C02B8113,AECF,4A34,B3,E9,5B,52,4E,51,44,B5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_191;
#undef  ADO_Reserved_191
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_192
#define ADO_Reserved_192    GUID_BUILDER(ADO_Reserved_192,1C90947B,4A3A,4ECD,8C,70,F4,3F,E5,2D,46,45)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_192;
#undef  ADO_Reserved_192
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_193
#define ADO_Reserved_193    GUID_BUILDER(ADO_Reserved_193,48175E98,6672,4DB4,B5,74,8C,93,25,8D,BF,14)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_193;
#undef  ADO_Reserved_193
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_194
#define ADO_Reserved_194    GUID_BUILDER(ADO_Reserved_194,99CB88BF,E5C1,4AF0,85,00,72,36,47,DC,D2,05)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_194;
#undef  ADO_Reserved_194
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_195
#define ADO_Reserved_195    GUID_BUILDER(ADO_Reserved_195,6A2CC3CC,7855,4B27,86,F7,98,6B,AA,F9,5F,0F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_195;
#undef  ADO_Reserved_195
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_196
#define ADO_Reserved_196    GUID_BUILDER(ADO_Reserved_196,7640B336,9EBB,4017,9E,EE,54,01,F4,EC,B1,70)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_196;
#undef  ADO_Reserved_196
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_197
#define ADO_Reserved_197    GUID_BUILDER(ADO_Reserved_197,507B39E1,2965,42EA,92,66,55,8D,E4,31,53,73)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_197;
#undef  ADO_Reserved_197
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_198
#define ADO_Reserved_198    GUID_BUILDER(ADO_Reserved_198,58C591FA,37FF,4428,A0,4A,46,71,98,17,74,8C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_198;
#undef  ADO_Reserved_198
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_199
#define ADO_Reserved_199    GUID_BUILDER(ADO_Reserved_199,24BE98E9,B43D,49B5,9C,41,20,AF,C2,FE,1D,8D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_199;
#undef  ADO_Reserved_199
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_200
#define ADO_Reserved_200    GUID_BUILDER(ADO_Reserved_200,041956C5,B951,4C8F,8C,61,8E,12,34,E1,E9,61)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_200;
#undef  ADO_Reserved_200
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_201
#define ADO_Reserved_201    GUID_BUILDER(ADO_Reserved_201,6C98D05C,D366,48B4,80,E3,8F,A1,CC,06,1D,B7)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_201;
#undef  ADO_Reserved_201
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_202
#define ADO_Reserved_202    GUID_BUILDER(ADO_Reserved_202,09570783,A1E8,4A52,BA,74,6C,AC,02,C0,14,8C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_202;
#undef  ADO_Reserved_202
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_203
#define ADO_Reserved_203    GUID_BUILDER(ADO_Reserved_203,96C8C205,FD0D,4B56,9A,12,39,B3,7E,9D,07,4D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_203;
#undef  ADO_Reserved_203
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_204
#define ADO_Reserved_204    GUID_BUILDER(ADO_Reserved_204,136C40E1,366E,4BA6,AF,71,C4,9A,EF,17,3F,C0)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_204;
#undef  ADO_Reserved_204
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_205
#define ADO_Reserved_205    GUID_BUILDER(ADO_Reserved_205,A298C799,06FB,466E,B5,6D,3E,CC,6D,0C,D6,75)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_205;
#undef  ADO_Reserved_205
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_206
#define ADO_Reserved_206    GUID_BUILDER(ADO_Reserved_206,41A96542,08F2,4609,B7,6A,ED,93,E5,5B,8C,60)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_206;
#undef  ADO_Reserved_206
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_207
#define ADO_Reserved_207    GUID_BUILDER(ADO_Reserved_207,65A3B57E,06F9,4572,80,91,17,3F,C4,A6,5A,16)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_207;
#undef  ADO_Reserved_207
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_208
#define ADO_Reserved_208    GUID_BUILDER(ADO_Reserved_208,114F3E9D,5431,4DC1,95,42,9B,85,44,CF,83,B2)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_208;
#undef  ADO_Reserved_208
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_209
#define ADO_Reserved_209    GUID_BUILDER(ADO_Reserved_209,41DE92D4,9F2A,4085,8C,C1,C1,04,3E,5B,11,12)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_209;
#undef  ADO_Reserved_209
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_210
#define ADO_Reserved_210    GUID_BUILDER(ADO_Reserved_210,E32A7A98,FF1E,45C9,AA,51,5F,86,9A,9A,48,57)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_210;
#undef  ADO_Reserved_210
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_211
#define ADO_Reserved_211    GUID_BUILDER(ADO_Reserved_211,5E5A209F,3EFC,48BC,A7,1E,F4,CE,BE,4C,A6,25)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_211;
#undef  ADO_Reserved_211
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_212
#define ADO_Reserved_212    GUID_BUILDER(ADO_Reserved_212,C556C1CC,8B2E,482B,B7,8C,E2,F6,FD,A0,4F,4D)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_212;
#undef  ADO_Reserved_212
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_213
#define ADO_Reserved_213    GUID_BUILDER(ADO_Reserved_213,39C54FD9,A22A,43D4,A4,36,D9,CB,C5,53,D5,5A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_213;
#undef  ADO_Reserved_213
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_214
#define ADO_Reserved_214    GUID_BUILDER(ADO_Reserved_214,750E0BA2,E25C,479F,B0,C1,58,44,A1,4D,08,77)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_214;
#undef  ADO_Reserved_214
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_215
#define ADO_Reserved_215    GUID_BUILDER(ADO_Reserved_215,7ECBDB2C,C5C2,48FB,8A,3A,30,B7,E7,BD,17,25)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_215;
#undef  ADO_Reserved_215
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_216
#define ADO_Reserved_216    GUID_BUILDER(ADO_Reserved_216,0BF18AC7,8BE1,49E6,A8,57,EA,89,3A,83,58,F5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_216;
#undef  ADO_Reserved_216
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_217
#define ADO_Reserved_217    GUID_BUILDER(ADO_Reserved_217,DA74EAB6,AAFE,42AD,8A,0D,B2,73,35,0C,82,E3)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_217;
#undef  ADO_Reserved_217
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_218
#define ADO_Reserved_218    GUID_BUILDER(ADO_Reserved_218,F6A3D173,E366,424A,AD,0C,25,5C,32,2D,09,80)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_218;
#undef  ADO_Reserved_218
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_219
#define ADO_Reserved_219    GUID_BUILDER(ADO_Reserved_219,7CD83BA3,0516,4366,BB,85,DE,53,03,F7,75,08)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_219;
#undef  ADO_Reserved_219
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_220
#define ADO_Reserved_220    GUID_BUILDER(ADO_Reserved_220,42EDFC05,3A70,4F5C,8C,32,06,5E,61,45,3C,BE)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_220;
#undef  ADO_Reserved_220
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_221
#define ADO_Reserved_221    GUID_BUILDER(ADO_Reserved_221,624BC037,05B0,44E1,85,A7,73,C4,7F,A0,CC,04)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_221;
#undef  ADO_Reserved_221
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_222
#define ADO_Reserved_222    GUID_BUILDER(ADO_Reserved_222,8811F8DD,FA15,4FA6,A7,6E,7E,DA,E7,0D,EC,D4)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_222;
#undef  ADO_Reserved_222
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_223
#define ADO_Reserved_223    GUID_BUILDER(ADO_Reserved_223,DD310D89,9F22,4F49,89,8C,AF,A2,7F,AF,11,1C)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_223;
#undef  ADO_Reserved_223
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_224
#define ADO_Reserved_224    GUID_BUILDER(ADO_Reserved_224,321E3A7D,DF0E,4FF7,98,5D,F6,E6,73,FD,E2,9F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_224;
#undef  ADO_Reserved_224
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_225
#define ADO_Reserved_225    GUID_BUILDER(ADO_Reserved_225,036D1B77,3737,47CB,9E,75,31,13,13,2D,32,B8)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_225;
#undef  ADO_Reserved_225
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_226
#define ADO_Reserved_226    GUID_BUILDER(ADO_Reserved_226,A46B9E8C,4740,4919,86,34,A3,57,73,F6,53,2F)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_226;
#undef  ADO_Reserved_226
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_227
#define ADO_Reserved_227    GUID_BUILDER(ADO_Reserved_227,7C064E3A,014E,4733,9D,00,5D,03,13,F7,B7,F5)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_227;
#undef  ADO_Reserved_227
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_228
#define ADO_Reserved_228    GUID_BUILDER(ADO_Reserved_228,7CBFF995,A041,4B0A,B7,9B,16,3A,74,2C,DC,CF)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_228;
#undef  ADO_Reserved_228
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_229
#define ADO_Reserved_229    GUID_BUILDER(ADO_Reserved_229,C3271965,BA03,4ABC,8F,D8,98,97,7E,4C,B3,9A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_229;
#undef  ADO_Reserved_229
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_230
#define ADO_Reserved_230    GUID_BUILDER(ADO_Reserved_230,565DC4B1,8D7A,41C6,AE,01,6C,EF,63,46,4D,5E)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_230;
#undef  ADO_Reserved_230
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_231
#define ADO_Reserved_231    GUID_BUILDER(ADO_Reserved_231,3331E567,EB74,45D2,86,32,20,43,47,DB,BE,04)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_231;
#undef  ADO_Reserved_231
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_232
#define ADO_Reserved_232    GUID_BUILDER(ADO_Reserved_232,3CEE44A8,6FC5,4CD5,AA,9D,1B,67,4C,B6,2E,EC)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_232;
#undef  ADO_Reserved_232
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_233
#define ADO_Reserved_233    GUID_BUILDER(ADO_Reserved_233,CD1BE145,71B9,4CCD,A7,AF,5B,BA,A0,2A,51,E6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_233;
#undef  ADO_Reserved_233
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_234
#define ADO_Reserved_234    GUID_BUILDER(ADO_Reserved_234,4203C429,F3F0,4DD7,91,24,51,E0,13,95,5E,7A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_234;
#undef  ADO_Reserved_234
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_235
#define ADO_Reserved_235    GUID_BUILDER(ADO_Reserved_235,BB256836,2804,492F,9C,B2,CF,83,CB,82,63,8A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_235;
#undef  ADO_Reserved_235
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_236
#define ADO_Reserved_236    GUID_BUILDER(ADO_Reserved_236,8B247756,34AA,45EF,B1,24,A9,60,66,AC,E8,D6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_236;
#undef  ADO_Reserved_236
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_237
#define ADO_Reserved_237    GUID_BUILDER(ADO_Reserved_237,EF1CF73C,4915,4289,AD,C4,DD,DA,62,70,70,A6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_237;
#undef  ADO_Reserved_237
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_238
#define ADO_Reserved_238    GUID_BUILDER(ADO_Reserved_238,D0EB0A94,91A0,49D3,97,26,94,52,66,5A,FE,53)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_238;
#undef  ADO_Reserved_238
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_239
#define ADO_Reserved_239    GUID_BUILDER(ADO_Reserved_239,D6F5003E,4C06,47B1,89,E9,D6,3C,3D,7D,41,B6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_239;
#undef  ADO_Reserved_239
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_240
#define ADO_Reserved_240    GUID_BUILDER(ADO_Reserved_240,AA803151,F4AE,4CE3,BC,92,97,1C,84,2E,F5,BC)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_240;
#undef  ADO_Reserved_240
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_241
#define ADO_Reserved_241    GUID_BUILDER(ADO_Reserved_241,C4BB086F,5B11,4C67,98,6C,8C,D4,8C,5C,E3,8B)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_241;
#undef  ADO_Reserved_241
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_242
#define ADO_Reserved_242    GUID_BUILDER(ADO_Reserved_242,F1C4A502,4744,478F,87,71,C6,94,CC,2D,F7,B6)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_242;
#undef  ADO_Reserved_242
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_243
#define ADO_Reserved_243    GUID_BUILDER(ADO_Reserved_243,2CD39761,F389,4F5E,BE,91,A6,B9,1B,18,AD,12)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_243;
#undef  ADO_Reserved_243
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_244
#define ADO_Reserved_244    GUID_BUILDER(ADO_Reserved_244,8895BA8F,0CB7,4354,A8,EA,CD,9D,F4,1B,F8,88)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_244;
#undef  ADO_Reserved_244
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_245
#define ADO_Reserved_245    GUID_BUILDER(ADO_Reserved_245,71E0B0DC,1245,441D,92,92,32,71,3F,57,97,7A)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_245;
#undef  ADO_Reserved_245
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_246
#define ADO_Reserved_246    GUID_BUILDER(ADO_Reserved_246,7604D0CB,F137,472D,8B,4C,85,66,72,9A,CF,03)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_246;
#undef  ADO_Reserved_246
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_247
#define ADO_Reserved_247    GUID_BUILDER(ADO_Reserved_247,94C9B5AC,8309,4F4B,8E,68,C4,37,7E,C2,B7,91)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_247;
#undef  ADO_Reserved_247
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_248
#define ADO_Reserved_248    GUID_BUILDER(ADO_Reserved_248,0E555180,5E2C,4BF6,90,A0,1B,36,3D,4B,B9,99)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_248;
#undef  ADO_Reserved_248
#endif // IMMEDIATE_GUID_USE

// ADO_Reserved_249
#define ADO_Reserved_249    GUID_BUILDER(ADO_Reserved_249,C077D666,6988,4EAC,A5,52,61,61,55,F9,6A,12)
#ifdef IMMEDIATE_GUID_USE
ADO_Reserved_249;
#undef  ADO_Reserved_249
#endif // IMMEDIATE_GUID_USE

#endif // RESERVED_GUIDS_BEYOND_THIS_POINT

#endif // !defined(NTDDI_VERSION) || (NTDDI_VERSION < NTDDI_WIN8)