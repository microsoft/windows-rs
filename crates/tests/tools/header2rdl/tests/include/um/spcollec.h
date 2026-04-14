/*****************************************************************************
// Copyright Microsoft Corporation. All Rights Reserved. 
* SPCollec.h *
*------------*
*       This header file contains the SAPI5 collection class templates. These
*   are a modified version of the MFC template classes without the dependencies.
*-----------------------------------------------------------------------------
*****************************************************************************/
#ifndef SPCollec_h
#define SPCollec_h

#ifndef _INC_LIMITS
#include <limits.h>
#endif

#ifndef _INC_STDLIB
#include <stdlib.h>
#endif

#ifndef _INC_SEARCH
#include <search.h>
#endif

#include <intsafe.h>

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/////////////////////////////////////////////////////////////////////////////
#define SPASSERT_VALID( a )             // This doesn't do anything right now

typedef void* SPLISTPOS;
typedef DWORD SPLISTHANDLE;

#define SP_BEFORE_START_POSITION ((void*)-1L)

inline BOOL SPIsValidAddress(const void* lp, UINT /* nBytes */, BOOL /* bReadWrite */)
{
    return (lp != NULL);
}

/////////////////////////////////////////////////////////////////////////////
// global helpers (can be overridden)

inline HRESULT SPConstructElementsVoid (void *pElements, size_t sizeof_TYPE, int nCount)
{
    HRESULT hr = S_OK;
    _ASSERT( nCount == 0 ||
             SPIsValidAddress( pElements, nCount * (UINT) sizeof_TYPE, TRUE ) );

    // default is bit-wise zero initialization
    memset(pElements, 0, nCount * sizeof_TYPE);
    return hr;
}

template<class TYPE>
inline HRESULT SPConstructElements(_In_reads_(nCount) TYPE* pElements, _In_ int nCount)
{
    HRESULT hr = S_OK;
    _ASSERT( nCount == 0 ||
             SPIsValidAddress( pElements, nCount * sizeof(TYPE), TRUE ) );

    // default is bit-wise zero initialization
    memset((void*)pElements, 0, nCount * sizeof(TYPE));
    return hr;
}

inline void SPDestructElementsVoid(void* pElements, size_t sizeof_TYPE, int nCount)
{
    _ASSERT( ( nCount == 0 ||
               SPIsValidAddress( pElements, nCount * (UINT) sizeof_TYPE, TRUE  ) ) );
    (void)pElements;  // not used
    (void)nCount; // not used
    (void)sizeof_TYPE; // not used

    // default does nothing
}

template<class TYPE>
inline void SPDestructElements(TYPE* pElements, int nCount)
{
    _ASSERT( ( nCount == 0 ||
               SPIsValidAddress( pElements, nCount * sizeof(TYPE), TRUE  ) ) );
    (void)pElements;  // not used
    (void)nCount; // not used

    // default does nothing
}

inline HRESULT SPCopyElementsVoid(void* pDest, const void* pSrc, size_t sizeof_TYPE, int nCount)
{
    HRESULT hr = S_OK;
    _ASSERT( ( nCount == 0 ||
               SPIsValidAddress( pDest, nCount * (UINT) sizeof_TYPE, TRUE  )) );
    _ASSERT( ( nCount == 0 ||
               SPIsValidAddress( pSrc, nCount * (UINT) sizeof_TYPE, FALSE  )) );

    // default is bit-wise copy
    memcpy(pDest, pSrc, nCount * sizeof_TYPE);
    return hr;
}

template<class TYPE>
inline HRESULT SPCopyElements(TYPE* pDest, const TYPE* pSrc, int nCount)
{
    HRESULT hr = S_OK;
    _ASSERT( ( nCount == 0 ||
               SPIsValidAddress( pDest, nCount * sizeof(TYPE), TRUE  )) );
    _ASSERT( ( nCount == 0 ||
               SPIsValidAddress( pSrc, nCount * sizeof(TYPE), FALSE  )) );

    // default is bit-wise copy
    memcpy(pDest, pSrc, nCount * sizeof(TYPE));
    return hr;
}

template<class TYPE, class ARG_TYPE>
BOOL SPCompareElements(const TYPE* pElement1, const ARG_TYPE* pElement2)
{
    _ASSERT( SPIsValidAddress( pElement1, sizeof(TYPE), FALSE ) );
    _ASSERT( SPIsValidAddress( pElement2, sizeof(ARG_TYPE), FALSE ) );
    return *pElement1 == *pElement2;
}

template<class ARG_KEY>
inline UINT SPHashKey(ARG_KEY key)
{
    // default identity hash - works for most primitive values
    return ((UINT)(DWORD_PTR)key) >> 4;
}

/////////////////////////////////////////////////////////////////////////////
// CSPPlex

struct CSPPlex    // warning variable length structure
{
    CSPPlex* pNext;
    UINT nMax;
    UINT nCur;
    /* BYTE data[maxNum*elementSize]; */
    void* data() { return this+1; }

    static CSPPlex* PASCAL Create( CSPPlex*& pHead, UINT nMax, UINT cbElement )
    {
	CSPPlex* p = NULL;
	UINT cbSize;
	if (SUCCEEDED(UIntMult(nMax, cbElement, &cbSize)) && SUCCEEDED(UIntAdd(cbSize, sizeof(CSPPlex), &cbSize)))
	{
	    p = (CSPPlex*) new BYTE[cbSize];
	}
        if (p != NULL)
        {
            p->nMax = nMax;
            p->nCur = 0;
            p->pNext = pHead;
            pHead = p;  // change head (adds in reverse order for simplicity)
        }
        return p;
    }

    void FreeDataChain()
    {
        CSPPlex* p = this;
        while (p != NULL)
        {
            BYTE* bytes = (BYTE*) p;
            CSPPlex* pNextLocal = p->pNext;
            delete[] bytes;
            p = pNextLocal;
        }
    }
};


/////////////////////////////////////////////////////////////////////////////
// CSPArray<TYPE, ARG_TYPE>

class CSPArrayVoid // non-template base worked class for template CSPArray
{
public:
// Construction
    CSPArrayVoid();

// Attributes
    int GetSize() const;
    int GetUpperBound() const;
    HRESULT SetSize(int nNewSize, int nGrowBy = -1);

    // Clean up
    void FreeExtra();
    void RemoveAll();

    // Operations that move elements around
    void    RemoveAt(int nIndex, int nCount = 1);
    void    Sort(int (__cdecl *compare )(const void *elem1, const void *elem2 ));

// Implementation
protected:
    void*  m_pData;          // the actual array of data
    int    m_nSize;          // # of elements (upperBound - 1)
    int    m_nMaxSize;       // max allocated
    int    m_nGrowBy;        // grow amount
    size_t m_sizeofTYPE;     // size of TYPE in template class
    
public:
    ~CSPArrayVoid();
#ifdef _DEBUG
    void AssertValid() const;
#endif
};

/////////////////////////////////////////////////////////////////////////////
// CSPArrayVoid implementation

inline int CSPArrayVoid::GetSize() const
    { return m_nSize; }
inline int CSPArrayVoid::GetUpperBound() const
    { return m_nSize-1; }
inline void CSPArrayVoid::RemoveAll()
    { SetSize(0, -1); }

inline CSPArrayVoid::CSPArrayVoid()
{
    m_pData = NULL;
    m_nSize = m_nMaxSize = m_nGrowBy = 0;
}

inline CSPArrayVoid::~CSPArrayVoid()
{
    SPASSERT_VALID( this );

    if (m_pData != NULL)
    {
        SPDestructElementsVoid(m_pData, m_sizeofTYPE, m_nSize);
        delete[] (BYTE*)m_pData;
    }
}

inline HRESULT CSPArrayVoid::SetSize(int nNewSize, int nGrowBy)
{
    SPASSERT_VALID( this );
    _ASSERT( nNewSize >= 0 );
    HRESULT hr = S_OK;

    if (nGrowBy != -1)
        m_nGrowBy = nGrowBy;  // set new size

    if (nNewSize == 0)
    {
        // shrink to nothing
        if (m_pData != NULL)
        {
            SPDestructElementsVoid(m_pData, m_sizeofTYPE, m_nSize);
            delete[] (BYTE*)m_pData;
            m_pData = NULL;
        }
        m_nSize = m_nMaxSize = 0;
    }
    else if (m_pData == NULL)
    {
        // create one with exact size
#ifdef SIZE_T_MAX
        _ASSERT( (UINT)nNewSize <= SIZE_T_MAX/m_sizeofTYPE );    // no overflow
#endif
        m_pData = (void *)new BYTE[nNewSize * m_sizeofTYPE];
        if( m_pData )
        {
            hr = SPConstructElementsVoid(m_pData, m_sizeofTYPE, nNewSize);
            if( SUCCEEDED( hr ) )
            {
                m_nSize = m_nMaxSize = nNewSize;
            }
            else
            {
                delete[] (BYTE*)m_pData;
                m_pData = NULL;
            }
        }
        else
        {
            hr = E_OUTOFMEMORY;
        }
    }
    else if (nNewSize <= m_nMaxSize)
    {
        // it fits
        if (nNewSize > m_nSize)
        {
            // initialize the new elements
            hr = SPConstructElementsVoid((BYTE *)m_pData+m_nSize*m_sizeofTYPE, m_sizeofTYPE, nNewSize-m_nSize);
        }
        else if (m_nSize > nNewSize)
        {
            // destroy the old elements
            SPDestructElementsVoid((BYTE *)m_pData+nNewSize*m_sizeofTYPE, m_sizeofTYPE, m_nSize-nNewSize);
        }

        if( SUCCEEDED( hr ) )
        {
            m_nSize = nNewSize;
        }
    }
    else
    {
        // otherwise, grow array
        nGrowBy = m_nGrowBy;
        if (nGrowBy == 0)
        {
            // heuristically determe growth when nGrowBy == 0
            //  (this avoids heap fragmentation in many situations)
            nGrowBy = min(1024, max(4, m_nSize / 8));
        }
        int nNewMax;
        if (nNewSize < m_nMaxSize + nGrowBy)
            nNewMax = m_nMaxSize + nGrowBy;  // granularity
        else
            nNewMax = nNewSize;  // no slush

        _ASSERT( nNewMax >= m_nMaxSize );  // no wrap around
#ifdef SIZE_T_MAX
        _ASSERT( (UINT)nNewMax <= SIZE_T_MAX/m_sizeofTYPE ); // no overflow
#endif
        void* pNewData = (void *)new BYTE[nNewMax * m_sizeofTYPE];

        if( pNewData )
        {
            // copy new data from old
            memcpy(pNewData, m_pData, m_nSize * m_sizeofTYPE);

            // construct remaining elements
            _ASSERT( nNewSize > m_nSize );
            hr = SPConstructElementsVoid((BYTE *)pNewData+m_nSize*m_sizeofTYPE, m_sizeofTYPE, nNewSize-m_nSize);

            // get rid of old stuff (note: no destructors called)
            delete[] (BYTE*)m_pData;
            m_pData = pNewData;
            m_nSize = nNewSize;
            m_nMaxSize = nNewMax;
        }
        else
        {
            hr = E_OUTOFMEMORY;
        }
    }
    return hr;
}

inline void CSPArrayVoid::FreeExtra()
{
    SPASSERT_VALID( this );

    if (m_nSize != m_nMaxSize)
    {
        // shrink to desired size
#ifdef SIZE_T_MAX
        _ASSERT( (UINT)m_nSize <= SIZE_T_MAX/m_sizeofTYPE); // no overflow
#endif
        void* pNewData = NULL;
        if (m_nSize != 0)
        {
            pNewData =  (void *)new BYTE[m_nSize * m_sizeofTYPE];
            _ASSERT(pNewData);
            // copy new data from old
            memcpy(pNewData, m_pData, m_nSize * m_sizeofTYPE);
        }

        // get rid of old stuff (note: no destructors called)
        delete[] (BYTE*)m_pData;
        m_pData = pNewData;
        m_nMaxSize = m_nSize;
    }
}

inline void CSPArrayVoid::RemoveAt(int nIndex, int nCount)
{
    SPASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );
    _ASSERT( nCount >= 0 );
    _ASSERT( nIndex + nCount <= m_nSize );

    // just remove a range
    int nMoveCount = m_nSize - (nIndex + nCount);
    SPDestructElementsVoid((BYTE *)m_pData + nIndex*m_sizeofTYPE, m_sizeofTYPE, nCount);
    if (nMoveCount)
        memcpy((BYTE *)m_pData + nIndex*m_sizeofTYPE, (BYTE *)m_pData + (nIndex + nCount)*m_sizeofTYPE,
            nMoveCount * m_sizeofTYPE);
    m_nSize -= nCount;
}

inline void CSPArrayVoid::Sort(int (__cdecl *compare )(const void *elem1, const void *elem2 ))
{
    SPASSERT_VALID( this );
    _ASSERT( m_pData != NULL );

    qsort( m_pData, m_nSize, m_sizeofTYPE, compare );
}

#ifdef _DEBUG
inline void CSPArrayVoid::AssertValid() const
{
    if (m_pData == NULL)
    {
        _ASSERT( m_nSize == 0 );
        _ASSERT( m_nMaxSize == 0 );
    }
    else
    {
        _ASSERT( m_nSize >= 0 );
        _ASSERT( m_nMaxSize >= 0 );
        _ASSERT( m_nSize <= m_nMaxSize );
        _ASSERT( SPIsValidAddress(m_pData, m_nMaxSize * (UINT) m_sizeofTYPE, TRUE ) );
    }
}
#endif //_DEBUG

/////////////////////////////////////////////////////////////////////////////
// now for the derived template class

template<class TYPE, class ARG_TYPE>
class CSPArray : public CSPArrayVoid
{
public:
// Construction
    CSPArray();

// Attributes
//    int GetSize() const;       // now in base class
//    int GetUpperBound() const; // now in base class

// Operations
    // Clean up
//    void FreeExtra();          // now in base class
//    void RemoveAll();          // now in base class

    // Accessing elements
    TYPE GetAt(int nIndex) const;
    void SetAt(int nIndex, ARG_TYPE newElement);
    TYPE& ElementAt(int nIndex);

    // Direct Access to the element data (may return NULL)
    const TYPE* GetData() const;
    TYPE* GetData();

    // Potentially growing the array
    HRESULT SetAtGrow(int nIndex, ARG_TYPE newElement);

    // the helper method add will call SetAtGrow and return the original size of the array (essentially the index of the entry you just added)
    int Add(ARG_TYPE newElement);
    // AddHR is the same as Add except it returns a HRESULT instead of the new index. Essentially the same as SetAtGrow but without requiring the target index
    HRESULT AddHR(ARG_TYPE newElement);
    int Append(const CSPArray& src);
    HRESULT Copy(const CSPArray& src);

    // overloaded operator helpers
    TYPE operator[](int nIndex) const;
    TYPE& operator[](int nIndex);

    // Operations that move elements around
    HRESULT InsertAt(int nIndex, ARG_TYPE newElement, int nCount = 1);
//    void    RemoveAt(int nIndex, int nCount = 1); // now in base class
    HRESULT InsertAt(int nStartIndex, CSPArray* pNewArray);
//    void    Sort(int (__cdecl *compare )(const void *elem1, const void *elem2 )); // now in base class

public:
    ~CSPArray() {};
#ifdef _DEBUG
//  void Dump(CDumpContext&) const;
    void AssertValid() const;
#endif
};

/////////////////////////////////////////////////////////////////////////////
// CSPArray<TYPE, ARG_TYPE> inline functions

template<class TYPE, class ARG_TYPE>
inline TYPE CSPArray<TYPE, ARG_TYPE>::GetAt(int nIndex) const
    { _ASSERT( (nIndex >= 0 && nIndex < m_nSize) );
        return ((TYPE *)m_pData)[nIndex]; }
template<class TYPE, class ARG_TYPE>
inline void CSPArray<TYPE, ARG_TYPE>::SetAt(int nIndex, ARG_TYPE newElement)
    { _ASSERT( (nIndex >= 0 && nIndex < m_nSize) );
        ((TYPE *)m_pData)[nIndex] = newElement; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPArray<TYPE, ARG_TYPE>::ElementAt(int nIndex)
    { _ASSERT( (nIndex >= 0 && nIndex < m_nSize) );
        return ((TYPE *)m_pData)[nIndex]; }
template<class TYPE, class ARG_TYPE>
inline const TYPE* CSPArray<TYPE, ARG_TYPE>::GetData() const
    { return (const TYPE*)m_pData; }
template<class TYPE, class ARG_TYPE>
inline TYPE* CSPArray<TYPE, ARG_TYPE>::GetData()
    { return (TYPE*)m_pData; }
template<class TYPE, class ARG_TYPE>
inline int CSPArray<TYPE, ARG_TYPE>::Add(ARG_TYPE newElement)
    { int nIndex = m_nSize;
        SetAtGrow(nIndex, newElement);
        return nIndex; }
template<class TYPE, class ARG_TYPE>
inline HRESULT CSPArray<TYPE, ARG_TYPE>::AddHR(ARG_TYPE newElement)
    {   return SetAtGrow(m_nSize, newElement); }
template<class TYPE, class ARG_TYPE>
inline TYPE CSPArray<TYPE, ARG_TYPE>::operator[](int nIndex) const
    { return GetAt(nIndex); }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPArray<TYPE, ARG_TYPE>::operator[](int nIndex)
    { return ElementAt(nIndex); }

/////////////////////////////////////////////////////////////////////////////
// CSPArray<TYPE, ARG_TYPE> out-of-line functions

template<class TYPE, class ARG_TYPE>
inline CSPArray<TYPE, ARG_TYPE>::CSPArray()
{
    m_sizeofTYPE = sizeof(TYPE);
}

template<class TYPE, class ARG_TYPE>
int CSPArray<TYPE, ARG_TYPE>::Append(const CSPArray& src)
{
    SPASSERT_VALID( this );
    _ASSERT( this != &src );   // cannot append to itself

    int nOldSize = m_nSize;
    HRESULT hr = SetSize(m_nSize + src.m_nSize);
    if( SUCCEEDED( hr ) )
    {
        hr = SPCopyElements((TYPE *)m_pData + nOldSize, (TYPE *)(src.m_pData), src.m_nSize);
    }
    return ( SUCCEEDED( hr ) )?(nOldSize):(-1);
}

template<class TYPE, class ARG_TYPE>
HRESULT CSPArray<TYPE, ARG_TYPE>::Copy(const CSPArray& src)
{
    SPASSERT_VALID( this );
    _ASSERT( this != &src );   // cannot copy to itself

    HRESULT hr = SetSize(src.m_nSize);
    if( SUCCEEDED( hr ) )
    {
        hr = SPCopyElements((TYPE *)m_pData, (TYPE *)(src.m_pData), src.m_nSize);
    }
    return hr;
}

template<class TYPE, class ARG_TYPE>
HRESULT CSPArray<TYPE, ARG_TYPE>::SetAtGrow(int nIndex, ARG_TYPE newElement)
{
    SPASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );
    HRESULT hr = S_OK;

    if (nIndex >= m_nSize)
    {
        hr = SetSize(nIndex+1, -1);
    }

    if( SUCCEEDED( hr ) )
    {
        ((TYPE *)m_pData)[nIndex] = newElement;
    }
    return hr;
}

template<class TYPE, class ARG_TYPE>
HRESULT CSPArray<TYPE, ARG_TYPE>::InsertAt(int nIndex, ARG_TYPE newElement, int nCount /*=1*/)
{
    SPASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );    // will expand to meet need
    _ASSERT( nCount > 0 );     // zero or negative size not allowed
    HRESULT hr = S_OK;

    if (nIndex >= m_nSize)
    {
        // adding after the end of the array
        hr = SetSize(nIndex + nCount, -1);   // grow so nIndex is valid
    }
    else
    {
        // inserting in the middle of the array
        int nOldSize = m_nSize;
        hr = SetSize(m_nSize + nCount, -1);  // grow it to new size
        if( SUCCEEDED( hr ) )
        {
            // shift old data up to fill gap
            memmove(&((TYPE *)m_pData)[nIndex+nCount], &((TYPE *)m_pData)[nIndex],
                (nOldSize-nIndex) * sizeof(TYPE));

            // re-init slots we copied from
            hr = SPConstructElements(&((TYPE *)m_pData)[nIndex], nCount);
        }
    }

    // insert new value in the gap
    if( SUCCEEDED( hr ) )
    {
        _ASSERT( nIndex + nCount <= m_nSize );
        while (nCount--)
            ((TYPE *)m_pData)[nIndex++] = newElement;
    }
    return hr;
}

template<class TYPE, class ARG_TYPE>
HRESULT CSPArray<TYPE, ARG_TYPE>::InsertAt(int nStartIndex, CSPArray* pNewArray)
{
    SPASSERT_VALID( this );
    SPASSERT_VALID( pNewArray );
    _ASSERT( nStartIndex >= 0 );
    HRESULT hr = S_OK;

    if (pNewArray->GetSize() > 0)
    {
        hr = InsertAt(nStartIndex, pNewArray->GetAt(0), pNewArray->GetSize());
        for (int i = 0; SUCCEEDED( hr )&& (i < pNewArray->GetSize()); i++)
        {
            SetAt(nStartIndex + i, pNewArray->GetAt(i));
        }
    }
    return hr;
}


/////////////////////////////////////////////////////////////////////////////
// CSPList<TYPE, ARG_TYPE>

template<class TYPE, class ARG_TYPE>
class CSPList
{
protected:
    struct CNode
    {
        CNode* pNext;
        CNode* pPrev;
        TYPE data;
    };
public:

// Construction
    CSPList(int nBlockSize = 10);

// Attributes (head and tail)
    // count of elements
    int GetCount() const;
    BOOL IsEmpty() const;

    // peek at head or tail
    TYPE& GetHead();
    TYPE GetHead() const;
    TYPE& GetTail();
    TYPE GetTail() const;

// Operations
    // get head or tail (and remove it) - don't call on empty list !
    TYPE RemoveHead();
    TYPE RemoveTail();

    // add before head or after tail
    SPLISTPOS AddHead(ARG_TYPE newElement);
    SPLISTPOS AddTail(ARG_TYPE newElement);

    // add another list of elements before head or after tail
    HRESULT AddHead(CSPList* pNewList);
    HRESULT AddTail(CSPList* pNewList);

    // remove all elements
    void RemoveAll();

    // iteration
    SPLISTPOS GetHeadPosition() const;
    SPLISTPOS GetTailPosition() const;
    TYPE& GetNext(SPLISTPOS& rPosition); // return *Position++
    TYPE GetNext(SPLISTPOS& rPosition) const; // return *Position++
    TYPE& GetPrev(SPLISTPOS& rPosition); // return *Position--
    TYPE GetPrev(SPLISTPOS& rPosition) const; // return *Position--

    // getting/modifying an element at a given position
    TYPE& GetAt(SPLISTPOS position);
    TYPE GetAt(SPLISTPOS position) const;
    void SetAt(SPLISTPOS pos, ARG_TYPE newElement);
    void RemoveAt(SPLISTPOS position);

    // inserting before or after a given position
    SPLISTPOS InsertBefore(SPLISTPOS position, ARG_TYPE newElement);
    SPLISTPOS InsertAfter(SPLISTPOS position, ARG_TYPE newElement);

    // helper functions (note: O(n) speed)
    SPLISTPOS Find(ARG_TYPE searchValue, SPLISTPOS startAfter = NULL) const;
        // defaults to starting at the HEAD, return NULL if not found
    SPLISTPOS FindIndex(int nIndex) const;
        // get the 'nIndex'th element (may return NULL)

// Implementation
protected:
    CNode* m_pNodeHead;
    CNode* m_pNodeTail;
    int m_nCount;
    CNode* m_pNodeFree;
    struct CSPPlex* m_pBlocks;
    int m_nBlockSize;

    CNode* NewNode(CNode* pPrev, CNode* pNext)
    {
        if (m_pNodeFree == NULL)
        {
            // add another block
            CSPPlex* pNewBlock = CSPPlex::Create(m_pBlocks, m_nBlockSize,sizeof(CNode));
            if (pNewBlock != NULL)
            {
                // chain them into free list
                CNode* pNode = (CNode*) pNewBlock->data();
                // free in reverse order to make it easier to debug
                pNode += m_nBlockSize - 1;
                for (int i = m_nBlockSize-1; i >= 0; i--, pNode--)
                {
                    pNode->pNext = m_pNodeFree;
                    m_pNodeFree = pNode;
                }
            }
        }

        CNode* pNode = m_pNodeFree;
        if( pNode )
        {
            if( SUCCEEDED( SPConstructElements(&pNode->data, 1) ) )
            {
                m_pNodeFree  = m_pNodeFree->pNext;
                pNode->pPrev = pPrev;
                pNode->pNext = pNext;
                m_nCount++;
                _ASSERT( m_nCount > 0 );  // make sure we don't overflow
            }
        }
        return pNode;
    }
    void FreeNode(CNode* pNode)
    {
        SPDestructElements(&pNode->data, 1);
        pNode->pNext = m_pNodeFree;
        m_pNodeFree = pNode;
        m_nCount--;
        _ASSERT( m_nCount >= 0 );  // make sure we don't underflow
    }

public:
    ~CSPList();
#ifdef _DEBUG
    void AssertValid() const;
#endif
};

/////////////////////////////////////////////////////////////////////////////
// CSPList<TYPE, ARG_TYPE> inline functions

template<class TYPE, class ARG_TYPE>
inline int CSPList<TYPE, ARG_TYPE>::GetCount() const
    { return m_nCount; }
template<class TYPE, class ARG_TYPE>
inline BOOL CSPList<TYPE, ARG_TYPE>::IsEmpty() const
    { return m_nCount == 0; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPList<TYPE, ARG_TYPE>::GetHead()
    {   _ASSERT( m_pNodeHead != NULL );
        return m_pNodeHead->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CSPList<TYPE, ARG_TYPE>::GetHead() const
    {   _ASSERT( m_pNodeHead != NULL );
        return m_pNodeHead->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPList<TYPE, ARG_TYPE>::GetTail()
    {   _ASSERT( m_pNodeTail != NULL );
        return m_pNodeTail->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CSPList<TYPE, ARG_TYPE>::GetTail() const
    {   _ASSERT( m_pNodeTail != NULL );
        return m_pNodeTail->data; }
template<class TYPE, class ARG_TYPE>
inline SPLISTPOS CSPList<TYPE, ARG_TYPE>::GetHeadPosition() const
    { return (SPLISTPOS) m_pNodeHead; }
template<class TYPE, class ARG_TYPE>
inline SPLISTPOS CSPList<TYPE, ARG_TYPE>::GetTailPosition() const
    { return (SPLISTPOS) m_pNodeTail; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPList<TYPE, ARG_TYPE>::GetNext(SPLISTPOS& rPosition) // return *Position++
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (SPLISTPOS) pNode->pNext;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CSPList<TYPE, ARG_TYPE>::GetNext(SPLISTPOS& rPosition) const // return *Position++
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (SPLISTPOS) pNode->pNext;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPList<TYPE, ARG_TYPE>::GetPrev(SPLISTPOS& rPosition) // return *Position--
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (SPLISTPOS) pNode->pPrev;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CSPList<TYPE, ARG_TYPE>::GetPrev(SPLISTPOS& rPosition) const // return *Position--
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (SPLISTPOS) pNode->pPrev;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CSPList<TYPE, ARG_TYPE>::GetAt(SPLISTPOS position)
    {   CNode* pNode = (CNode*) position;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CSPList<TYPE, ARG_TYPE>::GetAt(SPLISTPOS position) const
    {   CNode* pNode = (CNode*) position;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline void CSPList<TYPE, ARG_TYPE>::SetAt(SPLISTPOS pos, ARG_TYPE newElement)
    {   CNode* pNode = (CNode*) pos;
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        pNode->data = newElement; }

/////////////////////////////////////////////////////////////////////////////
// CSPList<TYPE, ARG_TYPE> out-of-line functions

template<class TYPE, class ARG_TYPE>
CSPList<TYPE, ARG_TYPE>::CSPList( int nBlockSize )
{
    _ASSERT( nBlockSize > 0 );

    m_nCount = 0;
    m_pNodeHead = m_pNodeTail = m_pNodeFree = NULL;
    m_pBlocks = NULL;
    m_nBlockSize = nBlockSize;
}

template<class TYPE, class ARG_TYPE>
void CSPList<TYPE, ARG_TYPE>::RemoveAll()
{
    SPASSERT_VALID( this );

    // destroy elements
    CNode* pNode;
    for (pNode = m_pNodeHead; pNode != NULL; pNode = pNode->pNext)
        SPDestructElements(&pNode->data, 1);

    m_nCount = 0;
    m_pNodeHead = m_pNodeTail = m_pNodeFree = NULL;
    
    if (m_pBlocks != NULL)
    {
        m_pBlocks->FreeDataChain();
        m_pBlocks = NULL;
    }
}

template<class TYPE, class ARG_TYPE>
CSPList<TYPE, ARG_TYPE>::~CSPList()
{
    RemoveAll();
    _ASSERT( m_nCount == 0 );
}

/////////////////////////////////////////////////////////////////////////////
// Node helpers
//
// Implementation note: CNode's are stored in CSPPlex blocks and
//  chained together. Free blocks are maintained in a singly linked list
//  using the 'pNext' member of CNode with 'm_pNodeFree' as the head.
//  Used blocks are maintained in a doubly linked list using both 'pNext'
//  and 'pPrev' as links and 'm_pNodeHead' and 'm_pNodeTail'
//   as the head/tail.
//
// We never free a CSPPlex block unless the List is destroyed or RemoveAll()
//  is used - so the total number of CSPPlex blocks may grow large depending
//  on the maximum past size of the list.
//

template<class TYPE, class ARG_TYPE>
SPLISTPOS CSPList<TYPE, ARG_TYPE>::AddHead(ARG_TYPE newElement)
{
    SPASSERT_VALID( this );

    CNode* pNewNode = NewNode(NULL, m_pNodeHead);
    if( pNewNode )
    {
        pNewNode->data = newElement;
        if (m_pNodeHead != NULL)
            m_pNodeHead->pPrev = pNewNode;
        else
            m_pNodeTail = pNewNode;
        m_pNodeHead = pNewNode;
    }
    return (SPLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
SPLISTPOS CSPList<TYPE, ARG_TYPE>::AddTail(ARG_TYPE newElement)
{
    SPASSERT_VALID( this );

    CNode* pNewNode = NewNode(m_pNodeTail, NULL);
    if( pNewNode )
    {
        pNewNode->data = newElement;
        if (m_pNodeTail != NULL)
            m_pNodeTail->pNext = pNewNode;
        else
            m_pNodeHead = pNewNode;
        m_pNodeTail = pNewNode;
    }
    return (SPLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
HRESULT CSPList<TYPE, ARG_TYPE>::AddHead(CSPList* pNewList)
{
    SPASSERT_VALID( this );
    SPASSERT_VALID( pNewList );

    // add a list of same elements to head (maintain order)
    SPLISTPOS pos = pNewList->GetTailPosition();
    HRESULT hr = S_OK;
    while (pos != NULL && SUCCEEDED(hr))
    {
        if (AddHead(pNewList->GetPrev(pos)) == NULL)
        {
            hr = E_OUTOFMEMORY;
        }
    }

    return hr;
}

template<class TYPE, class ARG_TYPE>
HRESULT CSPList<TYPE, ARG_TYPE>::AddTail(CSPList* pNewList)
{
    SPASSERT_VALID( this );
    SPASSERT_VALID( pNewList );

    // add a list of same elements
    SPLISTPOS pos = pNewList->GetHeadPosition();
    HRESULT hr = S_OK;
    while (pos != NULL && SUCCEEDED(hr))
    {
        if (AddTail(pNewList->GetNext(pos)) == NULL)
        {
            hr = E_OUTOFMEMORY;
        }
    }

    return hr;
}

template<class TYPE, class ARG_TYPE>
TYPE CSPList<TYPE, ARG_TYPE>::RemoveHead()
{
    SPASSERT_VALID( this );
    _ASSERT( m_pNodeHead != NULL );  // don't call on empty list !!!
    _ASSERT( SPIsValidAddress(m_pNodeHead, sizeof(CNode), TRUE ) );

    CNode* pOldNode = m_pNodeHead;
    TYPE returnValue = pOldNode->data;

    m_pNodeHead = pOldNode->pNext;
    if (m_pNodeHead != NULL)
        m_pNodeHead->pPrev = NULL;
    else
        m_pNodeTail = NULL;
    FreeNode(pOldNode);
    return returnValue;
}

template<class TYPE, class ARG_TYPE>
TYPE CSPList<TYPE, ARG_TYPE>::RemoveTail()
{
    SPASSERT_VALID( this );
    _ASSERT( m_pNodeTail != NULL );  // don't call on empty list !!!
    _ASSERT( SPIsValidAddress(m_pNodeTail, sizeof(CNode), TRUE ) );

    CNode* pOldNode = m_pNodeTail;
    TYPE returnValue = pOldNode->data;

    m_pNodeTail = pOldNode->pPrev;
    if (m_pNodeTail != NULL)
        m_pNodeTail->pNext = NULL;
    else
        m_pNodeHead = NULL;
    FreeNode(pOldNode);
    return returnValue;
}

template<class TYPE, class ARG_TYPE>
SPLISTPOS CSPList<TYPE, ARG_TYPE>::InsertBefore(SPLISTPOS position, ARG_TYPE newElement)
{
    SPASSERT_VALID( this );

    if (position == NULL)
        return AddHead(newElement); // insert before nothing -> head of the list

    // Insert it before position
    CNode* pOldNode = (CNode*) position;
    CNode* pNewNode = NewNode(pOldNode->pPrev, pOldNode);
    if( pNewNode )
    {
        pNewNode->data = newElement;

        if (pOldNode->pPrev != NULL)
        {
            _ASSERT( SPIsValidAddress(pOldNode->pPrev, sizeof(CNode), TRUE ) );
            pOldNode->pPrev->pNext = pNewNode;
        }
        else
        {
            _ASSERT( pOldNode == m_pNodeHead );
            m_pNodeHead = pNewNode;
        }
        pOldNode->pPrev = pNewNode;
    }
    return (SPLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
SPLISTPOS CSPList<TYPE, ARG_TYPE>::InsertAfter(SPLISTPOS position, ARG_TYPE newElement)
{
    SPASSERT_VALID( this );

    if (position == NULL)
        return AddTail(newElement); // insert after nothing -> tail of the list

    // Insert it before position
    CNode* pOldNode = (CNode*) position;
    _ASSERT( SPIsValidAddress(pOldNode, sizeof(CNode), TRUE ));
    CNode* pNewNode = NewNode(pOldNode, pOldNode->pNext);
    if( pNewNode )
    {
        pNewNode->data = newElement;

        if (pOldNode->pNext != NULL)
        {
            _ASSERT( SPIsValidAddress(pOldNode->pNext, sizeof(CNode), TRUE ));
            pOldNode->pNext->pPrev = pNewNode;
        }
        else
        {
            _ASSERT( pOldNode == m_pNodeTail );
            m_pNodeTail = pNewNode;
        }
        pOldNode->pNext = pNewNode;
    }
    return (SPLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
void CSPList<TYPE, ARG_TYPE>::RemoveAt(SPLISTPOS position)
{
    SPASSERT_VALID( this );

    CNode* pOldNode = (CNode*) position;
    _ASSERT( SPIsValidAddress(pOldNode, sizeof(CNode), TRUE ) );

    // remove pOldNode from list
    if (pOldNode == m_pNodeHead)
    {
        m_pNodeHead = pOldNode->pNext;
    }
    else
    {
        _ASSERT( SPIsValidAddress(pOldNode->pPrev, sizeof(CNode), TRUE ) );
        pOldNode->pPrev->pNext = pOldNode->pNext;
    }
    if (pOldNode == m_pNodeTail)
    {
        m_pNodeTail = pOldNode->pPrev;
    }
    else
    {
        _ASSERT( SPIsValidAddress(pOldNode->pNext, sizeof(CNode), TRUE ) );
        pOldNode->pNext->pPrev = pOldNode->pPrev;
    }
    FreeNode(pOldNode);
}

template<class TYPE, class ARG_TYPE>
SPLISTPOS CSPList<TYPE, ARG_TYPE>::FindIndex(int nIndex) const
{
    SPASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );

    if (nIndex >= m_nCount)
        return NULL;  // went too far

    CNode* pNode = m_pNodeHead;
    while (nIndex--)
    {
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ));
        pNode = pNode->pNext;
    }
    return (SPLISTPOS) pNode;
}

template<class TYPE, class ARG_TYPE>
SPLISTPOS CSPList<TYPE, ARG_TYPE>::Find(ARG_TYPE searchValue, SPLISTPOS startAfter) const
{
    SPASSERT_VALID( this );

    CNode* pNode = (CNode*) startAfter;
    if (pNode == NULL)
    {
        pNode = m_pNodeHead;  // start at head
    }
    else
    {
        _ASSERT( SPIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        pNode = pNode->pNext;  // start after the one specified
    }

    for (; pNode != NULL; pNode = pNode->pNext)
        if (SPCompareElements(&pNode->data, &searchValue))
            return (SPLISTPOS)pNode;
    return NULL;
}

#ifdef _DEBUG
template<class TYPE, class ARG_TYPE>
void CSPList<TYPE, ARG_TYPE>::AssertValid() const
{
    if (m_nCount == 0)
    {
        // empty list
        _ASSERT( m_pNodeHead == NULL );
        _ASSERT( m_pNodeTail == NULL );
    }
    else
    {
        // non-empty list
        _ASSERT( SPIsValidAddress(m_pNodeHead, sizeof(CNode), TRUE ));
        _ASSERT( SPIsValidAddress(m_pNodeTail, sizeof(CNode), TRUE ));
    }
}
#endif //_DEBUG



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //--- This must be the last line in the file
