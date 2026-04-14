// xkeycheck.h internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XKEYCHECK_H
#define _XKEYCHECK_H

// xkeycheck.h assumes that it's being included by yvals_core.h in a specific order.
// Nothing else should include xkeycheck.h.

#if _STL_COMPILER_PREPROCESSOR

#if !defined(_ALLOW_KEYWORD_MACROS) && !defined(__INTELLISENSE__)

// #if defined($KEYWORD)
// #define $KEYWORD EMIT WARNING C4005
// #error The C++ Standard Library forbids macroizing the keyword "$KEYWORD". \
// Enable warning C4005 to find the forbidden define.
// #endif // $KEYWORD

// *don't* check the "alternative token representations"

// keywords:
#if defined(alignas)
#define alignas EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "alignas". \
Enable warning C4005 to find the forbidden define.
#endif // alignas

#if defined(alignof)
#define alignof EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "alignof". \
Enable warning C4005 to find the forbidden define.
#endif // alignof

#if defined(asm)
#define asm EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "asm". \
Enable warning C4005 to find the forbidden define.
#endif // asm

#if defined(auto)
#define auto EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "auto". \
Enable warning C4005 to find the forbidden define.
#endif // auto

#if defined(bool)
#define bool EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "bool". \
Enable warning C4005 to find the forbidden define.
#endif // bool

#if defined(break)
#define break EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "break". \
Enable warning C4005 to find the forbidden define.
#endif // break

#if defined(case)
#define case EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "case". \
Enable warning C4005 to find the forbidden define.
#endif // case

#if defined(catch)
#define catch EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "catch". \
Enable warning C4005 to find the forbidden define.
#endif // catch

#if defined(char)
#define char EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "char". \
Enable warning C4005 to find the forbidden define.
#endif // char

#if defined(char8_t) && _HAS_CXX20
#define char8_t EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "char8_t". \
Enable warning C4005 to find the forbidden define.
#endif // char8_t

#if defined(char16_t)
#define char16_t EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "char16_t". \
Enable warning C4005 to find the forbidden define.
#endif // char16_t

#if defined(char32_t)
#define char32_t EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "char32_t". \
Enable warning C4005 to find the forbidden define.
#endif // char32_t

#if defined(class)
#define class EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "class". \
Enable warning C4005 to find the forbidden define.
#endif // class

#if defined(concept) && _HAS_CXX20
#define concept EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "concept". \
Enable warning C4005 to find the forbidden define.
#endif // concept

#if defined(const)
#define const EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "const". \
Enable warning C4005 to find the forbidden define.
#endif // const

#if defined(consteval) && _HAS_CXX20
#define consteval EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "consteval". \
Enable warning C4005 to find the forbidden define.
#endif // consteval

#if defined(constexpr)
#define constexpr EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "constexpr". \
Enable warning C4005 to find the forbidden define.
#endif // constexpr

#if defined(constinit) && _HAS_CXX20
#define constinit EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "constinit". \
Enable warning C4005 to find the forbidden define.
#endif // constinit

#if defined(const_cast)
#define const_cast EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "const_cast". \
Enable warning C4005 to find the forbidden define.
#endif // const_cast

#if defined(continue)
#define continue EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "continue". \
Enable warning C4005 to find the forbidden define.
#endif // continue

#if defined(co_await) && _HAS_CXX20
#define co_await EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "co_await". \
Enable warning C4005 to find the forbidden define.
#endif // co_await

#if defined(co_return) && _HAS_CXX20
#define co_return EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "co_return". \
Enable warning C4005 to find the forbidden define.
#endif // co_return

#if defined(co_yield) && _HAS_CXX20
#define co_yield EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "co_yield". \
Enable warning C4005 to find the forbidden define.
#endif // co_yield

#if defined(decltype)
#define decltype EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "decltype". \
Enable warning C4005 to find the forbidden define.
#endif // decltype

#if defined(default)
#define default EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "default". \
Enable warning C4005 to find the forbidden define.
#endif // default

#if defined(delete)
#define delete EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "delete". \
Enable warning C4005 to find the forbidden define.
#endif // delete

#if defined(do)
#define do EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "do". \
Enable warning C4005 to find the forbidden define.
#endif // do

#if defined(double)
#define double EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "double". \
Enable warning C4005 to find the forbidden define.
#endif // double

#if defined(dynamic_cast)
#define dynamic_cast EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "dynamic_cast". \
Enable warning C4005 to find the forbidden define.
#endif // dynamic_cast

#if defined(else)
#define else EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "else". \
Enable warning C4005 to find the forbidden define.
#endif // else

#if defined(enum)
#define enum EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "enum". \
Enable warning C4005 to find the forbidden define.
#endif // enum

#if defined(explicit)
#define explicit EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "explicit". \
Enable warning C4005 to find the forbidden define.
#endif // explicit

#if defined(export)
#define export EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "export". \
Enable warning C4005 to find the forbidden define.
#endif // export

#if defined(extern)
#define extern EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "extern". \
Enable warning C4005 to find the forbidden define.
#endif // extern

#if defined(false)
#define false EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "false". \
Enable warning C4005 to find the forbidden define.
#endif // false

#if defined(float)
#define float EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "float". \
Enable warning C4005 to find the forbidden define.
#endif // float

#if defined(for)
#define for EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "for". \
Enable warning C4005 to find the forbidden define.
#endif // for

#if defined(friend)
#define friend EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "friend". \
Enable warning C4005 to find the forbidden define.
#endif // friend

#if defined(goto)
#define goto EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "goto". \
Enable warning C4005 to find the forbidden define.
#endif // goto

#if defined(if)
#define if EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "if". \
Enable warning C4005 to find the forbidden define.
#endif // if

#if defined(inline)
#define inline EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "inline". \
Enable warning C4005 to find the forbidden define.
#endif // inline

#if defined(int)
#define int EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "int". \
Enable warning C4005 to find the forbidden define.
#endif // int

#if defined(long)
#define long EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "long". \
Enable warning C4005 to find the forbidden define.
#endif // long

#if defined(mutable)
#define mutable EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "mutable". \
Enable warning C4005 to find the forbidden define.
#endif // mutable

#if defined(namespace)
#define namespace EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "namespace". \
Enable warning C4005 to find the forbidden define.
#endif // namespace

#if defined(new) && defined(_ENFORCE_BAN_OF_MACRO_NEW)
#define new EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "new", though macroized new is supported on this \
implementation as a nonstandard extension. Enable warning C4005 to find the forbidden define, or re-enable the \
extension by removing _ENFORCE_BAN_OF_MACRO_NEW.
#endif // new

#if defined(noexcept)
#define noexcept EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "noexcept". \
Enable warning C4005 to find the forbidden define.
#endif // noexcept

#if defined(nullptr)
#define nullptr EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "nullptr". \
Enable warning C4005 to find the forbidden define.
#endif // nullptr

#if defined(operator)
#define operator EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "operator". \
Enable warning C4005 to find the forbidden define.
#endif // operator

#if defined(private)
#define private EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "private". \
Enable warning C4005 to find the forbidden define.
#endif // private

#if defined(protected)
#define protected EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "protected". \
Enable warning C4005 to find the forbidden define.
#endif // protected

#if defined(public)
#define public EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "public". \
Enable warning C4005 to find the forbidden define.
#endif // public

#if defined(register)
#define register EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "register". \
Enable warning C4005 to find the forbidden define.
#endif // register

#if defined(reinterpret_cast)
#define reinterpret_cast EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "reinterpret_cast". \
Enable warning C4005 to find the forbidden define.
#endif // reinterpret_cast

#if defined(requires) && _HAS_CXX20
#define requires EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "requires". \
Enable warning C4005 to find the forbidden define.
#endif // requires

#if defined(return)
#define return EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "return". \
Enable warning C4005 to find the forbidden define.
#endif // return

#if defined(short)
#define short EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "short". \
Enable warning C4005 to find the forbidden define.
#endif // short

#if defined(signed)
#define signed EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "signed". \
Enable warning C4005 to find the forbidden define.
#endif // signed

#if defined(sizeof)
#define sizeof EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "sizeof". \
Enable warning C4005 to find the forbidden define.
#endif // sizeof

#if defined(static)
#define static EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "static". \
Enable warning C4005 to find the forbidden define.
#endif // static

#if defined(static_assert)
#define static_assert EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "static_assert". \
Enable warning C4005 to find the forbidden define.
#endif // static_assert

#if defined(static_cast)
#define static_cast EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "static_cast". \
Enable warning C4005 to find the forbidden define.
#endif // static_cast

#if defined(struct)
#define struct EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "struct". \
Enable warning C4005 to find the forbidden define.
#endif // struct

#if defined(switch)
#define switch EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "switch". \
Enable warning C4005 to find the forbidden define.
#endif // switch

#if defined(template)
#define template EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "template". \
Enable warning C4005 to find the forbidden define.
#endif // template

#if defined(this)
#define this EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "this". \
Enable warning C4005 to find the forbidden define.
#endif // this

#if defined(thread_local)
#define thread_local EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "thread_local". \
Enable warning C4005 to find the forbidden define.
#endif // thread_local

#if defined(throw)
#define throw EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "throw". \
Enable warning C4005 to find the forbidden define.
#endif // throw

#if defined(true)
#define true EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "true". \
Enable warning C4005 to find the forbidden define.
#endif // true

#if defined(try)
#define try EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "try". \
Enable warning C4005 to find the forbidden define.
#endif // try

#if defined(typedef)
#define typedef EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "typedef". \
Enable warning C4005 to find the forbidden define.
#endif // typedef

#if defined(typeid)
#define typeid EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "typeid". \
Enable warning C4005 to find the forbidden define.
#endif // typeid

#if defined(typename)
#define typename EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "typename". \
Enable warning C4005 to find the forbidden define.
#endif // typename

#if defined(union)
#define union EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "union". \
Enable warning C4005 to find the forbidden define.
#endif // union

#if defined(unsigned)
#define unsigned EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "unsigned". \
Enable warning C4005 to find the forbidden define.
#endif // unsigned

#if defined(using)
#define using EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "using". \
Enable warning C4005 to find the forbidden define.
#endif // using

#if defined(virtual)
#define virtual EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "virtual". \
Enable warning C4005 to find the forbidden define.
#endif // virtual

#if defined(void)
#define void EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "void". \
Enable warning C4005 to find the forbidden define.
#endif // void

#if defined(volatile)
#define volatile EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "volatile". \
Enable warning C4005 to find the forbidden define.
#endif // volatile

#if defined(wchar_t)
#define wchar_t EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "wchar_t". \
Enable warning C4005 to find the forbidden define.
#endif // wchar_t

#if defined(while)
#define while EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the keyword "while". \
Enable warning C4005 to find the forbidden define.
#endif // while

// contextual keywords (a.k.a. "identifiers with special meaning"):
#if defined(final)
#define final EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the contextual keyword "final". \
Enable warning C4005 to find the forbidden define.
#endif // final

#if defined(import) && _HAS_CXX20
#define import EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the contextual keyword "import". \
Enable warning C4005 to find the forbidden define.
#endif // import

#if defined(module) && _HAS_CXX20
#define module EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the contextual keyword "module". \
Enable warning C4005 to find the forbidden define.
#endif // module

#if defined(override)
#define override EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the contextual keyword "override". \
Enable warning C4005 to find the forbidden define.
#endif // override

// attribute-tokens:
#if defined(carries_dependency)
#define carries_dependency EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "carries_dependency". \
Enable warning C4005 to find the forbidden define.
#endif // carries_dependency

#if defined(deprecated)
#define deprecated EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "deprecated". \
Enable warning C4005 to find the forbidden define.
#endif // deprecated

#if defined(fallthrough) && _HAS_CXX17
#define fallthrough EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "fallthrough". \
Enable warning C4005 to find the forbidden define.
#endif // fallthrough

// not checking "likely" because it is commonly defined as a function-like macro

#if defined(maybe_unused) && _HAS_CXX17
#define maybe_unused EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "maybe_unused". \
Enable warning C4005 to find the forbidden define.
#endif // maybe_unused

#if defined(nodiscard) // C++17 attribute-token, also enforced in C++14 mode
#define nodiscard EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "nodiscard". \
Enable warning C4005 to find the forbidden define.
#endif // nodiscard

#if defined(noreturn)
#define noreturn EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "noreturn". \
Enable warning C4005 to find the forbidden define.
#endif // noreturn

#if defined(no_unique_address) && _HAS_CXX20
#define no_unique_address EMIT WARNING C4005
#error The C++ Standard Library forbids macroizing the attribute-token "no_unique_address". \
Enable warning C4005 to find the forbidden define.
#endif // no_unique_address

// not checking "unlikely" because it is commonly defined as a function-like macro

#endif // !defined(_ALLOW_KEYWORD_MACROS) && !defined(__INTELLISENSE__)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XKEYCHECK_H
