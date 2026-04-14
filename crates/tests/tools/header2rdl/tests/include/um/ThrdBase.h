//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  ThrdBase.h
//
//  Purpose: Definition of ThreadBase class
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef __THREADBASE_H__
#define __THREADBASE_H__


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef _PREFAST_
#pragma prefast (push)
#pragma prefast (disable: 26135) //We are using locks correctly.
#endif /* _PREFAST_ */


class POLARITY CThreadBase
{
public:

	enum THREAD_SAFETY_MECHANISM
	{
		etsmFirst		= 0,
		etsmSerialized	= 0,
		etsmPriorityRead,
		etsmPriorityWrite,
		etsmLast
	};

	// Construction/Destruction
	CThreadBase( THREAD_SAFETY_MECHANISM etsm = etsmSerialized );
	virtual ~CThreadBase();

	// Thread Safe Ref/Counting functions
	LONG	AddRef( void );
	LONG	Release( void );

	// Provide Readable Read/Write accessors should
	// we not want to serialize at a later date.  Note
	// that timeouts have no meaning unless we're
	// doing a non-serialized implementation.

	BOOL	BeginRead( DWORD dwTimeOut = INFINITE );
	void	EndRead( void );

	BOOL	BeginWrite( DWORD dwTimeOut = INFINITE );
	void	EndWrite( void );

protected:

	virtual void	OnFinalRelease( void );

	// Thread Safety functions


private:

	CRITICAL_SECTION		m_cs;
	LONG					m_lRefCount;
	THREAD_SAFETY_MECHANISM	m_etsm;

	// Private thread safety functions.  We can maybe promote
	// these to protected if we see a need to later, however
	// for right now, everyone should specify if they mean
	// to read or write when they wish to access data that
	// may change.

	void	Lock( void );
	void	Unlock( void );

};

inline BOOL CThreadBase::BeginRead( DWORD dwTimeout /*=INFINITE*/ )
{
	UNREFERENCED_PARAMETER( dwTimeout );
	EnterCriticalSection( &m_cs );
	return TRUE;
}

inline void CThreadBase::EndRead( void )
{
	LeaveCriticalSection( &m_cs );
}

inline BOOL CThreadBase::BeginWrite( DWORD dwTimeout /*=INFINITE*/ )
{
	UNREFERENCED_PARAMETER( dwTimeout );
	EnterCriticalSection( &m_cs );
	return TRUE;
}

inline void CThreadBase::EndWrite( void )
{
	LeaveCriticalSection( &m_cs );
}

inline void CThreadBase::Lock( void )
{
	EnterCriticalSection( &m_cs );
}

inline void CThreadBase::Unlock( void )
{
	LeaveCriticalSection( &m_cs );
}


#ifdef _PREFAST_
#pragma prefast (pop)
#endif /* _PREFAST_ */



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion



#endif

