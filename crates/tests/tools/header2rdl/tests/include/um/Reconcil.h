//
//    Copyright (C) Microsoft.  All rights reserved.
//
// OLE reconciliation interface definitions.
#pragma once

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// for use in IStorage::SetStateBits()

#define STATEBITS_FLAT                 (0x0001)

// reconciliation HRESULTs
// TODO: IDLize this file and publish it.

#define REC_S_IDIDTHEUPDATES           MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0x1000)
#define REC_S_NOTCOMPLETE              MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0x1001)
#define REC_S_NOTCOMPLETEBUTPROPAGATE  MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0x1002)

#define REC_E_ABORTED                  MAKE_HRESULT(SEVERITY_ERROR,   FACILITY_ITF, 0x1000)
#define REC_E_NOCALLBACK               MAKE_HRESULT(SEVERITY_ERROR,   FACILITY_ITF, 0x1001)
#define REC_E_NORESIDUES               MAKE_HRESULT(SEVERITY_ERROR,   FACILITY_ITF, 0x1002)
#define REC_E_TOODIFFERENT             MAKE_HRESULT(SEVERITY_ERROR,   FACILITY_ITF, 0x1003)
#define REC_E_INEEDTODOTHEUPDATES      MAKE_HRESULT(SEVERITY_ERROR,   FACILITY_ITF, 0x1004)

#define IID_INotifyReplica __uuidof(INotifyReplica)
DECLARE_INTERFACE_IID_(INotifyReplica, IUnknown, "99180163-DA16-101A-935C-444553540000")
{
   // IUnknown

   IFACEMETHODIMP QueryInterface(THIS_
                                 _In_ REFIID riid,
                                 _Outptr_ void **ppvObject) PURE;

   IFACEMETHODIMP_(ULONG) AddRef(THIS) PURE;

   IFACEMETHODIMP_(ULONG) Release(THIS) PURE;

   // INotifyReplica

   STDMETHOD(YouAreAReplica)(THIS_
                             ULONG ulcOtherReplicas,
                             _Inout_updates_(ulcOtherReplicas) IMoniker **rgpmkOtherReplicas) PURE;
};

#define IID_IReconcileInitiator __uuidof(IReconcileInitiator)
DECLARE_INTERFACE_IID_(IReconcileInitiator, IUnknown, "99180161-DA16-101A-935C-444553540000")
{
   // IUnknown

   IFACEMETHODIMP QueryInterface(THIS_
                                 _In_ REFIID riid,
                                 _Outptr_ void **ppvObject) PURE;

   IFACEMETHODIMP_(ULONG) AddRef(THIS) PURE;

   IFACEMETHODIMP_(ULONG) Release(THIS) PURE;

   // IReconcileInitiator

   STDMETHOD(SetAbortCallback)(THIS_
                               _Inout_opt_ IUnknown *punkForAbort) PURE;

   STDMETHOD(SetProgressFeedback)(THIS_
                                  ULONG ulProgress,
                                  ULONG ulProgressMax) PURE;
};

// IReconcilableObject::Reconcile flags

typedef enum _reconcilef
{
   // interaction with the user is allowed

   RECONCILEF_MAYBOTHERUSER         = 0x0001,

   // hwndProgressFeedback may be used to provide reconciliation progress
   // feedback to the user.

   RECONCILEF_FEEDBACKWINDOWVALID   = 0x0002,

   // residue support not required

   RECONCILEF_NORESIDUESOK          = 0x0004,

   // caller not interested in callee's residues

   RECONCILEF_OMITSELFRESIDUE       = 0x0008,

   // Reconcile call resuming after a previous Reconcile() call returned
   // REC_E_NOTCOMPLETE

   RECONCILEF_RESUMERECONCILIATION  = 0x0010,

   // Object may perform all updates.

   RECONCILEF_YOUMAYDOTHEUPDATES    = 0x0020,

   // Only this object has been changed.

   RECONCILEF_ONLYYOUWERECHANGED    = 0x0040,

   // flag combinations

   ALL_RECONCILE_FLAGS              = (RECONCILEF_MAYBOTHERUSER |
                                       RECONCILEF_FEEDBACKWINDOWVALID |
                                       RECONCILEF_NORESIDUESOK |
                                       RECONCILEF_OMITSELFRESIDUE |
                                       RECONCILEF_RESUMERECONCILIATION |
                                       RECONCILEF_YOUMAYDOTHEUPDATES |
                                       RECONCILEF_ONLYYOUWERECHANGED)
}
RECONCILEF;

#define IID_IReconcilableObject __uuidof(IReconcilableObject)
DECLARE_INTERFACE_IID_(IReconcilableObject, IUnknown, "99180162-DA16-101A-935C-444553540000")
{
   // IUnknown

   IFACEMETHODIMP QueryInterface(THIS_
                                 _In_ REFIID riid,
                                 _Outptr_ void **ppvObject) PURE;

   IFACEMETHODIMP_(ULONG) AddRef(THIS) PURE;

   IFACEMETHODIMP_(ULONG) Release(THIS) PURE;

   // IReconcilableObject

   STDMETHOD(Reconcile)(THIS_
                        _Inout_ IReconcileInitiator *pInitiator,
                        DWORD dwFlags,
                        HWND hwndOwner,
                        HWND hwndProgressFeedback,
                        ULONG ulcInput,
                        _Inout_updates_(ulcInput) IMoniker **rgpmkOtherInput,
                        _Out_ PLONG plOutIndex,
                        _Inout_ IStorage *pstgNewResidues,
                        _Reserved_ PVOID pvReserved) PURE;

   STDMETHOD(GetProgressFeedbackMaxEstimate)(THIS_
                                             _Out_ PULONG pulProgressMax) PURE;
};

#define IID_IBriefcaseInitiator __uuidof(IBriefcaseInitiator)
DECLARE_INTERFACE_IID_(IBriefcaseInitiator, IUnknown, "99180164-DA16-101A-935C-444553540000")
{
   // IUnknown

   IFACEMETHODIMP QueryInterface(THIS_
                                 _In_ REFIID riid,
                                 _Outptr_ void **ppvObject) PURE;

   IFACEMETHODIMP_(ULONG) AddRef(THIS) PURE;

   IFACEMETHODIMP_(ULONG) Release(THIS) PURE;

   // IBriefcaseInitiator

   STDMETHOD(IsMonikerInBriefcase)(THIS_
                                   _Inout_ IMoniker *pmk) PURE;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion