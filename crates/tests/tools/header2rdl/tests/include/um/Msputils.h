/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    MSPutils.h

Abstract:
    
    This file defines several utility classes used by the MSP base classes.

--*/

#ifndef __MSPUTILS_H_
#define __MSPUTILS_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <intsafe.h>
#include <crtdbg.h>
#define MSP_ASSERT _ASSERTE

#if _ATL_VER >= 0x0300

 //
 // ATL 3.0 contains an equivalent of DECLARE_VQI in its END_COM_MAP(), so 
 // DECLARE_VQI() is not needed
 //
 
 #define DECLARE_VQI()

#else

 #define DECLARE_VQI() \
    STDMETHOD(QueryInterface)(REFIID iid, void ** ppvObject) = 0; \
    STDMETHOD_(ULONG, AddRef)() = 0; \
    STDMETHOD_(ULONG, Release)() = 0;

#endif

//
// this macro expands to the appropriate MSP_x value, depending on hr.
// this is useful for logging. for instance, the statements:
//
//      .....
//
//      if (FAILED(hr))
//      {
//          LOG((MSP_ERROR, "MyClass::MyFunc - exit. hr = 0x%lx", hr));
//      }
//      else
//      {
//          LOG((MSP_TRACE, "MyClass::MyFunc - exit. hr = 0x%lx", hr));
//      }
//
//      return hr;
//  }
//
//  can be replaced with:
//  
//      ....
//
//      LOG((MSP_(hr), "MyClass::MyFunc - exit. hr = 0x%lx", hr));
//      
//      return hr;
//  }
//


#define MSP_(hr) (FAILED(hr)?MSP_ERROR:MSP_TRACE)

//
// return TRUE if the (possibly aggregated) media type that was passed in is valid.
//
// here is the criteria for a valid aggregated media type:
//
// 1. there is one or more bit set
// 2. all bits that are set match the possible media types
// 3. there are no set bits that don't correspond to valid meda types
//

inline BOOL IsValidAggregatedMediaType(DWORD dwAggregatedMediaType) 
{   

    //
    // these are all possible media types
    //

    const DWORD dwAllPossibleMediaTypes =  TAPIMEDIATYPE_AUDIO | 
                                        TAPIMEDIATYPE_VIDEO | 
                                        TAPIMEDIATYPE_DATAMODEM | 
                                        TAPIMEDIATYPE_G3FAX | 
                                        TAPIMEDIATYPE_MULTITRACK;

    


    //
    // return value
    //

    BOOL bValidMediaType = FALSE;


    //
    // make sure that there is at least one allowed media type 
    //
    // and
    //
    // there are no invalid media types
    //

    if (  (0 == (dwAggregatedMediaType &    dwAllPossibleMediaTypes )   )  ||      // any valid bits set
          (0 != (dwAggregatedMediaType &  (~dwAllPossibleMediaTypes))   )      )   // no invalid bits are set
    {

        //
        // the media type is invalid.
        //

        bValidMediaType = FALSE;
    }
    else
    {

        //
        // the media type is valid.
        //

        bValidMediaType = TRUE;

    }

    return bValidMediaType;
}


//
// Make sure we have exactly one media type. That's not the case if
// dwMediaType is 0 or more than one bit is set in dwMediaType. Note
// that DWORD is unsigned so this should be safe.
//
inline BOOL IsSingleMediaType(DWORD dwMediaType) 
{   
    return !((dwMediaType == 0) || ((dwMediaType & (dwMediaType - 1)) != 0));
}

//
// Check to see if the mediatype is a single type and is in the mask.
//

inline BOOL IsValidSingleMediaType(DWORD dwMediaType, DWORD dwMask)
{
    return IsSingleMediaType(dwMediaType)
        && ((dwMediaType & dwMask) == dwMediaType);
}

/*++

CMSPArray template Description:

    Definitions for a simple vector template. The implementaion is borrowed
    from CMSPArray in atlapp.h. Modified only the allocation behavior.

    This array should only be used to store simple types. It doesn't call the
    constructor nor the destructor for each element in the array.

--*/
const DWORD INITIAL = 8;
const DWORD DELTA   = 8;

template <class T, DWORD dwInitial = INITIAL, DWORD dwDelta = DELTA>
class CMSPArray
{

protected:
    T* m_aT;
    int m_nSize;
    int m_nAllocSize;

public:
// Construction/destruction
    CMSPArray() : m_aT(NULL), m_nSize(0), m_nAllocSize(0)
    { }

    ~CMSPArray()
    {
        RemoveAll();
    }

// Operations
    int GetSize() const
    {
        return m_nSize;
    }
    BOOL Grow()
    {
        T* aT;
        DWORD dwTmpSize, cbAllocSize;
        int nNewAllocSize;

        // get the new number of elements to allocate in nNewAllocSize
        if(m_nAllocSize == 0)
        {
            if(FAILED(DWordToInt(dwInitial, &nNewAllocSize)))
            {
                return FALSE;
            }
        }
        else
        {
            if(FAILED(IntToDWord(m_nSize, &dwTmpSize))
                 || FAILED(DWordAdd(dwTmpSize, DELTA, &dwTmpSize))
                 || FAILED(DWordToInt(dwTmpSize, &nNewAllocSize))
               )
            {
                return FALSE;
            }
        }
 
        if( FAILED(IntToDWord(nNewAllocSize, &dwTmpSize))
            || FAILED(DWordMult(dwTmpSize, sizeof(T), &cbAllocSize))
          )
        {
            return FALSE;
        }
        aT = (T*)realloc(m_aT, cbAllocSize);
        if(aT == NULL)
            return FALSE;
        m_nAllocSize = nNewAllocSize;
        m_aT = aT;
        return TRUE;
    }

    BOOL Add(T& t)
    {
        if(m_nSize == m_nAllocSize)
        {
            if (!Grow()) return FALSE;
        }
        m_nSize++;
        SetAtIndex(m_nSize - 1, t);
        return TRUE;
    }
    BOOL Remove(T& t)
    {
        int nIndex = Find(t);
        if(nIndex == -1)
            return FALSE;
        return RemoveAt(nIndex);
    }
    BOOL RemoveAt(int nIndex)
    {
        if(nIndex != (m_nSize - 1))
            memmove((void*)&m_aT[nIndex], (void*)&m_aT[nIndex + 1], 
                (m_nSize - (nIndex + 1)) * sizeof(T));
        m_nSize--;
        return TRUE;
    }
    void RemoveAll()
    {
        if(m_nAllocSize > 0)
        {
            free(m_aT);
            m_aT = NULL;
            m_nSize = 0;
            m_nAllocSize = 0;
        }
    }
    T& operator[] (int nIndex) const
    {
        MSP_ASSERT(nIndex >= 0 && nIndex < m_nSize);
        return m_aT[nIndex];
    }
    T* GetData() const
    {
        return m_aT;
    }

// Implementation
    void SetAtIndex(int nIndex, T& t)
    {
        MSP_ASSERT(nIndex >= 0 && nIndex < m_nSize);
        m_aT[nIndex] = t;
    }
    int Find(T& t) const
    {
        for(int i = 0; i < m_nSize; i++)
        {
            if(m_aT[i] == t)
                return i;
        }
        return -1;  // not found
    }
};

/*++

CMSPCritSection Description:

    Definitions for a auto initialize critical section.

--*/
class CMSPCritSection
{
private:
    CRITICAL_SECTION m_CritSec;

public:
    CMSPCritSection()
    {
        InitializeCriticalSection(&m_CritSec);
    }

    ~CMSPCritSection()
    {
        DeleteCriticalSection(&m_CritSec);
    }

    void Lock() 
    {
        EnterCriticalSection(&m_CritSec);
    }

    BOOL TryLock() 
    {
        return TryEnterCriticalSection(&m_CritSec);
    }

    void Unlock() 
    {
        LeaveCriticalSection(&m_CritSec);
    }
};


/*++

CMSPCritSection Description:

    Definitions for a auto lock that unlocks when the variable is out
    of scope.

--*/
class CLock
{
private:
    CMSPCritSection &m_CriticalSection;

public:
    CLock(CMSPCritSection &CriticalSection)
        : m_CriticalSection(CriticalSection)
    {
        m_CriticalSection.Lock();
    }

    ~CLock()
    {
        m_CriticalSection.Unlock();
    }
};



///////////////////////////////////////////////////////////////////////////////
//
// CCSLock
//
// a plain old automatic lock that takes a pointer to CRITICAL_SECTION
//
// constructore enters crit section, destructor leaves critical section
//
// class client is responsible for passing a valid critical section
//

class CCSLock
{

private:
    CRITICAL_SECTION *m_pCritSec;

public:
    CCSLock(CRITICAL_SECTION *pCritSec)
        : m_pCritSec(pCritSec)
    {
        EnterCriticalSection(m_pCritSec);
    }

    ~CCSLock()
    {
        LeaveCriticalSection(m_pCritSec);
    }
};



/*++

LINK list:

    Definitions for a double link list.

--*/

//
// Calculate the address of the base of the structure given its type, and an
// address of a field within the structure.
//
#ifndef CONTAINING_RECORD
#define CONTAINING_RECORD(address, type, field) \
    ((type *)((PCHAR)(address) - (ULONG_PTR)(&((type *)0)->field)))
#endif


#ifndef InitializeListHead
//
//  VOID
//  InitializeListHead(
//      PLIST_ENTRY ListHead
//      );
//

#define InitializeListHead(ListHead) (\
    (ListHead)->Flink = (ListHead)->Blink = (ListHead))

//
//  BOOLEAN
//  IsListEmpty(
//      PLIST_ENTRY ListHead
//      );
//

#define IsListEmpty(ListHead) \
    ((ListHead)->Flink == (ListHead))

//
//  PLIST_ENTRY
//  RemoveHeadList(
//      PLIST_ENTRY ListHead
//      );
//

#define RemoveHeadList(ListHead) \
    (ListHead)->Flink;\
    {RemoveEntryList((ListHead)->Flink)}

//
//  PLIST_ENTRY
//  RemoveTailList(
//      PLIST_ENTRY ListHead
//      );
//

#define RemoveTailList(ListHead) \
    (ListHead)->Blink;\
    {RemoveEntryList((ListHead)->Blink)}

//
//  VOID
//  RemoveEntryList(
//      PLIST_ENTRY Entry
//      );
//

#define RemoveEntryList(Entry) {\
    PLIST_ENTRY _EX_Blink;\
    PLIST_ENTRY _EX_Flink;\
    _EX_Flink = (Entry)->Flink;\
    _EX_Blink = (Entry)->Blink;\
    _EX_Blink->Flink = _EX_Flink;\
    _EX_Flink->Blink = _EX_Blink;\
    }

//
//  VOID
//  InsertTailList(
//      PLIST_ENTRY ListHead,
//      PLIST_ENTRY Entry
//      );
//

#define InsertTailList(ListHead,Entry) {\
    PLIST_ENTRY _EX_Blink;\
    PLIST_ENTRY _EX_ListHead;\
    _EX_ListHead = (ListHead);\
    _EX_Blink = _EX_ListHead->Blink;\
    (Entry)->Flink = _EX_ListHead;\
    (Entry)->Blink = _EX_Blink;\
    _EX_Blink->Flink = (Entry);\
    _EX_ListHead->Blink = (Entry);\
    }

//
//  VOID
//  InsertHeadList(
//      PLIST_ENTRY ListHead,
//      PLIST_ENTRY Entry
//      );
//

#define InsertHeadList(ListHead,Entry) {\
    PLIST_ENTRY _EX_Flink;\
    PLIST_ENTRY _EX_ListHead;\
    _EX_ListHead = (ListHead);\
    _EX_Flink = _EX_ListHead->Flink;\
    (Entry)->Flink = _EX_Flink;\
    (Entry)->Blink = _EX_ListHead;\
    _EX_Flink->Blink = (Entry);\
    _EX_ListHead->Flink = (Entry);\
    }



BOOL IsNodeOnList(PLIST_ENTRY ListHead, PLIST_ENTRY Entry);


#endif //InitializeListHead

//
// Templates for private addref and release. See Platform SDK documentation.
//

template <class T> ULONG MSPAddRefHelper (T * pMyThis)
{
    LOG((MSP_INFO, "MSPAddRefHelper - this = 0x%08x", pMyThis));
    typedef CComAggObject<T> AggClass;
    AggClass * p = CONTAINING_RECORD(pMyThis, AggClass, m_contained);
    return p->AddRef();
}

template <class T> ULONG MSPReleaseHelper (T * pMyThis)
{
    LOG((MSP_INFO, "MSPReleaseHelper - this = 0x%08x", pMyThis));
    typedef CComAggObject<T> AggClass;
    AggClass * p = CONTAINING_RECORD(pMyThis, AggClass, m_contained);
    return p->Release();
}




//
//  Basic implementation for IObjectSafety.
//
//  Derive from this class to make your object safe for scripting on all its
//  interfaces
//

#include <Objsafe.h>


class CMSPObjectSafetyImpl : public IObjectSafety
{

public:
    
    CMSPObjectSafetyImpl()
        :m_dwSafety(0)
    {}


    //
    // we support the following safety options:
    //

    enum { SUPPORTED_SAFETY_OPTIONS = 
       INTERFACESAFE_FOR_UNTRUSTED_CALLER | INTERFACESAFE_FOR_UNTRUSTED_DATA };



    STDMETHOD(SetInterfaceSafetyOptions)(REFIID riid, DWORD dwOptionSetMask, DWORD dwEnabledOptions)
    {

        //
        // any options requested that we do not support?
        //
        
        if ( (~SUPPORTED_SAFETY_OPTIONS & dwOptionSetMask) != 0 )
        {
            return E_FAIL;
        }

        
        //
        // see if the interface is supported at all
        //

        IUnknown *pUnk = NULL;

        HRESULT hr = QueryInterface(riid, (void**)&pUnk);

        if (SUCCEEDED(hr))
        {

            //
            // we don't need the interface, just wanted to see if it 
            // was supported. so release.
            //
            
            pUnk->Release();
            pUnk = NULL;

            //
            // the object supports the interface. Set options
            // 

            s_CritSection.Lock();

            //
            // set the bits specified by the mask to the values specified by 
            // dwEnabledOptions
            //

            m_dwSafety = (dwEnabledOptions & dwOptionSetMask) |
                         (m_dwSafety & ~dwOptionSetMask);

            s_CritSection.Unlock();

        }

        return hr;
    }


    
    STDMETHOD(GetInterfaceSafetyOptions)(REFIID riid, DWORD *pdwSupportedOptions, DWORD *pdwEnabledOptions)
    {
        
        //
        // check caller's pointers
        //

        if (( !pdwSupportedOptions) ||
            ( !pdwEnabledOptions) )
        {
             return E_POINTER;
        }

        //
        //  if we fail, return something meaningful
        //

        *pdwSupportedOptions = 0;
        *pdwEnabledOptions = 0;



        //
        // see if the interface is supported at all
        //

        IUnknown *pUnk = NULL;

        HRESULT hr = QueryInterface(riid, (void**)&pUnk);

        if (SUCCEEDED(hr))
        {

            //
            // we don't need the interface, just wanted to see if it 
            // was supported. so release.
            //
            
            pUnk->Release();
            pUnk = NULL;

            //
            // the object supports the interface. get safe scripting options
            // 

            *pdwSupportedOptions = SUPPORTED_SAFETY_OPTIONS;

 
            s_CritSection.Lock();

            *pdwEnabledOptions = m_dwSafety;

            s_CritSection.Unlock();

        }

        return hr;
    }

private:

    DWORD m_dwSafety;

    // 
    // thread safety
    //
    // the critical section is shared among all instances of this class
    //

    static CMSPCritSection s_CritSection;
    
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  //__MSPUTILS_H_

// eof

