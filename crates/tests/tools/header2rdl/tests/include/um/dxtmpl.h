//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  File:       dxtmpl.h
//
//  Description:
//      This is the header file contains the DX collection class templates. It
//  has been derived from the MFC collection templates for compatibility.
//
//----------------------------------------------------------------------------

#ifndef DXTmpl_h
#define DXTmpl_h

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _INC_LIMITS
#include <limits.h>
#endif

#ifndef _INC_STRING
#include <string.h>
#endif

#ifndef _INC_STDLIB
#include <stdlib.h>
#endif

#ifndef _INC_SEARCH
#include <search.h>
#endif

#define DXASSERT_VALID( pObj )

#ifndef PASCAL_INLINE
#ifdef _M_CEE_PURE
#define PASCAL_INLINE  __clrcall
#else
#define PASCAL_INLINE  PASCAL
#endif
#endif

/////////////////////////////////////////////////////////////////////////////
typedef void* DXLISTPOS;
typedef DWORD DXLISTHANDLE;

#define DX_BEFORE_START_POSITION ((void*)-1L)

inline BOOL DXIsValidAddress(const void* lp, UINT nBytes, BOOL bReadWrite)
{
    // simple version using Win-32 APIs for pointer validation.
    return (lp != NULL && !IsBadReadPtr(lp, nBytes) &&
        (!bReadWrite || !IsBadWritePtr((LPVOID)lp, nBytes)));
}

/////////////////////////////////////////////////////////////////////////////
// global helpers (can be overridden)
template<class TYPE>
inline void DXConstructElements(TYPE* pElements, int nCount)
{
    _ASSERT( nCount == 0 ||
             DXIsValidAddress( pElements, nCount * sizeof(TYPE), TRUE ) );

    // default is bit-wise zero initialization
    memset((void*)pElements, 0, nCount * sizeof(TYPE));
}

template<class TYPE>
inline void DXDestructElements(TYPE* pElements, int nCount)
{
    _ASSERT( ( nCount == 0 ||
               DXIsValidAddress( pElements, nCount * sizeof(TYPE), TRUE  ) ) );
    (void)pElements;  // not used
    (void)nCount; // not used

    // default does nothing
}

template<class TYPE>
inline void DXCopyElements(TYPE* pDest, const TYPE* pSrc, int nCount)
{
    _ASSERT( ( nCount == 0 ||
               DXIsValidAddress( pDest, nCount * sizeof(TYPE), TRUE  )) );
    _ASSERT( ( nCount == 0 ||
               DXIsValidAddress( pSrc, nCount * sizeof(TYPE), FALSE  )) );

    // default is bit-wise copy
    memcpy(pDest, pSrc, nCount * sizeof(TYPE));
}

template<class TYPE, class ARG_TYPE>
BOOL DXCompareElements(const TYPE* pElement1, const ARG_TYPE* pElement2)
{
    _ASSERT( DXIsValidAddress( pElement1, sizeof(TYPE), FALSE ) );
    _ASSERT( DXIsValidAddress( pElement2, sizeof(ARG_TYPE), FALSE ) );
    return *pElement1 == *pElement2;
}

template<class ARG_KEY>
inline UINT DXHashKey(ARG_KEY key)
{
    // default identity hash - works for most primitive values
    return ((UINT)(void*)(DWORD)key) >> 4;
}

/////////////////////////////////////////////////////////////////////////////
// CDXPlex

struct CDXPlex    // warning variable length structure
{
    CDXPlex* pNext;
    UINT nMax;
    UINT nCur;
    /* BYTE data[maxNum*elementSize]; */
    void* data() { return this+1; }

    static CDXPlex* PASCAL_INLINE Create( CDXPlex*& pHead, UINT nMax, UINT cbElement )
    {
	if ((SIZE_T)(nMax * cbElement) > (SIZE_MAX - sizeof(CDXPlex)))
	    return NULL;
        CDXPlex* p = (CDXPlex*) new BYTE[sizeof(CDXPlex) + nMax * cbElement];
        if (p == NULL)
            return NULL;
        p->nMax = nMax;
        p->nCur = 0;
        p->pNext = pHead;
        pHead = p;  // change head (adds in reverse order for simplicity)
        return p;
    }

    void FreeDataChain()
    {
        CDXPlex* p = this;
        while (p != NULL)
        {
            BYTE* bytes = (BYTE*) p;
            CDXPlex* pNext = p->pNext;
            delete [] bytes;
            p = pNext;
        }
    }
};


/////////////////////////////////////////////////////////////////////////////
// CDXArray<TYPE, ARG_TYPE>

template<class TYPE, class ARG_TYPE>
class CDXArray
{
public:
// Construction
    CDXArray();

// Attributes
    int GetSize() const;
    int GetUpperBound() const;
    void SetSize(int nNewSize, int nGrowBy = -1);

// Operations
    // Clean up
    void FreeExtra();
    void RemoveAll();

    // Accessing elements
    TYPE GetAt(int nIndex) const;
    void SetAt(int nIndex, ARG_TYPE newElement);
    TYPE& ElementAt(int nIndex);

    // Direct Access to the element data (may return NULL)
    const TYPE* GetData() const;
    TYPE* GetData();

    // Potentially growing the array
    void SetAtGrow(int nIndex, ARG_TYPE newElement);
    int Add(ARG_TYPE newElement);
    int Append(const CDXArray& src);
    void Copy(const CDXArray& src);

    // overloaded operator helpers
    TYPE operator[](int nIndex) const;
    TYPE& operator[](int nIndex);

    // Operations that move elements around
    void InsertAt(int nIndex, ARG_TYPE newElement, int nCount = 1);
    void RemoveAt(int nIndex, int nCount = 1);
    void InsertAt(int nStartIndex, CDXArray* pNewArray);
    void Sort(int (__cdecl *compare )(const void *elem1, const void *elem2 ));

// Implementation
protected:
    TYPE* m_pData;   // the actual array of data
    int m_nSize;     // # of elements (upperBound - 1)
    int m_nMaxSize;  // max allocated
    int m_nGrowBy;   // grow amount

public:
    ~CDXArray();
#ifdef _DEBUG
//  void Dump(CDumpContext&) const;
    void AssertValid() const;
#endif
};

/////////////////////////////////////////////////////////////////////////////
// CDXArray<TYPE, ARG_TYPE> inline functions

template<class TYPE, class ARG_TYPE>
inline int CDXArray<TYPE, ARG_TYPE>::GetSize() const
    { return m_nSize; }
template<class TYPE, class ARG_TYPE>
inline int CDXArray<TYPE, ARG_TYPE>::GetUpperBound() const
    { return m_nSize-1; }
template<class TYPE, class ARG_TYPE>
inline void CDXArray<TYPE, ARG_TYPE>::RemoveAll()
    { SetSize(0, -1); }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXArray<TYPE, ARG_TYPE>::GetAt(int nIndex) const
    { _ASSERT( (nIndex >= 0 && nIndex < m_nSize) );
        return m_pData[nIndex]; }
template<class TYPE, class ARG_TYPE>
inline void CDXArray<TYPE, ARG_TYPE>::SetAt(int nIndex, ARG_TYPE newElement)
    { _ASSERT( (nIndex >= 0 && nIndex < m_nSize) );
        m_pData[nIndex] = newElement; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXArray<TYPE, ARG_TYPE>::ElementAt(int nIndex)
    { _ASSERT( (nIndex >= 0 && nIndex < m_nSize) );
        return m_pData[nIndex]; }
template<class TYPE, class ARG_TYPE>
inline const TYPE* CDXArray<TYPE, ARG_TYPE>::GetData() const
    { return (const TYPE*)m_pData; }
template<class TYPE, class ARG_TYPE>
inline TYPE* CDXArray<TYPE, ARG_TYPE>::GetData()
    { return (TYPE*)m_pData; }
template<class TYPE, class ARG_TYPE>
inline int CDXArray<TYPE, ARG_TYPE>::Add(ARG_TYPE newElement)
    { int nIndex = m_nSize;
        SetAtGrow(nIndex, newElement);
        return nIndex; }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXArray<TYPE, ARG_TYPE>::operator[](int nIndex) const
    { return GetAt(nIndex); }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXArray<TYPE, ARG_TYPE>::operator[](int nIndex)
    { return ElementAt(nIndex); }

/////////////////////////////////////////////////////////////////////////////
// CDXArray<TYPE, ARG_TYPE> out-of-line functions

template<class TYPE, class ARG_TYPE>
CDXArray<TYPE, ARG_TYPE>::CDXArray()
{
    m_pData = NULL;
    m_nSize = m_nMaxSize = m_nGrowBy = 0;
}

template<class TYPE, class ARG_TYPE>
CDXArray<TYPE, ARG_TYPE>::~CDXArray()
{
    DXASSERT_VALID( this );

    if (m_pData != NULL)
    {
        DXDestructElements(m_pData, m_nSize);
        delete[] (BYTE*)m_pData;
    }
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::SetSize(int nNewSize, int nGrowBy)
{
    DXASSERT_VALID( this );
    _ASSERT( nNewSize >= 0 );

    if (nGrowBy != -1)
        m_nGrowBy = nGrowBy;  // set new size

    if (nNewSize == 0)
    {
        // shrink to nothing
        if (m_pData != NULL)
        {
            DXDestructElements(m_pData, m_nSize);
            delete[] (BYTE*)m_pData;
            m_pData = NULL;
        }
        m_nSize = m_nMaxSize = 0;
    }
    else if (m_pData == NULL)
    {
        // create one with exact size
#ifdef SIZE_T_MAX
        _ASSERT( nNewSize <= SIZE_T_MAX/sizeof(TYPE) );    // no overflow
#endif
        m_pData = (TYPE*) new BYTE[nNewSize * sizeof(TYPE)];
        DXConstructElements(m_pData, nNewSize);
        m_nSize = m_nMaxSize = nNewSize;
    }
    else if (nNewSize <= m_nMaxSize)
    {
        // it fits
        if (nNewSize > m_nSize)
        {
            // initialize the new elements
            DXConstructElements(&m_pData[m_nSize], nNewSize-m_nSize);
        }
        else if (m_nSize > nNewSize)
        {
            // destroy the old elements
            DXDestructElements(&m_pData[nNewSize], m_nSize-nNewSize);
        }
        m_nSize = nNewSize;
    }
    else
    {
        // otherwise, grow array
        int nGrowBy = m_nGrowBy;
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
        _ASSERT( nNewMax <= SIZE_T_MAX/sizeof(TYPE) ); // no overflow
#endif
        TYPE* pNewData = (TYPE*) new BYTE[nNewMax * sizeof(TYPE)];

        // oh well, it's better than crashing
        if (pNewData == NULL)
            return;

        // copy new data from old
        memcpy(pNewData, m_pData, m_nSize * sizeof(TYPE));

        // construct remaining elements
        _ASSERT( nNewSize > m_nSize );
        DXConstructElements(&pNewData[m_nSize], nNewSize-m_nSize);

        // get rid of old stuff (note: no destructors called)
        delete[] (BYTE*)m_pData;
        m_pData = pNewData;
        m_nSize = nNewSize;
        m_nMaxSize = nNewMax;
    }
}

template<class TYPE, class ARG_TYPE>
int CDXArray<TYPE, ARG_TYPE>::Append(const CDXArray& src)
{
    DXASSERT_VALID( this );
    _ASSERT( this != &src );   // cannot append to itself

    int nOldSize = m_nSize;
    SetSize(m_nSize + src.m_nSize);
    DXCopyElements(m_pData + nOldSize, src.m_pData, src.m_nSize);
    return nOldSize;
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::Copy(const CDXArray& src)
{
    DXASSERT_VALID( this );
    _ASSERT( this != &src );   // cannot copy to itself

    SetSize(src.m_nSize);
    DXCopyElements(m_pData, src.m_pData, src.m_nSize);
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::FreeExtra()
{
    DXASSERT_VALID( this );

    if (m_nSize != m_nMaxSize)
    {
        // shrink to desired size
#ifdef SIZE_T_MAX
        _ASSERT( m_nSize <= SIZE_T_MAX/sizeof(TYPE)); // no overflow
#endif
        TYPE* pNewData = NULL;
        if (m_nSize != 0)
        {
            pNewData = (TYPE*) new BYTE[m_nSize * sizeof(TYPE)];

            // oh well, it's better than crashing
            if (pNewData == NULL)
                return;

            // copy new data from old
            memcpy(pNewData, m_pData, m_nSize * sizeof(TYPE));
        }

        // get rid of old stuff (note: no destructors called)
        delete[] (BYTE*)m_pData;
        m_pData = pNewData;
        m_nMaxSize = m_nSize;
    }
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::SetAtGrow(int nIndex, ARG_TYPE newElement)
{
    DXASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );

    if (nIndex >= m_nSize)
        SetSize(nIndex+1, -1);
    m_pData[nIndex] = newElement;
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::InsertAt(int nIndex, ARG_TYPE newElement, int nCount /*=1*/)
{
    DXASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );    // will expand to meet need
    _ASSERT( nCount > 0 );     // zero or negative size not allowed

    if (nIndex >= m_nSize)
    {
        // adding after the end of the array
        SetSize(nIndex + nCount, -1);   // grow so nIndex is valid
    }
    else
    {
        // inserting in the middle of the array
        int nOldSize = m_nSize;
        SetSize(m_nSize + nCount, -1);  // grow it to new size
        // shift old data up to fill gap
        memmove(&m_pData[nIndex+nCount], &m_pData[nIndex],
            (nOldSize-nIndex) * sizeof(TYPE));

        // re-init slots we copied from
        DXConstructElements(&m_pData[nIndex], nCount);
    }

    // insert new value in the gap
    _ASSERT( nIndex + nCount <= m_nSize );
    while (nCount--)
        m_pData[nIndex++] = newElement;
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::RemoveAt(int nIndex, int nCount)
{
    DXASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );
    _ASSERT( nCount >= 0 );
    _ASSERT( nIndex + nCount <= m_nSize );

    // just remove a range
    int nMoveCount = m_nSize - (nIndex + nCount);
    DXDestructElements(&m_pData[nIndex], nCount);
    if (nMoveCount)
        memcpy(&m_pData[nIndex], &m_pData[nIndex + nCount],
            nMoveCount * sizeof(TYPE));
    m_nSize -= nCount;
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::InsertAt(int nStartIndex, CDXArray* pNewArray)
{
    DXASSERT_VALID( this );
    DXASSERT_VALID( pNewArray );
    _ASSERT( nStartIndex >= 0 );

    if (pNewArray->GetSize() > 0)
    {
        InsertAt(nStartIndex, pNewArray->GetAt(0), pNewArray->GetSize());
        for (int i = 0; i < pNewArray->GetSize(); i++)
            SetAt(nStartIndex + i, pNewArray->GetAt(i));
    }
}

template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::Sort(int (__cdecl *compare )(const void *elem1, const void *elem2 ))
{
    DXASSERT_VALID( this );
    _ASSERT( m_pData != NULL );

    qsort( m_pData, m_nSize, sizeof(TYPE), compare );
}

#ifdef _DEBUG
template<class TYPE, class ARG_TYPE>
void CDXArray<TYPE, ARG_TYPE>::AssertValid() const
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
        _ASSERT( DXIsValidAddress(m_pData, m_nMaxSize * sizeof(TYPE), TRUE ) );
    }
}
#endif //_DEBUG

/////////////////////////////////////////////////////////////////////////////
// CDXList<TYPE, ARG_TYPE>

template<class TYPE, class ARG_TYPE>
class CDXList
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
    CDXList(int nBlockSize = 10);

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
    DXLISTPOS AddHead(ARG_TYPE newElement);
    DXLISTPOS AddTail(ARG_TYPE newElement);

    // add another list of elements before head or after tail
    void AddHead(CDXList* pNewList);
    void AddTail(CDXList* pNewList);

    // remove all elements
    void RemoveAll();

    // iteration
    DXLISTPOS GetHeadPosition() const;
    DXLISTPOS GetTailPosition() const;
    TYPE& GetNext(DXLISTPOS& rPosition); // return *Position++
    TYPE GetNext(DXLISTPOS& rPosition) const; // return *Position++
    TYPE& GetPrev(DXLISTPOS& rPosition); // return *Position--
    TYPE GetPrev(DXLISTPOS& rPosition) const; // return *Position--

    // getting/modifying an element at a given position
    TYPE& GetAt(DXLISTPOS position);
    TYPE GetAt(DXLISTPOS position) const;
    void SetAt(DXLISTPOS pos, ARG_TYPE newElement);
    void RemoveAt(DXLISTPOS position);

    // inserting before or after a given position
    DXLISTPOS InsertBefore(DXLISTPOS position, ARG_TYPE newElement);
    DXLISTPOS InsertAfter(DXLISTPOS position, ARG_TYPE newElement);

    // helper functions (note: O(n) speed)
    DXLISTPOS Find(ARG_TYPE searchValue, DXLISTPOS startAfter = NULL) const;
        // defaults to starting at the HEAD, return NULL if not found
    DXLISTPOS FindIndex(int nIndex) const;
        // get the 'nIndex'th element (may return NULL)

// Implementation
protected:
    CNode* m_pNodeHead;
    CNode* m_pNodeTail;
    int m_nCount;
    CNode* m_pNodeFree;
    struct CDXPlex* m_pBlocks;
    int m_nBlockSize;

    CNode* NewNode(CNode*, CNode*);
    void FreeNode(CNode*);

public:
    ~CDXList();
#ifdef _DEBUG
    void AssertValid() const;
#endif
};

/////////////////////////////////////////////////////////////////////////////
// CDXList<TYPE, ARG_TYPE> inline functions

template<class TYPE, class ARG_TYPE>
inline int CDXList<TYPE, ARG_TYPE>::GetCount() const
    { return m_nCount; }
template<class TYPE, class ARG_TYPE>
inline BOOL CDXList<TYPE, ARG_TYPE>::IsEmpty() const
    { return m_nCount == 0; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXList<TYPE, ARG_TYPE>::GetHead()
    {   _ASSERT( m_pNodeHead != NULL );
        return m_pNodeHead->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXList<TYPE, ARG_TYPE>::GetHead() const
    {   _ASSERT( m_pNodeHead != NULL );
        return m_pNodeHead->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXList<TYPE, ARG_TYPE>::GetTail()
    {   _ASSERT( m_pNodeTail != NULL );
        return m_pNodeTail->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXList<TYPE, ARG_TYPE>::GetTail() const
    {   _ASSERT( m_pNodeTail != NULL );
        return m_pNodeTail->data; }
template<class TYPE, class ARG_TYPE>
inline DXLISTPOS CDXList<TYPE, ARG_TYPE>::GetHeadPosition() const
    { return (DXLISTPOS) m_pNodeHead; }
template<class TYPE, class ARG_TYPE>
inline DXLISTPOS CDXList<TYPE, ARG_TYPE>::GetTailPosition() const
    { return (DXLISTPOS) m_pNodeTail; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXList<TYPE, ARG_TYPE>::GetNext(DXLISTPOS& rPosition) // return *Position++
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (DXLISTPOS) pNode->pNext;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXList<TYPE, ARG_TYPE>::GetNext(DXLISTPOS& rPosition) const // return *Position++
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (DXLISTPOS) pNode->pNext;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXList<TYPE, ARG_TYPE>::GetPrev(DXLISTPOS& rPosition) // return *Position--
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (DXLISTPOS) pNode->pPrev;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXList<TYPE, ARG_TYPE>::GetPrev(DXLISTPOS& rPosition) const // return *Position--
    {   CNode* pNode = (CNode*) rPosition;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        rPosition = (DXLISTPOS) pNode->pPrev;
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE& CDXList<TYPE, ARG_TYPE>::GetAt(DXLISTPOS position)
    {   CNode* pNode = (CNode*) position;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline TYPE CDXList<TYPE, ARG_TYPE>::GetAt(DXLISTPOS position) const
    {   CNode* pNode = (CNode*) position;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        return pNode->data; }
template<class TYPE, class ARG_TYPE>
inline void CDXList<TYPE, ARG_TYPE>::SetAt(DXLISTPOS pos, ARG_TYPE newElement)
    {   CNode* pNode = (CNode*) pos;
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        pNode->data = newElement; }

/////////////////////////////////////////////////////////////////////////////
// CDXList<TYPE, ARG_TYPE> out-of-line functions

template<class TYPE, class ARG_TYPE>
CDXList<TYPE, ARG_TYPE>::CDXList( int nBlockSize )
{
    _ASSERT( nBlockSize > 0 );

    m_nCount = 0;
    m_pNodeHead = m_pNodeTail = m_pNodeFree = NULL;
    m_pBlocks = NULL;
    m_nBlockSize = nBlockSize;
}

template<class TYPE, class ARG_TYPE>
void CDXList<TYPE, ARG_TYPE>::RemoveAll()
{
    DXASSERT_VALID( this );

    // destroy elements
    CNode* pNode;
    for (pNode = m_pNodeHead; pNode != NULL; pNode = pNode->pNext)
        DXDestructElements(&pNode->data, 1);

    m_nCount = 0;
    m_pNodeHead = m_pNodeTail = m_pNodeFree = NULL;
    m_pBlocks->FreeDataChain();
    m_pBlocks = NULL;
}

template<class TYPE, class ARG_TYPE>
CDXList<TYPE, ARG_TYPE>::~CDXList()
{
    RemoveAll();
    _ASSERT( m_nCount == 0 );
}

/////////////////////////////////////////////////////////////////////////////
// Node helpers
//
// Implementation note: CNode's are stored in CDXPlex blocks and
//  chained together. Free blocks are maintained in a singly linked list
//  using the 'pNext' member of CNode with 'm_pNodeFree' as the head.
//  Used blocks are maintained in a doubly linked list using both 'pNext'
//  and 'pPrev' as links and 'm_pNodeHead' and 'm_pNodeTail'
//   as the head/tail.
//
// We never free a CDXPlex block unless the List is destroyed or RemoveAll()
//  is used - so the total number of CDXPlex blocks may grow large depending
//  on the maximum past size of the list.
//

template<class TYPE, class ARG_TYPE>
typename CDXList<TYPE, ARG_TYPE>::CNode*
CDXList<TYPE, ARG_TYPE>::NewNode(CNode* pPrev, CNode* pNext)
{
    if (m_pNodeFree == NULL)
    {
        // add another block
        CDXPlex* pNewBlock = CDXPlex::Create(m_pBlocks, m_nBlockSize,
                 sizeof(CNode));

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
    _ASSERT( m_pNodeFree != NULL );  // we must have something

    CDXList::CNode* pNode = m_pNodeFree;
    m_pNodeFree = m_pNodeFree->pNext;
    pNode->pPrev = pPrev;
    pNode->pNext = pNext;
    m_nCount++;
    _ASSERT( m_nCount > 0 );  // make sure we don't overflow

    DXConstructElements(&pNode->data, 1);
    return pNode;
}

template<class TYPE, class ARG_TYPE>
void CDXList<TYPE, ARG_TYPE>::FreeNode(CNode* pNode)
{
    DXDestructElements(&pNode->data, 1);
    pNode->pNext = m_pNodeFree;
    m_pNodeFree = pNode;
    m_nCount--;
    _ASSERT( m_nCount >= 0 );  // make sure we don't underflow
}

template<class TYPE, class ARG_TYPE>
DXLISTPOS CDXList<TYPE, ARG_TYPE>::AddHead(ARG_TYPE newElement)
{
    DXASSERT_VALID( this );

    CNode* pNewNode = NewNode(NULL, m_pNodeHead);
    pNewNode->data = newElement;
    if (m_pNodeHead != NULL)
        m_pNodeHead->pPrev = pNewNode;
    else
        m_pNodeTail = pNewNode;
    m_pNodeHead = pNewNode;
    return (DXLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
DXLISTPOS CDXList<TYPE, ARG_TYPE>::AddTail(ARG_TYPE newElement)
{
    DXASSERT_VALID( this );

    CNode* pNewNode = NewNode(m_pNodeTail, NULL);
    pNewNode->data = newElement;
    if (m_pNodeTail != NULL)
        m_pNodeTail->pNext = pNewNode;
    else
        m_pNodeHead = pNewNode;
    m_pNodeTail = pNewNode;
    return (DXLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
void CDXList<TYPE, ARG_TYPE>::AddHead(CDXList* pNewList)
{
    DXASSERT_VALID( this );
    DXASSERT_VALID( pNewList );

    // add a list of same elements to head (maintain order)
    DXLISTPOS pos = pNewList->GetTailPosition();
    while (pos != NULL)
        AddHead(pNewList->GetPrev(pos));
}

template<class TYPE, class ARG_TYPE>
void CDXList<TYPE, ARG_TYPE>::AddTail(CDXList* pNewList)
{
    DXASSERT_VALID( this );
    DXASSERT_VALID( pNewList );

    // add a list of same elements
    DXLISTPOS pos = pNewList->GetHeadPosition();
    while (pos != NULL)
        AddTail(pNewList->GetNext(pos));
}

template<class TYPE, class ARG_TYPE>
TYPE CDXList<TYPE, ARG_TYPE>::RemoveHead()
{
    DXASSERT_VALID( this );
    _ASSERT( m_pNodeHead != NULL );  // don't call on empty list !!!
    _ASSERT( DXIsValidAddress(m_pNodeHead, sizeof(CNode), TRUE ) );

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
TYPE CDXList<TYPE, ARG_TYPE>::RemoveTail()
{
    DXASSERT_VALID( this );
    _ASSERT( m_pNodeTail != NULL );  // don't call on empty list !!!
    _ASSERT( DXIsValidAddress(m_pNodeTail, sizeof(CNode), TRUE ) );

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
DXLISTPOS CDXList<TYPE, ARG_TYPE>::InsertBefore(DXLISTPOS position, ARG_TYPE newElement)
{
    DXASSERT_VALID( this );

    if (position == NULL)
        return AddHead(newElement); // insert before nothing -> head of the list

    // Insert it before position
    CNode* pOldNode = (CNode*) position;
    CNode* pNewNode = NewNode(pOldNode->pPrev, pOldNode);
    pNewNode->data = newElement;

    if (pOldNode->pPrev != NULL)
    {
        _ASSERT( DXIsValidAddress(pOldNode->pPrev, sizeof(CNode), TRUE ) );
        pOldNode->pPrev->pNext = pNewNode;
    }
    else
    {
        _ASSERT( pOldNode == m_pNodeHead );
        m_pNodeHead = pNewNode;
    }
    pOldNode->pPrev = pNewNode;
    return (DXLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
DXLISTPOS CDXList<TYPE, ARG_TYPE>::InsertAfter(DXLISTPOS position, ARG_TYPE newElement)
{
    DXASSERT_VALID( this );

    if (position == NULL)
        return AddTail(newElement); // insert after nothing -> tail of the list

    // Insert it before position
    CNode* pOldNode = (CNode*) position;
    _ASSERT( DXIsValidAddress(pOldNode, sizeof(CNode), TRUE ));
    CNode* pNewNode = NewNode(pOldNode, pOldNode->pNext);
    pNewNode->data = newElement;

    if (pOldNode->pNext != NULL)
    {
        _ASSERT( DXIsValidAddress(pOldNode->pNext, sizeof(CNode), TRUE ));
        pOldNode->pNext->pPrev = pNewNode;
    }
    else
    {
        _ASSERT( pOldNode == m_pNodeTail );
        m_pNodeTail = pNewNode;
    }
    pOldNode->pNext = pNewNode;
    return (DXLISTPOS) pNewNode;
}

template<class TYPE, class ARG_TYPE>
void CDXList<TYPE, ARG_TYPE>::RemoveAt(DXLISTPOS position)
{
    DXASSERT_VALID( this );

    CNode* pOldNode = (CNode*) position;
    _ASSERT( DXIsValidAddress(pOldNode, sizeof(CNode), TRUE ) );

    // remove pOldNode from list
    if (pOldNode == m_pNodeHead)
    {
        m_pNodeHead = pOldNode->pNext;
    }
    else
    {
        _ASSERT( DXIsValidAddress(pOldNode->pPrev, sizeof(CNode), TRUE ) );
        pOldNode->pPrev->pNext = pOldNode->pNext;
    }
    if (pOldNode == m_pNodeTail)
    {
        m_pNodeTail = pOldNode->pPrev;
    }
    else
    {
        _ASSERT( DXIsValidAddress(pOldNode->pNext, sizeof(CNode), TRUE ) );
        pOldNode->pNext->pPrev = pOldNode->pPrev;
    }
    FreeNode(pOldNode);
}

template<class TYPE, class ARG_TYPE>
DXLISTPOS CDXList<TYPE, ARG_TYPE>::FindIndex(int nIndex) const
{
    DXASSERT_VALID( this );
    _ASSERT( nIndex >= 0 );

    if (nIndex >= m_nCount)
        return NULL;  // went too far

    CNode* pNode = m_pNodeHead;
    while (nIndex--)
    {
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ));
        pNode = pNode->pNext;
    }
    return (DXLISTPOS) pNode;
}

template<class TYPE, class ARG_TYPE>
DXLISTPOS CDXList<TYPE, ARG_TYPE>::Find(ARG_TYPE searchValue, DXLISTPOS startAfter) const
{
    DXASSERT_VALID( this );

    CNode* pNode = (CNode*) startAfter;
    if (pNode == NULL)
    {
        pNode = m_pNodeHead;  // start at head
    }
    else
    {
        _ASSERT( DXIsValidAddress(pNode, sizeof(CNode), TRUE ) );
        pNode = pNode->pNext;  // start after the one specified
    }

    for (; pNode != NULL; pNode = pNode->pNext)
        if (DXCompareElements(&pNode->data, &searchValue))
            return (DXLISTPOS)pNode;
    return NULL;
}

#ifdef _DEBUG
template<class TYPE, class ARG_TYPE>
void CDXList<TYPE, ARG_TYPE>::AssertValid() const
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
        _ASSERT( DXIsValidAddress(m_pNodeHead, sizeof(CNode), TRUE ));
        _ASSERT( DXIsValidAddress(m_pNodeTail, sizeof(CNode), TRUE ));
    }
}
#endif //_DEBUG

/////////////////////////////////////////////////////////////////////////////
// CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
class CDXMap
{
protected:
    // Association
    struct CAssoc
    {
        CAssoc* pNext;
        UINT nHashValue;  // needed for efficient iteration
        KEY key;
        VALUE value;
    };
public:
// Construction
    CDXMap( int nBlockSize = 10 );

// Attributes
    // number of elements
    int GetCount() const;
    BOOL IsEmpty() const;

    // Lookup
    BOOL Lookup(ARG_KEY key, VALUE& rValue) const;

// Operations
    // Lookup and add if not there
    VALUE& operator[](ARG_KEY key);

    // add a new (key, value) pair
    void SetAt(ARG_KEY key, ARG_VALUE newValue);

    // removing existing (key, ?) pair
    BOOL RemoveKey(ARG_KEY key);
    void RemoveAll();

    // iterating all (key, value) pairs
    DXLISTPOS GetStartPosition() const;
    void GetNextAssoc(DXLISTPOS& rNextPosition, KEY& rKey, VALUE& rValue) const;

    // advanced features for derived classes
    UINT GetHashTableSize() const;
    void InitHashTable(UINT hashSize, BOOL bAllocNow = TRUE);

// Implementation
protected:
    CAssoc** m_pHashTable;
    UINT m_nHashTableSize;
    int m_nCount;
    CAssoc* m_pFreeList;
    struct CDXPlex* m_pBlocks;
    int m_nBlockSize;

    CAssoc* NewAssoc();
    void FreeAssoc(CAssoc*);
    CAssoc* GetAssocAt(ARG_KEY, UINT&) const;

public:
    ~CDXMap();
#ifdef _DEBUG
//  void Dump(CDumpContext&) const;
    void AssertValid() const;
#endif
};

/////////////////////////////////////////////////////////////////////////////
// CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE> inline functions

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
inline int CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::GetCount() const
    { return m_nCount; }
template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
inline BOOL CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::IsEmpty() const
    { return m_nCount == 0; }
template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
inline void CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::SetAt(ARG_KEY key, ARG_VALUE newValue)
    { (*this)[key] = newValue; }
template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
inline DXLISTPOS CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::GetStartPosition() const
    { return (m_nCount == 0) ? NULL : DX_BEFORE_START_POSITION; }
template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
inline UINT CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::GetHashTableSize() const
    { return m_nHashTableSize; }

/////////////////////////////////////////////////////////////////////////////
// CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE> out-of-line functions

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::CDXMap( int nBlockSize )
{
    _ASSERT( nBlockSize > 0 );

    m_pHashTable = NULL;
    m_nHashTableSize = 17;  // default size
    m_nCount = 0;
    m_pFreeList = NULL;
    m_pBlocks = NULL;
    m_nBlockSize = nBlockSize;
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
void CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::InitHashTable(
    UINT nHashSize, BOOL bAllocNow)
//
// Used to force allocation of a hash table or to override the default
//   hash table size of (which is fairly small)
{
    DXASSERT_VALID( this );
    _ASSERT( m_nCount == 0 );
    _ASSERT( nHashSize > 0 );

    if (m_pHashTable != NULL)
    {
        // free hash table
        delete[] m_pHashTable;
        m_pHashTable = NULL;
    }

    if (bAllocNow)
    {
        m_pHashTable = new CAssoc* [nHashSize];

        // oh well, it's better than crashing
        if (m_pHashTable == NULL)
            return;

        memset(m_pHashTable, 0, sizeof(CAssoc*) * nHashSize);
    }
    m_nHashTableSize = nHashSize;
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
void CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::RemoveAll()
{
    DXASSERT_VALID( this );

    if (m_pHashTable != NULL)
    {
        // destroy elements (values and keys)
        for (UINT nHash = 0; nHash < m_nHashTableSize; nHash++)
        {
            CAssoc* pAssoc;
            for (pAssoc = m_pHashTable[nHash]; pAssoc != NULL;
              pAssoc = pAssoc->pNext)
            {
                DXDestructElements(&pAssoc->value, 1);
                DXDestructElements(&pAssoc->key, 1);
            }
        }
    }

    // free hash table
    delete[] m_pHashTable;
    m_pHashTable = NULL;

    m_nCount = 0;
    m_pFreeList = NULL;
    m_pBlocks->FreeDataChain();
    m_pBlocks = NULL;
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::~CDXMap()
{
    RemoveAll();
    _ASSERT( m_nCount == 0 );
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
typename CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::CAssoc*
CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::NewAssoc()
{
    if (m_pFreeList == NULL)
    {
        // add another block
        CDXPlex* newBlock = CDXPlex::Create(m_pBlocks, m_nBlockSize, sizeof(CDXMap::CAssoc));
        // chain them into free list
        CDXMap::CAssoc* pAssoc = (CDXMap::CAssoc*) newBlock->data();
        // free in reverse order to make it easier to debug
        pAssoc += m_nBlockSize - 1;
        for (int i = m_nBlockSize-1; i >= 0; i--, pAssoc--)
        {
            pAssoc->pNext = m_pFreeList;
            m_pFreeList = pAssoc;
        }
    }
    _ASSERT( m_pFreeList != NULL );  // we must have something

    CDXMap::CAssoc* pAssoc = m_pFreeList;
    m_pFreeList = m_pFreeList->pNext;
    m_nCount++;
    _ASSERT( m_nCount > 0 );  // make sure we don't overflow
    DXConstructElements(&pAssoc->key, 1);
    DXConstructElements(&pAssoc->value, 1);   // special construct values
    return pAssoc;
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
void CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::FreeAssoc(CAssoc* pAssoc)
{
    DXDestructElements(&pAssoc->value, 1);
    DXDestructElements(&pAssoc->key, 1);
    pAssoc->pNext = m_pFreeList;
    m_pFreeList = pAssoc;
    m_nCount--;
    _ASSERT( m_nCount >= 0 );  // make sure we don't underflow
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
typename CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::CAssoc*
CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::GetAssocAt(ARG_KEY key, UINT& nHash) const
// find association (or return NULL)
{
    nHash = DXHashKey(key) % m_nHashTableSize;

    if (m_pHashTable == NULL)
        return NULL;

    // see if it exists
    CAssoc* pAssoc;
    for (pAssoc = m_pHashTable[nHash]; pAssoc != NULL; pAssoc = pAssoc->pNext)
    {
        if (DXCompareElements(&pAssoc->key, &key))
            return pAssoc;
    }
    return NULL;
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
BOOL CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::Lookup(ARG_KEY key, VALUE& rValue) const
{
    DXASSERT_VALID( this );

    UINT nHash;
    CAssoc* pAssoc = GetAssocAt(key, nHash);
    if (pAssoc == NULL)
        return FALSE;  // not in map

    rValue = pAssoc->value;
    return TRUE;
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
VALUE& CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::operator[](ARG_KEY key)
{
    DXASSERT_VALID( this );

    UINT nHash;
    CAssoc* pAssoc;
    if ((pAssoc = GetAssocAt(key, nHash)) == NULL)
    {
        if (m_pHashTable == NULL)
            InitHashTable(m_nHashTableSize);

        // it doesn't exist, add a new Association
        pAssoc = NewAssoc();
        pAssoc->nHashValue = nHash;
        pAssoc->key = key;
        // 'pAssoc->value' is a constructed object, nothing more

        // put into hash table
        pAssoc->pNext = m_pHashTable[nHash];
        m_pHashTable[nHash] = pAssoc;
    }
    return pAssoc->value;  // return new reference
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
BOOL CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::RemoveKey(ARG_KEY key)
// remove key - return TRUE if removed
{
    DXASSERT_VALID( this );

    if (m_pHashTable == NULL)
        return FALSE;  // nothing in the table

    CAssoc** ppAssocPrev;
    ppAssocPrev = &m_pHashTable[DXHashKey(key) % m_nHashTableSize];

    CAssoc* pAssoc;
    for (pAssoc = *ppAssocPrev; pAssoc != NULL; pAssoc = pAssoc->pNext)
    {
        if (DXCompareElements(&pAssoc->key, &key))
        {
            // remove it
            *ppAssocPrev = pAssoc->pNext;  // remove from list
            FreeAssoc(pAssoc);
            return TRUE;
        }
        ppAssocPrev = &pAssoc->pNext;
    }
    return FALSE;  // not found
}

template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
void CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::GetNextAssoc(DXLISTPOS& rNextPosition,
    KEY& rKey, VALUE& rValue) const
{
    DXASSERT_VALID( this );
    _ASSERT( m_pHashTable != NULL );  // never call on empty map

    CAssoc* pAssocRet = (CAssoc*)rNextPosition;
    _ASSERT( pAssocRet != NULL );

    if (pAssocRet == (CAssoc*) DX_BEFORE_START_POSITION)
    {
        // find the first association
        for (UINT nBucket = 0; nBucket < m_nHashTableSize; nBucket++)
            if ((pAssocRet = m_pHashTable[nBucket]) != NULL)
                break;
        _ASSERT( pAssocRet != NULL );  // must find something
    }

    // find next association
    _ASSERT( DXIsValidAddress(pAssocRet, sizeof(CAssoc), TRUE ));
    CAssoc* pAssocNext;
    if ((pAssocNext = pAssocRet->pNext) == NULL)
    {
        // go to next bucket
        for (UINT nBucket = pAssocRet->nHashValue + 1;
          nBucket < m_nHashTableSize; nBucket++)
            if ((pAssocNext = m_pHashTable[nBucket]) != NULL)
                break;
    }

    rNextPosition = (DXLISTPOS) pAssocNext;

    // fill in return data
    rKey = pAssocRet->key;
    rValue = pAssocRet->value;
}

#ifdef _DEBUG
template<class KEY, class ARG_KEY, class VALUE, class ARG_VALUE>
void CDXMap<KEY, ARG_KEY, VALUE, ARG_VALUE>::AssertValid() const
{
    _ASSERT( m_nHashTableSize > 0 );
    _ASSERT( (m_nCount == 0 || m_pHashTable != NULL) );
        // non-empty map should have hash table
}
#endif //_DEBUG


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //--- This must be the last line in the file
