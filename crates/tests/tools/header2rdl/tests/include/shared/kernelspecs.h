/*****************************************************************************\
*                                                                             *
* KernelSpecs.h - markers for documenting the semantics of driver APIs        *
*                 See DriverSpecs.h for detailed comments                     *
*                 See also <SpecStrings.h>                                    *
*                                                                             *
* Version 1.2.00                                                              *
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
* As noted in DriverSpecs.h, this header contains "real" definitions for
* annotations that either never appear in user space, or which are meaningles
* in user space and are #defined to nothing by DriverSpecs.h.
*
* Further commentary appears in DriverSpecs.h.
\*****************************************************************************/

#ifndef KERNELSPECS_H
#define KERNELSPECS_H

#include "DriverSpecs.h"

#if _MSC_VER > 1000 // [
#pragma once
#endif // ]

#ifdef __cplusplus // [
extern "C" {
#endif // ]

    // ---------------------------------------------------------------------
    // The symbolic IRQL values can sometimes end up undefined, so define
    // the usual ones here, for PREfast purposes only.

    #define DISPATCH_LEVEL 2
    #define APC_LEVEL 1
    #define PASSIVE_LEVEL 0

    #if defined(_X86_)
    #define HIGH_LEVEL 31
    #endif
    #if defined(_AMD64_)
    #define HIGH_LEVEL 15
    #endif
    #if defined(_ARM_)
    #define HIGH_LEVEL 15
    #endif
    #if defined(_IA64_)
    #define HIGH_LEVEL 15
    #endif
    #if defined(_ARM64_)
    #define HIGH_LEVEL 15
    #endif

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
    // ';' inside the parens to keep MIDL happy
    __ANNOTATION(SAL_IRQL(__int64);)
    #undef __drv_setsIRQL
    #define __drv_setsIRQL(irql)                                              \
        _SAL1_Source_(__drv_setsIRQL, (), _Post_ _SA_annotes1(SAL_IRQL,irql)) /* legacy */

    //
    // The function exits at IRQL irql, but this may only raise the irql.
    //
    __ANNOTATION(SAL_raiseIRQL(__int64);)
    #undef _IRQL_raises_
    #define _IRQL_raises_(irql)                                               \
        _SAL2_Source_(_IRQL_raises_, (), _Post_ _SA_annotes1(SAL_raiseIRQL,irql))
    #undef __drv_raisesIRQL
    #define __drv_raisesIRQL(irql) _SAL1_Source_(__drv_raisesIRQL, (), _IRQL_raises_(irql)) /* legacy */

    //
    // The called function must be entered at IRQL level
    //
    __ANNOTATION(SAL_IRQL(__int64);)
    #undef _IRQL_requires_
    #define _IRQL_requires_(irql)                                             \
        _SAL2_Source_(_IRQL_requires_, (irql), _Pre_ _SA_annotes1(SAL_IRQL,irql))
    #undef __drv_requiresIRQL
    #define __drv_requiresIRQL(irql) _SAL1_Source_(__drv_requiresIRQL, (), _IRQL_requires_(irql)) /* legacy */

    //
    // The maximum IRQL at which the function may be called.
    //
    __ANNOTATION(SAL_maxIRQL(__int64);)
    #undef _IRQL_requires_max_
    #define _IRQL_requires_max_(irql)                                         \
        _SAL2_Source_(_IRQL_requires_max_, (irql), _Pre_ _SA_annotes1(SAL_maxIRQL,irql))
    #undef __drv_maxIRQL
    #define __drv_maxIRQL(irql) _SAL1_Source_(__drv_maxIRQL, (), _IRQL_requires_max_(irql)) /* legacy */

    //
    // The minimum IRQL at which the function may be called.
    //
    __ANNOTATION(SAL_minIRQL(__int64);)
    #undef _IRQL_requires_min_
    #define _IRQL_requires_min_(irql)                                         \
        _SAL2_Source_(_IRQL_requires_min_, (irql), _Pre_ _SA_annotes1(SAL_minIRQL,irql))
    #undef __drv_minIRQL
    #define __drv_minIRQL(irql) _SAL1_Source_(__drv_minIRQL, (), _IRQL_requires_min_(irql)) /* legacy */

    //
    // The current IRQL is saved in the annotated parameter
    //
    __ANNOTATION(SAL_saveIRQL(void);)
    #undef _IRQL_saves_
    #define _IRQL_saves_                                                      \
        _SAL2_Source_(_IRQL_saves_, (), _Post_ _SA_annotes0(SAL_saveIRQL))
    #undef __drv_savesIRQL
    #define __drv_savesIRQL _SAL1_Source_(__drv_savesIRQL, (), _IRQL_saves_) /* legacy */

    //
    // The current IRQL is saved in the (otherwise anonymous) global object
    // identified by kind and further refined by param.
    //
    __ANNOTATION(SAL_saveIRQLGlobal(__In_impl_ char *, ...);)
    #undef _IRQL_saves_global_
    #define _IRQL_saves_global_(kind,param)                                   \
        _SAL2_Source_(_IRQL_saves_global_, (kind, param), _Post_ _SA_annotes2(SAL_saveIRQLGlobal,#kind, param\t))
    #undef __drv_savesIRQLGlobal
    #define __drv_savesIRQLGlobal(kind,param) _SAL1_Source_(__drv_savesIRQLGlobal, (), _IRQL_saves_global_(kind, param)) /* legacy */

    //
    // The current IRQL is restored from the annotated parameter
    //
    __ANNOTATION(SAL_restoreIRQL(void);)
    #undef _IRQL_restores_
    #define _IRQL_restores_                                                   \
        _SAL2_Source_(_IRQL_restores_, (), _Post_ _SA_annotes0(SAL_restoreIRQL))
    #undef __drv_restoresIRQL
    #define __drv_restoresIRQL _SAL1_Source_(__drv_restoresIRQL, (), _IRQL_restores_) /* legacy */

    //
    // The current IRQL is restored from the (otherwise anonymous) global
    // object identified by kind and further refined by param.
    //
    __ANNOTATION(SAL_restoreIRQLGlobal(__In_impl_ char *, ...);)
    #undef _IRQL_restores_global_
    #define _IRQL_restores_global_(kind,param)                                \
        _SAL2_Source_(_IRQL_restores_global_, (kind, param), _Post_ _SA_annotes2(SAL_restoreIRQLGlobal, #kind, param\t))
    #undef __drv_restoresIRQLGlobal
    #define __drv_restoresIRQLGlobal(kind,param) _SAL1_Source_(__drv_restoresIRQLGlobal, (), _IRQL_restores_global_(kind, param)) /* legacy */

    //
    // The minimum IRQL to which the function can lower itself. The IRQL
    // at entry is assumed to be that value unless overridden.
    //
    __ANNOTATION(SAL_minFunctionIrql(__int64);)
    #undef _IRQL_always_function_min_
    #define _IRQL_always_function_min_(irql)                                  \
        _SAL2_Source_(_IRQL_always_function_min_, (irql), _Pre_ _SA_annotes1(SAL_minFunctionIrql,irql))
    #undef __drv_minFunctionIRQL
    #define __drv_minFunctionIRQL(irql) _SAL1_Source_(__drv_minFunctionIRQL, (), _IRQL_always_function_min_(irql)) /* legacy */

    //
    // The maximum IRQL to which the function can raise itself.
    //
    __ANNOTATION(SAL_maxFunctionIrql(__int64);)
    #undef _IRQL_always_function_max_
    #define _IRQL_always_function_max_(irql)                                  \
        _SAL2_Source_(_IRQL_always_function_max_, (irql), _Pre_ _SA_annotes1(SAL_maxFunctionIrql,irql))
    #undef __drv_maxFunctionIRQL
    #define __drv_maxFunctionIRQL(irql) _SAL1_Source_(__drv_maxFunctionIRQL, (), _IRQL_always_function_max_(irql) ) /* legacy */

    //
    // The function must exit with the same IRQL it was entered with.
    // (It may change it but it must restore it.)
    //
    __ANNOTATION(SAL_sameIRQL(void);)
    #undef _IRQL_requires_same_
    #define _IRQL_requires_same_                                              \
        _SAL2_Source_(_IRQL_requires_same_, (), _Post_ _SA_annotes0(SAL_sameIRQL))
    #undef __drv_sameIRQL
    #define __drv_sameIRQL _SAL1_Source_(__drv_sameIRQL, (), _IRQL_requires_same_) /* legacy */

    //
    // The annotated parameter contains the cancelIRQL, which will be restored
    // by the called function.
    //
    __ANNOTATION(SAL_UseCancelIrql(void);)
    #undef _IRQL_uses_cancel_
    #define _IRQL_uses_cancel_                                                \
        _SAL2_Source_(_IRQL_uses_cancel_, (), _Post_ _SA_annotes0(SAL_UseCancelIrql))
    #undef __drv_useCancelIRQL
    #define __drv_useCancelIRQL _SAL1_Source_(__drv_useCancelIRQL, (), _IRQL_uses_cancel_) /* legacy */

    //
    // The annotated parameter is an IRQL that will be restored and a new (probably the same)
    // value will be inserted.  (Use this in preference to directly coding it.)
    #undef _IRQL_inout_
    #define _IRQL_inout_                                                      \
        _IRQL_saves_ _IRQL_restores_


#ifdef _PREFAST_ // RC workaround; already #defined to nothing if not needed
    // Passing the cancel Irql to a utility function
    #undef _IRQL_is_cancel_
    #define _IRQL_is_cancel_                                                  \
        _SAL2_Source_(_IRQL_is_cancel_, (), _IRQL_uses_cancel_                \
        _Releases_nonreentrant_lock_(_Global_cancel_spin_lock_)               \
        _At_(return, _IRQL_always_function_min_(DISPATCH_LEVEL)               \
                 _IRQL_requires_(DISPATCH_LEVEL))
    #undef __drv_isCancelIRQL
    #define __drv_isCancelIRQL _SAL1_Source_(__drv_isCancelIRQL, (), _IRQL_is_cancel_) /* legacy */
#endif

#ifdef __cplusplus
}
#endif

#endif // KERNELSPECS_H
