//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  stllock.h
//
//  Purpose: Critical section class
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _STLLOCK_H_
#define _STLLOCK_H_


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef _PREFAST_
#pragma prefast (push)  
#pragma prefast (disable: 26135) //We are using locks correctly.
#endif /* _PREFAST_ */



class CCritSec : public CRITICAL_SECTION
{
public:
    CCritSec() 
    {
        InitializeCriticalSection(this);
    }
    ~CCritSec()
    {
        DeleteCriticalSection(this);
    }
    void Enter()
    {
        EnterCriticalSection(this);
    }
    void Leave()
    {
        LeaveCriticalSection(this);
    }
};

#ifdef _PREFAST_
#pragma prefast (pop)
#endif /* _PREFAST_ */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion





#endif

