/*****************************************************************************\
*                                                                             *
* DriverSpecs.h - markers for documenting the semantics of driver APIs        *
*                 See also <SpecStrings.h>                                    *
*                                                                             *
* Version 1.2.10                                                              *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/

/*****************************************************************************\
* NOTE                                                                        *
* NOTE                                                                        *
* NOTE                                                                        *
*   The macro bodies in this file are subject to change without notice.       *
*   Attempting to use the annotations in the macro bodies directly is not     *
*   supported.                                                                *
* NOTE                                                                        *
* NOTE                                                                        *
* NOTE                                                                        *
\*****************************************************************************/

/*****************************************************************************\
* The annotations described by KernelSpecs.h and DriverSpecs.h, taken together,
* are used to annotate drivers.  Many of the annotations are applicable to
* user space code (including subsystems) as well as to drivers.
*
* DriverSpecs.h contains those annotations which are appropriate to userspace
* code, or which might appear in headers that are shared between user space
* and kernel space.  In the case of annotations which might appear in such a
* shared header, but which are meaningless in user space, the annotations are
* #defined to nothing in DriverSpecs.h.
*
* KernelSpecs.h contains those annotations which either will only appear in
* kernel code or headers; or which might appear in shared headers.  In the
* latter case, it is assumed that DriverSpecs.h has been #included, and
* the anntoations are re-defined (using #undef) to give them a meaningful
* value.  In general, documentation for the shared-header annotations appears
* in DriverSpecs.h.
*
* Many annotations are context dependent.  They only apply to certain versions
* of Windows, or only to certain classes of driver.  These rules can be written
* using something like _When_(NTDDI_VERSION >= NTDDI_WINXP, ...)
* which causes the rule only to apply to Windows XP and later.  Many of these
* symbols are already defined in various Windows headers.
*
* To facilitate using this sort of conditional rule, we collect here the
* various known symbols that are (or reasonably might) be used in such
* a conditional annotation.  Some are speculative in that the symbol has
* not yet been defined because there are no known uses of it yet.
*
* Where the symbol already exists its relevant header is
* noted below (excluding the "really well known" ones).
*
* Each symbol is listed with the currently known possible values.
*
* Some symbols are marked as #define symbols -- they are used with #ifdef
* operators only.  To use them in _When_, use something like
* _When_(__drv_defined(NT), ...).
*
* WDK Version (copied for convenience from sdkddkver.h)
*     NTDDI_VERSION: NTDDI_WIN2K NTDDI_WIN2KSP1 NTDDI_WIN2KSP2 NTDDI_WIN2KSP3
*                    NTDDI_WIN2KSP4 NTDDI_WINXP NTDDI_WINXPSP1 NTDDI_WINXPSP2
*                    NTDDI_WS03 NTDDI_WS03SP1 NTDDI_VISTA
*     The WDK version is taken as the WDM version as well.
*
* OS Version: (copied for convenience from sdkddkver.h)
*     _WIN32_WINNT: _WIN32_WINNT_NT4 _WIN32_WINNT_WIN2K _WIN32_WINNT_WINXP
*                   _WIN32_WINNT_WS03 _WIN32_WINNT_LONGHORN
*     WINVER: 0x030B 0x0400 0x0500 0x0600
*     NT (#define symbol)
* (sdkddkver.h also defines symbols for IE versions should they be needed.)
*
* Compiler Version:
*     _MSC_VER: too many to list.
*     _MSC_FULL_VER: too many to list.
*
* KMDF Version:  (Currently defined/used only in makefiles.)
*     KMDF_VERSION_MAJOR: 1
*
* UMDF Version:  (Currently defined/used only in makefiles.)
*     UMDF_VERSION_MAJOR: 1
*
* Architecture kinds:
*     __WIN64 (#define symbols)
*     _X86_
*     _AMD64_
*     _IA64_
*
* Machine Architectures:
*     _M_IX86
*     _M_AMD64
*     _M_IA64
*
* Driver Kind (NYI: "not yet implemented")
*   Typically these will be defined in the most-common header for a
*   particular driver (or in individual source files if appropriate).
*   These are not intended to necessarily be orthogonal: more than one might
*   apply to a particular driver.
*     _DRIVER_TYPE_BUS: 1                // NYI
*     _DRIVER_TYPE_FUNCTIONAL: 1         // NYI
*     _DRIVER_TYPE_MINIPORT: 1           // NYI
*     _DRIVER_TYPE_STORAGE: 1            // NYI
*     _DRIVER_TYPE_DISPLAY: 1            // NYI
*     _DRIVER_TYPE_FILESYSTEM: 1
*     _DRIVER_TYPE_FILESYSTEM_FILTER: 1
*
* NDIS driver version: (see ndis.h for much more detail.)
*   These can be used to both identify an NDIS driver and to check the version.
*     NDIS40 NDIS50 NDIS51 NDIS60 (#defined symbols)
*     NDIS_PROTOCOL_MAJOR_VERSION.NDIS_PROTOCOL_MINOR_VERSION: 4.0 5.0 5.1 6.0
*     And many others in ndis.h (including MINIPORT)
*
\*****************************************************************************/

#ifndef DRIVERSPECS_H
#define DRIVERSPECS_H

// In case driverspecs.h is included directly (and w/o specstrings.h)
#ifndef SPECSTRINGS_H
#include <specstrings.h>
#endif

#include "sdv_driverspecs.h"
#include <concurrencysal.h>

#if _MSC_VER > 1000 // [
#pragma once
#endif // ]

#ifdef __cplusplus // [
extern "C" {
#endif // ]

#if (_MSC_VER >= 1000) && !defined(__midl) && defined(_PREFAST_) && defined(_MSC_EXTENSIONS)// [

    // ---------------------------------------------------------------------
    // Processing mode selection:
    //
    // __internal_kernel_driver
    //
    // Flag for headers that indicates a probable driver.
    // This should only be coded in headers that are normally used
    // as the "primary" header for a class of drivers.  It sets the
    // default to kernel mode driver.
    //
    // ';' inside the parens to keep MIDL happy
    __ANNOTATION(SAL_internal_kernel_driver(void);)

    #define __internal_kernel_driver                                          \
		_Analysis_mode_(_Analysis_internal_kernel_driver_)					  \
		_Analysis_mode_(_Analysis_local_leak_checks_)

    //
    // __kernel_code
    // __kernel_driver
    // __user_driver
    // __user_code
    //
    // Flags for compilation units that indicated specifically what kind of
    // code it is.
    // These should be coded as early as possible in any compilation unit
    // (.c/.cpp file) that doesn't get the correct default.  Whether before
    // or after __internal_kernel_driver
    //
    // Indicate that the code is kernel, but not driver, code.

    __ANNOTATION(SAL_kernel(void);)
    __ANNOTATION(SAL_nokernel(void);)
    __ANNOTATION(SAL_driver(void);)
    __ANNOTATION(SAL_nodriver(void);)

    #define __kernel_code                                                     \
		_Analysis_mode_(_Analysis_code_type_kernel_code_)					  \
		_Analysis_mode_(_Analysis_local_leak_checks_)

    // Indicate that the code is kernel, driver, code.
    #define __kernel_driver                                                   \
		_Analysis_mode_(_Analysis_code_type_kernel_driver_)					  \
		_Analysis_mode_(_Analysis_local_leak_checks_)

    // Indicate that the code is a user mode driver.
    #define __user_driver                                                     \
		_Analysis_mode_(_Analysis_code_type_user_driver_)					  \
		_Analysis_mode_(_Analysis_local_leak_checks_)

    // Indicate that the code is ordinary user mode code.
    #define __user_code                                                       \
		_Analysis_mode_(_Analysis_code_type_user_code_)						  \
		_Analysis_mode_(_Analysis_local_leak_checks_)

    // "landmark" function definition to pass information to the
    // analysis tools, as needed.

    __ANNOTATION(SAL_landmark(__In_impl_ char *);)

    #define _Landmark_(name) _SAL_L_Source_(_Landmark_, (name),               \
	_SA_annotes1(SAL_landmark, #name))

    #define __drv_Mode_impl(x)                                                \
      _Landmark_(x)                                                           \
      __inline void __GENSYM(__SAL_dummy_)(void){}

    // Macros to declare a function to be a particular class
    // of driver.

    #define __drv_WDM  __drv_Mode_impl(WDM)
    #define __drv_KMDF __drv_Mode_impl(KMDF)
    #define __drv_NDIS __drv_Mode_impl(NDIS)

    // Inform PREfast that operator new does [not] throw.
    // Be sure you really know which is actually in use before using one of
    // these.  The default is throwing (and cannot return NULL) which is
    // standard conformant, but much kernel code links with a non-throwing
    // operator new.
    //
    // Header <new> will set the default to throwing, so be sure to place
    // this after that header is included.
    //
    // Be sure to use these macros for this purpose as the implementation
    // could change.

    #define __prefast_operator_new_throws                                 \
        void* __cdecl operator new(size_t size) throw(std::bad_alloc);    \
        void* __cdecl operator new[](size_t size) throw(std::bad_alloc);  \
		_Analysis_mode_(_Analysis_operator_new_throw_)

    #define __prefast_operator_new_null                                   \
        void* __cdecl operator new(size_t size) throw();                  \
        void* __cdecl operator new[](size_t size) throw();				  \
		_Analysis_mode_(_Analysis_operator_new_null_)


    // Paging information:
    //    _Analysis_assume_section_locked_(n)
    //    _Analysis_assume_section_unlocked_(n)
    //       A call will be made to <n>, and we should assume it is [not]
    //       locked in memory.

    // Past this point, the named section is assumed to be locked in
    // memory, and calls to that section are deemed safe.
    // This is only used when dynamically locked pages are being used.
    __inline __nothrow
    void __AnalysisAssumeLockedSection(__In_impl_ char *p);
    __inline __nothrow
    void __AnalysisAssumeUnlockedSection(__In_impl_ char *p);

    #define _Analysis_assume_section_locked_(name)                        \
		__AnalysisAssumeLockedSection(name)
    #define _Analysis_assume_section_unlocked_(name)                      \
		__AnalysisAssumeUnlockedSection(name)

#else // ][

    #define __internal_kernel_driver
    #define __kernel_code
    #define __kernel_driver
    #define __user_driver
    #define __user_code
    #define __drv_Mode_impl(x)
    #define __drv_WDM
    #define __drv_KMDF
    #define __drv_NDIS
    #define __prefast_operator_new_throws
    #define __prefast_operator_new_null

    #define _Analysis_assume_section_locked_(name)
    #define _Analysis_assume_section_unlocked_(name)

#endif // ]

    // Callback with high IRQL level will never actually be called above 
    // 'level'
    #define _IRQL_limited_to_(level)                                      \
        ASSERT(KeGetCurrentIrql() <= level);                              \
        _Analysis_assume_(KeGetCurrentIrql() <= level);

    // core macros: these provide syntatic wrappers to make other uses
    // simpler.
    //
    // For example:
    //   __drv_in(__drv_nonconstant __setsIRQL)

    #define __drv_deref(annotes) __deref _Group_(annotes _SAL_nop_impl_)
    #define __drv_in(annotes) _Pre_ _Group_(annotes _SAL_nop_impl_)
    #define __drv_in_deref(annotes) _Pre_ __deref _Group_(annotes _SAL_nop_impl_)
    #define __drv_out(annotes) _Post_ _Group_(annotes _SAL_nop_impl_)
    #define __drv_out_deref(annotes) _Post_ __deref _Group_(annotes _SAL_nop_impl_)
    #define __drv_when(cond, annotes) _When_(cond, annotes _SAL_nop_impl_)
    #define __drv_at(expr,annotes) _At_(expr, annotes _SAL_nop_impl_)

    #define __drv_fun(annotes) _At_(return,annotes _SAL_nop_impl_)
    #define __drv_ret(annotes) _At_(return,annotes _SAL_nop_impl_)
    #define __drv_arg(expr,annotes) _At_(expr,annotes)
    #define __drv_unit(p)                                                     \
      typedef int ___drv_unit_##p                                             \
                __GENSYM(__prefast_flag_kernel_driver_mode);

    // Internal macros for convenience
    #define ___drv_unit_internal_kernel_driver                                \
         _SAL_L_Source_(__drv_unit_internal_kernel_driver, (), _SA_annotes0(SAL_internal_kernel_driver))

    //
    // __drv_unit
    //
    // Flags for compilation units that indicated specifically what kind of
    // code it is.
    // These should be coded as early as possible in any compilation unit
    // (.c/.cpp file) that doesn't get the correct default.     Whether before
    // or after __internal_kernel_driver is immaterial as long as it will
    // successfully parse.
    //
    // Indicate that the code is kernel, but not driver, code.
    #define ___drv_unit_kernel_code                                           \
            _SAL_L_Source_(___drv_unit_kernel_code, (), _SA_annotes0(SAL_kernel) _SA_annotes0(SAL_nodriver))

    // Indicate that the code is kernel, driver, code.
    #define ___drv_unit_kernel_driver                                         \
            _SAL_L_Source_(___drv_unit_kernel_driver, (), _SA_annotes0(SAL_kernel) _SA_annotes0(SAL_driver))

    // Indicate that the code is a user mode driver.
    #define ___drv_unit_user_driver                                           \
            _SAL_L_Source_(___drv_unit_user_driver, (), _SA_annotes0(SAL_nokernel) _SA_annotes0(SAL_driver))

    // Indicate that the code is ordinary user mode code.
    #define ___drv_unit_user_code                                             \
            _SAL_L_Source_(___drv_unit_user_code, (), _SA_annotes0(SAL_nokernel) _SA_annotes0(SAL_nodriver))


    // These are needed for backwards compatability.
    #ifndef __internal_kernel_driver

    #define __internal_kernel_driver   __drv_unit(internal_kernel_driver)
    #define __kernel_code              __drv_unit(kernel_code)
    #define __kernel_driver            __drv_unit(kernel_driver)
    #define __user_driver              __drv_unit(user_driver)
    #define __user_code                __drv_unit(user_code)

    #endif

    // ---------------------------------------------------------------------
    // Syntatic utilities:
    //
    // Needed to make the annotations convenient to use.
    //
    // So we can use a macro name that might be used in #ifdef context,
    // where it's defined with no value.
    // This should only be used inside a _When_ condition.
    //
    #define __drv_defined(x) _Macro_defined_( #x )

    // ---------------------------------------------------------------------
    // Callback properties:
    //
    // __drv_functionClass(x)
    //
    // Flag that the the annotated function
    // is a member of that function class. Some class names are recognized
    // by PREfast itself for special treatment.
    // This can be tested by the condition function _In_function_class_()
    //
    #define __drv_functionClass(x) _SAL1_Source_(__drv_functionClass, (x), _Function_class_(x))

    // ---------------------------------------------------------------------
    // Resources:
    //
    // __drv_acquiresResource(kind)
    // __drv_releasesResource(kind)
    // __drv_acquiresResourceGlobal(kind,param)
    // __drv_releasesResourceGlobal(kind,param)
    // __drv_mustHold(kind)
    // __drv_neverHold(kind)
    // __drv_mustHoldGlobal(kind,param)
    // __drv_neverHoldGlobal(kind,param)
    //
    // Flag that the annotated parameter acquires a resource of type kind.
    //

    __ANNOTATION(SAL_acquire(__In_impl_ char *);)
    #define _Kernel_acquires_resource_(kind)                                  \
        _SAL2_Source_(_Kernel_acquires_resource_, (#kind), _Post_ _SA_annotes1(SAL_acquire, #kind))

    #define __drv_acquiresResource(kind)                                      \
        _SAL1_1_Source_(__drv_acquiresResource, (kind), _Acquires_lock_(_Curr_))

    //
    // Flag that the annotated parameter releases a resource of type kind.
    //
    __ANNOTATION(SAL_release(__In_impl_ char *);)
    #define _Kernel_releases_resource_(kind)                                  \
        _SAL2_Source_(_Kernel_releases_resource_, (#kind), _Post_ _SA_annotes1(SAL_release, #kind))
    #define __drv_releasesResource(kind)                                      \
        _SAL1_1_Source_(__drv_releasesResource, (kind), _Releases_lock_(_Curr_))

    //
    // Flag that the annotated object acquires a global resource named by param
    //
    __ANNOTATION(SAL_acquireGlobal(__In_impl_ char *, ...);)
    #define __drv_innerAcquiresGlobal(kind, param)                            \
        _SAL1_1_Source_(_drv_innerAcquiresGlobal, (#kind, param\t), _Post_ _SA_annotes2(SAL_acquireGlobal, #kind, param\t))
    #define __drv_acquiresResourceGlobal(kind,param)                          \
        _SAL1_1_Source_(__drv_acquiresResourceGlobal, (kind, param), _Acquires_lock_(param))

    //
    // Flag that the annotated object releases a global resource named by param
    //
    __ANNOTATION(SAL_releaseGlobal(__In_impl_ char *, ...);)
    #define __drv_innerReleasesGlobal(kind, param)                            \
        _SAL1_1_Source_(__drv_InnerReleasesGlobal, (#kind, param\t), _Post_ _SA_annotes2(SAL_releaseGlobal,#kind, param\t))
    #define __drv_releasesResourceGlobal(kind, param)                         \
        _SAL1_1_Source_(__drv_releasesResourceGlobal, (kind, param), _Releases_lock_(param))

    //
    // Flag that the annotated parameter must hold a resource of type kind
    //
    __ANNOTATION(SAL_mustHold(__In_impl_ char *);)
    #define _Kernel_requires_resource_held_(kind)                             \
        _SAL2_Source_(_Kernel_requires_resource_held_, (#kind), _Pre_ _SA_annotes1(SAL_mustHold, #kind))

    #define __drv_mustHold(kind)                                              \
        _SAL_L_Source_(_drv_mustHold, (kind),                                 \
        _When_(!_Arg_comp_(#kind,"Memory"), _Requires_lock_held_(_Curr_))     \
        _When_(_Arg_comp_(#kind,"Memory"), _Kernel_requires_resource_held_(kind)))

    //
    // Flag that the annotated object must hold a global resource named by param
    //
    __ANNOTATION(SAL_mustHoldGlobal(__In_impl_ char *, ...);)
    #define __drv_innerMustHoldGlobal(kind, param)                            \
        _SAL_L_Source_(__drv_innerMustHoldGlobal, (#kind, param\t), _Pre_ _SA_annotes2(SAL_mustHoldGlobal, #kind, param\t))
    #define __drv_mustHoldGlobal(kind,param)                                  \
        _SAL1_1_Source_(__drv_mustHoldGlobal, (kind, param), _Requires_lock_held_(param))

    //
    // Flag that the annotated parameter must never hold a resource of type kind
    //
    __ANNOTATION(SAL_neverHold(__In_impl_ char *);)
    #define _Kernel_requires_resource_not_held_(kind)                         \
        _SAL2_Source_(_Kernel_requires_resource_not_held_, (#kind), _Pre_ _SA_annotes1(SAL_neverHold, #kind))

    #define __drv_neverHold(kind)                                             \
        _SAL1_1_Source_(__drv_neverHold, (kind), _Requires_lock_not_held_(_Curr_))

    //
    // Flag that the annotated object must never hold a global resource
    // of type kind named by param.
    //
    __ANNOTATION(SAL_neverHoldGlobal(__In_impl_ char *, ...);)
    #define __drv_innerNeverHoldGlobal(kind, param)                           \
        _SAL_L_Source_(__drv_innterNeverHoldGlobal, (#kind, param\t), _Pre_ _SA_annotes2(SAL_neverHoldGlobal, #kind, param\t))
    #define __drv_neverHoldGlobal(kind,param)                                 \
        _SAL1_1_Source_(__drv_neverHoldGlobal, (kind, param), _Requires_lock_not_held_(param))

    // Predicates to determine if a resource is held
    __PRIMOP(int, _Holds_resource_(__In_impl_ __deferTypecheck char *,__In_impl_ char *);)
    __PRIMOP(int, _Holds_resource_global_(__In_impl_ char *, ...);)

    //
    // ---------------------------------------------------------------------
    // IRQL annotations:
    //
    // _IRQL_raises_(irql)
    // _IRQL_requires_(irql)
    // _IRQL_requires_max_(irql)
    // _IRQL_requires_min_(irql)
    // _IRQL_saves_
    // _IRQL_saves_global_(kind,param)
    // _IRQL_restores_
    // _IRQL_restores_global_(kind,param)
    // _IRQL_always_function_min_(irql)
    // _IRQL_always_function_max_(irql)
    // _IRQL_requires_same_
    // _IRQL_uses_cancel_
    // 
    // Legacy IRQL annotations:
    //   Legacy annotation:        Use instead:
    //   __drv_setsIRQL            (Obsolete, use _IRQL_raises_)
    //   __drv_raisesIRQL          _IRQL_raises_
    //   __drv_requiresIRQL        _IRQL_requires_
    //   __drv_maxIRQL             _IRQL_requires_max_
    //   __drv_minIRQL             _IRQL_requires_min_
    //   __drv_savesIRQL           _IRQL_saves_
    //   __drv_savesIRQLGlobal     _IRQL_saves_global_
    //   __drv_restoresIRQL        _IRQL_restores_
    //   __drv_restoresIRQLGlobal  _IRQL_restores_global_
    //   __drv_minFunctionIRQL     _IRQL_always_function_min_
    //   __drv_maxFunctionIRQL     _IRQL_always_function_max_
    //   __drv_sameIRQL            _IRQL_requires_same_
    //   __drv_useCancelIRQL       _IRQL_uses_cancel_
    //

    //
    // The function exits at IRQL irql (obsolete, use _IRQL_raises_)
    //
    #define __drv_setsIRQL(irql)  /* see kernelspecs.h, legacy */

    //
    // The function exits at IRQL irql, but this may only raise the irql.
    //
    #define _IRQL_raises_(irql)  /* see kernelspecs.h */
    #define __drv_raisesIRQL _SAL1_1_Source_(__drv_raisesIRQL, (), _IRQL_raises_) /* legacy */

    //
    // The called function must be entered at IRQL level
    //
    #define _IRQL_requires_(irql)  /* see kernelspecs.h */
    #define __drv_requiresIRQL(irql) _SAL1_1_Source_(__drv_requiresIRQL, (irql), _IRQL_requires_(irql)) /* legacy */

    //
    // The maximum IRQL at which the function may be called.
    //
    #define _IRQL_requires_max_(irql)  /* see kernelspecs.h */
    #define __drv_maxIRQL(irql) _SAL1_1_Source_(__drv_maxIRQL, (irql), _IRQL_requires_max_(irql)) /* legacy */

    //
    // The minimum IRQL at which the function may be called.
    //
    #define _IRQL_requires_min_(irql)  /* see kernelspecs.h */
    #define __drv_minIRQL(irql) _SAL1_1_Source_(__drv_minIRQL, (irql), _IRQL_requires_min_(irql)) /* legacy */

    //
    // The current IRQL is saved in the annotated parameter
    //
    #define _IRQL_saves_  /* see kernelspecs.h */
    #define __drv_savesIRQL _SAL1_1_Source_(__drv_savesIRQL, (), _IRQL_saves_) /* legacy */

    //
    // The current IRQL is saved in the (otherwise anonymous) global object
    // identified by kind and further refined by param.
    //
    #define _IRQL_saves_global_(kind,param)  /* see kernelspecs.h */
    #define __drv_savesIRQLGlobal(kind,param) _SAL1_1_Source_(__drv_savesIRQLGlobal, (kind,param), _IRQL_saves_global_(kind,param)) /* legacy */

    //
    // The current IRQL is restored from the annotated parameter
    //
    #define _IRQL_restores_  /* see kernelspecs.h */
    #define __drv_restoresIRQL _SAL1_1_Source_(__drv_restoresIRQL, (), _IRQL_restores_) /* legacy */

    //
    // The current IRQL is restored from the (otherwise anonymous) global
    // object identified by kind and further refined by param.
    //
    #define _IRQL_restores_global_(kind,param)  /* see kernelspecs.h */
    #define __drv_restoresIRQLGlobal(kind,param) _SAL1_1_Source_(__drv_restoresIRQLGlobal, (kind,param), _IRQL_restores_global_(kind,param)) /* legacy */

    //
    // The minimum IRQL to which the function can lower itself.  The IRQL
    // at entry is assumed to be that value unless overridden.
    //
    #define _IRQL_always_function_min_(irql)  /* see kernelspecs.h */
    #define __drv_minFunctionIRQL(irql) _SAL1_1_Source_(__drv_minFunctionIRQL, (irql), _IRQL_always_function_min_(irql)) /* legacy */

    //
    // The maximum IRQL to which the function can raise itself.
    //
    #define _IRQL_always_function_max_(irql)  /* see kernelspecs.h */
    #define __drv_maxFunctionIRQL(irql) _SAL1_1_Source_(__drv_maxFunctionIRQL, (irql), _IRQL_always_function_max_(irql)) /* legacy */

    //
    // The function must exit with the same IRQL it was entered with.
    // (It may change it but it must restore it.)
    //
    #define _IRQL_requires_same_  /* see kernelspecs.h */
    #define __drv_sameIRQL _SAL1_1_Source_(__drv_sameIRQL, (), _IRQL_requires_same_) /* legacy */

    //
    // The annotated parameter contains the cancelIRQL, which will be restored
    // by the called function.
    //
    #define _IRQL_uses_cancel_  /* see kernelspecs.h */
    #define __drv_useCancelIRQL _SAL1_1_Source_(__drv_usesCancelIRQL, (), _IRQL_uses_cancel_) /* legacy */

    //
    // The annotated parameter is an IRQL that will be restored and a new (probably the same)
    // value will be inserted.  (Use this in preference to directly coding it.)
    #undef _IRQL_inout_
    #define _IRQL_inout_                                                      \
        _IRQL_saves_ _IRQL_restores_

    // ---------------------------------------------------------------------
    // Specific function behaviors

    // The annotated function clears the requirement that DoInitializeing
    // is cleared (or not).
    __ANNOTATION(SAL_clearDoInit(enum __SAL_YesNo);)
    #define _Kernel_clear_do_init_(yesNo)                                     \
        _SAL2_Source_(_Kernel_clear_do_init_, (yesNo), _Post_ _SA_annotes1(SAL_clearDoInit,yesNo))
    #define __drv_clearDoInit(yesNo) _SAL1_1_Source_(__drv_clearDoInit, (yesNo), _Kernel_clear_do_init_(yesNo)) /* legacy */

    // This is (or is like) IoGetDmaAdapter: look for misuse of DMA pointers
    __ANNOTATION(SAL_IoGetDmaAdapter(void);)
    #define _Kernel_IoGetDmaAdapter_                                          \
        _SAL2_Source_(_Kernel_IoGetDmaAdapter_, (), _Post_ _SA_annotes0(SAL_IoGetDmaAdapter))
    #define __drv_IoGetDmaAdapter _SAL1_1_Source_(__drv_IoGetDmaAdapter, (), _Kernel_IoGetDmaAdapter_) /* legacy */

    // ---------------------------------------------------------------------
    // Function and out parameter return values.
    //
    // __drv_valueIs(<list>)
    //
    // The function being annotated will return each of the specified values
    // during simulation.  The items in the list are <relational op><constant>,
    // e.g. ==0 or <0.
    // This is a ; separated list of values.  The internal parser will accept
    // a comma-separated list.  In the future __VA_ARGS__ could be used.
    // See the documentation for use of this.
    //

    __ANNOTATION(SAL_return(__In_impl_ __AuToQuOtE char *);)
    #define __drv_valueIs(arglist)                                            \
            _SAL1_1_Source_(__drv_valueIs, (arglist), _Post_ _SA_annotes1(SAL_return,arglist))

    // ---------------------------------------------------------------------
    // Additional parameter checking.
    //
    // __drv_constant
    // __drv_nonConstant
    // __drv_strictTypeMatch(mode)
    // __drv_strictType(type,mode)
    //
    // The actual parameter must evaluate to a constant (not a const).
    //
    #define __drv_constant _SAL1_1_Source_(__drv_constant, (), _Literal_)

    //
    // The actual parameter may never evaluate to a numeric constant
    // (exclusive of a const symbol).
    //
    #define __drv_nonConstant _SAL1_1_Source_(__drv_nonConstant, (), _Notliteral_)

    //
    // The actual parameter must match the type of the annotated formal
    // within the specifications set by mode.
    //
    __ANNOTATION(SAL_strictTypeMatch(__int64);)
    #define __drv_strictTypeMatch(mode)                                       \
        _SAL1_1_Source_(__drv_strictTypeMatch, (mode), _Pre_ _SA_annotes1(SAL_strictTypeMatch,mode))

    //
    // The actual parameter must match the type of typename (below)
    // within the specifications set by mode.
    //
    __ANNOTATION(SAL_strictType(__In_impl_ __AuToQuOtE char *);) // currently 1/2 args
    #define __drv_strictType(typename,mode)                                   \
        _SAL1_1_Source_(__drv_strictType, (typename,mode), _Pre_ _SA_annotes2(SAL_strictType, typename, mode))
    //
    //    The following modes are defined:
        #define __drv_typeConst   0    // constants of that type
        #define __drv_typeCond    1    // plus ?:
        #define __drv_typeBitset  2    // plus all operators
        #define __drv_typeExpr    3    // plus literal constants
    //
    // The actual parameter must be data (not a pointer).  Used to
    // prevent passing pointers to pointers when pointers to structures
    // are needed (because &pXXX is a common error when pXXX is
    // intended).
    #define __drv_notPointer _SAL1_1_Source_(__drv_notPointer, (), _Pre_ _SA_annotes1(SAL_mayBePointer,__no))

    //
    // Convenience for the most common form of the above.
    #define __drv_isObjectPointer _SAL1_1_Source_(__drv_isObjectPointer, (), _Points_to_data_)

    // ---------------------------------------------------------------------
    // Memory management
    //
    // __drv_aliasesMem
    // __drv_allocatesMem
    // __drv_freesMem
    //
    // The annotated parameter is "kept" by the function, creating an
    // alias, and relieving any obligation to free the object.
    //
    __ANNOTATION(SAL_IsAliased(void);)
    #define __drv_aliasesMem _SAL_L_Source_(__drv_aliasesMem, (), _Post_ _SA_annotes0(SAL_IsAliased))

    //
    // Allocate/release memory-like objects.
    // Kind is unused, but should be "mem" for malloc/free
    // and "object" for new/delete.
    __ANNOTATION(SAL_NeedsRelease(enum __SAL_YesNo);)
    #define __drv_allocatesMem(kind) _SAL_L_Source_(__drv_allocatesMem, (kind), _Post_ _SA_annotes1(SAL_NeedsRelease,__yes))

    #define __drv_freesMem(kind) _SAL_L_Source_(__drv_freesMem, (kind), _Post_ _SA_annotes1(SAL_NeedsRelease,__no))

    // ---------------------------------------------------------------------
    // Additional diagnostics
    //
    // __drv_preferredFunction
    // __drv_reportError
    //
    //
    // Function 'func' should be used for reason 'why'.  Often used
    // conditionally.
    //
    __ANNOTATION(SAL_preferredFunction(__In_impl_ __AuToQuOtE char *,
        __In_impl_ __AuToQuOtE char *);)
    __ANNOTATION(SAL_preferredFunction3(__In_impl_ __AuToQuOtE char *,
        __In_impl_ __AuToQuOtE char *, __In_impl_ __int64);)
    #define __drv_preferredFunction(func,why)                                 \
        _SAL_L_Source_(__drv_preferredFunction, (func,why), _Pre_ _SA_annotes2(SAL_preferredFunction, func, why))

    //
    // The error given by 'why' was detected.  Used conditionally.
    //
    __ANNOTATION(SAL_error(__In_impl_ __AuToQuOtE char *);)
    __ANNOTATION(SAL_error2(__In_impl_ __AuToQuOtE char *, __In_impl_ __int64);)
    #define __drv_reportError(why)                                            \
        _SAL_L_Source_(__drv_reportError, (why), _Pre_ _SA_annotes1(SAL_error,why))

    // ---------------------------------------------------------------------
    // Floating point save/restore:
    //
    // _Kernel_float_saved_
    // _Kernel_float_restored_
    // _Kernel_float_used_
    // __drv_floatSaved        (legacy, replaced by _Kernel_float_saved_)
    // __drv_floatRestored     (legacy, replaced by _Kernel_float_restored_)
    // __drv_floatUsed         (legacy, replaced by _Kernel_float_used_)
    //
    // The floating point hardware was saved (available to kernel)
    __ANNOTATION(SAL_floatSaved(void);)
    #define _Kernel_float_saved_ _SAL2_Source_(_Kernel_float_saved_, (), _Post_ _SA_annotes0(SAL_floatSaved))
    #define __drv_floatSaved _SAL1_1_Source_(__drv_floatSaved, (), _Kernel_float_saved_) /* legacy */

    //
    // The floating point hardware was restored (no longer available)
    __ANNOTATION(SAL_floatRestored(void);)
    #define _Kernel_float_restored_ _SAL2_Source_(_Kernel_float_restored_, (), _Post_ _SA_annotes0(SAL_floatRestored))
    #define __drv_floatRestored _SAL1_1_Source_(__drv_floatRestored, (), _Kernel_float_restored_) /* legacy */

    //
    // The function uses floating point.  Functions with floating point
    // in their type signature get this automatically.
    __ANNOTATION(SAL_floatUsed(void);)
    #define _Kernel_float_used_ _SAL2_Source_(_Kernel_float_used_, (), _Post_ _SA_annotes0(SAL_floatUsed))
    #define __drv_floatUsed _SAL1_1_Source_(__drv_floatUsed, (), _Kernel_float_used_) /* legacy */

    // ---------------------------------------------------------------------
    // Usage:
    //
    // __drv_interlocked
    // __drv_inTry
    // __drv_notInTry
    //
    // The parameter is used for interlocked instructions.
    #define __drv_interlocked _SAL1_1_Source_(__drv_interlocked, (), _Interlocked_operand_)

    // The function must be called inside a try block
    #define __drv_inTry _SAL_L_Source_(__drv_inTry, (), _Pre_ _SA_annotes1(SAL_inTry,__yes))

    // The function must not be called inside a try block
    #define __drv_notInTry _SAL_L_Source_(__drv_notInTry, (), _Pre_ _SA_annotes1(SAL_inTry,__no))

    // ---------------------------------------------------------------------
    // FormatString:
    //
    // kind can be "printf", "scanf", "strftime" or "FormatMessage".
    __ANNOTATION(SAL_IsFormatString(__In_impl_ char *);)
    #define __drv_formatString(kind)\
        _SAL1_1_Source_(__drv_formatString, (kind), _SA_annotes1(SAL_IsFormatString, #kind))

    // ---------------------------------------------------------------------
    // Function classes for drivers:
    //

    // Function class for driver dispatch functions:
	#define _Dispatch_type_(x)  _SAL2_Source_(Dispatch_type_, (x), _SA_annotes1(SAL_functionClassNew, #x))

    // Legacy function class for driver dispatch functions:
    #define __drv_dispatchType(x)                                             \
	    _SAL1_1_Source_(__drv_displatchType, (x), _SA_annotes1(SAL_functionClassNew, #x))

    // Legacy function class for driver dispatch functions - special case:
    #define __drv_dispatchType_other\
        _SAL1_1_Source_(__drv_dispatchType_other, (), __drv_dispatchType(IRP_MJ_OTHER))

    // Legacy function class for driver completion functions:
    __ANNOTATION(SAL_completionType(__In_impl_ __AuToQuOtE char *);)
    #define __drv_completionType(kindlist)\
        _SAL1_1_Source_(__drv_completionType, (kindlist), _SA_annotes1(SAL_completionType, #kindlist))

    // Legacy function class for driver callback functions (FDO or PDO):
    __ANNOTATION(SAL_callbackType(__In_impl_ __AuToQuOtE char *);)
    #define __drv_callbackType(kind)\
        _SAL1_1_Source_(__drv_callbackType, (kind), _SA_annotes1(SAL_callbackType, #kind))

    // ---------------------------------------------------------------------
    // Composite:

#ifdef _PREFAST_ // [ expand to nothing immediately to avoid RC problem
    //
    // Exclusive Resources
    //
    #define __drv_acquiresExclusiveResource(kind)                             \
        _SAL1_1_Source_(__drv_acquiresExclusiveResource, (kind), _Acquires_nonreentrant_lock_(_Curr_))

    #define __drv_releasesExclusiveResource(kind)                             \
        _SAL1_Source_(__drv_releasesExclusiveResource, (kind), _Releases_nonreentrant_lock_(_Curr_))

    #define __drv_acquiresExclusiveResourceGlobal(kind, param)                \
        _SAL1_Source_(__drv_acquiresExclusiveResourceGlobal, (kind), _Acquires_nonreentrant_lock_(param))

    #define __drv_releasesExclusiveResourceGlobal(kind, param)                \
        _SAL1_Source_(__drv_releasesExclusiveResourceGlobal, (kind), _Releases_nonreentrant_lock_(param))

    //
    // CancelSpinLock
    //
    #define __drv_acquiresCancelSpinLock                                      \
        _SAL1_Source_(__drv_acquiresCancelSpinLock, (kind), _Acquires_nonreentrant_lock_(_Global_cancel_spin_lock_))

    #define __drv_releasesCancelSpinLock                                      \
        _SAL1_Source_(__drv_releasesCancelSpinLock, (kind), _Releases_nonreentrant_lock_(_Global_cancel_spin_lock_))

    #define __drv_mustHoldCancelSpinLock                                      \
        _SAL1_Source_(__drv_mustHoldCancelSpinLock, (kind), _Requires_lock_held_(_Global_cancel_spin_lock_))

    #define __drv_neverHoldCancelSpinLock                                     \
        _SAL1_Source_(__drv_neverHoldCancelSpinLock, (kind), _Requires_lock_not_held_(_Global_cancel_spin_lock_))

    #define __drv_holdsCancelSpinLock()                                       \
        _SAL1_Source_(__drv_holdsCanselSpinLock, (kind), _Holds_resource_global_("CancelSpinLock",))

    //
    // CriticalRegion
    //
    #define __drv_acquiresCriticalRegion                                      \
        _SAL1_Source_(__drv_acquiresCriticalRegion, (kind), _Acquires_lock_(_Global_critical_region_))

    #define __drv_releasesCriticalRegion                                      \
        _SAL1_Source_(__drv_releasesCriticalRegion, (kind), _Releases_lock_(_Global_critical_region_))

    #define __drv_mustHoldCriticalRegion                                      \
        _SAL1_Source_(__drv_mustHoldCriticalRegion, (kind), _Requires_lock_held_(_Global_critical_region_))

    #define __drv_neverHoldCriticalRegion                                     \
        _SAL1_Source_(__drv_neverHoldCriticalRegion, (kind), _Requires_lock_not_held_(_Global_critical_region_))

    #define __drv_holdsCriticalRegion()                                       \
        _SAL1_Source_(__drv_holdsCriticalRegion, (kind), _Holds_resource_global_("CriticalRegion",))


extern int _Global_priority_region_;
    //
    // PriorityRegion
    //
    #define __drv_acquiresPriorityRegion                                      \
        _SAL1_Source_(__drv_acquiresPrioorityRegion, (kind), _Acquires_lock_(_Global_priority_region_))

    #define __drv_releasesPriorityRegion                                      \
        _SAL1_Source_(__drv_releasesPriorityRegion, (kind), _Releases_lock_(_Global_priority_region_))

    #define __drv_mustHoldPriorityRegion                                      \
        _SAL1_Source_(__drv_mustHoldPriorityRegion, (kind), _Requires_lock_held_(_Global_priority_region_))

    #define __drv_neverHoldPriorityRegion                                     \
        _SAL1_Source_(__drv_neverHoldPriorityRegion, (kind), _Requires_lock_not_held_(_Global_priority_region_))

    #define __drv_holdsPriorityRegion()                                       \
        _SAL1_Source_(__drv_holdsPriorityRegion, (kind), _Holds_resource_global_("PriorityRegion",))

#else // ][

    #define __drv_acquiresExclusiveResource(kind)
    #define __drv_releasesExclusiveResource(kind)
    #define __drv_acquiresExclusiveResourceGlobal(kind, param)
    #define __drv_releasesExclusiveResourceGlobal(kind, param)
    #define __drv_acquiresCancelSpinLock
    #define __drv_releasesCancelSpinLock
    #define __drv_mustHoldCancelSpinLock
    #define __drv_holdsCancelSpinLock()
    #define __drv_neverHoldCancelSpinLock
    #define __drv_acquiresCriticalRegion
    #define __drv_releasesCriticalRegion
    #define __drv_mustHoldCriticalRegion
    #define __drv_neverHoldCriticalRegion
    #define __drv_holdsCriticalRegion()
    #define __drv_acquiresPriorityRegion
    #define __drv_releasesPriorityRegion
    #define __drv_mustHoldPriorityRegion
    #define __drv_neverHoldPriorityRegion
    #define __drv_holdsPriorityRegion()

#endif // ]

    // Passing the cancel Irql to a utility function
    #define _IRQL_is_cancel_  /* see kernelspecs.h */
    #define __drv_isCancelIRQL _IRQL_is_cancel_ /* legacy */

    // Check if this is kernel or driver code
    __PRIMOP(int, _Is_kernel_(void);)
    __PRIMOP(int, _Is_driver_(void);)

#ifdef __cplusplus
}
#endif

#endif // DRIVERSPECS_H

