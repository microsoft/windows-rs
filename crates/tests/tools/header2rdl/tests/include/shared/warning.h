#ifndef _WARNING_H_
#define _WARNING_H_


#pragma warning(3:4092)   // sizeof returns 'unsigned long'
#pragma warning(4:4096)   // '__cdecl' must be used with '...'
#pragma warning(4:4121)   // structure is sensitive to alignment
#pragma warning(3:4125)   // decimal digit in octal sequence
#pragma warning(3:4130)   // logical operation on address of string constant
#pragma warning(3:4132)   // const object should be initialized
#if defined(_DBG_MEMCPY_INLINE_)
#pragma warning(disable:4163) // not available as an intrinsic function
#endif
#pragma warning(disable:4206) // Source File is empty
#pragma warning(4:4101)   // Unreferenced local variable
#pragma warning(4:4208)   // delete[exp] - exp evaluated but ignored
#pragma warning(3:4212)   // function declaration used ellipsis
#pragma warning(3:4242)   // convertion possible loss of data
#if defined(_M_IA64)
#pragma warning(disable:4714) // function marked as __forceinline not inlined
                              // (VC7.1:  inlining is not allowed in finally)
#endif
#pragma warning(4:4267)   // convertion from size_t to smaller type
#pragma warning(disable:4312)   // conversion to type of greater size
#pragma warning(4:4313)        // Mismatched types / format string specifiers
#pragma warning(disable:4324)  // structure padded due to __declspec(align())
#pragma warning(error:4700)    // Local used w/o being initialized
#pragma warning(error:4703)    // Local pointer potentially used used w/o being initialized
#pragma warning(error:4754)    // Bad integer overflow checks warning (ineffective check)
#pragma warning(error:4013)    // function' undefined - assuming extern returning int
#pragma warning(error:4551)    // Function call missing argument list
#pragma warning(error:4806)    // unsafe operation involving type 'bool'
#pragma warning(disable:4826)  // pointer sign extension
#pragma warning(4:4509)   // use of SEH with destructor
#pragma warning(4:4177)   // pragma data_seg s/b at global scope
#pragma warning(disable:4274)  // #ident ignored
#pragma warning(disable:4786)  // identifier was truncated to 255 chararcers in debug information.
#pragma warning(disable:4503)  // decorated name length exceeded, name was truncated.
#pragma warning(disable:4263)  // Derived override doesn't match base - who cares...
#pragma warning(disable:4264)  // base function is hidden - again who cares.
#pragma warning(disable:4710)  // Function marked as inline - wasn't
#pragma warning(disable:4917)  // A GUID can only be associated with a class, interface or namespace
#pragma warning(error:4552)    // <<, >> ops used to no effect (probably missing an = sign)
#pragma warning(error:4553)    // == op used w/o effect (probably s/b an = sign)
#pragma warning(3:4288)   // nonstandard extension used (loop counter)
#pragma warning(3:4532)   // jump out of __finally block
#pragma warning(error:4296)  // expression is always true/false
#pragma warning(3:4546)   // function call before comma missing argument list
// disable until __noop(arg,arg,arg) doesn't generate false hits.
// #pragma warning(3:4547)   // '<' : operator before comma has no effect; expected operator with side-effect
// #pragma warning(3:4548)   // expression before comma has no effect; expected expression with side-effect

#pragma warning(disable:4306)   // conversion from smaller to greater size

#if _MSC_VER > 1300
#pragma warning(disable:4356)	// static member cannot be initialized via derived class
#endif

// Ignoring these warnings can cause blatantly incorrect code generation.
// Since not all projects are -W4 -WX, treat as errors explicitly
#pragma warning(error:4028)   // C4028: formal parameter different from declaration
#pragma warning(error:4029)   // C4029: declared formal parameter list different from definition

#if 0
#pragma warning(3:4100)   // Unreferenced formal parameter
#pragma warning(3:4701)   // local may be used w/o init
// VSTS: 14791840: Disabling C4702 due to unmanageable number of instances 
// that are triggered when certain inlining occurs, which can vary based on the build flavors.
// #pragma warning(3:4702)   // Unreachable code
#pragma warning(3:4706)   // assignment w/i conditional expression
#pragma warning(3:4709)   // command operator w/o index expression
#endif

#if !defined(__cplusplus) && !defined(DONT_REDEFINE_TRYEXCEPT_IN_WARNING_H)
// DONT_REDEFINE_TRYEXCEPT_IN_WARNING_H allows C files to opt-out of getting these symbols redefined so that they can be used as regular C identifiers for variables
#undef try
#undef except
#undef finally
#undef leave
#define try                         __try
#define except                      __except
#define finally                     __finally
#define leave                       __leave
#endif

#pragma warning(disable: 4068)	// turn off unknown pragma warning so prefast pragmas won't show
				// show up in build.wrn/build.err

// Macros for suppressing PREfast warnings
#include <suppress.h>


#if _MSC_FULL_VER >= 140040702
#pragma warning(disable:4430)	// default-int warning (C++)
#pragma warning(disable:4431)	// default-int warning (C)
#pragma warning(disable:4812)	// obsolete template destructor syntax
#endif


#if defined(_M_IA64) && _MSC_VER > 1310
#define __TYPENAME typename
#elif defined(_M_IX86) && _MSC_FULL_VER >= 13102154
#define __TYPENAME typename
#elif defined(_M_AMD64) && _MSC_VER >= 1400
#define __TYPENAME typename
#elif defined(_M_ARM) || defined(_M_ARM64)
#define __TYPENAME typename
#else
#define __TYPENAME
#endif

#if _MSC_VER > 1400
#if !defined(DONT_DISABLE_PCH_WARNINGS_IN_WARNING_H)
#pragma warning(disable:4603)   // macro is not defined or definition is different after precompiled header use
#pragma warning(disable:4627)   // skipped when looking for precompiled header use
#endif
#endif

#ifdef _PREFAST_
#pragma warning(disable:4091)   // (If prefast only): annotations followed by ; are ignored (actual text is not that clear).
#endif

#endif // _WARNING_H_

