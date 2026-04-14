//
//	gcroot.h - Template class that wraps GCHandle from mscorlib.dll.
//		Copyright (C) Microsoft Corporation
//		All rights reserved.
//
//	This include file is provided for back-compatibility.
//	Include <msclr/gcroot.h> and use ::msclr::gcroot instead of ::gcroot.
//
//	Use this class to declare gc "pointers" that live in the C++ heap.
//
//	Example:
//		struct StringList {
//			gcroot<String^> str;
//			StringList *next;
//			StringList(); // should have ctors and dtors
//			~StringList();
//		};
//
//	By convention, we maintain a 1-to-1 relationship between C++ objects
//	and the handle slots they "point" to.  Thus, two distinct C++ objects
//	always refer to two distinct handles, even if they "point" to the same
//	object.  Therefore, when the C++ object is destroyed, its handle can
//	be freed without error.
//
//	Note that we cannot currently embed a GCHandle directly in an unmanaged C++
//	class.  We therefore store a void*, and use the conversion methods of
//	GCHandle to reconstitute a GCHandle from the void* on demand.
//
//	See msclr/gcroot.h for implementation details.

#if _MSC_VER > 1000
#pragma once
#endif

#if !defined(_INC_GCROOT)
#define _INC_GCROOT
#ifndef RC_INVOKED

#include <stddef.h>
#if !defined (_INC_MSCLR_GCROOT) // This means that msclr/gcroot.h is already included
#define __DEFINE_GCROOT_IN_GLOBAL_NAMESPACE // Scheduled for removal. The intention is to retain ::msclr::gcroot
#include <msclr/gcroot.h>
namespace msclr
{
    using ::gcroot;
}
#endif /* !defined (_INC_MSCLR_GCROOT) */

#endif /* RC_INVOKED */
#endif  // _INC_GCROOT
