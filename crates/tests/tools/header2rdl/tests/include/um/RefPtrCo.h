//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  RefPtrCo.h
//
//  Purpose: definition of TRefPointerCollection template
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef __REFPTRCOLLECTION_H__
#define __REFPTRCOLLECTION_H__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <chptrarr.h>

// Enumeration helpers
typedef	DWORD	REFPTRCOLLECTION_POSITION;
#define	REFPTRCOLLECTION_START	0xFFFFFFFF;

template <class TYPED_PTR> class TRefPointerCollection : public CThreadBase
{
public:

	// Construction/Destruction
	TRefPointerCollection();
	~TRefPointerCollection();
    TRefPointerCollection(const TRefPointerCollection& trpc);

	// Allows addition and enumeration of collection
	BOOL	Add( TYPED_PTR* ptr );

	BOOL		BeginEnum( REFPTRCOLLECTION_POSITION& pos );
	TYPED_PTR*	GetNext( REFPTRCOLLECTION_POSITION& pos );
	void		EndEnum( void );

	void		Empty( void );
    int         GetSize( void ) const;

protected:

	// Allows easy and quick transference of data (it was =, but
	// because we'll inherit classes off the template, we won't
	// inherit that particular overload (some C++ thingie)

	const TRefPointerCollection<TYPED_PTR>& Copy( const TRefPointerCollection<TYPED_PTR>& );


private:

	CHPtrArray		m_ptrArray;

};

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::TRefPointerCollection
//
//	Class Constructor.
//
//	Inputs:		None.
//
//	Outputs:	None.
//
//	Return:		None.
//
//	Comments:	None.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
TRefPointerCollection<TYPED_PTR>::TRefPointerCollection( void )
:	CThreadBase(),
	m_ptrArray()
{
}

////////////////////////////////////////////////////////////////////////
//
//	Function:	CRefPointerCollection::~CRefPointerCollection
//
//	Class Destructor.
//
//	Inputs:		None.
//
//	Outputs:	None.
//
//	Return:		None.
//
//	Comments:	None.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
TRefPointerCollection<TYPED_PTR>::~TRefPointerCollection( void )
{
	Empty();
}


////////////////////////////////////////////////////////////////////////
//
//	Function:	CRefPointerCollection::CRefPointerCollection
//              Copy constructor
//
//	Inputs:		None.
//
//	Outputs:	None.
//
//	Return:		None.
//
//	Comments:	None.
//
////////////////////////////////////////////////////////////////////////
template <class TYPED_PTR>
TRefPointerCollection<TYPED_PTR>::TRefPointerCollection(
    const TRefPointerCollection& trpc)
{
    Copy(trpc);	
}


////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::Add
//
//	Adds a new referenced pointer to the collection.
//
//	Inputs:		T*				ptr - Pointer to add.
//
//	Outputs:	None.
//
//	Return:		TRUE/FALSE		Success/Failure of Add.
//
//	Comments:	AddRefs the pointer, then adds it to the array.  We
//				will need Write Access to do this.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
BOOL TRefPointerCollection<TYPED_PTR>::Add( TYPED_PTR* ptr )
{
	BOOL	fReturn = FALSE;

	if ( NULL != ptr )
	{
		// Get write access
		if ( BeginWrite() )
		{
            try
            {
			    // If Add succeeds, the pointer will be released when it
			    // is removed.

			    ptr->AddRef();

			    if ( m_ptrArray.Add( (void*) ptr ) >= 0 )
			    {
				    fReturn = TRUE;
			    }
			    else
			    {
				    ptr->Release();	// Add failed, so Release the AddRef
			    }
            }
            catch ( CHeap_Exception& )
            {
    	        EndWrite() ;
                throw;
            }
            catch ( CFramework_Exception& e_FR )
            {
    	        EndWrite() ;
                throw;
            }

			EndWrite();	// Release the BeginWrite()
		}
	}

	return fReturn;
}

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::BeginEnum
//
//	Gains Read Access to the collection, then returns an appropriate
//	REFPTRCOLLECTION_POSITION to get the first index in the array.
//
//	Inputs:		None.
//
//	Outputs:	REFPTRCOLLECTION_POSITION&	pos - Position we retrieved.
//
//	Return:		BOOL		TRUE/FALSE - Access was granted
//
//	Comments:	We need Read Access to do this.  This can effectively
//				lock out other threads.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
BOOL TRefPointerCollection<TYPED_PTR>::BeginEnum( REFPTRCOLLECTION_POSITION& pos )
{
	BOOL	fReturn	=	FALSE;

	if ( BeginRead() )
	{
		pos = REFPTRCOLLECTION_START;
		fReturn = TRUE;
	}

	return fReturn;

}

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::EndEnum
//
//	Signals the end of an enumeration.
//
//	Inputs:		None.
//
//	Outputs:	None.
//
//	Return:		BOOL		TRUE/FALSE - Access was granted
//
//	Comments:	Ends Read Access granted by calling BeginEnum().
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
void TRefPointerCollection<TYPED_PTR>::EndEnum( void )
{
	EndRead();
}

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::GetNext
//
//	Uses the REFPTRCOLLECTION_POSITION to get the next index in the
//	collection.
//
//	Inputs:		None.
//
//	Outputs:	REFPTRCOLLECTION_POSITION&	pos - Position we retrieved.
//
//	Return:		T*		NULL if failure.
//
//	Comments:	We need Read Access to do this.  The pointer is AddRef'd
//				on the way out.  User must Release the pointer himself.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
TYPED_PTR* TRefPointerCollection<TYPED_PTR>::GetNext( REFPTRCOLLECTION_POSITION& pos )
{
	TYPED_PTR*	ptr = NULL;

	if ( BeginRead() )
	{
		if ( ++pos < (DWORD) m_ptrArray.GetSize() )
		{
			ptr = (TYPED_PTR*) m_ptrArray.GetAt( pos );

			if ( NULL != ptr )
			{
				ptr->AddRef();
			}
		}

		EndRead();
	}

	return ptr;
}

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::Empty
//
//	Empties out the collection, Releasing Pointers as it does do.
//
//	Inputs:		None.
//
//	Outputs:	None.
//
//	Return:		None.
//
//	Comments:	We need Write Access to do this.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
void TRefPointerCollection<TYPED_PTR>::Empty( void )
{
	// By default this is an infinite wait, so it best come back

	BeginWrite();

    try
    {

	    int	nSize	=	m_ptrArray.GetSize();

	    // Only empty it if it is not empty
	    if ( nSize > 0 )
	    {
		    TYPED_PTR*	ptr		=	NULL;

		    for ( int nCtr = 0; nCtr < nSize; nCtr++ )
		    {
			    ptr = (TYPED_PTR*) m_ptrArray[nCtr];

			    if ( NULL != ptr )
			    {
				    ptr->Release();	// AddRef we did when we added it
			    }
		    }

		    // Now dump the array
		    m_ptrArray.RemoveAll();

	    }	// IF nSize > 0

    }
    catch ( CHeap_Exception& )
    {
    	EndWrite() ;
        throw;
    }
    catch ( CFramework_Exception& e_FR )
    {
        EndWrite() ;
        throw;
    }

	EndWrite();
}

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::Copy
//
//	Empties out the collection, copies in another one, addrefing
//	pointers as we go.
//
//	Inputs:		const T&	collection
//
//	Outputs:	None.
//
//	Return:		const T&	this
//
//	Comments:	We need Write Access to do this.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
const TRefPointerCollection<TYPED_PTR>& TRefPointerCollection<TYPED_PTR>::Copy( const TRefPointerCollection<TYPED_PTR>& collection )
{
	// By default this is an infinite wait, so it best come back
	BeginWrite();

    try
    {

	    // Dump out the array
	    Empty();

	    int	nSize = collection.m_ptrArray.GetSize();

	    for ( int nCount = 0; nCount < nSize; nCount++ )
	    {
		    TYPED_PTR*	ptr = (TYPED_PTR*) collection.m_ptrArray[nCount];

		    // Add will automatically AddRef the pointer again.
		    Add( ptr );
	    }
    }
    catch ( CHeap_Exception& )
    {
    	EndWrite() ;
        throw;
    }
    catch ( CFramework_Exception& e_FR )
    {
        EndWrite() ;
        throw;
    }

	EndWrite();

	return *this;
}

////////////////////////////////////////////////////////////////////////
//
//	Function:	TRefPointerCollection::GetSize
//
//	Inputs:		None.
//
//	Outputs:	Number of elements in the collection
//
//	Return:		None.
//
//	Comments:	None.
//
////////////////////////////////////////////////////////////////////////

template <class TYPED_PTR>
int TRefPointerCollection<TYPED_PTR>::GetSize(void) const
{
    return m_ptrArray.GetSize();
}



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
