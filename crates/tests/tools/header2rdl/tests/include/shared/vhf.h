/*++

Copyright (c) Microsoft Corporation. All Rights Reserved.

Module Name:

    vhf.h

Abstract:

    This is the Virtual HID Framework (VHF) interface for both User mode and Kernel mode.

Revision History:

--*/

#ifndef __VHF_H__
#define __VHF_H__

#ifdef __cplusplus
extern "C" {
#endif

#if(NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

//
// _HID_XFER_PACKET structure is defined in hidclass.h. If hidclass.h is
// not already included, then the structure will not be defined at this
// point. Instead of requiring that hidclass.h be included, define this
// structure in this header if not already defined.
//
#ifndef __HIDCLASS_H__

typedef struct _HID_XFER_PACKET {
    PUCHAR  reportBuffer;
    ULONG   reportBufferLen;
    UCHAR   reportId;
} HID_XFER_PACKET, *PHID_XFER_PACKET;

#endif

typedef PVOID VHFHANDLE;
typedef PVOID VHFOPERATIONHANDLE;

typedef
_Function_class_(EVT_VHF_READY_FOR_NEXT_READ_REPORT)
_IRQL_requires_max_(DISPATCH_LEVEL)
_IRQL_requires_same_
VOID
EVT_VHF_READY_FOR_NEXT_READ_REPORT(
    _In_
        PVOID VhfClientContext
    );

typedef EVT_VHF_READY_FOR_NEXT_READ_REPORT *PEVT_VHF_READY_FOR_NEXT_READ_REPORT;

typedef
_Function_class_(EVT_VHF_CLEANUP)
_IRQL_requires_max_(DISPATCH_LEVEL)
_IRQL_requires_same_
VOID
EVT_VHF_CLEANUP(
    _In_
        PVOID VhfClientContext
    );

typedef EVT_VHF_CLEANUP *PEVT_VHF_CLEANUP;

typedef
_Function_class_(EVT_VHF_ASYNC_OPERATION)
_IRQL_requires_max_(DISPATCH_LEVEL)
_IRQL_requires_same_
VOID
EVT_VHF_ASYNC_OPERATION(
    _In_
        PVOID               VhfClientContext,
    _In_
        VHFOPERATIONHANDLE  VhfOperationHandle,
    _In_opt_
        PVOID               VhfOperationContext,
    _In_
        PHID_XFER_PACKET    HidTransferPacket
    );

typedef EVT_VHF_ASYNC_OPERATION *PEVT_VHF_ASYNC_OPERATION;

typedef struct _VHF_CONFIG {

    ULONG                               Size;

    PVOID                               VhfClientContext;

    ULONG                               OperationContextSize;

#ifdef _KERNEL_MODE
    PDEVICE_OBJECT                      DeviceObject;
#else
    HANDLE                              FileHandle;
#endif

    USHORT                              VendorID;
    USHORT                              ProductID;
    USHORT                              VersionNumber;

    GUID                                ContainerID;

    USHORT                              InstanceIDLength;
    _Field_size_bytes_full_(InstanceIDLength)   
    PWSTR                               InstanceID;

    USHORT                              ReportDescriptorLength;
    _Field_size_full_(ReportDescriptorLength)
    PUCHAR                              ReportDescriptor;

    PEVT_VHF_READY_FOR_NEXT_READ_REPORT EvtVhfReadyForNextReadReport;
    PEVT_VHF_ASYNC_OPERATION            EvtVhfAsyncOperationGetFeature;
    PEVT_VHF_ASYNC_OPERATION            EvtVhfAsyncOperationSetFeature;
    PEVT_VHF_ASYNC_OPERATION            EvtVhfAsyncOperationWriteReport;
    PEVT_VHF_ASYNC_OPERATION            EvtVhfAsyncOperationGetInputReport;
    PEVT_VHF_CLEANUP                    EvtVhfCleanup;

    USHORT                              HardwareIDsLength;
    _Field_size_bytes_full_(HardwareIDsLength)
    PWSTR                               HardwareIDs;

} VHF_CONFIG, *PVHF_CONFIG;

FORCEINLINE
VOID
VHF_CONFIG_INIT(
    _Out_
        PVHF_CONFIG     Config,
#ifdef _KERNEL_MODE
    _In_
        PDEVICE_OBJECT  DeviceObject,
#else
    _In_
        HANDLE          FileHandle,
#endif
    _In_
        USHORT          ReportDescriptorLength,
    _In_reads_bytes_(ReportDescriptorLength)
        PUCHAR          ReportDescriptor    
    )
{
    RtlZeroMemory(Config, sizeof(VHF_CONFIG));

    Config->Size                                = sizeof(VHF_CONFIG);
    Config->VhfClientContext                    = NULL;
    Config->OperationContextSize                = 0;

#ifdef _KERNEL_MODE
    Config->DeviceObject                        = DeviceObject;
#else
    Config->FileHandle                          = FileHandle;
#endif

    Config->VendorID                            = 0;
    Config->ProductID                           = 0;
    Config->VersionNumber                       = 0;

    Config->ReportDescriptorLength              = ReportDescriptorLength;
    Config->ReportDescriptor                    = ReportDescriptor;

    Config->EvtVhfReadyForNextReadReport        = NULL;
    Config->EvtVhfCleanup                       = NULL;
    Config->EvtVhfAsyncOperationGetFeature      = NULL;
    Config->EvtVhfAsyncOperationSetFeature      = NULL;
    Config->EvtVhfAsyncOperationWriteReport     = NULL;
    Config->EvtVhfAsyncOperationGetInputReport  = NULL;
}

_IRQL_requires_(PASSIVE_LEVEL)
_Must_inspect_result_
NTSTATUS
VhfCreate(
    _In_
        PVHF_CONFIG VhfConfig,
    _Out_
        VHFHANDLE*  VhfHandle
    );

_IRQL_requires_max_(DISPATCH_LEVEL)
_Must_inspect_result_
NTSTATUS
VhfStart(
    _In_
        VHFHANDLE   VhfHandle
    );

_When_(Wait == __true, _IRQL_requires_(PASSIVE_LEVEL))
_When_(Wait == __false, _IRQL_requires_max_(DISPATCH_LEVEL))
VOID
VhfDelete(
    _In_
        VHFHANDLE   VhfHandle,
    _In_
        BOOLEAN     Wait
    );

_IRQL_requires_max_(DISPATCH_LEVEL)
_Must_inspect_result_
NTSTATUS
VhfReadReportSubmit(
    _In_
        VHFHANDLE           VhfHandle,
    _In_
        PHID_XFER_PACKET    HidTransferPacket
    );

_IRQL_requires_max_(DISPATCH_LEVEL)
NTSTATUS
VhfAsyncOperationComplete(
    _In_
        VHFOPERATIONHANDLE  VhfOperationHandle,
    _In_
        NTSTATUS            CompletionStatus
    );

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#ifdef __cplusplus
}
#endif

#endif // __VHF_H__
