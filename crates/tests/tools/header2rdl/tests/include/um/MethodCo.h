//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  MethodCo.h
//
//  Purpose: declaration of MethodContext class
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _METHOD_CONTEXT_H__
#define _METHOD_CONTEXT_H__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//#include "ThrdBase.h"
//#include "refptrco.h"

class CInstance;
class Provider;
class MethodContext;
class CWbemProviderGlue;
class InternalMethodContextAsynch;

typedef HRESULT (WINAPI *LPProviderInstanceCallback)(Provider *pProvider, CInstance *pInstance, MethodContext *pContext, void *pUserData);

//////////////////////////////////////////////////////
//
//  STRUCT MethodContext
//
// a little something to make sure we can keep our threads from getting tangled
// idea is that there is one MethodContext for each request from CIMOM or another provider
// pointers are passed around.
//////////////////////////////////////////////////////
class POLARITY MethodContext : public CThreadBase
{
public:
    friend InternalMethodContextAsynch;
    friend CWbemProviderGlue;

    MethodContext(IWbemContext   __RPC_FAR *piContext, CWbemProviderGlue *pGlue);
    ~MethodContext();
    
    virtual HRESULT Commit(CInstance *pInstance) = 0;
    virtual IWbemContext __RPC_FAR *GetIWBEMContext();
    
    LONG AddRef(void);
    LONG Release(void);
    virtual void QueryPostProcess(void);
        
    bool SetStatusObject(IWbemClassObject *pObj);
    IWbemClassObject __RPC_FAR *GetStatusObject();

#ifdef PROVIDER_INSTRUMENTATION
    StopWatch *pStopWatch;
#endif
    
private:
    CWbemProviderGlue* GetProviderGlue();
    CWbemProviderGlue   *m_pGlue;
    IWbemContext        __RPC_FAR *m_pContext;
    IWbemClassObject    __RPC_FAR *m_pStatusObject;

};

// for queries and suchlike that originate in CIMOM
class 
__declspec(uuid("9113D3B4-D114-11d2-B35D-00104BC97924")) 
ExternalMethodContext  : public MethodContext
{
public:
    ExternalMethodContext(IWbemObjectSink __RPC_FAR *pResponseHandler,
                          IWbemContext    __RPC_FAR *pContext,
                          CWbemProviderGlue *pGlue,
                          void                      *pReserved = NULL
                          );
    
    HRESULT Commit(CInstance *pInstance);
    virtual void QueryPostProcess(void);
    
    LONG AddRef(void);
    LONG Release(void);
    
private:
    IWbemObjectSink __RPC_FAR *m_pResponseHandler;
    void                      *m_pReserved;
};

// for queries and suchlike that come from within.
// contains a list of objects returned. 
class 
__declspec(uuid("6AF4B074-D121-11d2-B35D-00104BC97924"))
InternalMethodContext : public MethodContext
{
public:
    InternalMethodContext(TRefPointerCollection<CInstance> *pList ,
                          IWbemContext    __RPC_FAR *pContext,
                          CWbemProviderGlue *pGlue);
    ~InternalMethodContext();
    
    HRESULT Commit(CInstance *pInstance);
    
    LONG AddRef(void);
    LONG Release(void);
    
private:
    TRefPointerCollection<CInstance> *m_pInstances;
};

// for queries and suchlike that come from within.
// "Asynch" is a bit of a misnomer - but it does help support
// asynchronous calls, in that each instance committed is routed
// to a callback function supplied by the requester
class 
__declspec(uuid("D98A82E8-D121-11d2-B35D-00104BC97924"))
InternalMethodContextAsynch : public MethodContext
{
public:
    InternalMethodContextAsynch(Provider *pThat,
                                LPProviderInstanceCallback pCallback,
                                IWbemContext __RPC_FAR *pContext,
                                MethodContext *pUsersContext,
                                void *pUserData);
    ~InternalMethodContextAsynch();
    
    HRESULT Commit(CInstance *pInstance);
    
    LONG AddRef(void);
    LONG Release(void);
    
private:
    Provider *m_pThat;
    LPProviderInstanceCallback m_pCallback;
    void *m_pUserData;
    MethodContext *m_pUsersContext;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
