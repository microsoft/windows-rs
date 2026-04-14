// iso646.h standard header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _ISO646
#define _ISO646

#if defined(RC_INVOKED) || defined(Q_MOC_RUN) || defined(__midl)
// do nothing, see _STL_COMPILER_PREPROCESSOR in yvals_core.h
#else // ^^^ non-compiler tools / C and C++ compilers vvv

#if !defined(__cplusplus) || defined(_MSC_EXTENSIONS)
#define and    &&
#define and_eq &=
#define bitand &
#define bitor  |
#define compl  ~
#define not    !
#define not_eq !=
#define or     ||
#define or_eq  |=
#define xor    ^
#define xor_eq ^=
#endif // !defined(__cplusplus) || defined(_MSC_EXTENSIONS)

#endif // ^^^ C and C++ compilers ^^^
#endif // _ISO646
