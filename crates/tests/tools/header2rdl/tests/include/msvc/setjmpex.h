//
// setjmpex.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// This header replaces setjmp with _setjmpex, which will enable the safe setjmp
// and longjmp functionality that works correctly with __try/__except/__finally.
//
#pragma once
#define _INC_SETJMPEX



#ifdef _M_IX86
    #define setjmp  _setjmp
    #define longjmp _longjmpex
#else
    #undef setjmp
    #define setjmp _setjmpex
#endif

#include <setjmp.h>
