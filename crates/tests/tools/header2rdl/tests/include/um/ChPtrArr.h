//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  chptrarr.h
//
//  Purpose: Non-MFC CPtrArray class definition
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef __CHPTRARRAY__
#define __CHPTRARRAY__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <windows.h>
#include <limits.h>
#include <assert.h>
#include <tchar.h>
#include <polarity.h>
#include <ProvExce.h>

class POLARITY CHPtrArray
{
    public :

        // Construction/destruction
        //=========================

    CHPtrArray() ;

// Attributes
    int GetSize() const ;
    int GetUpperBound() const ;
    void SetSize(int nNewSize, int nGrowBy = -1) throw ( CHeap_Exception ) ;

// Operations
    // Clean up
    void FreeExtra() throw ( CHeap_Exception ) ;
    void RemoveAll() ;

    // Accessing elements
    void* GetAt(int nIndex) const ;
    void SetAt(int nIndex, void* newElement) ;
    void*& ElementAt(int nIndex) ;

    // Direct Access to the element data (may return NULL)
    const void** GetData() const ;
    void** GetData() ;

    // Potentially growing the array
    void SetAtGrow(int nIndex, void* newElement) throw ( CHeap_Exception ) ;
    int Add(void* newElement) throw ( CHeap_Exception ) ;
    int Append(const CHPtrArray& src) throw ( CHeap_Exception ) ;
    void Copy(const CHPtrArray& src) throw ( CHeap_Exception ) ;

    // overloaded operator helpers
    void* operator[](int nIndex) const ;
    void*& operator[](int nIndex) ;

    // Operations that move elements around
    void InsertAt(int nIndex, void* newElement, int nCount = 1) throw ( CHeap_Exception ) ;
    void RemoveAt(int nIndex, int nCount = 1) ;
    void InsertAt(int nStartIndex, CHPtrArray* pNewArray) throw ( CHeap_Exception ) ;

// Implementation
protected:
    void** m_pData ;   // the actual array of data
    int m_nSize ;     // # of elements (upperBound - 1)
    int m_nMaxSize ;  // max allocated
    int m_nGrowBy ;   // grow amount

public:
    ~CHPtrArray() ;
#ifdef _DEBUG
//    void Dump(CDumpContext&) const ;
    void AssertValid() const ;
#endif

protected:
    // local typedefs for class templates
    typedef void* BASE_TYPE ;
    typedef void* BASE_ARG_TYPE ;
} ;



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
