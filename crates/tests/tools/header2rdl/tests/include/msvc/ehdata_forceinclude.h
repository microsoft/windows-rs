//
// Declares internal C++ exception handling data structures.
// This file is preprocessed and then the preprocessed results are force-included into every C++ TU.
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//

#ifndef _INC_EHDATA_FORCEINCLUDE
#define _INC_EHDATA_FORCEINCLUDE
#pragma once

#ifdef BUILDING_C1XX_FORCEINCLUDE
#define PMD 					_PMD
#define PMFN					_PMFN
#define TypeDescriptor			_TypeDescriptor
#define CatchableType			_CatchableType
#define _s_CatchableType		_s__CatchableType
#define CatchableTypeArray		_CatchableTypeArray
#define _s_CatchableTypeArray	_s__CatchableTypeArray
#define ThrowInfo				_ThrowInfo
#define _s_ThrowInfo			_s__ThrowInfo
#endif

#if defined(_M_CEE_PURE) || defined(BUILDING_C1XX_FORCEINCLUDE)
#define _EH_RELATIVE_TYPEINFO 0
#define _EH_RELATIVE_FUNCINFO 0
#define _RTTI_RELATIVE_TYPEINFO 0
#elif defined(_CHPE_X86_ARM64_EH_)
#define _EH_RELATIVE_TYPEINFO 0
#define _EH_RELATIVE_FUNCINFO 1
#define _RTTI_RELATIVE_TYPEINFO 0
#elif defined(_M_ARM)
#define _EH_RELATIVE_TYPEINFO 1
#define _EH_RELATIVE_FUNCINFO 1
#define _RTTI_RELATIVE_TYPEINFO 0
#elif defined(_M_X64) || defined(_M_ARM64)
#define _EH_RELATIVE_TYPEINFO 1
#define _EH_RELATIVE_FUNCINFO 1
#define _RTTI_RELATIVE_TYPEINFO 1
#else
#define _EH_RELATIVE_TYPEINFO 0
#define _EH_RELATIVE_FUNCINFO 0
#define _RTTI_RELATIVE_TYPEINFO 0
#endif

#pragma pack(push, ehdata, 4)

//
// PMD - Pointer to Member Data: generalized pointer-to-member descriptor
//
typedef struct PMD
{
	int	mdisp;	// Offset of intended data within base
	int	pdisp;	// Displacement to virtual base pointer
	int	vdisp;	// Index within vbTable to offset of base
	} PMD;

//
// PMFN - Pointer to Member Function
//
#if _EH_RELATIVE_TYPEINFO
typedef	int	PMFN;					// Image relative offset of Member Function
#else
typedef void (__cdecl * PMFN)(void*);
#endif

typedef void* (__stdcall * PGETWINRT_OOM_EXCEPTION)();

//
// TypeDescriptor - per-type record which uniquely identifies the type.
//
// Each type has a decorated name which uniquely identifies it, and a hash
// value which is computed by the compiler.  The hash function used is not
// important; the only thing which is essential is that it be the same for
// all time.
//
// The special type '...' (ellipsis) is represented by a null name.
//
#pragma warning(push)
#pragma warning(disable:4200)	// nonstandard extension used: array of runtime bound

#if defined(_M_X64) || defined(_M_ARM64) || defined(BUILDING_C1XX_FORCEINCLUDE)
#pragma pack(push, TypeDescriptor, 8)
#endif

typedef struct TypeDescriptor
{
#if defined(_WIN64) || defined(_RTTI) || defined(BUILDING_C1XX_FORCEINCLUDE)
	const void * pVFTable;	// Field overloaded by RTTI
#else
	unsigned long	hash;			// Hash value computed from type's decorated name
#endif
	void *	spare;			// reserved, possible for RTTI
	char			name[];			// The decorated name of the type; 0 terminated.
	} TypeDescriptor;

#if defined(_M_X64) || defined(_M_ARM64) || defined(BUILDING_C1XX_FORCEINCLUDE)
#pragma pack(pop, TypeDescriptor)
#endif
#pragma warning(pop)


/////////////////////////////////////////////////////////////////////////////
//
// Description of the thrown object.
//
// This information is broken down into three levels, to allow for maximum
// comdat folding (at the cost of some extra pointers).
//
// ThrowInfo is the head of the description, and contains information about
// 				the particular variant thrown.
// CatchableTypeArray is an array of pointers to type descriptors.  It will
//				be shared between objects thrown by reference but with varying
//				qualifiers.
// CatchableType is the description of an individual type, and how to effect
//				the conversion from a given type.
//
//---------------------------------------------------------------------------


//
// CatchableType - description of a type that can be caught.
//
// Note:  although isSimpleType can be part of ThrowInfo, it is more
//		  convenient for the run-time to have it here.
//
typedef const struct _s_CatchableType {
	unsigned int		properties;			// Catchable Type properties (Bit field)
#if _EH_RELATIVE_TYPEINFO
	int					pType;				// Image relative offset of TypeDescriptor
#else
	TypeDescriptor *	pType;				// Pointer to the type descriptor for this type
#endif
	PMD 				thisDisplacement;	// Pointer to instance of catch type within thrown object.
	int					sizeOrOffset;		// Size of simple-type object or offset into
											//  buffer of 'this' pointer for catch object
	PMFN				copyFunction;		// Copy constructor or CC-closure
} CatchableType;

//
// CatchableTypeArray - array of pointers to catchable types, with length
//
#pragma warning (push)
#pragma warning (disable:4200)	// nonstandard extension used: array of runtime bound
typedef const struct _s_CatchableTypeArray {
	int	nCatchableTypes;
#if _EH_RELATIVE_TYPEINFO
	int				arrayOfCatchableTypes[];	// Image relative offset of Catchable Types
#else
	CatchableType*	arrayOfCatchableTypes[];
#endif
	} CatchableTypeArray;
#pragma warning (pop)

//
// ThrowInfo - information describing the thrown object, statically built
// at the throw site.
//
// pExceptionObject (the dynamic part of the throw; see below) is always a
// reference, whether or not it is logically one.  If 'isSimpleType' is true,
// it is a reference to the simple type, which is 'size' bytes long.  If
// 'isReference' and 'isSimpleType' are both false, then it's a UDT or
// a pointer to any type (i.e. pExceptionObject points to a pointer).  If it's
// a pointer, copyFunction is NULL, otherwise it is a pointer to a copy
// constructor or copy constructor closure.
//
// The pForwardCompat function pointer is intended to be filled in by future
// versions, so that if say a DLL built with a newer version (say C10) throws,
// and a C9 frame attempts a catch, the frame handler attempting the catch (C9)
// can let the version that knows all the latest stuff do the work.
//
typedef const struct _s_ThrowInfo {
	unsigned int	attributes;							// Throw Info attributes (Bit field)
	PMFN			pmfnUnwind;							// Destructor to call when exception has been handled or aborted
#if _EH_RELATIVE_TYPEINFO && !defined(BUILDING_C1XX_FORCEINCLUDE)
	int				pForwardCompat;						// Image relative offset of Forward compatibility frame handler
	int				pCatchableTypeArray;				// Image relative offset of CatchableTypeArray
#else
	int	(__cdecl * pForwardCompat)(...);				// Forward compatibility frame handler
	CatchableTypeArray* pCatchableTypeArray;			// Pointer to list of pointers to types
#endif
} ThrowInfo;

//
// Here's how to throw:
// _ThrowInfo is the name of the type that is 'pre-injected' into the
// compiler (see macro above); since this prototype is known to the FE along with the pre-injected
// types, it has to match exactly.
//
extern "C" __declspec(noreturn) void __stdcall _CxxThrowException(
	void* pExceptionObject, _ThrowInfo* pThrowInfo) noexcept(false);

extern "C" int __cdecl __CxxExceptionFilter(void* ppExcept, void* pType, int adjectives, void *pBuildObj);

#ifdef prepifdef
	prepifdef _MANAGED
	int __clrcall ___CxxExceptionFilter(void* ppExcept, void* pType, int adjectives, void *pBuildObj);
	prependif	// _MANAGED
#else
	#ifdef _MANAGED
	int __clrcall ___CxxExceptionFilter(void* ppExcept, void* pType, int adjectives, void *pBuildObj);
	#endif		// _MANAGED
#endif

// Returns true if the object is really a C++ exception
// If it is, stores the previous exception in *storage, and saves the current one
// This is needed to keep track of the current exception object (used for rethrow & destruction)
extern "C" int __cdecl __CxxRegisterExceptionObject(void *exception, void *storage);

#ifdef prepifdef
	prepifdef _MANAGED
	int __clrcall ___CxxRegisterExceptionObject(void *exception, void *storage);
	prependif	// _MANAGED
#else
	#ifdef _MANAGED
	int __clrcall ___CxxRegisterExceptionObject(void *exception, void *storage);
	#endif		// _MANAGED
#endif

// Returns true if exception is a C++ rethrown exception
// This is needed, so Unregister can know whether or not to destroy the object
extern "C" int __cdecl __CxxDetectRethrow(void *exception);

#ifdef prepifdef
	prepifdef _MANAGED
	int __clrcall ___CxxDetectRethrow(void *exception);
	prependif	// _MANAGED
#else
	#ifdef _MANAGED
	int __clrcall ___CxxDetectRethrow(void *exception);
	#endif		// _MANAGED
#endif

// Returns the byte count of stack space required to store the exception info
extern "C" int __cdecl __CxxQueryExceptionSize(void);

#ifdef prepifdef
	prepifdef _MANAGED
	int __clrcall ___CxxQueryExceptionSize(void);
	prependif	// _MANAGED
#else
	#ifdef _MANAGED
	int __clrcall ___CxxQueryExceptionSize(void);
	#endif		// _MANAGED
#endif

// Pops the current exception, restoring the previous one from *storage
// This detects whether or not the exception object needs to be destroyed
extern "C" void __cdecl __CxxUnregisterExceptionObject(void *storage, int rethrow);

#ifdef prepifdef
	prepifdef _MANAGED
	void __clrcall ___CxxUnregisterExceptionObject(void *storage, int rethrow);
	prependif	// _MANAGED
#else
	#ifdef _MANAGED
	void __clrcall ___CxxUnregisterExceptionObject(void *storage, int rethrow);
	#endif		// _MANAGED
#endif

#pragma pack(pop, ehdata)

#ifdef BUILDING_C1XX_FORCEINCLUDE
#undef PMD
#undef PMFN
#undef TypeDescriptor
#undef CatchableType
#undef _s_CatchableType
#undef CatchableTypeArray
#undef _s_CatchableTypeArray
#undef ThrowInfo
#undef _s_ThrowInfo
#endif

#endif /* _INC_EHDATA_FORCEINCLUDE */
