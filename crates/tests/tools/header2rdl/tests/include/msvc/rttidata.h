//
// Declares internal C++ RTTI data structures.
// This file is preprocessed and then the preprocessed results are force-included into every C++ TU.
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//

#ifndef _INC_RTTIDATA
#define _INC_RTTIDATA
#pragma once

#include "ehdata_forceinclude.h"

#pragma pack(push, rttidata, 4)

#ifdef BUILDING_C1XX_FORCEINCLUDE
#define PMD 					_PMD
#define TypeDescriptor			_TypeDescriptor

#define _RTTIBaseClassDescriptor		__RTTIBaseClassDescriptor
#define _s_RTTIBaseClassDescriptor		_s__RTTIBaseClassDescriptor
#define _RTTIBaseClassArray 			__RTTIBaseClassArray
#define _s_RTTIBaseClassArray			_s__RTTIBaseClassArray
#define _RTTIClassHierarchyDescriptor	__RTTIClassHierarchyDescriptor
#define _s_RTTIClassHierarchyDescriptor _s__RTTIClassHierarchyDescriptor
#define _RTTICompleteObjectLocator		__RTTICompleteObjectLocator
#if VERSP_WIN64
 #define _s_RTTICompleteObjectLocator	_s__RTTICompleteObjectLocator2
#else
 #define _s_RTTICompleteObjectLocator	_s__RTTICompleteObjectLocator
#endif // VERSP_WIN64
#endif // BUILDING_C1XX_FORCEINCLUDE

struct _s_RTTIClassHierarchyDescriptor;
typedef const struct _s_RTTIClassHierarchyDescriptor _RTTIClassHierarchyDescriptor;

typedef const struct	_s_RTTIBaseClassDescriptor	{
#if _RTTI_RELATIVE_TYPEINFO
	int								pTypeDescriptor;	// Image relative offset of TypeDescriptor
#else
	TypeDescriptor*					pTypeDescriptor;
#endif
	unsigned long					numContainedBases;
	PMD								where;
	unsigned long					attributes;
#if _RTTI_RELATIVE_TYPEINFO
	int								pClassDescriptor;	// Image relative offset of _RTTIClassHierarchyDescriptor
#else
	_RTTIClassHierarchyDescriptor*	pClassDescriptor;
#endif
	} _RTTIBaseClassDescriptor;

#define BCD_NOTVISIBLE				0x00000001
#define BCD_AMBIGUOUS				0x00000002
#define BCD_PRIVORPROTBASE			0x00000004
#define BCD_PRIVORPROTINCOMPOBJ		0x00000008
#define BCD_VBOFCONTOBJ				0x00000010
#define BCD_NONPOLYMORPHIC			0x00000020
#define BCD_HASPCHD					0x00000040			// pClassDescriptor field is present

#define BCD_PTD(bcd)				((bcd).pTypeDescriptor)
#define BCD_PCHD(bcd)				((bcd).pClassDescriptor)
#if _RTTI_RELATIVE_TYPEINFO
#define BCD_PTD_IB(bcd,ib)			((TypeDescriptor*)((ib) + (bcd).pTypeDescriptor))
#define BCD_PCHD_IB(bcd,ib)			((_RTTIClassHierarchyDescriptor*)((ib) + (bcd).pClassDescriptor))
#endif // _RTTI_RELATIVE_TYPEINFO


//
//	_RTTIBaseClassArray
//
#pragma warning (push)
#pragma warning (disable:4200)	// nonstandard extension used: array of runtime bound
typedef const struct	_s_RTTIBaseClassArray	{
#if _RTTI_RELATIVE_TYPEINFO
	int							arrayOfBaseClassDescriptors[];  // Image relative offset of _RTTIBaseClassDescriptor
#else
	_RTTIBaseClassDescriptor*	arrayOfBaseClassDescriptors[];
#endif
	} _RTTIBaseClassArray;
#pragma warning (pop)

//
//	_RTTIClassHierarchyDescriptor
//
typedef const struct	_s_RTTIClassHierarchyDescriptor	{
	unsigned long			signature;
	unsigned long			attributes;
	unsigned long			numBaseClasses;
#if _RTTI_RELATIVE_TYPEINFO
	int						pBaseClassArray;    // Image relative offset of _RTTIBaseClassArray
#else
	_RTTIBaseClassArray*	pBaseClassArray;
#endif
	} _RTTIClassHierarchyDescriptor;

#define CHD_MULTINH					0x00000001
#define CHD_VIRTINH					0x00000002
#define CHD_AMBIGUOUS				0x00000004

#define CHD_PBCA(chd)				((chd).pBaseClassArray)
#define CHD_PBCD(bcd)				(bcd)
#if _RTTI_RELATIVE_TYPEINFO
#define CHD_PBCA_IB(chd,ib)			((_RTTIBaseClassArray*)((ib) + (chd).pBaseClassArray))
#define CHD_PBCD_IB(bcd,ib)			((_RTTIBaseClassDescriptor*)((ib) + bcd))
#endif

//
//	_RTTICompleteObjectLocator
//
typedef const struct	_s_RTTICompleteObjectLocator	{
	unsigned long							signature;
	unsigned long							offset;
	unsigned long							cdOffset;
#if _RTTI_RELATIVE_TYPEINFO
	int										pTypeDescriptor;	// Image relative offset of TypeDescriptor
	int										pClassDescriptor;	// Image relative offset of _RTTIClassHierarchyDescriptor
	int										pSelf;				// Image relative offset of this object
#else
	TypeDescriptor*							pTypeDescriptor;
	_RTTIClassHierarchyDescriptor*			pClassDescriptor;
 #if VERSP_WIN64	// TRANSITION, VSO#515783
	const _s_RTTICompleteObjectLocator* 	pSelf;
 #endif // VERSP_WIN64
#endif
	} _RTTICompleteObjectLocator;

#define COL_PTD(col)				((col).pTypeDescriptor)
#define COL_PCHD(col)				((col).pClassDescriptor)
#if _RTTI_RELATIVE_TYPEINFO
#define COL_PTD_IB(col,ib)			((TypeDescriptor*)((ib) + (col).pTypeDescriptor))
#define COL_PCHD_IB(col,ib)			((_RTTIClassHierarchyDescriptor*)((ib) + (col).pClassDescriptor))
#endif // _RTTI_RELATIVE_TYPEINFO

#define COL_SIG_REV0                0
#define COL_SIG_REV1                1

extern "C++" { // attach declarations to the global module, see N4910 [module.unit]/7
	class type_info;
} // extern "C++"

//
// Type of the result of __RTtypeid and internal applications of typeid().
//

typedef const type_info &__RTtypeidReturnType;

//
// Declaration of CRT entrypoints, as seen by the compiler.  Types are
// simplified so as to avoid type matching hassles.
//

// Perform a dynamic_cast on obj. of polymorphic type
extern "C" void*
#ifdef prepifdef
	prepifdef _M_CEE_PURE
	__clrcall
	prepelse
	__cdecl
	prependif	// _M_CEE_PURE
#else
	__CLRCALL_OR_CDECL
#endif
	__RTDynamicCast(
		void*,				// ptr to vfptr
		long,				// offset of vftable
		void*,				// src type
		void*,				// target type
		int) noexcept(false); // isReference

// Perform 'typeid' on obj. of polymorphic type
extern "C" void*
#ifdef prepifdef
	prepifdef _M_CEE_PURE
	__clrcall
	prepelse
	__cdecl
	prependif	// _M_CEE_PURE
#else
        __CLRCALL_OR_CDECL
#endif
        __RTtypeid (void*) noexcept(false);	// ptr to vfptr

// Perform a dynamic_cast from obj. of polymorphic type to void*
extern "C" void*
#ifdef prepifdef
	prepifdef _M_CEE_PURE
	__clrcall
	prepelse
	__cdecl
	prependif	// _M_CEE_PURE
#else
	__CLRCALL_OR_CDECL
#endif
	__RTCastToVoid (void*) noexcept(false); // ptr to vfptr

#pragma pack(pop, rttidata)

#ifdef BUILDING_C1XX_FORCEINCLUDE
#undef PMD
#undef TypeDescriptor

#undef _RTTIBaseClassDescriptor
#undef _s_RTTIBaseClassDescriptor
#undef _RTTIBaseClassArray
#undef _s_RTTIBaseClassArray
#undef _RTTIClassHierarchyDescriptor
#undef _s_RTTIClassHierarchyDescriptor
#undef _RTTICompleteObjectLocator
#undef _s_RTTICompleteObjectLocator
#endif

#endif /* _INC_RTTIDATA */
