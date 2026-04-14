//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  CHSTRARR.H
//
//  Purpose: Utility library version of MFC CHStringArray
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _CHStringArray_
#define _CHStringArray_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <polarity.h>
#include <ProvExce.h>

class POLARITY CHStringArray 
{
    public:

        CHStringArray();
        ~CHStringArray();

        // Attributes
        int GetSize() const             { return m_nSize; }
        int GetUpperBound() const       { return m_nSize-1; }
        void SetSize(int nNewSize, int nGrowBy = -1) throw ( CHeap_Exception ) ;

        // Operations
        // Clean up
        void FreeExtra() throw ( CHeap_Exception ) ;
        void RemoveAll()                { SetSize(0); }

        // Accessing elements
#if (!defined DEBUG && !defined _DEBUG)
        CHString GetAt(int nIndex) const{ return m_pData[nIndex]; }
        void SetAt(int nIndex, LPCWSTR newElement){ m_pData[nIndex] = newElement; }
        CHString& ElementAt(int nIndex) { return m_pData[nIndex]; }
#else
        CHString GetAt(int nIndex) const;
        void SetAt(int nIndex, LPCWSTR newElement);
        CHString& ElementAt(int nIndex);
#endif

        // Direct Access to the element data (may return NULL)
        const CHString* GetData() const { return (const CHString*)m_pData; }
        CHString* GetData()             { return (CHString*)m_pData; }

        // Potentially growing the array
        void SetAtGrow(int nIndex, LPCWSTR newElement) throw ( CHeap_Exception ) ;
        int Add(LPCWSTR newElement) throw ( CHeap_Exception ) 
        { 
            int nIndex = m_nSize;
            SetAtGrow(nIndex, newElement);
            return nIndex; 
        }

        int Append(const CHStringArray& src) throw ( CHeap_Exception ) ;
        void Copy(const CHStringArray& src) throw ( CHeap_Exception ) ;

        // overloaded operator helpers
        CHString operator[](int nIndex) const { return GetAt(nIndex); }
        CHString& operator[](int nIndex)      { return ElementAt(nIndex); }

        // Operations that move elements around
        void InsertAt(int nIndex, LPCWSTR newElement, int nCount = 1) throw ( CHeap_Exception ) ;
        void RemoveAt(int nIndex, int nCount = 1);
        void InsertAt(int nStartIndex, CHStringArray* pNewArray) throw ( CHeap_Exception ) ;

        // Implementation

    protected:
        CHString* m_pData;      // the actual array of data
        int m_nSize;            // # of elements (upperBound - 1)
        int m_nMaxSize;         // max allocated
        int m_nGrowBy;          // grow amount
                                // local typedefs for class templates
        typedef CHString BASE_TYPE;
        typedef LPCWSTR BASE_ARG_TYPE;
};
////////////////////////////////////////////////////////////////////////////

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

