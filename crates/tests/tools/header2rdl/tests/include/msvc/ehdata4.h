// Copyright (c) Microsoft Corporation. All rights reserved.

#pragma once

#include "ehdata4_export.h"
#include "ehdata.h"

#if !defined(RENAME_EH_EXTERN)
#define RENAME_EH_EXTERN(x) x
#endif

#if _EH_RELATIVE_FUNCINFO
inline uintptr_t __FuncRelToRealOffset(
    DispatcherContext   *pDC,
    unsigned int        funcRelOffset
)
{
    return (uintptr_t)(pDC->ImageBase + pDC->FunctionEntry->BeginAddress + funcRelOffset);
}

inline uintptr_t __RVAtoRealOffset(
    DispatcherContext   *pDC,
    int                 RVA
)
{
    return (uintptr_t)(pDC->ImageBase + RVA);
}
#endif

class RENAME_EH_EXTERN(__FrameHandler3)
{
public:
    using FuncInfo = FuncInfo;
    using HandlerType = HandlerType;
    using TryBlockMapEntry = TryBlockMapEntry;

    static TryBlockMapEntry * CatchTryBlock(
        FuncInfo            *pFuncInfo,
        __ehstate_t         curState
    );

    static bool ExecutionInCatch(
        DispatcherContext   *pDC,
        FuncInfo            *pFuncInfo
    );

    static __ehstate_t StateFromControlPc(
        FuncInfo            *pFuncInfo,
        DispatcherContext   *pDC
    );

    static __ehstate_t StateFromIp(
        FuncInfo            *pFuncInfo,
        DispatcherContext   *pDC,
        uintptr_t           Ip
    );

    static void FrameUnwindToState(
        EHRegistrationNode *pRN,
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo,
        __ehstate_t        targetState
    );

    static void FrameUnwindToEmptyState(
        EHRegistrationNode *pRN,
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo
    );

    inline static __ehstate_t GetHandlerSearchState(
        EHRegistrationNode *pRN,
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo
    );

    static __ehstate_t GetMaxState(
        DispatcherContext* /*pDC*/,
        FuncInfo           *pFuncInfo
    )
    {
        return pFuncInfo->maxState;
    };

    static int TypeMatch(
        HandlerType *pCatch,
        CatchableType *pCatchable,
        ThrowInfo *pThrow
    );

#if _EH_RELATIVE_FUNCINFO
    static EHRegistrationNode * GetEstablisherFrame(
        EHRegistrationNode  *pRN,
        DispatcherContext   *pDC,
        FuncInfo            *pFuncInfo,
        EHRegistrationNode  *pEstablisher
    );
#endif

    static int BuildCatchObjectHelper(
        EHExceptionRecord *pExcept,
        void              *pRN,
        HandlerType       *pCatch,
        CatchableType     *pConv
    );

    static void BuildCatchObject(
        EHExceptionRecord *pExcept,
        void *pRN,
        HandlerType *pCatch,
        CatchableType *pConv
    );

#if _EH_RELATIVE_FUNCINFO
    static void UnwindNestedFrames(
        EHRegistrationNode  *pFrame,
        EHExceptionRecord   *pExcept,
        CONTEXT             *pContext,
        EHRegistrationNode  *pEstablisher,
        void                *Handler,
        FuncInfo            *pFuncInfo,
        __ehstate_t         TargetUnwindState,
        __ehstate_t         CatchState,
        HandlerType         *pCatch,
        DispatcherContext   *pDC,
        BOOLEAN             recursive
    );
#endif

    inline static bool isEHs(FuncInfo* pFuncInfo)
    {
        return !!(FUNC_FLAGS(*pFuncInfo) & FI_EHS_FLAG);
    }

    inline static bool isNoExcept(FuncInfo* pFuncInfo)
    {
        return !!(FUNC_FLAGS(*pFuncInfo) & FI_EHNOEXCEPT_FLAG);
    }

    inline static uint32_t getMagicNum(FuncInfo* pFuncInfo)
    {
        return FUNC_MAGICNUM(*pFuncInfo);
    }

    inline static ESTypeList* getESTypes(FuncInfo* pFuncInfo);

    static __ehstate_t GetCurrentState(
        EHRegistrationNode  *pFrame,
        DispatcherContext   *pDC,
        FuncInfo            *pFuncInfo
    );

    static void SetState(
        EHRegistrationNode  *pRN,
        FuncInfo            *pFuncInfo,
        __ehstate_t          newState
    );

    static void SetUnwindTryBlock(
        EHRegistrationNode  *pRN,
        DispatcherContext   *pDC,
        FuncInfo            *pFuncInfo,
        int                 curState
    );

    static __ehstate_t GetUnwindTryBlock(
        EHRegistrationNode  *pRN,
        DispatcherContext   *pDC,
        FuncInfo            *pFuncInfo
    );

    static void * CxxCallCatchBlock(EXCEPTION_RECORD* pExcept);

    class TryBlockMap
    {
    public:
        TryBlockMap(FuncInfo *pFuncInfo, uintptr_t imageBase) :
            _pFuncInfo(pFuncInfo), _imageBase(imageBase) {}

        class iterator
        {
        public:
            iterator(TryBlockMap &tryBlockMap, unsigned index):
                _tryBlockMap(tryBlockMap), _index(index) {}

            iterator& operator++()
            {
                _index++;
                return *this;
            }

            TryBlockMapEntry operator*()
            {
            #if _EH_RELATIVE_FUNCINFO
                return *(FUNC_PTRYBLOCK(*_tryBlockMap._pFuncInfo, _index, (uintptr_t)(_tryBlockMap._imageBase)));
            #else
                return *(FUNC_PTRYBLOCK(*_tryBlockMap._pFuncInfo, _index));
            #endif
            }

            bool operator!=(const iterator &other) const
            {
                return (_index != other._index);
            }

            bool operator<(const iterator &other) const
            {
                return (_index < other._index);
            }

            iterator& operator=(const iterator &other)
            {
                this->_index = other._index;
                return *this;
            }

        private:
            TryBlockMap & _tryBlockMap;
            unsigned _index;
        };

        unsigned getNumTryBlocks()
        {
            return FUNC_NTRYBLOCKS(*_pFuncInfo);
        }

        using IteratorPair = std::pair<iterator, iterator>;

    private:
        FuncInfo* _pFuncInfo;
        uintptr_t _imageBase;
    };

    static TryBlockMap::IteratorPair GetRangeOfTrysToCheck(
        TryBlockMap       &TryBlockMap,
        __ehstate_t       curState,
        DispatcherContext *pDC,
        FuncInfo          *pFuncInfo,
        int                CatchDepth
    );

    class HandlerMap
    {
    public:
#pragma warning(push)
#pragma warning(disable: 4100) // unreferenced formal parameter
        HandlerMap(TryBlockMapEntry *tryMap, uintptr_t imageBase, uint32_t functionStart)
        {
#if _EH_RELATIVE_FUNCINFO
            _handler = (HandlerType *)((uintptr_t)(imageBase + tryMap->dispHandlerArray));
#else
            _handler = (HandlerType *)tryMap->pHandlerArray;
#endif
            _numHandlers = tryMap->nCatches;
        }
#pragma warning(pop)

        class iterator
        {
        public:
            iterator(HandlerMap & handlerMap, unsigned currBlock)
                : _handlerMap(handlerMap), _currBlock(currBlock)
            {}

            iterator& operator++()
            {
                _currBlock++;
                return *this;
            }

            HandlerType operator*()
            {
                return *((HandlerType *)(_handlerMap._handler + _currBlock));
            }

            bool operator==(const iterator &other) const
            {
                return (_currBlock == other._currBlock);
            }

            bool operator!=(const iterator &other) const
            {
                return !(*this == other);
            }

        private:
            HandlerMap & _handlerMap;
            unsigned _currBlock;
        };

        iterator begin()
        {
            return iterator(*this, 0);
        }

        iterator end()
        {
            return iterator(*this, _numHandlers);
        }

        HandlerType* getLastEntry()
        {
            return (HandlerType *)(_handler + _numHandlers - 1);
        }

    private:
        unsigned _numHandlers;
        HandlerType *_handler;
    };
};

class RENAME_EH_EXTERN(__FrameHandler4)
{
public:
    using FuncInfo = FH4::FuncInfo4;
    using HandlerType = FH4::HandlerType4;
    using TryBlockMapEntry = FH4::TryBlockMapEntry4;

    using UWMap = FH4::UWMap4;
    using TryBlockMap = FH4::TryBlockMap4;
    using HandlerMap = FH4::HandlerMap4;

    static bool ExecutionInCatch(
        DispatcherContext *pDC,
        FuncInfo          *pFuncInfo
    );

    static __ehstate_t StateFromControlPc(
        FuncInfo          *pFuncInfo,
        DispatcherContext *pDC
    );

    static __ehstate_t StateFromIp(
        FuncInfo          *pFuncInfo,
        DispatcherContext *pDC,
        uintptr_t         Ip
    );

    static void FrameUnwindToState(
        EHRegistrationNode *pRN,
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo,
        __ehstate_t        targetState
    );

    static void FrameUnwindToEmptyState(
        EHRegistrationNode *pRN,
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo
    );

    inline static __ehstate_t GetHandlerSearchState(
        EHRegistrationNode *pRN,
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo
    );

#if _EH_RELATIVE_FUNCINFO
    static __ehstate_t GetMaxState(
        DispatcherContext  *pDC,
        FuncInfo           *pFuncInfo
    )
    {
        if (pFuncInfo->dispUnwindMap)
        {
            uint8_t * buffer = reinterpret_cast<uint8_t *>(__RVAtoRealOffset(pDC, pFuncInfo->dispUnwindMap));
            return FH4::ReadUnsigned(&buffer);
        }

        return 0;
    }
#endif

    static int TypeMatch(
        HandlerType   *pCatch,
        CatchableType *pCatchable,
        ThrowInfo     *pThrow
    );

    static int BuildCatchObjectHelper(
        EHExceptionRecord *pExcept,
        void              *pRN,
        HandlerType       *pCatch,
        CatchableType     *pConv
    );

    static void BuildCatchObject(
        EHExceptionRecord *pExcept,
        void              *pRN,
        HandlerType       *pCatch,
        CatchableType     *pConv
    );

    static void UnwindNestedFrames(
        EHRegistrationNode  *pFrame,
        EHExceptionRecord   *pExcept,
        CONTEXT             *pContext,
        EHRegistrationNode  *pEstablisher,
        void                *Handler,
        FuncInfo            *pFuncInfo,
        __ehstate_t         TargetUnwindState,
        __ehstate_t         CatchState,
        HandlerType         *pCatch,
        DispatcherContext   *pDC,
        BOOLEAN             recursive
    );

    static EHRegistrationNode * GetEstablisherFrame(
        EHRegistrationNode  *pRN,
        DispatcherContext   *pDC,
        FuncInfo            *pFuncInfo,
        EHRegistrationNode  *pEstablisher
    );

    inline static bool isEHs(FuncInfo* pFuncInfo)
    {
        return pFuncInfo->header.EHs;
    }

    inline static bool isNoExcept(FuncInfo* pFuncInfo)
    {
        return pFuncInfo->header.NoExcept;
    }

    // For simplicity in merging, treat FrameHandler4 as having a magic number
    inline static uint32_t getMagicNum(FuncInfo* /*pFuncInfo*/)
    {
        return EH_MAGIC_NUMBER3;
    }

    // Not supported under FrameHandler4
    inline static ESTypeList* getESTypes(FuncInfo* /*pFuncInfo*/)
    {
        return nullptr;
    }

    static void * CxxCallCatchBlock(EXCEPTION_RECORD* pExcept);

    static TryBlockMap::IteratorPair GetRangeOfTrysToCheck(
        TryBlockMap       &TryBlockMap,
        __ehstate_t       curState,
        DispatcherContext *pDC,
        FuncInfo         *pFuncInfo,
        int               CatchDepth
    );
};
