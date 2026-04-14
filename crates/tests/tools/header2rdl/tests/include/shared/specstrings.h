#ifndef SPECSTRINGS_H // [
#define SPECSTRINGS_H

/***
*specstrings.h - support for markers for documenting the semantics of APIs
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*       [Public]
****/

/*************************************************************************
* See specstrings_strict.h for documentation of all user visible macros.
*************************************************************************/

#if _MSC_VER
#pragma once
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#endif

#if !defined(_SAL_VERSION_SAL2) // [

 #if defined(__BUILDMACHINE__) || defined(_USE_SAL2_ONLY)
  #define _SAL_VERSION_SAL2(_A) SAL_2_Clean_Violation_using ## _A
 #else
  #define _SAL_VERSION_SAL2(_A)
 #endif

 #ifdef _USE_SAL2_ONLY
  #define _SAL2_STRICT
  #define _SAL_VERSION_CHECK(_A) _SAL_VERSION_SAL2(_A)
 #else
  #define _SAL_VERSION_CHECK(_A)
 #endif

 #ifndef SAL_VERSION_CHECK
  #define SAL_VERSION_CHECK(_A) _SAL_VERSION_CHECK(_A)
  #define SAL_VERSION_SAL2(_A) _SAL_VERSION_SAL2(_A)
 #endif

#endif // ]

/* Begin compatibility fixes required for Win8/VS2012 RTM sal.h */
#ifndef _SAL_L_Source_

// Some annotations aren't officially SAL2 yet.
#if _USE_ATTRIBUTES_FOR_SAL /*IFSTRIP=IGN*/
#define _SAL_L_Source_(Name, args, annotes) _SA_annotes3(SAL_name, #Name, "", "2") _Group_(annotes _SAL_nop_impl_)
#else
#define _SAL_L_Source_(Name, args, annotes) _SA_annotes3(SAL_name, #Name, "", "2") _GrouP_(annotes _SAL_nop_impl_)
#endif

#endif

/* End compatibility fixes required for Win8/VS2012 RTM sal.h */

#include <sal.h>

/* This version symbol is deprecated in favor of __SAL_H_VER */
#ifndef __SAL_H_FULL_VER
#define __SAL_H_FULL_VER 140050727
#endif

#ifdef  __cplusplus
extern "C" {
#endif

/* version specific fixes to bring sal.h upto date */
#if __SAL_H_FULL_VER <= 140050727 // [

#if !defined(__midl) && defined(_PREFAST_) && _MSC_VER >= 1000 // [

/* Missing from RTM sal.h */
#define __inner_bound                     _SAL_L_Source_(__inner_bound, (), _SA_annotes0(SAL_bound))
#define __inner_range(lb,ub)              _SAL_L_Source_(__inner_range, (lb,ub), _SA_annotes2(SAL_range,lb,ub))
#define __inner_assume_bound_dec          __inline __nothrow void __AssumeBoundInt(_Post_ __inner_bound int i) {(void)i;}
#define __inner_assume_bound(i)           __AssumeBoundInt(i);
#define __inner_allocator                 _SAL_L_Source_(__inner_allocater, (), _SA_annotes0(SAL_allocator))

#define __static_context(ctx, annotes) \
    _SAL_L_Source_(__static_context, (ctx, annotes), _SA_annotes1(SAL_context,ctx) _Group_(__nop_impl(annotes)))

#define __failure(x) __static_context(SAL_return_convention, \
    _SAL_L_Source_(__failure, (x), _SA_annotes1(SAL_failure,x)))

__ANNOTATION(SAL_valueUndefined(void));
#define __valueUndefined _SAL_L_Source_(__valueUndefined, (), _SA_annotes0(SAL_valueUndefined))

enum __SAL_failureKind{__failureUnspecified = 0, __failureUndefined = 1};

__ANNOTATION(SAL_failureDefault(enum __SAL_failureKind));
#define __failureDefault(kind) _SAL_L_Source_(__failureDefault, (kind), __static_context(SAL_return_convention,  \
        _SA_annotes1(SAL_failureDefault,kind)))

#else // ][

#define __inner_bound
#define __inner_range(lb,ub)
#define __inner_assume_bound_dec
#define __inner_assume_bound(i)
#define __inner_allocator

#define __static_context(ctx, annotes)
#define __failure(x)
#define __valueUndefined
#define __failureDefault(x)

#endif // ]

#define __xcount(size)                                          _SAL1_Source_(__xcount, (size), __notnull __inexpressible_writableTo(size))
#define __in_xcount(size)                                       _SAL1_Source_(__in_xcount, (size),  __in _Pre_ __inexpressible_readableTo(size))
#define __out_xcount(size)                                      _SAL1_Source_(__out_xcount, (size),  __xcount(size) _Post_ __valid __refparam)
#define __out_xcount_part(size,length)                          _SAL1_Source_(__out_xcount_part_, (size,length),  __out_xcount(size) _Post_ __inexpressible_readableTo(length))
#define __out_xcount_full(size)                                 _SAL1_Source_(__out_xcount_full, (size),  __out_xcount_part(size,size))
#define __inout_xcount(size)                                    _SAL1_Source_(__inout_xcount, (size), __out_xcount(size) _Pre_ __valid)
#define __inout_xcount_part(size,length)                        _SAL1_Source_(__inout_xcount_part, (size,length),  __out_xcount_part(size,length) _Pre_ __valid _Pre_ __inexpressible_readableTo(length))
#define __inout_xcount_full(size)                               _SAL1_Source_(__inout_xcount_full, (size),  __inout_xcount_part(size,size))
#define __xcount_opt(size)                                      _SAL1_Source_(__xcount_opt, (size),  __xcount(size)                              __exceptthat __maybenull)
#define __in_xcount_opt(size)                                   _SAL1_Source_(__in_xcount_opt, (size),  __in_xcount(size)                           __exceptthat __maybenull)
#define __out_xcount_opt(size)                                  _SAL1_Source_(__out_xcount_opt, (size),  __out_xcount(size)                          __exceptthat __maybenull)
#define __out_xcount_part_opt(size,length)                      _SAL1_Source_(__out_xcount_part_opt, (size,length),  __out_xcount_part(size,length)              __exceptthat __maybenull)
#define __out_xcount_full_opt(size)                             _SAL1_Source_(__out_xcount_full_opt, (size),  __out_xcount_full(size)                     __exceptthat __maybenull)
#define __inout_xcount_opt(size)                                _SAL1_Source_(__inout_xcount_opt, (size),  __inout_xcount(size)                        __exceptthat __maybenull)
#define __inout_xcount_part_opt(size,length)                    _SAL1_Source_(__inout_xcount_part_opt, (size, length),  __inout_xcount_part(size,length)            __exceptthat __maybenull)
#define __inout_xcount_full_opt(size)                           _SAL1_Source_(__inout_xcount_full_opt, (size),  __inout_xcount_full(size)                   __exceptthat __maybenull)
#define __deref_xcount(size)                                    _SAL1_Source_(__deref_xcount, (size),  __ecount(1) _Post_ __elem_readableTo(1) _Post_ __deref __notnull _Post_ __deref __inexpressible_writableTo(size))
#define __deref_in                                              _SAL1_Source_(__deref_in, (),  __in _Pre_ __deref __deref __readonly)
#define __deref_in_ecount(size)                                 _SAL1_Source_(__deref_in_ecount, (size),  __deref_in _Pre_ __deref __elem_readableTo(size))
#define __deref_in_bcount(size)                                 _SAL1_Source_(__deref_in_bcount, (size),  __deref_in _Pre_ __deref __byte_readableTo(size))
#define __deref_in_xcount(size)                                 _SAL1_Source_(__deref_in_xcount, (size),  __deref_in _Pre_ __deref __inexpressible_readableTo(size))
#define __deref_out_xcount(size)                                _SAL1_Source_(__deref_out_xcount, (size),  __deref_xcount(size) _Post_ __deref __valid __refparam)
#define __deref_out_xcount_part(size,length)                    _SAL1_Source_(__deref_out_xcount_part, (size,length),  __deref_out_xcount(size) _Post_ __deref __inexpressible_readableTo(length))
#define __deref_out_xcount_full(size)                           _SAL1_Source_(__deref_out_xcount_full, (size),  __deref_out_xcount_part(size,size))
#define __deref_out_xcount(size)                                _SAL1_Source_(__deref_out_xcount, (size),  __deref_xcount(size) _Post_ __deref __valid __refparam)
#define __inout_xcount_opt(size)                                _SAL1_Source_(__inout_xcount_opt, (size),  __inout_xcount(size)                        __exceptthat __maybenull)
#define __deref_xcount(size)                                    _SAL1_Source_(__deref_xcount, (size),  __ecount(1) _Post_ __elem_readableTo(1) _Post_ __deref __notnull _Post_ __deref __inexpressible_writableTo(size))
#define __deref_in                                              _SAL1_Source_(__deref_in, (),  __in _Pre_ __deref __deref __readonly)
#define __deref_in_ecount(size)                                 _SAL1_Source_(__deref_in_ecount, (size),  __deref_in _Pre_ __deref __elem_readableTo(size))
#define __deref_in_bcount(size)                                 _SAL1_Source_(__deref_in_bcount, (size),  __deref_in _Pre_ __deref __byte_readableTo(size))
#define __deref_in_xcount(size)                                 _SAL1_Source_(__deref_in_xcount, (size),  __deref_in _Pre_ __deref __inexpressible_readableTo(size))
#define __deref_out_xcount(size)                                _SAL1_Source_(__deref_out_xcount, (size),  __deref_xcount(size) _Post_ __deref __valid __refparam)
#define __deref_out_xcount_part(size,length)                    _SAL1_Source_(__deref_out_xcount_part, (size,length),  __deref_out_xcount(size) _Post_ __deref __inexpressible_readableTo(length))
#define __deref_out_xcount_full(size)                           _SAL1_Source_(__deref_out_xcount_full, (size),  __deref_out_xcount_part(size,size))
#define __deref_out_xcount(size)                                _SAL1_Source_(__deref_out_xcount, (size),  __deref_xcount(size) _Post_ __deref __valid __refparam)
#define __deref_inout_xcount(size)                              _SAL1_Source_(__deref_inout_xcount, (size),  __deref_inout _Pre_ __deref __inexpressible_writableTo(size) _Post_ __deref __inexpressible_writableTo(size))
#define __deref_inout_xcount_part(size,length)                  _SAL1_Source_(__deref_inout_xcount_part, (size,length),  __deref_inout_xcount(size) _Pre_ __deref __inexpressible_readableTo(length) _Post_ __deref __inexpressible_readableTo(length))
#define __deref_inout_xcount_full(size)                         _SAL1_Source_(__deref_inout_xcount_full, (size),  __deref_inout_xcount_part(size,size))
#define __deref_xcount_opt(size)                                _SAL1_Source_(__deref_xcount_opt, (size),  __deref_xcount(size)                        _Post_ __deref __exceptthat __maybenull)
#define __deref_in_opt                                          _SAL1_Source_(__deref_in_opt, (),  __deref_in                                  _Pre_ __deref __exceptthat __maybenull)
#define __deref_in_opt_out                                      _SAL1_Source_(__deref_in_opt_out, (),  __deref_inout                               _Pre_ __deref __exceptthat __maybenull  _Post_ __deref __notnull)
#define __deref_in_ecount_opt(size)                             _SAL1_Source_(__deref_in_ecount_opt, (size),  __deref_in_ecount(size)                     _Pre_ __deref __exceptthat __maybenull)
#define __deref_in_bcount_opt(size)                             _SAL1_Source_(__deref_in_bcount_opt_, (size),  __deref_in_bcount(size)                     _Pre_ __deref __exceptthat __maybenull)
#define __deref_in_xcount_opt(size)                             _SAL1_Source_(__deref_in_xcount_opt, (size),  __deref_in_xcount(size)                     _Pre_ __deref __exceptthat __maybenull)
#define __deref_out_xcount_opt(size)                            _SAL1_Source_(__deref_out_xcount_opt, (size),  __deref_out_xcount(size)                    _Post_ __deref __exceptthat __maybenull)
#define __deref_out_xcount_part_opt(size,length)                _SAL1_Source_(__deref_out_xcount_part_opt, (size,length),  __deref_out_xcount_part(size,length)        _Post_ __deref __exceptthat __maybenull)
#define __deref_out_xcount_full_opt(size)                       _SAL1_Source_(__deref_out_xcount_full_opt, (size),  __deref_out_xcount_full(size)               _Post_ __deref __exceptthat __maybenull)
#define __deref_inout_xcount_opt(size)                          _SAL1_Source_(__deref_inout_xcount_opt, (size),  __deref_inout_xcount(size)                  _Pre_ __deref __exceptthat __maybenull _Post_ __deref __exceptthat __maybenull)
#define __deref_inout_xcount_part_opt(size,length)              _SAL1_Source_(__deref_inout_xcount_part_opt, (size,length),  __deref_inout_xcount_part(size,length)      _Pre_ __deref __exceptthat __maybenull _Post_ __deref __exceptthat __maybenull)
#define __deref_inout_xcount_full_opt(size)                     _SAL1_Source_(__deref_inout_xcount_full_opt, (size),  __deref_inout_xcount_full(size)             _Pre_ __deref __exceptthat __maybenull _Post_ __deref __exceptthat __maybenull)
#define __deref_opt_xcount(size)                                _SAL1_Source_(__deref_opt_xcount, (size),  __deref_xcount(size)                        __exceptthat __maybenull)
#define __deref_opt_in                                          _SAL1_Source_(__deref_opt_in, (),  __deref_in                                  __exceptthat __maybenull)
#define __deref_opt_in_ecount(size)                             _SAL1_Source_(__deref_opt_in_ecount, (size),  __deref_in_ecount(size)                     __exceptthat __maybenull)
#define __deref_opt_in_bcount(size)                             _SAL1_Source_(__deref_opt_in_bcount, (size),  __deref_in_bcount(size)                     __exceptthat __maybenull)
#define __deref_opt_in_xcount(size)                             _SAL1_Source_(__deref_opt_in_xcount, (size),  __deref_in_xcount(size)                     __exceptthat __maybenull)
#define __deref_opt_out_xcount(size)                            _SAL1_Source_(__deref_opt_out_xcount, (size),  __deref_out_xcount(size)                    __exceptthat __maybenull)
#define __deref_opt_out_xcount_part(size,length)                _SAL1_Source_(__deref_opt_out_xcount_part, (size,length),  __deref_out_xcount_part(size,length)        __exceptthat __maybenull)
#define __deref_opt_out_xcount_full(size)                       _SAL1_Source_(__deref_opt_out_xcount_full, (size),  __deref_out_xcount_full(size)               __exceptthat __maybenull)
#define __deref_opt_inout_xcount(size)                          _SAL1_Source_(__deref_opt_inout_xcount, (size),  __deref_inout_xcount(size)                  __exceptthat __maybenull)
#define __deref_opt_inout_xcount_part(size,length)              _SAL1_Source_(__deref_opt_inout_xcount_part, (size,length),  __deref_inout_xcount_part(size,length)      __exceptthat __maybenull)
#define __deref_opt_inout_xcount_full(size)                     _SAL1_Source_(__deref_opt_inout_xcount_full, (size),  __deref_inout_xcount_full(size)             __exceptthat __maybenull)
#define __deref_opt_xcount_opt(size)                            _SAL1_Source_(__deref_opt_xcount_opt, (size),  __deref_xcount_opt(size)                    __exceptthat __maybenull)
#define __deref_opt_in_opt                                      _SAL1_Source_(__deref_opt_in_opt, (),  __deref_in_opt                              __exceptthat __maybenull)
#define __deref_opt_in_ecount_opt(size)                         _SAL1_Source_(__deref_opt_in_ecount_opt, (size),  __deref_in_ecount_opt(size)                 __exceptthat __maybenull)
#define __deref_opt_in_bcount_opt(size)                         _SAL1_Source_(__deref_opt_in_bcount_opt, (size),  __deref_in_bcount_opt(size)                 __exceptthat __maybenull)
#define __deref_opt_in_xcount_opt(size)                         _SAL1_Source_(__deref_opt_in_xcount_opt, (size),  __deref_in_xcount_opt(size)                 __exceptthat __maybenull)
#define __deref_opt_out_xcount_opt(size)                        _SAL1_Source_(__deref_opt_out_xcount_opt, (size),  __deref_out_xcount_opt(size)                __exceptthat __maybenull)
#define __deref_opt_out_xcount_part_opt(size,length)            _SAL1_Source_(__deref_opt_out_xcount_part_opt, (size,length),  __deref_out_xcount_part_opt(size,length)    __exceptthat __maybenull)
#define __deref_opt_out_xcount_full_opt(size)                   _SAL1_Source_(__deref_opt_out_scount_full_opt, (size),  __deref_out_xcount_full_opt(size)           __exceptthat __maybenull)
#define __deref_opt_inout_xcount_opt(size)                      _SAL1_Source_(__deref_opt_inout_xcount_opt, (size),  __deref_inout_xcount_opt(size)              __exceptthat __maybenull)
#define __deref_opt_inout_xcount_part_opt(size,length)          _SAL1_Source_(__deref_inout_xcount_part_opt(size, (size,length), length)  __exceptthat __maybenull)
#define __deref_opt_inout_xcount_full_opt(size)                 _SAL1_Source_(__deref_opt_inout_scount_full_opt, (size),  __deref_inout_xcount_full_opt(size)         __exceptthat __maybenull)

#define __deref_in_ecount_iterator(size, incr)                  _SAL1_Source_(__deref_in_ecount_iterator, (size,incr),  __inout _Pre_ __deref __elem_readableTo(size) __deref_out_range(==, _Old_(*_Curr_) + incr))
#define __deref_out_ecount_iterator(size, incr)                 _SAL1_Source_(__deref_out_ecount_iterator, (size,incr),  __inout _Pre_ __deref __elem_writableTo(size) __deref_out_range(==, _Old_(*_Curr_) + incr))
#define __deref_inout_ecount_iterator(size, incr)               _SAL1_Source_(__deref_inout_ecount_iterator, (size,incr),  __inout _Pre_ __deref __elem_readableTo(size) _Pre_ __deref __elem_writableTo(size) __deref_out_range(==, _Old_(*_Curr_) + incr))

#define __post_bcount(size)                                     _SAL1_Source_(__post_bcount, (size),  _Post_ __byte_writableTo(size))
#define __post_ecount(size)                                     _SAL1_Source_(__post_ecount, (size),  _Post_ __elem_writableTo(size))

#define __deref_realloc_bcount(insize, outsize)                 _SAL1_Source_(__deref_realloc_bcount, (insize,outsize),  __inout _Pre_ __deref __byte_readableTo(insize) _Post_ __deref __byte_writableTo(outsize))

/* __in_ecount_or_z(c) specifies semantics like strncmp, where a string
 * parameter is either null terminated, or valid up to c elements.
 */
#define __in_ecount_or_z(c)            _SAL1_Source_(__in_ecount_or_z, (c),  _When_(_String_length_(_Curr_) < (c), __in_z) \
                                       _When_(_String_length_(_Curr_) >= (c), __in_ecount(c)))


/* Provide default definition to be overridden when needed */
#define __post_nullnullterminated

/* Must protect redfinitions of macros to workaround rc.exe issues. */
#ifndef RC_INVOKED // [

#undef __nullnullterminated
#define __nullnullterminated _SAL1_Source_(__nullnullterminated, (),  __inexpressible_readableTo("string terminated by two nulls") __nullterminated)

#undef __post_nullnullterminated
#define __post_nullnullterminated _SAL1_Source_(__post_nullnullterminated, (),  _Post_ __inexpressible_readableTo("string terminated by two nulls") _Post_ __nullterminated)

#endif // ]
#endif  //__SAL_H_FULL_VER <= 140050727 // ]

/************************************************************************
 New extensions to sal.h follow here.
*************************************************************************/

#if (_MSC_VER >= 1000) && !defined(__midl) && defined(_PREFAST_) // [

#define __file_parser(typ)                  _SAL_L_Source_(__file_parser, (typ),  _SA_annotes2(SAL_file_parser,"function",typ))
#define __file_parser_class(typ)            _SAL_L_Source_(__file_parser_class, (typ), _SA_annotes2(SAL_file_parser,"class",typ))
#define __file_parser_library(typ)          extern int _SAL_L_Source_(__file_parser_library, (typ), _SA_annotes2(SAL_file_parser, "library", typ)) __iSALFileParserLibrary##typ;
#define __source_code_content(typ)          extern int _SAL_L_Source_(__source_code_content, (typ),  _SA_annotes1(SAL_source_code_content, typ)) __iSAL_Source_Code_Content##typ;
#define __class_code_content(typ)           _SAL_L_Source_(__class_code_content, (typ),  _SA_annotes1(SAL_class_code_content, typ))
#define __analysis_assert(e)                __assume(e)
#define __analysis_hint(hint)               _SAL_L_Source_(__analysis_hint, (hint),  _SA_annotes1(SAL_analysisHint, hint))
// For "breakpoint": doesn't return as far as analysis is concerned.
#define __analysis_noreturn                 __declspec(noreturn)
/* Internal defintions */
#define __inner_data_source(src_raw)        _SAL_L_Source_(__inner_data_source, (src_raw),  _SA_annotes1(SAL_untrusted_data_source,src_raw))
#define __inner_this_data_source(src_raw)   _SAL_L_Source_(__inner_this_data_source, (src_raw),  _SA_annotes1(SAL_untrusted_data_source_this,src_raw))
#define __inner_out_validated(typ_raw)      _SAL_L_Source_(__inner_out_validated, (typ_raw),  _Post_ _SA_annotes1(SAL_validated,typ_raw))
#define __inner_this_out_validated(typ_raw) _SAL_L_Source_(__inner_this_out_validated, (typ_raw),  _SA_annotes1(SAL_validated_this,typ_raw))
#define __inner_assume_validated_dec        __inline __nothrow void __AssumeValidated(__inner_out_validated("BY_DESIGN") const void *p) {(void)p;}
#define __inner_assume_validated(p)         __AssumeValidated(p)
#define __inner_transfer(formal)            _SAL_L_Source_(__inner_transfer, (formal),  _SA_annotes1(SAL_transfer_adt_property_from,formal))
#define __inner_encoded                     _SAL_L_Source_(__inner_encoded, (),  _SA_annotes0(SAL_encoded))

#if defined(_MSC_EXTENSIONS) || defined(_PREFAST_) || defined(OACR) // [
#define __inner_adt_prop(adt,prop)               _SAL_L_Source_(__inner_adt_prop, (adt,prop),  _SA_annotes2(SAL_adt, adt,prop))
#define __inner_adt_add_prop(adt,prop)           _SAL_L_Source_(__inner_adt_add_prop, (adt,prop),  _SA_annotes2(SAL_add_adt_property,adt,prop))
#define __inner_adt_remove_prop(adt,prop)        _SAL_L_Source_(__inner_adt_remove_prop, (adt,prop),  _SA_annotes2(SAL_remove_adt_property,adt,prop))
#define __inner_adt_transfer_prop(arg)           _SAL_L_Source_(__inner_adt_trasnfer_prop, (arg),  _SA_annotes1(SAL_transfer_adt_property_from,arg))
#define __inner_adt_type_props(typ)              _SAL_L_Source_(__inner_adt_type_props, (typ), _SA_annotes1(SAL_post_type,typ))
#define __inner_volatile                         _SAL_L_Source_(__inner_volatile, (),  _SA_annotes0(SAL_volatile))
#define __inner_nonvolatile                      _SAL_L_Source_(__inner_nonvolatile, (),  _SA_annotes0(SAL_nonvolatile))
#define __inner_possibly_notnullterminated       _SAL_L_Source_(__inner_possibly_notnulltermiated, (),  _SA_annotes1(SAL_nullTerminated,__maybe))
#endif // ]

///////////////////////////////////////////////////////////////////////////////
//
// Low Level Memory Correctness annotations
// 
// These are the implementation details for the Memory Correctness annotations
// that support checking of kernel/user memory handling within kernel mode code.
//
///////////////////////////////////////////////////////////////////////////////

//
// The enum that defines the types of memory context that exist in the system.
//
enum _SAL_ExecutionContext { SAL_KernelMode, SAL_UserMode };

// 
// temporary hack: Globals that stand in for the globally available SAL-pseudo-variables
// _Current_execution_context_ and _Previous_execution_context_. NMM will need to be modified
// to offer these.
//
enum _SAL_ExecutionContext _Previous_execution_context_;
enum _SAL_ExecutionContext _Current_execution_context_;

//
// Annotation elements that support the memory origin concept.
// The "When" variant is a hack for now until NMM can parse _Previous_execution_context_
// and _Current_execution_context_
//
__ANNOTATION(SAL_MemoryOrigin(__In_impl_ enum _SAL_ExecutionContext);)
__ANNOTATION(SAL_MemoryOriginWhen(__In_impl_ enum _SAL_ExecutionContext, __In_impl_ enum _SAL_ExecutionContext);)

#define _Memory_origin_(context)    _SAL2_Source_(_Memory_origin_, (context), _SA_annotes1(SAL_MemoryOrigin, context))
#define _Memory_origin_when_(previousContext, context)   _SAL2_Source_(_Memory_origin_when_, (previousContext, context), _SA_annotes2(SAL_MemoryOriginWhen, previousContext, context))


// 
// Annotation elements that support the memory accessibility concept.
// The "When" variant is a hack for new until NMM can parse _Previous_execution_context_
// and _Current_execution_context_
//
__ANNOTATION(SAL_AccessibleTo(__In_impl_ enum _SAL_ExecutionContext, __In_impl_ __int64 count);)
__ANNOTATION(SAL_AccessibleToWhen(__In_impl_ enum _SAL_ExecutionContext, __In_impl_ enum _SAL_ExecutionContext, __In_impl_ __int64 count);)

#define _Accessible_bytes_(context, expr)    _SAL2_Source_(_Accessible_bytes_, (context, expr), _SA_annotes2(SAL_AccessibleTo, context, expr))
#define _Accessible_bytes_when_(previousContext, context, expr)   _SAL2_Source_(_Accessible_bytes_when_, (context, previousContext, expr), _SA_annotes3(SAL_AccessibleToWhen, context, previousContext, expr))
#define _Pre_accessible_bytes_(context, expr)      _Pre_ _Accessible_bytes_(context, expr)
#define _Pre_accessible_bytes_when_(context, previousContext, expr)   _Pre_ _Accessible_bytes_when_(context, previousContext, expr)


///////////////////////////////////////////////////////////////////////////////
//
// High Level Memory Correctness annotations
//
// These annotations form the expected interface for users to the Memory Correctness
// annotations that support checking of kernel/user memory handling within
// kernel mode code.
// 
///////////////////////////////////////////////////////////////////////////////
#define _User_            _Memory_origin_when_(SAL_UserMode, SAL_UserMode) _Pre_accessible_bytes_when_(SAL_UserMode, SAL_KernelMode, 0)
#define _User_on_(expr)   _When_((expr) != SAL_KernelMode, _User_always_)
#define _User_always_     _Memory_origin_(SAL_UserMode) _Pre_accessible_bytes_(SAL_KernelMode, 0)
#define _User_always_and_needs_probe_on_(mode)  _User_always_ _Pre_accessible_bytes_when_(SAL_UserMode, SAL_KernelMode, 0) _Pre_satisfies_(mode == _Previous_execution_context_)
#define _Kernel_entry_              _SAL2_Source_(_Kernel_entry_, (), __inner_control_entrypoint(UserToKernel))
#define _Kernel_entry_always_       _SAL2_Source_(_Kernel_entry_, (), __inner_control_entrypoint(UserToKernel)) _Pre_satisfies_(_Previous_execution_context_ == SAL_UserMode)


#else // ][

#define __file_parser(typ)
#define __file_parser_class(typ)
#define __file_parser_library(typ)
#define __source_code_content(typ)
#define __class_code_content(typ)
#define __analysis_assert(e)
#define __analysis_hint(hint)
#define __analysis_noreturn
/* Internal defintions */
#define __inner_data_source(src_raw)
#define __inner_this_data_source(src_raw)
#define __inner_out_validated(typ_raw)
#define __inner_this_out_validated(typ_raw)
#define __inner_assume_validated_dec
#define __inner_assume_validated(p)
#define __inner_transfer(formal)
#define __inner_encoded
#define __inner_adt_prop(adt,prop)
#define __inner_adt_add_prop(adt,prop)
#define __inner_adt_remove_prop(adt,prop)
#define __inner_adt_transfer_prop(arg)
#define __inner_adt_type_props(typ)
#define __inner_volatile
#define __inner_nonvolatile
#define __inner_possibly_notnullterminated

///////////////////////////////////////////////////////////////////////////////
//
// Low Level Memory Correctness annotations
// 
// These are the implementation details for the Memory Correctness annotations
// that support checking of kernel/user memory handling within kernel mode code.
//
///////////////////////////////////////////////////////////////////////////////
#define _Memory_origin_(context)
#define _Memory_origin_when_(previousContext, context)
#define _Accessible_bytes_(context, expr)    
#define _Accessible_bytes_when_(previousContext, context, expr)
#define _Pre_accessible_bytes_(context, expr)
#define _Pre_accessible_bytes_when_(context, previousContext, expr)   


///////////////////////////////////////////////////////////////////////////////
//
// High Level Memory Correctness annotations
//
// These annotations form the expected interface for users to the Memory Correctness
// annotations that support checking of kernel/user memory handling within
// kernel mode code.
// 
///////////////////////////////////////////////////////////////////////////////
#define _User_           
#define _User_on_(expr)  
#define _User_always_    
#define _User_always_and_needs_probe_on_(mode)
#define _Kernel_entry_              
#define _Kernel_entry_always_

#endif // ] // #if (_MSC_VER >= 1000) && !defined(__midl) && defined(_PREFAST_)

#define __field_ecount(size)                _SAL1_Source_(__field_ecount, (size),  __notnull __elem_writableTo(size))
#define __field_bcount(size)                _SAL1_Source_(__field_bcount, (size),  __notnull __byte_writableTo(size))
#define __field_xcount(size)                _SAL1_Source_(__field_xcount, (size),  __notnull __inexpressible_writableTo(size))

#define __field_ecount_opt(size)            _SAL1_Source_(__field_ecount_opt, (size),  __maybenull __elem_writableTo(size))
#define __field_bcount_opt(size)            _SAL1_Source_(__field_bcount_opt, (size),  __maybenull __byte_writableTo(size))
#define __field_xcount_opt(size)            _SAL1_Source_(__field_xcount_opt, (size),  __maybenull __inexpressible_writableTo(size))

#define __field_ecount_part(size,init)      _SAL1_Source_(__field_ecount_part, (size,init),  __notnull __elem_writableTo(size) __elem_readableTo(init))
#define __field_bcount_part(size,init)      _SAL1_Source_(__field_bcount_part, (size,init),  __notnull __byte_writableTo(size) __byte_readableTo(init))
#define __field_xcount_part(size,init)      _SAL1_Source_(__field_xcount_part, (size,init),  __notnull __inexpressible_writableTo(size) __inexpressible_readableTo(init))

#define __field_ecount_part_opt(size,init)  _SAL1_Source_(__field_ecount_part_opt, (size,init),  __maybenull __elem_writableTo(size) __elem_readableTo(init))
#define __field_bcount_part_opt(size,init)  _SAL1_Source_(__field_bcount_part_opt, (size,init), __maybenull __byte_writableTo(size) __byte_readableTo(init))
#define __field_xcount_part_opt(size,init)  _SAL1_Source_(__field_xcount_part_opt, (size,init),  __maybenull __inexpressible_writableTo(size) __inexpressible_readableTo(init))

#define __field_ecount_full(size)           _SAL1_Source_(__field_ecount_full, (size),  __field_ecount_part(size,size))
#define __field_bcount_full(size)           _SAL1_Source_(__field_bcount_full, (size),  __field_bcount_part(size,size))
#define __field_xcount_full(size)           _SAL1_Source_(__field_xcount_full, (size),  __field_xcount_part(size,size))

#define __field_ecount_full_opt(size)       _SAL1_Source_(__field_ecount_full_opt, (size),  __field_ecount_part_opt(size,size))
#define __field_bcount_full_opt(size)       _SAL1_Source_(__field_bcount_full_opt, (size), __field_bcount_part_opt(size,size))
#define __field_xcount_full_opt(size)       _SAL1_Source_(__field_xcount_full_opt, (size),  __field_xcount_part_opt(size,size))

#define __field_nullterminated              _SAL1_Source_(__field_nullterminated, (),  __nullterminated)

#define __struct_bcount(size)               _SAL1_Source_(__struct_bcount, (size),  __byte_writableTo(size))
#define __struct_xcount(size)               _SAL1_Source_(__struct_xcount, (size),  __inexpressible_writableTo(size))

#define __out_awcount(expr,size)            _SAL1_Source_(__out_awcount, (expr,size),  _Pre_ __notnull \
					    __byte_writableTo((expr) ? (size) : (size) * 2) \
                                            _Post_ __valid __refparam)
#define __in_awcount(expr,size)             _SAL1_Source_(__in_awcount, (expr,size),  _Pre_ __valid \
                                            _Pre_ _Notref_ __deref __readonly \
				            __byte_readableTo((expr) ? (size) : (size) * 2))
#define __post_invalid                      _SAL1_Source_(__post_invalid, (),  _Post_ __notvalid)
/* integer related macros */
#define __allocator                         _SAL_L_Source_(__allocator, (), __inner_allocator)
#define __deallocate(kind)                  _SAL_L_Source_(__deallocate, (kind), _Pre_ __notnull __post_invalid)
#define __deallocate_opt(kind)              _SAL_L_Source_(__deallocate_opt, (kind), _Pre_ __maybenull __post_invalid)
#define __bound                             _SAL_L_Source_(__bound, (), __inner_bound)
#define __range(lb,ub)                      _SAL_L_Source_(__range, (lb,ub), __inner_range(lb,ub))
#define __in_bound                          _SAL_L_Source_(__in_bound, (), _Pre_ __inner_bound)
#define __out_bound                         _SAL_L_Source_(__out_bound, (), _Post_ __inner_bound)
#define __deref_out_bound                   _SAL_L_Source_(__deref_out_bound, (), _Post_ __deref __inner_bound)
#define __in_range(lb,ub)                   _SAL_L_Source_(__in_range, (lb,ub), _Pre_ __inner_range(lb,ub))
#define __out_range(lb,ub)                  _SAL_L_Source_(__out_range, (lb,ub), _Post_ __inner_range(lb,ub))
#define __deref_in_range(lb,ub)             _SAL_L_Source_(__deref_in_range, (lb,ub), _Pre_ __deref __inner_range(lb,ub))
#define __deref_out_range(lb,ub)            _SAL_L_Source_(__deref_out_range, (lb,ub), _Post_ __deref __inner_range(lb,ub))
#define __deref_inout_range(lb,ub)          _SAL_L_Source_(__deref_inout_range, (lb,ub), __deref_in_range(lb,ub) __deref_out_range(lb,ub))
#define __field_range(lb,ub)                _SAL_L_Source_(__field_range, (lb,ub), __range(lb,ub))
#define __field_data_source(src_sym)        _SAL_L_Source_(__field_data_source, (lb,ub), __inner_data_source(#src_sym))

#define __range_max(a,b)                    _SAL_L_Source_(__range_max, (a,b), __range(==, a > b ? a : b))
#define __range_min(a,b)                    _SAL_L_Source_(__range_min, (a,b), __range(==, a < b ? a : b))


/* Penetration review macros */
#define __in_data_source(src_sym)           _SAL_L_Source_(__in_data_source, (src_sym),  _Pre_ __inner_data_source(#src_sym))
#define __out_data_source(src_sym)          _SAL_L_Source_(__out_data_source, (src_sym),  _Post_ __inner_data_source(#src_sym))
#define __out_validated(typ_sym)            _SAL_L_Source_(__out_validated, (src_sym),  __inner_out_validated(#typ_sym))
#define __this_out_data_source(src_sym)     _SAL_L_Source_(__this_out_data_source, (src_sym),  __inner_this_data_source(#src_sym))
#define __this_out_validated(typ_sym)       _SAL_L_Source_(__this_out_validated, (src_sym),  __inner_this_out_validated(#typ_sym))
#define __transfer(formal)                  _SAL_L_Source_(__transfer, (src_sym),  _Post_ __inner_transfer(formal))
#define __rpc_entry                         _SAL_L_Source_(__rpc_entry, (formal),  __inner_control_entrypoint(RPC))
#define __kernel_entry                      _SAL_L_Source_(__kernel_entry, (),  __inner_control_entrypoint(UserToKernel))
#define __gdi_entry                         _SAL_L_Source_(__gdi_entry, (),  __inner_control_entrypoint(GDI))
#define __encoded_pointer                   _SAL_L_Source_(__encoded_pointer, (),  __inner_encoded)
#define __encoded_array                     _SAL_L_Source_(__encoded_array, (),  __inner_encoded)
#define __field_encoded_pointer             _SAL_L_Source_(__field_encoded_pointer, (),  __inner_encoded)
#define __field_encoded_array               _SAL_L_Source_(__field_encoded_array, (),  __inner_encoded)
#if defined(_MSC_EXTENSIONS) || defined(_PREFAST_) || defined(OACR) // [
#define __type_has_adt_prop(adt,prop)       _SAL_L_Source_(__type_has_adt_prop, (adt,prop),  __inner_adt_prop(adt,prop))
#define __out_has_adt_prop(adt,prop)        _SAL_L_Source_(__out_has_adt_prop, (adt,prop),  _Post_ __inner_adt_add_prop(adt,prop))
#define __out_not_has_adt_prop(adt,prop)    _SAL_L_Source_(__out_not_has_adt_prop, (adt,prop),  _Post_ __inner_adt_remove_prop(adt,prop))
#define __out_transfer_adt_prop(arg)        _SAL_L_Source_(__out_transfer_adt_prop, (arg),  _Post_ __inner_adt_transfer_prop(arg))
#define __out_has_type_adt_props(typ)       _SAL_L_Source_(__out_has_type_adt_props, (typ),  _Post_ __inner_adt_type_props(typ))

/* useful PFD related macros */
#define __possibly_notnullterminated        _SAL_L_Source_(__possibly_notnullterminated, (),  __inner_possibly_notnullterminated)

/* Windows Internal */
#define __volatile                          _SAL_L_Source_(__volatile, (),  __inner_volatile)
#define __nonvolatile                       _SAL_L_Source_(__nonvolatile, (),  __inner_nonvolatile)
#else
#define __out_has_type_adt_props(typ)       /* nothing */
#endif
#define __deref_volatile                    _SAL_L_Source_(__deref_volatile, (),  __deref __volatile)
#define __deref_nonvolatile                 _SAL_L_Source_(__deref_nonvolatile, (),  __deref __nonvolatile)

/* declare stub functions for macros */
__inner_assume_validated_dec
__inner_assume_bound_dec
#define __analysis_assume_nullterminated(x) _Analysis_assume_nullterminated_(x)
#define __assume_validated(p) __inner_assume_validated(p)
#define __assume_bound(i) __inner_assume_bound(i)


/**************************************************************************
* SAL 2 extensions for Windows-specific APIs.
***************************************************************************/

// Annotation for parameters that are not used in any way by the function.
// Unlike _Reserved_, an _Unreferenced_parameter_ pointer need not be NULL.
#ifndef _Unreferenced_parameter_
#define _Unreferenced_parameter_  _SAL2_Source_(_Unreferenced_parameter_, (), _Const_)
#endif

// Pointer parameters that are freed by the function, and thus the pointed-to
// memory should not be used after return.
#ifndef _Frees_ptr_
#define _Frees_ptr_               _SAL_L_Source_(_Frees_ptr_, (),  _Pre_notnull_ _Post_ptr_invalid_ __drv_freesMem(Mem))
#endif
#ifndef _Frees_ptr_opt_
#define _Frees_ptr_opt_           _SAL_L_Source_(_Frees_ptr_opt_, (),  _Pre_maybenull_ _Post_ptr_invalid_ __drv_freesMem(Mem))
#endif


// Functions which behave like realloc in that they may succeed by
// freeing one block of memory and allocating another, or may fail, in
// which case the original block is still valid, or may be used to
// free memory directly by requesting that the new block is of size
// zero.  We do not say here whether the before may be null, we leave
// that to the annotation on the argument
#define _Reallocation_function_(after, before, size) \
       _Success_((after) != NULL || (size) == 0) \
       _At_((after), _Post_maybenull_ _Post_writable_byte_size_(size) \
            _When_(((before) == NULL || (size) > 0), _Must_inspect_result_)) \
       _At_((before), _Post_ptr_invalid_ __drv_freesMem(Mem))

#define _Ret_reallocated_bytes_(before, size) \
       _Reallocation_function_(_Curr_, before, size)


// NLS APIs allow strings to be specified either by an element count or
// null termination. Unlike _In_reads_or_z_, this is not whichever comes
// first, but based on whether the size is negative or not.
#define _In_NLS_string_(size)     _SAL_L_Source_(_In_NLS_string_, (size),  _When_((size) < 0,  _In_z_)           \
                                  _When_((size) >= 0, _In_reads_(size)))


// Minifilter CompletionContext parameters on the pre-operation callback
// default to NULL.  For return type FLT_PREOP_SUCCESS_WITH_CALLBACK or
// FLT_PREOP_SYNCHRONIZE, it may be set to NULL or a valid pointer.  For all
// other returns, it must be NULL.
#define _Flt_CompletionContext_Outptr_   \
           _SAL_L_Source_(_Flt_CompletionContext_Outptr_, (),  _Outptr_result_maybenull_ _Pre_valid_ \
           _At_(*_Curr_, _Pre_null_ \
               _When_(return != FLT_PREOP_SUCCESS_WITH_CALLBACK && return != FLT_PREOP_SYNCHRONIZE, _Post_null_)))

// Minifilter ConnectionCookie parameters on the port connect notify callback
// default to NULL.  On successful return, it may be set to NULL or non-NULL,
// but it must be NULL on failure.
#define _Flt_ConnectionCookie_Outptr_      \
     _SAL_L_Source_(_Flt_ConnectionCookie_Outptr_, (),  _Outptr_result_maybenull_ _Pre_valid_ \
     _At_(*_Curr_, _Pre_null_ _On_failure_(_Post_null_)))


//
// A common pattern is to pass an "_Inout_ PCHAR* ppBuf" of size "_Inout_ DWORD* pSize"
// to a function that writes to **pBuf, incrementing *ppBuf to point to one
// past the last written byte. Thus the length of the write is
// (*ppBuf - Old(*ppBuf)). The size of the remaining unwritten capacity
// is written to *pSize.
//
// This pattern is frequently used when progressively filling a
// large buffer in chunks
// (e.g. when reading from a network interface in a driver).
//
// It is expected that these supplementary annotations would be used inside an
// _At_, like so:
//
// _At_(*ppBuf, _Writes_and_advances_ptr_(*pBufSize))
// HRESULT WriteChunkOfData(_Inout_ PCHAR* ppBuf, _Inout_ DWORD* pBufSize);
//
#ifndef _Writes_and_advances_ptr_ // [
#define _Writes_and_advances_ptr_(size) \
                                _SAL2_Source_(_Writes_and_advances_ptr_, (size),  _At_((void*)_Curr_, _Inout_) \
                                _At_(_Curr_, \
                                    _Pre_writable_size_(size) \
                                    _Post_writable_size_(size) \
                                    _Post_satisfies_(_Curr_ - _Old_(_Curr_) == _Old_(size) - size)) \
                                _At_(_Old_(_Curr_), \
                                    _Post_readable_size_(_Old_(size) - size)))
#endif // ]

#ifndef _Writes_bytes_and_advances_ptr_ // [
#define _Writes_bytes_and_advances_ptr_(size) \
                                _SAL2_Source_(_Writes_bytes_and_advances_ptr, (size),  _At_((void*)_Curr_, _Inout_) \
                                _At_(_Curr_, \
                                    _Pre_writable_byte_size_(size) \
                                    _Post_writable_byte_size_(size) \
                                    _Post_satisfies_(((char*)_Curr_) - ((char*)_Old_(_Curr_)) == _Old_(size) - size)) \
                                _At_(_Old_(_Curr_), \
                                    _Post_readable_byte_size_(_Old_(size) - size)))
#endif // ]

//
// Gets the current error code (as returned by GetLastError()), and stores
// in _Curr_ as a postcondition. This is currently approximated by assuming
// that GetLastError() always returns a failed error code. This is not a
// completely accurate approximation, but reasonable.
//
#define _Post_equals_last_error_     _SAL2_Source_(_Post_equals_last_error_, (),  _Post_satisfies_(_Curr_ != 0))

//
// Indicates the function simply translates the given Win32 error code into an HRESULT
// with broadly the same semantics as HRESULT_FROM_WIN32().
//
// This convenience macro allows analyzers to understand the many bespoke error-translation
// functions as simple translators that can be treated as equivalent to HRESULT_FROM_WIN32().
// This results in fewer false positives and unnecessary path explorations. This is because
// for analysis, the specific translations rarely matter, it is more important to know the
// that function cannot fail and the conditions under which it will return an error-range
// HRESULT and when a success-range HRESULT.
//
#ifndef _Translates_Win32_to_HRESULT_ // [

#define _Translates_Win32_to_HRESULT_(errorCode)   \
                                        _SAL2_Source_(_Translates_Win32_to_HRESULT_, (errorCode), \
                                        _Always_( \
                                        _When_((HRESULT)errorCode <= 0, \
                                          _At_(_Curr_, _Post_equal_to_((HRESULT)errorCode))) \
                                        _When_((HRESULT)errorCode > 0, \
                                          _At_(_Curr_, _Post_equal_to_((HRESULT)((errorCode & 0x0000FFFF) | (FACILITY_WIN32 << 16) | 0x80000000))))))

#endif // ]

//
// Indicates the function just directly translates the given NTSTATUS error code into an HRESULT.
//
// This convenience macro allows analyzers to understand the many bespoke error-translation
// functions as simple translators that can be treated as equivalent to HRESULT_FROM_NT().
// This results in fewer false positives and unnecessary path explorations. This is because
// for analysis, the specific translations rarely matter.
//
#ifndef _Translates_NTSTATUS_to_HRESULT_    // [

#define _Translates_NTSTATUS_to_HRESULT_(status)    \
                                        _SAL2_Source_(_Translates_NTSTATUS_to_HRESULT_, (status), \
                                        _Always_( \
                                        _Post_equal_to_((HRESULT)(status | FACILITY_NT_BIT))))

#endif // ]

//
// Indicates the funtion just translates the result of GetLastError() into an HRESULT.
//
// This convenience macro allows analyzers to understand the many bespoke GetLastError-translation
// functions as simple translators that will always return a failure-value HRESULT (these functions
// are almost never meant to be called when GetLastError() could return 0).
// This results in fewer false positives and unnecessary path explorations. This is because
// for analysis, the specific translations rarely matter.
//
#ifndef _Translates_last_error_to_HRESULT_ // [

#define _Translates_last_error_to_HRESULT_          \
                                        _SAL2_Source_(_Translates_last_error_to_HRESULT_, (), \
                                        _Always_( \
                                        _Post_satisfies_(_Curr_ < 0)))

#endif // ]

#ifdef  __cplusplus
}
#endif

#ifdef _PREFIX_ // [
/**************************************************************************
* Defintion of __pfx_assume and __pfx_assert. Thse should be the only
* defintions of these functions.
***************************************************************************/
#if __cplusplus // [
extern "C" void __pfx_assert(bool, const char *);
extern "C" void __pfx_assume(bool, const char *);
#else // ][
void __pfx_assert(int, const char *);
void __pfx_assume(int, const char *);
#endif // ]
/**************************************************************************
* Redefintion of __analysis_assume and __analysis_assert for PREFIX build
**************************************************************************/
#undef  __analysis_assume
#undef  __analysis_assert
#define __analysis_assume(e) (__pfx_assume(e,"pfx_assume"),__assume(e));
#define __analysis_assert(e) (__pfx_assert(e,"pfx_assert"),__assume(e));
#endif /* ifdef _PREFIX_ */ // ]

/**************************************************************************
* This include should always be the last thing in this file.
* Must avoid redfinitions of macros to workaround rc.exe issues.
***************************************************************************/
#if !(defined(RC_INVOKED) || defined(SORTPP_PASS))
#include <specstrings_strict.h>
#endif /* if !(defined(RC_INVOKED) || defined(SORTPP_PASS)) */

/*
 If no SAL 2 appears to have been defined (_Outptr_ is a representative choice)
 then we must be operating in a downlevel build environment (such as VS10).
 We also test against the compiler version to identify a downlevel environment,
 as VS11 is the minimum required for SAL 2 support.

 If we are operating in a downlevel build environment (such as VS10)
 we need to undefine the following symbols before including driverspecs.h
 or we will end up referencing SAL 2 implementation symbols and cause
 build failures.
*/
#if (!defined(_Outptr_) || _MSC_FULL_VER <= 160000000) && !( defined( MIDL_PASS ) || defined(__midl) || defined(RC_INVOKED) ) /*IFSTRIP=IGN*/ // [
#undef __ANNOTATION
#define __ANNOTATION(fun) /* fun */
#undef __PRIMOP
#define __PRIMOP(type, fun)
#endif /* !defined(_Outptr_) || _MSC_VER <= 1600 */ // ]

#include <driverspecs.h>

/*
 If no SAL 2 appears to have been defined (_Outptr_ is a representative choice)
 then we must be operating in a downlevel build environment (such as VS10).
 We also test against the compiler version to identify a downlevel environment,
 as VS11 is the minimum required for SAL 2 support.

 If we are in a downlevel environment, we can go ahead and include no_sal2.h
 to make all of SAL 2 no-ops to ensure no build failures.
*/
#if (!defined(_Outptr_) || _MSC_FULL_VER <= 160000000) && !( defined( MIDL_PASS ) || defined(__midl) || defined(RC_INVOKED) ) && !( defined( _SDV_ ) ) /*IFSTRIP=IGN*/ // [
#include <no_sal2.h>
#endif /* !defined(_Outptr_) || _MSC_VER <= 1600 */ // ]

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif /* #ifndef SPECSTRINGS_H */  // ]

